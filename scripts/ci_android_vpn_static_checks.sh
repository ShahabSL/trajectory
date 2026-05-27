#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/clients/android/app/src/main/AndroidManifest.xml"
main_activity="$repo_root/clients/android/app/src/main/java/app/trajectory/android/MainActivity.kt"
vpn_service="$repo_root/clients/android/app/src/main/java/app/trajectory/android/TrajectoryVpnService.kt"
vpn_bridge="$repo_root/clients/android/app/src/main/java/app/trajectory/android/TrajectoryVpnBridge.kt"

grep -q 'android.permission.BIND_VPN_SERVICE' "$manifest"
grep -q 'android.permission.FOREGROUND_SERVICE_SPECIAL_USE' "$manifest"
grep -q 'android:foregroundServiceType="specialUse"' "$manifest"
grep -q 'android.app.PROPERTY_SPECIAL_USE_FGS_SUBTYPE' "$manifest"
grep -q 'android.net.VpnService.SUPPORTS_ALWAYS_ON' "$manifest"
grep -q 'android:value="false"' "$manifest"
grep -q 'VpnService.prepare' "$main_activity"
grep -q 'Builder()' "$vpn_service"
grep -q 'addDisallowedApplication(packageName)' "$vpn_service"
grep -q 'TrajectoryVpnBridge.run' "$vpn_service"
grep -q 'System.loadLibrary("trajectory_vpn_bridge")' "$vpn_bridge"

if [[ $# -gt 0 ]]; then
  apk="$1"
  test -f "$apk"
  apk_listing="$(unzip -Z1 "$apk")"
  grep -Fxq 'lib/arm64-v8a/libtrajectory_client.so' <<<"$apk_listing"
  grep -Fxq 'lib/arm64-v8a/libtrajectory_vpn_bridge.so' <<<"$apk_listing"
  grep -Fxq 'lib/x86_64/libtrajectory_client.so' <<<"$apk_listing"
  grep -Fxq 'lib/x86_64/libtrajectory_vpn_bridge.so' <<<"$apk_listing"
fi
