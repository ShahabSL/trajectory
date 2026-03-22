use anyhow::{Context as _, Result};
use eframe::egui;
use std::net::SocketAddr;
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
            .with_min_inner_size([980.0, 700.0]),
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
}

impl Default for TrajectoryDesktopApp {
    fn default() -> Self {
        let (event_tx, event_rx) = mpsc::channel();
        Self {
            access_key: String::new(),
            domain: "your.domain.example".to_owned(),
            resolvers_input: "1.1.1.1:53\n1.0.0.1:53\n8.8.8.8:53\n8.8.4.4:53\n9.9.9.9:53"
                .to_owned(),
            listen_port: "7000".to_owned(),
            keep_alive_ms: "50".to_owned(),
            run_state: RunState::Stopped,
            status_line: "Ready".to_owned(),
            logs: vec![timestamped("Desktop client initialized")],
            event_tx,
            event_rx,
            tunnel: None,
        }
    }
}

impl eframe::App for TrajectoryDesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.drain_events();
        ctx.request_repaint_after(Duration::from_millis(100));

        egui::TopBottomPanel::top("hero").show(ctx, |ui| {
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.heading(
                        egui::RichText::new("Trajectory Desktop")
                            .size(30.0)
                            .color(rgb(242, 242, 235)),
                    );
                    ui.label(
                        egui::RichText::new("Connect with your access key and start browsing.")
                            .color(rgb(181, 187, 180))
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
                                .color(rgb(18, 18, 15))
                                .strong(),
                        )
                        .fill(rgb(230, 214, 126))
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
                                .color(rgb(242, 242, 235))
                                .strong(),
                        )
                        .fill(rgb(88, 34, 28))
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
                                    .color(rgb(201, 207, 198)),
                            );
                            ui.add_space(4.0);
                        }
                    });
            });
        });
    }
}

impl TrajectoryDesktopApp {
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
    style.visuals.panel_fill = rgb(18, 22, 21);
    style.visuals.extreme_bg_color = rgb(8, 10, 10);
    style.visuals.window_fill = rgb(22, 27, 25);
    style.visuals.widgets.inactive.bg_fill = rgb(28, 34, 31);
    style.visuals.widgets.hovered.bg_fill = rgb(44, 53, 49);
    style.visuals.widgets.active.bg_fill = rgb(58, 70, 64);
    style.visuals.override_text_color = Some(rgb(235, 238, 230));
    style.visuals.widgets.noninteractive.fg_stroke.color = rgb(235, 238, 230);
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
        .fill(rgb(26, 32, 29))
        .stroke(egui::Stroke::new(1.0, rgb(57, 66, 61)))
        .corner_radius(14.0)
        .inner_margin(egui::Margin::same(18))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .strong()
                    .size(18.0)
                    .color(rgb(240, 239, 227)),
            );
            ui.add_space(12.0);
            add_contents(ui);
        });
}

fn status_pill(ui: &mut egui::Ui, state: RunState, label: &str) {
    let fill = match state {
        RunState::Stopped => rgb(72, 75, 78),
        RunState::Starting => rgb(180, 130, 34),
        RunState::Running => rgb(52, 121, 79),
        RunState::Error => rgb(145, 45, 45),
    };
    egui::Frame::new()
        .fill(fill)
        .corner_radius(24.0)
        .inner_margin(egui::Margin::symmetric(14, 8))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(label)
                    .strong()
                    .color(rgb(247, 246, 238)),
            );
        });
}

fn stat_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).color(rgb(154, 162, 154)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .strong()
                    .color(rgb(240, 239, 227)),
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
            .color(rgb(185, 191, 182)),
    );
}

fn instruction(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .color(rgb(203, 208, 199))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_mode_builds_app_state() {
        let app = TrajectoryDesktopApp::default();
        assert_eq!(app.run_state, RunState::Stopped);
        assert!(app.active_resolver_count() >= 4);
    }

    #[test]
    fn parses_client_config() {
        let mut app = TrajectoryDesktopApp::default();
        app.access_key = ClientAccessKey::generate().to_display_string();
        let config = app.parse_config().unwrap();
        assert_eq!(config.domain, "your.domain.example");
        assert_eq!(config.listen.port(), 7000);
        assert_eq!(config.resolvers.len(), 5);
    }

    #[test]
    fn blank_resolvers_fall_back_to_public_defaults() {
        let mut app = TrajectoryDesktopApp::default();
        app.access_key = ClientAccessKey::generate().to_display_string();
        app.resolvers_input.clear();
        let config = app.parse_config().unwrap();
        assert_eq!(config.resolvers, default_public_resolvers());
    }
}
