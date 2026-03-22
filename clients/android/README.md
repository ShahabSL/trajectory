# Trajectory Android

`clients/android` contains the native Android client for the shared Rust tunnel core.

## Architecture

- Jetpack Compose UI
- DataStore-backed settings persistence
- UniFFI-generated Kotlin bindings in `app/src/main/java/uniffi/trajectorymobile`
- the Rust mobile bridge crate in `crates/trajectory-mobile`
- Gradle-driven Rust JNI packaging via `cargo-ndk`
- Android `VpnService` for device traffic
- `hev-socks5-tunnel` as the tun-to-SOCKS bridge

## What works

- real Android project structure
- real Compose UI
- real Kotlin code wired to the generated Rust bindings
- persistent configuration model
- required access key stored locally with the rest of the tunnel profile
- proxy mode
- VPN mode
- tunnel start/stop/status/log control path
- Gradle builds the Rust bridge for `arm64-v8a` and `x86_64`
- Gradle builds and packages `libhev-socks5-tunnel.so` for Android
- debug and release APK assembly package both native libraries into the app
- emulator smoke tests can inject an access key, approve VPN consent, and verify traffic

During release hardening, both proxy and VPN modes were validated on a physical Android device.

GitHub Releases now attach the built Android APK as a release asset. Local maintainers can still build and sideload it directly with the scripts below.

## Build the release APK

```bash
scripts/build_android_release.sh
```

The helper isolates Gradle state automatically so it does not depend on a healthy `~/.gradle` daemon registry. Set `GRADLE_USER_HOME` yourself if you want persistent local Gradle caches across repeated builds.

The output APK is:

```text
clients/android/app/build/outputs/apk/release/app-release.apk
```

Install onto a connected device:

```bash
scripts/install_android_release.sh
```

Run the emulator smoke test once an Android emulator is booted and visible to `adb`:

```bash
scripts/test_android_emulator.sh
```

## External tooling still required

- Android SDK
- Android NDK
- `cargo-ndk`
- Java 17 or newer

Why this matters: the Android app is not just a controller around the Rust client. It owns a real `VpnService` plus a tun-to-SOCKS bridge, which is the standard way to carry normal app traffic on Android.
