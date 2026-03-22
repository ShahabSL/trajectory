use crate::auth::{ClientAccessKey, AUTH_TAG_LEN};
use crate::protocol::{
    build_probe_query, build_query, max_query_payload_for_domain, parse_dns_id,
    parse_response_meta, FLAG_DATA, FLAG_DOWNLINK, FLAG_FIN,
};
use anyhow::{bail, Context, Result};
use rand::{thread_rng, Rng};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinSet;

pub const DEFAULT_KEEPALIVE_MS: u64 = 100;

const CLIENT_WINDOW_SIZE: usize = 200;
const CLIENT_DOWNLINK_WINDOW: usize = 40;
const CLIENT_QUERY_TIMEOUT_MS: u64 = 250;
const CLIENT_MIN_QUERY_TIMEOUT_MS: u64 = 200;
const CLIENT_MAX_QUERY_TIMEOUT_MS: u64 = 2_000;
const CLIENT_MAX_INFLIGHT_PER_RESOLVER: usize = 64;
const CLIENT_MAX_SELECTED_RESOLVERS: usize = 4;
const CLIENT_RESOLVER_DEMOTE_AFTER: u32 = 2;
const CLIENT_RESOLVER_QUARANTINE_AFTER: u32 = 4;
const CLIENT_MAX_RESOLVER_PENALTY_MS: u64 = 15_000;
const CLIENT_REPROBE_INTERVAL_MS: u64 = 8_000;
const CLIENT_DEGRADED_REPROBE_INTERVAL_MS: u64 = 2_000;
const CLIENT_REPLACEMENT_MARGIN_US: u64 = 75_000;

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
    addr: SocketAddr,
    transport: ResolverTransport,
    label: String,
    generation: u64,
    srtt: Duration,
    in_flight: usize,
    sent: u64,
    timed_out: u64,
    consecutive_failures: u32,
    penalized_until: Instant,
}

#[derive(Clone)]
enum ResolverTransport {
    Udp { socket: Arc<UdpSocket> },
    Tcp { addr: SocketAddr },
}

#[derive(Clone, Copy)]
struct ResolverProfile {
    srtt: Duration,
    timed_out: u64,
    consecutive_failures: u32,
    penalized_until: Instant,
}

struct ProbedResolver {
    addr: SocketAddr,
    transport: ResolverTransport,
    label: String,
    srtt: Duration,
    prior: Option<ResolverProfile>,
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
    Packet {
        resolver_index: usize,
        generation: u64,
        bytes: Vec<u8>,
    },
    SendFailure {
        resolver_index: usize,
        generation: u64,
        dns_id: u16,
        message: String,
    },
}

struct ClientRuntime {
    active_sessions: AtomicUsize,
    resolver_profiles: Mutex<HashMap<SocketAddr, ResolverProfile>>,
}

struct ActiveSessionGuard(Arc<ClientRuntime>);

impl ActiveSessionGuard {
    fn new(runtime: Arc<ClientRuntime>) -> Self {
        runtime.active_sessions.fetch_add(1, Ordering::Relaxed);
        Self(runtime)
    }
}

impl Drop for ActiveSessionGuard {
    fn drop(&mut self) {
        self.0.active_sessions.fetch_sub(1, Ordering::Relaxed);
    }
}

