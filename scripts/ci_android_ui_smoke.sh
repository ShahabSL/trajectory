#!/usr/bin/env bash
set -euo pipefail

apk="${1:?usage: ci_android_ui_smoke.sh <apk> [artifact-dir]}"
artifact_dir="${2:-${RUNNER_TEMP:-/tmp}/trajectory-android-ui}"
package_name="app.trajectory.android"

mkdir -p "$artifact_dir"
test -f "$apk"

adb wait-for-device
adb install -r "$apk"
adb shell pm grant "$package_name" android.permission.POST_NOTIFICATIONS >/dev/null 2>&1 || true

activity="$(adb shell cmd package resolve-activity --brief "$package_name" | tail -n 1 | tr -d '\r')"
if [[ -z "$activity" || "$activity" != "$package_name/"* ]]; then
  echo "could not resolve launcher activity for $package_name: $activity" >&2
  exit 1
fi

adb shell am force-stop "$package_name"
adb shell logcat -c
adb shell am start -W -n "$activity" > "$artifact_dir/start.txt"
sleep 2

adb exec-out uiautomator dump /dev/tty > "$artifact_dir/main.xml"
adb exec-out screencap -p > "$artifact_dir/main.png"

grep -Fq "Trajectory" "$artifact_dir/main.xml"
grep -Fq "Tunnel" "$artifact_dir/main.xml"
grep -Fq "Resolvers" "$artifact_dir/main.xml"

adb shell input swipe 540 2140 540 620 600
sleep 1
adb exec-out uiautomator dump /dev/tty > "$artifact_dir/bottom.xml"
adb exec-out screencap -p > "$artifact_dir/bottom.png"

grep -Fq "Controls" "$artifact_dir/bottom.xml"
grep -Fq "Start VPN" "$artifact_dir/bottom.xml"
grep -Fq "Stop Trajectory" "$artifact_dir/bottom.xml"

adb logcat -d > "$artifact_dir/logcat.txt"
if grep -E "FATAL EXCEPTION|E AndroidRuntime" "$artifact_dir/logcat.txt"; then
  echo "Android crash detected during UI smoke test" >&2
  exit 1
fi
