use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{mpsc, oneshot, Mutex, Semaphore};
use tokio::task::JoinSet;
use tokio::time::timeout;
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::codec::{
    open_packet_with_key, open_packet_with_registry, seal_packet, sealed_packet_len,
    sealed_packet_len_with_extra_frame, AckRange, Direction, Frame, Packet, StreamRange,
};
use trajectory_core::dns::{
    build_a_response, build_aaaa_response, build_empty_response, build_ns_response, build_query,
    build_soa_response, build_txt_response, envelope_to_qname, parse_query, parse_txt_response,
    qname_to_envelope, txt_response_wire_len, CLASS_IN, TYPE_A, TYPE_AAAA, TYPE_NS, TYPE_SOA,
    TYPE_TXT,
};
use trajectory_core::engine::{
    ack_ranges_contain, PacketHistory, RetainedByteSendBuffer, SendBufferMode, SendBufferSlice,
    StreamAssembler,
};

const UPLOAD_READ_CHUNK: usize = 4096;
const UPLOAD_SEND_CHUNK_NORMAL: usize = 4096;
const UPLOAD_SEND_CHUNK_CONSTRAINED: usize = 4096;
const CLIENT_INFLIGHT_WINDOW: usize = 128;
const CLIENT_RECEIVE_WINDOW: u64 = 1024 * 1024;
const CLIENT_MAX_ACTIVE_STREAMS: usize = 32;
const SERVER_RECEIVE_WINDOW: u64 = 1024 * 1024;
const CLIENT_STREAM_ACK_RANGES: usize = 8;
const SERVER_STREAM_ACK_RANGES: usize = 8;
const CLIENT_QUERY_TIMEOUT: Duration = Duration::from_secs(20);
const DNS_TCP_WRITE_TIMEOUT: Duration = Duration::from_secs(20);
const SERVER_TCP_READ_TIMEOUT: Duration = Duration::from_secs(20);
const PATH_MIN_CWND: u32 = 2;
const PATH_INITIAL_CWND: u32 = 6;
const PATH_MAX_CWND_UDP: u32 = 96;
const PATH_MAX_CWND_TCP: u32 = 64;
const PROXY_INITIAL_CWND: u32 = 24;
const PROXY_MAX_CWND: u32 = 128;
const PATH_RTO_MIN_UDP: Duration = Duration::from_millis(250);
const PATH_RTO_MIN_TCP: Duration = Duration::from_secs(1);
const PATH_RTO_MAX_UDP: Duration = Duration::from_millis(2_500);
const PATH_RTO_MAX_TCP: Duration = Duration::from_secs(30);
const PATH_LOSS_EWMA_DENOMINATOR: u32 = 32;
const PATH_BULK_LOSSY_PPM: u32 = 50_000;
const PATH_SEVERE_LOSS_PPM: u32 = 80_000;
const PATH_MIN_RESPONSE_BYTES: u16 = 512;
const PATH_MTU_STEP: u16 = 128;
const PATH_MTU_PROBE_SUCCESSES: u32 = 16;
const PATH_INITIAL_RESPONSE_BYTES: u16 = 1232;
const TCP_PROXY_MAX_INFLIGHT: u32 = 128;
const TCP_RESOLVER_QUEUE: usize = 512;
const TCP_RESOLVER_LANES_DIRECT: usize = 2;
const TCP_RESOLVER_LANES_PROXY: usize = 4;
const UDP_RESOLVER_QUEUE: usize = 512;
const TCP_RECONNECT_DELAY: Duration = Duration::from_millis(250);
const SERVER_UPLOAD_QUEUE: usize = 1024;
const SERVER_UPLOAD_COALESCE_BYTES: usize = 4096;
const SERVER_UPLOAD_COALESCE_DELAY: Duration = Duration::from_millis(1);
const SERVER_DOWNLOAD_QUEUE: usize = 1024;
const SERVER_TARGET_READ_CHUNK: usize = 4096;
const SERVER_RETAINED_BYTE_LIMIT: usize = SERVER_DOWNLOAD_QUEUE * SERVER_TARGET_READ_CHUNK;
const SERVER_DOWNLOAD_FRAME_MAX: usize = 4096;
const SERVER_DOWNLOAD_FAIR_FRAME_MAX: usize = 512;
const SERVER_DOWNLOAD_FAIR_FRAME_MIN: usize = 128;
const SERVER_UDP_QUERY_CONCURRENCY: usize = 1024;
const SERVER_RESPONSE_CACHE: usize = 512;
const SERVER_UPLOAD_ACK_REPEAT: Duration = Duration::from_millis(250);
const SERVER_STATE_IDLE_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const SERVER_STATE_CLEANUP_INTERVAL: Duration = Duration::from_secs(30);
const DNS_RESPONSE_SAFETY_MARGIN: usize = 24;
const RESOLVER_FAILURE_QUARANTINE: Duration = Duration::from_secs(20);
const RESOLVER_FAILURE_QUARANTINE_TCP: Duration = Duration::from_secs(90);
const RESOLVER_PROBE_PARALLELISM_UDP: usize = 96;
const RESOLVER_PROBE_PARALLELISM_TCP: usize = 32;
const RESOLVER_TARGET_ADMITTED_UDP: usize = 64;
const RESOLVER_TARGET_ADMITTED_TCP: usize = 32;
const RESOLVER_ADMISSION_SAMPLE_FACTOR: usize = 1;
const RESOLVER_ADMISSION_TIMEOUT: Duration = Duration::from_millis(1_500);
const RESOLVER_ADMISSION_TIMEOUT_TCP: Duration = Duration::from_secs(20);
const RESOLVER_ADMISSION_DEADLINE: Duration = Duration::from_secs(60);
const RESOLVER_ADMISSION_MAX_ELAPSED_UDP: Duration = Duration::from_secs(8);
const RESOLVER_ADMISSION_MAX_ELAPSED_TCP: Duration = Duration::from_secs(14);
const RESOLVER_ADMISSION_MIN_RESPONSE_BPS_UDP: u64 = 128;
const RESOLVER_ADMISSION_MIN_RESPONSE_BPS_TCP: u64 = 64;
const RESOLVER_TCP_PREFERENCE: Duration = Duration::from_secs(5 * 60);
const CLIENT_PING_INFLIGHT_ACTIVE: usize = CLIENT_INFLIGHT_WINDOW / 2;
const CLIENT_PING_INFLIGHT_IDLE: usize = 4;
const CLIENT_GLOBAL_ACTIVE_PING_INFLIGHT: usize = 96;
const CLIENT_GLOBAL_IDLE_PING_INFLIGHT: usize = 2;
const CLIENT_TRANSPORT_EVENT_QUEUE: usize = 1024;
const CLIENT_STREAM_OUTPUT_QUEUE: usize = 256;
const CLIENT_STREAM_PENDING_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const HTTP_PROXY_HEADER_MAX: usize = 16 * 1024;
const HTTP_PROXY_HEADER_TIMEOUT: Duration = Duration::from_secs(15);
const CLIENT_ACTIVE_POLL_INTERVAL: Duration = Duration::from_millis(2);
const CLIENT_ACTIVE_POLL_GRACE: Duration = Duration::from_millis(1_500);
const CLIENT_ACTIVE_POLL_DATA_BUDGET: usize = 12;
const CLIENT_RESET_CLOSE_DELAY: Duration = Duration::from_millis(500);
const CLIENT_POLL_PROXY_HEADROOM: u32 = 4;
const CLIENT_POLL_RESOLVER_HEADROOM: u32 = 1;

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub listen: SocketAddr,
    pub http_listen: Option<SocketAddr>,
    pub resolvers: Vec<SocketAddr>,
    pub domain: String,
    pub access_key: ClientAccessKey,
    pub resolver_socks_proxy: Option<SocketAddr>,
    pub resolver_transport: ResolverTransportMode,
    pub poll_interval: Duration,
    pub dns_max_payload: u16,
    pub admission_report: Option<PathBuf>,
    pub resolver_cohort_size: Option<usize>,
    pub resolver_admission_min: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolverTransportMode {
    Auto,
    Udp,
    Tcp,
}

impl ClientConfig {
    fn tcp_first_resolver_path(&self) -> bool {
        self.resolver_socks_proxy.is_some() || self.resolver_transport == ResolverTransportMode::Tcp
    }

    fn allow_udp_to_tcp_fallback(&self) -> bool {
        self.resolver_socks_proxy.is_none()
            && self.resolver_transport == ResolverTransportMode::Auto
    }
}

#[derive(Clone)]
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub domain: String,
    pub target: SocketAddr,
    pub target_mode: ServerTargetMode,
    pub authorized_clients: Arc<HashMap<u32, ClientAccessKey>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerTargetMode {
    Tcp,
    Socks5Direct,
}

struct ClientRuntime {
    config: ClientConfig,
    tcp_pool: Option<Arc<ResolverPool>>,
    tcp_fallback_pool: Arc<ResolverPool>,
    udp_pool: Arc<UdpResolverPool>,
    resolver_health: Vec<Mutex<ResolverHealth>>,
    proxy_health: Option<Mutex<ProxyHealth>>,
    stream_slots: Arc<Semaphore>,
    active_ping_slots: Arc<Semaphore>,
    idle_ping_slots: Arc<Semaphore>,
    diag: Option<Arc<ClientDiag>>,
}

impl ClientRuntime {
    fn new(config: ClientConfig) -> Self {
        let resolver_count = config.resolvers.len();
        let tcp_pool = config
            .resolver_socks_proxy
            .map(|proxy| Arc::new(ResolverPool::new(Some(proxy))));
        let initial_timeout = if config.tcp_first_resolver_path() {
            Duration::from_secs(8)
        } else {
            Duration::from_millis(1_500)
        };
        let initial_response_bytes = if config.tcp_first_resolver_path() {
            config.dns_max_payload.max(PATH_MIN_RESPONSE_BYTES)
        } else {
            PATH_INITIAL_RESPONSE_BYTES
                .min(config.dns_max_payload)
                .max(PATH_MIN_RESPONSE_BYTES)
        };
        let resolver_health = config
            .resolvers
            .iter()
            .map(|_| Mutex::new(ResolverHealth::new(initial_timeout, initial_response_bytes)))
            .collect();
        let proxy_health = config
            .resolver_socks_proxy
            .map(|_| Mutex::new(ProxyHealth::default()));
        let stream_capacity = CLIENT_MAX_ACTIVE_STREAMS;
        let active_ping_capacity = CLIENT_GLOBAL_ACTIVE_PING_INFLIGHT;
        Self {
            config,
            tcp_pool,
            tcp_fallback_pool: Arc::new(ResolverPool::new(None)),
            udp_pool: Arc::new(UdpResolverPool::new()),
            resolver_health,
            proxy_health,
            stream_slots: Arc::new(Semaphore::new(stream_capacity)),
            active_ping_slots: Arc::new(Semaphore::new(active_ping_capacity)),
            idle_ping_slots: Arc::new(Semaphore::new(CLIENT_GLOBAL_IDLE_PING_INFLIGHT)),
            diag: std::env::var_os("TRAJECTORY_DIAG")
                .map(|_| Arc::new(ClientDiag::new(resolver_count))),
        }
    }

    async fn pick_resolver(
        &self,
        cursor: &mut usize,
        class: ClientSendClass,
    ) -> Option<PathPermit> {
        let now = Instant::now();
        let count = self.config.resolvers.len();
        let start = *cursor % count;
        if let Some(proxy) = &self.proxy_health {
            let proxy = proxy.lock().await;
            let limit = proxy.cwnd.min(TCP_PROXY_MAX_INFLIGHT);
            if proxy.in_flight.saturating_add(class.proxy_headroom()) >= limit
                || (class != ClientSendClass::Poll && proxy.next_send_at > now)
            {
                return None;
            }
        }
        let mut best = None::<(usize, u128, usize)>;
        let mut best_degraded = None::<(usize, u128, usize)>;
        let mut best_blocked = None::<(usize, u128, usize)>;
        let bulk_rtt_cutoff = if class == ClientSendClass::Data && count > 1 {
            self.bulk_rtt_cutoff(start, now).await
        } else {
            None
        };
        for offset in 0..count {
            let index = (start + offset) % count;
            let health = self.resolver_health[index].lock().await;
            let rtt_micros = health
                .srtt
                .unwrap_or_else(|| self.initial_rto())
                .as_micros();
            let blocked_for = health.blocked_until.and_then(|blocked_until| {
                (blocked_until > now).then_some(blocked_until.duration_since(now))
            });
            if health.in_flight.saturating_add(class.resolver_headroom()) >= health.cwnd {
                continue;
            }
            if class != ClientSendClass::Poll && health.next_send_at > now {
                continue;
            }
            let queue_penalty = rtt_micros.saturating_mul((health.in_flight as u128) + 1)
                / health.cwnd.max(1) as u128;
            let response_bytes = self.response_bytes_for_health(&health, now);
            let serialization_penalty = if health.goodput_ewma > 0 {
                (health.in_flight as u128)
                    .saturating_mul(response_bytes as u128)
                    .saturating_mul(1_000_000)
                    / health.goodput_ewma as u128
            } else {
                0
            };
            let failure_penalty = (health.failures as u128).saturating_mul(rtt_micros);
            let loss_penalty = rtt_micros.saturating_mul(health.loss_ewma_ppm as u128) / 20_000;
            let mtu_bonus = (response_bytes as u128).saturating_mul(10);
            let score = rtt_micros
                .saturating_add(queue_penalty)
                .saturating_add(serialization_penalty)
                .saturating_add(failure_penalty)
                .saturating_add(loss_penalty)
                .saturating_add(
                    blocked_for
                        .map(|duration| duration.as_micros().saturating_mul(4))
                        .unwrap_or(0),
                )
                .saturating_sub(mtu_bonus);
            let candidate = (index, score, offset);
            let degraded_for_bulk = class == ClientSendClass::Data
                && count > 1
                && (health.failures > 0
                    || health.loss_ewma_ppm >= PATH_BULK_LOSSY_PPM
                    || bulk_rtt_cutoff
                        .map(|cutoff| rtt_micros > cutoff)
                        .unwrap_or(false));
            let target = if blocked_for.is_some() {
                &mut best_blocked
            } else if degraded_for_bulk {
                &mut best_degraded
            } else {
                &mut best
            };
            if target
                .as_ref()
                .map(|(_, best_score, best_offset)| {
                    (candidate.1, candidate.2) < (*best_score, *best_offset)
                })
                .unwrap_or(true)
            {
                *target = Some(candidate);
            }
        }

        let index = match class {
            ClientSendClass::Control => best
                .or(best_degraded)
                .or(best_blocked)
                .map(|(index, _, _)| index),
            ClientSendClass::Data | ClientSendClass::Poll => best
                .or(best_degraded)
                .or(best_blocked)
                .map(|(index, _, _)| index),
        }?;
        if let Some(proxy) = &self.proxy_health {
            let mut proxy = proxy.lock().await;
            let limit = proxy.cwnd.min(TCP_PROXY_MAX_INFLIGHT);
            if proxy.in_flight.saturating_add(class.proxy_headroom()) >= limit
                || (class != ClientSendClass::Poll && proxy.next_send_at > now)
            {
                return None;
            }
            proxy.in_flight = proxy.in_flight.saturating_add(1);
            if class != ClientSendClass::Poll {
                proxy.next_send_at = now + proxy.pacing_interval;
            }
        }
        {
            let mut health = self.resolver_health[index].lock().await;
            if health.in_flight.saturating_add(class.resolver_headroom()) >= health.cwnd
                || (class != ClientSendClass::Poll && health.next_send_at > now)
            {
                if let Some(proxy) = &self.proxy_health {
                    let mut proxy = proxy.lock().await;
                    proxy.in_flight = proxy.in_flight.saturating_sub(1);
                }
                return None;
            }
            health.in_flight = health.in_flight.saturating_add(1);
            if class != ClientSendClass::Poll {
                health.next_send_at = now + health.pacing_interval;
            }
            let timeout = health.timeout;
            let max_response_bytes = self.response_bytes_for_health(&health, now);
            *cursor = index.wrapping_add(1);
            Some(PathPermit {
                resolver_index: index,
                resolver: self.config.resolvers[index],
                timeout,
                max_response_bytes,
            })
        }
    }

    async fn record_resolver_result(
        &self,
        index: usize,
        ok: bool,
        elapsed: Duration,
        truncated: bool,
        useful_bytes: usize,
        transport: Option<DnsTransportOutcome>,
    ) {
        if let Some(proxy) = &self.proxy_health {
            proxy
                .lock()
                .await
                .record_result(ok, elapsed, self.rto_min(), self.rto_max());
        }
        let mut health = self.resolver_health[index].lock().await;
        health.in_flight = health.in_flight.saturating_sub(1);
        if ok {
            health.failures = 0;
            health.blocked_until = None;
            match transport {
                Some(
                    DnsTransportOutcome::TcpFallbackAfterTruncation
                    | DnsTransportOutcome::TcpAfterUdpFailure
                    | DnsTransportOutcome::TcpPreferred,
                ) => {
                    health.prefer_tcp_until = Some(Instant::now() + RESOLVER_TCP_PREFERENCE);
                }
                Some(DnsTransportOutcome::Udp | DnsTransportOutcome::UdpAfterPreferredTcpError) => {
                    health.prefer_tcp_until = None;
                }
                Some(DnsTransportOutcome::TcpProxy) | None => {}
            }
            health.record_rtt(elapsed, self.rto_min(), self.rto_max());
            health.record_loss_sample(false);
            if truncated {
                health.clean_mtu_successes = 0;
                health.max_response_bytes = health
                    .max_response_bytes
                    .saturating_sub(PATH_MTU_STEP)
                    .max(PATH_MIN_RESPONSE_BYTES);
            } else {
                health.cwnd_successes = health.cwnd_successes.saturating_add(1);
                let growth_threshold = if self.config.tcp_first_resolver_path() {
                    health.cwnd.max(1)
                } else {
                    (health.cwnd / 2).max(4)
                };
                if health.cwnd_successes >= growth_threshold {
                    health.cwnd_successes = 0;
                    health.cwnd = health.cwnd.saturating_add(1).min(self.path_max_cwnd());
                }
                health.clean_mtu_successes = health.clean_mtu_successes.saturating_add(1);
                if health.clean_mtu_successes >= PATH_MTU_PROBE_SUCCESSES {
                    health.clean_mtu_successes = 0;
                    health.max_response_bytes = health
                        .max_response_bytes
                        .saturating_add(PATH_MTU_STEP)
                        .min(self.config.dns_max_payload.max(PATH_MIN_RESPONSE_BYTES));
                }
            }
            if useful_bytes > 0 {
                health.goodput_ewma =
                    update_goodput_ewma(health.goodput_ewma, useful_bytes as u64, elapsed);
            }
        } else {
            health.failures = health.failures.saturating_add(1);
            health.record_loss_sample(true);
            if self.config.tcp_first_resolver_path()
                || health.failures >= 2
                || health.loss_ewma_ppm >= PATH_SEVERE_LOSS_PPM
            {
                health.cwnd = ((health.cwnd.saturating_mul(3)) / 4).max(PATH_MIN_CWND);
            } else if health.cwnd > PATH_MIN_CWND {
                health.cwnd = health.cwnd.saturating_sub(1).max(PATH_MIN_CWND);
            }
            health.cwnd_successes = 0;
            health.timeout = (health.timeout * 2).min(self.rto_max());
            health.clean_mtu_successes = 0;
            let safe_response_bytes = PATH_INITIAL_RESPONSE_BYTES
                .min(self.config.dns_max_payload)
                .max(PATH_MIN_RESPONSE_BYTES);
            if health.max_response_bytes > safe_response_bytes {
                health.max_response_bytes = health
                    .max_response_bytes
                    .saturating_sub(PATH_MTU_STEP.saturating_mul(4))
                    .max(safe_response_bytes);
            }
            let tcp_path = self.config.tcp_first_resolver_path();
            let failure_limit = if tcp_path { 3 } else { 4 };
            if health.failures >= failure_limit {
                health.blocked_until = Some(Instant::now() + self.resolver_quarantine());
            }
        }
        health.update_pacing(self.config.tcp_first_resolver_path());
    }

    async fn prefer_tcp_for_resolver(&self, index: usize) -> bool {
        if self.config.resolver_socks_proxy.is_some() {
            return false;
        }
        if self.config.resolver_transport == ResolverTransportMode::Tcp {
            return true;
        }
        if self.config.resolver_transport == ResolverTransportMode::Udp {
            return false;
        }
        let health = self.resolver_health[index].lock().await;
        health
            .prefer_tcp_until
            .map(|until| until > Instant::now())
            .unwrap_or(false)
    }

    async fn release_resolver(&self, index: usize) {
        if let Some(proxy) = &self.proxy_health {
            let mut proxy = proxy.lock().await;
            proxy.in_flight = proxy.in_flight.saturating_sub(1);
        }
        let mut health = self.resolver_health[index].lock().await;
        health.in_flight = health.in_flight.saturating_sub(1);
    }

    async fn bulk_rtt_cutoff(&self, start: usize, now: Instant) -> Option<u128> {
        let mut best = None::<u128>;
        let count = self.config.resolvers.len();
        for offset in 0..count {
            let index = (start + offset) % count;
            let health = self.resolver_health[index].lock().await;
            if health
                .blocked_until
                .is_some_and(|blocked_until| blocked_until > now)
                || health.loss_ewma_ppm >= PATH_BULK_LOSSY_PPM
            {
                continue;
            }
            let rtt = health
                .srtt
                .unwrap_or_else(|| self.initial_rto())
                .as_micros();
            best = Some(best.map(|current| current.min(rtt)).unwrap_or(rtt));
        }
        best.map(|rtt| rtt.saturating_mul(7) / 4)
    }

    fn path_max_cwnd(&self) -> u32 {
        if self.config.tcp_first_resolver_path() {
            PATH_MAX_CWND_TCP
        } else {
            PATH_MAX_CWND_UDP
        }
    }

    fn initial_rto(&self) -> Duration {
        if self.config.tcp_first_resolver_path() {
            Duration::from_secs(8)
        } else {
            Duration::from_millis(1_500)
        }
    }

    fn rto_min(&self) -> Duration {
        if self.config.tcp_first_resolver_path() {
            PATH_RTO_MIN_TCP
        } else {
            PATH_RTO_MIN_UDP
        }
    }

    fn rto_max(&self) -> Duration {
        if self.config.tcp_first_resolver_path() {
            PATH_RTO_MAX_TCP
        } else {
            PATH_RTO_MAX_UDP
        }
    }

    fn resolver_quarantine(&self) -> Duration {
        if self.config.tcp_first_resolver_path() {
            RESOLVER_FAILURE_QUARANTINE_TCP
        } else {
            RESOLVER_FAILURE_QUARANTINE
        }
    }

    fn response_bytes_for_health(&self, health: &ResolverHealth, now: Instant) -> u16 {
        let direct_tcp_preferred = health
            .prefer_tcp_until
            .map(|until| until > now)
            .unwrap_or(false);
        if self.config.tcp_first_resolver_path() || direct_tcp_preferred {
            health
                .max_response_bytes
                .min(self.config.dns_max_payload.max(PATH_MIN_RESPONSE_BYTES))
                .max(PATH_MIN_RESPONSE_BYTES)
        } else {
            health.max_response_bytes
        }
    }
}

struct ResolverHealth {
    failures: u32,
    loss_ewma_ppm: u32,
    in_flight: u32,
    cwnd: u32,
    cwnd_successes: u32,
    max_response_bytes: u16,
    clean_mtu_successes: u32,
    goodput_ewma: u64,
    pacing_interval: Duration,
    next_send_at: Instant,
    srtt: Option<Duration>,
    rttvar: Duration,
    timeout: Duration,
    blocked_until: Option<Instant>,
    prefer_tcp_until: Option<Instant>,
}

impl ResolverHealth {
    fn new(initial_timeout: Duration, initial_response_bytes: u16) -> Self {
        Self {
            failures: 0,
            loss_ewma_ppm: 0,
            in_flight: 0,
            cwnd: PATH_INITIAL_CWND,
            cwnd_successes: 0,
            max_response_bytes: initial_response_bytes,
            clean_mtu_successes: 0,
            goodput_ewma: 0,
            pacing_interval: Duration::ZERO,
            next_send_at: Instant::now(),
            srtt: None,
            rttvar: initial_timeout / 2,
            timeout: initial_timeout,
            blocked_until: None,
            prefer_tcp_until: None,
        }
    }

