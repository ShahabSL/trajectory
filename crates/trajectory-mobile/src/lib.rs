use std::fmt;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::watch;
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::client::{
    default_client_config, default_public_resolvers, parse_socket_addr, run_until, ClientConfig,
};

uniffi::setup_scaffolding!("trajectorymobile");

const DEFAULT_DOMAIN: &str = "your.domain.example";
const DEFAULT_KEEP_ALIVE_MS: u64 = 50;

#[derive(Clone, Debug, uniffi::Record)]
pub struct MobileTunnelConfig {
    pub access_key: String,
    pub domain: String,
    pub listen_port: u16,
    pub keep_alive_ms: u64,
    pub resolvers: Vec<String>,
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct MobileLogEntry {
    pub timestamp: String,
    pub message: String,
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct MobileTunnelSnapshot {
    pub state: MobileTunnelState,
    pub status_text: String,
    pub listen_address: String,
    pub active_resolvers: u32,
    pub last_error: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, uniffi::Enum)]
pub enum MobileTunnelState {
    Idle,
    Starting,
    Running,
    Stopping,
    Failed,
}

#[derive(Debug, uniffi::Error)]
pub enum MobileError {
    InvalidConfiguration(String),
    AlreadyRunning,
    NotRunning,
    RuntimeFailure(String),
}

impl fmt::Display for MobileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => write!(f, "{message}"),
            Self::AlreadyRunning => write!(f, "the tunnel is already running"),
            Self::NotRunning => write!(f, "the tunnel is not running"),
            Self::RuntimeFailure(message) => write!(f, "{message}"),
        }
    }
}

#[derive(uniffi::Object)]
pub struct TrajectoryMobileController {
    inner: Arc<Mutex<ControllerInner>>,
}

struct ControllerInner {
    snapshot: MobileTunnelSnapshot,
    logs: Vec<MobileLogEntry>,
    stop_tx: Option<watch::Sender<bool>>,
    active_run_id: Option<u64>,
    next_run_id: u64,
}

#[uniffi::export]
pub fn default_mobile_config() -> MobileTunnelConfig {
    MobileTunnelConfig {
        access_key: String::new(),
        domain: DEFAULT_DOMAIN.to_owned(),
        listen_port: 7000,
        keep_alive_ms: DEFAULT_KEEP_ALIVE_MS,
        resolvers: recommended_resolvers(),
    }
}

#[uniffi::export]
pub fn recommended_resolvers() -> Vec<String> {
    default_public_resolvers()
        .into_iter()
        .map(|addr| addr.to_string())
        .collect()
}

