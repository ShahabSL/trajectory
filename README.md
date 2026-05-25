# Trajectory

Trajectory is a restricted-license Rust DNS transport with:

- encrypted per-client access keys
- recursive-DNS TXT/QNAME carriage
- UDP DNS for normal recursive paths
- persistent pipelined DNS-over-TCP through SOCKS for hostile paths
- signed resolver admission probes plus JSON benchmark/diagnostic hooks

## Workspace

- `crates/trajectory-core`: authentication, encrypted packets, DNS wire helpers, and stream/reliability primitives
- `crates/trajectory-cli`: `trajectory-client`, `trajectory-server`, and `trajectory-admin`
- `clients/desktop`: Tauri desktop proxy client for Windows, macOS, and Linux
- `clients/android`: native Android proxy client that packages `trajectory-client`
- `deploy/`: systemd units and server installation scripts
- `scripts/`: release packaging and benchmark/support tools

## Build

```bash
cargo build --release -p trajectory-cli --bins
```

## Test

```bash
cargo fmt --all --check
cargo test -p trajectory-core -p trajectory-cli --lib --bins
cargo test -p trajectory-core --test core_wire
cargo test -p trajectory-cli --test loopback_e2e
cargo clippy -p trajectory-core -p trajectory-cli --all-targets -- -D warnings
```

## Quick Start

### 1. Install The Server

Delegate a DNS name to the server first, then install:

```bash
sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --target-address socks5-direct \
  --client-label laptop
```

`socks5-direct` is the easiest first install: applications can speak SOCKS5 to the local client listener and the server will connect to the requested destination.

### 2. Add Or Rotate Client Keys

```bash
/opt/trajectory/trajectory-admin create-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --label phone

sudo systemctl restart trajectory.service
```

The server loads the client registry at startup, so restart after adding, disabling, or deleting keys.

### 3. Run A Client

```bash
TRAJECTORY_ACCESS_KEY='traj1_...' trajectory-client \
  --listen 127.0.0.1:7000 \
  --http-listen 127.0.0.1:7001 \
  --domain your.domain.example \
  --resolver 1.1.1.1:53 \
  --resolver 8.8.8.8:53
```

The `--listen` port is the raw/SOCKS-compatible tunnel. It behaves like a SOCKS5 proxy when the server target is `socks5-direct` or another SOCKS5 upstream. The optional `--http-listen` port is an HTTP proxy listener for browsers and tools that use HTTP CONNECT.

Desktop GUI:

```bash
cargo build --release -p trajectory-cli --bin trajectory-client
TRAJECTORY_CLIENT_BIN="$PWD/target/release/trajectory-client" \
  npm --prefix clients/desktop run tauri -- dev
```

### 4. Use The Local Listener

```bash
curl -I --max-time 20 --socks5-hostname 127.0.0.1:7000 https://example.com
curl -I --max-time 20 --proxy http://127.0.0.1:7001 https://example.com
```

Those proxy checks require a `socks5-direct` or SOCKS5-upstream server target. Raw `HOST:PORT` egress should be validated with that upstream protocol.

See [docs/SELF_HOSTING.md](docs/SELF_HOSTING.md) for the operator flow and systemd service files.

Desktop client guide: [docs/DESKTOP_CLIENT.md](docs/DESKTOP_CLIENT.md).
Android client guide: [docs/ANDROID_CLIENT.md](docs/ANDROID_CLIENT.md).
Client app roadmap: [docs/CLIENT_APPS.md](docs/CLIENT_APPS.md).
Secret-gated live CI: [docs/CI_E2E.md](docs/CI_E2E.md).

## Benchmarking

See [docs/BENCHMARK_OBSERVABILITY.md](docs/BENCHMARK_OBSERVABILITY.md) for browser
waterfall metrics, hammer summaries, acceptance gates, and `TRAJECTORY_DIAG`
log parsing.

## License

Trajectory is distributed under the [Trajectory Restricted License](LICENSE). Redistribution, public demos, videos, forks for distribution, and commercial use require written permission from the copyright holder.