    fn record_rtt(&mut self, elapsed: Duration, min_timeout: Duration, max_timeout: Duration) {
        let sample = elapsed.max(Duration::from_millis(1));
        match self.srtt {
            Some(srtt) => {
                let diff = srtt.abs_diff(sample);
                self.rttvar = duration_weighted_average(self.rttvar, diff, 3, 1);
                self.srtt = Some(duration_weighted_average(srtt, sample, 7, 1));
            }
            None => {
                self.srtt = Some(sample);
                self.rttvar = sample / 2;
            }
        }
        let srtt = self.srtt.unwrap_or(sample);
        self.timeout = (srtt + self.rttvar * 4).clamp(min_timeout, max_timeout);
    }

    fn record_loss_sample(&mut self, lost: bool) {
        let sample = if lost { 1_000_000 } else { 0 };
        self.loss_ewma_ppm = ((self.loss_ewma_ppm as u64)
            .saturating_mul((PATH_LOSS_EWMA_DENOMINATOR - 1) as u64)
            .saturating_add(sample as u64)
            / PATH_LOSS_EWMA_DENOMINATOR as u64) as u32;
    }

    fn update_pacing(&mut self, tcp_path: bool) {
        let base = if tcp_path {
            Duration::from_millis(2)
        } else {
            Duration::from_millis(1)
        };
        let Some(srtt) = self.srtt else {
            self.pacing_interval = base;
            return;
        };
        let cwnd = self.cwnd.max(1);
        self.pacing_interval = (srtt / cwnd).max(base);
    }
}

struct ProxyHealth {
    failures: u32,
    in_flight: u32,
    cwnd: u32,
    cwnd_successes: u32,
    pacing_interval: Duration,
    next_send_at: Instant,
    srtt: Option<Duration>,
    rttvar: Duration,
    timeout: Duration,
}

impl Default for ProxyHealth {
    fn default() -> Self {
        Self {
            failures: 0,
            in_flight: 0,
            cwnd: PROXY_INITIAL_CWND,
            cwnd_successes: 0,
            pacing_interval: Duration::ZERO,
            next_send_at: Instant::now(),
            srtt: None,
            rttvar: Duration::from_secs(4),
            timeout: Duration::from_secs(8),
        }
    }
}

impl ProxyHealth {
    fn record_result(
        &mut self,
        ok: bool,
        elapsed: Duration,
        min_timeout: Duration,
        max_timeout: Duration,
    ) {
        self.in_flight = self.in_flight.saturating_sub(1);
        if ok {
            self.failures = 0;
            self.record_rtt(elapsed, min_timeout, max_timeout);
            self.cwnd_successes = self.cwnd_successes.saturating_add(1);
            if self.cwnd_successes >= self.cwnd.max(1) {
                self.cwnd_successes = 0;
                self.cwnd = self.cwnd.saturating_add(1).min(PROXY_MAX_CWND);
            }
        } else {
            self.failures = self.failures.saturating_add(1);
            self.cwnd = (self.cwnd / 2).max(PATH_MIN_CWND);
            self.cwnd_successes = 0;
            self.timeout = (self.timeout * 2).min(max_timeout);
        }
        self.update_pacing();
    }

    fn record_rtt(&mut self, elapsed: Duration, min_timeout: Duration, max_timeout: Duration) {
        let sample = elapsed.max(Duration::from_millis(1));
        match self.srtt {
            Some(srtt) => {
                let diff = srtt.abs_diff(sample);
                self.rttvar = duration_weighted_average(self.rttvar, diff, 3, 1);
                self.srtt = Some(duration_weighted_average(srtt, sample, 7, 1));
            }
            None => {
                self.srtt = Some(sample);
                self.rttvar = sample / 2;
            }
        }
        let srtt = self.srtt.unwrap_or(sample);
        self.timeout = (srtt + self.rttvar * 4).clamp(min_timeout, max_timeout);
    }

    fn update_pacing(&mut self) {
        let Some(srtt) = self.srtt else {
            self.pacing_interval = Duration::ZERO;
            return;
        };
        self.pacing_interval = srtt / self.cwnd.max(1);
    }
}

impl Default for ResolverHealth {
    fn default() -> Self {
        Self::new(Duration::from_millis(1_500), PATH_INITIAL_RESPONSE_BYTES)
    }
}

fn duration_weighted_average(
    previous: Duration,
    sample: Duration,
    previous_weight: u32,
    sample_weight: u32,
) -> Duration {
    let denominator = previous_weight.saturating_add(sample_weight).max(1) as u128;
    let micros = (previous.as_micros() * previous_weight as u128
        + sample.as_micros() * sample_weight as u128)
        / denominator;
    Duration::from_micros(micros.min(u64::MAX as u128) as u64)
}

fn update_goodput_ewma(previous: u64, bytes: u64, elapsed: Duration) -> u64 {
    let micros = elapsed.as_micros().max(1) as u64;
    let sample = bytes.saturating_mul(1_000_000) / micros;
    if previous == 0 {
        sample
    } else {
        (previous.saturating_mul(7).saturating_add(sample)) / 8
    }
}

struct ResolverPool {
    proxy: Option<SocketAddr>,
    lanes_per_resolver: usize,
    senders: Mutex<HashMap<SocketAddr, ResolverLanes>>,
}

struct UdpResolverPool {
    senders: Mutex<HashMap<SocketAddr, mpsc::Sender<DnsUdpRequest>>>,
}

struct ResolverLanes {
    next: usize,
    senders: Vec<mpsc::Sender<DnsTcpRequest>>,
}

struct DnsTcpRequest {
    query: Vec<u8>,
    timeout: Duration,
    response: oneshot::Sender<Result<Vec<u8>>>,
}

struct DnsUdpRequest {
    query: Vec<u8>,
    timeout: Duration,
    response: oneshot::Sender<Result<Vec<u8>>>,
}

struct UdpInFlight {
    response: oneshot::Sender<Result<Vec<u8>>>,
    expires_at: Instant,
}

struct DnsTcpInFlight {
    response: oneshot::Sender<Result<Vec<u8>>>,
    sent_at: Instant,
    timeout: Duration,
    expires_at: Instant,
}

struct PathPermit {
    resolver_index: usize,
    resolver: SocketAddr,
    timeout: Duration,
    max_response_bytes: u16,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ClientSendClass {
    Control,
    Data,
    Poll,
}

impl ClientSendClass {
    fn proxy_headroom(self) -> u32 {
        match self {
            Self::Poll => CLIENT_POLL_PROXY_HEADROOM,
            Self::Control | Self::Data => 0,
        }
    }

    fn resolver_headroom(self) -> u32 {
        match self {
            Self::Poll => CLIENT_POLL_RESOLVER_HEADROOM,
            Self::Control | Self::Data => 0,
        }
    }
}

struct ClientDnsResult {
    resolver: SocketAddr,
    packet_no: u64,
    response_wire_bytes: usize,
    result: Result<Packet>,
}

enum ClientTransportEvent {
    OpenStream {
        stream_id: u64,
        output: mpsc::Sender<ClientStreamOutput>,
    },
    LocalBytes {
        stream_id: u64,
        bytes: Vec<u8>,
    },
    LocalFin {
        stream_id: u64,
    },
    LocalClosed {
        stream_id: u64,
    },
}

enum ClientStreamOutput {
    Bytes(Vec<u8>),
    Close,
}

struct ClientMuxStream {
    opened: bool,
    open_in_flight: bool,
    local_eof: bool,
    close_requested_at: Option<Instant>,
    close_in_flight: bool,
    closed: bool,
    local_output_closed: bool,
    send_offset: u64,
    upload_send: RetainedByteSendBuffer,
    downlink: StreamAssembler,
    last_stream_ack_advertised: Option<StreamAckState>,
    last_activity_at: Instant,
    pending_output: VecDeque<Vec<u8>>,
    pending_output_bytes: usize,
    output: mpsc::Sender<ClientStreamOutput>,
}

impl ClientMuxStream {
    fn new(output: mpsc::Sender<ClientStreamOutput>) -> Self {
        Self {
            opened: false,
            open_in_flight: false,
            local_eof: false,
            close_requested_at: None,
            close_in_flight: false,
            closed: false,
            local_output_closed: false,
            send_offset: 0,
            upload_send: RetainedByteSendBuffer::default(),
            downlink: StreamAssembler::default(),
            last_stream_ack_advertised: None,
            last_activity_at: Instant::now(),
            pending_output: VecDeque::new(),
            pending_output_bytes: 0,
            output,
        }
    }

    fn wants_poll(&self, now: Instant) -> bool {
        !self.closed
            && (self.upload_send.has_retained_bytes()
                || self.downlink.pending_len() > 0
                || self.pending_output_bytes > 0
                || now.saturating_duration_since(self.last_activity_at) <= CLIENT_ACTIVE_POLL_GRACE)
    }

    fn is_drained(&self) -> bool {
        self.closed
    }
}

enum MuxSentKind {
    Open {
        stream_id: u64,
        first_data: Option<SendBufferSlice>,
    },
    Data {
        stream_id: u64,
        slice: SendBufferSlice,
    },
    Close {
        stream_id: u64,
    },
    Ping,
}

struct MuxSentPacket {
    kind: MuxSentKind,
    stream_acks: Vec<(u64, StreamAckState)>,
}

struct ClientTransport {
    runtime: Arc<ClientRuntime>,
    conn_id: u64,
    next_packet_no: u64,
    resolver_cursor: usize,
    stream_cursor: usize,
    streams: HashMap<u64, ClientMuxStream>,
    stream_order: VecDeque<u64>,
    outstanding: HashMap<u64, MuxSentPacket>,
    received_server: PacketHistory,
    response_tx: mpsc::Sender<ClientDnsResult>,
    response_rx: mpsc::Receiver<ClientDnsResult>,
    last_ping_sent_at: Instant,
    active_poll_until: Instant,
    data_since_poll: usize,
    diag_started: Instant,
    next_diag_at: Instant,
}

struct ClientDiag {
    queries_sent: AtomicU64,
    queries_ok: AtomicU64,
    queries_failed: AtomicU64,
    query_wire_bytes: AtomicU64,
    request_packet_body_bytes: AtomicU64,
    request_envelope_bytes: AtomicU64,
    request_qname_chars: AtomicU64,
    response_wire_bytes: AtomicU64,
    response_useful_bytes: AtomicU64,
    data_bytes_received: AtomicU64,
    data_frames_received: AtomicU64,
    upload_data_bytes_sent: AtomicU64,
    upload_data_frames_sent: AtomicU64,
    upload_new_bytes_sent: AtomicU64,
    upload_repair_bytes_sent: AtomicU64,
    upload_new_packets_sent: AtomicU64,
    upload_repair_packets_sent: AtomicU64,
    upload_fin_packets_sent: AtomicU64,
    stream_ack_frames_sent: AtomicU64,
    packet_ack_ranges_sent: AtomicU64,
    data_packets_sent: AtomicU64,
    ping_packets_sent: AtomicU64,
    ping_responses_ok: AtomicU64,
    ping_responses_with_data: AtomicU64,
    ping_response_data_bytes: AtomicU64,
    data_responses_ok: AtomicU64,
    data_response_data_bytes: AtomicU64,
    open_packets_sent: AtomicU64,
    qname_too_long_splits: AtomicU64,
    tcp_fallbacks: AtomicU64,
    fill_stop_no_kind: AtomicU64,
    fill_stop_no_resolver_control: AtomicU64,
    fill_stop_no_resolver_data: AtomicU64,
    fill_stop_no_resolver_poll: AtomicU64,
    fill_stop_ping_slot: AtomicU64,
    resolver: Vec<ResolverDiag>,
}

impl ClientDiag {
    fn new(resolver_count: usize) -> Self {
        Self {
            queries_sent: AtomicU64::new(0),
            queries_ok: AtomicU64::new(0),
            queries_failed: AtomicU64::new(0),
            query_wire_bytes: AtomicU64::new(0),
            request_packet_body_bytes: AtomicU64::new(0),
            request_envelope_bytes: AtomicU64::new(0),
            request_qname_chars: AtomicU64::new(0),
            response_wire_bytes: AtomicU64::new(0),
            response_useful_bytes: AtomicU64::new(0),
            data_bytes_received: AtomicU64::new(0),
            data_frames_received: AtomicU64::new(0),
            upload_data_bytes_sent: AtomicU64::new(0),
            upload_data_frames_sent: AtomicU64::new(0),
            upload_new_bytes_sent: AtomicU64::new(0),
            upload_repair_bytes_sent: AtomicU64::new(0),
            upload_new_packets_sent: AtomicU64::new(0),
            upload_repair_packets_sent: AtomicU64::new(0),
            upload_fin_packets_sent: AtomicU64::new(0),
            stream_ack_frames_sent: AtomicU64::new(0),
            packet_ack_ranges_sent: AtomicU64::new(0),
            data_packets_sent: AtomicU64::new(0),
            ping_packets_sent: AtomicU64::new(0),
            ping_responses_ok: AtomicU64::new(0),
            ping_responses_with_data: AtomicU64::new(0),
            ping_response_data_bytes: AtomicU64::new(0),
            data_responses_ok: AtomicU64::new(0),
            data_response_data_bytes: AtomicU64::new(0),
            open_packets_sent: AtomicU64::new(0),
            qname_too_long_splits: AtomicU64::new(0),
            tcp_fallbacks: AtomicU64::new(0),
            fill_stop_no_kind: AtomicU64::new(0),
            fill_stop_no_resolver_control: AtomicU64::new(0),
            fill_stop_no_resolver_data: AtomicU64::new(0),
            fill_stop_no_resolver_poll: AtomicU64::new(0),
            fill_stop_ping_slot: AtomicU64::new(0),
            resolver: (0..resolver_count)
                .map(|_| ResolverDiag::default())
                .collect(),
        }
    }
}

#[derive(Default)]
struct ResolverDiag {
    sent: AtomicU64,
    ok: AtomicU64,
    failed: AtomicU64,
    query_wire_bytes: AtomicU64,
    response_wire_bytes: AtomicU64,
    useful_response_bytes: AtomicU64,
    elapsed_us_sum: AtomicU64,
    elapsed_us_max: AtomicU64,
    control_sent: AtomicU64,
    data_sent: AtomicU64,
    poll_sent: AtomicU64,
    udp_ok: AtomicU64,
    tcp_preferred_ok: AtomicU64,
    tcp_truncation_ok: AtomicU64,
    udp_after_tcp_error_ok: AtomicU64,
    tcp_proxy_ok: AtomicU64,
    truncated: AtomicU64,
}

impl ResolverPool {
    fn new(proxy: Option<SocketAddr>) -> Self {
        let lanes_per_resolver = if proxy.is_some() {
            TCP_RESOLVER_LANES_PROXY
        } else {
            TCP_RESOLVER_LANES_DIRECT
        };
        Self::new_with_lanes(proxy, lanes_per_resolver)
    }

    fn new_with_lanes(proxy: Option<SocketAddr>, lanes_per_resolver: usize) -> Self {
        Self {
            proxy,
            lanes_per_resolver: lanes_per_resolver.max(1),
            senders: Mutex::new(HashMap::new()),
        }
    }

    async fn query(
        &self,
        resolver: SocketAddr,
        query: &[u8],
        query_timeout: Duration,
    ) -> Result<Vec<u8>> {
        for _ in 0..2 {
            let sender = self.sender_for(resolver).await;
            let (tx, rx) = oneshot::channel();
            let request = DnsTcpRequest {
                query: query.to_vec(),
                timeout: query_timeout,
                response: tx,
            };
            if sender.send(request).await.is_err() {
                self.remove_sender(resolver).await;
                continue;
            }
            match timeout(query_timeout, rx).await {
                Ok(Ok(result)) => return result,
                Ok(Err(_)) => {
                    self.remove_sender(resolver).await;
                    continue;
                }
                Err(_) => {
                    self.remove_sender(resolver).await;
                    bail!("DNS-over-TCP response timed out");
                }
            }
        }
        bail!("DNS-over-TCP resolver worker failed");
    }

    async fn sender_for(&self, resolver: SocketAddr) -> mpsc::Sender<DnsTcpRequest> {
        let mut senders = self.senders.lock().await;
        let lanes = senders.entry(resolver).or_insert_with(|| ResolverLanes {
            next: 0,
            senders: Vec::with_capacity(self.lanes_per_resolver),
        });
        lanes.senders.retain(|sender| !sender.is_closed());
        while lanes.senders.len() < self.lanes_per_resolver {
            let (tx, rx) = mpsc::channel(TCP_RESOLVER_QUEUE);
            tokio::spawn(run_tcp_resolver_actor(self.proxy, resolver, rx));
            lanes.senders.push(tx);
        }

        let index = lanes.next % lanes.senders.len();
        lanes.next = lanes.next.wrapping_add(1);
        lanes.senders[index].clone()
    }

    async fn remove_sender(&self, resolver: SocketAddr) {
        self.senders.lock().await.remove(&resolver);
    }
}

impl UdpResolverPool {
    fn new() -> Self {
        Self {
            senders: Mutex::new(HashMap::new()),
        }
    }

    async fn query(
        &self,
        resolver: SocketAddr,
        query: &[u8],
        query_timeout: Duration,
    ) -> Result<Vec<u8>> {
        for _ in 0..2 {
            let sender = self.sender_for(resolver).await;
            let (tx, rx) = oneshot::channel();
            let request = DnsUdpRequest {
                query: query.to_vec(),
                timeout: query_timeout,
                response: tx,
            };
            if sender.send(request).await.is_err() {
                self.remove_sender(resolver).await;
                continue;
            }
            match timeout(query_timeout, rx).await {
                Ok(Ok(result)) => return result,
                Ok(Err(_)) => {
                    self.remove_sender(resolver).await;
                    continue;
                }
                Err(_) => bail!("UDP DNS response timed out"),
            }
        }
        bail!("UDP resolver worker failed");
    }

    async fn sender_for(&self, resolver: SocketAddr) -> mpsc::Sender<DnsUdpRequest> {
        let mut senders = self.senders.lock().await;
        if let Some(sender) = senders.get(&resolver) {
            if !sender.is_closed() {
                return sender.clone();
            }
        }

        let (tx, rx) = mpsc::channel(UDP_RESOLVER_QUEUE);
        tokio::spawn(run_udp_resolver_actor(resolver, rx));
        senders.insert(resolver, tx.clone());
        tx
    }

    async fn remove_sender(&self, resolver: SocketAddr) {
        self.senders.lock().await.remove(&resolver);
    }
}

pub async fn run_client(mut config: ClientConfig) -> Result<()> {
    if config.resolvers.is_empty() {
        bail!("at least one resolver is required");
    }
    config.resolvers = dedupe_resolvers(config.resolvers);
    if should_admit_resolvers(&config) {
        config.resolvers = admit_resolvers(config.clone()).await?;
    }

    let listener = TcpListener::bind(config.listen)
        .await
        .with_context(|| format!("bind local listener {}", config.listen))?;
    let http_listener = match config.http_listen {
        Some(addr) => Some(
            TcpListener::bind(addr)
                .await
                .with_context(|| format!("bind local HTTP proxy listener {addr}"))?,
        ),
        None => None,
    };
    eprintln!(
        "trajectory client listening on {} via {} resolver(s)",
        listener.local_addr()?,
        config.resolvers.len()
    );
    if let Some(listener) = &http_listener {
        eprintln!(
            "trajectory HTTP proxy listening on {}",
            listener.local_addr()?
        );
    }

    let runtime = Arc::new(ClientRuntime::new(config));
    let (transport_tx, transport_rx) =
        mpsc::channel::<ClientTransportEvent>(CLIENT_TRANSPORT_EVENT_QUEUE);
    let transport_runtime = Arc::clone(&runtime);
    tokio::spawn(async move {
        if let Err(error) = run_client_transport(transport_runtime, transport_rx).await {
            eprintln!("client transport failed: {error:#}");
        }
    });

    let next_stream_id = Arc::new(AtomicU64::new(0));
    if let Some(listener) = http_listener {
        let transport_tx = transport_tx.clone();
        let runtime = Arc::clone(&runtime);
        let next_stream_id = Arc::clone(&next_stream_id);
        tokio::spawn(async move {
            if let Err(error) =
                accept_http_proxy_streams(listener, runtime, transport_tx, next_stream_id).await
            {
                eprintln!("HTTP proxy listener failed: {error:#}");
            }
        });
    }

    loop {
        let (stream, peer) = listener.accept().await?;
        let runtime = Arc::clone(&runtime);
        let transport_tx = transport_tx.clone();
        let stream_id = next_stream_id.fetch_add(1, Ordering::Relaxed);
        tokio::spawn(async move {
            let _stream_slot = match Arc::clone(&runtime.stream_slots).acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => return,
            };
            if let Err(error) = run_local_stream_io(transport_tx, stream_id, stream).await {
                eprintln!("local stream {stream_id} from {peer} failed: {error:#}");
            }
        });
    }
}

fn dedupe_resolvers(resolvers: Vec<SocketAddr>) -> Vec<SocketAddr> {
    let mut seen = HashSet::new();
    resolvers
        .into_iter()
        .filter(|resolver| seen.insert(*resolver))
        .collect()
}

async fn admit_resolvers(config: ClientConfig) -> Result<Vec<SocketAddr>> {
    let tcp_path = config.tcp_first_resolver_path();
    let requested_min = config.resolver_admission_min.max(1);
    if requested_min > config.resolvers.len() {
        bail!(
            "resolver admission minimum {} exceeds {} candidate resolver(s)",
            requested_min,
            config.resolvers.len()
        );
    }
    if let Some(cohort_size) = config.resolver_cohort_size {
        if requested_min > cohort_size {
            bail!(
                "resolver admission minimum {} exceeds resolver cohort size {}",
                requested_min,
                cohort_size
            );
        }
    }
    let target = config
        .resolver_cohort_size
        .unwrap_or_else(|| resolver_target_admitted(tcp_path))
        .max(requested_min)
        .clamp(1, config.resolvers.len());
    let min_required = requested_min;
    let sample_target = (target * RESOLVER_ADMISSION_SAMPLE_FACTOR).min(config.resolvers.len());
    let mut probe_runtime = ClientRuntime::new(config.clone());
    if let Some(proxy) = config.resolver_socks_proxy {
        probe_runtime.tcp_pool = Some(Arc::new(ResolverPool::new_with_lanes(Some(proxy), 1)));
    }
    let probe_runtime = Arc::new(probe_runtime);
    let mut admitted = Vec::<(SocketAddr, Duration)>::new();
    let mut results = Vec::<AdmissionProbeResult>::new();
    let verbose_failures = std::env::var_os("TRAJECTORY_ADMISSION_DIAG").is_some();
    eprintln!(
        "probing {} resolver(s) before admission",
        config.resolvers.len()
    );
    let parallelism = resolver_probe_parallelism(tcp_path).min(config.resolvers.len().max(1));
    let probe_timeout = resolver_admission_probe_timeout(tcp_path);
    let mut candidates = config.resolvers.iter().copied();
    let mut probes = JoinSet::<AdmissionProbeResult>::new();
    fill_resolver_probe_set(
        &mut probes,
        &probe_runtime,
        &mut candidates,
        parallelism,
        probe_timeout,
    );

    let deadline = tokio::time::sleep(resolver_admission_deadline(
        config.resolvers.len(),
        parallelism,
        probe_timeout,
    ));
    tokio::pin!(deadline);
    while !probes.is_empty() {
        tokio::select! {
            maybe_result = probes.join_next() => {
                match maybe_result {
                    Some(Ok(result)) => {
                    if result.admitted {
                        eprintln!(
                            "admitted resolver {} elapsed={}ms response_bps={}",
                            result.resolver,
                            result.elapsed.as_millis(),
                            result.response_bps
                        );
                        admitted.push((result.resolver, result.elapsed));
                    } else if verbose_failures {
                        eprintln!(
                            "rejected resolver {} stage={} elapsed={}ms reason={}",
                            result.resolver,
                            result.stage,
                            result.elapsed.as_millis(),
                            result.error.as_deref().unwrap_or("unknown")
                        );
                    }
                    results.push(result);
                    if admitted.len() >= sample_target {
                        probes.abort_all();
                        admitted.sort_by_key(|(_, rtt)| *rtt);
                        let resolvers = admitted
                            .into_iter()
                            .take(target)
                            .map(|(resolver, _)| resolver)
                            .collect::<Vec<_>>();
                        if resolvers.len() < min_required {
                            write_admission_report_if_requested(&config, &results, &resolvers)?;
                            bail!(
                                "only {} resolver(s) passed signed tunnel admission; required {}",
                                resolvers.len(),
                                min_required
                            );
                        }
                        write_admission_report_if_requested(&config, &results, &resolvers)?;
                        eprintln!(
                            "using {} admitted resolver(s) out of {} candidate(s)",
                            resolvers.len(),
                            config.resolvers.len()
                        );
                        return Ok(resolvers);
                    }
                    }
                    Some(Err(_)) => {}
                    None => break,
                }
                fill_resolver_probe_set(
                    &mut probes,
                    &probe_runtime,
                    &mut candidates,
                    parallelism,
                    probe_timeout,
                );
            }
            _ = &mut deadline => {
                probes.abort_all();
                break;
            }
        }
    }

    if admitted.len() < min_required {
        write_admission_report_if_requested(&config, &results, &[])?;
        bail!(
            "only {} resolver(s) passed signed tunnel admission; required {}",
            admitted.len(),
            min_required
        );
    }
    eprintln!(
        "using {} admitted resolver(s) out of {} candidate(s)",
        admitted.len(),
        config.resolvers.len()
    );
    admitted.sort_by_key(|(_, rtt)| *rtt);
    let resolvers = admitted
        .into_iter()
        .take(target)
        .map(|(resolver, _)| resolver)
        .collect::<Vec<_>>();
    write_admission_report_if_requested(&config, &results, &resolvers)?;
    Ok(resolvers)
}

