# Releasing Trajectory

Trajectory releases publish CLI bundles, desktop client packages, the Android APK, and checksum manifests. A `v*` tag is user-facing: create it only when the release workflow is expected to publish, and do not treat failed release tags as shipped releases.

## Local Smoke

```bash
cargo test -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --tests
cargo build --release -p trajectory-cli --bins
python3 scripts/package_release.py --target x86_64-unknown-linux-gnu --output-dir dist/local
python3 scripts/ci_smoke_release_bundles.py dist/local
python3 scripts/validate_release_versions.py --release-tag v0.1.48
```

Desktop and Android release gates are run by GitHub Actions. Before tagging, run the platform checks from [CONTRIBUTING.md](CONTRIBUTING.md) for any client you touched.

## Release Assets

| Surface | Asset pattern |
| --- | --- |
| Linux CLI | `trajectory-vVERSION-x86_64-unknown-linux-gnu-cli.tar.gz` |
| Windows CLI | `trajectory-vVERSION-x86_64-pc-windows-msvc-cli.zip` |
| macOS Intel CLI | `trajectory-vVERSION-x86_64-apple-darwin-cli.tar.gz` |
| macOS Apple Silicon CLI | `trajectory-vVERSION-aarch64-apple-darwin-cli.tar.gz` |
| Per-target CLI checksums | `trajectory-vVERSION-TARGET-SHA256SUMS.txt` |
| Windows desktop | `Trajectory_VERSION_x64_portable.zip` |
| Linux desktop | `Trajectory_VERSION_ARCH.deb`, `Trajectory-VERSION-ARCH.rpm`, `Trajectory_VERSION_ARCH.AppImage` |
| macOS desktop | `Trajectory_vVERSION_x86_64-apple-darwin.app.tar.gz`, `Trajectory_vVERSION_aarch64-apple-darwin.app.tar.gz`, optional `.dmg` |
| Android | `trajectory-vVERSION-android.apk` |
| Merged checksums | `vVERSION-SHA256SUMS.txt` |

Windows desktop releases are portable-only. `.msi` and `.exe` installer assets are rejected by `scripts/validate_release_assets.py`.

## GitHub Flow

1. Make sure every version file matches the tag:

   ```bash
   python3 scripts/validate_release_versions.py --release-tag v0.1.48
   ```

2. Commit the release changes with a Conventional Commit.

3. Create an annotated tag:

   ```bash
   git tag -a v0.1.48 -m "v0.1.48"
   ```

4. Push `main` and the tag:

   ```bash
   git push origin main
   git push origin v0.1.48
   ```

5. Watch the `CI` and `Release` workflows until both pass. The release workflow validates versions, builds every asset, smoke-tests CLI/desktop/Android packages, verifies Android APK metadata/signing, validates the final asset matrix, deletes stale publishable assets from an existing release for the same tag, and uploads the release.

If a release workflow fails after a tag is pushed, prefer fixing forward with the next version tag. Only delete a remote tag when you are certain no user or automation should depend on it.

## Secrets

Release and live smoke jobs read secrets from GitHub Actions. Do not print or commit:

- Trajectory access keys
- live DNS domains or private resolver lists
- Android signing keystores or passwords
- server credentials
- resolver SOCKS proxy credentials

See [docs/CI_E2E.md](docs/CI_E2E.md) for the secret-gated e2e configuration.
