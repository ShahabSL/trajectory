use anyhow::{Context as _, Result};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime};
use tokio::sync::watch;
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::client::{
    default_client_config, default_public_resolvers, parse_socket_addr, ClientConfig,
};

fn main() -> eframe::Result {
    if std::env::args().any(|arg| arg == "--smoke-test") {
        let _ = TrajectoryDesktopApp::default();
        return Ok(());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1220.0, 820.0])
            .with_min_inner_size([980.0, 700.0])
            .with_icon(trajectory_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Trajectory Desktop",
        options,
        Box::new(|cc| {
            configure_theme(&cc.egui_ctx);
            Ok(Box::new(TrajectoryDesktopApp::default()))
        }),
    )
}

fn trajectory_icon() -> egui::IconData {
    const SCALE: usize = 8;
    const PATTERN: &[&str] = &[
        "................",
        "................",
        "..TTTTTTTTTTTT..",
        "..TT........TT..",
        "..TT........TT..",
        "......TTTT......",
        "......TTTT......",
        "......TTTT......",
        "......TTTT......",
        "......TTTT......",
        "......TTTT......",
        "......TTTT......",
        "......TTTT......",
        "................",
        "................",
        "................",
        "................",
        "................",
    ];

    let width = (PATTERN[0].len() * SCALE) as u32;
    let height = (PATTERN.len() * SCALE) as u32;
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);

    for row in PATTERN {
        for _ in 0..SCALE {
            for ch in row.chars() {
                let pixel = match ch {
                    'T' => [255, 255, 255, 255],
                    _ => [0, 0, 0, 255],
                };
                for _ in 0..SCALE {
                    rgba.extend_from_slice(&pixel);
                }
            }
        }
    }

    egui::IconData {
        rgba,
        width,
        height,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunState {
    Stopped,
    Starting,
    Running,
    Error,
}

#[derive(Debug)]
enum UiEvent {
    Started(SocketAddr),
    Stopped,
    Failed(String),
}

struct TunnelHandle {
    stop_tx: watch::Sender<bool>,
    join: thread::JoinHandle<()>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DesktopSettings {
    access_key: String,
    domain: String,
    resolvers_input: String,
    listen_port: String,
    keep_alive_ms: String,
}

impl Default for DesktopSettings {
    fn default() -> Self {
        Self {
            access_key: String::new(),
            domain: "your.domain.example".to_owned(),
            resolvers_input: default_resolvers_text(),
            listen_port: "7000".to_owned(),
            keep_alive_ms: "50".to_owned(),
        }
    }
}

struct TrajectoryDesktopApp {
    access_key: String,
    domain: String,
    resolvers_input: String,
    listen_port: String,
    keep_alive_ms: String,
    run_state: RunState,
    status_line: String,
    logs: Vec<String>,
    event_tx: Sender<UiEvent>,
    event_rx: Receiver<UiEvent>,
    tunnel: Option<TunnelHandle>,
    last_persisted_settings: DesktopSettings,
    last_save_attempt: Option<DesktopSettings>,
}

impl Default for TrajectoryDesktopApp {
    fn default() -> Self {
        Self::from_settings(load_desktop_settings().unwrap_or_else(|_| DesktopSettings::default()))
    }
}

impl TrajectoryDesktopApp {
    fn from_settings(settings: DesktopSettings) -> Self {
        let (event_tx, event_rx) = mpsc::channel();
        Self {
            access_key: settings.access_key.clone(),
            domain: settings.domain.clone(),
            resolvers_input: settings.resolvers_input.clone(),
            listen_port: settings.listen_port.clone(),
            keep_alive_ms: settings.keep_alive_ms.clone(),
            run_state: RunState::Stopped,
            status_line: "Ready".to_owned(),
            logs: vec![timestamped("Desktop client initialized")],
            event_tx,
            event_rx,
            tunnel: None,
            last_persisted_settings: settings,
            last_save_attempt: None,
        }
    }
}

impl eframe::App for TrajectoryDesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain_events();
        self.persist_settings_if_changed();
        ctx.request_repaint_after(Duration::from_millis(100));

        egui::TopBottomPanel::top("hero").show(ctx, |ui| {
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.heading(
                        egui::RichText::new("Trajectory Desktop")
                            .size(30.0)
                            .color(rgb(255, 255, 255)),
                    );
                    ui.label(
                        egui::RichText::new("Connect with your access key and start browsing.")
                            .color(rgb(176, 176, 176))
                            .size(15.0),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    status_pill(ui, self.run_state, &self.status_line);
                });
            });
            ui.add_space(8.0);
        });

        egui::SidePanel::left("config")
            .resizable(false)
            .default_width(360.0)
            .show(ctx, |ui| {
                panel_card(ui, "Connection", |ui| {
                    field_label(ui, "Authoritative domain");
                    ui.text_edit_singleline(&mut self.domain);
                    ui.add_space(10.0);

                    field_label(ui, "Access key");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.access_key)
                            .password(true)
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(10.0);

                    field_label(ui, "Local TCP listen port");
                    ui.text_edit_singleline(&mut self.listen_port);
                    ui.add_space(10.0);

                    field_label(ui, "Keep-alive interval (ms)");
                    ui.text_edit_singleline(&mut self.keep_alive_ms);
                    ui.add_space(10.0);

                    field_label(ui, "Recursive resolvers");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.resolvers_input)
                            .desired_rows(7)
                            .code_editor(),
                    );
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        let start = egui::Button::new(
                            egui::RichText::new("Start tunnel")
                                .color(rgb(0, 0, 0))
                                .strong(),
                        )
                        .fill(rgb(255, 255, 255))
                        .min_size([140.0, 42.0].into());
                        if ui
                            .add_enabled(self.run_state == RunState::Stopped, start)
                            .clicked()
                        {
                            if let Err(error) = self.start_tunnel() {
                                self.fail(error);
                            }
                        }

                        let stop = egui::Button::new(
                            egui::RichText::new("Stop")
                                .color(rgb(255, 255, 255))
                                .strong(),
                        )
                        .fill(rgb(38, 38, 38))
                        .min_size([100.0, 42.0].into());
                        if ui
                            .add_enabled(self.run_state == RunState::Running, stop)
                            .clicked()
                        {
                            self.stop_tunnel();
                        }
                    });
                });

                ui.add_space(14.0);
                panel_card(ui, "Connection", |ui| {
                    ui.label(instruction("Access key: required"));
                    ui.label(instruction("Server: your.domain.example"));
                    ui.label(instruction("Status and activity appear on the right."));
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                panel_card(&mut columns[0], "Live status", |ui| {
                    stat_row(ui, "Status", run_state_label(self.run_state));
                    stat_row(ui, "Listen", &format!("127.0.0.1:{}", self.listen_port));
                    stat_row(ui, "Domain", &self.domain);
                    stat_row(
                        ui,
                        "Access key",
                        if self.access_key.trim().is_empty() {
                            "Not set"
                        } else {
                            "Configured"
                        },
                    );
                });

                panel_card(&mut columns[1], "Diagnostics", |ui| {
                    stat_row(
                        ui,
                        "Resolver count",
                        &self.active_resolver_count().to_string(),
                    );
                    stat_row(ui, "Keep-alive", &format!("{} ms", self.keep_alive_ms));
                    stat_row(ui, "State", run_state_label(self.run_state));
                });
            });

            ui.add_space(14.0);
            panel_card(ui, "Event log", |ui| {
                egui::ScrollArea::vertical()
                    .max_height(420.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for entry in &self.logs {
                            ui.label(
                                egui::RichText::new(entry)
                                    .family(egui::FontFamily::Monospace)
                                    .color(rgb(210, 210, 210)),
                            );
                            ui.add_space(4.0);
                        }
                    });
            });
        });
    }
}

