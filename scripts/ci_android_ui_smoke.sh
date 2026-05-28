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

run_android_sidecar_help() {
  local package_dump="$artifact_dir/package-after-install.txt"
  local native_dir
  local primary_abi
  local binary
  timeout 10s adb shell dumpsys package "$package_name" > "$package_dump"
  native_dir="$(
    tr -d '\r' < "$package_dump" |
      awk '
        /nativeLibraryDir=/ || /legacyNativeLibraryDir=/ {
          value=$0
          sub(/^[^=]*=/, "", value)
          sub(/^[[:space:]]+/, "", value)
          print value
          exit
        }
      '
  )"
  if [[ -z "$native_dir" ]]; then
    echo "could not find nativeLibraryDir for $package_name" >&2
    return 1
  fi
  primary_abi="$(
    tr -d '\r' < "$package_dump" |
      awk '/primaryCpuAbi=/ {
        value=$0
        sub(/^[^=]*=/, "", value)
        sub(/^[[:space:]]+/, "", value)
        print value
        exit
      }'
  )"
  binary="$native_dir/libtrajectory_client.so"
  if ! adb shell "test -x '$binary'" >/dev/null 2>&1 && [[ -n "$primary_abi" && "$primary_abi" != "null" ]]; then
    binary="$native_dir/$primary_abi/libtrajectory_client.so"
  fi
  adb shell "$binary" --help > "$artifact_dir/native-sidecar-help.txt" 2>&1
  grep -Fq "Usage: trajectory-client" "$artifact_dir/native-sidecar-help.txt"
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
run_android_sidecar_help
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

screen_size="$(
  adb shell wm size |
    tr -d '\r' |
    awk '
      /Override size/ { size = $3 }
      /Physical size/ && size == "" { size = $3 }
      END { print size }
    '
)"
if [[ "$screen_size" =~ ^[0-9]+x[0-9]+$ ]]; then
  screen_width="${screen_size%x*}"
  screen_height="${screen_size#*x}"
else
  screen_width=1080
  screen_height=1920
fi

swipe_x=$((screen_width / 2))
swipe_start_y=$((screen_height * 68 / 100))
swipe_end_y=$((screen_height * 30 / 100))
swipe_restore_start_y=$((screen_height * 30 / 100))
swipe_restore_end_y=$((screen_height * 68 / 100))

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
  local raw="${output%.png}.raw-screencap"
  local visual_report="${output%.png}.visual.txt"
  timeout 20s adb exec-out screencap > "$raw"
  timeout 20s adb exec-out screencap -p > "$output"
  python3 - "$raw" "$visual_report" <<'PY'
import struct
import sys
from pathlib import Path

raw_path = Path(sys.argv[1])
report_path = Path(sys.argv[2])
data = raw_path.read_bytes()
if len(data) < 12:
    raise SystemExit("raw screencap is too short")

width, height, pixel_format = struct.unpack_from("<III", data, 0)
if width <= 0 or height <= 0 or width > 10000 or height > 10000:
    raise SystemExit(f"invalid screencap dimensions: {width}x{height}")

payload = data[12:]
pixels = width * height
if pixels == 0:
    raise SystemExit("screencap contained zero pixels")
bytes_per_pixel = len(payload) // pixels
if bytes_per_pixel < 3:
    raise SystemExit(
        f"unsupported screencap format={pixel_format} bytes_per_pixel={bytes_per_pixel}"
    )