fn resolver_target_admitted(tcp_path: bool) -> usize {
    if tcp_path {
        RESOLVER_TARGET_ADMITTED_TCP
    } else {
        RESOLVER_TARGET_ADMITTED_UDP
    }
}

fn should_admit_resolvers(config: &ClientConfig) -> bool {
    let tcp_path = config.tcp_first_resolver_path();
    tcp_path
        || config.resolver_cohort_size.is_some()
        || config.resolver_admission_min > 1
        || config.admission_report.is_some()
        || config.resolvers.len() > resolver_target_admitted(tcp_path)
}

fn resolver_probe_parallelism(tcp_path: bool) -> usize {
    if tcp_path {
        RESOLVER_PROBE_PARALLELISM_TCP
    } else {
        RESOLVER_PROBE_PARALLELISM_UDP
    }
}

fn resolver_admission_timeout(tcp_path: bool) -> Duration {
    if tcp_path {
        RESOLVER_ADMISSION_TIMEOUT_TCP
    } else {
        RESOLVER_ADMISSION_TIMEOUT
    }
}

fn resolver_admission_probe_timeout(tcp_path: bool) -> Duration {
    resolver_admission_timeout(tcp_path).saturating_mul(admission_challenges(tcp_path).len() as u32)
}

fn resolver_admission_deadline(
    candidate_count: usize,
    parallelism: usize,
    probe_timeout: Duration,
) -> Duration {
    let waves = candidate_count.div_ceil(parallelism.max(1)).max(1);
    let wave_budget = probe_timeout.saturating_mul(waves.min(12) as u32);
    RESOLVER_ADMISSION_DEADLINE
        .max(wave_budget)
        .min(Duration::from_secs(10 * 60))
}

fn fill_resolver_probe_set<I>(
    probes: &mut JoinSet<AdmissionProbeResult>,
    runtime: &Arc<ClientRuntime>,
    candidates: &mut I,
    parallelism: usize,
    probe_timeout: Duration,
) where
    I: Iterator<Item = SocketAddr>,
{
    while probes.len() < parallelism {
        let Some(resolver) = candidates.next() else {
            break;
        };
        let runtime = Arc::clone(runtime);
        probes.spawn(async move {
            match timeout(probe_timeout, probe_resolver(runtime, resolver)).await {
                Ok(result) => result,
                Err(_) => AdmissionProbeResult::rejected(
                    resolver,
                    "timeout",
                    probe_timeout,
                    format!(
                        "admission probe timed out after {}ms",
                        probe_timeout.as_millis()
                    ),
                ),
            }
        });
    }
}

async fn probe_resolver(runtime: Arc<ClientRuntime>, resolver: SocketAddr) -> AdmissionProbeResult {
    let started = Instant::now();
    let tcp_path = runtime.config.tcp_first_resolver_path();
    let mut probe = AdmissionProbe::new(runtime, resolver);
    let mut total_response_bytes = 0usize;
    let mut last_rtt = Duration::ZERO;
    let mut challenge_count = 0usize;

    let max_elapsed = admission_max_elapsed(tcp_path);
    for challenge in admission_challenges(tcp_path) {
        let elapsed = started.elapsed();
        if elapsed >= max_elapsed {
            return AdmissionProbeResult::rejected(
                resolver,
                challenge.stage,
                elapsed,
                format!(
                    "admission probe exceeded max elapsed before stage: {}ms >= {}ms",
                    elapsed.as_millis(),
                    max_elapsed.as_millis()
                ),
            )
            .with_probe_stats(last_rtt, challenge_count, total_response_bytes);
        }
        let remaining = max_elapsed.saturating_sub(elapsed);
        match timeout(remaining, probe.send_path_challenge(*challenge)).await {
            Ok(Ok(result)) => {
                challenge_count += 1;
                total_response_bytes += result.response_payload_bytes;
                last_rtt = result.rtt;
            }
            Ok(Err(error)) => {
                return AdmissionProbeResult {
                    resolver,
                    admitted: false,
                    stage: challenge.stage,
                    elapsed: started.elapsed(),
                    last_rtt,
                    challenge_count,
                    response_payload_bytes: total_response_bytes,
                    response_bps: response_bps(total_response_bytes, started.elapsed()),
                    error: Some(error),
                };
            }
            Err(_) => {
                return AdmissionProbeResult {
                    resolver,
                    admitted: false,
                    stage: challenge.stage,
                    elapsed: started.elapsed(),
                    last_rtt,
                    challenge_count,
                    response_payload_bytes: total_response_bytes,
                    response_bps: response_bps(total_response_bytes, started.elapsed()),
                    error: Some(format!(
                        "admission stage timed out after {}ms",
                        remaining.as_millis()
                    )),
                };
            }
        }
    }

    let elapsed = started.elapsed();
    if elapsed > max_elapsed {
        return AdmissionProbeResult::rejected(
            resolver,
            "sustained_probe",
            elapsed,
            format!(
                "admission probe exceeded max elapsed: {}ms > {}ms",
                elapsed.as_millis(),
                max_elapsed.as_millis()
            ),
        )
        .with_probe_stats(last_rtt, challenge_count, total_response_bytes);
    }

    let response_bps = response_bps(total_response_bytes, elapsed);
    let min_response_bps = admission_min_response_bps(tcp_path);
    if response_bps < min_response_bps {
        return AdmissionProbeResult::rejected(
            resolver,
            "sustained_probe",
            elapsed,
            format!(
                "admission probe response goodput too low: {response_bps} B/s < {min_response_bps} B/s"
            ),
        )
        .with_probe_stats(last_rtt, challenge_count, total_response_bytes);
    }

    AdmissionProbeResult {
        resolver,
        admitted: true,
        stage: "admitted",
        elapsed,
        last_rtt,
        challenge_count,
        response_payload_bytes: total_response_bytes,
        response_bps,
        error: None,
    }
}

#[derive(Clone, Copy)]
struct AdmissionChallenge {
    stage: &'static str,
    response_bytes: u16,
    request_padding: usize,
}

struct AdmissionChallengeResult {
    rtt: Duration,
    response_payload_bytes: usize,
}

#[derive(Debug)]
struct AdmissionProbeResult {
    resolver: SocketAddr,
    admitted: bool,
    stage: &'static str,
    elapsed: Duration,
    last_rtt: Duration,
    challenge_count: usize,
    response_payload_bytes: usize,
    response_bps: u64,
    error: Option<String>,
}

impl AdmissionProbeResult {
    fn rejected(
        resolver: SocketAddr,
        stage: &'static str,
        elapsed: Duration,
        error: String,
    ) -> Self {
        Self {
            resolver,
            admitted: false,
            stage,
            elapsed,
            last_rtt: Duration::ZERO,
            challenge_count: 0,
            response_payload_bytes: 0,
            response_bps: 0,
            error: Some(error),
        }
    }

    fn with_probe_stats(
        mut self,
        last_rtt: Duration,
        challenge_count: usize,
        response_payload_bytes: usize,
    ) -> Self {
        self.last_rtt = last_rtt;
        self.challenge_count = challenge_count;
        self.response_payload_bytes = response_payload_bytes;
        self.response_bps = response_bps(response_payload_bytes, self.elapsed);
        self
    }
}

const UDP_ADMISSION_CHALLENGES: [AdmissionChallenge; 4] = [
    AdmissionChallenge {
        stage: "signed_challenge",
        response_bytes: 64,
        request_padding: 16,
    },
    AdmissionChallenge {
        stage: "signed_challenge",
        response_bytes: 96,
        request_padding: 24,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 192,
        request_padding: 32,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 256,
        request_padding: 40,
    },
];

const TCP_PROXY_ADMISSION_CHALLENGES: [AdmissionChallenge; 6] = [
    AdmissionChallenge {
        stage: "signed_challenge",
        response_bytes: 64,
        request_padding: 16,
    },
    AdmissionChallenge {
        stage: "signed_challenge",
        response_bytes: 128,
        request_padding: 24,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 256,
        request_padding: 32,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 512,
        request_padding: 40,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 512,
        request_padding: 48,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 512,
        request_padding: 48,
    },
];

fn admission_challenges(tcp_path: bool) -> &'static [AdmissionChallenge] {
    if tcp_path {
        &TCP_PROXY_ADMISSION_CHALLENGES
    } else {
        &UDP_ADMISSION_CHALLENGES
    }
}

fn admission_max_elapsed(tcp_path: bool) -> Duration {
    if tcp_path {
        RESOLVER_ADMISSION_MAX_ELAPSED_TCP.max(Duration::from_secs(30))
    } else {
        RESOLVER_ADMISSION_MAX_ELAPSED_UDP
    }
}

fn admission_min_response_bps(tcp_path: bool) -> u64 {
    if tcp_path {
        RESOLVER_ADMISSION_MIN_RESPONSE_BPS_TCP
    } else {
        RESOLVER_ADMISSION_MIN_RESPONSE_BPS_UDP
    }
}

fn response_bps(response_payload_bytes: usize, elapsed: Duration) -> u64 {
    if elapsed.is_zero() {
        return response_payload_bytes as u64;
    }
    ((response_payload_bytes as u128 * 1_000_000_000u128) / elapsed.as_nanos())
        .min(u64::MAX as u128) as u64
}

fn write_admission_report_if_requested(
    config: &ClientConfig,
    results: &[AdmissionProbeResult],
    selected: &[SocketAddr],
) -> Result<()> {
    let Some(path) = &config.admission_report else {
        return Ok(());
    };
    write_admission_report(path, results, selected, config)
}

fn write_admission_report(
    path: &Path,
    results: &[AdmissionProbeResult],
    selected: &[SocketAddr],
    config: &ClientConfig,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create admission report directory {}", parent.display()))?;
    }
    let selected = selected.iter().copied().collect::<HashSet<_>>();
    let mut file = std::fs::File::create(path)
        .with_context(|| format!("create admission report {}", path.display()))?;
    use std::io::Write;
    writeln!(
        file,
        "{}",
        serde_json::json!({
            "event": "admission_summary",
            "candidate_count": config.resolvers.len(),
            "selected_count": selected.len(),
            "tcp_path": config.tcp_first_resolver_path(),
            "resolver_transport": format!("{:?}", config.resolver_transport).to_ascii_lowercase(),
            "domain": config.domain,
        })
    )?;
    for result in results {
        writeln!(
            file,
            "{}",
            serde_json::json!({
                "event": "admission_probe",
                "resolver": result.resolver.to_string(),
                "selected": selected.contains(&result.resolver),
                "admitted": result.admitted,
                "stage": result.stage,
                "elapsed_ms": result.elapsed.as_millis() as u64,
                "last_rtt_ms": result.last_rtt.as_millis() as u64,
                "challenge_count": result.challenge_count,
                "response_payload_bytes": result.response_payload_bytes,
                "response_bps": result.response_bps,
                "error": result.error,
            })
        )?;
    }
    Ok(())
}

struct AdmissionProbe {
    runtime: Arc<ClientRuntime>,
    resolver: SocketAddr,
    conn_id: u64,
    packet_no: u64,
    prefer_direct_tcp: bool,
    received_server: PacketHistory,
}

impl AdmissionProbe {
    fn new(runtime: Arc<ClientRuntime>, resolver: SocketAddr) -> Self {
        Self {
            runtime,
            resolver,
            conn_id: rand::random::<u64>(),
            packet_no: 0,
            prefer_direct_tcp: false,
            received_server: PacketHistory::default(),
        }
    }

    async fn send_path_challenge(
        &mut self,
        challenge: AdmissionChallenge,
    ) -> Result<AdmissionChallengeResult, String> {
        let current_packet_no = self.packet_no;
        self.packet_no = self
            .packet_no
            .checked_add(1)
            .ok_or_else(|| "admission packet number overflow".to_string())?;
        let mut packet = Packet::new(self.conn_id, current_packet_no);
        packet.max_response_bytes = self
            .runtime
            .config
            .dns_max_payload
            .max(PATH_MIN_RESPONSE_BYTES);
        let required_response_bytes =
            admission_challenge_response_bytes(challenge.response_bytes, packet.max_response_bytes);
        packet.ack_ranges = self.received_server.ack_ranges(1);
        packet.frames.push(Frame::PathChallenge {
            nonce: current_packet_no,
            response_bytes: required_response_bytes,
        });
        if challenge.request_padding > 0 {
            packet.frames.push(Frame::PathResponse {
                nonce: current_packet_no,
                bytes: vec![0; challenge.request_padding],
            });
        }
        let started = Instant::now();
        let response = send_dns_packet_inner(
            &self.runtime,
            None,
            self.resolver,
            None,
            &packet,
            resolver_admission_timeout(self.runtime.config.tcp_first_resolver_path()),
            self.prefer_direct_tcp
                || self.runtime.config.resolver_transport == ResolverTransportMode::Tcp,
        )
        .await
        .map_err(|error| error.to_string())?;
        if matches!(
            response.transport,
            DnsTransportOutcome::TcpPreferred
                | DnsTransportOutcome::TcpAfterUdpFailure
                | DnsTransportOutcome::TcpFallbackAfterTruncation
        ) {
            self.prefer_direct_tcp = true;
        } else if matches!(response.transport, DnsTransportOutcome::Udp) {
            self.prefer_direct_tcp = false;
        }
        let rtt = started.elapsed();
        if response.packet.conn_id != self.conn_id
            || self.received_server.is_acked(response.packet.packet_no)
            || !ack_ranges_contain(&response.packet.ack_ranges, current_packet_no)
        {
            return Err("signed challenge response did not acknowledge request packet".to_string());
        }
        let response_payload_bytes = response
            .packet
            .frames
            .iter()
            .map(|frame| match frame {
                Frame::PathResponse { bytes, .. } => bytes.len(),
                Frame::Data { bytes, .. } => bytes.len(),
                _ => 0,
            })
            .sum();
        let has_response = required_response_bytes == 0
            || response.packet.frames.iter().any(|frame| {
                matches!(
                    frame,
                    Frame::PathResponse { nonce, bytes }
                        if *nonce == current_packet_no && bytes.len() >= required_response_bytes as usize
                )
            });
        if !has_response {
            return Err("signed challenge response missed required path response".to_string());
        }
        self.received_server.insert(response.packet.packet_no);
        Ok(AdmissionChallengeResult {
            rtt,
            response_payload_bytes,
        })
    }
}

fn admission_challenge_response_bytes(requested: u16, max_response_bytes: u16) -> u16 {
    let payload_cap = max_response_bytes
        .saturating_sub((DNS_RESPONSE_SAFETY_MARGIN + 256) as u16)
        .max(64);
    requested.min(payload_cap)
}

async fn run_local_stream_io(
    transport_tx: mpsc::Sender<ClientTransportEvent>,
    stream_id: u64,
    local: TcpStream,
) -> Result<()> {
    local.set_nodelay(true).ok();
    let (mut reader, mut writer) = local.into_split();
    let (output_tx, output_rx) = mpsc::channel(CLIENT_STREAM_OUTPUT_QUEUE);
    transport_tx
        .send(ClientTransportEvent::OpenStream {
            stream_id,
            output: output_tx,
        })
        .await
        .context("register local stream with client transport")?;

    let read_tx = transport_tx.clone();
    let reader_task = tokio::spawn(async move {
        let mut buf = [0u8; UPLOAD_READ_CHUNK];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    let _ = read_tx
                        .send(ClientTransportEvent::LocalFin { stream_id })
                        .await;
                    return;
                }
                Ok(n) => {
                    if read_tx
                        .send(ClientTransportEvent::LocalBytes {
                            stream_id,
                            bytes: buf[..n].to_vec(),
                        })
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
                Err(_) => {
                    let _ = read_tx
                        .send(ClientTransportEvent::LocalClosed { stream_id })
                        .await;
                    return;
                }
            }
        }
    });

    let writer_result = write_local_stream_output(&mut writer, output_rx).await;
    reader_task.abort();
    let _ = transport_tx
        .send(ClientTransportEvent::LocalClosed { stream_id })
        .await;
    writer_result
}

async fn write_local_stream_output(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    mut output_rx: mpsc::Receiver<ClientStreamOutput>,
) -> Result<()> {
    while let Some(output) = output_rx.recv().await {
        match output {
            ClientStreamOutput::Bytes(bytes) => {
                if !bytes.is_empty() {
                    writer.write_all(&bytes).await?;
                }
            }
            ClientStreamOutput::Close => break,
        }
    }
    writer.shutdown().await.ok();
    Ok(())
}

async fn accept_http_proxy_streams(
    listener: TcpListener,
    runtime: Arc<ClientRuntime>,
    transport_tx: mpsc::Sender<ClientTransportEvent>,
    next_stream_id: Arc<AtomicU64>,
) -> Result<()> {
    loop {
        let (stream, peer) = listener.accept().await?;
        let runtime = Arc::clone(&runtime);
        let transport_tx = transport_tx.clone();
        let stream_id = next_stream_id.fetch_add(1, Ordering::Relaxed);
        tokio::spawn(async move {
            let _stream_slot = match Arc::clone(&runtime.stream_slots).acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => return,
            };
            if let Err(error) = run_http_proxy_stream_io(transport_tx, stream_id, stream).await {
                eprintln!("HTTP proxy stream {stream_id} from {peer} failed: {error:#}");
            }
        });
    }
}

async fn run_http_proxy_stream_io(
    transport_tx: mpsc::Sender<ClientTransportEvent>,
    stream_id: u64,
    mut local: TcpStream,
) -> Result<()> {
    local.set_nodelay(true).ok();
    let (header, after_header) = timeout(
        HTTP_PROXY_HEADER_TIMEOUT,
        read_http_proxy_header(&mut local),
    )
    .await
    .context("HTTP proxy request header timed out")??;
    let request = parse_http_proxy_request(&header, &after_header)?;
    let output_rx = register_client_stream(&transport_tx, stream_id).await?;
    let mut remote = TransportOutputReader::new(output_rx);

    match socks5_connect_via_transport(
        &transport_tx,
        stream_id,
        remote,
        &request.host,
        request.port,
    )
    .await
    {
        Ok(reader) => remote = reader,
        Err(error) => {
            let _ = local
                .write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n")
                .await;
            let _ = transport_tx
                .send(ClientTransportEvent::LocalClosed { stream_id })
                .await;
            return Err(error);
        }
    }

    if request.respond_connect_ok {
        local
            .write_all(b"HTTP/1.1 200 Connection Established\r\nProxy-Agent: trajectory\r\n\r\n")
            .await
            .context("write HTTP CONNECT success")?;
    }
    if !request.initial_upstream.is_empty() {
        send_transport_bytes(&transport_tx, stream_id, request.initial_upstream).await?;
    }

    let (mut reader, mut writer) = local.into_split();
    let read_tx = transport_tx.clone();
    let reader_task = tokio::spawn(async move {
        let mut buf = [0u8; UPLOAD_READ_CHUNK];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) => {
                    let _ = read_tx
                        .send(ClientTransportEvent::LocalFin { stream_id })
                        .await;
                    return;
                }
                Ok(n) => {
                    if read_tx
                        .send(ClientTransportEvent::LocalBytes {
                            stream_id,
                            bytes: buf[..n].to_vec(),
                        })
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
                Err(_) => {
                    let _ = read_tx
                        .send(ClientTransportEvent::LocalClosed { stream_id })
                        .await;
                    return;
                }
            }
        }
    });

    let writer_result = remote.write_to_local(&mut writer).await;
    reader_task.abort();
    let _ = transport_tx
        .send(ClientTransportEvent::LocalClosed { stream_id })
        .await;
    writer_result
}

async fn register_client_stream(
    transport_tx: &mpsc::Sender<ClientTransportEvent>,
    stream_id: u64,
) -> Result<mpsc::Receiver<ClientStreamOutput>> {
    let (output_tx, output_rx) = mpsc::channel(CLIENT_STREAM_OUTPUT_QUEUE);
    transport_tx
        .send(ClientTransportEvent::OpenStream {
            stream_id,
            output: output_tx,
        })
        .await
        .context("register local stream with client transport")?;
    Ok(output_rx)
}

async fn send_transport_bytes(
    transport_tx: &mpsc::Sender<ClientTransportEvent>,
    stream_id: u64,
    bytes: Vec<u8>,
) -> Result<()> {
    transport_tx
        .send(ClientTransportEvent::LocalBytes { stream_id, bytes })
        .await
        .context("send local bytes to client transport")
}

async fn read_http_proxy_header(stream: &mut TcpStream) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut data = Vec::with_capacity(1024);
    let mut buf = [0u8; 1024];
    loop {
        let n = stream
            .read(&mut buf)
            .await
            .context("read HTTP proxy request header")?;
        if n == 0 {
            bail!("HTTP proxy client closed before sending headers");
        }
        data.extend_from_slice(&buf[..n]);
        if data.len() > HTTP_PROXY_HEADER_MAX {
            bail!("HTTP proxy request header exceeded {HTTP_PROXY_HEADER_MAX} bytes");
        }
        if let Some(end) = find_http_header_end(&data) {
            let after = data.split_off(end);
            return Ok((data, after));
        }
    }
}

fn find_http_header_end(data: &[u8]) -> Option<usize> {
    data.windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
}

struct HttpProxyRequest {
    host: String,
    port: u16,
    respond_connect_ok: bool,
    initial_upstream: Vec<u8>,
}

fn parse_http_proxy_request(header: &[u8], after_header: &[u8]) -> Result<HttpProxyRequest> {
    let text = std::str::from_utf8(header).context("HTTP proxy request was not UTF-8")?;
    let mut lines = text.split("\r\n");
    let request_line = lines.next().context("missing HTTP request line")?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().context("missing HTTP method")?;
    let target = request_parts.next().context("missing HTTP target")?;
    let version = request_parts.next().context("missing HTTP version")?;
    if !version.starts_with("HTTP/") {
        bail!("unsupported HTTP proxy request version: {version}");
    }

    if method.eq_ignore_ascii_case("CONNECT") {
        let (host, port) = split_host_port(target, 443)?;
        return Ok(HttpProxyRequest {
            host,
            port,
            respond_connect_ok: true,
            initial_upstream: after_header.to_vec(),
        });
    }

    let Some(rest) = target.strip_prefix("http://") else {
        bail!("HTTP proxy mode supports CONNECT and absolute http:// requests");
    };
    let slash = rest.find('/').unwrap_or(rest.len());
    let authority = &rest[..slash];
    let path = if slash < rest.len() {
        &rest[slash..]
    } else {
        "/"
    };
    let (host, port) = split_host_port(authority, 80)?;
    let mut initial_upstream = format!("{method} {path} {version}\r\n").into_bytes();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if line
            .split_once(':')
            .map(|(name, _)| name.eq_ignore_ascii_case("Proxy-Connection"))
            .unwrap_or(false)
        {
            continue;
        }
        initial_upstream.extend_from_slice(line.as_bytes());
        initial_upstream.extend_from_slice(b"\r\n");
    }
    initial_upstream.extend_from_slice(b"\r\n");
    initial_upstream.extend_from_slice(after_header);

    Ok(HttpProxyRequest {
        host,
        port,
        respond_connect_ok: false,
        initial_upstream,
    })
}

fn split_host_port(value: &str, default_port: u16) -> Result<(String, u16)> {
    if value.is_empty() {
        bail!("empty proxy target host");
    }
    if let Some(rest) = value.strip_prefix('[') {
        let Some((host, remainder)) = rest.split_once(']') else {
            bail!("invalid bracketed IPv6 proxy target");
        };
        let port = if let Some(port) = remainder.strip_prefix(':') {
            parse_port(port)?
        } else {
            default_port
        };
        return Ok((host.to_string(), port));
    }

    let (host, port) = match value.rsplit_once(':') {
        Some((host, port)) if !host.contains(':') => (host, parse_port(port)?),
        Some(_) => bail!("IPv6 proxy targets must use [addr]:port syntax"),
        None => (value, default_port),
    };
    if host.is_empty() {
        bail!("empty proxy target host");
    }
    Ok((host.to_string(), port))
}

