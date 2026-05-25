# Contributing to Trajectory

Trajectory is now a core + CLI Rust workspace. Keep changes focused, keep docs aligned with behavior, and validate the packages you touch.

## Expectations

- Use Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- Keep transport behavior in `crates/trajectory-core` or `crates/trajectory-cli`.
- Update operator docs when commands, install flow, or release packaging changes.
- Do not check in generated build outputs or secrets.

## Common Commands

```bash
cargo fmt --all --check
cargo test -p trajectory-core -p trajectory-cli --lib --bins
cargo test -p trajectory-core --test core_wire
cargo test -p trajectory-cli --test loopback_e2e
cargo clippy -p trajectory-core -p trajectory-cli --all-targets -- -D warnings
cargo build --release -p trajectory-cli --bins
```

Live network e2e is secret-gated in GitHub Actions and must not run on pull requests. See [docs/CI_E2E.md](docs/CI_E2E.md).

## Maintained Surfaces

- `trajectory-client`
- `trajectory-server`
- `trajectory-admin`
- `deploy/` systemd/install assets
- `scripts/package_release.py`

Desktop, mobile, archive, and vendored legacy code are not maintained surfaces in this repo.