impl TrajectoryDesktopApp {
    fn current_settings(&self) -> DesktopSettings {
        DesktopSettings {
            access_key: self.access_key.clone(),
            domain: self.domain.clone(),
            resolvers_input: self.resolvers_input.clone(),
            listen_port: self.listen_port.clone(),
            keep_alive_ms: self.keep_alive_ms.clone(),
        }
    }

    fn persist_settings_if_changed(&mut self) {
        let settings = self.current_settings();
        if settings == self.last_persisted_settings
            || self.last_save_attempt.as_ref() == Some(&settings)
        {
            return;
        }

        self.last_save_attempt = Some(settings.clone());
        match save_desktop_settings(&settings) {
            Ok(()) => {
                self.last_persisted_settings = settings;
            }
            Err(error) => {
                self.logs
                    .push(timestamped(&format!("Settings save failed: {error:#}")));
            }
        }
    }

    fn start_tunnel(&mut self) -> Result<()> {
        let config = self.parse_config()?;
        let (stop_tx, stop_rx) = watch::channel(false);
        let tx = self.event_tx.clone();
        let listen = config.listen;

        self.run_state = RunState::Starting;
        self.status_line = format!("Starting tunnel on {listen}");
        self.logs.push(timestamped("Starting desktop tunnel"));

        let join = thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    let _ = tx.send(UiEvent::Failed(format!("runtime init failed: {error:#}")));
                    return;
                }
            };

            let _ = tx.send(UiEvent::Started(listen));
            let result = runtime.block_on(trajectory_core::client::run_until(config, stop_rx));
            match result {
                Ok(()) => {
                    let _ = tx.send(UiEvent::Stopped);
                }
                Err(error) => {
                    let _ = tx.send(UiEvent::Failed(format!("{error:#}")));
                }
            }
        });

        self.tunnel = Some(TunnelHandle { stop_tx, join });
        Ok(())
    }

    fn stop_tunnel(&mut self) {
        if let Some(handle) = self.tunnel.take() {
            let _ = handle.stop_tx.send(true);
            let _ = handle.join.join();
        }
        self.run_state = RunState::Stopped;
        self.status_line = "Tunnel stopped.".to_owned();
        self.logs.push(timestamped("Stopped desktop tunnel"));
    }

    fn parse_config(&self) -> Result<ClientConfig> {
        let port: u16 = self
            .listen_port
            .trim()
            .parse()
            .context("invalid local listen port")?;
        let keep_alive_ms: u64 = self
            .keep_alive_ms
            .trim()
            .parse()
            .context("invalid keep-alive interval")?;
        let mut resolvers = self
            .resolvers_input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|value| parse_socket_addr(value, 53))
            .collect::<Result<Vec<_>>>()?;
        if resolvers.is_empty() {
            resolvers = default_public_resolvers();
        }
        let access_key =
            ClientAccessKey::parse(self.access_key.trim()).context("invalid access key")?;
        let listen = format!("127.0.0.1:{port}")
            .parse()
            .context("invalid local listen address")?;

        let mut config =
            default_client_config(listen, resolvers, self.domain.trim().to_owned(), access_key);
        config.keep_alive_interval = Duration::from_millis(keep_alive_ms);
        Ok(config)
    }

    fn drain_events(&mut self) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                UiEvent::Started(addr) => {
                    self.run_state = RunState::Running;
                    self.status_line = format!("Tunnel listening on {addr}");
                    self.logs
                        .push(timestamped(&format!("Tunnel active on {addr}")));
                }
                UiEvent::Stopped => {
                    self.run_state = RunState::Stopped;
                    self.status_line = "Tunnel stopped.".to_owned();
                    self.logs.push(timestamped("Tunnel stopped"));
                }
                UiEvent::Failed(error) => {
                    self.run_state = RunState::Error;
                    self.status_line = "Tunnel failed".to_owned();
                    self.logs.push(timestamped(&format!("Error: {error}")));
                }
            }
        }
    }

    fn fail(&mut self, error: anyhow::Error) {
        self.run_state = RunState::Error;
        self.status_line = "Configuration error".to_owned();
        self.logs.push(timestamped(&format!("Error: {error:#}")));
    }

    fn active_resolver_count(&self) -> usize {
        self.resolvers_input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .count()
    }
}

