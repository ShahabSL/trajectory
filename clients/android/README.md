# Trajectory Android

Planned Android client wrapper for `trajectory-core`.

Recommended direction:

- Android app shell
- `VpnService` integration for full-device tunneling
- Rust core bridged into the app layer

Why this matters: Android should reuse the same transport core as desktop and CLI instead of reimplementing protocol logic.
