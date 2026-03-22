# Releasing Trajectory

Trajectory ships end-user downloads through GitHub Releases. Maintainers do not need to build every desktop or CLI platform locally.

## What gets published

Each GitHub Release currently publishes portable bundles for:

- Linux CLI
- Linux desktop
- Windows CLI
- Windows desktop
- macOS CLI
- macOS desktop
- Android APK

Each platform build also emits checksum manifests, and the workflow merges them into one top-level checksum file.

The CLI bundles include:

- `trajectory-client`
- `trajectory-server`
- `trajectory-admin`

iOS artifacts are not part of the GitHub Release workflow.

## Local packaging smoke test

Build the Linux artifacts locally:

```bash
python3 scripts/package_release.py --target x86_64-unknown-linux-gnu --output-dir dist/local
```

Verify checksums:

```bash
(cd dist/local && sha256sum -c trajectory-vVERSION-x86_64-unknown-linux-gnu-SHA256SUMS.txt)
```

## Automated release flow

There are two supported release paths:

1. Push a release tag such as `v0.1.0`
2. Manually run the `Release` workflow from GitHub Actions with an existing tag

Tag-based release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

Manual release:

- open `Actions`
- run `Release`
- provide `release_tag`
- choose draft/prerelease behavior

## Workflow behavior

The release workflow:

1. builds native artifacts on Ubuntu, Windows, and macOS runners
2. packages CLI and desktop bundles for each target
3. uploads them as workflow artifacts
4. merges per-target checksum manifests
5. creates or updates the GitHub Release with all assets attached

## Release artifact naming

Artifact names follow this shape:

- `trajectory-vVERSION-TARGET-cli.tar.gz`
- `trajectory-vVERSION-TARGET-desktop.tar.gz`
- `trajectory-vVERSION-TARGET-cli.zip`
- `trajectory-vVERSION-TARGET-desktop.zip`

Checksum manifests:

- `trajectory-vVERSION-TARGET-SHA256SUMS.txt`
- `vVERSION-SHA256SUMS.txt` on the published GitHub Release

## Android releases

The release workflow also builds and uploads the Android APK. Local maintainers can still build it directly:

```bash
scripts/build_android_release.sh
```

The helper isolates Gradle state automatically by default so it is less sensitive to machine-local daemon registry issues. Set `GRADLE_USER_HOME` if you want persistent local Gradle caches.

The output APK is:

```text
clients/android/app/build/outputs/apk/release/app-release.apk
```

You can install it onto a connected device with:

```bash
scripts/install_android_release.sh
```

## Signing and notarization

The current pipeline produces portable archives. Code signing and notarization are not configured yet.

Why this matters: portable archives are the most reliable cross-platform starting point for an open source Rust workspace, and they keep the release pipeline deterministic while leaving room for native installers later.
