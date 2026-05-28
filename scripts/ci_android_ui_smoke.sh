#!/usr/bin/env bash
set -euo pipefail

apk="${1:?usage: ci_android_ui_smoke.sh <apk> [artifact-dir]}"
artifact_dir="${2:-${RUNNER_TEMP:-/tmp}/trajectory-android-ui}"
package_name="app.trajectory.android"

mkdir -p "$artifact_dir"
test -f "$apk"

wait_for_boot_completed() {
  for _ in $(seq 1 90); do
    if [[ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]]; then
      return 0
    fi
    sleep 1
  done
  echo "Timed out waiting for Android boot completion" >&2
  return 1
}

install_apk() {
  local attempt
  for attempt in 1 2 3; do
    if adb install -r "$apk" > "$artifact_dir/install-attempt-${attempt}.txt" 2>&1; then
      return 0
    fi
    adb uninstall "$package_name" >/dev/null 2>&1 || true
    sleep 2
  done
  echo "Android APK install failed after retries" >&2
  return 1
}

record_failure_artifacts() {
  set +e
  timeout 10s adb logcat -d > "$artifact_dir/logcat.txt" 2>/dev/null
  timeout 10s adb shell dumpsys window > "$artifact_dir/window.txt" 2>/dev/null
  timeout 10s adb shell dumpsys activity activities > "$artifact_dir/activities.txt" 2>/dev/null
  timeout 10s adb shell dumpsys package "$package_name" > "$artifact_dir/package.txt" 2>/dev/null
  timeout 10s adb exec-out uiautomator dump /dev/tty > "$artifact_dir/failure.raw.xml" 2>/dev/null
  timeout 10s adb exec-out screencap -p > "$artifact_dir/failure.png" 2>/dev/null
}

on_exit() {
  local code=$?
  if [[ "$code" -ne 0 ]]; then
    record_failure_artifacts
  fi
  exit "$code"
}

trap on_exit EXIT

adb wait-for-device
wait_for_boot_completed
adb uninstall "$package_name" >/dev/null 2>&1 || true
install_apk
if adb shell pm grant "$package_name" android.permission.POST_NOTIFICATIONS > "$artifact_dir/notification-permission.txt" 2>&1; then
  echo "POST_NOTIFICATIONS granted by smoke harness" >> "$artifact_dir/notification-permission.txt"
else
  echo "POST_NOTIFICATIONS grant failed or is not applicable on this API level" >> "$artifact_dir/notification-permission.txt"
fi

activity="$(adb shell cmd package resolve-activity --brief "$package_name" | tail -n 1 | tr -d '\r')"
if [[ -z "$activity" || "$activity" != "$package_name/"* ]]; then
  echo "could not resolve launcher activity for $package_name: $activity" >&2
  exit 1
fi

adb shell am force-stop "$package_name"
adb shell logcat -c
adb shell am start -W -n "$activity" > "$artifact_dir/start.txt"
sleep 2

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

dump_ui_tree_raw() {
  local output="$1"
  local seconds="${2:-10}"
  for _ in 1 2; do
    if timeout "${seconds}s" adb exec-out uiautomator dump /dev/tty > "$output"; then
      return 0
    fi
    sleep 1
  done
  return 1
}

capture_screenshot() {
  local output="$1"
  timeout 20s adb exec-out screencap -p > "$output"
}

dump_screen() {
  local name="$1"
  dismiss_platform_dialogs
  dump_ui_tree_raw "$artifact_dir/$name.raw.xml"
  python3 - "$artifact_dir/$name.raw.xml" "$artifact_dir/$name.xml" <<'PY'
import sys
from pathlib import Path

raw = Path(sys.argv[1]).read_text(errors="replace")
end = raw.find("</hierarchy>")
if end < 0:
    raise SystemExit("uiautomator XML did not contain </hierarchy>")
Path(sys.argv[2]).write_text(raw[: end + len("</hierarchy>")])
PY
  capture_screenshot "$artifact_dir/$name.png"
}

