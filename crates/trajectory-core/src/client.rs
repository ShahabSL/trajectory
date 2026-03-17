use crate::protocol::{
    build_probe_query, build_query, parse_dns_id, parse_response_meta, DOWNLINK_WINDOW, FLAG_DATA,
    FLAG_DOWNLINK, FLAG_FIN, KEEPALIVE_MS, MAX_INFLIGHT_PER_RESOLVER, MAX_QUERY_PAYLOAD,
    POLL_WINDOW, QUERY_TIMEOUT_MS, WINDOW_SIZE,
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
    pub keep_alive_interval: Duration,
    pub request_timeout: Duration,
}

#[derive(Clone)]
struct Resolver {
    socket: Arc<UdpSocket>,
    srtt: Duration,
    in_flight: usize,
    sent: u64,
    timed_out: u64,
    penalized_until: Instant,
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

    let mut probed = Vec::with_capacity(config.resolvers.len());
    let (resp_tx, mut resp_rx) = mpsc::unbounded_channel::<(usize, Vec<u8>)>();
    for addr in config.resolvers.iter().copied() {
        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
        socket.connect(addr).await?;
        let srtt = probe_resolver(&socket, &config.domain, config.request_timeout)
            .await
            .unwrap_or(Duration::from_millis(2_000));
        probed.push((socket, srtt));
    }
    probed.sort_by_key(|(_, srtt)| *srtt);
    let keep = probed
        .iter()
        .filter(|(_, srtt)| *srtt < Duration::from_millis(2_000))
        .count()
        .clamp(1, 4);

    let mut resolvers = Vec::with_capacity(keep);
    for (index, (socket, srtt)) in probed.into_iter().take(keep).enumerate() {
        let rx_socket = socket.clone();
        let tx = resp_tx.clone();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 2048];
            loop {
                match rx_socket.recv(&mut buf).await {
                    Ok(len) => {
                        if tx.send((index, buf[..len].to_vec())).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        resolvers.push(Resolver {
            socket,
            srtt,
            in_flight: 0,
            sent: 0,
            timed_out: 0,
            penalized_until: Instant::now(),
        });
    }
    let (uplink_tx, mut uplink_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 65536];
        loop {
            match tcp_reader.read(&mut buf).await {
                Ok(0) => {
                    let _ = uplink_tx.send(Vec::new());
                    break;
                }
                Ok(len) => {
                    if uplink_tx.send(buf[..len].to_vec()).is_err() {
                        break;
                    }
                }
                Err(_) => {
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

    loop {
        while let Ok(chunk) = uplink_rx.try_recv() {
            if chunk.is_empty() {
                local_closed = true;
                break;
            }
            for part in chunk.chunks(MAX_QUERY_PAYLOAD) {
                pending.insert(
                    next_seq,
                    UplinkChunk {
                        data: part.to_vec(),
                        fin: false,
                        last_sent: Instant::now() - config.request_timeout,
                        attempts: 0,
                    },
                );
                next_seq = next_seq.wrapping_add(1);
            }
        }

        while let Ok((resolver_index, bytes)) = resp_rx.try_recv() {
            if let Ok((dns_id, maybe_response)) = parse_response_meta(&bytes) {
                if let Some(meta) = in_flight.remove(&dns_id) {
                    if let QueryKind::Uplink(seq) = meta.kind {
                        if let Some(chunk) = pending.get_mut(&seq) {
                            chunk.last_sent = Instant::now();
                        }
                    }
                    let resolver = &mut resolvers[resolver_index];
                    resolver.in_flight = resolver.in_flight.saturating_sub(1);
                    match maybe_response {
                        Some(response) => {
                            resolver.sent += 1;
                            resolver.timed_out = resolver.timed_out.saturating_sub(1);
                            let rtt = meta.sent_at.elapsed();
                            resolver.srtt = blend_rtt(resolver.srtt, rtt);
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
                            resolver.timed_out += 1;
                            let penalty = Duration::from_millis(750 * resolver.timed_out.min(4));
                            resolver.penalized_until = Instant::now() + penalty;
                            if let QueryKind::Uplink(seq) = meta.kind {
                                if let Some(chunk) = pending.get_mut(&seq) {
                                    chunk.last_sent = Instant::now() - config.request_timeout;
                                }
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
                if now.duration_since(meta.sent_at) >= config.request_timeout {
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
                        chunk.last_sent = Instant::now() - config.request_timeout;
                    }
                }
            }
        }

        while in_flight.len() < WINDOW_SIZE {
            let candidate = pick_chunk(&mut pending, config.request_timeout);
            let down_request = pick_down_request(down_next, &down_pending, &in_flight);
            let should_poll = candidate.is_none()
                && (down_request.is_some()
                    || (pending.is_empty()
                        && (in_flight.len() < POLL_WINDOW
                            || now.duration_since(last_send) >= config.keep_alive_interval)));
            let should_fin = local_closed && pending.is_empty() && !read_closed_sent;
            if candidate.is_none() && down_request.is_none() && !should_poll && !should_fin {
                break;
            }
            let resolver_index = pick_resolver(&resolvers).context("no resolvers")?;
            if resolvers[resolver_index].in_flight >= MAX_INFLIGHT_PER_RESOLVER {
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
            let request_id = thread_rng().gen::<u32>();
            let request = crate::protocol::RequestPacket {
                request_id,
                session_id,
                flags,
                down_ack: down_next,
                seq,
                payload,
            };
            let (dns_id, wire) = build_query(&request, &config.domain)?;
            resolvers[resolver_index].socket.send(&wire).await?;
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

async fn probe_resolver(socket: &UdpSocket, domain: &str, timeout: Duration) -> Result<Duration> {
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

pub fn parse_socket_addr(value: &str, default_port: u16) -> Result<SocketAddr> {
    if let Ok(addr) = value.parse() {
        return Ok(addr);
    }
    let combined = format!("{value}:{default_port}");
    combined
        .parse()
        .with_context(|| format!("invalid socket address {value}"))
}

pub fn default_client_config(listen: SocketAddr, resolvers: Vec<SocketAddr>, domain: String) -> ClientConfig {
    ClientConfig {
        listen,
        resolvers,
        domain,
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
