# Trajectory

Trajectory is a pure-Rust DNS tunnel optimized for recursive resolvers on raw UDP DNS transport.

The repository now follows a standard workspace layout:

- `crates/trajectory-core`: shared transport, protocol, client, and server logic
- `crates/trajectory-cli`: CLI binaries for `trajectory-client` and `trajectory-server`
- `clients/desktop`: desktop control application built on `eframe/egui`
- `clients/android`: Android wrapper plan for the shared core
- `clients/ios`: iOS wrapper plan for the shared core
- `scripts/`: benchmark and support tooling
- `deploy/`: service units and deployment assets

## Build

Build the CLI binaries:

```bash
cargo build --release -p trajectory-cli --bins
```

Build the desktop client:

```bash
cargo build --release -p trajectory-desktop
```

## Run the CLI

Server:

```bash
./target/release/trajectory-server \
  --dns-listen-port 53 \
  --target-address 127.0.0.1:22 \
  --domain t.7-b.cc
```

Client:

```bash
./target/release/trajectory-client \
  --tcp-listen-port 7000 \
  --resolver 1.1.1.1:53 \
  --resolver 1.0.0.1:53 \
  --resolver 8.8.8.8:53 \
  --resolver 8.8.4.4:53 \
  --resolver 9.9.9.9:53 \
  --domain t.7-b.cc \
  --congestion-control bbr \
  --keep-alive-interval 50
```

## Run the Desktop Client

```bash
cargo run -p trajectory-desktop
```

The desktop app configures and runs the same shared Rust client core used by the CLI.

## Browser Path

Once the client is running, create a local SOCKS proxy on top of the tunnel:

```bash
ssh -N -D 127.0.0.1:1080 \
  -o StrictHostKeyChecking=no \
  -o UserKnownHostsFile=/dev/null \
  -o PreferredAuthentications=password \
  -o PubkeyAuthentication=no \
  -p 7000 root@127.0.0.1
```

Then point Firefox at:

- SOCKS host: `127.0.0.1`
- Port: `1080`
- SOCKS v5
- Proxy DNS enabled

## Tests

Core and CLI:

```bash
cargo test
```

Desktop smoke test:

```bash
cargo test -p trajectory-desktop
cargo run -p trajectory-desktop -- --smoke-test
```

End-to-end browser harness:

```bash
/tmp/run-trajectory-browser-5.sh
```

## Deploy

Install the server binary and service unit:

```bash
install -d -m 755 /opt/trajectory
install -m 755 target/release/trajectory-server /opt/trajectory/trajectory-server
install -m 644 deploy/trajectory.service /etc/systemd/system/trajectory.service
systemctl daemon-reload
systemctl enable --now trajectory.service
```

If the host previously used `systemd-resolved`, replace `/etc/resolv.conf` with upstream resolvers before or after stopping it:

```bash
rm -f /etc/resolv.conf
cat >/etc/resolv.conf <<'EOF'
nameserver 1.1.1.1
nameserver 8.8.8.8
nameserver 9.9.9.9
options edns0
EOF
```

Learning Notes:
- A shared core plus thin platform wrappers is the normal open source structure for networked Rust applications that need desktop and mobile clients.
- Keeping the transport engine separate from UI code makes it easier to test protocol behavior and port it to new platforms.

Why This Matters:
- The CLI, desktop app, and future mobile wrappers all share one transport implementation.
- Product code and experimental legacy code are no longer mixed together in the main build path.
