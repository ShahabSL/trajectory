use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use std::time::{Duration, Instant};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{mpsc, oneshot, Mutex, Semaphore};
use tokio::task::JoinSet;
use tokio::time::timeout;
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::codec::{
    encoded_packet_len_frontier, frontier_short_conn_alias, frontier_short_sealed_alias,
    open_packet_frontier_short, open_packet_with_key, open_packet_with_registry, seal_packet,
    seal_packet_frontier, seal_packet_frontier_short, sealed_packet_len,
    sealed_packet_len_frontier, sealed_packet_len_frontier_short,
    sealed_packet_len_with_extra_frame, AckRange, Direction, Frame, Packet, StreamRange,
};
use trajectory_core::dns::{
    build_a_response, build_aaaa_response, build_empty_response, build_ns_response, build_query,
    build_soa_response, build_txt_response, compact_envelope_qname_len, envelope_qname_len,
    envelope_to_compact_qname, envelope_to_qname, parse_query, parse_txt_response,
    qname_to_envelope, txt_response_wire_len, CLASS_IN, TYPE_A, TYPE_AAAA, TYPE_NS, TYPE_SOA,
    TYPE_TXT,
};
use trajectory_core::engine::{
    ack_ranges_contain, PacketHistory, RetainedByteSendBuffer, SendBufferMode, SendBufferSlice,
    StreamAssembler,
};

const UPLOAD_READ_CHUNK: usize = 4096;
const UPLOAD_SEND_CHUNK_NORMAL: usize = 192;
const UPLOAD_SEND_CHUNK_CONSTRAINED: usize = 192;
const CLIENT_INFLIGHT_WINDOW_BASE: usize = 128;
const CLIENT_INFLIGHT_WINDOW_BULK: usize = 512;
const CLIENT_RESPONSE_CHANNEL: usize = CLIENT_INFLIGHT_WINDOW_BULK * 4;
const CLIENT_BULK_UPLOAD_PENDING_BYTES: usize = 32 * 1024;
const CLIENT_RECEIVE_WINDOW: u64 = 1024 * 1024;
const CLIENT_MAX_ACTIVE_STREAMS: usize = 32;
const SERVER_RECEIVE_WINDOW: u64 = 1024 * 1024;
const CLIENT_STREAM_ACK_RANGES: usize = 8;
const SERVER_STREAM_ACK_RANGES: usize = 8;
const CLIENT_QUERY_TIMEOUT: Duration = Duration::from_secs(20);
const DNS_TCP_WRITE_TIMEOUT: Duration = Duration::from_secs(20);
const SERVER_TCP_READ_TIMEOUT: Duration = Duration::from_secs(20);
const PATH_MIN_CWND: u32 = 2;
const PATH_INITIAL_CWND: u32 = 16;
const PATH_MAX_CWND_UDP: u32 = 256;
const PATH_MAX_CWND_TCP: u32 = 128;
const PROXY_INITIAL_CWND: u32 = 32;
const PROXY_MAX_CWND: u32 = 256;
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
const TCP_PROXY_MAX_INFLIGHT: u32 = 256;
const TCP_RESOLVER_QUEUE: usize = 2048;
const TCP_RESOLVER_LANES_DIRECT: usize = 8;
const TCP_RESOLVER_LANES_PROXY: usize = 16;
const UDP_RESOLVER_QUEUE: usize = 2048;
const UDP_RESOLVER_LANES: usize = 16;
const TCP_RECONNECT_DELAY: Duration = Duration::from_millis(250);
const SERVER_UPLOAD_QUEUE: usize = 1024;
const SERVER_UPLOAD_COALESCE_BYTES: usize = 4096;
const SERVER_UPLOAD_COALESCE_DELAY: Duration = Duration::from_millis(1);
const SERVER_DOWNLOAD_COALESCE_DELAY: Duration = Duration::from_millis(1);
const SERVER_DOWNLOAD_QUEUE: usize = 1024;
const SERVER_TARGET_READ_CHUNK: usize = 4096;
const SERVER_RETAINED_BYTE_LIMIT: usize = SERVER_DOWNLOAD_QUEUE * SERVER_TARGET_READ_CHUNK;
const SERVER_DOWNLOAD_FRAME_MAX: usize = 4096;
const SERVER_DOWNLOAD_ADMISSION_FRAME_MAX: usize = 192;
const SERVER_DOWNLOAD_ADMISSION_FRAME_MIN: usize = 32;
const SERVER_DOWNLOAD_FAIR_FRAME_MAX: usize = 512;
const SERVER_DOWNLOAD_FAIR_FRAME_MIN: usize = 128;
const SERVER_RESPONSE_SESSION_WORK_LIMIT: usize = 96;
const SERVER_UDP_QUERY_CONCURRENCY: usize = 1024;
const SERVER_RESPONSE_CACHE: usize = 512;
const SERVER_UPLOAD_ACK_REPEAT: Duration = Duration::from_millis(250);
const SERVER_STATE_IDLE_TIMEOUT: Duration = Duration::from_secs(90);
const SERVER_STATE_CLEANUP_INTERVAL: Duration = Duration::from_secs(30);
const DNS_RESPONSE_SAFETY_MARGIN: usize = 24;
const RESOLVER_FAILURE_QUARANTINE: Duration = Duration::from_secs(20);
const RESOLVER_FAILURE_QUARANTINE_TCP: Duration = Duration::from_secs(90);
const RESOLVER_PROBE_PARALLELISM_UDP: usize = 1;
const RESOLVER_PROBE_PARALLELISM_TCP: usize = 32;
const RESOLVER_TARGET_ADMITTED_UDP: usize = 64;
const RESOLVER_TARGET_ADMITTED_TCP: usize = 32;
const RESOLVER_ADMISSION_SAMPLE_FACTOR: usize = 3;
const RESOLVER_ADMISSION_TIMEOUT: Duration = Duration::from_millis(1_500);
const RESOLVER_ADMISSION_TIMEOUT_TCP: Duration = Duration::from_secs(20);
const RESOLVER_ADMISSION_DEADLINE: Duration = Duration::from_secs(60);
const RESOLVER_ADMISSION_MAX_ELAPSED_UDP: Duration = Duration::from_secs(10);
const RESOLVER_ADMISSION_MAX_ELAPSED_TCP: Duration = Duration::from_secs(14);
const RESOLVER_ADMISSION_MIN_RESPONSE_BPS_UDP: u64 = 128;
const RESOLVER_ADMISSION_MIN_RESPONSE_BPS_TCP: u64 = 64;
const RESOLVER_ADMISSION_STAGE_ATTEMPTS: usize = 2;
const RESOLVER_TCP_PREFERENCE: Duration = Duration::from_secs(5 * 60);
const RESOLVER_TCP_AFTER_UDP_FAILURE_PREFERENCE: Duration = Duration::from_secs(30);
const CLIENT_PING_INFLIGHT_ACTIVE: usize = 64;
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
const CLIENT_ACTIVE_POLL_DATA_BUDGET: usize = 96;
const CLIENT_TRANSPORT_IDLE_DELAY: Duration = Duration::from_millis(2);
const CLIENT_TRANSPORT_BULK_IDLE_DELAY: Duration = Duration::from_micros(250);
const CLIENT_CONN_ID_MASK: u64 = 0x0000_0007_ffff_ffff;
const FRONTIER_CLIENT_CONN_ID_MASK: u64 = 0x0000_0000_0fff_ffff;
const FRONTIER_SHORT_ALIAS_READY_NONCE: u64 = u64::MAX;
const CLIENT_RESET_CLOSE_DELAY: Duration = Duration::from_millis(500);
const CLIENT_POLL_PROXY_HEADROOM: u32 = 4;
const CLIENT_POLL_RESOLVER_HEADROOM: u32 = 1;
const STDERR_LOG_WINDOW: Duration = Duration::from_secs(1);
const STDERR_LOG_BURST: u32 = 8;

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub listen: SocketAddr,
    pub socks_listen: Option<SocketAddr>,
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
    pub mode: ClientMode,
    pub max_active_streams: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolverTransportMode {
    Auto,
    Udp,
    Tcp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientMode {
    Secure,
    Velocity,
    Resilient,
    Frontier,
}

impl ClientMode {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "secure" => Ok(Self::Secure),
            "velocity" | "fast" => Ok(Self::Velocity),
            "resilient" | "compat" => Ok(Self::Resilient),
            "frontier" => Ok(Self::Frontier),
            _ => bail!(
                "invalid client mode {value:?}; expected secure, velocity, resilient, or frontier"
            ),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Secure => "secure",
            Self::Velocity => "velocity",
            Self::Resilient => "resilient",
            Self::Frontier => "frontier",
        }
    }

    fn profile(self) -> ClientModeProfile {
        match self {
            Self::Secure => ClientModeProfile {
                upload_chunk_normal: UPLOAD_SEND_CHUNK_NORMAL,
                upload_chunk_constrained: UPLOAD_SEND_CHUNK_CONSTRAINED,
                inflight_base: CLIENT_INFLIGHT_WINDOW_BASE,
                inflight_bulk: CLIENT_INFLIGHT_WINDOW_BULK,
                bulk_pending_bytes: CLIENT_BULK_UPLOAD_PENDING_BYTES,
                max_active_streams: CLIENT_MAX_ACTIVE_STREAMS,
                global_active_ping_inflight: CLIENT_GLOBAL_ACTIVE_PING_INFLIGHT,
                global_idle_ping_inflight: CLIENT_GLOBAL_IDLE_PING_INFLIGHT,
                ping_inflight_active: CLIENT_PING_INFLIGHT_ACTIVE,
                ping_inflight_idle: CLIENT_PING_INFLIGHT_IDLE,
                active_poll_data_budget: CLIENT_ACTIVE_POLL_DATA_BUDGET,
                bulk_idle_delay: CLIENT_TRANSPORT_BULK_IDLE_DELAY,
                transport_idle_delay: CLIENT_TRANSPORT_IDLE_DELAY,
                path_initial_cwnd: PATH_INITIAL_CWND,
                path_max_cwnd_udp: PATH_MAX_CWND_UDP,
                path_max_cwnd_tcp: PATH_MAX_CWND_TCP,
                proxy_initial_cwnd: PROXY_INITIAL_CWND,
                proxy_max_cwnd: PROXY_MAX_CWND,
                tcp_lanes_direct: TCP_RESOLVER_LANES_DIRECT,
                tcp_lanes_proxy: TCP_RESOLVER_LANES_PROXY,
                udp_lanes: UDP_RESOLVER_LANES,
                mtu_probe_successes: PATH_MTU_PROBE_SUCCESSES,
            },
            Self::Velocity => ClientModeProfile {
                upload_chunk_normal: 256,
                upload_chunk_constrained: 256,
                inflight_base: 192,
                inflight_bulk: 768,
                bulk_pending_bytes: 16 * 1024,
                max_active_streams: 48,
                global_active_ping_inflight: 128,
                global_idle_ping_inflight: 4,
                ping_inflight_active: 96,
                ping_inflight_idle: 4,
                active_poll_data_budget: 192,
                bulk_idle_delay: Duration::from_micros(125),
                transport_idle_delay: Duration::from_millis(1),
                path_initial_cwnd: 24,
                path_max_cwnd_udp: 384,
                path_max_cwnd_tcp: 192,
                proxy_initial_cwnd: 48,
                proxy_max_cwnd: 384,
                tcp_lanes_direct: 12,
                tcp_lanes_proxy: 24,
                udp_lanes: 24,
                mtu_probe_successes: 8,
            },
            Self::Resilient => ClientModeProfile {
                upload_chunk_normal: 128,
                upload_chunk_constrained: 128,
                inflight_base: 96,
                inflight_bulk: 320,
                bulk_pending_bytes: 64 * 1024,
                max_active_streams: 24,
                global_active_ping_inflight: 64,
                global_idle_ping_inflight: 2,
                ping_inflight_active: 48,
                ping_inflight_idle: 2,
                active_poll_data_budget: 48,
                bulk_idle_delay: Duration::from_micros(500),
                transport_idle_delay: Duration::from_millis(3),
                path_initial_cwnd: 8,
                path_max_cwnd_udp: 128,
                path_max_cwnd_tcp: 96,
                proxy_initial_cwnd: 16,
                proxy_max_cwnd: 128,
                tcp_lanes_direct: 4,
                tcp_lanes_proxy: 8,
                udp_lanes: 8,
                mtu_probe_successes: 24,
            },
            Self::Frontier => ClientModeProfile {
                upload_chunk_normal: 256,
                upload_chunk_constrained: 256,
                inflight_base: 192,
                inflight_bulk: 768,
                bulk_pending_bytes: 16 * 1024,
                max_active_streams: 48,
                global_active_ping_inflight: 128,
                global_idle_ping_inflight: 4,
                ping_inflight_active: 96,
                ping_inflight_idle: 4,
                active_poll_data_budget: 192,
                bulk_idle_delay: Duration::from_micros(125),
                transport_idle_delay: Duration::from_millis(1),
                path_initial_cwnd: 24,
                path_max_cwnd_udp: 384,
                path_max_cwnd_tcp: 192,
                proxy_initial_cwnd: 48,
                proxy_max_cwnd: 384,
                tcp_lanes_direct: 12,
                tcp_lanes_proxy: 24,
                udp_lanes: 24,
                mtu_probe_successes: 8,
            },
        }
    }
}

