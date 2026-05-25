use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use trajectory_cli::load_client_registry;
use trajectory_cli::runtime::{run_server, ServerConfig, ServerTargetMode};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let mut listen_port = 53u16;
    let mut bind_host = "0.0.0.0".to_string();
    let mut target = "127.0.0.1:1080".parse::<SocketAddr>().unwrap();
    let mut target_mode = ServerTargetMode::Tcp;
    let mut domain = None::<String>;
    let mut client_db = None::<PathBuf>;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            "--dns-listen-port" | "-l" => {
                listen_port = args
                    .next()
                    .context("missing dns listen port")?
                    .parse()
                    .context("invalid dns listen port")?;
            }
            "--bind" => bind_host = args.next().context("missing bind host")?,
            "--dns-listen-ipv6" | "-6" => bind_host = "::".to_string(),
            "--target-address" | "-a" => {
                let value = args.next().context("missing target address")?;
                if value == "socks5-direct" {
                    target_mode = ServerTargetMode::Socks5Direct;
                } else {
                    target = value.parse().context("invalid target address")?;
                    target_mode = ServerTargetMode::Tcp;
                }
            }
            "--domain" | "-d" => domain = Some(args.next().context("missing domain")?),
            "--client-db" => {
                client_db = Some(PathBuf::from(
                    args.next().context("missing client db path")?,
                ));
            }
            other => anyhow::bail!("unknown argument: {other}"),
        }
    }

    let domain = domain.context("missing required --domain")?;
    let client_db = client_db.context("missing required --client-db")?;
    let registry = load_client_registry(&client_db)?;
    let active_keys = registry.active_keys()?;
    if active_keys.is_empty() {
        anyhow::bail!(
            "client registry {} does not contain any enabled client keys",
            client_db.display()
        );
    }
    let bind = format!("{bind_host}:{listen_port}")
        .parse()
        .with_context(|| format!("invalid bind address {bind_host}:{listen_port}"))?;

    run_server(ServerConfig {
        bind,
        domain,
        target,
        target_mode,
        authorized_clients: Arc::new(active_keys),
    })
    .await
}

fn print_usage() {
    println!(
        "\
Usage: trajectory-server [options]

Required:
  -d, --domain <DOMAIN>              Authoritative tunnel domain
      --client-db <PATH>             Client registry JSON path

Optional:
      --bind <HOST>                  Bind host (default: 0.0.0.0)
  -l, --dns-listen-port <PORT>       DNS UDP/TCP listen port (default: 53)
  -6, --dns-listen-ipv6              Bind to :: instead of IPv4
  -a, --target-address <HOST:PORT>   Upstream TCP target, or socks5-direct (default: 127.0.0.1:1080)
  -h, --help                         Show this help"
    );
}
