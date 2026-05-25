# Self-Hosting Trajectory

This guide is for operators who want to run a Trajectory DNS tunnel server and hand credentials to CLI users.

## Requirements

- a DNS domain delegated to the server host
- a Linux server with root access
- a local SOCKS5 upstream on the server, usually `127.0.0.1:1080`
- Rust locally if you are building binaries from source

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
  --target-address 127.0.0.1:1080 \
  --client-label phone
```

The installer copies binaries into `/opt/trajectory`, writes `/etc/trajectory/server.env`, creates `/opt/trajectory/trajectory-clients.json`, installs `trajectory.service`, and prints the initial access key when requested.

## Manage Clients

```bash
/opt/trajectory/trajectory-admin create-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --label laptop

/opt/trajectory/trajectory-admin list-clients \
  --client-db /opt/trajectory/trajectory-clients.json

/opt/trajectory/trajectory-admin disable-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --id 7cad96fb
```

## Run Client

```bash
./trajectory-client \
  --listen 127.0.0.1:7000 \
  --domain your.domain.example \
  --access-key traj1_0123abcd_BASE32SECRET \
  --resolver 1.1.1.1:53
```

For DNS-over-TCP through a local SOCKS gate:

```bash
./trajectory-client \
  --listen 127.0.0.1:7000 \
  --domain your.domain.example \
  --access-key traj1_0123abcd_BASE32SECRET \
  --resolver-file .secrets/dnses.txt \
  --resolver-socks-proxy 127.0.0.1:11092 \
  --dns-max-payload 512
```

## Validate

Server:

```bash
systemctl status trajectory.service
```

Client:

```bash
curl -I --max-time 20 --socks5-hostname 127.0.0.1:7000 http://example.com
```

## Notes

- `trajectory.service` binds UDP/TCP port 53 by default.
- Public recursive paths vary by resolver and by authoritative server placement.
- SOCKS-gated DNS-over-TCP paths should use resolver admission; provide a resolver file and let the client select working paths.
