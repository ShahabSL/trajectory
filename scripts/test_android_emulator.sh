#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APK_PATH="${APK_PATH:-$ROOT_DIR/clients/android/app/build/outputs/apk/debug/app-debug.apk}"
PACKAGE_NAME="cc.sevenb.trajectorymobile"
ACTIVITY_NAME="$PACKAGE_NAME/.MainActivity"
AUTOSTART_EXTRA="cc.sevenb.trajectorymobile.extra.AUTOSTART"

if ! command -v adb >/dev/null 2>&1; then
  echo "adb is required on PATH" >&2
  exit 1
fi

if [[ ! -f "$APK_PATH" ]]; then
  echo "APK not found at $APK_PATH" >&2
  exit 1
fi

adb wait-for-device >/dev/null
until [[ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]]; do
  sleep 2
done

adb install -r "$APK_PATH" >/dev/null
adb shell am force-stop "$PACKAGE_NAME" >/dev/null
adb logcat -c

adb shell am start -n "$ACTIVITY_NAME" --ez "$AUTOSTART_EXTRA" true >/dev/null
sleep 6

service_dump="$(adb shell dumpsys activity services "$PACKAGE_NAME")"
if ! grep -q 'TrajectoryTunnelService' <<<"$service_dump"; then
  echo "Foreground service did not start" >&2
  echo "$service_dump" >&2
  exit 1
fi

if ! grep -q 'isForeground=true' <<<"$service_dump"; then
  echo "Tunnel service is not running in foreground mode" >&2
  echo "$service_dump" >&2
  exit 1
fi

port_dump="$(adb shell ss -ltnp | grep ':7000' || true)"
if ! grep -q '127.0.0.1:7000' <<<"$port_dump"; then
  echo "Tunnel is not listening on 127.0.0.1:7000" >&2
  echo "$port_dump" >&2
  exit 1
fi

adb shell input keyevent 3 >/dev/null
sleep 5

service_after_home="$(adb shell dumpsys activity services "$PACKAGE_NAME")"
if ! grep -q 'TrajectoryTunnelService' <<<"$service_after_home"; then
  echo "Foreground service died after backgrounding the app" >&2
  echo "$service_after_home" >&2
  exit 1
fi

log_dump="$(adb logcat -d | grep -E 'TrajectoryTunnelSvc|TrajectoryViewModel|AndroidRuntime|FATAL EXCEPTION' || true)"
if grep -qE 'AndroidRuntime|FATAL EXCEPTION' <<<"$log_dump"; then
  echo "Crash detected in logcat" >&2
  echo "$log_dump" >&2
  exit 1
fi

if ! grep -q 'Tunnel controller started successfully' <<<"$log_dump"; then
  echo "Tunnel controller never reported a successful start" >&2
  echo "$log_dump" >&2
  exit 1
fi

echo "Android emulator smoke test passed"
