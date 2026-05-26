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

screen_size="$(adb shell wm size | tr -d '\r' | awk '/Physical size/ {print $3; exit}')"
if [[ "$screen_size" =~ ^[0-9]+x[0-9]+$ ]]; then
  screen_width="${screen_size%x*}"
  screen_height="${screen_size#*x}"
else
  screen_width=1080
  screen_height=1920
fi

swipe_x=$((screen_width / 2))
swipe_start_y=$((screen_height * 78 / 100))
swipe_end_y=$((screen_height * 28 / 100))

xml_files=("$artifact_dir/main.xml")
for step in middle bottom final; do
  adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
  sleep 1
  adb exec-out uiautomator dump /dev/tty > "$artifact_dir/$step.xml"
  adb exec-out screencap -p > "$artifact_dir/$step.png"
  xml_files+=("$artifact_dir/$step.xml")
done

cat "${xml_files[@]}" > "$artifact_dir/all.xml"
grep -Fq "Controls" "$artifact_dir/all.xml"
grep -Fq "Start VPN" "$artifact_dir/all.xml"
grep -Fq "Stop Trajectory" "$artifact_dir/all.xml"

adb logcat -d > "$artifact_dir/logcat.txt"
if grep -E "FATAL EXCEPTION|E AndroidRuntime" "$artifact_dir/logcat.txt"; then
  echo "Android crash detected during UI smoke test" >&2
  exit 1
fi