fn configure_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(14.0, 10.0);
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = rgb(0, 0, 0);
    style.visuals.extreme_bg_color = rgb(6, 6, 6);
    style.visuals.window_fill = rgb(14, 14, 14);
    style.visuals.widgets.inactive.bg_fill = rgb(24, 24, 24);
    style.visuals.widgets.hovered.bg_fill = rgb(38, 38, 38);
    style.visuals.widgets.active.bg_fill = rgb(54, 54, 54);
    style.visuals.override_text_color = Some(rgb(255, 255, 255));
    style.visuals.widgets.noninteractive.fg_stroke.color = rgb(255, 255, 255);
    style.visuals.window_stroke = egui::Stroke::new(1.0, rgb(54, 54, 54));
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(30.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(14.0, egui::FontFamily::Monospace),
    );
    ctx.set_style(style);
}

fn panel_card(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(rgb(16, 16, 16))
        .stroke(egui::Stroke::new(1.0, rgb(54, 54, 54)))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .strong()
                    .size(18.0)
                    .color(rgb(255, 255, 255)),
            );
            ui.add_space(12.0);
            add_contents(ui);
        });
}

fn status_pill(ui: &mut egui::Ui, state: RunState, label: &str) {
    let fill = match state {
        RunState::Stopped => rgb(36, 36, 36),
        RunState::Starting => rgb(92, 92, 92),
        RunState::Running => rgb(220, 220, 220),
        RunState::Error => rgb(78, 78, 78),
    };
    egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, rgb(84, 84, 84)))
        .corner_radius(24.0)
        .inner_margin(egui::Margin::symmetric(14, 8))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(label)
                    .strong()
                    .color(if state == RunState::Running {
                        rgb(0, 0, 0)
                    } else {
                        rgb(255, 255, 255)
                    }),
            );
        });
}

