# Releasing Trajectory

Trajectory releases publish CLI bundles only.

## Local Smoke

```bash
cargo test -p trajectory-core -p trajectory-cli --tests
cargo build --release -p trajectory-cli --bins
python3 scripts/package_release.py --target x86_64-unknown-linux-gnu --output-dir dist/local
```

## Release Contents

Each bundle includes:

- `trajectory-client`
- `trajectory-server`
- `trajectory-admin`
- project README and build metadata

Artifact names:

- `trajectory-vVERSION-TARGET-cli.tar.gz`
- `trajectory-vVERSION-TARGET-cli.zip`
- `trajectory-vVERSION-TARGET-SHA256SUMS.txt`

## GitHub Flow

Push a tag such as `v0.1.0`, or run the `Release` workflow manually with an existing tag.

The workflow builds Linux, Windows, and macOS CLI bundles, merges checksum manifests, and creates or updates the GitHub Release.
