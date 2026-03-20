use anyhow::{Context, Result};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::watch;
use trajectory_cli::{load_client_registry, save_client_registry};
use trajectory_core::auth::StoredClientRegistry;

fn main() -> Result<()> {
    let args = ServerTuiArgs::parse()?;
    let mut terminal = ratatui::init();
    let mut app = ServerTuiApp::new(args)?;
    let result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    result?;
    Ok(())
}

fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut ServerTuiApp) -> Result<()> {
    loop {
        app.drain_events();
        terminal.draw(|frame| app.render(frame))?;
        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && app.handle_key(key.code)? {
                    app.stop_server();
                    break;
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
struct ServerTuiArgs {
    bind: SocketAddr,
    domain: String,
    target: SocketAddr,
    client_db: PathBuf,
}

impl ServerTuiArgs {
    fn parse() -> Result<Self> {
        let mut listen_port = 53u16;
        let mut bind_host = "0.0.0.0".to_owned();
        let mut target = "127.0.0.1:1080".parse::<SocketAddr>().unwrap();
        let mut domain = None::<String>;
        let mut client_db = PathBuf::from("trajectory-clients.json");

        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dns-listen-port" | "-l" => {
                    listen_port = args
                        .next()
                        .context("missing dns listen port")?
                        .parse()
                        .context("invalid dns listen port")?;
                }
                "--bind" => bind_host = args.next().context("missing bind host")?,
                "--dns-listen-ipv6" | "-6" => bind_host = "::".to_owned(),
                "--target-address" | "-a" => {
                    target = args
                        .next()
                        .context("missing target address")?
                        .parse()
                        .context("invalid target address")?;
                }
                "--domain" | "-d" => domain = Some(args.next().context("missing domain")?),
                "--client-db" => {
                    client_db = PathBuf::from(args.next().context("missing client db path")?);
                }
                other => anyhow::bail!("unknown argument: {other}"),
            }
        }

        let domain = domain.context("missing required --domain")?;
        let bind = format!("{bind_host}:{listen_port}")
            .parse()
            .with_context(|| format!("invalid bind address {bind_host}:{listen_port}"))?;
        Ok(Self {
            bind,
            domain,
            target,
            client_db,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ServerRunState {
    Stopped,
    Starting,
    Running,
    Failed,
}

enum ServerEvent {
    Started,
    Stopped,
    Failed(String),
}

struct ServerHandle {
    stop_tx: watch::Sender<bool>,
    join: thread::JoinHandle<()>,
}

struct ServerTuiApp {
    args: ServerTuiArgs,
    registry: StoredClientRegistry,
    list_state: ListState,
    generated_key: Option<String>,
    pending_delete: Option<u32>,
    notice: String,
    server_state: ServerRunState,
    server_handle: Option<ServerHandle>,
    server_tx: Sender<ServerEvent>,
    server_rx: Receiver<ServerEvent>,
}

impl ServerTuiApp {
    fn new(args: ServerTuiArgs) -> Result<Self> {
        let registry = load_client_registry(&args.client_db)?;
        let (server_tx, server_rx) = mpsc::channel();
        let mut list_state = ListState::default();
        if !registry.keys.is_empty() {
            list_state.select(Some(0));
        }
        Ok(Self {
            args,
            registry,
            list_state,
            generated_key: None,
            pending_delete: None,
            notice: "g: new client key | e: enable/disable | d: delete | s: start/stop server | q: quit".to_owned(),
            server_state: ServerRunState::Stopped,
            server_handle: None,
            server_tx,
            server_rx,
        })
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('g') => self.generate_client_key()?,
            KeyCode::Char('e') => self.toggle_selected_client()?,
            KeyCode::Char('d') | KeyCode::Delete => self.delete_selected_client()?,
            KeyCode::Char('s') => {
                if self.server_state == ServerRunState::Running
                    || self.server_state == ServerRunState::Starting
                {
                    self.stop_server();
                } else {
                    self.start_server()?;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => self.next_client(),
            KeyCode::Up | KeyCode::Char('k') => self.previous_client(),
            _ => {}
        }
        Ok(false)
    }

    fn generate_client_key(&mut self) -> Result<()> {
        self.pending_delete = None;
        let label = format!("Client {}", self.registry.keys.len() + 1);
        let record = self.registry.create_key(label)?;
        save_client_registry(&self.args.client_db, &self.registry)?;
        self.generated_key = Some(record.access_key_string()?);
        self.notice = format!("Generated {}", record.label);
        self.list_state
            .select(Some(self.registry.keys.len().saturating_sub(1)));
        if self.server_state == ServerRunState::Running {
            self.restart_server()?;
        }
        Ok(())
    }

    fn toggle_selected_client(&mut self) -> Result<()> {
        self.pending_delete = None;
        let Some(index) = self.list_state.selected() else {
            return Ok(());
        };
        if let Some(entry) = self.registry.keys.get_mut(index) {
            entry.enabled = !entry.enabled;
            self.notice = if entry.enabled {
                format!("Enabled {}", entry.label)
            } else {
                format!("Disabled {}", entry.label)
            };
            save_client_registry(&self.args.client_db, &self.registry)?;
            self.sync_server_after_registry_change()?;
        }
        Ok(())
    }

    fn delete_selected_client(&mut self) -> Result<()> {
        let Some(index) = self.list_state.selected() else {
            return Ok(());
        };
        let Some(entry) = self.registry.keys.get(index) else {
            return Ok(());
        };

        if self.pending_delete != Some(entry.id) {
            self.pending_delete = Some(entry.id);
            self.notice = format!("Press d again to delete {} ({:08x})", entry.label, entry.id);
            return Ok(());
        }

        let removed = self
            .registry
            .remove_key_at(index)
            .context("selected client disappeared")?;
        self.pending_delete = None;
        save_client_registry(&self.args.client_db, &self.registry)?;
        self.notice = format!("Deleted {}", removed.label);
        self.generated_key = None;
        if self.registry.keys.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(index.min(self.registry.keys.len() - 1)));
        }
        self.sync_server_after_registry_change()?;
        Ok(())
    }

    fn next_client(&mut self) {
        if self.registry.keys.is_empty() {
            self.list_state.select(None);
            return;
        }
        self.pending_delete = None;
        let next = match self.list_state.selected() {
            Some(index) => (index + 1) % self.registry.keys.len(),
            None => 0,
        };
        self.list_state.select(Some(next));
    }

    fn previous_client(&mut self) {
        if self.registry.keys.is_empty() {
            self.list_state.select(None);
            return;
        }
        self.pending_delete = None;
        let prev = match self.list_state.selected() {
            Some(0) | None => self.registry.keys.len() - 1,
            Some(index) => index - 1,
        };
        self.list_state.select(Some(prev));
    }

    fn sync_server_after_registry_change(&mut self) -> Result<()> {
        if self.server_state != ServerRunState::Running {
            return Ok(());
        }
        if self.registry.active_keys()?.is_empty() {
            self.stop_server();
            self.notice = "Server stopped: no enabled client keys remain".to_owned();
            return Ok(());
        }
        self.restart_server()
    }

    fn start_server(&mut self) -> Result<()> {
        let active_keys = self.registry.active_keys()?;
        if active_keys.is_empty() {
            self.server_state = ServerRunState::Failed;
            self.notice = "Generate and enable at least one client key before starting the server".to_owned();
            return Ok(());
        }

        let config = trajectory_core::server::ServerConfig {
            bind: self.args.bind,
            domain: self.args.domain.clone(),
            target: self.args.target,
            authorized_clients: Arc::new(active_keys),
        };
        let (stop_tx, stop_rx) = watch::channel(false);
        let tx = self.server_tx.clone();
        self.server_state = ServerRunState::Starting;
        self.notice = format!("Starting server on {}", self.args.bind);

        let join = thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    let _ = tx.send(ServerEvent::Failed(format!("runtime init failed: {error:#}")));
                    return;
                }
            };
            let _ = tx.send(ServerEvent::Started);
            match runtime.block_on(trajectory_core::server::run_until(config, stop_rx)) {
                Ok(()) => {
                    let _ = tx.send(ServerEvent::Stopped);
                }
                Err(error) => {
                    let _ = tx.send(ServerEvent::Failed(format!("{error:#}")));
                }
            }
        });

        self.server_handle = Some(ServerHandle { stop_tx, join });
        Ok(())
    }

    fn stop_server(&mut self) {
        if let Some(handle) = self.server_handle.take() {
            let _ = handle.stop_tx.send(true);
            let _ = handle.join.join();
        }
        self.server_state = ServerRunState::Stopped;
        self.notice = "Server stopped".to_owned();
    }

    fn restart_server(&mut self) -> Result<()> {
        self.stop_server();
        self.start_server()
    }

    fn drain_events(&mut self) {
        while let Ok(event) = self.server_rx.try_recv() {
            match event {
                ServerEvent::Started => {
                    self.server_state = ServerRunState::Running;
                    self.notice = format!("Server running on {} for {}", self.args.bind, self.args.domain);
                }
                ServerEvent::Stopped => {
                    self.server_state = ServerRunState::Stopped;
                    self.notice = "Server stopped".to_owned();
                }
                ServerEvent::Failed(error) => {
                    self.server_state = ServerRunState::Failed;
                    self.notice = error;
                }
            }
        }
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(6),
            Constraint::Min(12),
            Constraint::Length(7),
        ])
        .areas(frame.area());

        let status_style = match self.server_state {
            ServerRunState::Stopped => Style::default().fg(Color::Gray),
            ServerRunState::Starting => Style::default().fg(Color::Yellow),
            ServerRunState::Running => Style::default().fg(Color::Green),
            ServerRunState::Failed => Style::default().fg(Color::Red),
        };

        let header_text = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Trajectory Server", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(
                    match self.server_state {
                        ServerRunState::Stopped => "Stopped",
                        ServerRunState::Starting => "Starting",
                        ServerRunState::Running => "Running",
                        ServerRunState::Failed => "Failed",
                    },
                    status_style.add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(format!("Bind: {}    Domain: {}    Target: {}", self.args.bind, self.args.domain, self.args.target)),
            Line::from(format!("Client registry: {}", self.args.client_db.display())),
        ])
        .block(Block::default().borders(Borders::ALL).title("Server"))
        .wrap(Wrap { trim: false });
        frame.render_widget(header_text, header);

        let [list_area, detail_area] =
            Layout::horizontal([Constraint::Percentage(38), Constraint::Percentage(62)]).areas(body);

        let items = if self.registry.keys.is_empty() {
            vec![ListItem::new("No client keys yet")]
        } else {
            self.registry
                .keys
                .iter()
                .map(|entry| {
                    let status = if entry.enabled { "enabled" } else { "disabled" };
                    ListItem::new(Line::from(vec![
                        Span::styled(&entry.label, Style::default().fg(Color::White)),
                        Span::raw(" "),
                        Span::styled(format!("[{}]", status), Style::default().fg(Color::Gray)),
                    ]))
                })
                .collect()
        };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Clients"))
            .highlight_style(Style::default().bg(Color::Rgb(36, 63, 92)).add_modifier(Modifier::BOLD))
            .highlight_symbol("› ");
        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        let detail = if let Some(index) = self.list_state.selected().filter(|_| !self.registry.keys.is_empty()) {
            let entry = &self.registry.keys[index];
            let access_key = entry
                .access_key_string()
                .unwrap_or_else(|_| "Invalid stored key".to_owned());
            let mut detail_lines = vec![
                Line::from(vec![
                    Span::styled(&entry.label, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::raw(" "),
                    Span::styled(
                        if entry.enabled { "enabled" } else { "disabled" },
                        Style::default().fg(if entry.enabled { Color::Green } else { Color::Yellow }),
                    ),
                ]),
                Line::from(format!("Created: {}", entry.created_unix)),
                Line::from(format!("Client ID: {:08x}", entry.id)),
                Line::from(""),
                Line::from("Access key"),
                Line::from(access_key),
                Line::from(""),
                Line::from("Share this key with the client app along with the domain and resolver inputs."),
            ];
            if self.pending_delete == Some(entry.id) {
                detail_lines.push(Line::from(""));
                detail_lines.push(Line::from(Span::styled(
                    "Press d again to confirm deletion.",
                    Style::default().fg(Color::Yellow),
                )));
            }
            Paragraph::new(detail_lines)
            .block(Block::default().borders(Borders::ALL).title("Selected client"))
            .wrap(Wrap { trim: false })
        } else {
            Paragraph::new("Press g to generate the first client key.")
                .block(Block::default().borders(Borders::ALL).title("Selected client"))
        };
        frame.render_widget(detail, detail_area);

        let footer_lines = vec![
            Line::from(self.notice.clone()),
            Line::from("g new key    e enable/disable    d delete    s start/stop server    ↑↓ select    q quit"),
            Line::from(
                self.generated_key
                    .clone()
                    .map(|key| format!("Last generated key: {key}"))
                    .unwrap_or_else(|| "Last generated key: none".to_owned()),
            ),
        ];
        let footer_widget = Paragraph::new(footer_lines)
            .block(Block::default().borders(Borders::ALL).title("Actions"))
            .wrap(Wrap { trim: false });
        frame.render_widget(footer_widget, footer);
    }
}
