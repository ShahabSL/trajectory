# Trajectory Desktop

`trajectory-desktop` is the desktop wrapper around the shared Rust transport core.

It provides:

- tunnel configuration
- resolver management
- start/stop controls
- local endpoint status
- diagnostic logging
- the same default public-resolver fallback used by the CLI and mobile bridge

Build the release binary:

```bash
cargo build --release -p trajectory-desktop
```

Run the smoke test:

```bash
cargo test -p trajectory-desktop
target/release/trajectory-desktop --smoke-test
```

Release bundles for end users are produced by `scripts/package_release.py` locally and by `.github/workflows/release.yml` in CI.

Why this matters: the desktop app is a thin product wrapper around `trajectory-core`, not a separate transport implementation.
