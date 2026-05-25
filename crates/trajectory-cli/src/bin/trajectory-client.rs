use anyhow::{Context, Result};
use std::fs;
use std::net::SocketAddr;
use std::time::Duration;
use trajectory_cli::runtime::{parse_socket_addr, run_client, ClientConfig};
use trajectory_core::auth::ClientAccessKey;

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
    let mut resolver_socks_proxy = None::<SocketAddr>;
    let mut poll_interval = Duration::from_millis(25);
    let mut dns_max_payload = None::<u16>;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            "--tcp-listen-port" | "-l" => {
                let port = args.next().context("missing tcp listen port")?;
                listen = format!("127.0.0.1:{port}").parse()?;
            }
            "--listen" => {
                listen = args
                    .next()
                    .context("missing listen address")?
                    .parse()
                    .context("invalid listen address")?;
            }
            "--resolver" | "-r" => {
                let resolver = args.next().context("missing resolver")?;
                resolvers.push(parse_socket_addr(&resolver, 53)?);
            }
            "--resolver-file" => {
                let path = args.next().context("missing resolver file")?;
                let contents = fs::read_to_string(&path).with_context(|| format!("read {path}"))?;
                for (line_index, line) in contents.lines().enumerate() {
                    let resolver = line.split('#').next().unwrap_or("").trim();
                    if resolver.is_empty() {
                        continue;
                    }
                    resolvers.push(parse_socket_addr(resolver, 53).with_context(|| {
                        format!("invalid resolver at {path}:{}", line_index + 1)
                    })?);
                }
            }
            "--domain" | "-d" => domain = Some(args.next().context("missing domain")?),
            "--access-key" | "-k" => {
                let value = args.next().context("missing access key")?;
                access_key = Some(ClientAccessKey::parse(&value)?);
            }
            "--resolver-socks-proxy" => {
                let proxy = args.next().context("missing resolver socks proxy")?;
                resolver_socks_proxy = Some(parse_socket_addr(&proxy, 1080)?);
            }
            "--poll-interval-ms" | "--keep-alive-interval" | "-t" => {
                let ms: u64 = args
                    .next()
                    .context("missing poll interval")?
                    .parse()
                    .context("invalid poll interval")?;
                poll_interval = Duration::from_millis(ms.max(1));
            }
            "--dns-max-payload" => {
                dns_max_payload = Some(
                    args.next()
                        .context("missing dns max payload")?
                        .parse()
                        .context("invalid dns max payload")?,
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
    let access_key = match access_key {
        Some(key) => key,
        None => {
            let value = std::env::var("TRAJECTORY_ACCESS_KEY")
                .context("missing required --access-key or TRAJECTORY_ACCESS_KEY")?;
            ClientAccessKey::parse(&value)?
        }
    };
    if resolvers.is_empty() {
        resolvers.push("1.1.1.1:53".parse().unwrap());
        resolvers.push("8.8.8.8:53".parse().unwrap());
    }
    let dns_max_payload = dns_max_payload.unwrap_or_else(|| {
        if resolver_socks_proxy.is_some() {
            4096
        } else {
            1232
        }
    });

    run_client(ClientConfig {
        listen,
        resolvers,
        domain,
        access_key,
        resolver_socks_proxy,
        poll_interval,
        dns_max_payload,
    })
    .await
}

fn print_usage() {
    println!(
        "\
Usage: trajectory-client [options]

Required:
  -d, --domain <DOMAIN>              Authoritative tunnel domain
  -k, --access-key <ACCESS_KEY>      Client access key; or TRAJECTORY_ACCESS_KEY

Optional:
  -l, --tcp-listen-port <PORT>       Local raw TCP listen port (default: 5201)
      --listen <HOST:PORT>           Full local listen address
  -r, --resolver <HOST:PORT>         Recursive resolver; repeat for multiple
      --resolver-file <PATH>         Read recursive resolvers from a file
      --resolver-socks-proxy <ADDR>  Send DNS-over-TCP through SOCKS5 proxy
  -t, --poll-interval-ms <MS>        Delay after resolver failures
      --dns-max-payload <BYTES>      Advertised response payload budget (default: 1232, or 4096 with --resolver-socks-proxy)
  -h, --help                         Show this help"
    );
}