#[derive(Clone, Copy)]
struct ClientModeProfile {
    upload_chunk_normal: usize,
    upload_chunk_constrained: usize,
    inflight_base: usize,
    inflight_bulk: usize,
    bulk_pending_bytes: usize,
    max_active_streams: usize,
    global_active_ping_inflight: usize,
    global_idle_ping_inflight: usize,
    ping_inflight_active: usize,
    ping_inflight_idle: usize,
    active_poll_data_budget: usize,
    bulk_idle_delay: Duration,
    transport_idle_delay: Duration,
    path_initial_cwnd: u32,
    path_max_cwnd_udp: u32,
    path_max_cwnd_tcp: u32,
    proxy_initial_cwnd: u32,
    proxy_max_cwnd: u32,
    tcp_lanes_direct: usize,
    tcp_lanes_proxy: usize,
    udp_lanes: usize,
    mtu_probe_successes: u32,
}

impl ClientConfig {
    fn tcp_first_resolver_path(&self) -> bool {
        self.resolver_socks_proxy.is_some() || self.resolver_transport == ResolverTransportMode::Tcp
    }

    fn allow_udp_to_tcp_fallback(&self) -> bool {
        self.resolver_socks_proxy.is_none()
            && self.resolver_transport == ResolverTransportMode::Auto
    }

    fn mode_profile(&self) -> ClientModeProfile {
        self.mode.profile()
    }
}

#[derive(Clone)]
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub domain: String,
    pub target: SocketAddr,
    pub target_mode: ServerTargetMode,
    pub udp_gateway_listen: Option<SocketAddr>,
    pub authorized_clients: Arc<HashMap<u32, ClientAccessKey>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerTargetMode {
    Tcp,
    Socks5Direct,
}

#[derive(Clone, Copy)]
struct StderrLogState {
    window_start: Instant,
    emitted: u32,
    suppressed: u64,
}

impl Default for StderrLogState {
    fn default() -> Self {
        Self {
            window_start: Instant::now(),
            emitted: 0,
            suppressed: 0,
        }
    }
}

