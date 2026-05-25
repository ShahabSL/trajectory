# Android Client

Trajectory has a native Android client under `clients/android`.

The Android app packages the existing Rust `trajectory-client` binary as an
arm64 and x86_64 native executable and runs it from a foreground service. It
exposes local SOCKS and HTTP proxy ports on `127.0.0.1`.

## What Works

- Kotlin/Android app shell
- foreground proxy service
- app-private profile metadata
- access key encryption through Android Keystore-backed AES-GCM storage
- packaged `trajectory-client` arm64 sidecar
- packaged `trajectory-client` x86_64 sidecar for emulator CI
- experimental Android VPN mode through `VpnService`
- TUN-to-SOCKS packet bridge through the packaged Rust `trajectory_vpn_bridge`
  JNI library
- DNS over TCP through the Trajectory tunnel while VPN mode is active
- loop protection through Android `addDisallowedApplication(packageName)` so
  the sidecar's resolver/control sockets bypass the TUN
- SOCKS listener on `127.0.0.1:7000`
- HTTP listener on `127.0.0.1:7001`
- resolver list and optional resolver SOCKS gate
- access key passed through `TRAJECTORY_ACCESS_KEY`, not argv

## What Is Not Claimed Yet

Android VPN mode is real but still intentionally conservative:

- non-DNS UDP is not claimed until Trajectory has a tested UDP gateway path
- IPv6 is disabled by default until leak tests pass on devices
- always-on/lockdown VPN is opted out in the manifest until kill-switch behavior
  is tested on real devices
- OEM background restrictions, sleep/wake, network handoff, and Play policy
  review still require device-lab validation

## Build

```sh
ANDROID_HOME="$PWD/.tooling/android-sdk" \
ANDROID_SDK_ROOT="$PWD/.tooling/android-sdk" \
ANDROID_NDK_HOME="$PWD/.tooling/android-sdk/ndk/29.0.14206865" \
  ./clients/android/gradlew -p clients/android :app:assembleDebug :app:assembleRelease --no-daemon
```

The APKs are written to:

```text
clients/android/app/build/outputs/apk/debug/app-debug.apk
clients/android/app/build/outputs/apk/release/app-release-unsigned.apk
```

The release APK is unsigned unless a signing configuration is added through
protected release secrets.

## Product Boundary

Proxy mode is stream-level and opt-in per app. VPN mode is packet-level and must
not be called a complete privacy VPN until route, DNS, UDP, IPv6,
loop-protection, and kill-switch behavior are tested on real Android devices.