dismiss_platform_dialogs() {
  local probe="$artifact_dir/platform-dialog-probe.raw.xml"
  local action
  for _ in 1 2 3; do
    if ! timeout 5s adb exec-out uiautomator dump /dev/tty > "$probe" 2>/dev/null; then
      return 0
    fi
    if ! action="$(python3 - "$probe" "$package_name" <<'PY'
import re
import sys
import xml.etree.ElementTree as ET

raw = open(sys.argv[1], errors="replace").read()
package_name = sys.argv[2]
end = raw.find("</hierarchy>")
if end < 0:
    raise SystemExit(1)
root = ET.fromstring(raw[: end + len("</hierarchy>")])
texts = []
for node in root.iter("node"):
    for key in ("text", "content-desc"):
        value = node.attrib.get(key, "")
        if value:
            texts.append(value)
problem_texts = [
    text for text in texts
    if "isn't responding" in text or "keeps stopping" in text or "has stopped" in text
]
for text in problem_texts:
    if "Trajectory" in text or package_name in text:
        print("FAIL")
        raise SystemExit(0)
if not problem_texts:
    print("NONE")
    raise SystemExit(0)
for node in root.iter("node"):
    if node.attrib.get("text") in {"Wait", "Close app", "OK"}:
        match = re.match(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]", node.attrib.get("bounds", ""))
        if match:
            x1, y1, x2, y2 = map(int, match.groups())
            print("TAP", (x1 + x2) // 2, (y1 + y2) // 2)
            raise SystemExit(0)
print("NONE")
PY
    )"; then
      return 0
    fi
    if [[ "$action" == "FAIL" ]]; then
      echo "Trajectory app platform crash/ANR dialog detected during Android UI smoke test" >&2
      exit 1
    fi
    if [[ "$action" != TAP* ]]; then
      return 0
    fi
    adb shell input tap ${action#TAP }
    sleep 2
  done
}

tap_node() {
  local needle="$1"
  local xml="$2"
  local coords
  if ! coords="$(python3 - "$xml" "$needle" <<'PY'
import re
import sys
import xml.etree.ElementTree as ET

tree = ET.parse(sys.argv[1])
needle = sys.argv[2]
for node in tree.iter("node"):
    if node.attrib.get("content-desc") == needle or node.attrib.get("text") == needle:
        bounds = node.attrib.get("bounds", "")
        match = re.match(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]", bounds)
        if match:
            x1, y1, x2, y2 = map(int, match.groups())
            print((x1 + x2) // 2, (y1 + y2) // 2)
            raise SystemExit(0)
raise SystemExit(1)
PY
  )"; then
    return 1
  fi
  adb shell input tap ${coords}
}

xml_has_text() {
  local text="$1"
  shift
  local xml
  for xml in "$@"; do
    if [[ -f "$xml" ]] && grep -Fq "$text" "$xml"; then
      return 0
    fi
  done
  return 1
}

scroll_until_text() {
  local text="$1"
  local prefix="$2"
  local max_swipes="${3:-6}"
  local pass

  for pass in $(seq 1 "$max_swipes"); do
    if xml_has_text "$text" "$artifact_dir/${prefix}_top.xml" "$artifact_dir/${prefix}_bottom.xml" "$artifact_dir/${prefix}_lower.xml" "$artifact_dir/${prefix}_scroll_"*.xml; then
      return 0
    fi
    adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
    sleep 1
    dump_screen "${prefix}_scroll_${pass}"
  done

  xml_has_text "$text" "$artifact_dir/${prefix}_top.xml" "$artifact_dir/${prefix}_bottom.xml" "$artifact_dir/${prefix}_lower.xml" "$artifact_dir/${prefix}_scroll_"*.xml
}

wait_for_text() {
  local text="$1"
  local screen_name="$2"
  for _ in 1 2 3 4 5 6 7 8 9 10; do
    dump_screen "$screen_name"
    if grep -Fq "$text" "$artifact_dir/$screen_name.xml"; then
      return 0
    fi
    sleep 1
  done
  echo "Timed out waiting for Android UI text: $text" >&2
  return 1
}

assert_texts() {
  local xml="$1"
  shift
  local missing=0
  local text

  for text in "$@"; do
    if ! grep -Fq "$text" "$xml"; then
      echo "Android UI smoke missing text in $(basename "$xml"): $text" >&2
      missing=1
    fi
  done

  return "$missing"
}

capture_tab() {
  local label="$1"
  local prefix="$2"
  dump_screen "${prefix}_top"
  adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
  sleep 1
  dump_screen "${prefix}_bottom"
  cat "$artifact_dir/${prefix}_top.xml" "$artifact_dir/${prefix}_bottom.xml" > "$artifact_dir/${prefix}.xml"
  xml_files+=("$artifact_dir/${prefix}.xml")
  if [[ "$label" != "Status" ]]; then
    tap_node "nav.${label,,}" "$artifact_dir/${prefix}_bottom.xml" || tap_node "$label" "$artifact_dir/${prefix}_bottom.xml"
    sleep 1
  fi
}

xml_files=()
wait_for_text "Start proxy" main
assert_texts "$artifact_dir/main.xml" "Trajectory" "Status" "Start proxy"
xml_files+=("$artifact_dir/main.xml")

nav_source="$artifact_dir/main.xml"
for tab in Profile Resolvers VPN Diagnostics; do
  tap_node "$tab tab" "$nav_source" || tap_node "$tab" "$nav_source"
  sleep 1
  dump_screen "${tab,,}_top"
  adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
  sleep 1
  dump_screen "${tab,,}_bottom"
  if [[ "$tab" == "Resolvers" ]]; then
    if tap_node "Frontier experimental mode" "$artifact_dir/${tab,,}_bottom.xml" || tap_node "Frontier" "$artifact_dir/${tab,,}_bottom.xml"; then
      sleep 1
      dump_screen "frontier_selected"
    else
      echo "Frontier experimental mode was not selectable from the Resolvers screen" >&2
      exit 1
    fi
  fi
  adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
  sleep 1
  dump_screen "${tab,,}_lower"
  if [[ "$tab" == "Resolvers" ]]; then
    if ! scroll_until_text "Check DNS list" "${tab,,}" 6; then
      echo "Resolvers screen did not expose Check DNS list after scroll search" >&2
      exit 1
    fi
  fi
  cat "$artifact_dir/${tab,,}_top.xml" "$artifact_dir/${tab,,}_bottom.xml" "$artifact_dir/${tab,,}_lower.xml" "$artifact_dir/${tab,,}_scroll_"*.xml > "$artifact_dir/${tab,,}.xml" 2>/dev/null || \
    cat "$artifact_dir/${tab,,}_top.xml" "$artifact_dir/${tab,,}_bottom.xml" "$artifact_dir/${tab,,}_lower.xml" > "$artifact_dir/${tab,,}.xml"
  xml_files+=("$artifact_dir/${tab,,}.xml")
  if [[ "$tab" == "Resolvers" ]]; then
    xml_files+=("$artifact_dir/frontier_selected.xml")
  fi
  adb shell input swipe "$swipe_x" "$swipe_end_y" "$swipe_x" "$swipe_start_y" 600
  adb shell input swipe "$swipe_x" "$swipe_end_y" "$swipe_x" "$swipe_start_y" 600
  sleep 1
  dump_screen "nav_${tab,,}"
  nav_source="$artifact_dir/nav_${tab,,}.xml"
done

cat "${xml_files[@]}" > "$artifact_dir/all.xml"
assert_texts "$artifact_dir/all.xml" \
  "Profile" \
  "Tunnel domain" \
  "Resolvers" \
  "Check DNS list" \
  "Frontier" \
  "Experimental" \
  "VPN" \
  "MTU" \
  "Diagnostics" \
  "Runtime log" \
  "Start VPN" \
  "Stop Trajectory"

adb logcat -d > "$artifact_dir/logcat.txt"
if grep -E "FATAL EXCEPTION|E AndroidRuntime" "$artifact_dir/logcat.txt"; then
  echo "Android crash detected during UI smoke test" >&2
  exit 1
fi

find "$artifact_dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.xml' -o -name '*.txt' \) \
  -printf '%f\n' | sort > "$artifact_dir/manifest.txt"