step = max(1, pixels // 50000)
sampled = 0
non_background = 0
ink = 0
edges = 0
min_luma = 255
max_luma = 0
previous_luma = None
buckets: set[tuple[int, int, int]] = set()

for index in range(0, pixels, step):
    y = index // width
    if y < height * 0.05 or y > height * 0.92:
        continue
    offset = index * bytes_per_pixel
    if offset + 2 >= len(payload):
        break
    red, green, blue = payload[offset], payload[offset + 1], payload[offset + 2]
    luma = int((red * 299 + green * 587 + blue * 114) / 1000)
    chroma = max(red, green, blue) - min(red, green, blue)
    sampled += 1
    min_luma = min(min_luma, luma)
    max_luma = max(max_luma, luma)
    if luma < 242 or chroma > 8:
        non_background += 1
    if luma < 115:
        ink += 1
    if previous_luma is not None and abs(luma - previous_luma) > 22:
        edges += 1
    previous_luma = luma
    buckets.add((red // 32, green // 32, blue // 32))

if sampled < 1000:
    raise SystemExit(f"not enough visual samples: {sampled}")

non_background_ratio = non_background / sampled
ink_ratio = ink / sampled
edge_ratio = edges / max(1, sampled - 1)
contrast = max_luma - min_luma
report = (
    f"width={width}\n"
    f"height={height}\n"
    f"format={pixel_format}\n"
    f"sampled={sampled}\n"
    f"non_background_ratio={non_background_ratio:.5f}\n"
    f"ink_ratio={ink_ratio:.5f}\n"
    f"edge_ratio={edge_ratio:.5f}\n"
    f"contrast={contrast}\n"
    f"color_buckets={len(buckets)}\n"
)
report_path.write_text(report, encoding="utf-8")

failures = []
if contrast < 35:
    failures.append(f"contrast too low ({contrast})")
if non_background_ratio < 0.015:
    failures.append(f"non-background pixel ratio too low ({non_background_ratio:.5f})")
if ink_ratio < 0.001:
    failures.append(f"ink pixel ratio too low ({ink_ratio:.5f})")
if edge_ratio < 0.0005:
    failures.append(f"edge ratio too low ({edge_ratio:.5f})")
if len(buckets) < 6:
    failures.append(f"too few color buckets ({len(buckets)})")
if failures:
    raise SystemExit("; ".join(failures))
PY
  rm -f "$raw"
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

assert_checked_mode() {
  local mode_description="$1"
  local xml="$2"
  python3 - "$xml" "$mode_description" <<'PY'
import sys
import xml.etree.ElementTree as ET

tree = ET.parse(sys.argv[1])
mode_description = sys.argv[2]

def contains_description(node):
    if node.attrib.get("content-desc") == mode_description:
        return True
    return any(contains_description(child) for child in list(node))

for node in tree.iter("node"):
    if (
        node.attrib.get("checkable") == "true"
        and node.attrib.get("checked") == "true"
        and contains_description(node)
    ):
        raise SystemExit(0)

raise SystemExit(1)
PY
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

adb_input_text() {
  local value="$1"
  value="${value//%/%25}"
  value="${value// /%s}"
  value="${value//\\/\\\\}"
  value="${value//\"/\\\"}"
  adb shell input text "$value"
}

tap_first_text_field_after_label() {
  local label="$1"
  local xml="$2"
  local coords
  if ! coords="$(python3 - "$xml" "$label" <<'PY'
import re
import sys
import xml.etree.ElementTree as ET

tree = ET.parse(sys.argv[1])
label = sys.argv[2]
nodes = list(tree.iter("node"))

def bounds(node):
    match = re.match(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]", node.attrib.get("bounds", ""))
    if not match:
        return None
    return tuple(map(int, match.groups()))

label_bounds = None
for node in nodes:
    if node.attrib.get("text") == label or node.attrib.get("content-desc") == label:
        label_bounds = bounds(node)
        break

if not label_bounds:
    raise SystemExit(1)

lx1, ly1, lx2, ly2 = label_bounds
for node in nodes:
    if "EditText" not in node.attrib.get("class", ""):
        continue
    current = bounds(node)
    if not current:
        continue
    x1, y1, x2, y2 = current
    if y1 <= ly1 <= y2 or (abs(y1 - ly1) < 120 and x1 <= lx1 <= x2):
        print((x1 + x2) // 2, (y1 + y2) // 2)
        raise SystemExit(0)

raise SystemExit(1)
PY
  )"; then
    return 1
  fi
  adb shell input tap ${coords}
}

clear_focused_field() {
  adb shell input keyevent KEYCODE_MOVE_END >/dev/null 2>&1 || true
  for _ in $(seq 1 96); do
    adb shell input keyevent KEYCODE_DEL >/dev/null 2>&1 || true
  done
}

run_optional_live_proxy_smoke() {
  local domain="${TRAJECTORY_ANDROID_SMOKE_DOMAIN:-}"
  local access_key="${TRAJECTORY_ANDROID_SMOKE_ACCESS_KEY:-}"
  local http_fetch_url="${TRAJECTORY_ANDROID_SMOKE_FETCH_URL:-http://example.com/}"
  local http_port="${TRAJECTORY_ANDROID_SMOKE_HTTP_PORT:-7001}"
  local http_host="$http_fetch_url"
  http_host="${http_host#*//}"
  http_host="${http_host%%/*}"

  if [[ -z "$domain" || -z "$access_key" ]]; then
    echo "Android live proxy smoke skipped; TRAJECTORY_ANDROID_SMOKE_DOMAIN/ACCESS_KEY are not set" \
      > "$artifact_dir/live-proxy-smoke-skipped.txt"
    return 0
  fi

  tap_node "Profile tab" "$nav_source" || tap_node "Profile" "$nav_source"
  sleep 1
  dump_screen "live_profile"
  tap_first_text_field_after_label "Tunnel domain" "$artifact_dir/live_profile.xml"
  clear_focused_field
  adb_input_text "$domain"
  tap_first_text_field_after_label "Access key" "$artifact_dir/live_profile.xml"
  clear_focused_field
  adb_input_text "$access_key"
  adb shell input keyevent KEYCODE_BACK >/dev/null 2>&1 || true
  sleep 1

  if [[ -n "${TRAJECTORY_ANDROID_SMOKE_RESOLVERS:-}" ]]; then
    local first_resolver
    first_resolver="$(
      printf '%s\n' "$TRAJECTORY_ANDROID_SMOKE_RESOLVERS" |
        tr ', ' '\n' |
        awk 'NF { print; exit }'
    )"
    tap_node "Resolvers tab" "$artifact_dir/live_profile.xml" || tap_node "Resolvers" "$artifact_dir/live_profile.xml"
    sleep 1
    dump_screen "live_resolvers_top"
    adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
    sleep 1
    dump_screen "live_resolvers_form"
    tap_first_text_field_after_label "DNS resolvers" "$artifact_dir/live_resolvers_form.xml"
    clear_focused_field
    adb_input_text "$first_resolver"
    adb shell input keyevent KEYCODE_BACK >/dev/null 2>&1 || true
    sleep 1
  fi

  tap_node "Status tab" "$artifact_dir/live_profile.xml" || tap_node "Status" "$artifact_dir/live_profile.xml"
  sleep 1
  dump_screen "live_status"
  tap_node "Save profile" "$artifact_dir/live_status.xml"
  sleep 1
  dump_screen "live_saved"
  tap_node "Start proxy" "$artifact_dir/live_saved.xml"

  for pass in $(seq 1 60); do
    sleep 1
    dump_screen "live_proxy_${pass}"
    if grep -Fq "Proxy connected" "$artifact_dir/live_proxy_${pass}.xml"; then
      echo "Proxy connected after ${pass}s" > "$artifact_dir/live-proxy-smoke.txt"
      if ! timeout 25s adb shell "printf 'GET ${http_fetch_url} HTTP/1.1\r\nHost: ${http_host}\r\nConnection: close\r\n\r\n' | toybox nc -w 20 127.0.0.1 ${http_port}" \
        > "$artifact_dir/live-proxy-http.txt" 2>&1; then
        echo "Android live proxy smoke could not fetch through HTTP proxy on 127.0.0.1:${http_port}" >&2
        return 1
      fi
      if ! grep -Eq '^HTTP/[0-9.]+ [23][0-9][0-9]' "$artifact_dir/live-proxy-http.txt"; then
        echo "Android live proxy smoke did not receive a 2xx/3xx HTTP response" >&2
        return 1
      fi
      tap_node "Stop Trajectory" "$artifact_dir/live_proxy_${pass}.xml" || true
      return 0
    fi
    if grep -Fq "Failed" "$artifact_dir/live_proxy_${pass}.xml"; then
      echo "Android live proxy smoke reached Failed state" >&2
      return 1
    fi
  done

  echo "Timed out waiting for Android live proxy to reach Proxy connected" >&2
  return 1
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
    frontier_selected=0
    for frontier_xml in "$artifact_dir/${tab,,}_top.xml" "$artifact_dir/${tab,,}_bottom.xml"; do
      if tap_node "Frontier experimental mode" "$frontier_xml" || tap_node "Frontier" "$frontier_xml"; then
        frontier_selected=1
        break
      fi
    done
    for pass in 1 2 3 4 5 6 7 8; do
      if [[ "$frontier_selected" -eq 1 ]]; then
        break
      fi
      adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
      sleep 1
      dump_screen "${tab,,}_frontier_scroll_${pass}"
      if tap_node "Frontier experimental mode" "$artifact_dir/${tab,,}_frontier_scroll_${pass}.xml" || tap_node "Frontier" "$artifact_dir/${tab,,}_frontier_scroll_${pass}.xml"; then
        frontier_selected=1
        break
      fi
    done
    if [[ "$frontier_selected" -ne 1 ]]; then
      echo "Frontier experimental mode was not selectable from the Resolvers screen" >&2
      exit 1
    fi
    sleep 1
    dump_screen "frontier_selected"
    if ! assert_checked_mode "Frontier experimental mode" "$artifact_dir/frontier_selected.xml"; then
      echo "Frontier experimental mode was visible but not checked after tap" >&2
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
  adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 600
  adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 600
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

run_optional_live_proxy_smoke

adb logcat -d > "$artifact_dir/logcat.txt"
if grep -E "FATAL EXCEPTION|E AndroidRuntime" "$artifact_dir/logcat.txt"; then
  echo "Android crash detected during UI smoke test" >&2
  exit 1
fi

find "$artifact_dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.xml' -o -name '*.txt' \) \
  -printf '%f\n' | sort > "$artifact_dir/manifest.txt"
