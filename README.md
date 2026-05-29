# Trajectory

<p align="center">
  <img src="assets/branding/trajectory-mark.png" alt="Trajectory logo" width="160" />
</p>

<p align="center">
  <a href="https://github.com/ShahabSL/trajectory/actions/workflows/ci.yml"><img src="https://github.com/ShahabSL/trajectory/actions/workflows/ci.yml/badge.svg" alt="CI status" /></a>
  <a href="https://github.com/ShahabSL/trajectory/releases"><img src="https://img.shields.io/github/v/release/ShahabSL/trajectory?label=release" alt="Latest release" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-restricted-lightgrey.svg" alt="Restricted license" /></a>
</p>

Trajectory is a restricted-license DNS-native transport for proxying traffic through recursive DNS paths. It ships a Rust server/client toolchain plus desktop and Android clients with SOCKS5, HTTP proxy, and Android VPN modes.

The core transport uses encrypted per-client access keys, TXT/QNAME carriage, UDP DNS for normal recursive paths, persistent pipelined DNS-over-TCP for restricted paths, signed resolver admission probes, and diagnostic hooks for measuring throughput, latency, and resolver behavior.

## Download

Download release assets from [GitHub Releases](https://github.com/ShahabSL/trajectory/releases).

| Platform | Asset |
| --- | --- |
| Linux CLI | `trajectory-vVERSION-x86_64-unknown-linux-gnu-cli.tar.gz` |
| Windows CLI | `trajectory-vVERSION-x86_64-pc-windows-msvc-cli.zip` |
| macOS CLI | `trajectory-vVERSION-x86_64-apple-darwin-cli.tar.gz` or `trajectory-vVERSION-aarch64-apple-darwin-cli.tar.gz` |
| Windows desktop | `Trajectory_VERSION_x64_portable.zip` |
| Linux desktop | `.deb`, `.rpm`, or `.AppImage` |
| macOS desktop | `.app.tar.gz` and optional `.dmg` |
| Android | `trajectory-vVERSION-android.apk` |

Every release includes a merged `vVERSION-SHA256SUMS.txt` manifest.

## Quick Start

### 1. Install The Server

Delegate a DNS name to the server first, then install:

```bash
cargo build --release -p trajectory-cli --bins

sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --target-address socks5-direct \
  --client-label laptop
```

`socks5-direct` is the easiest first install: applications speak SOCKS5 to the local Trajectory client, and the server connects to the requested destination.

### 2. Add Or Rotate Client Keys

```bash
/opt/trajectory/trajectory-admin create-client \
  --client-db /opt/trajectory/trajectory-clients.json \
  --label phone

sudo systemctl restart trajectory.service
```

The server loads the client registry at startup, so restart after adding, disabling, or deleting keys.

### 3. Run The CLI Client

```bash
TRAJECTORY_ACCESS_KEY='traj1_...' trajectory-client \
  --listen 127.0.0.1:0 \
  --socks-listen 127.0.0.1:7000 \
  --http-listen 127.0.0.1:7001 \
  --domain your.domain.example \
  --resolver 1.1.1.1:53 \
  --resolver 8.8.8.8:53
```

Use `--socks-listen` for browsers and SOCKS5-capable apps. Use `--http-listen` for HTTP CONNECT proxy clients. `--listen 127.0.0.1:0` keeps the raw TCP tunnel on an unused local port when you only need proxy mode.

### 4. Check The Listener

```bash
curl -I --max-time 20 --socks5-hostname 127.0.0.1:7000 https://example.com
curl -I --max-time 20 --proxy http://127.0.0.1:7001 https://example.com
```

Those checks require a `socks5-direct` or SOCKS5-upstream server target.

## Clients

| Client | Status | Guide |
| --- | --- | --- |
| Rust CLI | Primary transport client and server tooling | [Self hosting](docs/SELF_HOSTING.md) |
| Desktop | Tauri client for Windows, macOS, and Linux | [Desktop client](docs/DESKTOP_CLIENT.md) |
| Android | Native proxy and VPN client with packaged sidecar | [Android client](docs/ANDROID_CLIENT.md) |

Desktop development:

```bash
cargo build --release -p trajectory-cli --bin trajectory-client
TRAJECTORY_CLIENT_BIN="$PWD/target/release/trajectory-client" \
  npm --prefix clients/desktop run tauri -- dev
```

## Repository Layout

- `crates/trajectory-core`: authentication, encrypted packets, DNS wire helpers, stream/reliability primitives
- `crates/trajectory-cli`: `trajectory-client`, `trajectory-server`, and `trajectory-admin`
- `crates/trajectory-vpn-bridge`: Android VPN bridge sidecar
- `clients/desktop`: Tauri desktop client
- `clients/android`: native Android proxy/VPN client
- `deploy/`: systemd units and server installation scripts
- `scripts/`: release packaging, CI smoke checks, benchmark, and support tools
- `assets/branding`: canonical logo and generated brand assets

## Validation

Fast local checks:

```bash
cargo fmt --all --check
cargo test -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --lib --bins
cargo test -p trajectory-core --test core_wire
cargo test -p trajectory-cli --test loopback_e2e
cargo clippy -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --all-targets -- -D warnings
```

Release and client gates are documented in [RELEASING.md](RELEASING.md). Live network e2e uses GitHub Actions secrets and is described in [docs/CI_E2E.md](docs/CI_E2E.md).

## Documentation

- [Self hosting](docs/SELF_HOSTING.md)
- [Desktop client](docs/DESKTOP_CLIENT.md)
- [Android client](docs/ANDROID_CLIENT.md)
- [Client app roadmap](docs/CLIENT_APPS.md)
- [CI e2e and secrets](docs/CI_E2E.md)
- [Benchmark observability](docs/BENCHMARK_OBSERVABILITY.md)
- [Security policy](SECURITY.md)
- [Disclaimer](DISCLAIMER.md)

## Product Boundary

Trajectory is source-available under a restricted license. It is not OSI open source. Redistribution, public demos, videos, forks for distribution, and commercial use require written permission from Shahab Lavasani.

Trajectory is transport software. It does not claim anonymity, endpoint compromise protection, resolver trust, or censorship resistance. Operators are responsible for lawful use, server hardening, DNS delegation, credential rotation, and monitoring.

## License

Trajectory is distributed under the [Trajectory Restricted License](LICENSE).
