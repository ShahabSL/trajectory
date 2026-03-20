# Trajectory iOS

`clients/ios` contains the native iPhone client for the shared Rust tunnel core.

Architecture:

- SwiftUI app UI
- UniFFI-generated Swift bindings in `TrajectoryMobileApp/Sources/Generated`
- XcodeGen-compatible project description in `project.yml`
- Packet Tunnel extension target layout for the future Network Extension path

What works repo-side:

- real SwiftUI application sources
- real view model wired to the generated Rust bindings
- persistent configuration path via `@AppStorage`
- required access key stored locally with the tunnel profile
- packet-tunnel extension source and project structure
- generated Swift bindings expose the same authenticated mobile controller used on Android

What still depends on external Apple tooling:

- Xcode / xcodebuild
- Swift toolchain on macOS
- code signing and Apple entitlements
- iOS device/simulator runtime

Why this matters: the shared Rust core and Swift bindings are in place, but this Linux host cannot compile or sign iOS targets.
