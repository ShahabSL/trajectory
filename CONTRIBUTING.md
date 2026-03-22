# Contributing to Trajectory

Trajectory is a multi-surface Rust workspace. Keep changes focused, keep docs aligned with behavior, and validate the surface you touched before sending it upstream.

## Development expectations

- use Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`)
- prefer shared fixes in `crates/trajectory-core` over platform-specific drift
- update operator or client docs when install, release, or user-facing behavior changes
- do not check in generated build outputs

## Common commands

Workspace tests:

```bash
cargo test --workspace
```

CLI and desktop release builds:

```bash
cargo build --release -p trajectory-cli --bins -p trajectory-desktop
```

Desktop smoke test:

```bash
target/release/trajectory-desktop --smoke-test
```

Android release APK:

```bash
scripts/build_android_release.sh
```

Android install onto a connected device:

```bash
scripts/install_android_release.sh
```

## Operator-facing changes

If you touch server install, credential management, or release packaging:

- keep `deploy/install_server.sh` and `docs/SELF_HOSTING.md` aligned
- keep `scripts/package_release.py` and `RELEASING.md` aligned
- validate `trajectory-admin --help`, `trajectory-server --help`, and `trajectory-client --help`

## Release-ready client surfaces

Maintained release surfaces from this repository are:

- CLI client
- CLI server
- CLI admin tool
- desktop client
- Android app

The iOS app source ships, but iOS archives still require an Apple-hosted build environment.

Learning Notes:
- Repositories stay maintainable when operator tooling and docs are treated as first-class product surfaces, not as cleanup tasks.
- Thin platform wrappers keep transport fixes concentrated in one shared core instead of fragmenting across clients.

Why This Matters:
- Contributors need one obvious path to build, test, and package the maintained surfaces.
- Most release regressions are docs drift; putting those files in the contribution checklist cuts that risk.