#[uniffi::export]
pub fn mobile_core_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[uniffi::export]
impl TrajectoryMobileController {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Arc::new(Mutex::new(ControllerInner {
                snapshot: MobileTunnelSnapshot {
                    state: MobileTunnelState::Idle,
                    status_text: "Disconnected".to_owned(),
                    listen_address: "127.0.0.1:7000".to_owned(),
                    active_resolvers: recommended_resolvers().len() as u32,
                    last_error: None,
                },
                logs: vec![log_entry("Mobile controller initialized")],
                stop_tx: None,
                active_run_id: None,
                next_run_id: 1,
            })),
        })
    }

    pub fn snapshot(&self) -> MobileTunnelSnapshot {
        self.inner.lock().unwrap().snapshot.clone()
    }

    pub fn logs(&self) -> Vec<MobileLogEntry> {
        self.inner.lock().unwrap().logs.clone()
    }

    pub fn clear_logs(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.logs.clear();
        inner.logs.push(log_entry("Cleared mobile diagnostics log"));
    }

    pub fn start(&self, config: MobileTunnelConfig) -> Result<(), MobileError> {
        let core_config = build_core_config(&config)?;
        let listen_address = core_config.listen.to_string();
        let resolver_count = core_config.resolvers.len() as u32;
        let (stop_tx, stop_rx) = watch::channel(false);
        let inner = self.inner.clone();

        let run_id = {
            let mut state = self.inner.lock().unwrap();
            if state.active_run_id.is_some() {
                return Err(MobileError::AlreadyRunning);
            }
            let run_id = state.next_run_id;
            state.next_run_id += 1;
            state.active_run_id = Some(run_id);
            state.stop_tx = Some(stop_tx);
            state.snapshot = MobileTunnelSnapshot {
                state: MobileTunnelState::Starting,
                status_text: "Connecting".to_owned(),
                listen_address: listen_address.clone(),
                active_resolvers: resolver_count,
                last_error: None,
            };
            state.logs.push(log_entry(&format!(
                "Starting mobile tunnel on {listen_address} with {resolver_count} resolvers"
            )));
            run_id
        };

        std::thread::spawn(move || {
            {
                let mut state = inner.lock().unwrap();
                if state.active_run_id == Some(run_id) {
                    state.snapshot.state = MobileTunnelState::Running;
                    state.snapshot.status_text = "Connected".to_owned();
                    state
                        .logs
                        .push(log_entry(&format!("Tunnel active on {listen_address}")));
                }
            }

            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    let mut state = inner.lock().unwrap();
                    transition_to_failure(
                        &mut state,
                        run_id,
                        format!("runtime init failed: {error:#}"),
                    );
                    return;
                }
            };

            let result = runtime.block_on(run_until(core_config, stop_rx));
            let mut state = inner.lock().unwrap();
            if state.active_run_id != Some(run_id) {
                return;
            }
            state.stop_tx = None;
            state.active_run_id = None;
            match result {
                Ok(()) => {
                    let was_stopping = state.snapshot.state == MobileTunnelState::Stopping;
                    state.snapshot.state = MobileTunnelState::Idle;
                    state.snapshot.status_text = if was_stopping {
                        "Disconnected".to_owned()
                    } else {
                        "Disconnected".to_owned()
                    };
                    state.snapshot.last_error = None;
                    state.logs.push(log_entry(if was_stopping {
                        "Tunnel stopped from mobile UI"
                    } else {
                        "Tunnel finished without an explicit stop request"
                    }));
                }
                Err(error) => {
                    transition_to_failure(&mut state, run_id, format!("{error:#}"));
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) -> Result<(), MobileError> {
        let stop_tx = {
            let mut state = self.inner.lock().unwrap();
            let Some(stop_tx) = state.stop_tx.clone() else {
                return Err(MobileError::NotRunning);
            };
            state.snapshot.state = MobileTunnelState::Stopping;
            state.snapshot.status_text = "Disconnecting".to_owned();
            state.logs.push(log_entry("Stopping mobile tunnel"));
            stop_tx
        };
        stop_tx
            .send(true)
            .map_err(|error| MobileError::RuntimeFailure(format!("stop signal failed: {error}")))?;
        Ok(())
    }
}

fn transition_to_failure(
    state: &mut MutexGuard<'_, ControllerInner>,
    run_id: u64,
    message: String,
) {
    if state.active_run_id == Some(run_id) {
        state.stop_tx = None;
        state.active_run_id = None;
        state.snapshot.state = MobileTunnelState::Failed;
        state.snapshot.status_text = "Tunnel failed".to_owned();
        state.snapshot.last_error = Some(message.clone());
        state.logs.push(log_entry(&format!("Error: {message}")));
    }
}

fn build_core_config(config: &MobileTunnelConfig) -> Result<ClientConfig, MobileError> {
    let domain = config.domain.trim();
    if domain.is_empty() {
        return Err(MobileError::InvalidConfiguration(
            "domain must not be empty".to_owned(),
        ));
    }
    let access_key = ClientAccessKey::parse(config.access_key.trim()).map_err(|error| {
        MobileError::InvalidConfiguration(format!("invalid access key: {error:#}"))
    })?;

    let listen: SocketAddr = format!("127.0.0.1:{}", config.listen_port)
        .parse()
        .map_err(|error| {
            MobileError::InvalidConfiguration(format!("invalid listen port: {error}"))
        })?;

    let mut resolvers = config
        .resolvers
        .iter()
        .map(|value| parse_socket_addr(value.trim(), 53))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| MobileError::InvalidConfiguration(format!("{error:#}")))?;
    if resolvers.is_empty() {
        resolvers = default_public_resolvers();
    }

    let mut core_config = default_client_config(listen, resolvers, domain.to_owned(), access_key);
    core_config.keep_alive_interval = Duration::from_millis(config.keep_alive_ms.max(20));
    Ok(core_config)
}

fn log_entry(message: &str) -> MobileLogEntry {
    MobileLogEntry {
        timestamp: unix_timestamp(),
        message: message.to_owned(),
    }
}

fn unix_timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}", duration.as_secs()),
        Err(_) => "0".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommended_resolvers_match_core_defaults() {
        let expected: Vec<_> = default_public_resolvers()
            .into_iter()
            .map(|addr| addr.to_string())
            .collect();
        assert_eq!(recommended_resolvers(), expected);
    }

    #[test]
    fn build_core_config_falls_back_to_public_defaults_when_resolvers_are_blank() {
        let config = MobileTunnelConfig {
            access_key: ClientAccessKey::generate().to_display_string(),
            domain: DEFAULT_DOMAIN.to_owned(),
            listen_port: 7000,
            keep_alive_ms: DEFAULT_KEEP_ALIVE_MS,
            resolvers: Vec::new(),
        };
        let core = build_core_config(&config).expect("mobile config should use default resolvers");
        assert_eq!(core.resolvers, default_public_resolvers());
    }
}
