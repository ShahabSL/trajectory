# Client Apps Roadmap

Trajectory ships a Rust engine, CLI binaries, a proxy-first Tauri desktop
client under `clients/desktop`, and an Android client with proxy mode plus an
experimental `VpnService` packet path. The CLI supports a SOCKS-compatible
listener and an HTTP proxy listener.

## Build In This Repo First

1. Reusable client engine API
   - start
   - stop
   - status snapshot
   - structured event stream
   - profile update
   - current desktop bridge: implemented by supervising the CLI sidecar

2. Local proxy adapters
   - raw TCP listener for existing behavior: done
   - SOCKS-compatible local proxy through server `socks5-direct`: done
   - HTTP CONNECT and absolute-form `http://...` proxy listener: done
   - optional local proxy auth for LAN sharing: not implemented

3. Profile schema
   - tunnel domain
   - access key reference
   - resolver list
   - resolver SOCKS gate
   - local SOCKS/HTTP ports
   - LAN sharing controls

4. Structured diagnostics
   - admitted resolvers
   - RTT/loss/goodput
   - active/quarantined state
   - upload/download counters
   - scrubbed export for bug reports

## Platform Clients

| Platform | Status | Recommended App | VPN Path | Proxy Path |
| --- | --- | --- | --- | --- |
| Windows | desktop proxy client implemented; VPN planned | Tauri desktop shell + Rust service/helper | Wintun or WireGuardNT-style adapter | SOCKS5 + HTTP CONNECT |
| macOS | desktop proxy client implemented; VPN planned | Tauri shell + Network Extension target later | Network Extension packet tunnel | SOCKS5 + HTTP CONNECT |
| Linux | desktop proxy client implemented; VPN planned | Tauri shell + Rust daemon/helper | `/dev/net/tun` with `CAP_NET_ADMIN` | SOCKS5 + HTTP CONNECT |
| Android | proxy client implemented; experimental TCP+DNS VPN implemented | Kotlin app + Rust sidecar + Rust JNI bridge | `VpnService` + tun2proxy bridge to local SOCKS | SOCKS5 + HTTP CONNECT |

VPN mode is not just a UI toggle. It changes the product from stream proxying to
IP packet handling, platform permissions, signing, installers, and routing
policy. Android now has the first packet path; desktop VPN remains platform
helper/provider work.

On Android, app-wide capture generally requires `VpnService`; a local proxy alone
only helps apps that are explicitly configured to use that proxy.

## UI Structure

The UI should be quiet, monochrome, and operational.

Tabs:

- Status: connect toggle, current profile, endpoints, health, throughput
- Profiles: add/import/export, domain, key status, resolver set
- Proxy: SOCKS5, HTTP CONNECT, local ports, copy buttons
- LAN Sharing: bind address, auth, allowlist, warning state
- VPN: platform availability, permission/admin flow, full/split routes
- Resolvers: active/standby/quarantine, RTT, loss, goodput
- Diagnostics: logs, counters, scrubbed export
- Settings: launch at login, updates, advanced transport knobs

Transport modes are user intent contracts:

- `secure`: default conservative baseline
- `velocity`: stable fast profile for normal networks
- `resilient`: compatibility-first profile for difficult networks
- `frontier`: experimental highest-ceiling profile, available in clients but
  visually separated from stable modes

Default safety:

- bind to `127.0.0.1`
- require explicit confirmation for LAN binding
- require auth when listening on LAN
- never display full access keys after save
- redact secrets from exported diagnostics

## Prototype

The live desktop implementation is in `clients/desktop`. Open
[client-ui-prototype.html](client-ui-prototype.html) only as the historical
visual prototype.

## Test Strategy

Desktop proxy MVP:

- Rust unit tests for config/profile parsing
- engine lifecycle tests: start, stop, reconnect, invalid key
- proxy tests for SOCKS5 and HTTP CONNECT
- Playwright UI tests for all tabs and form states
- desktop browser-preview smoke tests that render Status/Settings, select
  Frontier, and upload screenshots
- desktop packaged smoke tests that inspect installable artifacts, launch the
  real Tauri bundle/AppImage/MSI-extracted app where the platform allows it, and
  require packaged backend setup, webview page-load, frontend IPC, and backend
  state readiness markers
- OS build matrix: Windows, macOS, Linux

VPN clients:

- Android JVM tests for profile validation and sidecar command construction
- Android manifest/static checks for `VpnService`, foreground-service type,
  native bridge packaging, and consent flow
- Android emulator UI smoke tests that install the APK, launch the main screen,
  assert the UI tree, scroll to lower controls, select Frontier experimental mode,
  run the packaged native sidecar with `--help`, apply screenshot visual-density
  checks, and upload screenshots
- platform-specific simulator/device tests
- permission/entitlement validation
- route table tests
- DNS leak tests
- UDP/IPv6 behavior tests
- kill-switch behavior tests once implemented

Release gates:

- packaged CLI archives are extracted and run through raw TCP, SOCKS5, and HTTP
  CONNECT loopbacks
- desktop bundles must pass UI preview smoke plus packaged backend/page-load,
  frontend IPC, and backend state readiness from the distributable app
- Android debug and signed release APKs must install, run the sidecar, and render
  visually nonblank screens on emulator CI
- release assets must match the expected set, per-target SHA256 manifests, and
  the merged SHA256 manifest
- no crash on missing server, wrong key, dead resolver, network loss
- LAN sharing disabled by default
- all proxy modes pass e2e against a live protected test server
