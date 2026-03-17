# Trajectory Android

`clients/android` contains the native Android client for the shared Rust tunnel core.

Architecture:

- Jetpack Compose UI for the control surface
- DataStore-backed settings persistence
- UniFFI-generated Kotlin bindings in `app/src/main/java/uniffi/trajectorymobile`
- the Rust mobile bridge crate in `crates/trajectory-mobile`
- Gradle-driven Rust JNI packaging via `cargo ndk`

What works repo-side:

- real Android project structure
- real Compose UI
- real Kotlin code wired to the generated Rust bindings
- persistent configuration model
- tunnel start/stop/status/log control path
- Gradle builds the Rust bridge for `arm64-v8a` and `x86_64`
- debug APK assembly packages `libtrajectory_mobile.so` into the app

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

Why this matters: the app code is real, the Kotlin layer calls the shared Rust tunnel controller, and the APK now bundles the Rust mobile bridge instead of stopping at UI scaffolding.
