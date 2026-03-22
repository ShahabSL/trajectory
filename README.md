# Trajectory

<p align="center">
  <img src="assets/branding/trajectory-pixel-t.svg" width="128" height="128" alt="Trajectory pixel-art T logo" />
</p>

Trajectory is a recursive-DNS tunnel with:

- a shared Rust transport core
- authenticated per-client access keys
- CLI, desktop, Android, and iOS app surfaces over the same client engine
- automatic public-resolver probing and active-path selection on the client

## Workspace Layout

- `crates/trajectory-core`: shared transport, protocol, client, and server logic
- `crates/trajectory-cli`: `trajectory-client`, `trajectory-server`, `trajectory-admin`, and `trajectory-server-tui`
- `crates/trajectory-mobile`: UniFFI mobile bridge over the shared Rust client core
- `clients/desktop`: `eframe/egui` desktop client
- `clients/android`: Android client project
- `clients/ios`: iPhone app project sources
- `deploy/`: server install assets and systemd units
- `scripts/`: release, benchmark, and support tooling

## Shipping Surfaces

Release-ready from this repository:

- CLI client
- CLI server
- CLI admin tool for client-key management
- desktop client
- Android app in proxy mode and VPN mode
- iOS app source for the loopback/mobile-controller path

Not shipped as a supported release surface:

- iOS packet-tunnel extension

The generated iOS shipping project excludes the broken packet-tunnel target on purpose.

## Quick Start

Full self-hosting and credential handoff are documented in [docs/SELF_HOSTING.md](docs/SELF_HOSTING.md).

Minimal flow:

1. Build the server/admin/client binaries:

```bash
cargo build --release -p trajectory-cli --bins
```

2. Install the server:

```bash
sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --target-address 127.0.0.1:1080 \
  --client-label phone
```

3. Copy the printed `traj1_...` access key into a client and use the same domain.

Resolvers may be left blank. Current clients fall back to the built-in public resolver set (`1.1.1.1`, `1.0.0.1`, `8.8.8.8`, `8.8.4.4`, `9.9.9.9`) and the client automatically probes the full cohort, selects the strongest active subset, and refreshes paths during runtime.

## Build

CLI binaries:

```bash
cargo build --release -p trajectory-cli --bins
```

Desktop client:

```bash
cargo build --release -p trajectory-desktop
```

Mobile bridge:

```bash
cargo build -p trajectory-mobile
python3 scripts/generate_mobile_bindings.py --profile debug
```

Android release APK:

```bash
scripts/build_android_release.sh
```

The Android build helper isolates Gradle state automatically so it does not depend on a healthy machine-global `~/.gradle` daemon registry. Set `GRADLE_USER_HOME` yourself if you want persistent local Gradle caches across repeated builds.

## Operator Tools

Create a client key without using the TUI:

```bash
cargo run -p trajectory-cli --bin trajectory-admin -- \
  create-client \
  --client-db trajectory-clients.json \
  --label phone
```

List clients:

```bash
cargo run -p trajectory-cli --bin trajectory-admin -- \
  list-clients \
  --client-db trajectory-clients.json
```

Disable a client:

```bash
cargo run -p trajectory-cli --bin trajectory-admin -- \
  disable-client \
  --client-db trajectory-clients.json \
  --id 0123abcd
```

The TUI still exists for maintainers who prefer it:

```bash
cargo run -p trajectory-cli --bin trajectory-server-tui -- \
  --domain your.domain.example \
  --client-db trajectory-clients.json
```

## Client Usage

CLI:

```bash
./target/release/trajectory-client \
  --tcp-listen-port 7000 \
  --domain your.domain.example \
  --access-key traj1_0123abcd_BASE32SECRET \
  --keep-alive-interval 50
```

Desktop:

```bash
cargo run -p trajectory-desktop
```

Android:

```bash
scripts/install_android_release.sh
```

Then open the app and paste:

- domain
- access key

## Downloads

GitHub Releases currently publish:

- Linux CLI and desktop bundles
- Windows CLI and desktop bundles
- macOS CLI and desktop bundles
- Android APK
- checksum manifests

Maintainer release flow is documented in [RELEASING.md](RELEASING.md).
Contributor workflow is documented in [CONTRIBUTING.md](CONTRIBUTING.md).

## Tests

Workspace:

```bash
cargo test --workspace
```

Desktop smoke:

```bash
cargo test -p trajectory-desktop
target/release/trajectory-desktop --smoke-test
```

Android emulator smoke:

```bash
scripts/test_android_emulator.sh
```

Basic tunnel check:

```bash
./target/release/trajectory-client \
  --tcp-listen-port 7000 \
  --domain your.domain.example \
  --access-key traj1_0123abcd_BASE32SECRET \
  --keep-alive-interval 50

curl -I --socks5-hostname 127.0.0.1:7000 https://example.com
```

## Deploy

Use the installer script:

```bash
sudo deploy/install_server.sh \
  --domain your.domain.example \
  --server-bin target/release/trajectory-server \
  --admin-bin target/release/trajectory-admin \
  --target-address 127.0.0.1:1080 \
  --client-label phone
```

See [docs/SELF_HOSTING.md](docs/SELF_HOSTING.md) for the full operator flow, optional `hev-socks5-server` installation, and client-key management.

## Licensing

Trajectory is dual-licensed under either:

- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)

Learning Notes:
- A shared core plus thin platform wrappers is the normal open source structure for networked Rust applications that need desktop and mobile clients.
- An operator-facing admin CLI is a better default open source story than making routine key management depend on a server-side TUI session.

Why This Matters:
- The CLI, desktop app, and mobile wrappers all share one transport implementation.
- Server installation and client-key creation now have a scriptable path that is suitable for documentation, automation, and packaging.
