# Trajectory Desktop

`trajectory-desktop` is the first end-user client for the shared Rust transport core.

It provides:

- tunnel configuration
- resolver management
- start/stop controls
- local endpoint status
- diagnostic logging

Why this matters: the desktop app is a thin product wrapper around `trajectory-core`, not a separate transport implementation.
