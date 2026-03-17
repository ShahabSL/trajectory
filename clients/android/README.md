# Trajectory Android

`clients/android` contains the native Android client for the shared Rust tunnel core.

Architecture:

- Jetpack Compose UI for the control surface
- DataStore-backed settings persistence
- UniFFI-generated Kotlin bindings in `app/src/main/java/uniffi/trajectorymobile`
- the Rust mobile bridge crate in `crates/trajectory-mobile`

What works repo-side:

- real Android project structure
- real Compose UI
- real Kotlin code wired to the generated Rust bindings
- persistent configuration model
- tunnel start/stop/status/log control path
- local debug APK builds when an Android SDK and Gradle runtime are available

What still depends on external mobile tooling:

- packaged Android `.so` builds for device ABIs
- a full release pipeline for signed APK/AAB publishing

Build locally once the Android SDK and Gradle runtime are available:

```bash
export ANDROID_SDK_ROOT=/path/to/android-sdk
export ANDROID_HOME=$ANDROID_SDK_ROOT
gradle -p clients/android assembleDebug
```

Why this matters: the app code is real and targets the shared Rust transport, and the project now builds into a debug APK when the Android toolchain is present.
