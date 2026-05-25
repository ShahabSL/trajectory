# Self-Hosting Trajectory

This guide is for operators who want to run a Trajectory DNS tunnel server and hand credentials to client users.

## Requirements

- a DNS domain delegated to the server host, for example `t.example.com`
- UDP and TCP port `53` open to the server
- a Linux server with root access
- Rust locally if building from source
- an egress mode, chosen during install

Port `53` is commonly occupied by `systemd-resolved`. If the server cannot bind DNS, check:

```bash
sudo ss -tulpn | grep ':53'
```

## Egress Modes

| Mode | Use When | Client Behavior |
| --- | --- | --- |
| `socks5-direct` | easiest first install | clients can point SOCKS5-capable apps at `trajectory-client` |
| `127.0.0.1:1080` | a SOCKS5 daemon already runs on the server | clients can point SOCKS5-capable apps at `trajectory-client` |
| `HOST:PORT` | you want a fixed raw TCP upstream | clients must speak that upstream protocol |

For a first install, use `socks5-direct`.

## Build

```bash
cargo build --release -p trajectory-cli --bins
```

This produces:

- `target/release/trajectory-server`
- `target/release/trajectory-admin`
- `target/release/trajectory-client`

## Install Server

```bash
sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --target-address socks5-direct \
  --client-label laptop
```

The installer copies binaries into `/opt/trajectory`, writes `/etc/trajectory/server.env`, creates `/opt/trajectory/trajectory-clients.json`, installs `trajectory.service`, and prints the initial access key.

If port `53` is occupied by `systemd-resolved`, add:

```bash
--disable-systemd-resolved
```

That flag changes host DNS configuration. The installer backs up
`/etc/resolv.conf` before replacing it, but only use it on a server where you
are comfortable disabling `systemd-resolved`.

## Manage Clients

```bash
/opt/trajectory/trajectory-admin create-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --label phone

/opt/trajectory/trajectory-admin list-clients \
  --client-db /opt/trajectory/trajectory-clients.json

/opt/trajectory/trajectory-admin disable-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --id 7cad96fb
```

Restart after registry changes:

```bash
sudo systemctl restart trajectory.service
```

The server loads the registry at startup.

## Run Client

Read the access key without placing it in shell history:

```bash
read -rsp 'Trajectory access key: ' TRAJECTORY_ACCESS_KEY; echo
export TRAJECTORY_ACCESS_KEY

./trajectory-client \
  --listen 127.0.0.1:7000 \
  --http-listen 127.0.0.1:7001 \
  --domain your.domain.example \
  --resolver 1.1.1.1:53 \
  --resolver 8.8.8.8:53
```

Validate with a SOCKS-aware command:

```bash
curl -I --max-time 20 --socks5-hostname 127.0.0.1:7000 https://example.com
curl -I --max-time 20 --proxy http://127.0.0.1:7001 https://example.com
```

Use `--socks5-hostname`, not `--socks5`, so DNS names are resolved through the tunnel path instead of locally.

Use the HTTP proxy listener for browsers or tools that expect `http://host:port`
proxy settings. It supports HTTPS `CONNECT` and absolute-form `http://...`
requests, and it requires the server egress target to be `socks5-direct` or a
SOCKS5 upstream.

The SOCKS and HTTP curl checks apply to `socks5-direct` or SOCKS5-upstream
server installs. If you configured a raw `HOST:PORT` server target, validate
with the protocol spoken by that upstream instead.

## Resolver Files

`--resolver-file` accepts one resolver per line:

```text
1.1.1.1
8.8.8.8:53
# comments are allowed
```

Rules:

- read once at startup
- blank lines and `#` comments are ignored
- missing ports default to `53`
- invalid lines abort startup
- duplicates are deduped
- direct UDP admission runs when more than the active resolver target is supplied
  (default target: 64)
- SOCKS-gated DNS-over-TCP admission always runs (default target: 32)
- `--resolver-cohort-size` changes the admitted startup selection size
- `--resolver-admission-min` sets the minimum admitted startup resolvers
- `--admission-report` is written when startup admission runs

Write admission diagnostics when testing large files:

```bash
./trajectory-client \
  --listen 127.0.0.1:7000 \
  --domain your.domain.example \
  --resolver-file resolvers.txt \
  --admission-report admission.jsonl
```

## SOCKS-Gated Resolver Paths

For DNS-over-TCP through a local SOCKS gate:

```bash
./trajectory-client \
  --listen 127.0.0.1:7000 \
  --domain your.domain.example \
  --resolver-file resolvers.txt \
  --resolver-socks-proxy 127.0.0.1:11092 \
  --dns-max-payload 512 \
  --admission-report admission.jsonl
```

The SOCKS gate must allow TCP CONNECT to resolver port `53` and must sustain DNS-over-TCP lanes. Twoman is an external test path, not shipped by Trajectory. If the SOCKS gate itself cannot load a site directly, that site is not a fair Trajectory target through that gate.

## Validate Server

```bash
sudo systemctl status trajectory.service --no-pager
sudo journalctl -u trajectory.service -n 80 --no-pager
dig @127.0.0.1 your.domain.example SOA
```

## Troubleshooting

`bind UDP DNS ... address already in use`
: Port `53` is occupied. Run `sudo ss -tulpn | grep ':53'`.

Client starts but `curl` hangs
: Check DNS delegation, firewall, domain spelling, disabled key, and resolver path reachability.

`no resolvers passed signed tunnel admission`
: The resolver file or SOCKS-gated path cannot carry signed Trajectory DNS traffic to the authoritative server.

`connect target 127.0.0.1:1080`
: The server is configured for a local upstream that is not running. Reinstall with `--target-address socks5-direct` or start the upstream SOCKS daemon.

Local `curl` says connection refused
: `trajectory-client` is not running or the command points at the wrong `--listen` port.

Browser works inconsistently
: Configure the browser as SOCKS5 with remote hostname resolution. For command-line validation, use `--socks5-hostname`.
