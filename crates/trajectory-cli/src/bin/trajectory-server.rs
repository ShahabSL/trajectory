use anyhow::{Context, Result};
use std::net::SocketAddr;

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
    let mut target_address = "127.0.0.1:22".parse::<SocketAddr>().unwrap();
    let mut domain = None::<String>;

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
            "--dns-listen-ipv6" | "-6" => bind_host = "::".to_string(),
            "--target-address" | "-a" => {
                target_address = args
                    .next()
                    .context("missing target address")?
                    .parse()
                    .context("invalid target address")?;
            }
            "--domain" | "-d" => domain = Some(args.next().context("missing domain")?),
            "--cert" | "-c" => {
                let _ = args.next().context("missing cert path")?;
            }
            "--key" | "-k" => {
                let _ = args.next().context("missing key path")?;
            }
            other => anyhow::bail!("unknown argument: {other}"),
        }
    }

    let domain = domain.context("missing required --domain")?;
    let bind = format!("{bind_host}:{listen_port}")
        .parse()
        .with_context(|| format!("invalid bind address {bind_host}:{listen_port}"))?;
    trajectory_core::server::run(trajectory_core::server::ServerConfig {
        bind,
        domain,
        target: target_address,
    })
    .await
}
