# Trajectory Android

`clients/android` contains the native Android client for the shared Rust tunnel core.

Architecture:

- Jetpack Compose UI for the client
- DataStore-backed settings persistence
- UniFFI-generated Kotlin bindings in `app/src/main/java/uniffi/trajectorymobile`
- the Rust mobile bridge crate in `crates/trajectory-mobile`
- Gradle-driven Rust JNI packaging via `cargo ndk`
- Android `VpnService` for the device traffic path
- `hev-socks5-tunnel` as the tun-to-SOCKS bridge

What works repo-side:

- real Android project structure
- real Compose UI
- real Kotlin code wired to the generated Rust bindings
- persistent configuration model
- required access key stored locally with the rest of the tunnel profile
- real VPN service path for device traffic
- tunnel start/stop/status/log control path
- Gradle builds the Rust bridge for `arm64-v8a` and `x86_64`
- Gradle builds and packages `libhev-socks5-tunnel.so` for Android
- debug APK assembly packages both native libraries into the app
- emulator smoke tests can inject an access key, approve VPN consent, and verify traffic

What still depends on external mobile tooling:

- a full release pipeline for signed APK/AAB publishing
- installed Android SDK, NDK, and `cargo-ndk`

Build locally once the Android SDK, NDK, and `cargo-ndk` are available:

```bash
export ANDROID_SDK_ROOT=/path/to/android-sdk
export ANDROID_HOME=$ANDROID_SDK_ROOT
export JAVA_HOME=/path/to/jdk17-or-newer
gradle -p clients/android assembleDebug
```

The resulting debug APK is written to:

```text
clients/android/app/build/outputs/apk/debug/app-debug.apk
```

Run the emulator smoke test once an Android emulator is booted and visible to `adb`:

```bash
scripts/test_android_emulator.sh
```

The smoke test verifies:

- the APK installs cleanly
- the app launches with deterministic access-key/config injection plus autostart
- the VPN permission flow is accepted automatically in the emulator
- the local SOCKS listener opens on `127.0.0.1:7000`
- the VPN service starts and stays in foreground mode
- browser traffic increases the VPN counters

Why this matters: the Android app is no longer just a controller around the Rust client. It now owns a real `VpnService` plus a tun-to-SOCKS bridge, which is the standard way to carry normal app traffic on Android.
