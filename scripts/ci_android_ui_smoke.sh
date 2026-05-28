#!/usr/bin/env bash
set -euo pipefail

apk="${1:?usage: ci_android_ui_smoke.sh <apk> [artifact-dir]}"
artifact_dir="${2:-${RUNNER_TEMP:-/tmp}/trajectory-android-ui}"
package_name="app.trajectory.android"

mkdir -p "$artifact_dir"
test -f "$apk"

adb wait-for-device
adb uninstall "$package_name" >/dev/null 2>&1 || true
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

dump_screen() {
  local name="$1"
  dismiss_platform_dialogs
  adb exec-out uiautomator dump /dev/tty > "$artifact_dir/$name.raw.xml"
  python3 - "$artifact_dir/$name.raw.xml" "$artifact_dir/$name.xml" <<'PY'
import sys
from pathlib import Path

raw = Path(sys.argv[1]).read_text(errors="replace")
end = raw.find("</hierarchy>")
if end < 0:
    raise SystemExit("uiautomator XML did not contain </hierarchy>")
Path(sys.argv[2]).write_text(raw[: end + len("</hierarchy>")])
PY
  adb exec-out screencap -p > "$artifact_dir/$name.png"
}

dismiss_platform_dialogs() {
  local probe="$artifact_dir/platform-dialog-probe.raw.xml"
  adb exec-out uiautomator dump /dev/tty > "$probe" 2>/dev/null || return 0
  if ! grep -Fq "System UI isn't responding" "$probe"; then
    return 0
  fi
  local coords
  if coords="$(python3 - "$probe" <<'PY'
import re
import sys
import xml.etree.ElementTree as ET

raw = open(sys.argv[1], errors="replace").read()
end = raw.find("</hierarchy>")
if end < 0:
    raise SystemExit(1)
root = ET.fromstring(raw[: end + len("</hierarchy>")])
for node in root.iter("node"):
    if node.attrib.get("text") == "Wait":
        match = re.match(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]", node.attrib.get("bounds", ""))
        if match:
            x1, y1, x2, y2 = map(int, match.groups())
            print((x1 + x2) // 2, (y1 + y2) // 2)
            raise SystemExit(0)
raise SystemExit(1)
PY
  )"; then
    adb shell input tap ${coords}
    sleep 2
  fi
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
dump_screen main
grep -Fq "Trajectory" "$artifact_dir/main.xml"
grep -Fq "Status" "$artifact_dir/main.xml"
grep -Fq "Start proxy" "$artifact_dir/main.xml"
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
grep -Fq "Profile" "$artifact_dir/all.xml"
grep -Fq "Tunnel domain" "$artifact_dir/all.xml"
grep -Fq "Resolvers" "$artifact_dir/all.xml"
grep -Fq "Check DNS list" "$artifact_dir/all.xml"
grep -Fq "Frontier" "$artifact_dir/all.xml"
grep -Fq "Experimental" "$artifact_dir/all.xml"
grep -Fq "VPN" "$artifact_dir/all.xml"
grep -Fq "MTU" "$artifact_dir/all.xml"
grep -Fq "Diagnostics" "$artifact_dir/all.xml"
grep -Fq "Runtime log" "$artifact_dir/all.xml"
grep -Fq "Start VPN" "$artifact_dir/all.xml"
grep -Fq "Stop Trajectory" "$artifact_dir/all.xml"

adb logcat -d > "$artifact_dir/logcat.txt"
if grep -E "FATAL EXCEPTION|E AndroidRuntime" "$artifact_dir/logcat.txt"; then
  echo "Android crash detected during UI smoke test" >&2
  exit 1
fi

find "$artifact_dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.xml' -o -name '*.txt' \) \
  -printf '%f\n' | sort > "$artifact_dir/manifest.txt"
