# Contributing to Trajectory

Trajectory is a restricted-license Rust transport with first-class CLI, desktop, Android, deployment, and release-packaging surfaces. Keep changes focused, keep docs aligned with behavior, and validate the packages you touch.

## Expectations

- Use Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- Keep transport behavior in `crates/trajectory-core`, `crates/trajectory-cli`, or `crates/trajectory-vpn-bridge`.
- Update operator docs when commands, install flow, client UX, or release packaging changes.
- Do not check in generated build outputs, private resolver lists, access keys, signing material, or live endpoint credentials.
- Treat the license as source-available/restricted unless Shahab Lavasani explicitly changes it.

## Maintained Surfaces

- `trajectory-client`
- `trajectory-server`
- `trajectory-admin`
- `trajectory-vpn-bridge`
- `clients/desktop`
- `clients/android`
- `deploy/` systemd/install assets
- `scripts/` packaging, smoke, benchmark, and support tools
- `.github/workflows/` release and CI automation
- `docs/` and repository presentation

## Common Commands

Fast Rust checks:

```bash
cargo fmt --all --check
cargo test -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --lib --bins
cargo test -p trajectory-core --test core_wire
cargo test -p trajectory-cli --test loopback_e2e
cargo clippy -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --all-targets -- -D warnings
cargo build --release -p trajectory-cli --bins
```

Desktop checks:

```bash
npm --prefix clients/desktop ci
npm --prefix clients/desktop run build
npm --prefix clients/desktop run ui:smoke -- --preview /tmp/trajectory-desktop-ui
npm --prefix clients/desktop run tauri:check
```

Android checks:

```bash
ANDROID_HOME="$HOME/Android/Sdk" ANDROID_SDK_ROOT="$HOME/Android/Sdk" \
  ./clients/android/gradlew -p clients/android \
  :app:assembleDebug :app:assembleRelease :smokeprobe:assembleDebug :app:testDebugUnitTest --no-daemon

scripts/ci_android_vpn_static_checks.sh clients/android/app/build/outputs/apk/release/app-release-unsigned.apk
```

Release validation:

```bash
python scripts/validate_release_versions.py --release-tag v0.1.48
python scripts/package_release.py --target x86_64-unknown-linux-gnu --output-dir dist/local
python scripts/ci_smoke_release_bundles.py dist/local
```

Live network e2e is secret-gated in GitHub Actions and must not run on pull requests. See [docs/CI_E2E.md](docs/CI_E2E.md).

## Generated Files

Do not commit local build directories such as `target/`, `dist/`, `clients/desktop/dist/`, `clients/desktop/src-tauri/target/`, `clients/android/app/build/`, or `clients/android/smokeprobe/build/`.

Brand assets are generated from `assets/branding/trajectory-logo.png`; when the logo changes, run `python3 scripts/generate_brand_assets.py` before opening a PR.