fn stat_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(rgb(150, 150, 150)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .strong()
                    .color(rgb(255, 255, 255)),
            );
        });
    });
    ui.add_space(6.0);
}

fn field_label(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .strong()
            .size(13.0)
            .color(rgb(192, 192, 192)),
    );
}

fn instruction(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(rgb(196, 196, 196))
        .size(14.0)
}

fn run_state_label(state: RunState) -> &'static str {
    match state {
        RunState::Stopped => "Stopped",
        RunState::Starting => "Starting",
        RunState::Running => "Running",
        RunState::Error => "Error",
    }
}

fn timestamped(message: &str) -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("[{now}] {message}")
}

fn rgb(r: u8, g: u8, b: u8) -> egui::Color32 {
    egui::Color32::from_rgb(r, g, b)
}

fn default_resolvers_text() -> String {
    default_public_resolvers()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

fn load_desktop_settings() -> Result<DesktopSettings> {
    let path = desktop_settings_path().context("desktop settings path unavailable")?;
    let raw = fs::read_to_string(path)?;
    let settings = serde_json::from_str(&raw)?;
    Ok(settings)
}

fn save_desktop_settings(settings: &DesktopSettings) -> Result<()> {
    let path = desktop_settings_path().context("desktop settings path unavailable")?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let raw = serde_json::to_string_pretty(settings)?;
    fs::write(path, raw)?;
    Ok(())
}

fn desktop_settings_path() -> Option<PathBuf> {
    if let Some(value) = std::env::var_os("TRAJECTORY_DESKTOP_CONFIG") {
        return Some(PathBuf::from(value));
    }

    #[cfg(target_os = "windows")]
    {
        return std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .map(|path| path.join("Trajectory").join("desktop-settings.json"));
    }

    #[cfg(target_os = "macos")]
    {
        return std::env::var_os("HOME").map(PathBuf::from).map(|path| {
            path.join("Library")
                .join("Application Support")
                .join("Trajectory")
                .join("desktop-settings.json")
        });
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Some(path) = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
            return Some(path.join("trajectory").join("desktop-settings.json"));
        }
        return std::env::var_os("HOME").map(PathBuf::from).map(|path| {
            path.join(".config")
                .join("trajectory")
                .join("desktop-settings.json")
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_mode_builds_app_state() {
        let app = TrajectoryDesktopApp::from_settings(DesktopSettings::default());
        assert_eq!(app.run_state, RunState::Stopped);
        assert!(app.active_resolver_count() >= 4);
    }

    #[test]
    fn parses_client_config() {
        let mut app = TrajectoryDesktopApp::from_settings(DesktopSettings::default());
        app.access_key = ClientAccessKey::generate().to_display_string();
        let config = app.parse_config().unwrap();
        assert_eq!(config.domain, "your.domain.example");
        assert_eq!(config.listen.port(), 7000);
        assert_eq!(config.resolvers.len(), 5);
    }

    #[test]
    fn blank_resolvers_fall_back_to_public_defaults() {
        let mut app = TrajectoryDesktopApp::from_settings(DesktopSettings::default());
        app.access_key = ClientAccessKey::generate().to_display_string();
        app.resolvers_input.clear();
        let config = app.parse_config().unwrap();
        assert_eq!(config.resolvers, default_public_resolvers());
    }

    #[test]
    fn desktop_settings_round_trip_json() {
        let settings = DesktopSettings {
            access_key: "traj1_test".to_owned(),
            domain: "example.com".to_owned(),
            resolvers_input: "1.1.1.1:53\n8.8.8.8:53".to_owned(),
            listen_port: "7000".to_owned(),
            keep_alive_ms: "50".to_owned(),
        };
        let raw = serde_json::to_string(&settings).unwrap();
        let decoded: DesktopSettings = serde_json::from_str(&raw).unwrap();
        assert_eq!(decoded, settings);
    }
}
