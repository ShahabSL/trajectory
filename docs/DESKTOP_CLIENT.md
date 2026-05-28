# Desktop Client

Trajectory now has a proxy-first desktop client under `clients/desktop`.
It is a Tauri 2 + React shell that starts the existing optimized
`trajectory-client` binary, keeps the access key in the child process
environment, and exposes local SOCKS5 and HTTP proxy endpoints.

## What Works

- Windows, macOS, and Linux desktop shell build path
- profile editing for domain, access key, resolvers, resolver file, resolver
  SOCKS gate, DNS payload, and admission controls
- stable `secure`, `velocity`, and `resilient` transport modes plus
  `frontier` as an explicit experimental mode in Advanced/Settings UI
- local optimized SOCKS5 listener through `--socks-listen`
- local HTTP proxy listener through `--http-listen`
- connect/disconnect lifecycle for `trajectory-client`
- access keys stored by the Rust backend in the OS credential store
- atomic profile metadata writes in the app config directory
- packaged Tauri sidecar staging through `clients/desktop/scripts/stage-sidecar.sh`
- manual system proxy apply/clear for supported desktop OSes; apply requires a
  connected profile and an enabled localhost HTTP listener
- runtime log tail and safe redacted profile export
- LAN binding gate so non-localhost listeners are explicit

## What Is Not Claimed Yet

Whole-device VPN is separate platform work:

- Android requires `VpnService`, a foreground service, `protect()` for control
  sockets, route/DNS configuration, and a packet loop.
- macOS requires a Network Extension packet tunnel target plus entitlement,
  signing, and notarization.
- Windows requires a TUN adapter path such as Wintun and an installer/helper.
- Linux requires `/dev/net/tun`, routing, and either root or `CAP_NET_ADMIN`.

Proxy mode is the shippable client path today. VPN mode should wrap this local
proxy with platform packet adapters once those adapters are implemented and
tested.

## Development Run

Build the transport binary first:

```sh
cargo build --release -p trajectory-cli --bin trajectory-client
```

Run the desktop app:

```sh
TRAJECTORY_CLIENT_BIN="$PWD/target/release/trajectory-client" \
  npm --prefix clients/desktop run tauri -- dev
```

The Rust backend stores profile metadata in the app config directory and stores
access keys in the OS credential store. Paste your tunnel domain and access key
in the Profiles tab, verify resolvers in the Resolvers tab, then connect from
the top bar.

`Frontier` is intentionally available for power users and breakthrough testing,
but the UI labels it experimental. Use `Velocity` when you want the stable fast
profile; use `Frontier` when you accept cohort-sensitive behavior while testing
the highest-ceiling transport profile.

## Browser Preview

For UI-only development without starting the Rust controller:

```sh
npm --prefix clients/desktop run dev
```

Open `http://127.0.0.1:1420`. Browser preview uses a mock runtime and cannot
start `trajectory-client` or access the OS credential store.

## Direct CLI Equivalent

The desktop app launches the optimized SOCKS listener and keeps the raw TCP
listener on an ephemeral loopback port. The HTTP listener is optional. The
desktop app ultimately launches this shape of command:

```sh
TRAJECTORY_ACCESS_KEY="$ACCESS_KEY" target/release/trajectory-client \
  --listen 127.0.0.1:0 \
  --socks-listen 127.0.0.1:7000 \
  --http-listen 127.0.0.1:7001 \
  --domain "$DOMAIN" \
  --resolver 1.1.1.1:53 \
  --resolver 1.0.0.1:53 \
  --resolver 8.8.8.8:53 \
  --resolver 8.8.4.4:53
```

Use `--resolver-file /path/to/dnses.txt` and `--resolver-socks-proxy
127.0.0.1:11092` from the UI when testing through a constrained gate.

## Validation

```sh
npm --prefix clients/desktop ci
npm --prefix clients/desktop run build
npm --prefix clients/desktop run ui:smoke
npm --prefix clients/desktop run tauri:check
npm --prefix clients/desktop run tauri:build
```

For live tunnel validation, use the existing protected CI workflow documented in
`docs/CI_E2E.md`. Live secrets must stay in the protected
`trajectory-live-e2e` environment, not in pull request CI.
