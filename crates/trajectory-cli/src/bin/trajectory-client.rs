use anyhow::{Context, Result};
use std::net::SocketAddr;
use trajectory_core::auth::ClientAccessKey;
use trajectory_core::client::{default_client_config, parse_socket_addr, require_resolvers};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let mut listen = "127.0.0.1:5201".parse::<SocketAddr>().unwrap();
    let mut resolvers = Vec::new();
    let mut domain = None::<String>;
    let mut access_key = None::<ClientAccessKey>;
    let mut keep_alive_ms = None::<u64>;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--tcp-listen-port" | "-l" => {
                let port = args.next().context("missing tcp listen port")?;
                listen = format!("127.0.0.1:{port}").parse()?;
            }
            "--resolver" | "-r" => {
                let resolver = args.next().context("missing resolver")?;
                resolvers.push(parse_socket_addr(&resolver, 53)?);
            }
            "--domain" | "-d" => domain = Some(args.next().context("missing domain")?),
            "--access-key" | "-k" => {
                let value = args.next().context("missing access key")?;
                access_key = Some(ClientAccessKey::parse(&value)?);
            }
            "--keep-alive-interval" | "-t" => {
                keep_alive_ms = Some(
                    args.next()
                        .context("missing keep alive interval")?
                        .parse()
                        .context("invalid keep alive interval")?,
                );
            }
            "--congestion-control" | "-c" => {
                let _ = args.next().context("missing congestion control")?;
            }
            "--gso" | "-g" => {}
            other => anyhow::bail!("unknown argument: {other}"),
        }
    }

    let domain = domain.context("missing required --domain")?;
    let access_key = access_key.context("missing required --access-key")?;
    require_resolvers(&resolvers)?;
    let mut config = default_client_config(listen, resolvers, domain, access_key);
    if let Some(ms) = keep_alive_ms {
        config.keep_alive_interval = if ms == 0 {
            std::time::Duration::from_millis(trajectory_core::protocol::KEEPALIVE_MS)
        } else {
            std::time::Duration::from_millis(ms)
        };
    }
    trajectory_core::client::run(config).await
}
