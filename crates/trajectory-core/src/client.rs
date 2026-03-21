use crate::auth::{ClientAccessKey, AUTH_TAG_LEN};
use crate::protocol::{
    build_probe_query, build_query, max_query_payload_for_domain, parse_dns_id,
    parse_response_meta, DOWNLINK_WINDOW, FLAG_DATA, FLAG_DOWNLINK, FLAG_FIN, KEEPALIVE_MS,
    MAX_INFLIGHT_PER_RESOLVER, POLL_WINDOW, QUERY_TIMEOUT_MS, WINDOW_SIZE,
};
use anyhow::{bail, Context, Result};
use rand::{thread_rng, Rng};
use std::collections::{BTreeMap, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{mpsc, watch};

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub listen: SocketAddr,
    pub resolvers: Vec<SocketAddr>,
    pub domain: String,
    pub access_key: ClientAccessKey,
    pub keep_alive_interval: Duration,
    pub request_timeout: Duration,
}

#[derive(Clone)]
struct Resolver {
    transport: ResolverTransport,
    label: String,
    srtt: Duration,
    in_flight: usize,
    sent: u64,
    timed_out: u64,
    penalized_until: Instant,
}

#[derive(Clone)]
enum ResolverTransport {
    Udp { socket: Arc<UdpSocket> },
    Tcp { addr: SocketAddr },
}

struct UplinkChunk {
    data: Vec<u8>,
    fin: bool,
    last_sent: Instant,
    attempts: u32,
}

struct QueryMeta {
    kind: QueryKind,
    resolver: usize,
    sent_at: Instant,
}

#[derive(Clone, Copy, Debug)]
enum QueryKind {
    Uplink(u32),
    Downlink(u32),
    KeepAlive,
}

enum ResolverEvent {
    Packet { resolver_index: usize, bytes: Vec<u8> },
    SendFailure {
        resolver_index: usize,
        dns_id: u16,
        message: String,
    },
}

fn debug_client_enabled() -> bool {
    std::env::var_os("TRAJECTORY_DEBUG").is_some()
}

fn debug_client_log(message: impl AsRef<str>) {
    if debug_client_enabled() {
        eprintln!("[trajectory-client] {}", message.as_ref());
    }
}

fn tcp_resolver_timeout(timeout: Duration) -> Duration {
    timeout.max(Duration::from_secs(3))
}

fn max_inflight_for(resolver: &Resolver) -> usize {
    match resolver.transport {
        ResolverTransport::Udp { .. } => MAX_INFLIGHT_PER_RESOLVER,
        ResolverTransport::Tcp { .. } => 2,
    }
}

pub async fn run(config: ClientConfig) -> Result<()> {
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    run_until(config, shutdown_rx).await
}

pub async fn run_until(config: ClientConfig, mut shutdown_rx: watch::Receiver<bool>) -> Result<()> {
    let listener = TcpListener::bind(config.listen).await?;
    loop {
        let accept = tokio::select! {
            changed = shutdown_rx.changed() => {
                changed.ok();
                return Ok(());
            }
            result = listener.accept() => result,
        };
        let (stream, _) = accept?;
        let config = config.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_stream(stream, config).await {
                eprintln!("client session error: {error:#}");
            }
        });
    }
}