fn spawn_udp_listener(
    socket: Arc<UdpSocket>,
    resolver_index: usize,
    generation: u64,
    tx: mpsc::UnboundedSender<ResolverEvent>,
) {
    tokio::spawn(async move {
        let mut buf = vec![0u8; 2048];
        loop {
            match socket.recv(&mut buf).await {
                Ok(len) => {
                    if tx
                        .send(ResolverEvent::Packet {
                            resolver_index,
                            generation,
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
        ResolverTransport::Udp { .. } => adaptive_inflight_for(resolver),
        ResolverTransport::Tcp { .. } => 2,
    }
}

pub async fn run(config: ClientConfig) -> Result<()> {
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    run_until(config, shutdown_rx).await
}

pub async fn run_until(config: ClientConfig, mut shutdown_rx: watch::Receiver<bool>) -> Result<()> {
    let listener = TcpListener::bind(config.listen).await?;
    let runtime = Arc::new(ClientRuntime {
        active_sessions: AtomicUsize::new(0),
        resolver_profiles: Mutex::new(HashMap::new()),
    });
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
        let runtime = runtime.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_stream(stream, config, runtime).await {
                if !is_expected_local_close(&error) {
                    eprintln!("client session error: {error:#}");
                }
            }
        });
    }
}

async fn handle_stream(
    stream: TcpStream,
    config: ClientConfig,
    runtime: Arc<ClientRuntime>,
) -> Result<()> {
    let session_id = thread_rng().gen::<u64>();
    let _active_session = ActiveSessionGuard::new(runtime.clone());
    let (mut tcp_reader, mut tcp_writer) = stream.into_split();
    debug_client_log(format!(
        "accepted session {session_id:016x} for domain {} via {} resolvers",
        config.domain,
        config.resolvers.len()
    ));
    let max_query_payload = max_query_payload_for_domain(&config.domain)?;

    let mut probes = JoinSet::new();
    let (resp_tx, mut resp_rx) = mpsc::unbounded_channel::<ResolverEvent>();
    for &addr in &config.resolvers {
        let domain = config.domain.clone();
        let probe_timeout = config.request_timeout;
        let prior = snapshot_resolver_profile(&runtime, addr);
        probes.spawn(async move { probe_resolver(addr, domain, probe_timeout, prior).await });
    }

    let mut probed = Vec::with_capacity(config.resolvers.len());
    while let Some(result) = probes.join_next().await {
        match result {
            Ok(Ok(candidate)) => {
                debug_client_log(format!(
                    "resolver {} probe candidate {} in {} ms",
                    candidate.addr,
                    candidate.label,
                    candidate.srtt.as_millis()
                ));
                probed.push(candidate);
            }
            Ok(Err(error)) => debug_client_log(format!("resolver probe failed: {error:#}")),
            Err(error) => debug_client_log(format!("resolver probe task failed: {error:#}")),
        }
    }

    let selected = select_resolvers(probed);
    if selected.is_empty() {
        bail!("no active resolvers after probe");
    }

    let mut next_generation = 0u64;
    let mut resolvers = Vec::with_capacity(selected.len());
    for (index, candidate) in selected.into_iter().enumerate() {
        let ProbedResolver {
            addr,
            transport,
            srtt,
            label,
            prior,
        } = candidate;
        if let ResolverTransport::Udp { socket } = &transport {
            next_generation += 1;
            spawn_udp_listener(socket.clone(), index, next_generation, resp_tx.clone());
        } else {
            next_generation += 1;
        }
        let resolver = Resolver {
            addr,
            transport,
            label,
            generation: next_generation,
            srtt: prime_resolver_srtt(srtt, prior),
            in_flight: 0,
            sent: 0,
            timed_out: prior.map_or(0, |profile| profile.timed_out.saturating_sub(1)),
            consecutive_failures: 0,
            penalized_until: Instant::now(),
        };
        resolvers.push(resolver);
    }
    debug_client_log(format!(
        "session {session_id:016x} selected {} active resolvers",
        resolvers.len()
    ));
    let mut reprobes = JoinSet::new();
    let mut reprobe_inflight = HashSet::new();
    let mut next_reprobe_at = Instant::now() + Duration::from_millis(CLIENT_REPROBE_INTERVAL_MS);
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

        while let Some(result) = reprobes.try_join_next() {
            match result {
                Ok((addr, Ok(candidate))) => {
                    reprobe_inflight.remove(&addr);
                    maybe_refresh_active_resolver(
                        &candidate,
                        &mut resolvers,
                        &runtime,
                        &resp_tx,
                        &mut next_generation,
                    );
                    maybe_replace_degraded_resolver(
                        candidate,
                        &mut resolvers,
                        &runtime,
                        &resp_tx,
                        &mut next_generation,
                    );
                }
                Ok((addr, Err(error))) => {
                    reprobe_inflight.remove(&addr);
                    debug_client_log(format!("background resolver probe failed: {error:#}"));
                }
                Err(error) => {
                    debug_client_log(format!("background resolver probe task failed: {error:#}"));
                }
            }
        }

        while let Ok(event) = resp_rx.try_recv() {
            match event {
                ResolverEvent::Packet {
                    resolver_index,
                    generation,
                    bytes,
                } => match parse_response_meta(&bytes) {
                    Ok((dns_id, maybe_response)) => {
                        if resolvers[resolver_index].generation != generation {
                            continue;
                        }
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
                                        mark_resolver_failure(resolver, Instant::now());
                                        persist_resolver_profile(&runtime, resolver);
                                        continue;
                                    }
                                    resolver.sent += 1;
                                    let rtt = meta.sent_at.elapsed();
                                    mark_resolver_success(resolver, Instant::now(), rtt);
                                    persist_resolver_profile(&runtime, resolver);
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
                                        resolver_label, meta.kind
                                    ));
                                    mark_resolver_failure(resolver, Instant::now());
                                    persist_resolver_profile(&runtime, resolver);
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
                    generation,
                    dns_id,
                    message,
                } => {
                    if resolvers[resolver_index].generation != generation {
                        continue;
                    }
                    debug_client_log(format!(
                        "resolver {} request {} failed: {}",
                        resolvers[resolver_index].label, dns_id, message
                    ));
                    if let Some(meta) = in_flight.remove(&dns_id) {
                        let resolver = &mut resolvers[resolver_index];
                        resolver.in_flight = resolver.in_flight.saturating_sub(1);
                        mark_resolver_failure(resolver, Instant::now());
                        persist_resolver_profile(&runtime, resolver);
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
        if now >= next_reprobe_at {
            schedule_background_reprobes(
                &config,
                &runtime,
                &resolvers,
                &mut reprobes,
                &mut reprobe_inflight,
            );
            next_reprobe_at = next_reprobe_deadline(&resolvers, now);
        }
        let timed_out: Vec<u16> = in_flight
            .iter()
            .filter_map(|(request_id, meta)| {
                let timeout = match resolvers.get(meta.resolver) {
                    Some(resolver) => {
                        timeout_for_resolver(resolver, config.request_timeout, request_timeout)
                    }
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
                mark_resolver_failure(resolver, Instant::now());
                persist_resolver_profile(&runtime, resolver);
                if let QueryKind::Uplink(seq) = meta.kind {
                    if let Some(chunk) = pending.get_mut(&seq) {
                        chunk.last_sent = Instant::now() - request_timeout;
                    }
                }
            }
        }

        let active_sessions = runtime.active_sessions.load(Ordering::Relaxed).max(1);
        let window_limit = fair_share_limit(CLIENT_WINDOW_SIZE, active_sessions, 12);
        let poll_window = fair_share_limit(CLIENT_WINDOW_SIZE, active_sessions, 1);
        while in_flight.len() < window_limit {
            let candidate = pick_chunk(&mut pending, request_timeout);
            let down_request = if session_bootstrapped {
                pick_down_request(down_next, &down_pending, &in_flight)
            } else {
                None
            };
            let should_poll =
                candidate.is_none()
                    && session_bootstrapped
                    && (down_request.is_some()
                        || (pending.is_empty()
                            && (in_flight.len() < poll_window
                                || now.duration_since(last_send)
                                    >= config
                                        .keep_alive_interval
                                        .mul_f32(if active_sessions > 1 { 4.0 } else { 1.0 }))));
            let should_fin = local_closed && pending.is_empty() && !read_closed_sent;
            if candidate.is_none() && down_request.is_none() && !should_poll && !should_fin {
                break;
            }
            let resolver_index = pick_resolver(&resolvers).context("no resolvers")?;
            let resolver_limit = fair_share_limit(
                max_inflight_for(&resolvers[resolver_index]),
                active_sessions,
                1,
            );
            if resolvers[resolver_index].in_flight >= resolver_limit {
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
            } else if should_poll {
                (0, down_next, Vec::new(), QueryKind::KeepAlive)
            } else {
                break;
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
                kind, dns_id, resolvers[resolver_index].label
            ));
            let resolver_generation = resolvers[resolver_index].generation;
            send_query(
                &resolvers[resolver_index],
                resolver_index,
                resolver_generation,
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

fn schedule_background_reprobes(
    config: &ClientConfig,
    runtime: &Arc<ClientRuntime>,
    resolvers: &[Resolver],
    reprobes: &mut JoinSet<(SocketAddr, Result<ProbedResolver>)>,
    reprobe_inflight: &mut HashSet<SocketAddr>,
) {
    let active_addrs: HashSet<_> = resolvers.iter().map(|resolver| resolver.addr).collect();
    let probe_all = resolvers.iter().any(|resolver| {
        resolver.consecutive_failures >= CLIENT_RESOLVER_DEMOTE_AFTER
            || resolver.srtt >= Duration::from_millis(400)
    });
    for &addr in &config.resolvers {
        if !probe_all && active_addrs.contains(&addr) {
            continue;
        }
        if !reprobe_inflight.insert(addr) {
            continue;
        }
        let domain = config.domain.clone();
        let timeout = config.request_timeout;
        let prior = snapshot_resolver_profile(runtime, addr);
        reprobes.spawn(async move { (addr, probe_resolver(addr, domain, timeout, prior).await) });
    }
}

fn next_reprobe_deadline(resolvers: &[Resolver], now: Instant) -> Instant {
    let interval_ms = if resolvers
        .iter()
        .any(|resolver| resolver.consecutive_failures >= CLIENT_RESOLVER_DEMOTE_AFTER)
    {
        CLIENT_DEGRADED_REPROBE_INTERVAL_MS
    } else {
        CLIENT_REPROBE_INTERVAL_MS
    };
    now + Duration::from_millis(interval_ms)
}

fn maybe_refresh_active_resolver(
    candidate: &ProbedResolver,
    resolvers: &mut [Resolver],
    runtime: &Arc<ClientRuntime>,
    resp_tx: &mpsc::UnboundedSender<ResolverEvent>,
    next_generation: &mut u64,
) {
    let Some(index) = resolvers
        .iter()
        .position(|resolver| resolver.addr == candidate.addr)
    else {
        return;
    };
    let now = Instant::now();
    let resolver = &mut resolvers[index];
    if resolver.in_flight > 0 {
        mark_resolver_success(resolver, now, candidate.srtt);
        persist_resolver_profile(runtime, resolver);
        return;
    }
    let current_score = active_resolver_sort_key(resolver, now);
    let candidate_score = probe_candidate_sort_key(
        &ProbeCandidateMeta {
            is_udp: matches!(candidate.transport, ResolverTransport::Udp { .. }),
            srtt: candidate.srtt,
            prior: candidate.prior,
        },
        now,
        true,
    );
    let transport_changed = matches!(resolver.transport, ResolverTransport::Tcp { .. })
        != matches!(candidate.transport, ResolverTransport::Tcp { .. });
    if transport_changed
        || candidate_score.saturating_add(CLIENT_REPLACEMENT_MARGIN_US) < current_score
        || resolver.consecutive_failures >= CLIENT_RESOLVER_DEMOTE_AFTER
    {
        debug_client_log(format!(
            "refreshing active resolver slot {} with new probe for {}",
            index, candidate.label
        ));
        install_resolver_candidate(
            resolvers,
            index,
            candidate,
            resp_tx,
            next_generation,
            runtime,
        );
    } else {
        mark_resolver_success(resolver, now, candidate.srtt);
        persist_resolver_profile(runtime, resolver);
    }
}

fn maybe_replace_degraded_resolver(
    candidate: ProbedResolver,
    resolvers: &mut [Resolver],
    runtime: &Arc<ClientRuntime>,
    resp_tx: &mpsc::UnboundedSender<ResolverEvent>,
    next_generation: &mut u64,
) {
    if resolvers
        .iter()
        .any(|resolver| resolver.addr == candidate.addr)
    {
        return;
    }
    let now = Instant::now();
    let candidate_score = probe_candidate_sort_key(
        &ProbeCandidateMeta {
            is_udp: matches!(candidate.transport, ResolverTransport::Udp { .. }),
            srtt: candidate.srtt,
            prior: candidate.prior,
        },
        now,
        true,
    );
    let Some((index, active_score)) = replacement_slot(resolvers, now) else {
        return;
    };
    let resolver = &resolvers[index];
    let clearly_better =
        candidate_score.saturating_add(CLIENT_REPLACEMENT_MARGIN_US) < active_score;
    let active_degraded = resolver.consecutive_failures >= CLIENT_RESOLVER_DEMOTE_AFTER
        || resolver.srtt >= Duration::from_millis(400);
    if clearly_better || (active_degraded && candidate_score < active_score) {
        debug_client_log(format!(
            "replacing degraded resolver {} with standby {}",
            resolver.label, candidate.label
        ));
        install_resolver_candidate(
            resolvers,
            index,
            &candidate,
            resp_tx,
            next_generation,
            runtime,
        );
    }
}

fn replacement_slot(resolvers: &[Resolver], now: Instant) -> Option<(usize, u64)> {
    resolvers
        .iter()
        .enumerate()
        .filter(|(_, resolver)| resolver.in_flight == 0)
        .map(|(index, resolver)| (index, active_resolver_sort_key(resolver, now)))
        .max_by_key(|(_, score)| *score)
}

fn active_resolver_sort_key(resolver: &Resolver, now: Instant) -> u64 {
    probe_candidate_sort_key(
        &ProbeCandidateMeta {
            is_udp: matches!(resolver.transport, ResolverTransport::Udp { .. }),
            srtt: resolver.srtt,
            prior: Some(ResolverProfile {
                srtt: resolver.srtt,
                timed_out: resolver.timed_out,
                consecutive_failures: resolver.consecutive_failures,
                penalized_until: resolver.penalized_until,
            }),
        },
        now,
        true,
    )
}

fn install_resolver_candidate(
    resolvers: &mut [Resolver],
    index: usize,
    candidate: &ProbedResolver,
    resp_tx: &mpsc::UnboundedSender<ResolverEvent>,
    next_generation: &mut u64,
    runtime: &Arc<ClientRuntime>,
) {
    *next_generation += 1;
    let generation = *next_generation;
    if let ResolverTransport::Udp { socket } = &candidate.transport {
        spawn_udp_listener(socket.clone(), index, generation, resp_tx.clone());
    }
    resolvers[index] = Resolver {
        addr: candidate.addr,
        transport: candidate.transport.clone(),
        label: candidate.label.clone(),
        generation,
        srtt: prime_resolver_srtt(candidate.srtt, candidate.prior),
        in_flight: 0,
        sent: 0,
        timed_out: candidate
            .prior
            .map_or(0, |profile| profile.timed_out.saturating_sub(1)),
        consecutive_failures: 0,
        penalized_until: Instant::now(),
    };
    persist_resolver_profile(runtime, &resolvers[index]);
}

fn pick_chunk(pending: &mut BTreeMap<u32, UplinkChunk>, timeout: Duration) -> Option<u32> {
    let now = Instant::now();
    pending
        .iter()
        .find_map(|(seq, chunk)| (now.duration_since(chunk.last_sent) >= timeout).then_some(*seq))
}

fn snapshot_resolver_profile(
    runtime: &Arc<ClientRuntime>,
    addr: SocketAddr,
) -> Option<ResolverProfile> {
    runtime
        .resolver_profiles
        .lock()
        .ok()
        .and_then(|profiles| profiles.get(&addr).copied())
}

fn persist_resolver_profile(runtime: &Arc<ClientRuntime>, resolver: &Resolver) {
    if let Ok(mut profiles) = runtime.resolver_profiles.lock() {
        profiles.insert(
            resolver.addr,
            ResolverProfile {
                srtt: resolver.srtt,
                timed_out: resolver.timed_out,
                consecutive_failures: resolver.consecutive_failures,
                penalized_until: resolver.penalized_until,
            },
        );
    }
}

fn prime_resolver_srtt(srtt: Duration, prior: Option<ResolverProfile>) -> Duration {
    prior.map_or(srtt, |profile| blend_rtt(profile.srtt, srtt))
}

fn select_resolvers(probed: Vec<ProbedResolver>) -> Vec<ProbedResolver> {
    let now = Instant::now();
    let metadata: Vec<_> = probed
        .iter()
        .map(|candidate| ProbeCandidateMeta {
            is_udp: matches!(candidate.transport, ResolverTransport::Udp { .. }),
            srtt: candidate.srtt,
            prior: candidate.prior,
        })
        .collect();
    let selected = select_resolver_indices(&metadata, now);
    let selected: std::collections::HashSet<_> = selected.into_iter().collect();
    probed
        .into_iter()
        .enumerate()
        .filter_map(|(index, candidate)| selected.contains(&index).then_some(candidate))
        .collect()
}

#[derive(Clone, Copy)]
struct ProbeCandidateMeta {
    is_udp: bool,
    srtt: Duration,
    prior: Option<ResolverProfile>,
}

fn select_resolver_indices(metadata: &[ProbeCandidateMeta], now: Instant) -> Vec<usize> {
    let has_viable_udp = metadata
        .iter()
        .any(|candidate| candidate.is_udp && candidate.srtt < Duration::from_millis(2_000));
    let mut candidates: Vec<_> = metadata
        .iter()
        .enumerate()
        .filter(|(_, candidate)| {
            if has_viable_udp {
                candidate.is_udp && candidate.srtt < Duration::from_millis(2_000)
            } else {
                candidate.srtt < Duration::from_millis(5_000)
            }
        })
        .collect();
    candidates
        .sort_by_key(|(_, candidate)| probe_candidate_sort_key(candidate, now, has_viable_udp));
    let limit = if has_viable_udp {
        CLIENT_MAX_SELECTED_RESOLVERS
    } else {
        1
    };
    candidates
        .into_iter()
        .take(limit.max(1))
        .map(|(index, _)| index)
        .collect()
}

fn probe_candidate_sort_key(candidate: &ProbeCandidateMeta, now: Instant, prefer_udp: bool) -> u64 {
    let cooldown = candidate
        .prior
        .and_then(|profile| profile.penalized_until.checked_duration_since(now))
        .unwrap_or_default()
        .as_micros() as u64;
    let timed_out = candidate
        .prior
        .map_or(0, |profile| profile.timed_out.min(8));
    let failures = candidate
        .prior
        .map_or(0, |profile| profile.consecutive_failures.min(8) as u64);
    candidate.srtt.as_micros() as u64
        + cooldown
        + timed_out.saturating_mul(125_000)
        + failures.saturating_mul(500_000)
        + if prefer_udp && !candidate.is_udp {
            3_000_000
        } else {
            0
        }
}

fn pick_resolver(resolvers: &[Resolver]) -> Result<usize> {
    let now = Instant::now();
    let healthy: Vec<_> = resolvers
        .iter()
        .enumerate()
        .filter(|(_, resolver)| resolver.penalized_until <= now)
        .collect();
    let candidates = if healthy.is_empty() {
        resolvers.iter().enumerate().collect()
    } else {
        healthy
    };
    candidates
        .into_iter()
        .min_by_key(|(_, resolver)| {
            let cooldown = resolver
                .penalized_until
                .checked_duration_since(now)
                .unwrap_or_default()
                .as_micros() as u64;
            let inflight_cap = adaptive_inflight_for(resolver) as u64;
            let inflight_penalty = if resolver.in_flight as u64 >= inflight_cap {
                5_000_000
            } else {
                0
            };
            resolver.srtt.as_micros() as u64 * (resolver.in_flight as u64 + 1)
                + resolver.consecutive_failures.min(8) as u64 * 500_000
                + resolver.timed_out.min(8).saturating_mul(125_000)
                + cooldown
                + inflight_penalty
        })
        .map(|(index, _)| index)
        .context("no resolvers configured")
}

fn blend_rtt(old: Duration, sample: Duration) -> Duration {
    let old_us = old.as_micros() as u64;
    let sample_us = sample.as_micros() as u64;
    Duration::from_micros((old_us * 7 + sample_us) / 8)
}

fn clamp_duration(value: Duration, min: Duration, max: Duration) -> Duration {
    value.max(min).min(max)
}

fn timeout_for_resolver(
    resolver: &Resolver,
    udp_floor: Duration,
    tcp_timeout: Duration,
) -> Duration {
    match resolver.transport {
        ResolverTransport::Udp { .. } => {
            let adaptive = resolver
                .srtt
                .saturating_mul(4)
                .saturating_add(Duration::from_millis(
                    50 + u64::from(resolver.consecutive_failures.min(8)) * 50,
                ));
            clamp_duration(
                adaptive.max(udp_floor),
                Duration::from_millis(CLIENT_MIN_QUERY_TIMEOUT_MS),
                Duration::from_millis(CLIENT_MAX_QUERY_TIMEOUT_MS),
            )
        }
        ResolverTransport::Tcp { .. } => tcp_timeout,
    }
}

fn adaptive_inflight_for(resolver: &Resolver) -> usize {
    if resolver.consecutive_failures >= CLIENT_RESOLVER_QUARANTINE_AFTER {
        return 1;
    }
    if resolver.consecutive_failures >= CLIENT_RESOLVER_DEMOTE_AFTER {
        return 4;
    }
    if resolver.srtt >= Duration::from_millis(600) {
        return 6;
    }
    if resolver.srtt >= Duration::from_millis(350) {
        return 10;
    }
    if resolver.srtt >= Duration::from_millis(200) {
        return 16;
    }
    if resolver.srtt >= Duration::from_millis(120) {
        return 24;
    }
    CLIENT_MAX_INFLIGHT_PER_RESOLVER
}

fn resolver_penalty(resolver: &Resolver) -> Duration {
    resolver_penalty_for_failures(resolver.consecutive_failures)
}

fn resolver_penalty_for_failures(consecutive_failures: u32) -> Duration {
    let multiplier = 1u64 << consecutive_failures.min(6);
    Duration::from_millis((250 * multiplier).min(CLIENT_MAX_RESOLVER_PENALTY_MS))
}

fn mark_resolver_success(resolver: &mut Resolver, now: Instant, rtt: Duration) {
    resolver.timed_out = resolver.timed_out.saturating_sub(1);
    resolver.consecutive_failures = 0;
    resolver.penalized_until = now;
    resolver.srtt = blend_rtt(resolver.srtt, rtt);
}

fn mark_resolver_failure(resolver: &mut Resolver, now: Instant) {
    resolver.timed_out = resolver.timed_out.saturating_add(1);
    resolver.consecutive_failures = resolver.consecutive_failures.saturating_add(1);
    resolver.penalized_until = now + resolver_penalty(resolver);
}

fn fair_share_limit(limit: usize, active_sessions: usize, min_limit: usize) -> usize {
    if active_sessions <= 1 {
        return limit;
    }
    (limit / active_sessions).max(min_limit).min(limit)
}

fn pick_down_request(
    down_next: u32,
    down_pending: &BTreeMap<u32, Vec<u8>>,
    in_flight: &HashMap<u16, QueryMeta>,
) -> Option<u32> {
    let max_seq = down_next.saturating_add(CLIENT_DOWNLINK_WINDOW as u32);
    (down_next..max_seq).find(|seq| {
        !down_pending.contains_key(seq)
            && !in_flight.values().any(
                |meta| matches!(meta.kind, QueryKind::Downlink(requested) if requested == *seq),
            )
    })
}

async fn probe_udp_resolver(
    socket: &UdpSocket,
    domain: &str,
    timeout: Duration,
) -> Result<Duration> {
    let (dns_id, wire) = build_probe_query(domain)?;
    let started = Instant::now();
    socket.send(&wire).await?;
    let deadline = Instant::now() + timeout;
    let mut buf = vec![0u8; 2048];
    loop {
        let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
            bail!("deadline has elapsed");
        };
        let len = tokio::time::timeout(remaining, socket.recv(&mut buf)).await??;
        let received_id = parse_dns_id(&buf[..len])?;
        if received_id == dns_id {
            return Ok(started.elapsed());
        }
    }
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

async fn probe_resolver(
    addr: SocketAddr,
    domain: String,
    timeout: Duration,
    prior: Option<ResolverProfile>,
) -> Result<ProbedResolver> {
    let socket = bind_probe_socket(addr).await?;
    match probe_udp_resolver(&socket, &domain, timeout).await {
        Ok(srtt) => Ok(ProbedResolver {
            addr,
            transport: ResolverTransport::Udp { socket },
            label: format!("{addr} (udp)"),
            srtt,
            prior,
        }),
        Err(udp_error) => {
            match probe_tcp_resolver(addr, &domain, tcp_resolver_timeout(timeout)).await {
                Ok(srtt) => Ok(ProbedResolver {
                    addr,
                    transport: ResolverTransport::Tcp { addr },
                    label: format!("{addr} (tcp)"),
                    srtt,
                    prior,
                }),
                Err(tcp_error) => {
                    debug_client_log(format!(
                    "resolver {addr} probe failed over udp ({udp_error:#}) and tcp ({tcp_error:#})"
                ));
                    Ok(ProbedResolver {
                        addr,
                        transport: ResolverTransport::Udp { socket },
                        label: format!("{addr} (udp)"),
                        srtt: prior.map_or(Duration::from_millis(2_000), |profile| {
                            profile.srtt.max(Duration::from_millis(2_000))
                        }),
                        prior,
                    })
                }
            }
        }
    }
}

async fn bind_probe_socket(addr: SocketAddr) -> Result<Arc<UdpSocket>> {
    let bind_addr = match addr {
        SocketAddr::V4(v4) if v4.ip().is_loopback() => "127.0.0.1:0",
        SocketAddr::V4(_) => "0.0.0.0:0",
        SocketAddr::V6(v6) if v6.ip().is_loopback() => "[::1]:0",
        SocketAddr::V6(_) => "[::]:0",
    };
    for _ in 0..8 {
        let socket = match UdpSocket::bind(bind_addr).await {
            Ok(socket) => Arc::new(socket),
            Err(error) if error.kind() == ErrorKind::AddrInUse => continue,
            Err(error) => return Err(error.into()),
        };
        match socket.connect(addr).await {
            Ok(()) => return Ok(socket),
            Err(error) if error.kind() == ErrorKind::AddrInUse => continue,
            Err(error) => return Err(error.into()),
        }
    }
    bail!("failed to bind a probe socket without colliding with {addr}");
}

async fn send_query(
    resolver: &Resolver,
    resolver_index: usize,
    generation: u64,
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
                            generation,
                            bytes,
                        });
                    }
                    Err(error) => {
                        let _ = resp_tx.send(ResolverEvent::SendFailure {
                            resolver_index,
                            generation,
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
        keep_alive_interval: Duration::from_millis(DEFAULT_KEEPALIVE_MS),
        request_timeout: Duration::from_millis(CLIENT_QUERY_TIMEOUT_MS),
    }
}

pub fn default_public_resolvers() -> Vec<SocketAddr> {
    [
        "1.1.1.1:53",
        "1.0.0.1:53",
        "8.8.8.8:53",
        "8.8.4.4:53",
        "9.9.9.9:53",
    ]
    .into_iter()
    .map(|value| value.parse().expect("valid built-in resolver address"))
    .collect()
}

pub fn require_resolvers(resolvers: &[SocketAddr]) -> Result<()> {
    if resolvers.is_empty() {
        bail!("missing required --resolver");
    }
    Ok(())
}

fn is_expected_local_close(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause
            .downcast_ref::<std::io::Error>()
            .is_some_and(|io_error| {
                matches!(
                    io_error.kind(),
                    ErrorKind::BrokenPipe
                        | ErrorKind::ConnectionReset
                        | ErrorKind::ConnectionAborted
                        | ErrorKind::UnexpectedEof
                        | ErrorKind::NotConnected
                )
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_public_resolvers_contains_standard_public_set() {
        let resolvers = default_public_resolvers();
        assert_eq!(resolvers.len(), 5);
        assert_eq!(resolvers[0], "1.1.1.1:53".parse().unwrap());
        assert_eq!(resolvers[4], "9.9.9.9:53".parse().unwrap());
    }

    #[test]
    fn select_resolver_indices_uses_full_cohort_and_drops_worst_candidate() {
        let now = Instant::now();
        let metadata = vec![
            ProbeCandidateMeta {
                is_udp: true,
                srtt: Duration::from_millis(25),
                prior: None,
            },
            ProbeCandidateMeta {
                is_udp: true,
                srtt: Duration::from_millis(30),
                prior: None,
            },
            ProbeCandidateMeta {
                is_udp: true,
                srtt: Duration::from_millis(35),
                prior: None,
            },
            ProbeCandidateMeta {
                is_udp: true,
                srtt: Duration::from_millis(40),
                prior: None,
            },
            ProbeCandidateMeta {
                is_udp: true,
                srtt: Duration::from_millis(900),
                prior: Some(ResolverProfile {
                    srtt: Duration::from_millis(900),
                    timed_out: 6,
                    consecutive_failures: 4,
                    penalized_until: now + Duration::from_secs(10),
                }),
            },
        ];
        let selected = select_resolver_indices(&metadata, now);
        assert_eq!(selected, vec![0, 1, 2, 3]);
    }

    #[test]
    fn select_resolver_indices_falls_back_to_fastest_candidate_when_udp_is_bad() {
        let now = Instant::now();
        let metadata = vec![
            ProbeCandidateMeta {
                is_udp: true,
                srtt: Duration::from_secs(3),
                prior: None,
            },
            ProbeCandidateMeta {
                is_udp: false,
                srtt: Duration::from_millis(120),
                prior: None,
            },
            ProbeCandidateMeta {
                is_udp: false,
                srtt: Duration::from_millis(240),
                prior: None,
            },
        ];
        let selected = select_resolver_indices(&metadata, now);
        assert_eq!(selected, vec![1]);
    }
}
