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
- `deploy/`: systemd units and server installation scripts
- `scripts/`: release packaging and benchmark/support tools

## Build

```bash
cargo build --release -p trajectory-cli --bins
```

## Test

```bash
cargo test -p trajectory-core -p trajectory-cli --tests
cargo clippy -p trajectory-core -p trajectory-cli --bins --tests -- -D warnings
```

## Server

```bash
trajectory-server \
  --bind 0.0.0.0 \
  --dns-listen-port 53 \
  --target-address 127.0.0.1:1080 \
  --domain your.domain.example \
  --client-db /opt/trajectory/trajectory-clients.json
```

The server is authoritative for the tunnel zone and forwards stream traffic to the configured TCP target, usually a local SOCKS5 service.

## Client

```bash
trajectory-client \
  --listen 127.0.0.1:7000 \
  --domain your.domain.example \
  --access-key traj1_... \
  --resolver 1.1.1.1:53
```

For a SOCKS-gated DNS-over-TCP resolver path:

```bash
trajectory-client \
  --listen 127.0.0.1:7000 \
  --domain your.domain.example \
  --access-key traj1_... \
  --resolver-file .secrets/dnses.txt \
  --resolver-socks-proxy 127.0.0.1:11092 \
  --dns-max-payload 512
```

## Admin

```bash
trajectory-admin create-client --client-db trajectory-clients.json --label phone
trajectory-admin list-clients --client-db trajectory-clients.json
trajectory-admin disable-client --client-db trajectory-clients.json --id 0123abcd
```

## Deploy

See [docs/SELF_HOSTING.md](docs/SELF_HOSTING.md) for the operator flow and systemd service files.

## Benchmarking

See [docs/BENCHMARK_OBSERVABILITY.md](docs/BENCHMARK_OBSERVABILITY.md) for browser
waterfall metrics, hammer summaries, acceptance gates, and `TRAJECTORY_DIAG`
log parsing.

## License

Trajectory is distributed under the [Trajectory Restricted License](LICENSE). Redistribution, public demos, videos, forks for distribution, and commercial use require written permission from the copyright holder.