async fn handle_stream(stream: TcpStream, config: ClientConfig) -> Result<()> {
    let session_id = thread_rng().gen::<u64>();
    let (mut tcp_reader, mut tcp_writer) = stream.into_split();
    debug_client_log(format!(
        "accepted session {session_id:016x} for domain {} via {} resolvers",
        config.domain,
        config.resolvers.len()
    ));
    let max_query_payload = max_query_payload_for_domain(&config.domain)?;

    let mut probed = Vec::with_capacity(config.resolvers.len());
    let (resp_tx, mut resp_rx) = mpsc::unbounded_channel::<ResolverEvent>();
    for addr in config.resolvers.iter().copied() {
        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
        socket.connect(addr).await?;
        match probe_udp_resolver(&socket, &config.domain, config.request_timeout).await {
            Ok(srtt) => {
                debug_client_log(format!("resolver {addr} probe ok in {} ms over udp", srtt.as_millis()));
                probed.push((
                    ResolverTransport::Udp { socket },
                    srtt,
                    format!("{addr} (udp)"),
                ));
            }
            Err(udp_error) => match probe_tcp_resolver(addr, &config.domain, tcp_resolver_timeout(config.request_timeout)).await {
                Ok(srtt) => {
                    debug_client_log(format!(
                        "resolver {addr} probe fell back to tcp in {} ms after udp failed: {udp_error:#}",
                        srtt.as_millis()
                    ));
                    probed.push((
                        ResolverTransport::Tcp { addr },
                        srtt,
                        format!("{addr} (tcp)"),
                    ));
                }
                Err(tcp_error) => {
                    debug_client_log(format!(
                        "resolver {addr} probe failed over udp ({udp_error:#}) and tcp ({tcp_error:#})"
                    ));
                    probed.push((
                        ResolverTransport::Udp { socket },
                        Duration::from_millis(2_000),
                        format!("{addr} (udp)"),
                    ));
                }
            },
        }
    }
    probed.sort_by_key(|(_, srtt, _)| *srtt);
    let has_udp = probed.iter().any(|(transport, srtt, _)| {
        matches!(transport, ResolverTransport::Udp { .. }) && *srtt < Duration::from_millis(2_000)
    });

    let selected: Vec<_> = if has_udp {
        probed
            .into_iter()
            .filter(|(transport, srtt, _)| {
                matches!(transport, ResolverTransport::Udp { .. }) && *srtt < Duration::from_millis(2_000)
            })
            .take(4)
            .collect()
    } else {
        probed
            .into_iter()
            .filter(|(_, srtt, _)| *srtt < Duration::from_millis(5_000))
            .take(1)
            .collect()
    };

    let mut resolvers = Vec::with_capacity(selected.len());
    for (index, (transport, srtt, label)) in selected.into_iter().enumerate() {
        if let ResolverTransport::Udp { socket } = &transport {
            let rx_socket = socket.clone();
            let tx = resp_tx.clone();
            tokio::spawn(async move {
                let mut buf = vec![0u8; 2048];
                loop {
                    match rx_socket.recv(&mut buf).await {
                        Ok(len) => {
                            if tx
                                .send(ResolverEvent::Packet {
                                    resolver_index: index,
                                    bytes: buf[..len].to_vec(),
                                })
                                .is_err()
                            {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        resolvers.push(Resolver {
            transport,
            label,
            srtt,
            in_flight: 0,
            sent: 0,
            timed_out: 0,
            penalized_until: Instant::now(),
        });
    }
    debug_client_log(format!("session {session_id:016x} selected {} active resolvers", resolvers.len()));
    let request_timeout = if resolvers
        .iter()
        .any(|resolver| matches!(resolver.transport, ResolverTransport::Tcp { .. }))
    {
        tcp_resolver_timeout(config.request_timeout)
    } else {
        config.request_timeout
    };
    let (uplink_tx, mut uplink_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 65536];
        loop {
            match tcp_reader.read(&mut buf).await {
                Ok(0) => {
                    debug_client_log("local tcp stream closed");
                    let _ = uplink_tx.send(Vec::new());
                    break;
                }
                Ok(len) => {
                    debug_client_log(format!("read {len} bytes from local tcp stream"));
                    if uplink_tx.send(buf[..len].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => {
                    debug_client_log("local tcp stream read failed");
                    let _ = uplink_tx.send(Vec::new());
                    break;
                }
            }
        }
    });

    let mut pending = BTreeMap::<u32, UplinkChunk>::new();
    let mut next_seq = 0u32;
    let mut remote_ack = 0u32;
    let mut down_next = 0u32;
    let mut down_pending = BTreeMap::<u32, Vec<u8>>::new();
    let mut in_flight = HashMap::<u16, QueryMeta>::new();
    let mut local_closed = false;
    let mut last_send = Instant::now() - config.keep_alive_interval;
    let mut read_closed_sent = false;
    let mut session_bootstrapped = false;

    loop {
        while let Ok(chunk) = uplink_rx.try_recv() {
            if chunk.is_empty() {
                local_closed = true;
                break;
            }
            for part in chunk.chunks(max_query_payload) {
                pending.insert(
                    next_seq,
                    UplinkChunk {
                        data: part.to_vec(),
                        fin: false,
                        last_sent: Instant::now() - request_timeout,
                        attempts: 0,
                    },
                );
                next_seq = next_seq.wrapping_add(1);
            }
        }

        while let Ok(event) = resp_rx.try_recv() {
            match event {
                ResolverEvent::Packet {
                    resolver_index,
                    bytes,
                } => match parse_response_meta(&bytes) {
                Ok((dns_id, maybe_response)) => {
                if let Some(meta) = in_flight.remove(&dns_id) {
                    if let QueryKind::Uplink(seq) = meta.kind {
                        if let Some(chunk) = pending.get_mut(&seq) {
                            chunk.last_sent = Instant::now();
                        }
                    }
                    let resolver_label = resolvers[resolver_index].label.clone();
                    let resolver = &mut resolvers[resolver_index];
                    resolver.in_flight = resolver.in_flight.saturating_sub(1);
                    match maybe_response {
                        Some(response) => {
                            if !response.verify(&config.access_key).unwrap_or(false) {
                                debug_client_log(format!(
                                    "resolver {} returned response with invalid auth tag",
                                    resolver_label
                                ));
                                resolver.timed_out += 1;
                                let penalty = Duration::from_millis(750 * resolver.timed_out.min(4));
                                resolver.penalized_until = Instant::now() + penalty;
                                continue;
                            }
                            resolver.sent += 1;
                            resolver.timed_out = resolver.timed_out.saturating_sub(1);
                            let rtt = meta.sent_at.elapsed();
                            resolver.srtt = blend_rtt(resolver.srtt, rtt);
                            session_bootstrapped = true;
                            if response.ack > remote_ack {
                                remote_ack = response.ack;
                                pending.retain(|seq, _| *seq >= remote_ack);
                            }
                            if response.flags & FLAG_DOWNLINK != 0 {
                                down_pending
                                    .entry(response.down_seq)
                                    .or_insert(response.payload.clone());
                                while let Some(bytes) = down_pending.remove(&down_next) {
                                    tcp_writer.write_all(&bytes).await?;
                                    down_next = down_next.wrapping_add(1);
                                }
                            }
                        }
                        None => {
                            debug_client_log(format!(
                                "resolver {} returned empty response for {:?}",
                                resolver_label,
                                meta.kind
                            ));
                                resolver.timed_out += 1;
                                let penalty = Duration::from_millis(750 * resolver.timed_out.min(4));
                                resolver.penalized_until = Instant::now() + penalty;
                                if let QueryKind::Uplink(seq) = meta.kind {
                                    if let Some(chunk) = pending.get_mut(&seq) {
                                        chunk.last_sent = Instant::now() - request_timeout;
                                    }
                                }
                            }
                    }
                }
                }
                Err(error) => {
                    debug_client_log(format!(
                        "failed to parse dns response from resolver {}: {error:#}",
                        resolvers[resolver_index].label
                    ));
                }
                },
                ResolverEvent::SendFailure {
                    resolver_index,
                    dns_id,
                    message,
                } => {
                    debug_client_log(format!(
                        "resolver {} request {} failed: {}",
                        resolvers[resolver_index].label,
                        dns_id,
                        message
                    ));
                    if let Some(meta) = in_flight.remove(&dns_id) {
                        let resolver = &mut resolvers[resolver_index];
                        resolver.in_flight = resolver.in_flight.saturating_sub(1);
                        resolver.timed_out += 1;
                        let penalty = Duration::from_millis(750 * resolver.timed_out.min(4));
                        resolver.penalized_until = Instant::now() + penalty;
                        if let QueryKind::Uplink(seq) = meta.kind {
                            if let Some(chunk) = pending.get_mut(&seq) {
                                chunk.last_sent = Instant::now() - request_timeout;
                            }
                        }
                    }
                }
            }
        }

        let now = Instant::now();
        let timed_out: Vec<u16> = in_flight
            .iter()
            .filter_map(|(request_id, meta)| {
                let timeout = match resolvers.get(meta.resolver) {
                    Some(resolver) => match resolver.transport {
                        ResolverTransport::Udp { .. } => config.request_timeout,
                        ResolverTransport::Tcp { .. } => request_timeout,
                    },
                    None => config.request_timeout,
                };
                if now.duration_since(meta.sent_at) >= timeout {
                    Some(*request_id)
                } else {
                    None
                }
            })
            .collect();
        for request_id in timed_out {
            if let Some(meta) = in_flight.remove(&request_id) {
                let resolver = &mut resolvers[meta.resolver];
                resolver.in_flight = resolver.in_flight.saturating_sub(1);
                resolver.timed_out += 1;
                let penalty = Duration::from_millis(750 * resolver.timed_out.min(4));
                resolver.penalized_until = Instant::now() + penalty;
                if let QueryKind::Uplink(seq) = meta.kind {
                    if let Some(chunk) = pending.get_mut(&seq) {
                        chunk.last_sent = Instant::now() - request_timeout;
                    }
                }
            }
        }

        while in_flight.len() < WINDOW_SIZE {
            let candidate = pick_chunk(&mut pending, request_timeout);
            let down_request = if session_bootstrapped {
                pick_down_request(down_next, &down_pending, &in_flight)
            } else {
                None
            };
            let should_poll = candidate.is_none()
                && session_bootstrapped
                && (down_request.is_some()
                    || (pending.is_empty()
                        && (in_flight.len() < POLL_WINDOW
                            || now.duration_since(last_send) >= config.keep_alive_interval)));
            let should_fin = local_closed && pending.is_empty() && !read_closed_sent;
            if candidate.is_none() && down_request.is_none() && !should_poll && !should_fin {
                break;
            }
            let resolver_index = pick_resolver(&resolvers).context("no resolvers")?;
            if resolvers[resolver_index].in_flight >= max_inflight_for(&resolvers[resolver_index]) {
                break;
            }
            let (flags, seq, payload, kind) = if let Some(seq) = candidate {
                let chunk = pending.get_mut(&seq).unwrap();
                let mut flags = FLAG_DATA;
                if chunk.fin {
                    flags |= FLAG_FIN;
                }
                chunk.last_sent = now;
                chunk.attempts += 1;
                (flags, seq, chunk.data.clone(), QueryKind::Uplink(seq))
            } else if let Some(seq) = down_request {
                (0, seq, Vec::new(), QueryKind::Downlink(seq))
            } else if should_fin {
                read_closed_sent = true;
                (FLAG_FIN, down_next, Vec::new(), QueryKind::KeepAlive)
            } else {
                (0, down_next, Vec::new(), QueryKind::KeepAlive)
            };
            let mut request = crate::protocol::RequestPacket {
                session_id,
                flags,
                down_ack: down_next,
                seq,
                client_id: 0,
                auth_tag: [0; AUTH_TAG_LEN],
                payload,
            };
            request.sign(&config.access_key)?;
            let (dns_id, wire) = build_query(&request, &config.domain)?;
            debug_client_log(format!(
                "sent {:?} request {} via resolver {}",
                kind,
                dns_id,
                resolvers[resolver_index].label
            ));
            send_query(
                &resolvers[resolver_index],
                resolver_index,
                dns_id,
                wire,
                request_timeout,
                resp_tx.clone(),
            )
            .await?;
            resolvers[resolver_index].in_flight += 1;
            in_flight.insert(
                dns_id,
                QueryMeta {
                    kind,
                    resolver: resolver_index,
                    sent_at: Instant::now(),
                },
            );
            last_send = Instant::now();
        }

        let done = local_closed && read_closed_sent && pending.is_empty() && in_flight.is_empty();
        if done {
            let _ = tcp_writer.shutdown().await;
            break;
        }

        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    Ok(())
}

fn pick_chunk(pending: &mut BTreeMap<u32, UplinkChunk>, timeout: Duration) -> Option<u32> {
    let now = Instant::now();
    pending
        .iter()
        .find_map(|(seq, chunk)| (now.duration_since(chunk.last_sent) >= timeout).then_some(*seq))
}

fn pick_resolver(resolvers: &[Resolver]) -> Result<usize> {
    let now = Instant::now();
    resolvers
        .iter()
        .enumerate()
        .min_by_key(|(_, resolver)| {
            let cooldown = resolver
                .penalized_until
                .checked_duration_since(now)
                .unwrap_or_default()
                .as_micros() as u64;
            resolver.srtt.as_micros() as u64 * (resolver.in_flight as u64 + 1)
                + resolver.timed_out.min(4).saturating_mul(250_000)
                + cooldown
        })
        .map(|(index, _)| index)
        .context("no resolvers configured")
}

fn blend_rtt(old: Duration, sample: Duration) -> Duration {
    let old_us = old.as_micros() as u64;
    let sample_us = sample.as_micros() as u64;
    Duration::from_micros((old_us * 7 + sample_us) / 8)
}

fn pick_down_request(
    down_next: u32,
    down_pending: &BTreeMap<u32, Vec<u8>>,
    in_flight: &HashMap<u16, QueryMeta>,
) -> Option<u32> {
    let max_seq = down_next.saturating_add(DOWNLINK_WINDOW as u32);
    (down_next..max_seq).find(|seq| {
        !down_pending.contains_key(seq)
            && !in_flight
                .values()
                .any(|meta| matches!(meta.kind, QueryKind::Downlink(requested) if requested == *seq))
    })
}

async fn probe_udp_resolver(socket: &UdpSocket, domain: &str, timeout: Duration) -> Result<Duration> {
    let (dns_id, wire) = build_probe_query(domain)?;
    let started = Instant::now();
    socket.send(&wire).await?;
    let mut buf = vec![0u8; 2048];
    let len = tokio::time::timeout(timeout, socket.recv(&mut buf)).await??;
    let received_id = parse_dns_id(&buf[..len])?;
    if received_id != dns_id {
        bail!("mismatched probe response id");
    }
    Ok(started.elapsed())
}

async fn probe_tcp_resolver(addr: SocketAddr, domain: &str, timeout: Duration) -> Result<Duration> {
    let (dns_id, wire) = build_probe_query(domain)?;
    let started = Instant::now();
    let response = send_tcp_query(addr, wire, timeout).await?;
    let received_id = parse_dns_id(&response)?;
    if received_id != dns_id {
        bail!("mismatched probe response id");
    }
    Ok(started.elapsed())
}

async fn send_query(
    resolver: &Resolver,
    resolver_index: usize,
    dns_id: u16,
    wire: Vec<u8>,
    timeout: Duration,
    resp_tx: mpsc::UnboundedSender<ResolverEvent>,
) -> Result<()> {
    match &resolver.transport {
        ResolverTransport::Udp { socket } => {
            socket.send(&wire).await?;
        }
        ResolverTransport::Tcp { addr } => {
            let addr = *addr;
            tokio::spawn(async move {
                match send_tcp_query(addr, wire, timeout).await {
                    Ok(bytes) => {
                        let _ = resp_tx.send(ResolverEvent::Packet {
                            resolver_index,
                            bytes,
                        });
                    }
                    Err(error) => {
                        let _ = resp_tx.send(ResolverEvent::SendFailure {
                            resolver_index,
                            dns_id,
                            message: format!("{error:#}"),
                        });
                    }
                }
            });
        }
    }
    Ok(())
}

async fn send_tcp_query(addr: SocketAddr, wire: Vec<u8>, timeout: Duration) -> Result<Vec<u8>> {
    let mut stream = tokio::time::timeout(timeout, TcpStream::connect(addr))
        .await
        .context("dns tcp connect timed out")?
        .with_context(|| format!("connect to dns resolver {addr} over tcp"))?;
    stream
        .write_all(&(wire.len() as u16).to_be_bytes())
        .await
        .with_context(|| format!("write dns tcp length to {addr}"))?;
    stream
        .write_all(&wire)
        .await
        .with_context(|| format!("write dns tcp payload to {addr}"))?;

    let mut length_bytes = [0u8; 2];
    tokio::time::timeout(timeout, stream.read_exact(&mut length_bytes))
        .await
        .context("dns tcp length read timed out")?
        .with_context(|| format!("read dns tcp length from {addr}"))?;

    let response_len = u16::from_be_bytes(length_bytes) as usize;
    let mut response = vec![0u8; response_len];
    tokio::time::timeout(timeout, stream.read_exact(&mut response))
        .await
        .context("dns tcp payload read timed out")?
        .with_context(|| format!("read dns tcp payload from {addr}"))?;
    Ok(response)
}

pub fn parse_socket_addr(value: &str, default_port: u16) -> Result<SocketAddr> {
    if let Ok(addr) = value.parse() {
        return Ok(addr);
    }
    let combined = format!("{value}:{default_port}");
    combined
        .parse()
        .with_context(|| format!("invalid socket address {value}"))
}

pub fn default_client_config(
    listen: SocketAddr,
    resolvers: Vec<SocketAddr>,
    domain: String,
    access_key: ClientAccessKey,
) -> ClientConfig {
    ClientConfig {
        listen,
        resolvers,
        domain,
        access_key,
        keep_alive_interval: Duration::from_millis(KEEPALIVE_MS),
        request_timeout: Duration::from_millis(QUERY_TIMEOUT_MS),
    }
}

pub fn require_resolvers(resolvers: &[SocketAddr]) -> Result<()> {
    if resolvers.is_empty() {
        bail!("missing required --resolver");
    }
    Ok(())
}