fn parse_port(value: &str) -> Result<u16> {
    let port = value.parse::<u16>().context("invalid proxy target port")?;
    if port == 0 {
        bail!("proxy target port must be non-zero");
    }
    Ok(port)
}

async fn socks5_connect_via_transport(
    transport_tx: &mpsc::Sender<ClientTransportEvent>,
    stream_id: u64,
    mut remote: TransportOutputReader,
    host: &str,
    port: u16,
) -> Result<TransportOutputReader> {
    send_transport_bytes(transport_tx, stream_id, vec![0x05, 0x01, 0x00]).await?;
    let method = remote.read_exact_vec(2).await?;
    if method != [0x05, 0x00] {
        bail!("server SOCKS5 egress rejected no-auth method");
    }

    send_transport_bytes(transport_tx, stream_id, socks5_connect_request(host, port)?).await?;
    let head = remote.read_exact_vec(4).await?;
    if head[0] != 0x05 {
        bail!("server SOCKS5 egress returned invalid version");
    }
    if head[1] != 0x00 {
        bail!("server SOCKS5 egress connect failed with code {}", head[1]);
    }
    match head[3] {
        0x01 => {
            let _ = remote.read_exact_vec(6).await?;
        }
        0x03 => {
            let len = remote.read_exact_vec(1).await?[0] as usize;
            let _ = remote.read_exact_vec(len + 2).await?;
        }
        0x04 => {
            let _ = remote.read_exact_vec(18).await?;
        }
        other => bail!("server SOCKS5 egress returned unsupported address type {other}"),
    }
    Ok(remote)
}

fn socks5_connect_request(host: &str, port: u16) -> Result<Vec<u8>> {
    let mut request = vec![0x05, 0x01, 0x00];
    if let Ok(ip) = host.parse::<std::net::IpAddr>() {
        match ip {
            std::net::IpAddr::V4(addr) => {
                request.push(0x01);
                request.extend_from_slice(&addr.octets());
            }
            std::net::IpAddr::V6(addr) => {
                request.push(0x04);
                request.extend_from_slice(&addr.octets());
            }
        }
    } else {
        let host_bytes = host.as_bytes();
        if host_bytes.is_empty() || host_bytes.len() > u8::MAX as usize {
            bail!("domain proxy target must be 1..=255 bytes");
        }
        request.push(0x03);
        request.push(host_bytes.len() as u8);
        request.extend_from_slice(host_bytes);
    }
    request.extend_from_slice(&port.to_be_bytes());
    Ok(request)
}

struct TransportOutputReader {
    output_rx: mpsc::Receiver<ClientStreamOutput>,
    pending: VecDeque<u8>,
}

impl TransportOutputReader {
    fn new(output_rx: mpsc::Receiver<ClientStreamOutput>) -> Self {
        Self {
            output_rx,
            pending: VecDeque::new(),
        }
    }

    async fn read_exact_vec(&mut self, len: usize) -> Result<Vec<u8>> {
        while self.pending.len() < len {
            match self.output_rx.recv().await {
                Some(ClientStreamOutput::Bytes(bytes)) => self.pending.extend(bytes),
                Some(ClientStreamOutput::Close) | None => {
                    bail!("remote stream closed during proxy handshake")
                }
            }
        }
        Ok(self.pending.drain(..len).collect())
    }

    async fn write_to_local(mut self, writer: &mut tokio::net::tcp::OwnedWriteHalf) -> Result<()> {
        if !self.pending.is_empty() {
            let bytes = self.pending.drain(..).collect::<Vec<_>>();
            writer.write_all(&bytes).await?;
        }
        while let Some(output) = self.output_rx.recv().await {
            match output {
                ClientStreamOutput::Bytes(bytes) => {
                    if !bytes.is_empty() {
                        writer.write_all(&bytes).await?;
                    }
                }
                ClientStreamOutput::Close => break,
            }
        }
        writer.shutdown().await.ok();
        Ok(())
    }
}

async fn run_client_transport(
    runtime: Arc<ClientRuntime>,
    event_rx: mpsc::Receiver<ClientTransportEvent>,
) -> Result<()> {
    let (response_tx, response_rx) = mpsc::channel::<ClientDnsResult>(CLIENT_INFLIGHT_WINDOW * 4);
    let now = Instant::now();
    let mut transport = ClientTransport {
        runtime,
        conn_id: rand::random::<u64>(),
        next_packet_no: 0,
        resolver_cursor: 0,
        stream_cursor: 0,
        streams: HashMap::new(),
        stream_order: VecDeque::new(),
        outstanding: HashMap::new(),
        received_server: PacketHistory::default(),
        response_tx,
        response_rx,
        last_ping_sent_at: now.checked_sub(Duration::from_secs(1)).unwrap_or(now),
        active_poll_until: now,
        data_since_poll: 0,
        diag_started: now,
        next_diag_at: now + Duration::from_secs(1),
    };
    transport.run(event_rx).await
}

impl ClientTransport {
    async fn run(&mut self, mut event_rx: mpsc::Receiver<ClientTransportEvent>) -> Result<()> {
        loop {
            self.drain_events(&mut event_rx).await?;
            self.flush_pending_outputs();
            self.cleanup_drained_streams().await;
            self.emit_diag_if_due().await;
            self.fill_outbound_window().await?;

            tokio::select! {
                event = event_rx.recv() => {
                    let Some(event) = event else {
                        if self.streams.is_empty() && self.outstanding.is_empty() {
                            return Ok(());
                        }
                        tokio::time::sleep(Duration::from_millis(2)).await;
                        continue;
                    };
                    self.handle_event(event).await?;
                }
                response = self.response_rx.recv() => {
                    let Some(response) = response else {
                        bail!("client DNS response channel closed");
                    };
                    self.handle_dns_response(response).await?;
                }
                _ = tokio::time::sleep(Duration::from_millis(2)) => {}
            }
        }
    }

    async fn drain_events(
        &mut self,
        event_rx: &mut mpsc::Receiver<ClientTransportEvent>,
    ) -> Result<()> {
        loop {
            match event_rx.try_recv() {
                Ok(event) => self.handle_event(event).await?,
                Err(mpsc::error::TryRecvError::Empty) => return Ok(()),
                Err(mpsc::error::TryRecvError::Disconnected) => return Ok(()),
            }
        }
    }

    async fn handle_event(&mut self, event: ClientTransportEvent) -> Result<()> {
        match event {
            ClientTransportEvent::OpenStream { stream_id, output } => {
                self.streams.insert(stream_id, ClientMuxStream::new(output));
                self.stream_order.push_back(stream_id);
                self.active_poll_until = Instant::now() + CLIENT_ACTIVE_POLL_GRACE;
            }
            ClientTransportEvent::LocalBytes { stream_id, bytes } => {
                if let Some(stream) = self.streams.get_mut(&stream_id) {
                    if !stream.closed && !stream.local_eof {
                        let now = Instant::now();
                        let offset = stream.send_offset;
                        stream
                            .upload_send
                            .append(offset, false, bytes)
                            .context("retain local upload bytes")?;
                        stream.send_offset = stream.upload_send.end_offset();
                        stream.last_activity_at = now;
                        self.active_poll_until = now + CLIENT_ACTIVE_POLL_GRACE;
                    }
                }
            }
            ClientTransportEvent::LocalFin { stream_id } => {
                self.mark_stream_fin(stream_id)?;
            }
            ClientTransportEvent::LocalClosed { stream_id } => {
                self.mark_stream_fin(stream_id)?;
                if let Some(stream) = self.streams.get_mut(&stream_id) {
                    stream.close_requested_at = Some(Instant::now());
                }
            }
        }
        Ok(())
    }

    fn mark_stream_fin(&mut self, stream_id: u64) -> Result<()> {
        if let Some(stream) = self.streams.get_mut(&stream_id) {
            if !stream.local_eof {
                stream.local_eof = true;
                let now = Instant::now();
                let offset = stream.send_offset;
                stream
                    .upload_send
                    .append(offset, true, Vec::new())
                    .context("retain local upload fin")?;
                stream.last_activity_at = now;
                self.active_poll_until = now + CLIENT_ACTIVE_POLL_GRACE;
            }
        }
        Ok(())
    }

    async fn cleanup_drained_streams(&mut self) {
        let drained = self
            .streams
            .iter()
            .filter_map(|(&stream_id, stream)| stream.is_drained().then_some(stream_id))
            .collect::<Vec<_>>();
        for stream_id in drained {
            if let Some(mut stream) = self.streams.remove(&stream_id) {
                send_stream_output_close(&mut stream);
            }
        }
        self.stream_order
            .retain(|stream_id| self.streams.contains_key(stream_id));
        if !self.stream_order.is_empty() {
            self.stream_cursor %= self.stream_order.len();
        } else {
            self.stream_cursor = 0;
        }
    }

    fn flush_pending_outputs(&mut self) {
        let now = Instant::now();
        let mut requested_close = false;
        for stream in self.streams.values_mut() {
            if !flush_stream_output(stream) {
                mark_stream_output_closed(stream, close_due_now(now));
                requested_close = true;
            }
        }
        if requested_close {
            self.active_poll_until = now + CLIENT_ACTIVE_POLL_GRACE;
        }
    }

    async fn emit_diag_if_due(&mut self) {
        let Some(diag) = &self.runtime.diag else {
            return;
        };
        let now = Instant::now();
        if now < self.next_diag_at {
            return;
        }
        let pending_streams = self
            .streams
            .values()
            .filter(|stream| !stream.closed)
            .count();
        let downlink_pending = self
            .streams
            .values()
            .map(|stream| stream.downlink.pending_len())
            .sum::<usize>();
        let mut resolver_snapshots = Vec::with_capacity(self.runtime.config.resolvers.len());
        for (index, resolver) in self.runtime.config.resolvers.iter().enumerate() {
            let health = self.runtime.resolver_health[index].lock().await;
            let resolver_diag = &diag.resolver[index];
            let blocked_ms = health
                .blocked_until
                .and_then(|until| (until > now).then_some(until.duration_since(now).as_millis()))
                .unwrap_or(0) as u64;
            let prefer_tcp_ms = health
                .prefer_tcp_until
                .and_then(|until| (until > now).then_some(until.duration_since(now).as_millis()))
                .unwrap_or(0) as u64;
            resolver_snapshots.push(serde_json::json!({
                "index": index,
                "resolver": resolver.to_string(),
                "cwnd": health.cwnd,
                "in_flight": health.in_flight,
                "failures": health.failures,
                "loss_ewma_ppm": health.loss_ewma_ppm,
                "max_response_bytes": health.max_response_bytes,
                "goodput_ewma_Bps": health.goodput_ewma,
                "pacing_ms": health.pacing_interval.as_millis() as u64,
                "srtt_ms": health.srtt.map(|duration| duration.as_millis() as u64),
                "rttvar_ms": health.rttvar.as_millis() as u64,
                "timeout_ms": health.timeout.as_millis() as u64,
                "blocked_ms": blocked_ms,
                "prefer_tcp_ms": prefer_tcp_ms,
                "sent": resolver_diag.sent.load(Ordering::Relaxed),
                "ok": resolver_diag.ok.load(Ordering::Relaxed),
                "failed": resolver_diag.failed.load(Ordering::Relaxed),
                "control_sent": resolver_diag.control_sent.load(Ordering::Relaxed),
                "data_sent": resolver_diag.data_sent.load(Ordering::Relaxed),
                "poll_sent": resolver_diag.poll_sent.load(Ordering::Relaxed),
                "query_wire_bytes": resolver_diag.query_wire_bytes.load(Ordering::Relaxed),
                "response_wire_bytes": resolver_diag.response_wire_bytes.load(Ordering::Relaxed),
                "useful_response_bytes": resolver_diag.useful_response_bytes.load(Ordering::Relaxed),
                "elapsed_us_sum": resolver_diag.elapsed_us_sum.load(Ordering::Relaxed),
                "elapsed_us_max": resolver_diag.elapsed_us_max.load(Ordering::Relaxed),
                "udp_ok": resolver_diag.udp_ok.load(Ordering::Relaxed),
                "tcp_preferred_ok": resolver_diag.tcp_preferred_ok.load(Ordering::Relaxed),
                "tcp_truncation_ok": resolver_diag.tcp_truncation_ok.load(Ordering::Relaxed),
                "udp_after_tcp_error_ok": resolver_diag.udp_after_tcp_error_ok.load(Ordering::Relaxed),
                "tcp_proxy_ok": resolver_diag.tcp_proxy_ok.load(Ordering::Relaxed),
                "truncated": resolver_diag.truncated.load(Ordering::Relaxed),
            }));
        }
        let payload = serde_json::json!({
            "kind": "client_transport_diag",
            "conn_id": self.conn_id,
            "elapsed_ms": self.diag_started.elapsed().as_millis() as u64,
            "streams": pending_streams,
            "outstanding": self.outstanding.len(),
            "downlink_pending": downlink_pending,
            "queries_sent": diag.queries_sent.load(Ordering::Relaxed),
            "queries_ok": diag.queries_ok.load(Ordering::Relaxed),
            "queries_failed": diag.queries_failed.load(Ordering::Relaxed),
            "query_wire_bytes": diag.query_wire_bytes.load(Ordering::Relaxed),
            "request_packet_body_bytes": diag.request_packet_body_bytes.load(Ordering::Relaxed),
            "request_envelope_bytes": diag.request_envelope_bytes.load(Ordering::Relaxed),
            "request_qname_chars": diag.request_qname_chars.load(Ordering::Relaxed),
            "response_wire_bytes": diag.response_wire_bytes.load(Ordering::Relaxed),
            "response_useful_bytes": diag.response_useful_bytes.load(Ordering::Relaxed),
            "data_bytes_received": diag.data_bytes_received.load(Ordering::Relaxed),
            "data_frames_received": diag.data_frames_received.load(Ordering::Relaxed),
            "upload_data_bytes_sent": diag.upload_data_bytes_sent.load(Ordering::Relaxed),
            "upload_data_frames_sent": diag.upload_data_frames_sent.load(Ordering::Relaxed),
            "upload_new_bytes_sent": diag.upload_new_bytes_sent.load(Ordering::Relaxed),
            "upload_repair_bytes_sent": diag.upload_repair_bytes_sent.load(Ordering::Relaxed),
            "upload_new_packets_sent": diag.upload_new_packets_sent.load(Ordering::Relaxed),
            "upload_repair_packets_sent": diag.upload_repair_packets_sent.load(Ordering::Relaxed),
            "upload_fin_packets_sent": diag.upload_fin_packets_sent.load(Ordering::Relaxed),
            "stream_ack_frames_sent": diag.stream_ack_frames_sent.load(Ordering::Relaxed),
            "packet_ack_ranges_sent": diag.packet_ack_ranges_sent.load(Ordering::Relaxed),
            "open_packets_sent": diag.open_packets_sent.load(Ordering::Relaxed),
            "data_packets_sent": diag.data_packets_sent.load(Ordering::Relaxed),
            "ping_packets_sent": diag.ping_packets_sent.load(Ordering::Relaxed),
            "ping_responses_ok": diag.ping_responses_ok.load(Ordering::Relaxed),
            "ping_responses_with_data": diag.ping_responses_with_data.load(Ordering::Relaxed),
            "ping_response_data_bytes": diag.ping_response_data_bytes.load(Ordering::Relaxed),
            "data_responses_ok": diag.data_responses_ok.load(Ordering::Relaxed),
            "data_response_data_bytes": diag.data_response_data_bytes.load(Ordering::Relaxed),
            "qname_too_long_splits": diag.qname_too_long_splits.load(Ordering::Relaxed),
            "tcp_fallbacks": diag.tcp_fallbacks.load(Ordering::Relaxed),
            "fill_stop_no_kind": diag.fill_stop_no_kind.load(Ordering::Relaxed),
            "fill_stop_no_resolver_control": diag.fill_stop_no_resolver_control.load(Ordering::Relaxed),
            "fill_stop_no_resolver_data": diag.fill_stop_no_resolver_data.load(Ordering::Relaxed),
            "fill_stop_no_resolver_poll": diag.fill_stop_no_resolver_poll.load(Ordering::Relaxed),
            "fill_stop_ping_slot": diag.fill_stop_ping_slot.load(Ordering::Relaxed),
            "resolvers": resolver_snapshots,
        });
        eprintln!("{payload}");
        self.next_diag_at = now + Duration::from_secs(1);
    }