fn rate_limited_eprintln(key: impl Into<String>, message: impl Into<String>) {
    static LIMITER: OnceLock<StdMutex<HashMap<String, StderrLogState>>> = OnceLock::new();

    let key = key.into();
    let message = message.into();
    let now = Instant::now();
    let mut lines = Vec::new();
    let limiter = LIMITER.get_or_init(|| StdMutex::new(HashMap::new()));
    if let Ok(mut states) = limiter.lock() {
        let state = states.entry(key.clone()).or_default();
        if now.saturating_duration_since(state.window_start) >= STDERR_LOG_WINDOW {
            if state.suppressed > 0 {
                lines.push(format!(
                    "{key}: suppressed {} repeated log line(s)",
                    state.suppressed
                ));
            }
            *state = StderrLogState {
                window_start: now,
                emitted: 0,
                suppressed: 0,
            };
        }
        if state.emitted < STDERR_LOG_BURST {
            state.emitted += 1;
            lines.push(message);
        } else {
            state.suppressed = state.suppressed.saturating_add(1);
        }
    } else {
        lines.push(message);
    }

    for line in lines {
        eprintln!("{line}");
    }
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
        let profile = config.mode_profile();
        let tcp_pool = config
            .resolver_socks_proxy
            .map(|proxy| Arc::new(ResolverPool::new_for_mode(Some(proxy), profile)));
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
            .map(|_| {
                Mutex::new(ResolverHealth::new(
                    initial_timeout,
                    initial_response_bytes,
                    profile.path_initial_cwnd,
                ))
            })
            .collect();
        let proxy_health = config
            .resolver_socks_proxy
            .map(|_| Mutex::new(ProxyHealth::new(profile.proxy_initial_cwnd)));
        let stream_capacity = config
            .max_active_streams
            .unwrap_or(profile.max_active_streams)
            .clamp(1, 1024);
        let active_ping_capacity = profile.global_active_ping_inflight;
        let idle_ping_capacity = profile.global_idle_ping_inflight;
        Self {
            config,
            tcp_pool,
            tcp_fallback_pool: Arc::new(ResolverPool::new_for_mode(None, profile)),
            udp_pool: Arc::new(UdpResolverPool::new(profile.udp_lanes)),
            resolver_health,
            proxy_health,
            stream_slots: Arc::new(Semaphore::new(stream_capacity)),
            active_ping_slots: Arc::new(Semaphore::new(active_ping_capacity)),
            idle_ping_slots: Arc::new(Semaphore::new(idle_ping_capacity)),
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
            let limit = proxy
                .cwnd
                .min(TCP_PROXY_MAX_INFLIGHT.max(self.config.mode_profile().proxy_max_cwnd));
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
            let degraded_for_bulk = class == ClientSendClass::Data
                && count > 1
                && (health.failures > 0
                    || health.loss_ewma_ppm >= PATH_BULK_LOSSY_PPM
                    || bulk_rtt_cutoff
                        .map(|cutoff| rtt_micros > cutoff)
                        .unwrap_or(false));
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
            let upload_serialization_penalty =
                if class == ClientSendClass::Data && health.upload_goodput_ewma > 0 {
                    (health.in_flight as u128)
                        .saturating_mul(self.upload_chunk_bytes() as u128)
                        .saturating_mul(1_000_000)
                        / health.upload_goodput_ewma as u128
                } else {
                    0
                };
            let failure_penalty = (health.failures as u128).saturating_mul(rtt_micros);
            let loss_penalty = rtt_micros.saturating_mul(health.loss_ewma_ppm as u128) / 20_000;
            let mtu_bonus = (response_bytes as u128).saturating_mul(10);
            let upload_goodput_bonus = if class == ClientSendClass::Data {
                (health.upload_goodput_ewma as u128).min(1_000_000) / 8
            } else {
                0
            };
            let score = rtt_micros
                .saturating_add(queue_penalty)
                .saturating_add(if class == ClientSendClass::Data {
                    upload_serialization_penalty
                } else {
                    serialization_penalty
                })
                .saturating_add(failure_penalty)
                .saturating_add(loss_penalty)
                .saturating_add(
                    blocked_for
                        .map(|duration| duration.as_micros().saturating_mul(4))
                        .unwrap_or(0),
                )
                .saturating_sub(mtu_bonus)
                .saturating_sub(upload_goodput_bonus);
            let candidate = (index, score, offset);
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
            let limit = proxy
                .cwnd
                .min(TCP_PROXY_MAX_INFLIGHT.max(self.config.mode_profile().proxy_max_cwnd));
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

    async fn record_resolver_result(&self, index: usize, sample: ResolverResultSample) {
        if let Some(proxy) = &self.proxy_health {
            let profile = self.config.mode_profile();
            proxy.lock().await.record_result(
                sample.ok,
                sample.elapsed,
                self.rto_min(),
                self.rto_max(),
                profile.proxy_max_cwnd,
            );
        }
        let mut health = self.resolver_health[index].lock().await;
        health.in_flight = health.in_flight.saturating_sub(1);
        if sample.ok {
            health.failures = 0;
            health.blocked_until = None;
            match sample.transport {
                Some(DnsTransportOutcome::TcpFallbackAfterTruncation) => {
                    health.prefer_tcp_until = Some(Instant::now() + RESOLVER_TCP_PREFERENCE);
                }
                Some(DnsTransportOutcome::TcpPreferred) => {
                    health.prefer_tcp_until = Some(Instant::now() + RESOLVER_TCP_PREFERENCE);
                }
                Some(DnsTransportOutcome::TcpAfterUdpFailure) => {
                    let preference = if self.config.resolver_transport == ResolverTransportMode::Tcp
                    {
                        RESOLVER_TCP_PREFERENCE
                    } else {
                        RESOLVER_TCP_AFTER_UDP_FAILURE_PREFERENCE
                    };
                    let until = Instant::now() + preference;
                    health.prefer_tcp_until = Some(until);
                    if matches!(sample.class, Some(ClientSendClass::Data)) {
                        health.prefer_tcp_data_until = Some(until);
                    }
                }
                Some(DnsTransportOutcome::Udp | DnsTransportOutcome::UdpAfterPreferredTcpError) => {
                    health.prefer_tcp_until = None;
                    if matches!(
                        sample.class,
                        Some(ClientSendClass::Data)
                            | Some(ClientSendClass::Control)
                            | Some(ClientSendClass::Poll)
                    ) {
                        health.prefer_tcp_data_until = None;
                    }
                }
                Some(DnsTransportOutcome::TcpProxy) | None => {}
            }
            health.record_rtt(sample.elapsed, self.rto_min(), self.rto_max());
            health.record_loss_sample(false);
            if sample.truncated {
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
                if health.clean_mtu_successes >= self.config.mode_profile().mtu_probe_successes {
                    health.clean_mtu_successes = 0;
                    health.max_response_bytes = health
                        .max_response_bytes
                        .saturating_add(PATH_MTU_STEP)
                        .min(self.config.dns_max_payload.max(PATH_MIN_RESPONSE_BYTES));
                }
            }
            if sample.useful_bytes > 0 {
                health.goodput_ewma = update_goodput_ewma(
                    health.goodput_ewma,
                    sample.useful_bytes as u64,
                    sample.elapsed,
                );
            }
            if sample.request_upload_bytes > 0 {
                health.upload_goodput_ewma = update_goodput_ewma(
                    health.upload_goodput_ewma,
                    sample.request_upload_bytes as u64,
                    sample.elapsed,
                );
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

    async fn prefer_tcp_for_resolver(&self, index: usize, class: Option<ClientSendClass>) -> bool {
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
        let now = Instant::now();
        let data_preferred = health
            .prefer_tcp_data_until
            .map(|until| until > now)
            .unwrap_or(false);
        if matches!(class, Some(ClientSendClass::Data)) {
            return data_preferred;
        }
        data_preferred
            || health
                .prefer_tcp_until
                .map(|until| until > now)
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
        let profile = self.config.mode_profile();
        if self.config.tcp_first_resolver_path() {
            profile.path_max_cwnd_tcp
        } else {
            profile.path_max_cwnd_udp
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

    fn upload_chunk_bytes(&self) -> usize {
        let profile = self.config.mode_profile();
        if self.config.tcp_first_resolver_path() {
            profile.upload_chunk_constrained
        } else {
            profile.upload_chunk_normal
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
    upload_goodput_ewma: u64,
    pacing_interval: Duration,
    next_send_at: Instant,
    srtt: Option<Duration>,
    rttvar: Duration,
    timeout: Duration,
    blocked_until: Option<Instant>,
    prefer_tcp_until: Option<Instant>,
    prefer_tcp_data_until: Option<Instant>,
}

impl ResolverHealth {
    fn new(initial_timeout: Duration, initial_response_bytes: u16, initial_cwnd: u32) -> Self {
        Self {
            failures: 0,
            loss_ewma_ppm: 0,
            in_flight: 0,
            cwnd: initial_cwnd.max(PATH_MIN_CWND),
            cwnd_successes: 0,
            max_response_bytes: initial_response_bytes,
            clean_mtu_successes: 0,
            goodput_ewma: 0,
            upload_goodput_ewma: 0,
            pacing_interval: Duration::ZERO,
            next_send_at: Instant::now(),
            srtt: None,
            rttvar: initial_timeout / 2,
            timeout: initial_timeout,
            blocked_until: None,
            prefer_tcp_until: None,
            prefer_tcp_data_until: None,
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
            Duration::from_millis(1)
        } else {
            Duration::from_micros(500)
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
        Self::new(PROXY_INITIAL_CWND)
    }
}

impl ProxyHealth {
    fn new(initial_cwnd: u32) -> Self {
        Self {
            failures: 0,
            in_flight: 0,
            cwnd: initial_cwnd.max(PATH_MIN_CWND),
            cwnd_successes: 0,
            pacing_interval: Duration::ZERO,
            next_send_at: Instant::now(),
            srtt: None,
            rttvar: Duration::from_secs(4),
            timeout: Duration::from_secs(8),
        }
    }

    fn record_result(
        &mut self,
        ok: bool,
        elapsed: Duration,
        min_timeout: Duration,
        max_timeout: Duration,
        max_cwnd: u32,
    ) {
        self.in_flight = self.in_flight.saturating_sub(1);
        if ok {
            self.failures = 0;
            self.record_rtt(elapsed, min_timeout, max_timeout);
            self.cwnd_successes = self.cwnd_successes.saturating_add(1);
            if self.cwnd_successes >= self.cwnd.max(1) {
                self.cwnd_successes = 0;
                self.cwnd = self.cwnd.saturating_add(1).min(max_cwnd.max(PATH_MIN_CWND));
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
        Self::new(
            Duration::from_millis(1_500),
            PATH_INITIAL_RESPONSE_BYTES,
            PATH_INITIAL_CWND,
        )
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
    senders: Mutex<HashMap<SocketAddr, ResolverLanes<DnsTcpRequest>>>,
}

struct UdpResolverPool {
    lanes_per_resolver: usize,
    senders: Mutex<HashMap<SocketAddr, ResolverLanes<DnsUdpRequest>>>,
}

struct ResolverLanes<T> {
    next: usize,
    senders: Vec<mpsc::Sender<T>>,
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

struct ResolverResultSample {
    ok: bool,
    elapsed: Duration,
    truncated: bool,
    useful_bytes: usize,
    request_upload_bytes: usize,
    class: Option<ClientSendClass>,
    transport: Option<DnsTransportOutcome>,
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
        target: Option<OpenTarget>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct OpenTarget {
    host: String,
    port: u16,
}

impl OpenTarget {
    fn new(host: String, port: u16) -> Result<Self> {
        if host.is_empty() {
            bail!("empty proxy target host");
        }
        if host.len() > 255 {
            bail!("proxy target host is too long");
        }
        if port == 0 {
            bail!("proxy target port must be non-zero");
        }
        Ok(Self { host, port })
    }
}

enum ClientStreamOutput {
    Bytes(Vec<u8>),
    Close,
}

struct ClientMuxStream {
    target: Option<OpenTarget>,
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
    fn new(target: Option<OpenTarget>, output: mpsc::Sender<ClientStreamOutput>) -> Self {
        Self {
            target,
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
        target: Option<OpenTarget>,
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
    frontier_short_header_ready: bool,
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
    #[cfg(test)]
    fn new(proxy: Option<SocketAddr>) -> Self {
        let lanes_per_resolver = if proxy.is_some() {
            TCP_RESOLVER_LANES_PROXY
        } else {
            TCP_RESOLVER_LANES_DIRECT
        };
        Self::new_with_lanes(proxy, lanes_per_resolver)
    }

    fn new_for_mode(proxy: Option<SocketAddr>, profile: ClientModeProfile) -> Self {
        let lanes_per_resolver = if proxy.is_some() {
            profile.tcp_lanes_proxy
        } else {
            profile.tcp_lanes_direct
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
    fn new(lanes_per_resolver: usize) -> Self {
        Self {
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
        let lanes = senders.entry(resolver).or_insert_with(|| ResolverLanes {
            next: 0,
            senders: Vec::with_capacity(UDP_RESOLVER_LANES),
        });
        lanes.senders.retain(|sender| !sender.is_closed());
        while lanes.senders.len() < self.lanes_per_resolver {
            let (tx, rx) = mpsc::channel(UDP_RESOLVER_QUEUE);
            tokio::spawn(run_udp_resolver_actor(resolver, rx));
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
    let socks_listener = match config.socks_listen {
        Some(addr) => Some(
            TcpListener::bind(addr)
                .await
                .with_context(|| format!("bind local SOCKS proxy listener {addr}"))?,
        ),
        None => None,
    };
    let http_listener = match config.http_listen {
        Some(addr) => Some(
            TcpListener::bind(addr)
                .await
                .with_context(|| format!("bind local HTTP proxy listener {addr}"))?,
        ),
        None => None,
    };
    eprintln!(
        "trajectory client listening on {} via {} resolver(s), mode={}",
        listener.local_addr()?,
        config.resolvers.len(),
        config.mode.as_str()
    );
    if let Some(listener) = &http_listener {
        eprintln!(
            "trajectory HTTP proxy listening on {}",
            listener.local_addr()?
        );
    }
    if let Some(listener) = &socks_listener {
        eprintln!(
            "trajectory SOCKS proxy listening on {}",
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
    if let Some(listener) = socks_listener {
        let transport_tx = transport_tx.clone();
        let runtime = Arc::clone(&runtime);
        let next_stream_id = Arc::clone(&next_stream_id);
        tokio::spawn(async move {
            if let Err(error) =
                accept_socks_proxy_streams(listener, runtime, transport_tx, next_stream_id).await
            {
                eprintln!("SOCKS proxy listener failed: {error:#}");
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

fn fresh_client_conn_id(mode: ClientMode) -> u64 {
    (rand::random::<u64>() & client_conn_id_mask(mode)).max(1)
}

fn client_conn_id_mask(mode: ClientMode) -> u64 {
    match mode {
        ClientMode::Frontier => FRONTIER_CLIENT_CONN_ID_MASK,
        ClientMode::Secure | ClientMode::Velocity | ClientMode::Resilient => CLIENT_CONN_ID_MASK,
    }
}

fn frontier_short_alias_signal(conn_id: u64) -> Vec<u8> {
    frontier_short_conn_alias(conn_id).to_be_bytes().to_vec()
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
    let mut admitted = Vec::<AdmissionProbeResult>::new();
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
                        admitted.push(result.clone());
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
                        sort_admitted_resolvers(&mut admitted);
                        let resolvers = admitted
                            .into_iter()
                            .take(target)
                            .map(|result| result.resolver)
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
    sort_admitted_resolvers(&mut admitted);
    let resolvers = admitted
        .into_iter()
        .take(target)
        .map(|result| result.resolver)
        .collect::<Vec<_>>();
    write_admission_report_if_requested(&config, &results, &resolvers)?;
    Ok(resolvers)
}

fn sort_admitted_resolvers(admitted: &mut [AdmissionProbeResult]) {
    admitted.sort_by(|left, right| {
        left.elapsed
            .cmp(&right.elapsed)
            .then_with(|| right.response_bps.cmp(&left.response_bps))
            .then_with(|| left.resolver.cmp(&right.resolver))
    });
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
        || config.resolvers.len() > 1
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
        let mut last_error = None::<String>;
        for attempt in 1..=RESOLVER_ADMISSION_STAGE_ATTEMPTS {
            let remaining = max_elapsed.saturating_sub(started.elapsed());
            match timeout(remaining, probe.send_path_challenge(*challenge)).await {
                Ok(Ok(result)) => {
                    challenge_count += 1;
                    total_response_bytes += result.response_payload_bytes;
                    last_rtt = result.rtt;
                    last_error = None;
                    break;
                }
                Ok(Err(error))
                    if attempt < RESOLVER_ADMISSION_STAGE_ATTEMPTS
                        && started.elapsed() < max_elapsed =>
                {
                    last_error = Some(error);
                    continue;
                }
                Ok(Err(error)) => {
                    last_error = Some(error);
                    break;
                }
                Err(_) => {
                    last_error = Some(format!(
                        "admission stage timed out after {}ms",
                        remaining.as_millis()
                    ));
                    break;
                }
            }
        }
        if let Some(error) = last_error {
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

#[derive(Clone, Debug)]
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

const UDP_ADMISSION_CHALLENGES: [AdmissionChallenge; 8] = [
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
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 128,
        request_padding: 96,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 128,
        request_padding: 112,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 128,
        request_padding: 112,
    },
    AdmissionChallenge {
        stage: "sustained_probe",
        response_bytes: 128,
        request_padding: 112,
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
            "mode": config.mode.as_str(),
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
        let mode = runtime.config.mode;
        Self {
            runtime,
            resolver,
            conn_id: fresh_client_conn_id(mode),
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
        fit_admission_probe_packet(&self.runtime.config, &mut packet)?;
        let started = Instant::now();
        let response = send_dns_packet_inner(
            &self.runtime,
            None,
            self.resolver,
            None,
            &packet,
            DnsSendOptions {
                query_timeout: resolver_admission_timeout(
                    self.runtime.config.tcp_first_resolver_path(),
                ),
                direct_tcp_first: self.prefer_direct_tcp
                    || self.runtime.config.resolver_transport == ResolverTransportMode::Tcp,
                use_frontier_short_header: false,
            },
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

fn fit_admission_probe_packet(config: &ClientConfig, packet: &mut Packet) -> Result<(), String> {
    if client_request_fits(config, packet, false) {
        return Ok(());
    }

    let Some(frame_index) = packet
        .frames
        .iter()
        .position(|frame| matches!(frame, Frame::PathResponse { .. }))
    else {
        return Err("admission signed challenge cannot fit DNS query name".to_string());
    };

    let original_padding = match &packet.frames[frame_index] {
        Frame::PathResponse { bytes, .. } => bytes.len(),
        _ => 0,
    };
    let mut low = 0usize;
    let mut high = original_padding;
    while low < high {
        let mid = (low + high).div_ceil(2);
        resize_path_response_padding(packet, frame_index, mid);
        if client_request_fits(config, packet, false) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    resize_path_response_padding(packet, frame_index, low);
    if client_request_fits(config, packet, false) {
        return Ok(());
    }

    packet.frames.remove(frame_index);
    if client_request_fits(config, packet, false) {
        return Ok(());
    }

    Err("admission signed challenge cannot fit DNS query name after trimming padding".to_string())
}

fn resize_path_response_padding(packet: &mut Packet, frame_index: usize, len: usize) {
    if let Some(Frame::PathResponse { bytes, .. }) = packet.frames.get_mut(frame_index) {
        bytes.resize(len, 0);
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
            target: None,
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
                log_proxy_stream_error("HTTP", stream_id, peer, &error);
            }
        });
    }
}

async fn accept_socks_proxy_streams(
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
            if let Err(error) = run_socks_proxy_stream_io(transport_tx, stream_id, stream).await {
                log_proxy_stream_error("SOCKS", stream_id, peer, &error);
            }
        });
    }
}

fn log_proxy_stream_error(kind: &str, stream_id: u64, peer: SocketAddr, error: &anyhow::Error) {
    if is_benign_proxy_stream_error(error) {
        return;
    }
    rate_limited_eprintln(
        format!("{kind}:proxy-stream-failed"),
        format!("{kind} proxy stream {stream_id} from {peer} failed: {error:#}"),
    );
}

fn is_benign_proxy_stream_error(error: &anyhow::Error) -> bool {
    let text = format!("{error:#}").to_lowercase();
    text.contains("broken pipe")
        || text.contains("connection reset by peer")
        || text.contains("early eof")
        || text.contains("client closed before sending headers")
}

async fn run_socks_proxy_stream_io(
    transport_tx: mpsc::Sender<ClientTransportEvent>,
    stream_id: u64,
    mut local: TcpStream,
) -> Result<()> {
    local.set_nodelay(true).ok();
    let request = match read_local_socks5_request(&mut local).await {
        Ok(request) => request,
        Err(error) => {
            let _ = send_local_socks5_reply(&mut local, 0x01).await;
            return Err(error);
        }
    };
    let output_rx = register_client_stream(
        &transport_tx,
        stream_id,
        Some(OpenTarget::new(request.host, request.port)?),
    )
    .await?;
    send_local_socks5_reply(&mut local, 0x00).await?;

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

    let writer_result = write_local_stream_output(&mut writer, output_rx).await;
    reader_task.abort();
    let _ = transport_tx
        .send(ClientTransportEvent::LocalClosed { stream_id })
        .await;
    writer_result
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
    let output_rx = register_client_stream(
        &transport_tx,
        stream_id,
        Some(OpenTarget::new(request.host.clone(), request.port)?),
    )
    .await?;
    let remote = TransportOutputReader::new(output_rx);

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
    target: Option<OpenTarget>,
) -> Result<mpsc::Receiver<ClientStreamOutput>> {
    let (output_tx, output_rx) = mpsc::channel(CLIENT_STREAM_OUTPUT_QUEUE);
    transport_tx
        .send(ClientTransportEvent::OpenStream {
            stream_id,
            target,
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

struct SocksProxyRequest {
    host: String,
    port: u16,
}

async fn read_local_socks5_request(stream: &mut TcpStream) -> Result<SocksProxyRequest> {
    timeout(HTTP_PROXY_HEADER_TIMEOUT, async {
        let mut greeting = [0u8; 2];
        stream
            .read_exact(&mut greeting)
            .await
            .context("read SOCKS greeting")?;
        if greeting[0] != 0x05 {
            bail!("SOCKS client used unsupported version");
        }
        let mut methods = vec![0u8; greeting[1] as usize];
        stream
            .read_exact(&mut methods)
            .await
            .context("read SOCKS methods")?;
        if !methods.contains(&0x00) {
            stream.write_all(&[0x05, 0xff]).await.ok();
            bail!("SOCKS client did not offer no-auth method");
        }
        stream
            .write_all(&[0x05, 0x00])
            .await
            .context("write SOCKS method selection")?;

        let mut head = [0u8; 4];
        stream
            .read_exact(&mut head)
            .await
            .context("read SOCKS connect header")?;
        if head[0] != 0x05 || head[1] != 0x01 || head[2] != 0x00 {
            bail!("SOCKS proxy mode supports CONNECT only");
        }
        let host = match head[3] {
            0x01 => {
                let mut octets = [0u8; 4];
                stream
                    .read_exact(&mut octets)
                    .await
                    .context("read SOCKS IPv4 target")?;
                std::net::Ipv4Addr::from(octets).to_string()
            }
            0x03 => {
                let mut len = [0u8; 1];
                stream
                    .read_exact(&mut len)
                    .await
                    .context("read SOCKS domain length")?;
                let mut name = vec![0u8; len[0] as usize];
                stream
                    .read_exact(&mut name)
                    .await
                    .context("read SOCKS domain target")?;
                String::from_utf8(name).context("SOCKS domain is not valid UTF-8")?
            }
            0x04 => {
                let mut octets = [0u8; 16];
                stream
                    .read_exact(&mut octets)
                    .await
                    .context("read SOCKS IPv6 target")?;
                std::net::Ipv6Addr::from(octets).to_string()
            }
            other => bail!("SOCKS target used unsupported address type {other}"),
        };
        let mut port = [0u8; 2];
        stream
            .read_exact(&mut port)
            .await
            .context("read SOCKS target port")?;
        let port = u16::from_be_bytes(port);
        OpenTarget::new(host.clone(), port)?;
        Ok(SocksProxyRequest { host, port })
    })
    .await
    .context("SOCKS request timed out")?
}

async fn send_local_socks5_reply(stream: &mut TcpStream, code: u8) -> Result<()> {
    stream
        .write_all(&[0x05, code, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await
        .context("write SOCKS reply")
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
    let (response_tx, response_rx) = mpsc::channel::<ClientDnsResult>(CLIENT_RESPONSE_CHANNEL);
    let now = Instant::now();
    let conn_id = fresh_client_conn_id(runtime.config.mode);
    let mut transport = ClientTransport {
        runtime,
        conn_id,
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
        frontier_short_header_ready: false,
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
            let idle_delay = self.idle_delay();

            tokio::select! {
                event = event_rx.recv() => {
                    let Some(event) = event else {
                        if self.streams.is_empty() && self.outstanding.is_empty() {
                            return Ok(());
                        }
                        tokio::time::sleep(idle_delay).await;
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
                _ = tokio::time::sleep(idle_delay) => {}
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
            ClientTransportEvent::OpenStream {
                stream_id,
                target,
                output,
            } => {
                self.streams
                    .insert(stream_id, ClientMuxStream::new(target, output));
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
        let upload_pending_bytes = self.upload_pending_bytes();
        let outbound_window = self.outbound_window();
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
            let prefer_tcp_data_ms = health
                .prefer_tcp_data_until
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
                "upload_goodput_ewma_Bps": health.upload_goodput_ewma,
                "pacing_ms": health.pacing_interval.as_millis() as u64,
                "srtt_ms": health.srtt.map(|duration| duration.as_millis() as u64),
                "rttvar_ms": health.rttvar.as_millis() as u64,
                "timeout_ms": health.timeout.as_millis() as u64,
                "blocked_ms": blocked_ms,
                "prefer_tcp_ms": prefer_tcp_ms,
                "prefer_tcp_data_ms": prefer_tcp_data_ms,
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
            "mode": self.runtime.config.mode.as_str(),
            "conn_id": self.conn_id,
            "elapsed_ms": self.diag_started.elapsed().as_millis() as u64,
            "streams": pending_streams,
            "outstanding": self.outstanding.len(),
            "outbound_window": outbound_window,
            "frontier_short_header_ready": self.frontier_short_header_ready,
            "upload_bulk_mode": self.bulk_upload_mode(),
            "upload_pending_bytes": upload_pending_bytes,
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
        while self.outstanding.len() < self.outbound_window() {
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
                ClientSendClass::Data if !self.has_downlink_pressure() => 0,
                ClientSendClass::Data => 2,
                ClientSendClass::Control | ClientSendClass::Poll => 4,
            });

            let mut sent = MuxSentPacket {
                kind,
                stream_acks: Vec::new(),
            };
            if class != ClientSendClass::Data || self.has_downlink_pressure() {
                self.append_due_stream_acks(&mut request, &mut sent);
            }
            self.append_kind_frames(&mut request, &sent.kind);

            sent.kind = fit_mux_client_request_to_dns_budget(
                &self.runtime.config,
                &mut request,
                sent.kind,
                self.frontier_short_header_ready,
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
                    ..
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
            let use_frontier_short_header = self.frontier_short_header_ready;
            tokio::spawn(async move {
                let _ping_slot = ping_slot;
                let started = Instant::now();
                let request_upload_bytes = packet_useful_data_bytes(&request);
                let result = send_dns_packet(
                    &runtime_for_query,
                    Some(path.resolver_index),
                    path.resolver,
                    Some(class),
                    &request,
                    path.timeout,
                    use_frontier_short_header,
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
                            ResolverResultSample {
                                ok: result.is_ok(),
                                elapsed,
                                truncated,
                                useful_bytes,
                                request_upload_bytes,
                                class: Some(class),
                                transport,
                            },
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
                target,
                first_data,
            } => {
                let (host, port) = target
                    .as_ref()
                    .map(|target| (target.host.clone(), target.port))
                    .unwrap_or_else(|| (String::new(), 0));
                request.frames.push(Frame::Open {
                    stream_id: *stream_id,
                    host,
                    port,
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
            let (target, first_data) = self
                .streams
                .get(&stream_id)
                .map(|stream| {
                    (
                        stream.target.clone(),
                        stream.upload_send.peek_next(upload_chunk),
                    )
                })
                .unwrap_or((None, None));
            return Some(MuxSentKind::Open {
                stream_id,
                target,
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
        if self.bulk_upload_mode() {
            return false;
        }
        if self.data_since_poll < self.runtime.config.mode_profile().active_poll_data_budget {
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

    fn outbound_window(&self) -> usize {
        let profile = self.runtime.config.mode_profile();
        if self.bulk_upload_mode() {
            profile.inflight_bulk
        } else {
            profile.inflight_base
        }
    }

    fn idle_delay(&self) -> Duration {
        let profile = self.runtime.config.mode_profile();
        if self.bulk_upload_mode() {
            profile.bulk_idle_delay
        } else {
            profile.transport_idle_delay
        }
    }

    fn bulk_upload_mode(&self) -> bool {
        self.upload_pending_bytes() >= self.runtime.config.mode_profile().bulk_pending_bytes
            && self.streams.values().any(|stream| {
                !stream.closed
                    && (stream.upload_send.has_pending_send()
                        || stream.upload_send.has_retained_bytes())
            })
    }

    fn has_downlink_pressure(&self) -> bool {
        self.streams.values().any(|stream| {
            !stream.closed && (stream.downlink.pending_len() > 0 || stream.pending_output_bytes > 0)
        })
    }

    fn upload_pending_bytes(&self) -> usize {
        self.streams
            .values()
            .filter(|stream| !stream.closed)
            .map(|stream| stream.upload_send.retained_len())
            .sum()
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
        let profile = self.runtime.config.mode_profile();
        if self.runtime.config.tcp_first_resolver_path() {
            profile.upload_chunk_constrained
        } else {
            profile.upload_chunk_normal
        }
    }

    fn poll_is_active(&self, now: Instant) -> bool {
        self.active_poll_until > now || self.streams.values().any(|stream| stream.wants_poll(now))
    }

    fn poll_interval(&self, now: Instant) -> Duration {
        client_poll_interval(self.poll_is_active(now), self.runtime.config.poll_interval)
    }

    fn ping_inflight_limit(&self, now: Instant) -> usize {
        let profile = self.runtime.config.mode_profile();
        if self.streams.values().any(|stream| stream.wants_poll(now)) {
            profile.ping_inflight_active
        } else {
            profile.ping_inflight_idle
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
                ..
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
                rate_limited_eprintln(
                    format!("resolver:{}:packet-failed", result.resolver),
                    format!(
                        "resolver {} packet {} failed: {error:#}",
                        result.resolver, result.packet_no
                    ),
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
            Frame::Open { .. } | Frame::Ping { .. } | Frame::PathChallenge { .. } => {}
            Frame::PathResponse { nonce, bytes }
                if nonce == FRONTIER_SHORT_ALIAS_READY_NONCE
                    && bytes == frontier_short_alias_signal(self.conn_id) =>
            {
                if self.runtime.config.mode == ClientMode::Frontier {
                    self.frontier_short_header_ready = true;
                }
            }
            Frame::PathResponse { .. } => {}
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
    use_frontier_short_header: bool,
) -> Result<DnsPacketOutcome> {
    send_dns_packet_inner(
        runtime,
        resolver_index,
        resolver,
        class,
        packet,
        DnsSendOptions {
            query_timeout,
            direct_tcp_first: false,
            use_frontier_short_header,
        },
    )
    .await
}

#[derive(Clone, Copy)]
struct DnsSendOptions {
    query_timeout: Duration,
    direct_tcp_first: bool,
    use_frontier_short_header: bool,
}

async fn send_dns_packet_inner(
    runtime: &ClientRuntime,
    resolver_index: Option<usize>,
    resolver: SocketAddr,
    class: Option<ClientSendClass>,
    packet: &Packet,
    options: DnsSendOptions,
) -> Result<DnsPacketOutcome> {
    let config = &runtime.config;
    let envelope = seal_client_packet(config, packet, options.use_frontier_short_header)?;
    let qname = if config.mode == ClientMode::Frontier {
        envelope_to_compact_qname(&envelope, &config.domain)?
    } else {
        envelope_to_qname(&envelope, &config.domain)?
    };
    let dns_id = (packet.packet_no as u16).wrapping_mul(31).wrapping_add(7);
    let query = build_query(dns_id, &qname, packet.max_response_bytes)?;
    if let Some(diag) = &runtime.diag {
        diag.queries_sent.fetch_add(1, Ordering::Relaxed);
        diag.query_wire_bytes
            .fetch_add(query.len() as u64, Ordering::Relaxed);
        diag.request_packet_body_bytes.fetch_add(
            encoded_client_packet_len(config, packet) as u64,
            Ordering::Relaxed,
        );
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
    let prefer_tcp = options.direct_tcp_first
        || match resolver_index {
            Some(index) => runtime.prefer_tcp_for_resolver(index, class).await,
            None => false,
        };
    let (response, transport) = if let Some(pool) = &runtime.tcp_pool {
        (
            pool.query(resolver, &query, options.query_timeout).await?,
            DnsTransportOutcome::TcpProxy,
        )
    } else if prefer_tcp {
        match runtime
            .tcp_fallback_pool
            .query(
                resolver,
                &query,
                options.query_timeout.max(PATH_RTO_MIN_TCP),
            )
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
                rate_limited_eprintln(
                    format!("resolver:{resolver}:preferred-tcp-failed"),
                    format!(
                        "resolver {resolver} preferred TCP failed ({tcp_error:#}); retrying UDP"
                    ),
                );
                runtime.tcp_fallback_pool.remove_sender(resolver).await;
                (
                    runtime
                        .udp_pool
                        .query(resolver, &query, options.query_timeout)
                        .await?,
                    DnsTransportOutcome::UdpAfterPreferredTcpError,
                )
            }
        }
    } else {
        let response = match runtime
            .udp_pool
            .query(resolver, &query, options.query_timeout)
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
                rate_limited_eprintln(
                    format!("resolver:{resolver}:udp-failed"),
                    format!("resolver {resolver} UDP failed ({udp_error:#}); retrying over TCP"),
                );
                match runtime
                    .tcp_fallback_pool
                    .query(
                        resolver,
                        &query,
                        options.query_timeout.max(PATH_RTO_MIN_TCP),
                    )
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
                rate_limited_eprintln(
                    format!("resolver:{resolver}:udp-truncated"),
                    format!("resolver {resolver} returned truncated UDP DNS response; retrying over TCP"),
                );
                let tcp_response = runtime
                    .tcp_fallback_pool
                    .query(
                        resolver,
                        &query,
                        options.query_timeout.max(PATH_RTO_MIN_TCP),
                    )
                    .await?;
                return Ok(DnsPacketOutcome {
                    packet: open_dns_response(&config.access_key, &tcp_response)?,
                    response_wire_bytes: tcp_response.len().max(udp_response_len),
                    truncated: true,
                    transport: DnsTransportOutcome::TcpFallbackAfterTruncation,
                });
            }
            Err(error) if config.allow_udp_to_tcp_fallback() => {
                if let Some(diag) = &runtime.diag {
                    diag.tcp_fallbacks.fetch_add(1, Ordering::Relaxed);
                }
                let udp_response_len = response.len();
                rate_limited_eprintln(
                    format!("resolver:{resolver}:udp-unusable"),
                    format!(
                        "resolver {resolver} returned unusable UDP DNS response ({error:#}); retrying over TCP"
                    ),
                );
                match runtime
                    .tcp_fallback_pool
                    .query(
                        resolver,
                        &query,
                        options.query_timeout.max(PATH_RTO_MIN_TCP),
                    )
                    .await
                {
                    Ok(tcp_response) => {
                        return Ok(DnsPacketOutcome {
                            packet: open_dns_response(&config.access_key, &tcp_response)?,
                            response_wire_bytes: tcp_response.len().max(udp_response_len),
                            truncated: false,
                            transport: DnsTransportOutcome::TcpAfterUdpFailure,
                        });
                    }
                    Err(tcp_error) => {
                        runtime.tcp_fallback_pool.remove_sender(resolver).await;
                        return Err(tcp_error.context(format!(
                            "UDP DNS response was unusable ({error:#}); DNS-over-TCP fallback failed"
                        )));
                    }
                }
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

fn seal_client_packet(
    config: &ClientConfig,
    packet: &Packet,
    use_frontier_short_header: bool,
) -> Result<Vec<u8>> {
    if config.mode == ClientMode::Frontier && use_frontier_short_header {
        seal_packet_frontier_short(&config.access_key, Direction::ClientToServer, packet)
    } else if config.mode == ClientMode::Frontier {
        seal_packet_frontier(&config.access_key, Direction::ClientToServer, packet)
    } else {
        seal_packet(&config.access_key, Direction::ClientToServer, packet)
    }
}

fn encoded_client_packet_len(config: &ClientConfig, packet: &Packet) -> usize {
    if config.mode == ClientMode::Frontier {
        encoded_packet_len_frontier(packet).unwrap_or(usize::MAX)
    } else {
        packet.encoded_len()
    }
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
        rate_limited_eprintln(
            format!("resolver:{resolver}:udp-worker-failed"),
            format!("resolver {resolver} persistent UDP worker failed: {error:#}"),
        );
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
                    rate_limited_eprintln(
                        format!("resolver:{resolver}:tcp-connection-failed"),
                        format!("resolver {resolver} persistent TCP connection failed: {error:#}"),
                    );
                    tokio::time::sleep(TCP_RECONNECT_DELAY).await;
                }
            }
            Err(error) => {
                let _ = first_request.response.send(Err(anyhow::anyhow!(
                    "DNS-over-TCP connect failed: {error:#}"
                )));
                rate_limited_eprintln(
                    format!("resolver:{resolver}:tcp-connect-failed"),
                    format!("resolver {resolver} DNS-over-TCP connect failed: {error:#}"),
                );
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
    let udp_gateway = match config.udp_gateway_listen {
        Some(bind) => Some(
            TcpListener::bind(bind)
                .await
                .with_context(|| format!("bind UDP gateway {bind}"))?,
        ),
        None => None,
    };
    let shared = Arc::new(ServerState::new(config));

    eprintln!("trajectory server listening on {}", udp.local_addr()?);
    if let Some(udp_gateway) = udp_gateway {
        tokio::spawn(async move {
            if let Err(error) = run_udp_gateway_server(udp_gateway).await {
                eprintln!("UDP gateway failed: {error:#}");
            }
        });
    }
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
    frontier_aliases: Mutex<HashMap<u32, FrontierShortAlias>>,
    next_cleanup_at: StdMutex<Instant>,
}

impl ServerState {
    fn new(config: ServerConfig) -> Self {
        Self {
            config,
            sessions: Mutex::new(HashMap::new()),
            connections: Mutex::new(HashMap::new()),
            frontier_aliases: Mutex::new(HashMap::new()),
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

    async fn register_frontier_alias(&self, client_id: u32, conn_id: u64) -> bool {
        let alias = frontier_short_conn_alias(conn_id);
        let mut aliases = self.frontier_aliases.lock().await;
        match aliases.get_mut(&alias) {
            Some(existing) if existing.client_id == client_id && existing.conn_id == conn_id => {
                existing.last_activity = Instant::now();
                true
            }
            Some(_) => false,
            None => {
                aliases.insert(
                    alias,
                    FrontierShortAlias {
                        client_id,
                        conn_id,
                        last_activity: Instant::now(),
                    },
                );
                true
            }
        }
    }

    async fn frontier_alias_connection(&self, alias: u32) -> Option<(ClientAccessKey, u64)> {
        let (client_id, conn_id) = {
            let mut aliases = self.frontier_aliases.lock().await;
            let entry = aliases.get_mut(&alias)?;
            entry.last_activity = Instant::now();
            (entry.client_id, entry.conn_id)
        };
        let key = self.config.authorized_clients.get(&client_id)?.clone();
        Some((key, conn_id))
    }

    async fn session(&self, key: SessionKey) -> Option<Arc<SessionHandle>> {
        let session = self.sessions.lock().await.get(&key).cloned();
        if let Some(session) = &session {
            session.touch();
        }
        session
    }

    async fn get_or_create_session(
        &self,
        key: SessionKey,
        open_target: Option<OpenTarget>,
    ) -> Arc<SessionHandle> {
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
                ServerTargetMode::Socks5Direct => match open_target {
                    Some(target) => {
                        run_server_direct_target_session(
                            SocketTarget::from_open_target(target),
                            upload_rx,
                            download_tx,
                        )
                        .await
                    }
                    None => run_server_socks5_direct_session(upload_rx, download_tx).await,
                },
            };
            if let Err(error) = result {
                if !is_benign_server_target_error(&error) {
                    eprintln!("server target session failed: {error:#}");
                }
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
        self.frontier_aliases
            .lock()
            .await
            .retain(|_, alias| alias.last_activity > cutoff);
    }
}

type ConnectionKey = (u32, u64);
type SessionKey = (u32, u64, u64);

struct FrontierShortAlias {
    client_id: u32,
    conn_id: u64,
    last_activity: Instant,
}

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
    upload_rx: mpsc::Receiver<UploadFrame>,
    download_tx: mpsc::Sender<DownloadFrame>,
) -> Result<()> {
    let stream = TcpStream::connect(target)
        .await
        .with_context(|| format!("connect target {target}"))?;
    run_server_connected_stream(stream, upload_rx, download_tx).await
}

async fn run_server_direct_target_session(
    target: SocketTarget,
    upload_rx: mpsc::Receiver<UploadFrame>,
    download_tx: mpsc::Sender<DownloadFrame>,
) -> Result<()> {
    let stream = match connect_socket_target(target).await {
        Ok(stream) => stream,
        Err(error) => {
            let _ = download_tx
                .send(DownloadFrame {
                    offset: 0,
                    fin: true,
                    bytes: Vec::new(),
                })
                .await;
            return Err(error);
        }
    };
    run_server_connected_stream(stream, upload_rx, download_tx).await
}

async fn run_server_connected_stream(
    stream: TcpStream,
    mut upload_rx: mpsc::Receiver<UploadFrame>,
    download_tx: mpsc::Sender<DownloadFrame>,
) -> Result<()> {
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

#[derive(Clone, Debug, Eq, PartialEq)]
enum SocketTarget {
    Ip(SocketAddr),
    Domain(String, u16),
}

impl SocketTarget {
    fn from_open_target(target: OpenTarget) -> Self {
        if let Ok(ip) = target.host.parse::<std::net::IpAddr>() {
            Self::Ip(SocketAddr::new(ip, target.port))
        } else {
            Self::Domain(target.host, target.port)
        }
    }
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

const UDPGW_FLAG_KEEPALIVE: u8 = 0x01;
const UDPGW_FLAG_DATA: u8 = 0x02;
const UDPGW_FLAG_ERROR: u8 = 0x20;
const SERVER_UDPGW_RESPONSE_CHANNEL: usize = 128;
const SERVER_UDPGW_RESPONSE_MTU: usize = 4096;
const SERVER_UDPGW_CLIENT_IDLE: Duration = Duration::from_secs(60);
const SERVER_UDPGW_SOCKET_IDLE: Duration = Duration::from_secs(60);

#[derive(Clone)]
struct UdpGatewayAssociation {
    socket: Arc<UdpSocket>,
    target: SocketTarget,
    generation: u64,
}

#[derive(Clone, Debug)]
struct UdpGatewayPacket {
    flags: u8,
    conn_id: u16,
    target: Option<SocketTarget>,
    data: Vec<u8>,
}

impl UdpGatewayPacket {
    fn keepalive(conn_id: u16) -> Self {
        Self {
            flags: UDPGW_FLAG_KEEPALIVE,
            conn_id,
            target: None,
            data: Vec::new(),
        }
    }

    fn error(conn_id: u16) -> Self {
        Self {
            flags: UDPGW_FLAG_ERROR,
            conn_id,
            target: None,
            data: Vec::new(),
        }
    }

    fn data(conn_id: u16, target: SocketTarget, data: Vec<u8>) -> Self {
        Self {
            flags: UDPGW_FLAG_DATA,
            conn_id,
            target: Some(target),
            data,
        }
    }
}

async fn run_udp_gateway_server(listener: TcpListener) -> Result<()> {
    eprintln!(
        "trajectory UDP gateway listening on {}",
        listener.local_addr()?
    );
    loop {
        let (stream, peer) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(error) = run_udp_gateway_client(stream).await {
                if !is_benign_udp_gateway_error(&error) {
                    eprintln!("UDP gateway connection from {peer} failed: {error:#}");
                }
            }
        });
    }
}

fn is_benign_server_target_error(error: &anyhow::Error) -> bool {
    let text = format!("{error:#}").to_ascii_lowercase();
    text.contains("channel closed")
        || text.contains("broken pipe")
        || text.contains("connection reset by peer")
        || text.contains("early eof")
}

fn is_benign_udp_gateway_error(error: &anyhow::Error) -> bool {
    let text = format!("{error:#}").to_ascii_lowercase();
    text.contains("early eof")
        || text.contains("broken pipe")
        || text.contains("connection reset by peer")
        || text.contains("unsupported address type")
}

async fn run_udp_gateway_client(stream: TcpStream) -> Result<()> {
    let (mut reader, mut writer) = stream.into_split();
    let (tx, mut rx) = mpsc::channel::<UdpGatewayPacket>(SERVER_UDPGW_RESPONSE_CHANNEL);
    let associations = Arc::new(Mutex::new(HashMap::<u16, UdpGatewayAssociation>::new()));
    let next_generation = Arc::new(AtomicU64::new(1));

    let read_tx = tx.clone();
    let read_associations = Arc::clone(&associations);
    let read_generations = Arc::clone(&next_generation);
    let read_task = tokio::spawn(async move {
        let mut last_activity = Instant::now();
        loop {
            let packet =
                match timeout(Duration::from_secs(2), read_udp_gateway_packet(&mut reader)).await {
                    Ok(Ok(packet)) => packet,
                    Ok(Err(error)) => return Err(error),
                    Err(_) if last_activity.elapsed() >= SERVER_UDPGW_CLIENT_IDLE => return Ok(()),
                    Err(_) => continue,
                };
            last_activity = Instant::now();

            if packet.flags & UDPGW_FLAG_KEEPALIVE == UDPGW_FLAG_KEEPALIVE {
                let _ = read_tx
                    .send(UdpGatewayPacket::keepalive(packet.conn_id))
                    .await;
                continue;
            }
            if packet.flags & UDPGW_FLAG_DATA != UDPGW_FLAG_DATA {
                let _ = read_tx.send(UdpGatewayPacket::error(packet.conn_id)).await;
                continue;
            }

            let tx = read_tx.clone();
            let associations = Arc::clone(&read_associations);
            let generations = Arc::clone(&read_generations);
            tokio::spawn(async move {
                if handle_udp_gateway_data_packet(
                    packet.clone(),
                    associations,
                    generations,
                    tx.clone(),
                )
                .await
                .is_err()
                {
                    let _ = tx.send(UdpGatewayPacket::error(packet.conn_id)).await;
                }
            });
        }
    });

    let write_task = tokio::spawn(async move {
        while let Some(packet) = rx.recv().await {
            write_udp_gateway_packet(&mut writer, &packet).await?;
        }
        Ok::<(), anyhow::Error>(())
    });

    tokio::select! {
        result = read_task => result.context("UDP gateway reader task panicked")??,
        result = write_task => result.context("UDP gateway writer task panicked")??,
    }
    Ok(())
}

async fn handle_udp_gateway_data_packet(
    packet: UdpGatewayPacket,
    associations: Arc<Mutex<HashMap<u16, UdpGatewayAssociation>>>,
    next_generation: Arc<AtomicU64>,
    tx: mpsc::Sender<UdpGatewayPacket>,
) -> Result<()> {
    let target = packet
        .target
        .clone()
        .context("UDP gateway data packet missing target")?;
    let association = {
        let existing = associations.lock().await.get(&packet.conn_id).cloned();
        match existing {
            Some(association) if association.target == target => association,
            _ => {
                let generation = next_generation.fetch_add(1, Ordering::Relaxed);
                let association = create_udp_gateway_association(
                    packet.conn_id,
                    target.clone(),
                    generation,
                    Arc::clone(&associations),
                    tx,
                )
                .await?;
                associations
                    .lock()
                    .await
                    .insert(packet.conn_id, association.clone());
                association
            }
        }
    };
    association
        .socket
        .send(&packet.data)
        .await
        .context("send UDP gateway datagram")?;
    Ok(())
}

async fn create_udp_gateway_association(
    conn_id: u16,
    target: SocketTarget,
    generation: u64,
    associations: Arc<Mutex<HashMap<u16, UdpGatewayAssociation>>>,
    tx: mpsc::Sender<UdpGatewayPacket>,
) -> Result<UdpGatewayAssociation> {
    let destination = resolve_udp_gateway_target(&target).await?;
    let bind = match destination {
        SocketAddr::V4(_) => "0.0.0.0:0",
        SocketAddr::V6(_) => "[::]:0",
    };
    let socket = Arc::new(
        UdpSocket::bind(bind)
            .await
            .with_context(|| format!("bind UDP gateway socket for {destination}"))?,
    );
    socket
        .connect(destination)
        .await
        .with_context(|| format!("connect UDP gateway target {destination}"))?;

    let reader_socket = Arc::clone(&socket);
    let response_target = target.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; SERVER_UDPGW_RESPONSE_MTU];
        loop {
            let len = match timeout(SERVER_UDPGW_SOCKET_IDLE, reader_socket.recv(&mut buf)).await {
                Ok(Ok(len)) => len,
                Ok(Err(_)) | Err(_) => break,
            };
            let current = associations.lock().await.get(&conn_id).cloned();
            if !matches!(
                current,
                Some(association)
                    if association.generation == generation && association.target == response_target
            ) {
                break;
            }
            if tx
                .send(UdpGatewayPacket::data(
                    conn_id,
                    response_target.clone(),
                    buf[..len].to_vec(),
                ))
                .await
                .is_err()
            {
                break;
            }
        }
        let mut associations = associations.lock().await;
        if matches!(
            associations.get(&conn_id),
            Some(association) if association.generation == generation
        ) {
            associations.remove(&conn_id);
        }
    });

    Ok(UdpGatewayAssociation {
        socket,
        target,
        generation,
    })
}

async fn resolve_udp_gateway_target(target: &SocketTarget) -> Result<SocketAddr> {
    match target {
        SocketTarget::Ip(addr) => Ok(*addr),
        SocketTarget::Domain(host, port) => {
            let mut addrs = tokio::net::lookup_host((host.as_str(), *port))
                .await
                .with_context(|| format!("resolve UDP gateway target {host}:{port}"))?;
            addrs
                .next()
                .with_context(|| format!("no UDP gateway target address for {host}:{port}"))
        }
    }
}

async fn read_udp_gateway_packet<R>(reader: &mut R) -> Result<UdpGatewayPacket>
where
    R: AsyncRead + Unpin,
{
    let mut len = [0u8; 2];
    reader
        .read_exact(&mut len)
        .await
        .context("read UDP gateway packet length")?;
    let len = u16::from_be_bytes(len) as usize;
    if len < 3 {
        bail!("UDP gateway packet is shorter than header");
    }
    let mut body = vec![0u8; len];
    reader
        .read_exact(&mut body)
        .await
        .context("read UDP gateway packet body")?;
    decode_udp_gateway_packet_body(&body)
}

fn decode_udp_gateway_packet_body(body: &[u8]) -> Result<UdpGatewayPacket> {
    if body.len() < 3 {
        bail!("UDP gateway packet is shorter than header");
    }
    let flags = body[0];
    let conn_id = u16::from_be_bytes([body[1], body[2]]);
    let mut index = 3;
    let target = if flags & UDPGW_FLAG_DATA == UDPGW_FLAG_DATA {
        Some(decode_udp_gateway_address(body, &mut index)?)
    } else {
        None
    };
    Ok(UdpGatewayPacket {
        flags,
        conn_id,
        target,
        data: body[index..].to_vec(),
    })
}

fn decode_udp_gateway_address(body: &[u8], index: &mut usize) -> Result<SocketTarget> {
    let atyp = *body
        .get(*index)
        .context("UDP gateway packet missing address type")?;
    *index += 1;
    match atyp {
        0x01 => {
            let octets = read_udp_gateway_bytes(body, index, 4)?;
            let port = read_udp_gateway_port(body, index)?;
            Ok(SocketTarget::Ip(SocketAddr::from((
                std::net::Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
                port,
            ))))
        }
        0x03 => {
            let len = *body
                .get(*index)
                .context("UDP gateway packet missing domain length")?
                as usize;
            *index += 1;
            let name = String::from_utf8(read_udp_gateway_bytes(body, index, len)?.to_vec())
                .context("UDP gateway domain is not valid UTF-8")?;
            let port = read_udp_gateway_port(body, index)?;
            Ok(SocketTarget::Domain(name, port))
        }
        0x04 => {
            let octets = read_udp_gateway_bytes(body, index, 16)?;
            let mut ip = [0u8; 16];
            ip.copy_from_slice(octets);
            let port = read_udp_gateway_port(body, index)?;
            Ok(SocketTarget::Ip(SocketAddr::from((
                std::net::Ipv6Addr::from(ip),
                port,
            ))))
        }
        _ => bail!("UDP gateway packet used unsupported address type {atyp}"),
    }
}

fn read_udp_gateway_bytes<'a>(body: &'a [u8], index: &mut usize, len: usize) -> Result<&'a [u8]> {
    let end = index.saturating_add(len);
    let bytes = body
        .get(*index..end)
        .context("UDP gateway packet truncated")?;
    *index = end;
    Ok(bytes)
}

fn read_udp_gateway_port(body: &[u8], index: &mut usize) -> Result<u16> {
    let bytes = read_udp_gateway_bytes(body, index, 2)?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

async fn write_udp_gateway_packet<W>(writer: &mut W, packet: &UdpGatewayPacket) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    let mut body = Vec::with_capacity(3 + packet.data.len() + 32);
    body.push(packet.flags);
    body.extend_from_slice(&packet.conn_id.to_be_bytes());
    if let Some(target) = &packet.target {
        encode_udp_gateway_address(&mut body, target)?;
    }
    body.extend_from_slice(&packet.data);
    if body.len() > u16::MAX as usize {
        bail!("UDP gateway packet exceeds wire length limit");
    }
    writer
        .write_all(&(body.len() as u16).to_be_bytes())
        .await
        .context("write UDP gateway packet length")?;
    writer
        .write_all(&body)
        .await
        .context("write UDP gateway packet body")?;
    Ok(())
}

fn encode_udp_gateway_address(out: &mut Vec<u8>, target: &SocketTarget) -> Result<()> {
    match target {
        SocketTarget::Ip(SocketAddr::V4(addr)) => {
            out.push(0x01);
            out.extend_from_slice(&addr.ip().octets());
            out.extend_from_slice(&addr.port().to_be_bytes());
        }
        SocketTarget::Ip(SocketAddr::V6(addr)) => {
            out.push(0x04);
            out.extend_from_slice(&addr.ip().octets());
            out.extend_from_slice(&addr.port().to_be_bytes());
        }
        SocketTarget::Domain(host, port) => {
            if host.len() > u8::MAX as usize {
                bail!("UDP gateway domain is too long");
            }
            out.push(0x03);
            out.push(host.len() as u8);
            out.extend_from_slice(host.as_bytes());
            out.extend_from_slice(&port.to_be_bytes());
        }
    }
    Ok(())
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

async fn open_client_packet(
    state: &ServerState,
    envelope: &[u8],
) -> Result<(ClientAccessKey, Packet, bool)> {
    if let Ok(Some(alias)) = frontier_short_sealed_alias(envelope) {
        if let Some((key, conn_id)) = state.frontier_alias_connection(alias).await {
            if let Ok(packet) =
                open_packet_frontier_short(&key, conn_id, Direction::ClientToServer, envelope)
            {
                return Ok((key, packet, true));
            }
        }
    }
    let (key, packet) = open_packet_with_registry(
        &state.config.authorized_clients,
        Direction::ClientToServer,
        envelope,
    )?;
    Ok((key, packet, false))
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
    let (key, packet, used_frontier_short_header) =
        match open_client_packet(&state, &envelope).await {
            Ok(opened) => opened,
            Err(_) => return build_empty_response(&query, 0),
        };
    let frontier_alias_ready = !used_frontier_short_header
        && state
            .register_frontier_alias(key.client_id, packet.conn_id)
            .await;

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
            Frame::Open {
                stream_id,
                host,
                port,
            } => {
                push_unique_stream(&mut active_streams, stream_id);
                if !duplicate_client_packet {
                    let open_target = if host.is_empty() && port == 0 {
                        None
                    } else {
                        Some(OpenTarget::new(host, port).context("invalid open target")?)
                    };
                    state
                        .get_or_create_session(
                            (key.client_id, packet.conn_id, stream_id),
                            open_target,
                        )
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

    if frontier_alias_ready {
        let alias_frame = Frame::PathResponse {
            nonce: FRONTIER_SHORT_ALIAS_READY_NONCE,
            bytes: frontier_short_alias_signal(packet.conn_id),
        };
        if response_frame_fits(&query, &key, &response, &alias_frame)? {
            response.frames.push(alias_frame);
        }
    }

    let sessions = state
        .sessions_for_connection(key.client_id, packet.conn_id, &active_streams)
        .await;
    let sessions = connection.rotate_download_sessions(sessions).await;
    let sessions = bound_sessions_for_response(sessions, &active_streams);
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

fn bound_sessions_for_response(
    sessions: Vec<(u64, Arc<SessionHandle>)>,
    active_streams: &[u64],
) -> Vec<(u64, Arc<SessionHandle>)> {
    let limit = SERVER_RESPONSE_SESSION_WORK_LIMIT.max(active_streams.len());
    if sessions.len() <= limit {
        return sessions;
    }

    let mut out = Vec::with_capacity(limit);
    let mut seen = HashSet::new();
    for (stream_id, session) in &sessions {
        if active_streams.contains(stream_id) {
            out.push((*stream_id, Arc::clone(session)));
            seen.insert(*stream_id);
        }
    }
    for (stream_id, session) in sessions {
        if out.len() >= limit {
            break;
        }
        if seen.insert(stream_id) {
            out.push((stream_id, session));
        }
    }
    out
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
    if sessions.len() > 1 && should_coalesce_downloads(sessions).await {
        tokio::time::sleep(SERVER_DOWNLOAD_COALESCE_DELAY).await;
        for (_, session) in sessions {
            stage_download_frames(session).await?;
        }
    }
    append_admission_download_frames(query, key, response, sessions).await?;

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

async fn append_admission_download_frames(
    query: &trajectory_core::dns::DnsQuery,
    key: &ClientAccessKey,
    response: &mut Packet,
    sessions: &[(u64, Arc<SessionHandle>)],
) -> Result<()> {
    let pending = pending_download_session_count(sessions).await;
    if pending <= 1 {
        return Ok(());
    }
    let max_frame = download_admission_frame_max(response.max_response_bytes, pending);
    for (stream_id, session) in sessions {
        if response.frames.len() >= 60 {
            return Ok(());
        }
        let has_pending = {
            let send = session.download_send.lock().await;
            send.has_pending_send()
        };
        if has_pending {
            append_one_download_frame(query, key, response, *stream_id, session, max_frame).await?;
        }
    }
    Ok(())
}

async fn should_coalesce_downloads(sessions: &[(u64, Arc<SessionHandle>)]) -> bool {
    let mut ready = false;
    let mut waiting = false;
    for (_, session) in sessions {
        let send = session.download_send.lock().await;
        if send.has_pending_send() {
            ready = true;
        } else {
            waiting = true;
        }
        if ready && waiting {
            return true;
        }
    }
    false
}

async fn pending_download_session_count(sessions: &[(u64, Arc<SessionHandle>)]) -> usize {
    let mut pending = 0;
    for (_, session) in sessions {
        let send = session.download_send.lock().await;
        if send.has_pending_send() {
            pending += 1;
        }
    }
    pending
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

fn download_admission_frame_max(max_response_bytes: u16, pending_sessions: usize) -> usize {
    if pending_sessions <= 1 {
        return SERVER_DOWNLOAD_FRAME_MAX;
    }
    let usable = (max_response_bytes as usize).saturating_sub(DNS_RESPONSE_SAFETY_MARGIN + 160);
    let reserved_slots = pending_sessions.min(8) + 1;
    (usable / reserved_slots).clamp(
        SERVER_DOWNLOAD_ADMISSION_FRAME_MIN,
        SERVER_DOWNLOAD_ADMISSION_FRAME_MAX,
    )
}

async fn stage_download_frames(session: &SessionHandle) -> Result<()> {
    let mut send = session.download_send.lock().await;
    let mut download_rx = session.download_rx.lock().await;
    while send.retained_len() < SERVER_RETAINED_BYTE_LIMIT {
        match download_rx.try_recv() {
            Ok(download) => {
                send.append(download.offset, download.fin, download.bytes)
                    .context("retain target download bytes")?;
                session.touch();
            }
            Err(mpsc::error::TryRecvError::Empty) => break,
            Err(mpsc::error::TryRecvError::Disconnected) => {
                send.mark_fin_at_end();
                session.touch();
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
        session.touch();
        return Ok(true);
    }

    if send.is_finished() {
        let frame = Frame::Close { stream_id, code: 0 };
        if response_frame_fits(query, key, response, &frame)? || response.frames.is_empty() {
            response.frames.push(frame);
            session.touch();
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
    let envelope_len = match sealed_packet_len_with_extra_frame(response, frame) {
        Ok(len) => len,
        Err(_) => return Ok(false),
    };
    Ok(txt_response_wire_len(query, envelope_len) <= budget)
}

fn response_packet_fits(
    query: &trajectory_core::dns::DnsQuery,
    _key: &ClientAccessKey,
    response: &Packet,
) -> Result<bool> {
    let budget = dns_response_budget(response.max_response_bytes);
    let envelope_len = match sealed_packet_len(response) {
        Ok(len) => len,
        Err(_) => return Ok(false),
    };
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
    use_frontier_short_header: bool,
) -> Result<MuxSentKind> {
    while !client_request_fits(config, request, use_frontier_short_header) {
        if shrink_packet_ack_ranges(request) {
            continue;
        }
        if shrink_stream_ack_to_zero_ranges(request) {
            continue;
        }
        if remove_stream_ack_frame(request) {
            continue;
        }
        if shrink_mux_data_frame(config, request, &mut kind, use_frontier_short_header) {
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

    if !client_request_fits(config, request, use_frontier_short_header)
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
            target,
            first_data: Some(_),
        } if !client_request_fits(config, request, use_frontier_short_header) => {
            remove_last_data_frame(request);
            let kind = MuxSentKind::Open {
                stream_id,
                target,
                first_data: None,
            };
            if client_request_fits(config, request, use_frontier_short_header) {
                Ok(kind)
            } else {
                bail!("client DNS request cannot fit query name after reducing open packet")
            }
        }
        other if client_request_fits(config, request, use_frontier_short_header) => Ok(other),
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
    use_frontier_short_header: bool,
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
        if client_request_fits(config, request, use_frontier_short_header) {
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

fn client_request_fits(
    config: &ClientConfig,
    request: &Packet,
    use_frontier_short_header: bool,
) -> bool {
    sealed_client_packet_len(config, request, use_frontier_short_header)
        .and_then(|envelope_len| {
            if config.mode == ClientMode::Frontier {
                compact_envelope_qname_len(envelope_len, &config.domain)
            } else {
                envelope_qname_len(envelope_len, &config.domain)
            }
        })
        .map(|qname_len| qname_len <= 253)
        .unwrap_or(false)
}

fn sealed_client_packet_len(
    config: &ClientConfig,
    request: &Packet,
    use_frontier_short_header: bool,
) -> Result<usize> {
    if config.mode == ClientMode::Frontier && use_frontier_short_header {
        sealed_packet_len_frontier_short(request)
    } else if config.mode == ClientMode::Frontier {
        sealed_packet_len_frontier(request)
    } else {
        sealed_packet_len(request)
    }
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
            socks_listen: None,
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
            mode: ClientMode::Secure,
            max_active_streams: None,
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
    fn proxy_stream_disconnects_are_not_logged_as_failures() {
        assert!(is_benign_proxy_stream_error(&anyhow::anyhow!(
            "write tcp response: Broken pipe (os error 32)"
        )));
        assert!(is_benign_proxy_stream_error(&anyhow::anyhow!(
            "write tcp response: Connection reset by peer (os error 104)"
        )));
        assert!(is_benign_proxy_stream_error(&anyhow::anyhow!(
            "HTTP proxy client closed before sending headers"
        )));
        assert!(is_benign_proxy_stream_error(&anyhow::anyhow!(
            "read SOCKS greeting: early eof"
        )));
        assert!(!is_benign_proxy_stream_error(&anyhow::anyhow!(
            "SOCKS proxy mode supports CONNECT only"
        )));
        assert!(!is_benign_proxy_stream_error(&anyhow::anyhow!(
            "register local stream with client transport"
        )));
    }

    #[test]
    fn normal_server_stream_shutdowns_are_not_logged_as_failures() {
        assert!(is_benign_server_target_error(&anyhow::anyhow!(
            "queue target download bytes: channel closed"
        )));
        assert!(is_benign_udp_gateway_error(&anyhow::anyhow!(
            "read UDP gateway packet length: early eof"
        )));
        assert!(is_benign_udp_gateway_error(&anyhow::anyhow!(
            "UDP gateway packet used unsupported address type 99"
        )));
        assert!(!is_benign_udp_gateway_error(&anyhow::anyhow!(
            "resolve UDP gateway target failed"
        )));
    }

    #[test]
    fn udp_gateway_packet_codec_round_trips_domain_targets() {
        let target = SocketTarget::Domain("example.com".to_string(), 443);
        let packet = UdpGatewayPacket::data(42, target.clone(), b"hello".to_vec());
        let mut body = Vec::new();
        body.push(packet.flags);
        body.extend_from_slice(&packet.conn_id.to_be_bytes());
        encode_udp_gateway_address(&mut body, packet.target.as_ref().unwrap()).unwrap();
        body.extend_from_slice(&packet.data);

        let decoded = decode_udp_gateway_packet_body(&body).unwrap();

        assert_eq!(decoded.flags, UDPGW_FLAG_DATA);
        assert_eq!(decoded.conn_id, 42);
        assert_eq!(decoded.target, Some(target));
        assert_eq!(decoded.data, b"hello");
    }

    #[tokio::test]
    async fn udp_gateway_forwards_datagrams_over_tcp_connection() {
        let echo_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let echo_addr = echo_socket.local_addr().unwrap();
        let echo_task = tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let (len, peer) = echo_socket.recv_from(&mut buf).await.unwrap();
            echo_socket.send_to(&buf[..len], peer).await.unwrap();
        });

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let listen_addr = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            run_udp_gateway_client(stream).await
        });

        let mut stream = TcpStream::connect(listen_addr).await.unwrap();
        write_udp_gateway_packet(
            &mut stream,
            &UdpGatewayPacket::data(7, SocketTarget::Ip(echo_addr), b"ping".to_vec()),
        )
        .await
        .unwrap();

        let response = timeout(Duration::from_secs(2), read_udp_gateway_packet(&mut stream))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(response.flags, UDPGW_FLAG_DATA);
        assert_eq!(response.conn_id, 7);
        assert_eq!(response.target, Some(SocketTarget::Ip(echo_addr)));
        assert_eq!(response.data, b"ping");

        echo_task.await.unwrap();
        server_task.abort();
    }

    #[tokio::test]
    async fn udp_gateway_ignores_stale_responses_after_conn_id_retarget() {
        let old_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let old_addr = old_socket.local_addr().unwrap();
        let old_task = tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let (len, peer) = old_socket.recv_from(&mut buf).await.unwrap();
            tokio::time::sleep(Duration::from_millis(150)).await;
            old_socket.send_to(&buf[..len], peer).await.unwrap();
        });

        let new_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let new_addr = new_socket.local_addr().unwrap();
        let new_task = tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let (len, peer) = new_socket.recv_from(&mut buf).await.unwrap();
            new_socket.send_to(&buf[..len], peer).await.unwrap();
        });

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let listen_addr = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            run_udp_gateway_client(stream).await
        });

        let mut stream = TcpStream::connect(listen_addr).await.unwrap();
        write_udp_gateway_packet(
            &mut stream,
            &UdpGatewayPacket::data(9, SocketTarget::Ip(old_addr), b"old".to_vec()),
        )
        .await
        .unwrap();
        write_udp_gateway_packet(
            &mut stream,
            &UdpGatewayPacket::data(9, SocketTarget::Ip(new_addr), b"new".to_vec()),
        )
        .await
        .unwrap();

        let response = timeout(Duration::from_secs(2), read_udp_gateway_packet(&mut stream))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(response.flags, UDPGW_FLAG_DATA);
        assert_eq!(response.conn_id, 9);
        assert_eq!(response.target, Some(SocketTarget::Ip(new_addr)));
        assert_eq!(response.data, b"new");
        assert!(
            timeout(
                Duration::from_millis(350),
                read_udp_gateway_packet(&mut stream)
            )
            .await
            .is_err(),
            "stale response from previous UDP target was forwarded"
        );

        old_task.await.unwrap();
        new_task.await.unwrap();
        server_task.abort();
    }

    #[test]
    fn proxy_path_has_no_global_fixed_pacing_floor() {
        let mut health = ProxyHealth::default();
        health.record_result(
            true,
            Duration::from_millis(20),
            Duration::from_millis(1),
            Duration::from_secs(30),
            PROXY_MAX_CWND,
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
    fn admission_download_cap_preserves_first_frame_slots() {
        assert_eq!(
            download_admission_frame_max(1232, 1),
            SERVER_DOWNLOAD_FRAME_MAX
        );
        assert!(download_admission_frame_max(700, 2) <= SERVER_DOWNLOAD_ADMISSION_FRAME_MAX);
        assert!(download_admission_frame_max(700, 2) >= SERVER_DOWNLOAD_ADMISSION_FRAME_MIN);
        assert!(
            download_admission_frame_max(700, 8) < download_admission_frame_max(700, 2),
            "more pending streams should receive smaller admission slices"
        );
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
        config.resolvers = vec!["192.0.2.1:53".parse().unwrap(); 2];
        assert!(should_admit_resolvers(&config));
    }

    #[test]
    fn admission_thresholds_are_stricter_for_proxy_paths() {
        assert!(admission_challenges(false).len() >= admission_challenges(true).len());
        assert!(admission_max_elapsed(true) > admission_max_elapsed(false));
        assert!(admission_min_response_bps(false) > admission_min_response_bps(true));
        assert!(resolver_admission_probe_timeout(true) > resolver_admission_timeout(true));
    }

    #[test]
    fn admission_probe_trims_padding_to_dns_name_budget() {
        let mut config = test_client_config(false, 1232);
        config.domain = "t.android-smoke".to_string();
        config.mode = ClientMode::Secure;

        let mut packet = Packet::new(u64::MAX - 1, 7);
        packet.max_response_bytes = config.dns_max_payload;
        packet.ack_ranges = vec![AckRange { first: 1, last: 4 }];
        packet.frames.push(Frame::PathChallenge {
            nonce: 7,
            response_bytes: 128,
        });
        packet.frames.push(Frame::PathResponse {
            nonce: 7,
            bytes: vec![0; 112],
        });

        assert!(!client_request_fits(&config, &packet, false));
        fit_admission_probe_packet(&config, &mut packet).unwrap();
        assert!(client_request_fits(&config, &packet, false));

        let padding_len = packet
            .frames
            .iter()
            .find_map(|frame| match frame {
                Frame::PathResponse { bytes, .. } => Some(bytes.len()),
                _ => None,
            })
            .unwrap_or(0);
        assert!(padding_len < 112);
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
    async fn direct_udp_timeout_falls_back_to_tcp_with_short_tcp_preference() {
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
            false,
        )
        .await
        .unwrap();

        assert_eq!(outcome.transport, DnsTransportOutcome::TcpAfterUdpFailure);
        runtime
            .record_resolver_result(
                0,
                ResolverResultSample {
                    ok: true,
                    elapsed: Duration::from_millis(60),
                    truncated: outcome.truncated,
                    useful_bytes: 0,
                    request_upload_bytes: 0,
                    class: Some(ClientSendClass::Control),
                    transport: Some(outcome.transport),
                },
            )
            .await;
        assert!(
            runtime
                .prefer_tcp_for_resolver(0, Some(ClientSendClass::Control))
                .await
        );
        assert!(
            !runtime
                .prefer_tcp_for_resolver(0, Some(ClientSendClass::Data))
                .await
        );
        runtime
            .record_resolver_result(
                0,
                ResolverResultSample {
                    ok: true,
                    elapsed: Duration::from_millis(60),
                    truncated: false,
                    useful_bytes: 0,
                    request_upload_bytes: 64,
                    class: Some(ClientSendClass::Data),
                    transport: Some(DnsTransportOutcome::TcpAfterUdpFailure),
                },
            )
            .await;
        assert!(
            runtime
                .prefer_tcp_for_resolver(0, Some(ClientSendClass::Data))
                .await
        );
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
            false,
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

    #[test]
    fn response_budget_shrinks_packets_over_frame_limit() {
        let key = ClientAccessKey::generate();
        let query_bytes = build_query(9, "t-aa.t.example.test", 4096).unwrap();
        let query = parse_query(&query_bytes).unwrap();
        let mut packet = Packet::new(11, 3);
        packet.max_response_bytes = 4096;
        for nonce in 0..65 {
            packet.frames.push(Frame::Ping { nonce });
        }

        assert!(!response_packet_fits(&query, &key, &packet).unwrap());
        assert!(!response_frame_fits(&query, &key, &packet, &Frame::Ping { nonce: 99 }).unwrap());

        ensure_response_packet_fits(&query, &key, &mut packet, 123).unwrap();

        assert!(packet.frames.len() <= 64);
        assert!(response_packet_fits(&query, &key, &packet).unwrap());
    }

    #[tokio::test]
    async fn server_opens_frontier_short_packet_after_alias_registration() {
        let key = ClientAccessKey::generate();
        let state = ServerState::new(ServerConfig {
            bind: "127.0.0.1:0".parse().unwrap(),
            domain: "t.example.test".to_string(),
            target: "127.0.0.1:1".parse().unwrap(),
            target_mode: ServerTargetMode::Tcp,
            udp_gateway_listen: None,
            authorized_clients: Arc::new(HashMap::from([(key.client_id, key.clone())])),
        });
        let mut full_packet = Packet::new(0x123456, 0);
        full_packet.frames.push(Frame::Ping { nonce: 0 });
        let full = seal_packet_frontier(&key, Direction::ClientToServer, &full_packet).unwrap();

        let (opened_key, opened, used_short) = open_client_packet(&state, &full).await.unwrap();
        assert_eq!(opened_key.client_id, key.client_id);
        assert_eq!(opened, full_packet);
        assert!(!used_short);
        assert!(
            state
                .register_frontier_alias(key.client_id, full_packet.conn_id)
                .await
        );

        let mut short_packet = Packet::new(full_packet.conn_id, 1);
        short_packet.frames.push(Frame::Data {
            stream_id: 0,
            offset: 0,
            fin: false,
            bytes: vec![1; 64],
        });
        let short =
            seal_packet_frontier_short(&key, Direction::ClientToServer, &short_packet).unwrap();
        let (_, opened, used_short) = open_client_packet(&state, &short).await.unwrap();
        assert_eq!(opened, short_packet);
        assert!(used_short);
    }

    #[tokio::test]
    async fn session_scan_does_not_refresh_idle_sessions() {
        let key = ClientAccessKey::generate();
        let state = ServerState::new(ServerConfig {
            bind: "127.0.0.1:0".parse().unwrap(),
            domain: "t.example.test".to_string(),
            target: "127.0.0.1:1".parse().unwrap(),
            target_mode: ServerTargetMode::Tcp,
            udp_gateway_listen: None,
            authorized_clients: Arc::new(HashMap::from([(key.client_id, key.clone())])),
        });
        let old_activity = Instant::now() - SERVER_STATE_IDLE_TIMEOUT - Duration::from_secs(1);
        for stream_id in [1, 2] {
            let (upload_tx, _upload_rx) = mpsc::channel(1);
            let (_download_tx, download_rx) = mpsc::channel(1);
            let session = Arc::new(SessionHandle::new(upload_tx, download_rx));
            *session.last_activity.lock().unwrap() = old_activity;
            state
                .sessions
                .lock()
                .await
                .insert((key.client_id, 7, stream_id), session);
        }

        let sessions = state
            .sessions_for_connection(key.client_id, 7, &[1, 2])
            .await;
        assert_eq!(sessions.len(), 2);

        *state.next_cleanup_at.lock().unwrap() = Instant::now() - Duration::from_secs(1);
        state.cleanup_idle(Instant::now()).await;
        assert!(state.sessions.lock().await.is_empty());
    }

    #[test]
    fn response_session_work_bound_preserves_active_streams() {
        let sessions = (0..150)
            .map(|stream_id| {
                let (upload_tx, _upload_rx) = mpsc::channel(1);
                let (_download_tx, download_rx) = mpsc::channel(1);
                (
                    stream_id,
                    Arc::new(SessionHandle::new(upload_tx, download_rx)),
                )
            })
            .collect::<Vec<_>>();
        let active_streams = [3, 120, 149];

        let bounded = bound_sessions_for_response(sessions, &active_streams);
        let stream_ids = bounded
            .iter()
            .map(|(stream_id, _)| *stream_id)
            .collect::<HashSet<_>>();

        assert_eq!(bounded.len(), SERVER_RESPONSE_SESSION_WORK_LIMIT);
        assert_eq!(stream_ids.len(), SERVER_RESPONSE_SESSION_WORK_LIMIT);
        for active in active_streams {
            assert!(stream_ids.contains(&active));
        }
    }

    #[test]
    fn max_active_streams_override_controls_stream_slots() {
        let mut config = test_client_config(false, 1232);
        config.max_active_streams = Some(256);
        let runtime = ClientRuntime::new(config);

        assert_eq!(runtime.stream_slots.available_permits(), 256);
    }

    #[test]
    fn frontier_uses_shorter_connection_aliases() {
        assert!(
            client_conn_id_mask(ClientMode::Frontier) < client_conn_id_mask(ClientMode::Velocity)
        );
        for _ in 0..1024 {
            let conn_id = fresh_client_conn_id(ClientMode::Frontier);
            assert!(conn_id > 0);
            assert!(conn_id <= FRONTIER_CLIENT_CONN_ID_MASK);
        }
    }
}
