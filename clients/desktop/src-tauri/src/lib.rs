use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::webview::PageLoadEvent;
use tauri::State;

const LOG_LIMIT: usize = 400;
const KEYRING_SERVICE: &str = "com.shahablavasani.trajectory";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlatformCapabilities {
    os: String,
    arch: String,
    proxy_mode: CapabilityState,
    lan_sharing: CapabilityState,
    system_proxy: CapabilityState,
    vpn_mode: CapabilityState,
    android_vpn: CapabilityState,
    notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum CapabilityState {
    Available,
    Manual,
    Planned,
    Unsupported,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeSnapshot {
    phase: ConnectionPhase,
    active_profile_id: Option<String>,
    active_profile_name: Option<String>,
    pid: Option<u32>,
    started_at: Option<String>,
    socks_endpoint: Option<String>,
    http_endpoint: Option<String>,
    binary_path: Option<String>,
    status_detail: Option<String>,
    last_error: Option<String>,
    log_lines: Vec<String>,
    capabilities: PlatformCapabilities,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum ConnectionPhase {
    Disconnected,
    Starting,
    Connected,
    Stopping,
    Failed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProxyEndpoint {
    host: String,
    port: u16,
    enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TrajectoryProfile {
    id: String,
    name: String,
    domain: String,
    #[serde(default)]
    access_key: String,
    #[serde(default)]
    access_key_saved: bool,
    resolvers: Vec<String>,
    resolver_file: Option<String>,
    resolver_socks_proxy: Option<String>,
    #[serde(default = "default_resolver_transport")]
    resolver_transport: String,
    #[serde(default = "default_transport_mode")]
    transport_mode: String,
    socks: ProxyEndpoint,
    http: ProxyEndpoint,
    dns_max_payload: u16,
    resolver_cohort_size: Option<usize>,
    resolver_admission_min: usize,
    poll_interval_ms: u64,
    allow_lan_without_auth: bool,
    admission_report: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct StoredProfile {
    id: String,
    name: String,
    domain: String,
    resolvers: Vec<String>,
    resolver_file: Option<String>,
    resolver_socks_proxy: Option<String>,
    #[serde(default = "default_resolver_transport")]
    resolver_transport: String,
    #[serde(default = "default_transport_mode")]
    transport_mode: String,
    socks: ProxyEndpoint,
    http: ProxyEndpoint,
    dns_max_payload: u16,
    resolver_cohort_size: Option<usize>,
    resolver_admission_min: usize,
    poll_interval_ms: u64,
    allow_lan_without_auth: bool,
    admission_report: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileStore {
    selected_profile_id: Option<String>,
    profiles: Vec<StoredProfile>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileStoreSnapshot {
    profiles: Vec<TrajectoryProfile>,
    selected_profile_id: Option<String>,
}

struct RuntimeState {
    child: Option<Child>,
    snapshot: RuntimeSnapshot,
}

struct AppState {
    runtime: Mutex<RuntimeState>,
    logs: Arc<Mutex<VecDeque<String>>>,
    data_dir: PathBuf,
}

#[tauri::command]
fn load_snapshot(state: State<'_, AppState>) -> Result<RuntimeSnapshot, String> {
    refresh_snapshot(&state)
}

#[tauri::command]
fn load_profiles(state: State<'_, AppState>) -> Result<ProfileStoreSnapshot, String> {
    let store = load_profile_store(&state.data_dir)?;
    profile_store_snapshot(store)
}

#[tauri::command]
fn save_profile(
    state: State<'_, AppState>,
    profile: TrajectoryProfile,
) -> Result<ProfileStoreSnapshot, String> {
    validate_profile_metadata(&profile)?;
    let mut store = load_profile_store(&state.data_dir)?;
    if !profile.access_key.trim().is_empty() {
        store_access_key(&profile.id, &profile.access_key)?;
    }

    let stored = StoredProfile::from_profile(&profile);
    if let Some(existing) = store
        .profiles
        .iter_mut()
        .find(|candidate| candidate.id == stored.id)
    {
        *existing = stored;
    } else {
        store.profiles.push(stored);
    }
    store.selected_profile_id = Some(profile.id.clone());
    save_profile_store(&state.data_dir, &store)?;
    profile_store_snapshot(store)
}

#[tauri::command]
fn delete_profile(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ProfileStoreSnapshot, String> {
    let mut store = load_profile_store(&state.data_dir)?;
    if store.profiles.len() <= 1 {
        return Err("keep at least one profile".to_string());
    }
    store.profiles.retain(|profile| profile.id != profile_id);
    let _ = delete_access_key(&profile_id);
    if !store
        .profiles
        .iter()
        .any(|profile| Some(&profile.id) == store.selected_profile_id.as_ref())
    {
        store.selected_profile_id = store.profiles.first().map(|profile| profile.id.clone());
    }
    save_profile_store(&state.data_dir, &store)?;
    profile_store_snapshot(store)
}

#[tauri::command]
fn set_selected_profile(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<ProfileStoreSnapshot, String> {
    let mut store = load_profile_store(&state.data_dir)?;
    if !store
        .profiles
        .iter()
        .any(|profile| profile.id == profile_id)
    {
        return Err("selected profile does not exist".to_string());
    }
    store.selected_profile_id = Some(profile_id);
    save_profile_store(&state.data_dir, &store)?;
    profile_store_snapshot(store)
}

#[tauri::command]
fn connect_profile(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<RuntimeSnapshot, String> {
    connect_profile_state(&state, profile_id)
}

fn connect_profile_state(
    state: &State<'_, AppState>,
    profile_id: String,
) -> Result<RuntimeSnapshot, String> {
    let profile = load_profile_with_secret(&state.data_dir, &profile_id)?;
    validate_profile_for_connect(&profile)?;
    stop_child(state)?;

    let binary = find_client_binary()?;
    let mut command = Command::new(&binary);
    command
        .arg("--listen")
        .arg("127.0.0.1:0")
        .arg("--socks-listen")
        .arg(endpoint_arg(&profile.socks))
        .arg("--domain")
        .arg(profile.domain.clone())
        .arg("--dns-max-payload")
        .arg(profile.dns_max_payload.to_string())
        .arg("--resolver-admission-min")
        .arg(profile.resolver_admission_min.to_string())
        .arg("--poll-interval-ms")
        .arg(profile.poll_interval_ms.max(1).to_string())
        .arg("--resolver-transport")
        .arg(profile.resolver_transport.clone())
        .arg("--mode")
        .arg(profile.transport_mode.clone())
        .env("TRAJECTORY_ACCESS_KEY", &profile.access_key)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if profile.http.enabled {
        command
            .arg("--http-listen")
            .arg(endpoint_arg(&profile.http));
    }

    for resolver in &profile.resolvers {
        command.arg("--resolver").arg(resolver);
    }

    if let Some(path) = profile
        .resolver_file
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        command.arg("--resolver-file").arg(path);
    }

    if let Some(proxy) = profile
        .resolver_socks_proxy
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        command.arg("--resolver-socks-proxy").arg(proxy);
    }

    if let Some(size) = profile.resolver_cohort_size {
        command.arg("--resolver-cohort-size").arg(size.to_string());
    }

    if profile.admission_report {
        command
            .arg("--admission-report")
            .arg(std::env::temp_dir().join("trajectory-desktop-admission.jsonl"));
    }

    push_log(
        &state.logs,
        format!(
            "starting {} with profile {}",
            binary.display(),
            profile.name
        ),
    );
    let mut child = command
        .spawn()
        .map_err(|error| format!("failed to start trajectory-client: {error}"))?;

    if let Some(stdout) = child.stdout.take() {
        spawn_log_reader(state.logs.clone(), "out", stdout);
    }
    if let Some(stderr) = child.stderr.take() {
        spawn_log_reader(state.logs.clone(), "err", stderr);
    }

    let pid = child.id();
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "runtime lock poisoned".to_string())?;
    runtime.child = Some(child);
    runtime.snapshot.phase = ConnectionPhase::Starting;
    runtime.snapshot.active_profile_id = Some(profile.id);
    runtime.snapshot.active_profile_name = Some(profile.name);
    runtime.snapshot.pid = Some(pid);
    runtime.snapshot.started_at = Some(now_string());
    runtime.snapshot.socks_endpoint = profile.socks.enabled.then(|| endpoint_arg(&profile.socks));
    runtime.snapshot.http_endpoint = profile.http.enabled.then(|| endpoint_arg(&profile.http));
    runtime.snapshot.binary_path = Some(binary.display().to_string());
    runtime.snapshot.status_detail =
        Some("trajectory-client spawned; waiting for listener readiness.".to_string());
    runtime.snapshot.last_error = None;
    runtime.snapshot.log_lines = collect_logs(&state.logs);
    Ok(runtime.snapshot.clone())
}

#[tauri::command]
fn disconnect_profile(state: State<'_, AppState>) -> Result<RuntimeSnapshot, String> {
    stop_child(&state)?;
    refresh_snapshot(&state)
}

#[tauri::command]
fn enable_system_proxy(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<RuntimeSnapshot, String> {
    let snapshot = refresh_snapshot(&state)?;
    if snapshot.active_profile_id.as_deref() != Some(profile_id.as_str())
        || !matches!(snapshot.phase, ConnectionPhase::Connected)
    {
        return Err(
            "wait until the profile is fully connected before applying system proxy settings"
                .to_string(),
        );
    }
    let profile = load_profile_metadata(&state.data_dir, &profile_id)?;
    validate_profile_for_system_proxy(&profile)?;
    set_system_proxy(&profile)?;
    push_log(
        &state.logs,
        "system proxy enabled for current user".to_string(),
    );
    refresh_snapshot(&state)
}

#[tauri::command]
fn disable_system_proxy(state: State<'_, AppState>) -> Result<RuntimeSnapshot, String> {
    clear_system_proxy()?;
    push_log(
        &state.logs,
        "system proxy cleared for current user".to_string(),
    );
    refresh_snapshot(&state)
}

#[tauri::command]
fn mark_frontend_ready(visible_text: Option<String>) -> Result<(), String> {
    let visible_text = visible_text.unwrap_or_default();
    if std::env::var_os("TRAJECTORY_DESKTOP_SMOKE_READY_FILE").is_some() {
        for required in [
            "Trajectory",
            "Status",
            "Proxy",
            "Resolvers",
            "Disconnected",
            "Connection Readiness",
            "Connect Apps",
            "Tunnel Target",
        ] {
            if !visible_text.contains(required) {
                return Err(format!(
                    "desktop packaged frontend smoke did not render expected text: {required}"
                ));
            }
        }
    }
    write_smoke_marker_env(
        "TRAJECTORY_DESKTOP_SMOKE_READY_FILE",
        format!(
            "frontend ready at {}\nvisible_text_len={}\n",
            now_string(),
            visible_text.len(),
        ),
    )
}

#[tauri::command]
fn mark_smoke_state_ready(state: State<'_, AppState>) -> Result<(), String> {
    let state_marker_enabled = std::env::var_os("TRAJECTORY_DESKTOP_SMOKE_STATE_FILE").is_some();
    let live_marker_enabled = std::env::var_os("TRAJECTORY_DESKTOP_SMOKE_LIVE_FILE").is_some();
    if !state_marker_enabled && !live_marker_enabled {
        return Ok(());
    }
    let profile_state = profile_store_snapshot(load_profile_store(&state.data_dir)?)?;
    if profile_state.profiles.is_empty() {
        return Err("smoke state check loaded zero profiles".to_string());
    }
    let runtime_state = if live_marker_enabled {
        let profile_id = profile_state
            .selected_profile_id
            .clone()
            .or_else(|| {
                profile_state
                    .profiles
                    .first()
                    .map(|profile| profile.id.clone())
            })
            .ok_or_else(|| "desktop live smoke has no selected profile".to_string())?;
        let _ = connect_profile_state(&state, profile_id)?;
        let runtime_state = wait_for_connected_smoke(&state, Duration::from_secs(60))?;
        let http_fetch_result = smoke_fetch_http_proxy(&runtime_state)?;
        let socks_fetch_result = smoke_fetch_socks_proxy(&runtime_state)?;
        let socks_endpoint = runtime_state.socks_endpoint.clone();
        let http_endpoint = runtime_state.http_endpoint.clone();
        stop_child(&state)?;
        let shutdown_result = wait_for_shutdown_smoke(socks_endpoint.as_deref(), http_endpoint.as_deref())?;
        write_smoke_marker_env(
            "TRAJECTORY_DESKTOP_SMOKE_LIVE_FILE",
            format!(
                "live proxy ready at {}\nhttp_endpoint={}\nsocks_endpoint={}\n{}{}\n{}\n",
                now_string(),
                runtime_state.http_endpoint.as_deref().unwrap_or(""),
                runtime_state.socks_endpoint.as_deref().unwrap_or(""),
                http_fetch_result,
                socks_fetch_result,
                shutdown_result,
            ),
        )?;
        runtime_state
    } else {
        refresh_snapshot(&state)?
    };
    if runtime_state.capabilities.os.is_empty() || runtime_state.capabilities.arch.is_empty() {
        return Err("smoke state check loaded incomplete platform capabilities".to_string());
    }

    if state_marker_enabled {
        write_smoke_marker_env(
            "TRAJECTORY_DESKTOP_SMOKE_STATE_FILE",
            format!(
                "state ready at {}\nprofiles={}\nphase={:?}\nos={}\narch={}\n",
                now_string(),
                profile_state.profiles.len(),
                runtime_state.phase,
                runtime_state.capabilities.os,
                runtime_state.capabilities.arch,
            ),
        )
    } else {
        Ok(())
    }
}

pub fn run() {
    let data_dir = app_config_dir();
    tauri::Builder::default()
        .manage(AppState {
            runtime: Mutex::new(RuntimeState {
                child: None,
                snapshot: initial_snapshot(),
            }),
            logs: Arc::new(Mutex::new(VecDeque::new())),
            data_dir,
        })
        .setup(|_| {
            write_smoke_marker_env(
                "TRAJECTORY_DESKTOP_SMOKE_BACKEND_FILE",
                format!("backend setup ready at {}\n", now_string()),
            )
            .map_err(|error| Box::<dyn std::error::Error>::from(std::io::Error::other(error)))?;
            Ok(())
        })
        .on_page_load(|_, payload| {
            if payload.event() == PageLoadEvent::Finished {
                let _ = write_smoke_marker_env(
                    "TRAJECTORY_DESKTOP_SMOKE_PAGE_FILE",
                    format!("page loaded at {}\nurl={}\n", now_string(), payload.url()),
                );
            }
        })
        .invoke_handler(tauri::generate_handler![
            load_snapshot,
            load_profiles,
            save_profile,
            delete_profile,
            set_selected_profile,
            connect_profile,
            disconnect_profile,
            enable_system_proxy,
            disable_system_proxy,
            mark_frontend_ready,
            mark_smoke_state_ready
        ])
        .run(tauri::generate_context!())
        .expect("error while running trajectory desktop");
}

fn write_smoke_marker_env(var: &str, contents: String) -> Result<(), String> {
    let Ok(path) = std::env::var(var) else {
        return Ok(());
    };
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create smoke marker directory: {error}"))?;
    }
    fs::write(path, contents).map_err(|error| format!("failed to write smoke marker: {error}"))
}

impl StoredProfile {
    fn from_profile(profile: &TrajectoryProfile) -> Self {
        Self {
            id: profile.id.clone(),
            name: profile.name.clone(),
            domain: profile.domain.clone(),
            resolvers: profile.resolvers.clone(),
            resolver_file: profile.resolver_file.clone(),
            resolver_socks_proxy: profile.resolver_socks_proxy.clone(),
            resolver_transport: profile.resolver_transport.clone(),
            transport_mode: profile.transport_mode.clone(),
            socks: profile.socks.clone(),
            http: profile.http.clone(),
            dns_max_payload: profile.dns_max_payload,
            resolver_cohort_size: profile.resolver_cohort_size,
            resolver_admission_min: profile.resolver_admission_min,
            poll_interval_ms: profile.poll_interval_ms,
            allow_lan_without_auth: profile.allow_lan_without_auth,
            admission_report: profile.admission_report,
        }
    }

    fn to_profile(&self) -> TrajectoryProfile {
        TrajectoryProfile {
            id: self.id.clone(),
            name: self.name.clone(),
            domain: self.domain.clone(),
            access_key: String::new(),
            access_key_saved: access_key_exists(&self.id),
            resolvers: self.resolvers.clone(),
            resolver_file: self.resolver_file.clone(),
            resolver_socks_proxy: self.resolver_socks_proxy.clone(),
            resolver_transport: self.resolver_transport.clone(),
            transport_mode: self.transport_mode.clone(),
            socks: self.socks.clone(),
            http: self.http.clone(),
            dns_max_payload: self.dns_max_payload,
            resolver_cohort_size: self.resolver_cohort_size,
            resolver_admission_min: self.resolver_admission_min,
            poll_interval_ms: self.poll_interval_ms,
            allow_lan_without_auth: self.allow_lan_without_auth,
            admission_report: self.admission_report,
        }
    }
}

fn initial_snapshot() -> RuntimeSnapshot {
    RuntimeSnapshot {
        phase: ConnectionPhase::Disconnected,
        active_profile_id: None,
        active_profile_name: None,
        pid: None,
        started_at: None,
        socks_endpoint: None,
        http_endpoint: None,
        binary_path: None,
        status_detail: Some("No trajectory-client process is running.".to_string()),
        last_error: None,
        log_lines: Vec::new(),
        capabilities: platform_capabilities(),
    }
}

fn platform_capabilities() -> PlatformCapabilities {
    let os = std::env::consts::OS.to_string();
    let vpn_mode = match os.as_str() {
        "linux" | "windows" | "macos" => CapabilityState::Planned,
        _ => CapabilityState::Unsupported,
    };
    PlatformCapabilities {
        os,
        arch: std::env::consts::ARCH.to_string(),
        proxy_mode: CapabilityState::Available,
        lan_sharing: CapabilityState::Manual,
        system_proxy: CapabilityState::Manual,
        vpn_mode,
        android_vpn: CapabilityState::Planned,
        notes: vec![
            "Proxy mode starts the existing optimized trajectory-client binary.".to_string(),
            "Whole-device VPN requires platform-native packet adapters.".to_string(),
        ],
    }
}

fn refresh_snapshot(state: &State<'_, AppState>) -> Result<RuntimeSnapshot, String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "runtime lock poisoned".to_string())?;
    let mut exited = None;
    if let Some(child) = runtime.child.as_mut() {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("failed to inspect trajectory-client: {error}"))?
        {
            exited = Some(status.to_string());
        }
    }

    if let Some(status) = exited {
        runtime.child = None;
        runtime.snapshot.phase = ConnectionPhase::Failed;
        runtime.snapshot.pid = None;
        runtime.snapshot.last_error = Some(format!("trajectory-client exited with {status}"));
        runtime.snapshot.status_detail =
            Some("trajectory-client exited before readiness could be proven.".to_string());
        push_log(
            &state.logs,
            format!("trajectory-client exited with {status}"),
        );
    } else if runtime.child.is_some() {
        let logs = collect_logs(&state.logs);
        if endpoints_ready(&runtime.snapshot) {
            runtime.snapshot.phase = ConnectionPhase::Connected;
            runtime.snapshot.status_detail =
                Some("Local SOCKS/HTTP listeners are accepting connections.".to_string());
        } else {
            runtime.snapshot.phase = ConnectionPhase::Starting;
            runtime.snapshot.status_detail = Some(startup_detail_from_logs(&logs));
        }
    }

    runtime.snapshot.log_lines = collect_logs(&state.logs);
    Ok(runtime.snapshot.clone())
}

fn endpoints_ready(snapshot: &RuntimeSnapshot) -> bool {
    let socks_ready = snapshot
        .socks_endpoint
        .as_deref()
        .map(endpoint_ready)
        .unwrap_or(false);
    let http_ready = snapshot
        .http_endpoint
        .as_deref()
        .map(endpoint_ready)
        .unwrap_or(true);
    socks_ready && http_ready
}

fn endpoint_ready(endpoint: &str) -> bool {
    let Ok(mut addrs) = endpoint.to_socket_addrs() else {
        return false;
    };
    addrs.any(|addr| TcpStream::connect_timeout(&addr, Duration::from_millis(180)).is_ok())
}

fn wait_for_connected_smoke(
    state: &State<'_, AppState>,
    timeout: Duration,
) -> Result<RuntimeSnapshot, String> {
    let deadline = Instant::now() + timeout;
    let mut last_snapshot = refresh_snapshot(state)?;
    while Instant::now() < deadline {
        last_snapshot = refresh_snapshot(state)?;
        match last_snapshot.phase {
            ConnectionPhase::Connected => return Ok(last_snapshot),
            ConnectionPhase::Failed => {
                return Err(format!(
                    "desktop live smoke failed before readiness: {}",
                    last_snapshot.last_error.clone().unwrap_or_default()
                ));
            }
            _ => thread::sleep(Duration::from_millis(250)),
        }
    }
    Err(format!(
        "timed out waiting for desktop live smoke readiness: {}",
        last_snapshot
            .status_detail
            .clone()
            .unwrap_or_else(|| "no status detail".to_string())
    ))
}

fn smoke_fetch_http_proxy(snapshot: &RuntimeSnapshot) -> Result<String, String> {
    let endpoint = snapshot
        .http_endpoint
        .as_deref()
        .ok_or_else(|| "desktop live smoke has no HTTP endpoint".to_string())?;
    let url = std::env::var("TRAJECTORY_DESKTOP_SMOKE_FETCH_URL")
        .unwrap_or_else(|_| "http://example.com/".to_string());
    let target = http_url_target(&url)?;
    let mut addrs = endpoint
        .to_socket_addrs()
        .map_err(|error| format!("resolve desktop smoke HTTP endpoint {endpoint}: {error}"))?;
    let addr = addrs
        .next()
        .ok_or_else(|| format!("desktop smoke HTTP endpoint has no addresses: {endpoint}"))?;
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))
        .map_err(|error| format!("connect desktop smoke HTTP proxy {endpoint}: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(25)))
        .map_err(|error| format!("set desktop smoke read timeout: {error}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .map_err(|error| format!("set desktop smoke write timeout: {error}"))?;
    let request = format!(
        "GET {url} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        target.authority
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write desktop smoke HTTP request: {error}"))?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| format!("read desktop smoke HTTP response: {error}"))?;
    let status = String::from_utf8_lossy(&response)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_string();
    let response_text = String::from_utf8_lossy(&response);
    let ok = status.starts_with("HTTP/")
        && status
            .split_whitespace()
            .nth(1)
            .and_then(|code| code.parse::<u16>().ok())
            .map(|code| (200..400).contains(&code))
            .unwrap_or(false);
    if !ok {
        return Err(format!(
            "desktop live smoke HTTP proxy returned non-success status: {status}"
        ));
    }
    let expected_body = std::env::var("TRAJECTORY_DESKTOP_SMOKE_EXPECT_BODY")
        .unwrap_or_default();
    if !expected_body.is_empty() && !response_text.contains(&expected_body) {
        return Err("desktop live smoke HTTP proxy response missed expected body marker".to_string());
    }
    Ok(format!(
        "fetch_url={url}\nstatus={status}\nbody_marker={}\n",
        if expected_body.is_empty() { "not-required" } else { "matched" }
    ))
}

fn smoke_fetch_socks_proxy(snapshot: &RuntimeSnapshot) -> Result<String, String> {
    let endpoint = snapshot
        .socks_endpoint
        .as_deref()
        .ok_or_else(|| "desktop live smoke has no SOCKS endpoint".to_string())?;
    let url = std::env::var("TRAJECTORY_DESKTOP_SMOKE_FETCH_URL")
        .unwrap_or_else(|_| "http://example.com/".to_string());
    let target = http_url_target(&url)?;
    let mut addrs = endpoint
        .to_socket_addrs()
        .map_err(|error| format!("resolve desktop smoke SOCKS endpoint {endpoint}: {error}"))?;
    let addr = addrs
        .next()
        .ok_or_else(|| format!("desktop smoke SOCKS endpoint has no addresses: {endpoint}"))?;
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))
        .map_err(|error| format!("connect desktop smoke SOCKS proxy {endpoint}: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(25)))
        .map_err(|error| format!("set desktop smoke SOCKS read timeout: {error}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(10)))
        .map_err(|error| format!("set desktop smoke SOCKS write timeout: {error}"))?;
    stream
        .write_all(&[0x05, 0x01, 0x00])
        .map_err(|error| format!("write desktop smoke SOCKS greeting: {error}"))?;
    let mut greeting = [0_u8; 2];
    stream
        .read_exact(&mut greeting)
        .map_err(|error| format!("read desktop smoke SOCKS greeting: {error}"))?;
    if greeting != [0x05, 0x00] {
        return Err(format!(
            "desktop live smoke SOCKS greeting rejected: {greeting:?}"
        ));
    }

    let host_bytes = target.host.as_bytes();
    if host_bytes.len() > u8::MAX as usize {
        return Err("desktop live smoke SOCKS host name is too long".to_string());
    }
    let mut connect = Vec::with_capacity(7 + host_bytes.len());
    connect.extend_from_slice(&[0x05, 0x01, 0x00, 0x03, host_bytes.len() as u8]);
    connect.extend_from_slice(host_bytes);
    connect.extend_from_slice(&target.port.to_be_bytes());
    stream
        .write_all(&connect)
        .map_err(|error| format!("write desktop smoke SOCKS connect: {error}"))?;
    let mut header = [0_u8; 4];
    stream
        .read_exact(&mut header)
        .map_err(|error| format!("read desktop smoke SOCKS connect header: {error}"))?;
    if header[0] != 0x05 || header[1] != 0x00 {
        return Err(format!(
            "desktop live smoke SOCKS CONNECT failed: {header:?}"
        ));
    }
    match header[3] {
        0x01 => {
            let mut rest = [0_u8; 6];
            stream
                .read_exact(&mut rest)
                .map_err(|error| format!("read desktop smoke SOCKS IPv4 bind: {error}"))?;
        }
        0x03 => {
            let mut len = [0_u8; 1];
            stream
                .read_exact(&mut len)
                .map_err(|error| format!("read desktop smoke SOCKS domain bind length: {error}"))?;
            let mut rest = vec![0_u8; len[0] as usize + 2];
            stream
                .read_exact(&mut rest)
                .map_err(|error| format!("read desktop smoke SOCKS domain bind: {error}"))?;
        }
        0x04 => {
            let mut rest = [0_u8; 18];
            stream
                .read_exact(&mut rest)
                .map_err(|error| format!("read desktop smoke SOCKS IPv6 bind: {error}"))?;
        }
        atyp => {
            return Err(format!("desktop live smoke SOCKS CONNECT returned unknown ATYP {atyp}"));
        }
    }

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        target.path, target.authority
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write desktop smoke SOCKS HTTP request: {error}"))?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| format!("read desktop smoke SOCKS HTTP response: {error}"))?;
    let status = String::from_utf8_lossy(&response)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_string();
    let response_text = String::from_utf8_lossy(&response);
    let ok = status.starts_with("HTTP/")
        && status
            .split_whitespace()
            .nth(1)
            .and_then(|code| code.parse::<u16>().ok())
            .map(|code| (200..400).contains(&code))
            .unwrap_or(false);
    if !ok {
        return Err(format!(
            "desktop live smoke SOCKS fetch returned non-success status: {status}"
        ));
    }
    let expected_body = std::env::var("TRAJECTORY_DESKTOP_SMOKE_EXPECT_BODY")
        .unwrap_or_default();
    if !expected_body.is_empty() && !response_text.contains(&expected_body) {
        return Err("desktop live smoke SOCKS response missed expected body marker".to_string());
    }
    Ok(format!(
        "socks_fetch_url={url}\nsocks_status={status}\nsocks_body_marker={}\n",
        if expected_body.is_empty() { "not-required" } else { "matched" }
    ))
}

fn wait_for_shutdown_smoke(
    socks_endpoint: Option<&str>,
    http_endpoint: Option<&str>,
) -> Result<String, String> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        let socks_closed = socks_endpoint.map(|endpoint| !endpoint_ready(endpoint)).unwrap_or(true);
        let http_closed = http_endpoint.map(|endpoint| !endpoint_ready(endpoint)).unwrap_or(true);
        if socks_closed && http_closed {
            return Ok("shutdown=clean".to_string());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err(format!(
        "desktop live smoke did not prove listener shutdown: socks_closed={} http_closed={}",
        socks_endpoint.map(|endpoint| !endpoint_ready(endpoint)).unwrap_or(true),
        http_endpoint.map(|endpoint| !endpoint_ready(endpoint)).unwrap_or(true),
    ))
}

struct HttpSmokeTarget {
    authority: String,
    host: String,
    port: u16,
    path: String,
}

fn http_url_target(url: &str) -> Result<HttpSmokeTarget, String> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| "desktop live smoke fetch URL must be plain http://".to_string())?;
    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, format!("/{path}")),
        None => (rest, "/".to_string()),
    };
    if authority.is_empty() {
        return Err("desktop live smoke fetch URL is missing host".to_string());
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() => {
            let port = port
                .parse::<u16>()
                .map_err(|error| format!("desktop live smoke URL has invalid port: {error}"))?;
            (host.to_string(), port)
        }
        _ => (authority.to_string(), 80),
    };
    Ok(HttpSmokeTarget {
        authority: authority.to_string(),
        host,
        port,
        path,
    })
}

fn startup_detail_from_logs(logs: &[String]) -> String {
    for line in logs.iter().rev() {
        let lower = line.to_ascii_lowercase();
        if lower.contains("probing ") && lower.contains(" resolver") {
            return "Checking resolver admission before exposing local listeners.".to_string();
        }
        if lower.contains("using ") && lower.contains(" admitted resolver") {
            return "Resolver admission passed; waiting for local listeners.".to_string();
        }
        if lower.contains("trajectory http proxy listening") {
            return "HTTP listener announced; verifying loopback readiness.".to_string();
        }
        if lower.contains("trajectory socks proxy listening") {
            return "SOCKS listener announced; waiting for HTTP readiness.".to_string();
        }
        if lower.contains("failed") || lower.contains("timed out") {
            return "Resolver path errors observed while starting; still waiting for readiness."
                .to_string();
        }
    }
    "Process started; waiting for local proxy listeners.".to_string()
}

fn stop_child(state: &State<'_, AppState>) -> Result<(), String> {
    let mut runtime = state
        .runtime
        .lock()
        .map_err(|_| "runtime lock poisoned".to_string())?;
    runtime.snapshot.phase = ConnectionPhase::Stopping;
    if let Some(mut child) = runtime.child.take() {
        push_log(&state.logs, "stopping trajectory-client".to_string());
        let _ = child.kill();
        let _ = child.wait();
    }
    runtime.snapshot = RuntimeSnapshot {
        capabilities: runtime.snapshot.capabilities.clone(),
        log_lines: collect_logs(&state.logs),
        ..initial_snapshot()
    };
    Ok(())
}

fn default_profile() -> StoredProfile {
    StoredProfile {
        id: "default".to_string(),
        name: "Local proxy".to_string(),
        domain: String::new(),
        resolvers: vec![
            "1.1.1.1:53".to_string(),
            "1.0.0.1:53".to_string(),
            "8.8.8.8:53".to_string(),
            "8.8.4.4:53".to_string(),
        ],
        resolver_file: None,
        resolver_socks_proxy: None,
        resolver_transport: default_resolver_transport(),
        transport_mode: default_transport_mode(),
        socks: ProxyEndpoint {
            host: "127.0.0.1".to_string(),
            port: 7000,
            enabled: true,
        },
        http: ProxyEndpoint {
            host: "127.0.0.1".to_string(),
            port: 7001,
            enabled: true,
        },
        dns_max_payload: 1232,
        resolver_cohort_size: None,
        resolver_admission_min: 1,
        poll_interval_ms: 25,
        allow_lan_without_auth: false,
        admission_report: true,
    }
}

fn default_transport_mode() -> String {
    "secure".to_string()
}

fn default_resolver_transport() -> String {
    "auto".to_string()
}

fn profile_store_snapshot(mut store: ProfileStore) -> Result<ProfileStoreSnapshot, String> {
    if store.profiles.is_empty() {
        store.profiles.push(default_profile());
        store.selected_profile_id = Some("default".to_string());
    }
    if !store
        .profiles
        .iter()
        .any(|profile| Some(&profile.id) == store.selected_profile_id.as_ref())
    {
        store.selected_profile_id = store.profiles.first().map(|profile| profile.id.clone());
    }
    Ok(ProfileStoreSnapshot {
        profiles: store
            .profiles
            .iter()
            .map(StoredProfile::to_profile)
            .collect(),
        selected_profile_id: store.selected_profile_id,
    })
}

fn load_profile_store(data_dir: &Path) -> Result<ProfileStore, String> {
    let path = profile_store_path(data_dir);
    if !path.exists() {
        return Ok(ProfileStore {
            selected_profile_id: Some("default".to_string()),
            profiles: vec![default_profile()],
        });
    }
    let contents =
        fs::read_to_string(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    serde_json::from_str(&contents).map_err(|error| format!("parse {}: {error}", path.display()))
}

fn save_profile_store(data_dir: &Path, store: &ProfileStore) -> Result<(), String> {
    fs::create_dir_all(data_dir)
        .map_err(|error| format!("create {}: {error}", data_dir.display()))?;
    let path = profile_store_path(data_dir);
    let contents = serde_json::to_string_pretty(store)
        .map_err(|error| format!("serialize profile store: {error}"))?;
    let tmp_path = path.with_extension("json.tmp");
    write_private_file(&tmp_path, contents.as_bytes())?;
    fs::rename(&tmp_path, &path).map_err(|error| {
        let _ = fs::remove_file(&tmp_path);
        format!("replace {}: {error}", path.display())
    })
}

fn profile_store_path(data_dir: &Path) -> PathBuf {
    data_dir.join("profiles.json")
}

fn load_profile_with_secret(
    data_dir: &Path,
    profile_id: &str,
) -> Result<TrajectoryProfile, String> {
    let mut profile = load_profile_metadata(data_dir, profile_id)?;
    profile.access_key = load_access_key(profile_id)?;
    profile.access_key_saved = true;
    Ok(profile)
}

fn load_profile_metadata(data_dir: &Path, profile_id: &str) -> Result<TrajectoryProfile, String> {
    let store = load_profile_store(data_dir)?;
    store
        .profiles
        .iter()
        .find(|profile| profile.id == profile_id)
        .map(StoredProfile::to_profile)
        .ok_or_else(|| "profile does not exist".to_string())
}

fn write_private_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut options = OpenOptions::new();
    options.create(true).truncate(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|error| format!("open {}: {error}", path.display()))?;
    use std::io::Write;
    file.write_all(bytes)
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    file.sync_all()
        .map_err(|error| format!("sync {}: {error}", path.display()))
}

fn store_access_key(profile_id: &str, access_key: &str) -> Result<(), String> {
    if std::env::var_os("TRAJECTORY_DESKTOP_SMOKE_ACCESS_KEY").is_some() {
        return Ok(());
    }
    Entry::new(KEYRING_SERVICE, profile_id)
        .map_err(|error| format!("open OS credential store: {error}"))?
        .set_password(access_key)
        .map_err(|error| format!("store access key in OS credential store: {error}"))
}

fn load_access_key(profile_id: &str) -> Result<String, String> {
    if let Ok(access_key) = std::env::var("TRAJECTORY_DESKTOP_SMOKE_ACCESS_KEY") {
        if !access_key.trim().is_empty() {
            return Ok(access_key);
        }
    }
    Entry::new(KEYRING_SERVICE, profile_id)
        .map_err(|error| format!("open OS credential store: {error}"))?
        .get_password()
        .map_err(|error| format!("read access key from OS credential store: {error}"))
}

fn delete_access_key(profile_id: &str) -> Result<(), String> {
    Entry::new(KEYRING_SERVICE, profile_id)
        .map_err(|error| format!("open OS credential store: {error}"))?
        .delete_credential()
        .map_err(|error| format!("delete access key from OS credential store: {error}"))
}

fn access_key_exists(profile_id: &str) -> bool {
    if std::env::var_os("TRAJECTORY_DESKTOP_SMOKE").is_some() {
        return std::env::var_os("TRAJECTORY_DESKTOP_SMOKE_ACCESS_KEY").is_some();
    }
    Entry::new(KEYRING_SERVICE, profile_id)
        .and_then(|entry| entry.get_password().map(|_| ()))
        .is_ok()
}

fn validate_profile_metadata(profile: &TrajectoryProfile) -> Result<(), String> {
    if profile.name.trim().is_empty() {
        return Err("profile name is required".to_string());
    }
    if profile.socks.host.trim().is_empty() || profile.http.host.trim().is_empty() {
        return Err("listener hosts are required".to_string());
    }
    validate_profile_common(profile)
}

fn validate_profile_for_connect(profile: &TrajectoryProfile) -> Result<(), String> {
    if profile.domain.trim().is_empty() {
        return Err("profile domain is required".to_string());
    }
    if profile.access_key.trim().is_empty() {
        return Err("access key is required".to_string());
    }
    validate_profile_common(profile)
}

fn validate_profile_for_system_proxy(profile: &TrajectoryProfile) -> Result<(), String> {
    validate_profile_common(profile)?;
    if !profile.http.enabled {
        return Err(
            "HTTP proxy listener must be enabled before applying system proxy settings".to_string(),
        );
    }
    if profile.http.host != "127.0.0.1" && profile.http.host != "::1" {
        return Err("system proxy requires a localhost HTTP listener".to_string());
    }
    Ok(())
}

fn validate_profile_common(profile: &TrajectoryProfile) -> Result<(), String> {
    if !profile.socks.enabled {
        return Err("SOCKS5 listener is required by the current client runtime".to_string());
    }
    if profile.resolvers.is_empty()
        && profile
            .resolver_file
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty()
    {
        return Err("add resolvers or a resolver file".to_string());
    }
    if !matches!(
        profile.transport_mode.as_str(),
        "secure" | "velocity" | "resilient" | "frontier"
    ) {
        return Err("transport mode must be secure, velocity, resilient, or frontier".to_string());
    }
    if !matches!(profile.resolver_transport.as_str(), "auto" | "udp" | "tcp") {
        return Err("resolver transport must be auto, udp, or tcp".to_string());
    }
    for endpoint in [&profile.socks, &profile.http] {
        if endpoint.enabled
            && endpoint.host != "127.0.0.1"
            && endpoint.host != "::1"
            && !profile.allow_lan_without_auth
        {
            return Err("LAN binding requires explicit confirmation".to_string());
        }
    }
    Ok(())
}

fn endpoint_arg(endpoint: &ProxyEndpoint) -> String {
    format!("{}:{}", endpoint.host, endpoint.port)
}

fn find_client_binary() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("TRAJECTORY_CLIENT_BIN") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }
        return Err(format!(
            "TRAJECTORY_CLIENT_BIN points to missing file: {}",
            path.display()
        ));
    }

    let binary = if cfg!(windows) {
        "trajectory-client.exe"
    } else {
        "trajectory-client"
    };

    let mut candidates = Vec::new();
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            candidates.push(dir.join(binary));
            candidates.push(dir.join("resources").join(binary));
            candidates.extend(find_prefixed_files(dir, "trajectory-client"));
            candidates.extend(find_prefixed_files(
                &dir.join("resources"),
                "trajectory-client",
            ));
        }
    }

    if std::env::var_os("TRAJECTORY_DESKTOP_SMOKE").is_none() {
        let current_dir = std::env::current_dir().map_err(|error| error.to_string())?;
        for ancestor in current_dir.ancestors() {
            candidates.push(ancestor.join("target").join("release").join(binary));
            candidates.push(ancestor.join("target").join("debug").join(binary));
        }
    }

    candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            "trajectory-client binary not found; build it first or set TRAJECTORY_CLIENT_BIN"
                .to_string()
        })
}

fn find_prefixed_files(dir: &Path, prefix: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.starts_with(prefix))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn set_system_proxy(profile: &TrajectoryProfile) -> Result<(), String> {
    let socks = endpoint_arg(&profile.socks);
    let http = endpoint_arg(&profile.http);
    match std::env::consts::OS {
        "macos" => set_macos_proxy(&profile.socks, &profile.http),
        "windows" => run_command(
            "reg",
            &[
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
                "/v",
                "ProxyEnable",
                "/t",
                "REG_DWORD",
                "/d",
                "1",
                "/f",
            ],
        )
        .and_then(|_| {
            run_command(
                "reg",
                &[
                    "add",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
                    "/v",
                    "ProxyServer",
                    "/d",
                    &format!("http={http};https={http};socks={socks}"),
                    "/f",
                ],
            )
        }),
        "linux" => set_gnome_proxy(&profile.socks, &profile.http),
        other => Err(format!("system proxy is not supported on {other}")),
    }
}

fn clear_system_proxy() -> Result<(), String> {
    match std::env::consts::OS {
        "macos" => clear_macos_proxy(),
        "windows" => run_command(
            "reg",
            &[
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
                "/v",
                "ProxyEnable",
                "/t",
                "REG_DWORD",
                "/d",
                "0",
                "/f",
            ],
        ),
        "linux" => run_command(
            "gsettings",
            &["set", "org.gnome.system.proxy", "mode", "none"],
        ),
        other => Err(format!("system proxy is not supported on {other}")),
    }
}

fn set_macos_proxy(socks: &ProxyEndpoint, http: &ProxyEndpoint) -> Result<(), String> {
    let services = macos_network_services()?;
    for service in services {
        run_command(
            "networksetup",
            &["-setwebproxy", &service, &http.host, &http.port.to_string()],
        )?;
        run_command(
            "networksetup",
            &[
                "-setsecurewebproxy",
                &service,
                &http.host,
                &http.port.to_string(),
            ],
        )?;
        run_command(
            "networksetup",
            &[
                "-setsocksfirewallproxy",
                &service,
                &socks.host,
                &socks.port.to_string(),
            ],
        )?;
    }
    Ok(())
}

fn clear_macos_proxy() -> Result<(), String> {
    let services = macos_network_services()?;
    for service in services {
        run_command("networksetup", &["-setwebproxystate", &service, "off"])?;
        run_command(
            "networksetup",
            &["-setsecurewebproxystate", &service, "off"],
        )?;
        run_command(
            "networksetup",
            &["-setsocksfirewallproxystate", &service, "off"],
        )?;
    }
    Ok(())
}

fn macos_network_services() -> Result<Vec<String>, String> {
    let output = Command::new("networksetup")
        .arg("-listallnetworkservices")
        .output()
        .map_err(|error| format!("run networksetup: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty() && !line.starts_with('*'))
        .map(|line| line.to_string())
        .collect())
}

fn set_gnome_proxy(socks: &ProxyEndpoint, http: &ProxyEndpoint) -> Result<(), String> {
    run_command(
        "gsettings",
        &["set", "org.gnome.system.proxy", "mode", "manual"],
    )?;
    run_command(
        "gsettings",
        &["set", "org.gnome.system.proxy.http", "host", &http.host],
    )?;
    run_command(
        "gsettings",
        &[
            "set",
            "org.gnome.system.proxy.http",
            "port",
            &http.port.to_string(),
        ],
    )?;
    run_command(
        "gsettings",
        &["set", "org.gnome.system.proxy.https", "host", &http.host],
    )?;
    run_command(
        "gsettings",
        &[
            "set",
            "org.gnome.system.proxy.https",
            "port",
            &http.port.to_string(),
        ],
    )?;
    run_command(
        "gsettings",
        &["set", "org.gnome.system.proxy.socks", "host", &socks.host],
    )?;
    run_command(
        "gsettings",
        &[
            "set",
            "org.gnome.system.proxy.socks",
            "port",
            &socks.port.to_string(),
        ],
    )
}

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| format!("run {program}: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("{program} failed: {}", stderr.trim()))
    }
}

fn spawn_log_reader<R>(logs: Arc<Mutex<VecDeque<String>>>, label: &'static str, reader: R)
where
    R: std::io::Read + Send + 'static,
{
    thread::spawn(move || {
        let reader = BufReader::new(reader);
        for line in reader.lines() {
            match line {
                Ok(line) => push_log(&logs, format!("{label}: {}", redact_secrets(&line))),
                Err(error) => {
                    push_log(&logs, format!("{label}: log read failed: {error}"));
                    break;
                }
            }
        }
    });
}

fn push_log(logs: &Arc<Mutex<VecDeque<String>>>, line: String) {
    if let Ok(mut logs) = logs.lock() {
        logs.push_back(redact_secrets(&line));
        while logs.len() > LOG_LIMIT {
            logs.pop_front();
        }
    }
}

fn collect_logs(logs: &Arc<Mutex<VecDeque<String>>>) -> Vec<String> {
    logs.lock()
        .map(|logs| logs.iter().cloned().collect())
        .unwrap_or_default()
}

fn redact_secrets(line: &str) -> String {
    line.split_whitespace()
        .map(|word| {
            if word.starts_with("traj1_") || word.contains("TRAJECTORY_ACCESS_KEY") {
                "[redacted]"
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn app_config_dir() -> PathBuf {
    if let Some(path) = std::env::var_os("TRAJECTORY_DESKTOP_CONFIG_DIR") {
        return PathBuf::from(path);
    }
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    match std::env::consts::OS {
        "macos" => home
            .join("Library")
            .join("Application Support")
            .join("Trajectory"),
        "windows" => std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or(home)
            .join("Trajectory"),
        _ => std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"))
            .join("trajectory"),
    }
}

fn now_string() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    format!("{seconds}")
}