    async fn fill_outbound_window(&mut self) -> Result<()> {
        while self.outstanding.len() < CLIENT_INFLIGHT_WINDOW {
            let Some(kind) = self.next_send_kind() else {
                if let Some(diag) = &self.runtime.diag {
                    diag.fill_stop_no_kind.fetch_add(1, Ordering::Relaxed);
                }
                break;
            };
            let class = mux_send_class(&kind);
            let Some(path) = self
                .runtime
                .pick_resolver(&mut self.resolver_cursor, class)
                .await
            else {
                if let Some(diag) = &self.runtime.diag {
                    match class {
                        ClientSendClass::Control => diag
                            .fill_stop_no_resolver_control
                            .fetch_add(1, Ordering::Relaxed),
                        ClientSendClass::Data => diag
                            .fill_stop_no_resolver_data
                            .fetch_add(1, Ordering::Relaxed),
                        ClientSendClass::Poll => diag
                            .fill_stop_no_resolver_poll
                            .fetch_add(1, Ordering::Relaxed),
                    };
                }
                break;
            };

            let sent_packet_no = self.next_packet_no;
            self.next_packet_no = self
                .next_packet_no
                .checked_add(1)
                .context("client packet number exhausted")?;
            let mut request = Packet::new(self.conn_id, sent_packet_no);
            request.max_response_bytes = path.max_response_bytes;
            request.ack_ranges = self.received_server.ack_ranges(match class {
                ClientSendClass::Data => 2,
                ClientSendClass::Control | ClientSendClass::Poll => 4,
            });

            let mut sent = MuxSentPacket {
                kind,
                stream_acks: Vec::new(),
            };
            self.append_due_stream_acks(&mut request, &mut sent);
            self.append_kind_frames(&mut request, &sent.kind);

            sent.kind = fit_mux_client_request_to_dns_budget(
                &self.runtime.config,
                &mut request,
                sent.kind,
            )?;
            match &sent.kind {
                MuxSentKind::Open {
                    first_data: Some(_),
                    ..
                }
                | MuxSentKind::Data { .. } => {
                    self.data_since_poll = self.data_since_poll.saturating_add(1);
                }
                MuxSentKind::Ping => {
                    self.data_since_poll = 0;
                }
                MuxSentKind::Open {
                    first_data: None, ..
                }
                | MuxSentKind::Close { .. } => {}
            }
            sent.stream_acks = stream_acks_in_request(&request);
            let is_poll = matches!(mux_send_class(&sent.kind), ClientSendClass::Poll);
            let ping_slot = if is_poll {
                let ping_slots = if self.poll_is_active(Instant::now()) {
                    Arc::clone(&self.runtime.active_ping_slots)
                } else {
                    Arc::clone(&self.runtime.idle_ping_slots)
                };
                match ping_slots.try_acquire_owned() {
                    Ok(permit) => {
                        self.last_ping_sent_at = Instant::now();
                        Some(permit)
                    }
                    Err(_) => {
                        if let Some(diag) = &self.runtime.diag {
                            diag.fill_stop_ping_slot.fetch_add(1, Ordering::Relaxed);
                        }
                        self.runtime.release_resolver(path.resolver_index).await;
                        break;
                    }
                }
            } else {
                None
            };

            match &sent.kind {
                MuxSentKind::Open {
                    stream_id,
                    first_data,
                } => {
                    if let Some(stream) = self.streams.get_mut(stream_id) {
                        stream.open_in_flight = true;
                    }
                    if let Some(diag) = &self.runtime.diag {
                        diag.open_packets_sent.fetch_add(1, Ordering::Relaxed);
                        if let Some(slice) = first_data {
                            count_upload_slice_diag(diag, slice);
                        }
                    }
                }
                MuxSentKind::Data { stream_id, slice } => {
                    if let Some(stream) = self.streams.get_mut(stream_id) {
                        stream.upload_send.mark_sent(slice);
                    }
                    if let Some(diag) = &self.runtime.diag {
                        diag.data_packets_sent.fetch_add(1, Ordering::Relaxed);
                        count_upload_slice_diag(diag, slice);
                    }
                }
                MuxSentKind::Close { stream_id } => {
                    if let Some(stream) = self.streams.get_mut(stream_id) {
                        stream.close_in_flight = true;
                    }
                }
                MuxSentKind::Ping => {
                    if let Some(diag) = &self.runtime.diag {
                        diag.ping_packets_sent.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }

            self.outstanding.insert(sent_packet_no, sent);
            let runtime_for_query = Arc::clone(&self.runtime);
            let response_tx = self.response_tx.clone();
            tokio::spawn(async move {
                let _ping_slot = ping_slot;
                let started = Instant::now();
                let result = send_dns_packet(
                    &runtime_for_query,
                    Some(path.resolver_index),
                    path.resolver,
                    Some(class),
                    &request,
                    path.timeout,
                )
                .await;
                let elapsed = started.elapsed();
                let truncated = result
                    .as_ref()
                    .map(|outcome| outcome.truncated)
                    .unwrap_or(false);
                let response_wire_bytes = result
                    .as_ref()
                    .map(|outcome| outcome.response_wire_bytes)
                    .unwrap_or(0);
                let transport = result.as_ref().ok().map(|outcome| outcome.transport);
                let useful_bytes = result
                    .as_ref()
                    .map(|outcome| packet_useful_data_bytes(&outcome.packet))
                    .unwrap_or(0);
                if let Some(diag) = &runtime_for_query.diag {
                    diag.response_useful_bytes
                        .fetch_add(useful_bytes as u64, Ordering::Relaxed);
                    if let Some(resolver_diag) = diag.resolver.get(path.resolver_index) {
                        if result.is_ok() {
                            resolver_diag.ok.fetch_add(1, Ordering::Relaxed);
                        } else {
                            resolver_diag.failed.fetch_add(1, Ordering::Relaxed);
                        }
                        resolver_diag
                            .response_wire_bytes
                            .fetch_add(response_wire_bytes as u64, Ordering::Relaxed);
                        resolver_diag
                            .useful_response_bytes
                            .fetch_add(useful_bytes as u64, Ordering::Relaxed);
                        let elapsed_us = elapsed.as_micros().min(u128::from(u64::MAX)) as u64;
                        resolver_diag
                            .elapsed_us_sum
                            .fetch_add(elapsed_us, Ordering::Relaxed);
                        resolver_diag
                            .elapsed_us_max
                            .fetch_max(elapsed_us, Ordering::Relaxed);
                        if truncated {
                            resolver_diag.truncated.fetch_add(1, Ordering::Relaxed);
                        }
                        match transport {
                            Some(DnsTransportOutcome::Udp) => {
                                resolver_diag.udp_ok.fetch_add(1, Ordering::Relaxed);
                            }
                            Some(DnsTransportOutcome::TcpPreferred) => {
                                resolver_diag
                                    .tcp_preferred_ok
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                            Some(DnsTransportOutcome::TcpAfterUdpFailure) => {
                                resolver_diag
                                    .tcp_preferred_ok
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                            Some(DnsTransportOutcome::TcpFallbackAfterTruncation) => {
                                resolver_diag
                                    .tcp_truncation_ok
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                            Some(DnsTransportOutcome::UdpAfterPreferredTcpError) => {
                                resolver_diag
                                    .udp_after_tcp_error_ok
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                            Some(DnsTransportOutcome::TcpProxy) => {
                                resolver_diag.tcp_proxy_ok.fetch_add(1, Ordering::Relaxed);
                            }
                            None => {}
                        }
                    }
                }
                let request_too_large = result
                    .as_ref()
                    .err()
                    .map(|error| error.to_string().contains("exceeds 253 bytes"))
                    .unwrap_or(false);
                if request_too_large {
                    runtime_for_query
                        .release_resolver(path.resolver_index)
                        .await;
                } else {
                    runtime_for_query
                        .record_resolver_result(
                            path.resolver_index,
                            result.is_ok(),
                            elapsed,
                            truncated,
                            useful_bytes,
                            transport,
                        )
                        .await;
                }
                let result = result.map(|outcome| outcome.packet);
                let _ = response_tx
                    .send(ClientDnsResult {
                        resolver: path.resolver,
                        packet_no: sent_packet_no,
                        response_wire_bytes,
                        result,
                    })
                    .await;
            });
        }
        Ok(())
    }

    fn append_due_stream_acks(&mut self, request: &mut Packet, sent: &mut MuxSentPacket) {
        let len = self.stream_order.len();
        if len == 0 {
            return;
        }
        let start = self.stream_cursor % len;
        for offset in 0..len {
            if request.frames.len() >= 16 {
                break;
            }
            let stream_id = self.stream_order[(start + offset) % len];
            let Some(stream) = self.streams.get_mut(&stream_id) else {
                continue;
            };
            let frame = stream.downlink.stream_ack_frame(
                stream_id,
                CLIENT_RECEIVE_WINDOW,
                CLIENT_STREAM_ACK_RANGES,
            );
            let Some(state) = stream_ack_state(&frame) else {
                continue;
            };
            if stream.last_stream_ack_advertised.as_ref() == Some(&state) {
                continue;
            }
            sent.stream_acks.push((stream_id, state));
            request.frames.push(frame);
        }
    }

    fn append_kind_frames(&mut self, request: &mut Packet, kind: &MuxSentKind) {
        match kind {
            MuxSentKind::Open {
                stream_id,
                first_data,
            } => {
                request.frames.push(Frame::Open {
                    stream_id: *stream_id,
                    host: String::new(),
                    port: 0,
                });
                if let Some(segment) = first_data {
                    request.frames.push(Frame::Data {
                        stream_id: *stream_id,
                        offset: segment.offset,
                        fin: segment.fin,
                        bytes: segment.bytes.clone(),
                    });
                }
            }
            MuxSentKind::Data { stream_id, slice } => {
                request.frames.push(Frame::Data {
                    stream_id: *stream_id,
                    offset: slice.offset,
                    fin: slice.fin,
                    bytes: slice.bytes.clone(),
                });
            }
            MuxSentKind::Close { stream_id } => {
                request.frames.push(Frame::Close {
                    stream_id: *stream_id,
                    code: 0,
                });
            }
            MuxSentKind::Ping => {
                request.frames.push(Frame::Ping {
                    nonce: request.packet_no,
                });
            }
        }
    }

    fn next_send_kind(&mut self) -> Option<MuxSentKind> {
        if let Some(stream_id) = self.choose_stream_for_close() {
            return Some(MuxSentKind::Close { stream_id });
        }
        if let Some(stream_id) = self.choose_stream_for_open() {
            let upload_chunk = self.upload_chunk();
            let first_data = self
                .streams
                .get(&stream_id)
                .and_then(|stream| stream.upload_send.peek_next(upload_chunk));
            return Some(MuxSentKind::Open {
                stream_id,
                first_data,
            });
        }
        let now = Instant::now();
        if self.poll_should_preempt_data(now) {
            return Some(MuxSentKind::Ping);
        }
        if let Some((stream_id, slice)) = self.choose_stream_for_data() {
            return Some(MuxSentKind::Data { stream_id, slice });
        }
        let ping_inflight = self
            .outstanding
            .values()
            .filter(|sent| matches!(sent.kind, MuxSentKind::Ping))
            .count();
        let poll_demand = self.streams.values().any(|stream| stream.wants_poll(now));
        let can_poll = poll_demand
            || now
                .checked_duration_since(self.last_ping_sent_at)
                .map(|elapsed| elapsed >= self.poll_interval(now))
                .unwrap_or(true);
        (can_poll && ping_inflight < self.ping_inflight_limit(now)).then_some(MuxSentKind::Ping)
    }

    fn poll_should_preempt_data(&self, now: Instant) -> bool {
        if self.data_since_poll < CLIENT_ACTIVE_POLL_DATA_BUDGET {
            return false;
        }
        if !self.streams.values().any(|stream| stream.wants_poll(now)) {
            return false;
        }
        let ping_inflight = self
            .outstanding
            .values()
            .filter(|sent| matches!(sent.kind, MuxSentKind::Ping))
            .count();
        ping_inflight < self.ping_inflight_limit(now)
            && now
                .checked_duration_since(self.last_ping_sent_at)
                .map(|elapsed| elapsed >= CLIENT_ACTIVE_POLL_INTERVAL)
                .unwrap_or(true)
    }

    fn choose_stream_for_open(&mut self) -> Option<u64> {
        self.choose_stream_matching(|stream| {
            !stream.opened
                && !stream.open_in_flight
                && !stream.closed
                && (stream.upload_send.has_pending_send() || stream.local_eof)
        })
    }

    fn choose_stream_for_close(&mut self) -> Option<u64> {
        let now = Instant::now();
        self.choose_stream_matching(|stream| {
            let reset_due = stream
                .close_requested_at
                .map(|requested| {
                    now.saturating_duration_since(requested) >= Duration::from_millis(500)
                })
                .unwrap_or(false);
            let graceful_done = stream.local_eof
                && stream.upload_send.is_finished()
                && stream.downlink.is_finished();
            stream.opened
                && !stream.close_in_flight
                && !stream.closed
                && (reset_due || graceful_done)
        })
    }

    fn choose_stream_for_data(&mut self) -> Option<(u64, SendBufferSlice)> {
        let upload_chunk = self.upload_chunk();
        let stream_id = self.choose_stream_matching(|stream| {
            stream.opened && !stream.closed && stream.upload_send.peek_next(upload_chunk).is_some()
        })?;
        let slice = self
            .streams
            .get(&stream_id)?
            .upload_send
            .peek_next(upload_chunk)?;
        Some((stream_id, slice))
    }

    fn choose_stream_matching<F>(&mut self, mut predicate: F) -> Option<u64>
    where
        F: FnMut(&ClientMuxStream) -> bool,
    {
        let len = self.stream_order.len();
        if len == 0 {
            return None;
        }
        for _ in 0..len {
            let index = self.stream_cursor % len;
            self.stream_cursor = self.stream_cursor.wrapping_add(1);
            let stream_id = self.stream_order[index];
            if self
                .streams
                .get(&stream_id)
                .map(&mut predicate)
                .unwrap_or(false)
            {
                return Some(stream_id);
            }
        }
        None
    }

    fn upload_chunk(&self) -> usize {
        if self.runtime.config.tcp_first_resolver_path() {
            UPLOAD_SEND_CHUNK_CONSTRAINED
        } else {
            UPLOAD_SEND_CHUNK_NORMAL
        }
    }

    fn poll_is_active(&self, now: Instant) -> bool {
        self.active_poll_until > now || self.streams.values().any(|stream| stream.wants_poll(now))
    }

    fn poll_interval(&self, now: Instant) -> Duration {
        client_poll_interval(self.poll_is_active(now), self.runtime.config.poll_interval)
    }

    fn ping_inflight_limit(&self, now: Instant) -> usize {
        if self.streams.values().any(|stream| stream.wants_poll(now)) {
            CLIENT_PING_INFLIGHT_ACTIVE
        } else {
            CLIENT_PING_INFLIGHT_IDLE
        }
    }

    fn mark_stream_acks_advertised(&mut self, stream_acks: Vec<(u64, StreamAckState)>) {
        for (stream_id, ack_state) in stream_acks {
            if let Some(stream) = self.streams.get_mut(&stream_id) {
                let should_update = stream
                    .last_stream_ack_advertised
                    .as_ref()
                    .map(|last| {
                        ack_state.cumulative_offset >= last.cumulative_offset
                            && ack_state.max_stream_data >= last.max_stream_data
                    })
                    .unwrap_or(true);
                if should_update {
                    stream.last_stream_ack_advertised = Some(ack_state);
                }
            }
        }
    }

    fn apply_sent_packet_ack(&mut self, kind: MuxSentKind, acked: bool) {
        match kind {
            MuxSentKind::Open {
                stream_id,
                first_data,
            } => {
                if let Some(stream) = self.streams.get_mut(&stream_id) {
                    stream.open_in_flight = false;
                    if acked {
                        stream.opened = true;
                        self.active_poll_until = Instant::now() + CLIENT_ACTIVE_POLL_GRACE;
                        if let Some(slice) = &first_data {
                            stream.upload_send.mark_sent(slice);
                        }
                    }
                }
            }
            MuxSentKind::Data { .. } | MuxSentKind::Ping => {}
            MuxSentKind::Close { stream_id } => {
                if let Some(stream) = self.streams.get_mut(&stream_id) {
                    if acked {
                        stream.closed = true;
                        send_stream_output_close(stream);
                    } else {
                        stream.close_in_flight = false;
                    }
                }
            }
        }
    }

    async fn handle_dns_response(&mut self, result: ClientDnsResult) -> Result<()> {
        let Some(sent) = self.outstanding.remove(&result.packet_no) else {
            return Ok(());
        };
        match result.result {
            Ok(response) => {
                let response_useful_bytes = packet_useful_data_bytes(&response);
                if response.conn_id != self.conn_id {
                    if let Some(diag) = &self.runtime.diag {
                        diag.queries_failed.fetch_add(1, Ordering::Relaxed);
                        diag.response_wire_bytes
                            .fetch_add(result.response_wire_bytes as u64, Ordering::Relaxed);
                    }
                    self.apply_sent_packet_ack(sent.kind, false);
                    eprintln!(
                        "resolver {} packet {} returned response for unexpected connection {}",
                        result.resolver, result.packet_no, response.conn_id
                    );
                    return Ok(());
                }
                if self.received_server.is_acked(response.packet_no) {
                    if let Some(diag) = &self.runtime.diag {
                        diag.queries_failed.fetch_add(1, Ordering::Relaxed);
                        diag.response_wire_bytes
                            .fetch_add(result.response_wire_bytes as u64, Ordering::Relaxed);
                    }
                    self.apply_sent_packet_ack(sent.kind, false);
                    return Ok(());
                }
                if let Some(diag) = &self.runtime.diag {
                    diag.queries_ok.fetch_add(1, Ordering::Relaxed);
                    diag.response_wire_bytes
                        .fetch_add(result.response_wire_bytes as u64, Ordering::Relaxed);
                    match &sent.kind {
                        MuxSentKind::Ping => {
                            diag.ping_responses_ok.fetch_add(1, Ordering::Relaxed);
                            diag.ping_response_data_bytes
                                .fetch_add(response_useful_bytes as u64, Ordering::Relaxed);
                            if response_useful_bytes > 0 {
                                diag.ping_responses_with_data
                                    .fetch_add(1, Ordering::Relaxed);
                            }
                        }
                        MuxSentKind::Data { .. } => {
                            diag.data_responses_ok.fetch_add(1, Ordering::Relaxed);
                            diag.data_response_data_bytes
                                .fetch_add(response_useful_bytes as u64, Ordering::Relaxed);
                        }
                        MuxSentKind::Open { .. } | MuxSentKind::Close { .. } => {}
                    }
                }
                let acked = ack_ranges_contain(&response.ack_ranges, result.packet_no);
                if acked {
                    self.mark_stream_acks_advertised(sent.stream_acks);
                }
                self.apply_sent_packet_ack(sent.kind, acked);

                self.received_server.insert(response.packet_no);
                for frame in response.frames {
                    self.handle_response_frame(frame).await?;
                }
            }
            Err(error) => {
                if let Some(diag) = &self.runtime.diag {
                    diag.queries_failed.fetch_add(1, Ordering::Relaxed);
                }
                let request_too_large = error.to_string().contains("exceeds 253 bytes");
                match sent.kind {
                    MuxSentKind::Open { stream_id, .. } => {
                        if let Some(stream) = self.streams.get_mut(&stream_id) {
                            stream.open_in_flight = false;
                        }
                    }
                    MuxSentKind::Data { .. } if request_too_large => {
                        if let Some(diag) = &self.runtime.diag {
                            diag.qname_too_long_splits.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    MuxSentKind::Close { stream_id } => {
                        if let Some(stream) = self.streams.get_mut(&stream_id) {
                            stream.close_in_flight = false;
                        }
                    }
                    MuxSentKind::Data { .. } | MuxSentKind::Ping => {}
                }
                eprintln!(
                    "resolver {} packet {} failed: {error:#}",
                    result.resolver, result.packet_no
                );
            }
        }
        Ok(())
    }

    async fn handle_response_frame(&mut self, frame: Frame) -> Result<()> {
        match frame {
            Frame::Data {
                stream_id,
                offset,
                fin,
                bytes,
            } => {
                let Some(stream) = self.streams.get_mut(&stream_id) else {
                    return Ok(());
                };
                if !bytes.is_empty() || fin {
                    let now = Instant::now();
                    stream.last_activity_at = now;
                    self.active_poll_until = now + CLIENT_ACTIVE_POLL_GRACE;
                }
                if let Some(diag) = &self.runtime.diag {
                    diag.data_frames_received.fetch_add(1, Ordering::Relaxed);
                    diag.data_bytes_received
                        .fetch_add(bytes.len() as u64, Ordering::Relaxed);
                }
                let ready = stream
                    .downlink
                    .try_insert_with_window(offset, fin, bytes, CLIENT_RECEIVE_WINDOW)
                    .context("apply server stream data")?;
                if !ready.is_empty() {
                    queue_stream_output(stream, ready, Instant::now());
                    if !flush_stream_output(stream) {
                        mark_stream_output_closed(stream, close_due_now(Instant::now()));
                        self.active_poll_until = Instant::now() + CLIENT_ACTIVE_POLL_GRACE;
                    }
                }
            }
            Frame::Close { stream_id, .. } => {
                if let Some(stream) = self.streams.get_mut(&stream_id) {
                    let now = Instant::now();
                    stream.last_activity_at = now;
                    send_stream_output_close(stream);
                    request_stream_protocol_close(stream, close_due_now(now));
                    self.active_poll_until = now + CLIENT_ACTIVE_POLL_GRACE;
                }
            }
            Frame::StreamAck {
                stream_id,
                cumulative_offset,
                max_stream_data,
                fin_offset,
                ranges,
            } => {
                if let Some(stream) = self.streams.get_mut(&stream_id) {
                    stream.upload_send.apply_stream_ack(
                        cumulative_offset,
                        &ranges,
                        max_stream_data,
                        fin_offset,
                    );
                }
            }
            Frame::Open { .. }
            | Frame::Ping { .. }
            | Frame::PathChallenge { .. }
            | Frame::PathResponse { .. } => {}
        }
        Ok(())
    }
}

fn queue_stream_output(stream: &mut ClientMuxStream, bytes: Vec<u8>, now: Instant) {
    if bytes.is_empty() || stream.closed {
        return;
    }
    stream.pending_output_bytes = stream.pending_output_bytes.saturating_add(bytes.len());
    stream.pending_output.push_back(bytes);
    if stream.pending_output_bytes > CLIENT_STREAM_PENDING_OUTPUT_BYTES {
        mark_stream_output_closed(stream, close_due_now(now));
    }
}

fn flush_stream_output(stream: &mut ClientMuxStream) -> bool {
    while let Some(bytes) = stream.pending_output.pop_front() {
        let len = bytes.len();
        match stream.output.try_send(ClientStreamOutput::Bytes(bytes)) {
            Ok(()) => {
                stream.pending_output_bytes = stream.pending_output_bytes.saturating_sub(len);
            }
            Err(mpsc::error::TrySendError::Full(ClientStreamOutput::Bytes(bytes))) => {
                stream.pending_output.push_front(bytes);
                return true;
            }
            Err(mpsc::error::TrySendError::Closed(_)) => return false,
            Err(mpsc::error::TrySendError::Full(ClientStreamOutput::Close)) => return true,
        }
    }
    true
}

fn close_due_now(now: Instant) -> Instant {
    now.checked_sub(CLIENT_RESET_CLOSE_DELAY).unwrap_or(now)
}

fn request_stream_protocol_close(stream: &mut ClientMuxStream, requested_at: Instant) {
    if stream
        .close_requested_at
        .map(|current| requested_at < current)
        .unwrap_or(true)
    {
        stream.close_requested_at = Some(requested_at);
    }
}

fn send_stream_output_close(stream: &mut ClientMuxStream) {
    if stream.local_output_closed {
        return;
    }
    stream.local_output_closed = true;
    let _ = stream.output.try_send(ClientStreamOutput::Close);
}

fn mark_stream_output_closed(stream: &mut ClientMuxStream, requested_at: Instant) {
    stream.pending_output.clear();
    stream.pending_output_bytes = 0;
    send_stream_output_close(stream);
    request_stream_protocol_close(stream, requested_at);
}

async fn send_dns_packet(
    runtime: &ClientRuntime,
    resolver_index: Option<usize>,
    resolver: SocketAddr,
    class: Option<ClientSendClass>,
    packet: &Packet,
    query_timeout: Duration,
) -> Result<DnsPacketOutcome> {
    send_dns_packet_inner(
        runtime,
        resolver_index,
        resolver,
        class,
        packet,
        query_timeout,
        false,
    )
    .await
}

async fn send_dns_packet_inner(
    runtime: &ClientRuntime,
    resolver_index: Option<usize>,
    resolver: SocketAddr,
    class: Option<ClientSendClass>,
    packet: &Packet,
    query_timeout: Duration,
    direct_tcp_first: bool,
) -> Result<DnsPacketOutcome> {
    let config = &runtime.config;
    let envelope = seal_packet(&config.access_key, Direction::ClientToServer, packet)?;
    let qname = envelope_to_qname(&envelope, &config.domain)?;
    let dns_id = (packet.packet_no as u16).wrapping_mul(31).wrapping_add(7);
    let query = build_query(dns_id, &qname, packet.max_response_bytes)?;
    if let Some(diag) = &runtime.diag {
        diag.queries_sent.fetch_add(1, Ordering::Relaxed);
        diag.query_wire_bytes
            .fetch_add(query.len() as u64, Ordering::Relaxed);
        diag.request_packet_body_bytes
            .fetch_add(packet.encoded_len() as u64, Ordering::Relaxed);
        diag.request_envelope_bytes
            .fetch_add(envelope.len() as u64, Ordering::Relaxed);
        diag.request_qname_chars
            .fetch_add(qname.len() as u64, Ordering::Relaxed);
        let frame_stats = request_frame_diag(packet);
        diag.upload_data_bytes_sent
            .fetch_add(frame_stats.data_bytes as u64, Ordering::Relaxed);
        diag.upload_data_frames_sent
            .fetch_add(frame_stats.data_frames as u64, Ordering::Relaxed);
        diag.stream_ack_frames_sent
            .fetch_add(frame_stats.stream_ack_frames as u64, Ordering::Relaxed);
        diag.packet_ack_ranges_sent
            .fetch_add(packet.ack_ranges.len() as u64, Ordering::Relaxed);
        if let Some(index) = resolver_index.and_then(|index| diag.resolver.get(index)) {
            index.sent.fetch_add(1, Ordering::Relaxed);
            index
                .query_wire_bytes
                .fetch_add(query.len() as u64, Ordering::Relaxed);
            match class {
                Some(ClientSendClass::Control) => {
                    index.control_sent.fetch_add(1, Ordering::Relaxed);
                }
                Some(ClientSendClass::Data) => {
                    index.data_sent.fetch_add(1, Ordering::Relaxed);
                }
                Some(ClientSendClass::Poll) => {
                    index.poll_sent.fetch_add(1, Ordering::Relaxed);
                }
                None => {}
            }
        }
    }
    let prefer_tcp = direct_tcp_first
        || match resolver_index {
            Some(index) => runtime.prefer_tcp_for_resolver(index).await,
            None => false,
        };
    let (response, transport) = if let Some(pool) = &runtime.tcp_pool {
        (
            pool.query(resolver, &query, query_timeout).await?,
            DnsTransportOutcome::TcpProxy,
        )
    } else if prefer_tcp {
        match runtime
            .tcp_fallback_pool
            .query(resolver, &query, query_timeout.max(PATH_RTO_MIN_TCP))
            .await
        {
            Ok(tcp_response) => {
                return Ok(DnsPacketOutcome {
                    packet: open_dns_response(&config.access_key, &tcp_response)?,
                    response_wire_bytes: tcp_response.len(),
                    truncated: false,
                    transport: DnsTransportOutcome::TcpPreferred,
                });
            }
            Err(tcp_error) => {
                if config.resolver_transport == ResolverTransportMode::Tcp {
                    runtime.tcp_fallback_pool.remove_sender(resolver).await;
                    return Err(tcp_error.context("DNS-over-TCP resolver query failed"));
                }
                eprintln!("resolver {resolver} preferred TCP failed ({tcp_error:#}); retrying UDP");
                runtime.tcp_fallback_pool.remove_sender(resolver).await;
                (
                    runtime
                        .udp_pool
                        .query(resolver, &query, query_timeout)
                        .await?,
                    DnsTransportOutcome::UdpAfterPreferredTcpError,
                )
            }
        }
    } else {
        let response = match runtime
            .udp_pool
            .query(resolver, &query, query_timeout)
            .await
        {
            Ok(response) => response,
            Err(udp_error) => {
                if !config.allow_udp_to_tcp_fallback() {
                    return Err(udp_error.context("UDP DNS response failed"));
                }
                if let Some(diag) = &runtime.diag {
                    diag.tcp_fallbacks.fetch_add(1, Ordering::Relaxed);
                }
                eprintln!("resolver {resolver} UDP failed ({udp_error:#}); retrying over TCP");
                match runtime
                    .tcp_fallback_pool
                    .query(resolver, &query, query_timeout.max(PATH_RTO_MIN_TCP))
                    .await
                {
                    Ok(tcp_response) => {
                        return Ok(DnsPacketOutcome {
                            packet: open_dns_response(&config.access_key, &tcp_response)?,
                            response_wire_bytes: tcp_response.len(),
                            truncated: false,
                            transport: DnsTransportOutcome::TcpAfterUdpFailure,
                        });
                    }
                    Err(tcp_error) => {
                        runtime.tcp_fallback_pool.remove_sender(resolver).await;
                        return Err(tcp_error.context(format!(
                            "UDP DNS response failed ({udp_error:#}); DNS-over-TCP fallback failed"
                        )));
                    }
                }
            }
        };
        match open_dns_response(&config.access_key, &response) {
            Ok(packet) => {
                return Ok(DnsPacketOutcome {
                    packet,
                    response_wire_bytes: response.len(),
                    truncated: false,
                    transport: DnsTransportOutcome::Udp,
                });
            }
            Err(_error) if dns_response_is_truncated(&response) => {
                if let Some(diag) = &runtime.diag {
                    diag.tcp_fallbacks.fetch_add(1, Ordering::Relaxed);
                }
                let udp_response_len = response.len();
                eprintln!(
                    "resolver {resolver} returned truncated UDP DNS response; retrying over TCP"
                );
                let tcp_response = runtime
                    .tcp_fallback_pool
                    .query(resolver, &query, query_timeout.max(PATH_RTO_MIN_TCP))
                    .await?;
                return Ok(DnsPacketOutcome {
                    packet: open_dns_response(&config.access_key, &tcp_response)?,
                    response_wire_bytes: tcp_response.len().max(udp_response_len),
                    truncated: true,
                    transport: DnsTransportOutcome::TcpFallbackAfterTruncation,
                });
            }
            Err(error) => return Err(error),
        }
    };
    Ok(DnsPacketOutcome {
        packet: open_dns_response(&config.access_key, &response)?,
        response_wire_bytes: response.len(),
        truncated: false,
        transport,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DnsTransportOutcome {
    Udp,
    TcpPreferred,
    TcpAfterUdpFailure,
    TcpFallbackAfterTruncation,
    UdpAfterPreferredTcpError,
    TcpProxy,
}

struct DnsPacketOutcome {
    packet: Packet,
    response_wire_bytes: usize,
    truncated: bool,
    transport: DnsTransportOutcome,
}

#[derive(Default)]
struct RequestFrameDiag {
    data_frames: usize,
    data_bytes: usize,
    stream_ack_frames: usize,
}

fn request_frame_diag(packet: &Packet) -> RequestFrameDiag {
    packet
        .frames
        .iter()
        .fold(RequestFrameDiag::default(), |mut diag, frame| {
            match frame {
                Frame::Data { bytes, .. } => {
                    diag.data_frames += 1;
                    diag.data_bytes += bytes.len();
                }
                Frame::StreamAck { .. } => {
                    diag.stream_ack_frames += 1;
                }
                _ => {}
            }
            diag
        })
}

fn count_upload_slice_diag(diag: &ClientDiag, slice: &SendBufferSlice) {
    if slice.fin {
        diag.upload_fin_packets_sent.fetch_add(1, Ordering::Relaxed);
    }
    match slice.mode {
        SendBufferMode::New => {
            diag.upload_new_packets_sent.fetch_add(1, Ordering::Relaxed);
            diag.upload_new_bytes_sent
                .fetch_add(slice.bytes.len() as u64, Ordering::Relaxed);
        }
        SendBufferMode::Repair => {
            diag.upload_repair_packets_sent
                .fetch_add(1, Ordering::Relaxed);
            diag.upload_repair_bytes_sent
                .fetch_add(slice.bytes.len() as u64, Ordering::Relaxed);
        }
    }
}

fn packet_useful_data_bytes(packet: &Packet) -> usize {
    packet
        .frames
        .iter()
        .map(|frame| match frame {
            Frame::Data { bytes, .. } => bytes.len(),
            _ => 0,
        })
        .sum()
}

fn open_dns_response(key: &ClientAccessKey, response: &[u8]) -> Result<Packet> {
    let envelope = parse_txt_response(response)?;
    open_packet_with_key(key, Direction::ServerToClient, &envelope)
}

fn dns_response_is_truncated(response: &[u8]) -> bool {
    response
        .get(2..4)
        .and_then(|flags| flags.try_into().ok())
        .map(u16::from_be_bytes)
        .map(|flags| flags & 0x0200 != 0)
        .unwrap_or(false)
}

async fn run_udp_resolver_actor(resolver: SocketAddr, mut requests: mpsc::Receiver<DnsUdpRequest>) {
    if let Err(error) = serve_udp_resolver(resolver, &mut requests).await {
        eprintln!("resolver {resolver} persistent UDP worker failed: {error:#}");
    }
}

async fn serve_udp_resolver(
    resolver: SocketAddr,
    requests: &mut mpsc::Receiver<DnsUdpRequest>,
) -> Result<()> {
    let bind = if resolver.is_ipv6() {
        "[::]:0"
    } else {
        "0.0.0.0:0"
    };
    let socket = UdpSocket::bind(bind).await?;
    socket
        .connect(resolver)
        .await
        .with_context(|| format!("connect UDP resolver {resolver}"))?;
    let mut next_dns_id = rand::random::<u16>();
    let mut inflight = HashMap::<u16, UdpInFlight>::new();
    let mut buf = vec![0u8; 4096];

    loop {
        expire_udp_inflight(&mut inflight);
        tokio::select! {
            request = requests.recv() => {
                let Some(request) = request else {
                    fail_udp_inflight(&mut inflight, "UDP resolver worker stopped");
                    return Ok(());
                };
                submit_dns_udp_request(resolver, &socket, &mut next_dns_id, &mut inflight, request).await?;
            }
            received = socket.recv(&mut buf), if !inflight.is_empty() => {
                let len = received.with_context(|| format!("read UDP DNS response from {resolver}"))?;
                if len < 2 {
                    continue;
                }
                let dns_id = u16::from_be_bytes(buf[0..2].try_into().unwrap());
                if let Some(inflight) = inflight.remove(&dns_id) {
                    let _ = inflight.response.send(Ok(buf[..len].to_vec()));
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(25)), if !inflight.is_empty() => {
                expire_udp_inflight(&mut inflight);
            }
        }
    }
}

async fn submit_dns_udp_request(
    resolver: SocketAddr,
    socket: &UdpSocket,
    next_dns_id: &mut u16,
    inflight: &mut HashMap<u16, UdpInFlight>,
    mut request: DnsUdpRequest,
) -> Result<()> {
    let dns_id = allocate_udp_dns_id(next_dns_id, inflight)
        .context("all UDP DNS message IDs are in flight")?;
    if request.query.len() < 2 {
        let _ = request
            .response
            .send(Err(anyhow::anyhow!("DNS query too short")));
        return Ok(());
    }
    request.query[0..2].copy_from_slice(&dns_id.to_be_bytes());
    match socket.send(&request.query).await {
        Ok(_) => {
            let expires_at = Instant::now() + request.timeout;
            inflight.insert(
                dns_id,
                UdpInFlight {
                    response: request.response,
                    expires_at,
                },
            );
            Ok(())
        }
        Err(error) => {
            let _ = request.response.send(Err(anyhow::anyhow!(
                "write UDP query to {resolver}: {error}"
            )));
            fail_udp_inflight(inflight, "UDP write failed");
            Err(error).context("write UDP DNS query")
        }
    }
}

fn allocate_udp_dns_id(next: &mut u16, inflight: &HashMap<u16, UdpInFlight>) -> Option<u16> {
    for _ in 0..=u16::MAX {
        let candidate = *next;
        *next = next.wrapping_add(1);
        if !inflight.contains_key(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn expire_udp_inflight(inflight: &mut HashMap<u16, UdpInFlight>) {
    let now = Instant::now();
    let expired = inflight
        .iter()
        .filter_map(|(dns_id, request)| (request.expires_at <= now).then_some(*dns_id))
        .collect::<Vec<_>>();
    for dns_id in expired {
        if let Some(request) = inflight.remove(&dns_id) {
            let _ = request
                .response
                .send(Err(anyhow::anyhow!("UDP DNS response timed out")));
        }
    }
}

fn fail_udp_inflight(inflight: &mut HashMap<u16, UdpInFlight>, message: &'static str) {
    for (_, request) in inflight.drain() {
        let _ = request.response.send(Err(anyhow::anyhow!(message)));
    }
}

async fn run_tcp_resolver_actor(
    proxy: Option<SocketAddr>,
    resolver: SocketAddr,
    mut requests: mpsc::Receiver<DnsTcpRequest>,
) {
    loop {
        let Some(first_request) = requests.recv().await else {
            return;
        };
        match connect_dns_tcp(proxy, resolver).await {
            Ok(stream) => {
                if let Err(error) =
                    serve_tcp_resolver_connection(resolver, stream, &mut requests, first_request)
                        .await
                {
                    eprintln!("resolver {resolver} persistent TCP connection failed: {error:#}");
                    tokio::time::sleep(TCP_RECONNECT_DELAY).await;
                }
            }
            Err(error) => {
                let _ = first_request.response.send(Err(anyhow::anyhow!(
                    "DNS-over-TCP connect failed: {error:#}"
                )));
                eprintln!("resolver {resolver} DNS-over-TCP connect failed: {error:#}");
                tokio::time::sleep(TCP_RECONNECT_DELAY).await;
            }
        }
    }
}

async fn connect_dns_tcp(proxy: Option<SocketAddr>, resolver: SocketAddr) -> Result<TcpStream> {
    if let Some(proxy) = proxy {
        let mut stream = timeout(CLIENT_QUERY_TIMEOUT, TcpStream::connect(proxy))
            .await
            .context("connect SOCKS proxy timed out")?
            .with_context(|| format!("connect SOCKS proxy {proxy}"))?;
        stream.set_nodelay(true).ok();
        timeout(CLIENT_QUERY_TIMEOUT, socks5_connect(&mut stream, resolver))
            .await
            .context("SOCKS handshake timed out")??;
        return Ok(stream);
    }

    let stream = timeout(CLIENT_QUERY_TIMEOUT, TcpStream::connect(resolver))
        .await
        .with_context(|| format!("connect DNS-over-TCP resolver {resolver} timed out"))?
        .with_context(|| format!("connect DNS-over-TCP resolver {resolver}"))?;
    stream.set_nodelay(true).ok();
    Ok(stream)
}

async fn serve_tcp_resolver_connection(
    resolver: SocketAddr,
    stream: TcpStream,
    requests: &mut mpsc::Receiver<DnsTcpRequest>,
    first_request: DnsTcpRequest,
) -> Result<()> {
    let (mut reader, mut writer) = stream.into_split();
    let (reader_tx, mut reader_rx) = mpsc::channel::<Result<Vec<u8>>>(TCP_RESOLVER_QUEUE);
    tokio::spawn(async move {
        loop {
            let response = read_dns_tcp_message(&mut reader).await;
            let done = response.is_err();
            if reader_tx.send(response).await.is_err() || done {
                break;
            }
        }
    });
    let mut next_dns_id = rand::random::<u16>();
    let mut inflight = HashMap::<u16, DnsTcpInFlight>::new();
    let mut next_request = Some(first_request);

    loop {
        if expire_tcp_inflight(&mut inflight) {
            return Err(anyhow::anyhow!("DNS-over-TCP lane timed out"));
        }
        while inflight.is_empty() {
            match reader_rx.try_recv() {
                Ok(Ok(_)) => {}
                Ok(Err(error)) => return Err(error).context("read DNS-over-TCP response"),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => break,
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => return Ok(()),
            }
        }
        if let Some(request) = next_request.take() {
            submit_dns_tcp_request(
                resolver,
                &mut writer,
                &mut next_dns_id,
                &mut inflight,
                request,
            )
            .await?;
            continue;
        }

        tokio::select! {
            response = reader_rx.recv(), if !inflight.is_empty() => {
                let response = match response {
                    Some(Ok(response)) => response,
                    Some(Err(error)) => {
                        fail_inflight(&mut inflight, "DNS-over-TCP read failed");
                        return Err(error).context("read DNS-over-TCP response");
                    }
                    None => {
                        fail_inflight(&mut inflight, "DNS-over-TCP reader stopped");
                        return Ok(());
                    }
                };
                if response.len() < 2 {
                    continue;
                }
                let dns_id = u16::from_be_bytes(response[0..2].try_into().unwrap());
                if let Some(inflight) = inflight.remove(&dns_id) {
                    let _ = inflight.response.send(Ok(response));
                }
            }
            request = requests.recv() => {
                let Some(request) = request else {
                    fail_inflight(&mut inflight, "resolver worker stopped");
                    return Ok(());
                };
                submit_dns_tcp_request(resolver, &mut writer, &mut next_dns_id, &mut inflight, request).await?;
            }
            _ = tokio::time::sleep(Duration::from_millis(25)), if !inflight.is_empty() => {
                if expire_tcp_inflight(&mut inflight) {
                    return Err(anyhow::anyhow!("DNS-over-TCP lane timed out"));
                }
            }
        }
    }
}

async fn submit_dns_tcp_request(
    resolver: SocketAddr,
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    next_dns_id: &mut u16,
    inflight: &mut HashMap<u16, DnsTcpInFlight>,
    mut request: DnsTcpRequest,
) -> Result<()> {
    let dns_id = allocate_dns_id(next_dns_id, inflight)
        .context("all DNS-over-TCP message IDs are in flight")?;
    if request.query.len() < 2 {
        let _ = request
            .response
            .send(Err(anyhow::anyhow!("DNS query too short")));
        return Ok(());
    }
    request.query[0..2].copy_from_slice(&dns_id.to_be_bytes());
    match write_dns_tcp_message(writer, &request.query).await {
        Ok(()) => {
            let sent_at = Instant::now();
            inflight.insert(
                dns_id,
                DnsTcpInFlight {
                    response: request.response,
                    sent_at,
                    timeout: request.timeout,
                    expires_at: sent_at + request.timeout,
                },
            );
            Ok(())
        }
        Err(error) => {
            let _ = request.response.send(Err(anyhow::anyhow!(
                "write DNS-over-TCP query to {resolver}: {error}"
            )));
            fail_inflight(inflight, "DNS-over-TCP write failed");
            Err(error).context("write DNS-over-TCP query")
        }
    }
}

fn allocate_dns_id(next: &mut u16, inflight: &HashMap<u16, DnsTcpInFlight>) -> Option<u16> {
    for _ in 0..=u16::MAX {
        let candidate = *next;
        *next = next.wrapping_add(1);
        if !inflight.contains_key(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn fail_inflight(inflight: &mut HashMap<u16, DnsTcpInFlight>, message: &'static str) {
    for (_, request) in inflight.drain() {
        let _ = request.response.send(Err(anyhow::anyhow!(message)));
    }
}

fn expire_tcp_inflight(inflight: &mut HashMap<u16, DnsTcpInFlight>) -> bool {
    let now = Instant::now();
    let expired = inflight
        .iter()
        .filter_map(|(dns_id, request)| (request.expires_at <= now).then_some(*dns_id))
        .collect::<Vec<_>>();
    if expired.is_empty() {
        return false;
    }
    let hol = inflight.len() >= 16
        && inflight
            .values()
            .filter(|request| now.saturating_duration_since(request.sent_at) >= request.timeout)
            .count()
            >= 8;
    for dns_id in expired {
        if let Some(request) = inflight.remove(&dns_id) {
            let _ = request
                .response
                .send(Err(anyhow::anyhow!("DNS-over-TCP response timed out")));
        }
    }
    if hol {
        fail_inflight(inflight, "DNS-over-TCP lane closed after HOL timeout");
    }
    hol
}

async fn write_dns_tcp_message<W>(writer: &mut W, query: &[u8]) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    if query.len() > u16::MAX as usize {
        bail!("DNS query too large for TCP framing");
    }
    let mut message = Vec::with_capacity(query.len() + 2);
    message.extend_from_slice(&(query.len() as u16).to_be_bytes());
    message.extend_from_slice(query);
    timeout(DNS_TCP_WRITE_TIMEOUT, async {
        writer.write_all(&message).await?;
        writer.flush().await
    })
    .await
    .context("DNS-over-TCP write timed out")??;
    Ok(())
}

async fn read_dns_tcp_message<R>(reader: &mut R) -> Result<Vec<u8>>
where
    R: AsyncRead + Unpin,
{
    let mut len_buf = [0u8; 2];
    timeout(CLIENT_QUERY_TIMEOUT, reader.read_exact(&mut len_buf))
        .await
        .context("DNS-over-TCP length timed out")??;
    let len = u16::from_be_bytes(len_buf) as usize;
    let mut response = vec![0u8; len];
    timeout(CLIENT_QUERY_TIMEOUT, reader.read_exact(&mut response))
        .await
        .context("DNS-over-TCP response timed out")??;
    Ok(response)
}

async fn read_dns_tcp_message_unbounded<R>(reader: &mut R) -> Result<Vec<u8>>
where
    R: AsyncRead + Unpin,
{
    let mut len_buf = [0u8; 2];
    timeout(SERVER_TCP_READ_TIMEOUT, reader.read_exact(&mut len_buf))
        .await
        .context("TCP DNS length timed out")??;
    let len = u16::from_be_bytes(len_buf) as usize;
    let mut response = vec![0u8; len];
    timeout(SERVER_TCP_READ_TIMEOUT, reader.read_exact(&mut response))
        .await
        .context("TCP DNS message timed out")??;
    Ok(response)
}

async fn socks5_connect(stream: &mut TcpStream, target: SocketAddr) -> Result<()> {
    stream.write_all(&[0x05, 0x01, 0x00]).await?;
    let mut greeting = [0u8; 2];
    stream.read_exact(&mut greeting).await?;
    if greeting != [0x05, 0x00] {
        bail!("SOCKS proxy rejected no-auth method");
    }

    let mut req = Vec::with_capacity(32);
    req.extend_from_slice(&[0x05, 0x01, 0x00]);
    match target {
        SocketAddr::V4(addr) => {
            req.push(0x01);
            req.extend_from_slice(&addr.ip().octets());
            req.extend_from_slice(&addr.port().to_be_bytes());
        }
        SocketAddr::V6(addr) => {
            req.push(0x04);
            req.extend_from_slice(&addr.ip().octets());
            req.extend_from_slice(&addr.port().to_be_bytes());
        }
    }
    stream.write_all(&req).await?;

    let mut head = [0u8; 4];
    stream.read_exact(&mut head).await?;
    if head[0] != 0x05 || head[1] != 0x00 {
        bail!("SOCKS connect failed with code {}", head[1]);
    }
    let extra = match head[3] {
        0x01 => 4 + 2,
        0x03 => {
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).await?;
            len[0] as usize + 2
        }
        0x04 => 16 + 2,
        other => bail!("SOCKS reply used unsupported address type {other}"),
    };
    let mut discard = vec![0u8; extra];
    stream.read_exact(&mut discard).await?;
    Ok(())
}

pub async fn run_server(config: ServerConfig) -> Result<()> {
    let udp = UdpSocket::bind(config.bind)
        .await
        .with_context(|| format!("bind UDP DNS {}", config.bind))?;
    let tcp = TcpListener::bind(config.bind)
        .await
        .with_context(|| format!("bind TCP DNS {}", config.bind))?;
    let shared = Arc::new(ServerState::new(config));

    eprintln!("trajectory server listening on {}", udp.local_addr()?);
    let udp_state = Arc::clone(&shared);
    tokio::spawn(async move {
        if let Err(error) = run_udp_server(udp_state, udp).await {
            eprintln!("UDP server failed: {error:#}");
        }
    });

    loop {
        let (stream, peer) = tcp.accept().await?;
        let state = Arc::clone(&shared);
        tokio::spawn(async move {
            if let Err(error) = run_tcp_dns_connection(state, stream).await {
                eprintln!("TCP DNS connection from {peer} failed: {error:#}");
            }
        });
    }
}

struct ServerState {
    config: ServerConfig,
    sessions: Mutex<HashMap<SessionKey, Arc<SessionHandle>>>,
    connections: Mutex<HashMap<ConnectionKey, Arc<ServerConnection>>>,
    next_cleanup_at: StdMutex<Instant>,
}

impl ServerState {
    fn new(config: ServerConfig) -> Self {
        Self {
            config,
            sessions: Mutex::new(HashMap::new()),
            connections: Mutex::new(HashMap::new()),
            next_cleanup_at: StdMutex::new(Instant::now() + SERVER_STATE_CLEANUP_INTERVAL),
        }
    }

    async fn connection(&self, client_id: u32, conn_id: u64) -> Arc<ServerConnection> {
        let key = (client_id, conn_id);
        let connection = {
            let mut connections = self.connections.lock().await;
            connections
                .entry(key)
                .or_insert_with(|| Arc::new(ServerConnection::default()))
                .clone()
        };
        connection.touch();
        connection
    }

    async fn session(&self, key: SessionKey) -> Option<Arc<SessionHandle>> {
        let session = self.sessions.lock().await.get(&key).cloned();
        if let Some(session) = &session {
            session.touch();
        }
        session
    }

    async fn get_or_create_session(&self, key: SessionKey) -> Arc<SessionHandle> {
        let mut sessions = self.sessions.lock().await;
        if let Some(handle) = sessions.get(&key) {
            handle.touch();
            return Arc::clone(handle);
        }

        let (upload_tx, upload_rx) = mpsc::channel(SERVER_UPLOAD_QUEUE);
        let (download_tx, download_rx) = mpsc::channel(SERVER_DOWNLOAD_QUEUE);
        let handle = Arc::new(SessionHandle::new(upload_tx, download_rx));
        sessions.insert(key, Arc::clone(&handle));

        let target = self.config.target;
        let target_mode = self.config.target_mode;
        tokio::spawn(async move {
            let result = match target_mode {
                ServerTargetMode::Tcp => run_server_session(target, upload_rx, download_tx).await,
                ServerTargetMode::Socks5Direct => {
                    run_server_socks5_direct_session(upload_rx, download_tx).await
                }
            };
            if let Err(error) = result {
                eprintln!("server target session failed: {error:#}");
            }
        });
        handle
    }

    async fn remove_session(&self, key: SessionKey) {
        self.sessions.lock().await.remove(&key);
    }

    async fn remove_terminal_sessions_acked(
        &self,
        client_id: u32,
        conn_id: u64,
        packet_ack_ranges: &[AckRange],
    ) {
        let candidates = {
            let sessions = self.sessions.lock().await;
            sessions
                .iter()
                .filter_map(
                    |(&(session_client_id, session_conn_id, stream_id), session)| {
                        (session_client_id == client_id && session_conn_id == conn_id).then_some((
                            (session_client_id, session_conn_id, stream_id),
                            Arc::clone(session),
                        ))
                    },
                )
                .collect::<Vec<_>>()
        };

        let mut remove = Vec::new();
        for (session_key, session) in candidates {
            let terminal_close = session.terminal_close_sent.lock().await;
            if terminal_close
                .as_ref()
                .map(|sent| ack_ranges_contain(packet_ack_ranges, sent.packet_no))
                .unwrap_or(false)
            {
                remove.push(session_key);
            }
        }
        if remove.is_empty() {
            return;
        }

        let mut sessions = self.sessions.lock().await;
        for key in remove {
            sessions.remove(&key);
        }
    }

    async fn sessions_for_connection(
        &self,
        client_id: u32,
        conn_id: u64,
        preferred_streams: &[u64],
    ) -> Vec<(u64, Arc<SessionHandle>)> {
        let sessions = self.sessions.lock().await;
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        for stream_id in preferred_streams {
            let key = (client_id, conn_id, *stream_id);
            if let Some(session) = sessions.get(&key) {
                out.push((*stream_id, Arc::clone(session)));
                seen.insert(*stream_id);
            }
        }
        let mut rest = sessions
            .iter()
            .filter_map(
                |(&(session_client_id, session_conn_id, stream_id), session)| {
                    (session_client_id == client_id
                        && session_conn_id == conn_id
                        && !seen.contains(&stream_id))
                    .then_some((stream_id, Arc::clone(session)))
                },
            )
            .collect::<Vec<_>>();
        rest.sort_by_key(|(stream_id, _)| *stream_id);
        out.extend(rest);
        for (_, session) in &out {
            session.touch();
        }
        out
    }

    async fn cleanup_idle(&self, now: Instant) {
        let due = {
            let Ok(mut next_cleanup_at) = self.next_cleanup_at.lock() else {
                return;
            };
            if now < *next_cleanup_at {
                false
            } else {
                *next_cleanup_at = now + SERVER_STATE_CLEANUP_INTERVAL;
                true
            }
        };
        if !due {
            return;
        }
        let cutoff = now.checked_sub(SERVER_STATE_IDLE_TIMEOUT).unwrap_or(now);

        self.sessions
            .lock()
            .await
            .retain(|_, session| session.last_activity() > cutoff);
        self.connections
            .lock()
            .await
            .retain(|_, connection| connection.last_activity() > cutoff);
    }
}

type ConnectionKey = (u32, u64);
type SessionKey = (u32, u64, u64);

struct ServerConnection {
    seen_client: Mutex<PacketHistory>,
    next_packet_no: Mutex<u64>,
    response_cache: Mutex<ResponseCache>,
    download_cursor: Mutex<usize>,
    last_activity: StdMutex<Instant>,
}

impl Default for ServerConnection {
    fn default() -> Self {
        Self {
            seen_client: Mutex::new(PacketHistory::default()),
            next_packet_no: Mutex::new(0),
            response_cache: Mutex::new(ResponseCache::default()),
            download_cursor: Mutex::new(0),
            last_activity: StdMutex::new(Instant::now()),
        }
    }
}

impl ServerConnection {
    fn touch(&self) {
        if let Ok(mut last_activity) = self.last_activity.lock() {
            *last_activity = Instant::now();
        }
    }

    fn last_activity(&self) -> Instant {
        self.last_activity
            .lock()
            .map(|last_activity| *last_activity)
            .unwrap_or_else(|_| Instant::now())
    }

    async fn ack_ranges_after_insert(
        &self,
        packet_no: u64,
    ) -> (Vec<trajectory_core::codec::AckRange>, bool) {
        let mut seen = self.seen_client.lock().await;
        let duplicate = seen.is_acked(packet_no);
        seen.insert(packet_no);
        (seen.ack_ranges(4), duplicate)
    }

    async fn next_packet_no(&self) -> Result<u64> {
        let mut next = self.next_packet_no.lock().await;
        let packet_no = *next;
        *next = next
            .checked_add(1)
            .context("server packet number exhausted")?;
        Ok(packet_no)
    }

    async fn cached_response(&self, client_packet_no: u64) -> Option<Vec<u8>> {
        self.response_cache
            .lock()
            .await
            .envelopes
            .get(&client_packet_no)
            .cloned()
    }

    async fn cache_response(&self, client_packet_no: u64, envelope: Vec<u8>) {
        self.response_cache
            .lock()
            .await
            .insert(client_packet_no, envelope);
    }

    async fn rotate_download_sessions(
        &self,
        mut sessions: Vec<(u64, Arc<SessionHandle>)>,
    ) -> Vec<(u64, Arc<SessionHandle>)> {
        if sessions.len() <= 1 {
            return sessions;
        }
        let mut cursor = self.download_cursor.lock().await;
        let start = *cursor % sessions.len();
        sessions.rotate_left(start);
        *cursor = cursor.wrapping_add(1) % sessions.len();
        sessions
    }
}

#[derive(Default)]
struct ResponseCache {
    order: VecDeque<u64>,
    envelopes: HashMap<u64, Vec<u8>>,
}

impl ResponseCache {
    fn insert(&mut self, packet_no: u64, envelope: Vec<u8>) {
        if !self.envelopes.contains_key(&packet_no) {
            self.order.push_back(packet_no);
        }
        self.envelopes.insert(packet_no, envelope);
        while self.order.len() > SERVER_RESPONSE_CACHE {
            if let Some(oldest) = self.order.pop_front() {
                self.envelopes.remove(&oldest);
            }
        }
    }
}

struct SessionHandle {
    upload_tx: mpsc::Sender<UploadFrame>,
    upload_recv: Mutex<StreamAssembler>,
    upload_finished: Mutex<bool>,
    last_upload_ack_sent: Mutex<Option<SentUploadAck>>,
    terminal_close_sent: Mutex<Option<SentTerminalClose>>,
    download_rx: Mutex<mpsc::Receiver<DownloadFrame>>,
    download_send: Mutex<RetainedByteSendBuffer>,
    last_activity: StdMutex<Instant>,
}

impl SessionHandle {
    fn new(
        upload_tx: mpsc::Sender<UploadFrame>,
        download_rx: mpsc::Receiver<DownloadFrame>,
    ) -> Self {
        Self {
            upload_tx,
            upload_recv: Mutex::new(StreamAssembler::default()),
            upload_finished: Mutex::new(false),
            last_upload_ack_sent: Mutex::new(None),
            terminal_close_sent: Mutex::new(None),
            download_rx: Mutex::new(download_rx),
            download_send: Mutex::new(RetainedByteSendBuffer::default()),
            last_activity: StdMutex::new(Instant::now()),
        }
    }

    fn touch(&self) {
        if let Ok(mut last_activity) = self.last_activity.lock() {
            *last_activity = Instant::now();
        }
    }

    fn last_activity(&self) -> Instant {
        self.last_activity
            .lock()
            .map(|last_activity| *last_activity)
            .unwrap_or_else(|_| Instant::now())
    }
}

struct SentUploadAck {
    state: StreamAckState,
    packet_no: u64,
    sent_at: Instant,
}

struct SentTerminalClose {
    packet_no: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StreamAckState {
    cumulative_offset: u64,
    max_stream_data: u64,
    fin_offset: Option<u64>,
    ranges: Vec<StreamRange>,
}

struct UploadFrame {
    fin: bool,
    bytes: Vec<u8>,
}

struct DownloadFrame {
    offset: u64,
    fin: bool,
    bytes: Vec<u8>,
}

async fn run_server_session(
    target: SocketAddr,
    mut upload_rx: mpsc::Receiver<UploadFrame>,
    download_tx: mpsc::Sender<DownloadFrame>,
) -> Result<()> {
    let stream = TcpStream::connect(target)
        .await
        .with_context(|| format!("connect target {target}"))?;
    let (mut reader, mut writer) = stream.into_split();

    let upload_task = tokio::spawn(async move {
        let mut pending = Vec::with_capacity(SERVER_UPLOAD_COALESCE_BYTES);
        while let Some(first) = upload_rx.recv().await {
            pending.clear();
            pending.extend_from_slice(&first.bytes);
            let mut fin = first.fin;
            let coalesce_delay = tokio::time::sleep(SERVER_UPLOAD_COALESCE_DELAY);
            tokio::pin!(coalesce_delay);

            loop {
                while pending.len() < SERVER_UPLOAD_COALESCE_BYTES {
                    match upload_rx.try_recv() {
                        Ok(frame) => {
                            pending.extend_from_slice(&frame.bytes);
                            fin |= frame.fin;
                            if fin {
                                break;
                            }
                        }
                        Err(mpsc::error::TryRecvError::Empty) => break,
                        Err(mpsc::error::TryRecvError::Disconnected) => {
                            fin = true;
                            break;
                        }
                    }
                }
                if fin || pending.len() >= SERVER_UPLOAD_COALESCE_BYTES {
                    break;
                }
                tokio::select! {
                    _ = &mut coalesce_delay => break,
                    maybe_frame = upload_rx.recv() => {
                        match maybe_frame {
                            Some(frame) => {
                                pending.extend_from_slice(&frame.bytes);
                                fin |= frame.fin;
                            }
                            None => {
                                fin = true;
                                break;
                            }
                        }
                    }
                }
            }

            if !pending.is_empty() {
                writer.write_all(&pending).await?;
            }
            if fin {
                writer.shutdown().await?;
                break;
            }
        }
        Ok::<(), anyhow::Error>(())
    });

    let mut offset = 0u64;
    let mut buf = vec![0u8; SERVER_TARGET_READ_CHUNK];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            let _ = download_tx
                .send(DownloadFrame {
                    offset,
                    fin: true,
                    bytes: Vec::new(),
                })
                .await;
            break;
        }
        let bytes = buf[..n].to_vec();
        download_tx
            .send(DownloadFrame {
                offset,
                fin: false,
                bytes,
            })
            .await
            .context("queue target download bytes")?;
        offset += n as u64;
    }

    upload_task.abort();
    let _ = upload_task.await;
    Ok(())
}

struct UploadFrameReader {
    upload_rx: mpsc::Receiver<UploadFrame>,
    pending: VecDeque<u8>,
    fin: bool,
}

impl UploadFrameReader {
    fn new(upload_rx: mpsc::Receiver<UploadFrame>) -> Self {
        Self {
            upload_rx,
            pending: VecDeque::new(),
            fin: false,
        }
    }

    async fn read_exact_vec(&mut self, len: usize) -> Result<Vec<u8>> {
        while self.pending.len() < len {
            if self.fin {
                bail!("SOCKS stream ended before {len} bytes were available");
            }
            let Some(frame) = self.upload_rx.recv().await else {
                self.fin = true;
                continue;
            };
            self.pending.extend(frame.bytes);
            self.fin |= frame.fin;
        }
        Ok(self.pending.drain(..len).collect())
    }

    fn take_pending(&mut self) -> Vec<u8> {
        self.pending.drain(..).collect()
    }

    async fn next_chunk(&mut self) -> Option<Vec<u8>> {
        if !self.pending.is_empty() {
            return Some(self.take_pending());
        }
        if self.fin {
            return None;
        }
        let frame = self.upload_rx.recv().await?;
        self.fin |= frame.fin;
        Some(frame.bytes)
    }

    fn is_finished(&self) -> bool {
        self.fin && self.pending.is_empty()
    }
}

async fn run_server_socks5_direct_session(
    upload_rx: mpsc::Receiver<UploadFrame>,
    download_tx: mpsc::Sender<DownloadFrame>,
) -> Result<()> {
    let mut upload = UploadFrameReader::new(upload_rx);
    let mut download_offset = 0u64;

    let greeting = upload.read_exact_vec(2).await?;
    if greeting.first().copied() != Some(0x05) {
        bail!("SOCKS client used unsupported version");
    }
    let method_count = greeting[1] as usize;
    let methods = upload.read_exact_vec(method_count).await?;
    if !methods.contains(&0x00) {
        send_download_chunk(&download_tx, &mut download_offset, vec![0x05, 0xff], true).await?;
        return Ok(());
    }
    send_download_chunk(&download_tx, &mut download_offset, vec![0x05, 0x00], false).await?;

    let head = upload.read_exact_vec(4).await?;
    if head.len() != 4 || head[0] != 0x05 || head[1] != 0x01 || head[2] != 0x00 {
        send_socks5_reply(&download_tx, &mut download_offset, 0x07, true).await?;
        return Ok(());
    }

    let atyp = head[3];
    let target = match atyp {
        0x01 => {
            let octets = upload.read_exact_vec(4).await?;
            let port = read_socks_port(&mut upload).await?;
            SocketTarget::Ip(SocketAddr::from((
                std::net::Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
                port,
            )))
        }
        0x03 => {
            let len = upload.read_exact_vec(1).await?[0] as usize;
            let name = String::from_utf8(upload.read_exact_vec(len).await?)
                .context("SOCKS domain is not valid UTF-8")?;
            let port = read_socks_port(&mut upload).await?;
            SocketTarget::Domain(name, port)
        }
        0x04 => {
            let octets = upload.read_exact_vec(16).await?;
            let mut ip = [0u8; 16];
            ip.copy_from_slice(&octets);
            let port = read_socks_port(&mut upload).await?;
            SocketTarget::Ip(SocketAddr::from((std::net::Ipv6Addr::from(ip), port)))
        }
        _ => {
            send_socks5_reply(&download_tx, &mut download_offset, 0x08, true).await?;
            return Ok(());
        }
    };

    let upstream = match connect_socket_target(target).await {
        Ok(stream) => stream,
        Err(_) => {
            send_socks5_reply(&download_tx, &mut download_offset, 0x01, true).await?;
            return Ok(());
        }
    };
    send_socks5_reply(&download_tx, &mut download_offset, 0x00, false).await?;

    let (mut upstream_reader, mut upstream_writer) = upstream.into_split();
    let upload_task = tokio::spawn(async move {
        let mut pending = Vec::with_capacity(SERVER_UPLOAD_COALESCE_BYTES);
        if !upload.pending.is_empty() {
            pending.extend_from_slice(&upload.take_pending());
        }
        loop {
            if pending.is_empty() {
                let Some(chunk) = upload.next_chunk().await else {
                    break;
                };
                pending.extend_from_slice(&chunk);
            }

            let coalesce_delay = tokio::time::sleep(SERVER_UPLOAD_COALESCE_DELAY);
            tokio::pin!(coalesce_delay);
            loop {
                if upload.is_finished() || pending.len() >= SERVER_UPLOAD_COALESCE_BYTES {
                    break;
                }
                tokio::select! {
                    _ = &mut coalesce_delay => break,
                    maybe_chunk = upload.next_chunk() => {
                        match maybe_chunk {
                            Some(chunk) => pending.extend_from_slice(&chunk),
                            None => break,
                        }
                    }
                }
            }

            if !pending.is_empty() {
                upstream_writer.write_all(&pending).await?;
                pending.clear();
            }
            if upload.is_finished() {
                break;
            }
        }
        upstream_writer.shutdown().await.ok();
        Ok::<(), anyhow::Error>(())
    });

    let mut buf = vec![0u8; SERVER_TARGET_READ_CHUNK];
    loop {
        let n = upstream_reader.read(&mut buf).await?;
        if n == 0 {
            send_download_chunk(&download_tx, &mut download_offset, Vec::new(), true).await?;
            break;
        }
        send_download_chunk(&download_tx, &mut download_offset, buf[..n].to_vec(), false).await?;
    }
    upload_task.abort();
    let _ = upload_task.await;
    Ok(())
}

enum SocketTarget {
    Ip(SocketAddr),
    Domain(String, u16),
}

async fn read_socks_port(upload: &mut UploadFrameReader) -> Result<u16> {
    let port = upload.read_exact_vec(2).await?;
    Ok(u16::from_be_bytes([port[0], port[1]]))
}

async fn connect_socket_target(target: SocketTarget) -> Result<TcpStream> {
    match target {
        SocketTarget::Ip(addr) => TcpStream::connect(addr)
            .await
            .with_context(|| format!("connect SOCKS target {addr}")),
        SocketTarget::Domain(host, port) => TcpStream::connect((host.as_str(), port))
            .await
            .with_context(|| format!("connect SOCKS target {host}:{port}")),
    }
}

async fn send_socks5_reply(
    download_tx: &mpsc::Sender<DownloadFrame>,
    offset: &mut u64,
    code: u8,
    fin: bool,
) -> Result<()> {
    send_download_chunk(
        download_tx,
        offset,
        vec![0x05, code, 0x00, 0x01, 0, 0, 0, 0, 0, 0],
        fin,
    )
    .await
}

async fn send_download_chunk(
    download_tx: &mpsc::Sender<DownloadFrame>,
    offset: &mut u64,
    bytes: Vec<u8>,
    fin: bool,
) -> Result<()> {
    let frame_offset = *offset;
    *offset = offset.saturating_add(bytes.len() as u64);
    download_tx
        .send(DownloadFrame {
            offset: frame_offset,
            fin,
            bytes,
        })
        .await
        .context("queue SOCKS direct download bytes")
}

async fn run_udp_server(state: Arc<ServerState>, socket: UdpSocket) -> Result<()> {
    let socket = Arc::new(socket);
    let query_slots = Arc::new(Semaphore::new(SERVER_UDP_QUERY_CONCURRENCY));
    let mut buf = vec![0u8; 4096];
    loop {
        let slot = Arc::clone(&query_slots).acquire_owned().await?;
        let (len, peer) = socket.recv_from(&mut buf).await?;
        let query = buf[..len].to_vec();
        let state = Arc::clone(&state);
        let socket = Arc::clone(&socket);
        tokio::spawn(async move {
            let _slot = slot;
            match handle_dns_query(state, &query).await {
                Ok(response) => {
                    if let Err(error) = socket.send_to(&response, peer).await {
                        eprintln!("failed to send UDP response to {peer}: {error:#}");
                    }
                }
                Err(error) => {
                    eprintln!("drop invalid UDP query from {peer}: {error:#}");
                }
            }
        });
    }
}

async fn run_tcp_dns_connection(state: Arc<ServerState>, stream: TcpStream) -> Result<()> {
    stream.set_nodelay(true).ok();
    let (mut reader, mut writer) = stream.into_split();
    let (responses_tx, mut responses_rx) = mpsc::channel::<Vec<u8>>(128);

    let writer_task = tokio::spawn(async move {
        while let Some(response) = responses_rx.recv().await {
            write_dns_tcp_message(&mut writer, &response).await?;
        }
        Ok::<(), anyhow::Error>(())
    });

    loop {
        let query = match read_dns_tcp_message_unbounded(&mut reader).await {
            Ok(query) => query,
            Err(_) => break,
        };
        let state = Arc::clone(&state);
        let responses_tx = responses_tx.clone();
        tokio::spawn(async move {
            match handle_dns_query(state, &query).await {
                Ok(response) => {
                    let _ = responses_tx.send(response).await;
                }
                Err(error) => {
                    eprintln!("drop invalid TCP query: {error:#}");
                }
            }
        });
    }
    drop(responses_tx);
    match writer_task.await {
        Ok(result) => result,
        Err(error) => Err(error).context("TCP DNS writer task failed"),
    }
}

async fn handle_dns_query(state: Arc<ServerState>, query_bytes: &[u8]) -> Result<Vec<u8>> {
    let query = parse_query(query_bytes)?;
    if query.qclass == CLASS_IN && is_in_domain(&query.qname, &state.config.domain) {
        match query.qtype {
            TYPE_A => return build_a_response(&query, 0),
            TYPE_AAAA => return build_aaaa_response(&query, 0),
            TYPE_NS => return build_ns_response(&query, &state.config.domain, 0),
            TYPE_SOA => return build_soa_response(&query, &state.config.domain, 0),
            _ => {}
        }
    }
    if query.qtype != TYPE_TXT || query.qclass != CLASS_IN {
        return build_empty_response(&query, 0);
    }
    let envelope = match qname_to_envelope(&query.qname, &state.config.domain) {
        Ok(envelope) => envelope,
        Err(_) => return build_empty_response(&query, 0),
    };
    let (key, packet) = match open_packet_with_registry(
        &state.config.authorized_clients,
        Direction::ClientToServer,
        &envelope,
    ) {
        Ok(opened) => opened,
        Err(_) => return build_empty_response(&query, 0),
    };

    state.cleanup_idle(Instant::now()).await;
    let connection = state.connection(key.client_id, packet.conn_id).await;
    if let Some(envelope) = connection.cached_response(packet.packet_no).await {
        return build_txt_response(&query, &envelope, 0);
    }

    let mut response = Packet::new(packet.conn_id, connection.next_packet_no().await?);
    response.max_response_bytes = query.udp_payload_size.unwrap_or(packet.max_response_bytes);
    let (ack_ranges, duplicate_client_packet) =
        connection.ack_ranges_after_insert(packet.packet_no).await;
    response.ack_ranges = ack_ranges;
    let client_packet_ack_ranges = packet.ack_ranges.clone();
    state
        .remove_terminal_sessions_acked(key.client_id, packet.conn_id, &client_packet_ack_ranges)
        .await;
    let mut client_stream_acks = Vec::<(u64, u64, u64, Option<u64>, Vec<StreamRange>)>::new();
    let mut active_streams = Vec::<u64>::new();
    let mut force_upload_ack_retransmit = duplicate_client_packet;

    for frame in packet.frames {
        match frame {
            Frame::StreamAck {
                stream_id,
                cumulative_offset,
                max_stream_data,
                fin_offset,
                ranges,
            } => {
                push_unique_stream(&mut active_streams, stream_id);
                client_stream_acks.push((
                    stream_id,
                    cumulative_offset,
                    max_stream_data,
                    fin_offset,
                    ranges,
                ));
            }
            Frame::Open { stream_id, .. } => {
                push_unique_stream(&mut active_streams, stream_id);
                if !duplicate_client_packet {
                    state
                        .get_or_create_session((key.client_id, packet.conn_id, stream_id))
                        .await;
                }
            }
            Frame::Data {
                stream_id,
                offset,
                fin,
                bytes,
            } => {
                push_unique_stream(&mut active_streams, stream_id);
                if duplicate_client_packet {
                    continue;
                }
                let session_key = (key.client_id, packet.conn_id, stream_id);
                if let Some(session) = state.session(session_key).await {
                    let mut upload = session.upload_recv.lock().await;
                    let ready = upload
                        .try_insert_with_window(offset, fin, bytes, SERVER_RECEIVE_WINDOW)
                        .context("apply client stream data")?;
                    let upload_finished = upload.is_finished();
                    drop(upload);

                    if !ready.is_empty() {
                        let _ = session
                            .upload_tx
                            .send(UploadFrame {
                                fin: false,
                                bytes: ready,
                            })
                            .await;
                    }
                    if upload_finished {
                        let mut finished = session.upload_finished.lock().await;
                        if !*finished {
                            *finished = true;
                            let _ = session
                                .upload_tx
                                .send(UploadFrame {
                                    fin: true,
                                    bytes: Vec::new(),
                                })
                                .await;
                        }
                    }
                }
            }
            Frame::Close { stream_id, .. } => {
                push_unique_stream(&mut active_streams, stream_id);
                force_upload_ack_retransmit = true;
                if !duplicate_client_packet {
                    state
                        .remove_session((key.client_id, packet.conn_id, stream_id))
                        .await;
                }
            }
            Frame::PathChallenge {
                nonce,
                response_bytes,
            } => {
                if let Some(frame) =
                    path_response_that_fits(&query, &key, &response, nonce, response_bytes)?
                {
                    response.frames.push(frame);
                }
            }
            Frame::Ping { .. } => {}
            Frame::PathResponse { .. } => {}
        }
    }

    let sessions = state
        .sessions_for_connection(key.client_id, packet.conn_id, &active_streams)
        .await;
    let sessions = connection.rotate_download_sessions(sessions).await;
    for (stream_id, session) in &sessions {
        let upload_ack = {
            let upload = session.upload_recv.lock().await;
            upload.stream_ack_frame(*stream_id, SERVER_RECEIVE_WINDOW, SERVER_STREAM_ACK_RANGES)
        };
        let upload_ack_state = stream_ack_state(&upload_ack);
        let should_send_upload_ack = match upload_ack_state.as_ref() {
            Some(state) => {
                should_send_upload_ack(
                    session,
                    state,
                    force_upload_ack_retransmit,
                    &client_packet_ack_ranges,
                )
                .await
            }
            None => false,
        };
        if should_send_upload_ack && response_frame_fits(&query, &key, &response, &upload_ack)? {
            response.frames.push(upload_ack);
        }
        if let Some((_, cumulative_offset, max_stream_data, fin_offset, ranges)) =
            client_stream_acks
                .iter()
                .find(|(ack_stream_id, _, _, _, _)| ack_stream_id == stream_id)
        {
            session.download_send.lock().await.apply_stream_ack(
                *cumulative_offset,
                ranges,
                *max_stream_data,
                *fin_offset,
            );
        }
    }
    append_download_frames_for_sessions(&query, &key, &mut response, &sessions).await?;

    if response.frames.is_empty() {
        response.frames.push(Frame::Ping {
            nonce: packet.packet_no,
        });
    }
    ensure_response_packet_fits(&query, &key, &mut response, packet.packet_no)?;
    record_sent_response_state(&response, &sessions).await;

    let envelope = seal_packet(&key, Direction::ServerToClient, &response)?;
    connection
        .cache_response(packet.packet_no, envelope.clone())
        .await;
    build_txt_response(&query, &envelope, 0)
}

fn push_unique_stream(streams: &mut Vec<u64>, stream_id: u64) {
    if !streams.contains(&stream_id) {
        streams.push(stream_id);
    }
}

async fn should_send_upload_ack(
    session: &SessionHandle,
    state: &StreamAckState,
    force_retransmit: bool,
    packet_ack_ranges: &[AckRange],
) -> bool {
    let last = session.last_upload_ack_sent.lock().await;
    let Some(last) = last.as_ref() else {
        return true;
    };
    if &last.state != state {
        return true;
    }
    if !ack_ranges_contain(packet_ack_ranges, last.packet_no) {
        return true;
    }
    force_retransmit && last.sent_at.elapsed() >= SERVER_UPLOAD_ACK_REPEAT
}

async fn record_sent_response_state(response: &Packet, sessions: &[(u64, Arc<SessionHandle>)]) {
    for frame in &response.frames {
        match frame {
            Frame::StreamAck { stream_id, .. } => {
                let Some(session) = sessions
                    .iter()
                    .find_map(|(id, session)| (*id == *stream_id).then_some(session))
                else {
                    continue;
                };
                let Some(state) = stream_ack_state(frame) else {
                    continue;
                };
                *session.last_upload_ack_sent.lock().await = Some(SentUploadAck {
                    state,
                    packet_no: response.packet_no,
                    sent_at: Instant::now(),
                });
            }
            Frame::Close { stream_id, .. } => {
                let Some(session) = sessions
                    .iter()
                    .find_map(|(id, session)| (*id == *stream_id).then_some(session))
                else {
                    continue;
                };
                *session.terminal_close_sent.lock().await = Some(SentTerminalClose {
                    packet_no: response.packet_no,
                });
            }
            Frame::Data { .. }
            | Frame::Open { .. }
            | Frame::Ping { .. }
            | Frame::PathChallenge { .. }
            | Frame::PathResponse { .. } => {}
        }
    }
}

async fn append_download_frames_for_sessions(
    query: &trajectory_core::dns::DnsQuery,
    key: &ClientAccessKey,
    response: &mut Packet,
    sessions: &[(u64, Arc<SessionHandle>)],
) -> Result<()> {
    for (_, session) in sessions {
        stage_download_frames(session).await?;
    }

    loop {
        let mut progressed = false;
        let max_download_frame =
            download_frame_max_for_response(response.max_response_bytes, sessions.len());
        for (stream_id, session) in sessions {
            if response.frames.len() >= 60 {
                return Ok(());
            }
            if append_one_download_frame(
                query,
                key,
                response,
                *stream_id,
                session,
                max_download_frame,
            )
            .await?
            {
                progressed = true;
            }
        }
        if !progressed {
            break;
        }
    }
    Ok(())
}

fn download_frame_max_for_response(max_response_bytes: u16, active_sessions: usize) -> usize {
    if active_sessions <= 1 {
        return SERVER_DOWNLOAD_FRAME_MAX;
    }
    let usable = (max_response_bytes as usize).saturating_sub(DNS_RESPONSE_SAFETY_MARGIN + 160);
    let reserved_slots = active_sessions.min(4) + 1;
    (usable / reserved_slots).clamp(
        SERVER_DOWNLOAD_FAIR_FRAME_MIN,
        SERVER_DOWNLOAD_FAIR_FRAME_MAX,
    )
}

async fn stage_download_frames(session: &SessionHandle) -> Result<()> {
    let mut send = session.download_send.lock().await;
    let mut download_rx = session.download_rx.lock().await;
    while send.retained_len() < SERVER_RETAINED_BYTE_LIMIT {
        match download_rx.try_recv() {
            Ok(download) => send
                .append(download.offset, download.fin, download.bytes)
                .context("retain target download bytes")?,
            Err(mpsc::error::TryRecvError::Empty) => break,
            Err(mpsc::error::TryRecvError::Disconnected) => {
                send.mark_fin_at_end();
                break;
            }
        }
    }
    Ok(())
}

async fn append_one_download_frame(
    query: &trajectory_core::dns::DnsQuery,
    key: &ClientAccessKey,
    response: &mut Packet,
    stream_id: u64,
    session: &SessionHandle,
    max_frame_bytes: usize,
) -> Result<bool> {
    let mut send = session.download_send.lock().await;
    if let Some(slice) =
        next_download_slice_that_fits(query, key, response, stream_id, &send, max_frame_bytes)?
    {
        let frame = Frame::Data {
            stream_id,
            offset: slice.offset,
            fin: slice.fin,
            bytes: slice.bytes.clone(),
        };
        response.frames.push(frame);
        send.mark_sent(&slice);
        return Ok(true);
    }

    if send.is_finished() {
        let frame = Frame::Close { stream_id, code: 0 };
        if response_frame_fits(query, key, response, &frame)? || response.frames.is_empty() {
            response.frames.push(frame);
            return Ok(true);
        }
    }
    Ok(false)
}

fn path_response_that_fits(
    query: &trajectory_core::dns::DnsQuery,
    key: &ClientAccessKey,
    response: &Packet,
    nonce: u64,
    response_bytes: u16,
) -> Result<Option<Frame>> {
    let mut len = response_bytes as usize;
    loop {
        let frame = Frame::PathResponse {
            nonce,
            bytes: vec![0; len],
        };
        if response_frame_fits(query, key, response, &frame)? {
            return Ok(Some(frame));
        }
        if len == 0 {
            return Ok(None);
        }
        len /= 2;
    }
}

fn next_download_slice_that_fits(
    query: &trajectory_core::dns::DnsQuery,
    key: &ClientAccessKey,
    response: &Packet,
    stream_id: u64,
    send: &RetainedByteSendBuffer,
    max_frame_bytes: usize,
) -> Result<Option<SendBufferSlice>> {
    let Some(max_slice) = send.peek_next(max_frame_bytes.max(1)) else {
        return Ok(None);
    };
    if max_slice.bytes.is_empty() {
        let frame = Frame::Data {
            stream_id,
            offset: max_slice.offset,
            fin: max_slice.fin,
            bytes: Vec::new(),
        };
        return Ok(response_frame_fits(query, key, response, &frame)?.then_some(max_slice));
    }
    let mut low = 1usize;
    let mut high = max_slice.bytes.len();
    let mut best = None::<SendBufferSlice>;
    while low <= high {
        let mid = low + (high - low) / 2;
        let Some(slice) = send.peek_next(mid) else {
            break;
        };
        let frame = Frame::Data {
            stream_id,
            offset: slice.offset,
            fin: slice.fin,
            bytes: slice.bytes.clone(),
        };
        if response_frame_fits(query, key, response, &frame)? {
            best = Some(slice);
            low = mid + 1;
        } else {
            high = mid.saturating_sub(1);
        }
    }
    Ok(best)
}

fn response_frame_fits(
    query: &trajectory_core::dns::DnsQuery,
    _key: &ClientAccessKey,
    response: &Packet,
    frame: &Frame,
) -> Result<bool> {
    let budget = dns_response_budget(response.max_response_bytes);
    let envelope_len = sealed_packet_len_with_extra_frame(response, frame)?;
    Ok(txt_response_wire_len(query, envelope_len) <= budget)
}

fn response_packet_fits(
    query: &trajectory_core::dns::DnsQuery,
    _key: &ClientAccessKey,
    response: &Packet,
) -> Result<bool> {
    let budget = dns_response_budget(response.max_response_bytes);
    let envelope_len = sealed_packet_len(response)?;
    Ok(txt_response_wire_len(query, envelope_len) <= budget)
}

fn dns_response_budget(max_response_bytes: u16) -> usize {
    let advertised_budget = max_response_bytes.max(512) as usize;
    let safety_margin = if advertised_budget <= 512 {
        0
    } else {
        DNS_RESPONSE_SAFETY_MARGIN
    };
    advertised_budget.saturating_sub(safety_margin).max(256)
}

fn ensure_response_packet_fits(
    query: &trajectory_core::dns::DnsQuery,
    key: &ClientAccessKey,
    response: &mut Packet,
    fallback_nonce: u64,
) -> Result<()> {
    while !response_packet_fits(query, key, response)? {
        if shrink_packet_ack_ranges(response) {
            continue;
        }
        if shrink_stream_ack_to_zero_ranges(response) {
            continue;
        }
        if remove_stream_ack_frame(response) {
            continue;
        }
        if remove_last_non_data_frame(response) {
            continue;
        }
        break;
    }

    if !response_packet_fits(query, key, response)? {
        response.ack_ranges.clear();
        response.frames.clear();
        response.frames.push(Frame::Ping {
            nonce: fallback_nonce,
        });
    }

    if !response_packet_fits(query, key, response)? {
        bail!("server DNS response cannot fit advertised response budget")
    }
    Ok(())
}

fn remove_last_non_data_frame(packet: &mut Packet) -> bool {
    let Some(index) = packet
        .frames
        .iter()
        .rposition(|frame| !matches!(frame, Frame::Data { .. }))
    else {
        return false;
    };
    packet.frames.remove(index);
    true
}

pub fn parse_socket_addr(value: &str, default_port: u16) -> Result<SocketAddr> {
    if let Ok(addr) = value.parse::<SocketAddr>() {
        return Ok(addr);
    }
    format!("{value}:{default_port}")
        .parse()
        .with_context(|| format!("invalid socket address {value}"))
}

fn client_poll_interval(active: bool, configured: Duration) -> Duration {
    if active {
        CLIENT_ACTIVE_POLL_INTERVAL
    } else {
        configured
    }
}

fn stream_acks_in_request(request: &Packet) -> Vec<(u64, StreamAckState)> {
    request
        .frames
        .iter()
        .filter_map(|frame| match frame {
            Frame::StreamAck { stream_id, .. } => {
                stream_ack_state(frame).map(|state| (*stream_id, state))
            }
            _ => None,
        })
        .collect()
}

fn stream_ack_state(frame: &Frame) -> Option<StreamAckState> {
    match frame {
        Frame::StreamAck {
            cumulative_offset,
            max_stream_data,
            fin_offset,
            ranges,
            ..
        } => Some(StreamAckState {
            cumulative_offset: *cumulative_offset,
            max_stream_data: *max_stream_data,
            fin_offset: *fin_offset,
            ranges: ranges.clone(),
        }),
        _ => None,
    }
}

fn mux_send_class(kind: &MuxSentKind) -> ClientSendClass {
    match kind {
        MuxSentKind::Open { .. } | MuxSentKind::Close { .. } => ClientSendClass::Control,
        MuxSentKind::Data { .. } => ClientSendClass::Data,
        MuxSentKind::Ping => ClientSendClass::Poll,
    }
}

fn fit_mux_client_request_to_dns_budget(
    config: &ClientConfig,
    request: &mut Packet,
    mut kind: MuxSentKind,
) -> Result<MuxSentKind> {
    while !client_request_fits(config, request) {
        if shrink_packet_ack_ranges(request) {
            continue;
        }
        if shrink_stream_ack_to_zero_ranges(request) {
            continue;
        }
        if remove_stream_ack_frame(request) {
            continue;
        }
        if shrink_mux_data_frame(config, request, &mut kind) {
            continue;
        }
        if drop_mux_open_first_data(request, &mut kind) {
            continue;
        }
        if drop_mux_data_frame(request, &mut kind) {
            continue;
        }
        break;
    }

    if !client_request_fits(config, request)
        && matches!(kind, MuxSentKind::Data { .. } | MuxSentKind::Ping)
    {
        request.frames.clear();
        request.ack_ranges.clear();
        request.frames.push(Frame::Ping {
            nonce: request.packet_no,
        });
        kind = MuxSentKind::Ping;
    }

    match kind {
        MuxSentKind::Open {
            stream_id,
            first_data: Some(_),
        } if !client_request_fits(config, request) => {
            remove_last_data_frame(request);
            let kind = MuxSentKind::Open {
                stream_id,
                first_data: None,
            };
            if client_request_fits(config, request) {
                Ok(kind)
            } else {
                bail!("client DNS request cannot fit query name after reducing open packet")
            }
        }
        other if client_request_fits(config, request) => Ok(other),
        _ => bail!("client DNS request cannot fit query name after reductions"),
    }
}

fn drop_mux_open_first_data(request: &mut Packet, kind: &mut MuxSentKind) -> bool {
    let MuxSentKind::Open { first_data, .. } = kind else {
        return false;
    };
    if first_data.is_none() {
        return false;
    }
    *first_data = None;
    remove_last_data_frame(request);
    true
}

fn shrink_mux_data_frame(
    config: &ClientConfig,
    request: &mut Packet,
    kind: &mut MuxSentKind,
) -> bool {
    let current_len = mux_data_len(kind);
    if current_len <= 1 {
        return false;
    }
    let Some(original_bytes) = mux_data_bytes(kind) else {
        return false;
    };

    let mut low = 1usize;
    let mut high = current_len - 1;
    let mut best = None;
    while low <= high {
        let mid = low + (high - low) / 2;
        truncate_mux_data_frame(request, kind, &original_bytes, mid);
        if client_request_fits(config, request) {
            best = Some(mid);
            low = mid + 1;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    if let Some(best) = best {
        truncate_mux_data_frame(request, kind, &original_bytes, best);
    } else {
        truncate_mux_data_frame(request, kind, &original_bytes, 1);
    }
    true
}

fn mux_data_bytes(kind: &MuxSentKind) -> Option<Vec<u8>> {
    match kind {
        MuxSentKind::Open {
            first_data: Some(slice),
            ..
        }
        | MuxSentKind::Data { slice, .. } => Some(slice.bytes.clone()),
        _ => None,
    }
}

fn mux_data_len(kind: &MuxSentKind) -> usize {
    match kind {
        MuxSentKind::Open {
            first_data: Some(slice),
            ..
        }
        | MuxSentKind::Data { slice, .. } => slice.bytes.len(),
        _ => 0,
    }
}

fn truncate_mux_data_frame(
    request: &mut Packet,
    kind: &mut MuxSentKind,
    original_bytes: &[u8],
    len: usize,
) -> bool {
    let len = len.min(original_bytes.len());
    let truncated = original_bytes[..len].to_vec();
    let (bytes, fin) = match kind {
        MuxSentKind::Open {
            first_data: Some(slice),
            ..
        }
        | MuxSentKind::Data { slice, .. } => {
            slice.bytes = truncated;
            slice.fin = false;
            (slice.bytes.clone(), slice.fin)
        }
        _ => return false,
    };

    if let Some(Frame::Data {
        fin: frame_fin,
        bytes: frame_bytes,
        ..
    }) = request
        .frames
        .iter_mut()
        .rev()
        .find(|frame| matches!(frame, Frame::Data { .. }))
    {
        *frame_fin = fin;
        *frame_bytes = bytes;
    }
    true
}

fn drop_mux_data_frame(request: &mut Packet, kind: &mut MuxSentKind) -> bool {
    if !matches!(kind, MuxSentKind::Data { .. }) {
        return false;
    }
    if !remove_last_data_frame(request) {
        return false;
    }
    *kind = MuxSentKind::Ping;
    request.frames.push(Frame::Ping {
        nonce: request.packet_no,
    });
    true
}

fn remove_last_data_frame(request: &mut Packet) -> bool {
    let Some(index) = request.frames.iter().rposition(is_bulk_data_frame) else {
        return false;
    };
    request.frames.remove(index);
    true
}

fn is_bulk_data_frame(frame: &Frame) -> bool {
    matches!(frame, Frame::Data { .. })
}

fn shrink_stream_ack_to_zero_ranges(request: &mut Packet) -> bool {
    request.frames.iter_mut().any(|frame| match frame {
        Frame::StreamAck { ranges, .. } if !ranges.is_empty() => {
            ranges.clear();
            true
        }
        _ => false,
    })
}

fn remove_stream_ack_frame(request: &mut Packet) -> bool {
    let Some(index) = request
        .frames
        .iter()
        .position(|frame| matches!(frame, Frame::StreamAck { .. }))
    else {
        return false;
    };
    request.frames.remove(index);
    true
}

fn shrink_packet_ack_ranges(request: &mut Packet) -> bool {
    request.ack_ranges.pop().is_some()
}

fn client_request_fits(config: &ClientConfig, request: &Packet) -> bool {
    seal_packet(&config.access_key, Direction::ClientToServer, request)
        .and_then(|envelope| envelope_to_qname(&envelope, &config.domain))
        .is_ok()
}

fn is_in_domain(qname: &str, domain: &str) -> bool {
    let qname = qname.trim_end_matches('.').to_ascii_lowercase();
    let domain = domain.trim_end_matches('.').to_ascii_lowercase();
    qname == domain || qname.ends_with(&format!(".{domain}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client_config(tcp_path: bool, dns_max_payload: u16) -> ClientConfig {
        ClientConfig {
            listen: "127.0.0.1:0".parse().unwrap(),
            http_listen: None,
            resolvers: vec!["192.0.2.1:53".parse().unwrap()],
            domain: "t.example.test".to_string(),
            access_key: ClientAccessKey::generate(),
            resolver_socks_proxy: tcp_path.then(|| "127.0.0.1:11092".parse().unwrap()),
            resolver_transport: if tcp_path {
                ResolverTransportMode::Tcp
            } else {
                ResolverTransportMode::Auto
            },
            poll_interval: Duration::from_millis(5),
            dns_max_payload,
            admission_report: None,
            resolver_cohort_size: None,
            resolver_admission_min: 1,
        }
    }

    #[test]
    fn tcp_proxy_path_always_uses_signed_admission() {
        let mut config = test_client_config(true, 1232);
        assert!(should_admit_resolvers(&config));

        config.resolvers = vec!["192.0.2.1:53".parse().unwrap(); RESOLVER_TARGET_ADMITTED_TCP];
        assert!(should_admit_resolvers(&config));

        config.resolvers = vec!["192.0.2.1:53".parse().unwrap(); RESOLVER_TARGET_ADMITTED_TCP + 1];
        assert!(should_admit_resolvers(&config));
    }

    #[test]
    fn tcp_resolver_pool_uses_extra_lanes_on_proxy_paths() {
        let direct = ResolverPool::new(None);
        let proxy = ResolverPool::new(Some("127.0.0.1:11092".parse().unwrap()));

        assert_eq!(direct.lanes_per_resolver, TCP_RESOLVER_LANES_DIRECT);
        assert_eq!(proxy.lanes_per_resolver, TCP_RESOLVER_LANES_PROXY);
        assert!(proxy.lanes_per_resolver > direct.lanes_per_resolver);
    }

    #[test]
    fn proxy_path_has_no_global_fixed_pacing_floor() {
        let mut health = ProxyHealth::default();
        health.record_result(
            true,
            Duration::from_millis(20),
            Duration::from_millis(1),
            Duration::from_secs(30),
        );

        assert!(health.pacing_interval < Duration::from_millis(10));
    }

    #[test]
    fn fair_download_cap_scales_with_response_budget() {
        assert_eq!(
            download_frame_max_for_response(1232, 1),
            SERVER_DOWNLOAD_FRAME_MAX
        );
        assert_eq!(
            download_frame_max_for_response(700, 2),
            SERVER_DOWNLOAD_FAIR_FRAME_MIN.max((700usize - DNS_RESPONSE_SAFETY_MARGIN - 160) / 3)
        );
        assert!(download_frame_max_for_response(4096, 2) <= SERVER_DOWNLOAD_FAIR_FRAME_MAX);
        assert!(download_frame_max_for_response(1232, 6) < SERVER_DOWNLOAD_FAIR_FRAME_MAX);
    }

    #[test]
    fn direct_path_admits_when_requested_or_above_target() {
        let mut config = test_client_config(false, 1232);
        assert!(!should_admit_resolvers(&config));

        config.resolver_cohort_size = Some(1);
        assert!(should_admit_resolvers(&config));

        config.resolver_cohort_size = None;
        config.resolver_admission_min = 2;
        assert!(should_admit_resolvers(&config));

        config.resolver_admission_min = 1;
        config.admission_report = Some(PathBuf::from("admission.jsonl"));
        assert!(should_admit_resolvers(&config));

        config.admission_report = None;
        config.resolvers = vec!["192.0.2.1:53".parse().unwrap(); RESOLVER_TARGET_ADMITTED_UDP];
        assert!(!should_admit_resolvers(&config));

        config.resolvers = vec!["192.0.2.1:53".parse().unwrap(); RESOLVER_TARGET_ADMITTED_UDP + 1];
        assert!(should_admit_resolvers(&config));
    }

    #[test]
    fn admission_thresholds_are_stricter_for_proxy_paths() {
        assert!(admission_challenges(true).len() > admission_challenges(false).len());
        assert!(admission_max_elapsed(true) > admission_max_elapsed(false));
        assert!(admission_min_response_bps(false) > admission_min_response_bps(true));
        assert!(resolver_admission_probe_timeout(true) > resolver_admission_timeout(true));
    }

    #[test]
    fn response_bps_handles_zero_elapsed() {
        assert_eq!(response_bps(512, Duration::ZERO), 512);
        assert_eq!(response_bps(1000, Duration::from_secs(1)), 1000);
    }

    #[test]
    fn admission_deadline_scales_with_candidate_waves() {
        let deadline = resolver_admission_deadline(588, 32, Duration::from_secs(20));
        assert!(deadline > RESOLVER_ADMISSION_DEADLINE);
        assert!(deadline <= Duration::from_secs(10 * 60));
    }

    #[tokio::test]
    async fn poll_can_use_blocked_resolver_when_all_paths_are_degraded() {
        let runtime = ClientRuntime::new(test_client_config(true, 1232));
        {
            let mut health = runtime.resolver_health[0].lock().await;
            health.blocked_until = Some(Instant::now() + Duration::from_secs(60));
        }

        let mut cursor = 0;
        let permit = runtime
            .pick_resolver(&mut cursor, ClientSendClass::Poll)
            .await
            .expect("polls must keep clocking the DNS downlink on the least-bad path");

        assert_eq!(permit.resolver, "192.0.2.1:53".parse().unwrap());
        runtime.release_resolver(permit.resolver_index).await;
    }

    #[tokio::test]
    async fn poll_bypasses_pacing_timer_to_keep_downlink_clocked() {
        let runtime = ClientRuntime::new(test_client_config(true, 1232));
        {
            let mut proxy = runtime.proxy_health.as_ref().unwrap().lock().await;
            proxy.next_send_at = Instant::now() + Duration::from_secs(30);
        }
        {
            let mut health = runtime.resolver_health[0].lock().await;
            health.next_send_at = Instant::now() + Duration::from_secs(30);
        }

        let mut cursor = 0;
        let permit = runtime
            .pick_resolver(&mut cursor, ClientSendClass::Poll)
            .await
            .expect("polling must continue while downlink repair may be pending");

        runtime.release_resolver(permit.resolver_index).await;
    }

    #[tokio::test]
    async fn tcp_proxy_response_budget_honors_configured_dns_payload() {
        let runtime = ClientRuntime::new(test_client_config(true, 4096));
        let health = runtime.resolver_health[0].lock().await;

        assert_eq!(
            runtime.response_bytes_for_health(&health, Instant::now()),
            4096
        );
    }

    #[tokio::test]
    async fn direct_udp_timeout_falls_back_to_tcp_and_prefers_tcp_after_success() {
        let udp_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let resolver = udp_socket.local_addr().unwrap();
        let tcp_listener = TcpListener::bind(resolver).await.unwrap();
        let key = ClientAccessKey::generate();
        let server_key = key.clone();
        let domain = "t.example.test".to_string();
        let server_domain = domain.clone();
        let tcp_server = tokio::spawn(async move {
            let (mut stream, _) = tcp_listener.accept().await.unwrap();
            let query_bytes = read_dns_tcp_message(&mut stream).await.unwrap();
            let query = parse_query(&query_bytes).unwrap();
            let request_envelope = qname_to_envelope(&query.qname, &server_domain).unwrap();
            let request =
                open_packet_with_key(&server_key, Direction::ClientToServer, &request_envelope)
                    .unwrap();
            let mut response = Packet::new(request.conn_id, request.packet_no);
            response.ack_ranges.push(AckRange {
                first: request.packet_no,
                last: request.packet_no,
            });
            response.frames.push(Frame::PathResponse {
                nonce: 0,
                bytes: vec![1, 2, 3, 4],
            });
            let response_envelope =
                seal_packet(&server_key, Direction::ServerToClient, &response).unwrap();
            let response_bytes = build_txt_response(&query, &response_envelope, 0).unwrap();
            write_dns_tcp_message(&mut stream, &response_bytes)
                .await
                .unwrap();
        });

        let mut config = test_client_config(false, 1232);
        config.resolvers = vec![resolver];
        config.domain = domain;
        config.access_key = key;
        let runtime = ClientRuntime::new(config);
        let mut packet = Packet::new(99, 7);
        packet.max_response_bytes = 1232;
        packet.frames.push(Frame::PathChallenge {
            nonce: 0,
            response_bytes: 0,
        });

        let outcome = send_dns_packet(
            &runtime,
            Some(0),
            resolver,
            Some(ClientSendClass::Control),
            &packet,
            Duration::from_millis(50),
        )
        .await
        .unwrap();

        assert_eq!(outcome.transport, DnsTransportOutcome::TcpAfterUdpFailure);
        runtime
            .record_resolver_result(
                0,
                true,
                Duration::from_millis(60),
                outcome.truncated,
                0,
                Some(outcome.transport),
            )
            .await;
        assert!(runtime.prefer_tcp_for_resolver(0).await);
        tcp_server.await.unwrap();
        drop(udp_socket);
    }

    #[tokio::test]
    async fn explicit_direct_tcp_transport_does_not_probe_udp_first() {
        let tcp_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let resolver = tcp_listener.local_addr().unwrap();
        let key = ClientAccessKey::generate();
        let server_key = key.clone();
        let domain = "t.example.test".to_string();
        let server_domain = domain.clone();
        let tcp_server = tokio::spawn(async move {
            let (mut stream, _) = tcp_listener.accept().await.unwrap();
            let query_bytes = read_dns_tcp_message(&mut stream).await.unwrap();
            let query = parse_query(&query_bytes).unwrap();
            let request_envelope = qname_to_envelope(&query.qname, &server_domain).unwrap();
            let request =
                open_packet_with_key(&server_key, Direction::ClientToServer, &request_envelope)
                    .unwrap();
            let response = Packet::new(request.conn_id, request.packet_no);
            let response_envelope =
                seal_packet(&server_key, Direction::ServerToClient, &response).unwrap();
            let response_bytes = build_txt_response(&query, &response_envelope, 0).unwrap();
            write_dns_tcp_message(&mut stream, &response_bytes)
                .await
                .unwrap();
        });

        let mut config = test_client_config(false, 4096);
        config.resolvers = vec![resolver];
        config.domain = domain;
        config.access_key = key;
        config.resolver_transport = ResolverTransportMode::Tcp;
        let runtime = ClientRuntime::new(config);
        let mut packet = Packet::new(11, 1);
        packet.max_response_bytes = 4096;

        let outcome = send_dns_packet(
            &runtime,
            Some(0),
            resolver,
            Some(ClientSendClass::Control),
            &packet,
            Duration::from_millis(250),
        )
        .await
        .unwrap();

        assert_eq!(outcome.transport, DnsTransportOutcome::TcpPreferred);
        tcp_server.await.unwrap();
    }

    #[tokio::test]
    async fn upload_ack_repeats_until_containing_response_packet_is_acked() {
        let (upload_tx, _upload_rx) = mpsc::channel(1);
        let (_download_tx, download_rx) = mpsc::channel(1);
        let session = SessionHandle::new(upload_tx, download_rx);
        let state = StreamAckState {
            cumulative_offset: 128,
            max_stream_data: 4096,
            fin_offset: None,
            ranges: Vec::new(),
        };

        assert!(should_send_upload_ack(&session, &state, false, &[]).await);
        *session.last_upload_ack_sent.lock().await = Some(SentUploadAck {
            state: state.clone(),
            packet_no: 7,
            sent_at: Instant::now(),
        });

        assert!(should_send_upload_ack(&session, &state, false, &[]).await);
        assert!(
            !should_send_upload_ack(&session, &state, false, &[AckRange { first: 7, last: 7 }])
                .await
        );

        let advanced = StreamAckState {
            cumulative_offset: 192,
            ..state
        };
        assert!(
            should_send_upload_ack(
                &session,
                &advanced,
                false,
                &[AckRange { first: 7, last: 7 }]
            )
            .await
        );
    }

    #[test]
    fn sealed_response_size_estimator_matches_dns_builder() {
        let key = ClientAccessKey::generate();
        let query_bytes = build_query(9, "t-aa.t.example.test", 1232).unwrap();
        let query = parse_query(&query_bytes).unwrap();
        let mut packet = Packet::new(11, 3);
        packet.max_response_bytes = 1232;
        packet.ack_ranges = vec![AckRange { first: 1, last: 2 }];
        packet.frames.push(Frame::Data {
            stream_id: 1,
            offset: 0,
            fin: false,
            bytes: vec![b'x'; 300],
        });
        packet.frames.push(Frame::StreamAck {
            stream_id: 1,
            cumulative_offset: 64,
            max_stream_data: 4096,
            fin_offset: None,
            ranges: vec![StreamRange {
                start: 128,
                end: 256,
            }],
        });

        let envelope = seal_packet(&key, Direction::ServerToClient, &packet).unwrap();
        let response = build_txt_response(&query, &envelope, 0).unwrap();

        assert_eq!(
            txt_response_wire_len(&query, sealed_packet_len(&packet).unwrap()),
            response.len()
        );
    }
}
