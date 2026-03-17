# Trajectory Desktop

`trajectory-desktop` is the first end-user client for the shared Rust transport core.

It provides:

- tunnel configuration
- resolver management
- start/stop controls
- local endpoint status
- diagnostic logging

Why this matters: the desktop app is a thin product wrapper around `trajectory-core`, not a separate transport implementation.

Release bundles for end users are produced by `scripts/package_release.py` locally and by `.github/workflows/release.yml` in CI.
