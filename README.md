# Trajectory

Trajectory is a pure-Rust DNS tunnel optimized for recursive resolvers on raw UDP DNS transport.
Server operators generate per-client access keys, and every client must present one before the
tunnel will pass traffic.

The repository now follows a standard workspace layout:

- `crates/trajectory-core`: shared transport, protocol, client, and server logic
- `crates/trajectory-cli`: CLI binaries for `trajectory-client` and `trajectory-server`
- `crates/trajectory-mobile`: UniFFI-powered mobile bridge over the shared Rust client core
- `clients/desktop`: desktop control application built on `eframe/egui`
- `clients/android`: Android client project that consumes the shared mobile bridge
- `clients/ios`: iPhone app and packet-tunnel extension scaffold that consume the shared mobile bridge
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

Build the mobile bridge used by Android and iOS:

```bash
cargo build -p trajectory-mobile
python3 scripts/generate_mobile_bindings.py --profile debug
```

## Run the CLI

Generate client keys with the server TUI:

```bash
cargo run -p trajectory-cli --bin trajectory-server-tui -- \
  --dns-listen-port 53 \
  --target-address 127.0.0.1:1080 \
  --domain t.7-b.cc \
  --client-db trajectory-clients.json
```

Key commands:

- `g`: generate a new client access key
- `e`: enable or disable the selected client
- `d`: delete the selected client, with confirmation
- `s`: start or stop the authenticated server
- `q`: quit

Run the server directly once a client registry exists:

```bash
./target/release/trajectory-server \
  --dns-listen-port 53 \
  --target-address 127.0.0.1:1080 \
  --domain t.7-b.cc \
  --client-db trajectory-clients.json
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
  --access-key traj1_0123abcd_BASE32SECRET \
  --congestion-control bbr \
  --keep-alive-interval 50
```

## Run the Desktop Client

```bash
cargo run -p trajectory-desktop
```

The desktop app configures and runs the same shared Rust client core used by the CLI, including
the required access key field normal users need to paste from the server operator.

## Mobile Clients

The mobile apps live under:

- `clients/android`
- `clients/ios`

They share the same Rust tunnel logic through `crates/trajectory-mobile`, which exposes a stable UniFFI API to Kotlin and Swift.

What is implemented now:

- Android app with a real `VpnService` traffic path
- Android tun-to-SOCKS bridge via `hev-socks5-tunnel`
- Android Compose UI, persisted settings, access-key entry, status/log views,
  and a real bridge to the Rust tunnel controller
- iPhone app structure with SwiftUI, persisted settings, access-key entry, status/log views,
  and a real bridge to the Rust tunnel controller
- packet-tunnel service scaffolding on iPhone so full-device mode can be added without changing the shared core boundary
- Android Gradle build integration that compiles and packages both `libtrajectory_mobile.so`
  and `libhev-socks5-tunnel.so` into the APK

What still depends on external platform toolchains:

- Android APK/AAB builds require a full Android SDK + NDK + Gradle environment plus `cargo-ndk`
- iOS app builds require Xcode/Swift on macOS

## Downloads

End users should download release artifacts from GitHub Releases instead of building from source.

Each release publishes:

- Linux CLI and desktop bundles
- Windows CLI and desktop bundles
- macOS CLI and desktop bundles
- checksum manifests

Maintainer release flow is documented in [RELEASING.md](RELEASING.md).

## Browser Path

Once the client is running, use the local Trajectory listener as a SOCKS5 proxy directly.

Point Firefox at:

- SOCKS host: `127.0.0.1`
- Port: `7000`
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
./target/release/trajectory-client \
  --tcp-listen-port 7000 \
  --resolver 1.1.1.1:53 \
  --resolver 1.0.0.1:53 \
  --resolver 8.8.8.8:53 \
  --resolver 8.8.4.4:53 \
  --resolver 9.9.9.9:53 \
  --domain t.7-b.cc \
  --access-key traj1_0123abcd_BASE32SECRET \
  --congestion-control bbr \
  --keep-alive-interval 50

curl -I --socks5-hostname 127.0.0.1:7000 https://example.com
```

## Release Packaging

Build portable Linux bundles locally:

```bash
python3 scripts/package_release.py --target x86_64-unknown-linux-gnu --output-dir dist/local
```

This produces:

- `trajectory-vVERSION-x86_64-unknown-linux-gnu-cli.tar.gz`
- `trajectory-vVERSION-x86_64-unknown-linux-gnu-desktop.tar.gz`
- `trajectory-vVERSION-x86_64-unknown-linux-gnu-SHA256SUMS.txt`

The GitHub Actions workflows under `.github/workflows/` build the Windows, macOS, and Linux release bundles and publish them on tags.

## Licensing

Trajectory is dual-licensed under either:

- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)

## Deploy

Install the server binary, local SOCKS upstream, client registry, and service units:

```bash
install -d -m 755 /opt/trajectory
install -m 755 target/release/trajectory-server /opt/trajectory/trajectory-server
install -m 755 /path/to/hev-socks5-server /opt/trajectory/hev-socks5-server
install -m 640 trajectory-clients.json /opt/trajectory/trajectory-clients.json
install -m 644 deploy/hev-socks5-server.yml /opt/trajectory/hev-socks5-server.yml
install -m 644 deploy/trajectory-socks.service /etc/systemd/system/trajectory-socks.service
install -m 644 deploy/trajectory.service /etc/systemd/system/trajectory.service
systemctl daemon-reload
systemctl enable --now trajectory-socks.service trajectory.service
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
