# Slipstream Porting Gap

Trajectory now has two layers:

- a Rust CLI/runtime wrapper that can deploy and benchmark the upstream Slipstream core
- the original Rust MVP transport for experimentation

It is still not a complete Rust-native port of Slipstream.

## Missing for Full Parity

- Replace the upstream C transport core with a Rust-native QUIC multipath implementation or bindings layer that preserves the same behavior.
- Port Slipstream's congestion-control changes for rate-limited DNS resolvers.
- Port the custom QUIC polling frame behavior and keep-alive semantics.
- Port the server-side path spoofing, queueing, and out-of-order handling behavior described in upstream protocol docs.
- Port the high-performance DNS codec/base32 path instead of the current Rust MVP framing.
- Add behavior-level compatibility tests so Trajectory and upstream Slipstream can be validated against the same scenarios.

## Current Evidence

- `scripts/benchmark_public.py` can deploy either Trajectory engine or upstream Slipstream to the VPS and compare them through a public resolver.
- The `slipstream` engine in Trajectory is expected to track upstream behavior closely because it executes the upstream client/server binaries under the Rust CLI.
- The `mvp` engine remains materially slower than upstream Slipstream in prior public-resolver tests.

## Next Engineering Steps

1. Stabilize the wrapper mode with deployment fixtures and parity benchmarks against the upstream CLI.
2. Carve the upstream transport surface behind narrower Rust traits so the runtime can swap from wrapper mode to native mode incrementally.
3. Port one subsystem at a time, starting with DNS framing/socket loop, then QUIC polling semantics, then congestion-control/path logic.
4. Expand the benchmark harness to cover upload, download, and loss/retransmission cases.
