#!/usr/bin/env bash
set -euo pipefail

apk="${1:?usage: ci_android_ui_smoke.sh <apk> [artifact-dir]}"
artifact_dir="${2:-${RUNNER_TEMP:-/tmp}/trajectory-android-ui}"
package_name="app.trajectory.android"
smoke_probe_package="app.trajectory.smokeprobe"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -z "${ANDROID_HOME:-}" && -d "$repo_root/.tooling/android-sdk" ]]; then
  export ANDROID_HOME="$repo_root/.tooling/android-sdk"
fi
if [[ -n "${ANDROID_HOME:-}" && -d "$ANDROID_HOME/platform-tools" ]]; then
  export PATH="$ANDROID_HOME/platform-tools:$PATH"
fi
smoke_probe_apk="${TRAJECTORY_ANDROID_SMOKE_PROBE_APK:-$repo_root/clients/android/smokeprobe/build/outputs/apk/debug/smokeprobe-debug.apk}"

mkdir -p "$artifact_dir"
test -f "$apk"

local_live_dir=""
local_live_server_pid=""
local_origin_pid=""
local_origin_marker=""

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

adb_wait_ready() {
  local attempt
  for attempt in $(seq 1 20); do
    timeout 10s adb wait-for-device >/dev/null 2>&1 || true
    if [[ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]] &&
      adb shell true >/dev/null 2>&1; then
      return 0
    fi
    if [[ "$attempt" -eq 8 ]]; then
      adb reconnect >/dev/null 2>&1 || true
    fi
    sleep 1
  done
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

build_smoke_probe_apk() {
  if [[ -n "${TRAJECTORY_ANDROID_SMOKE_PROBE_APK:-}" && -f "$smoke_probe_apk" ]]; then
    return 0
  fi
  if [[ -n "${TRAJECTORY_ANDROID_SMOKE_PROBE_APK:-}" ]]; then
    echo "Android VPN smoke probe APK was not found: $smoke_probe_apk" >&2
    return 1
  fi
  "$repo_root/clients/android/gradlew" -p "$repo_root/clients/android" \
    :smokeprobe:assembleDebug --no-daemon \
    > "$artifact_dir/smokeprobe-build.txt" 2>&1
  test -f "$smoke_probe_apk"
}

install_smoke_probe_apk() {
  build_smoke_probe_apk
  local attempt
  for attempt in 1 2 3; do
    if adb install -r "$smoke_probe_apk" > "$artifact_dir/smokeprobe-install-attempt-${attempt}.txt" 2>&1; then
      return 0
    fi
    adb uninstall "$smoke_probe_package" >/dev/null 2>&1 || true
    sleep 2
  done
  echo "Android smoke probe APK install failed after retries" >&2
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
  if [[ -n "$local_origin_pid" ]] && kill -0 "$local_origin_pid" >/dev/null 2>&1; then
    kill "$local_origin_pid" >/dev/null 2>&1 || true
    wait "$local_origin_pid" >/dev/null 2>&1 || true
  fi
  if [[ -n "$local_live_server_pid" ]] && kill -0 "$local_live_server_pid" >/dev/null 2>&1; then
    kill "$local_live_server_pid" >/dev/null 2>&1 || true
    wait "$local_live_server_pid" >/dev/null 2>&1 || true
  fi
  if [[ -n "$local_live_dir" ]]; then
    rm -rf "$local_live_dir"
  fi
  exit "$code"
}

trap on_exit EXIT

adb wait-for-device
wait_for_boot_completed
adb uninstall "$package_name" >/dev/null 2>&1 || true
adb uninstall "$smoke_probe_package" >/dev/null 2>&1 || true
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
short_swipe_start_y=$((screen_height * 70 / 100))
short_swipe_end_y=$((screen_height * 55 / 100))
swipe_restore_start_y=$((screen_height * 30 / 100))
swipe_restore_end_y=$((screen_height * 68 / 100))

dump_ui_tree_raw() {
  local output="$1"
  local seconds="${2:-10}"
  for _ in 1 2 3 4; do
    if timeout "${seconds}s" adb exec-out uiautomator dump /dev/tty > "$output"; then
      return 0
    fi
    adb_wait_ready || true
    sleep 1
  done
  return 1
}

valid_png_file() {
  python3 - "$1" <<'PY'
import struct
import sys
from pathlib import Path

path = Path(sys.argv[1])
try:
    data = path.read_bytes()
except FileNotFoundError:
    raise SystemExit(1)
signature = b"\x89PNG\r\n\x1a\n"
if not data.startswith(signature):
    raise SystemExit(1)
offset = len(signature)
seen_ihdr = False
seen_idat = False
while offset + 12 <= len(data):
    length = struct.unpack(">I", data[offset : offset + 4])[0]
    kind = data[offset + 4 : offset + 8]
    end = offset + 12 + length
    if end > len(data):
        raise SystemExit(1)
    if kind == b"IHDR":
        width, height = struct.unpack(">II", data[offset + 8 : offset + 16])
        if width <= 0 or height <= 0 or width > 10000 or height > 10000:
            raise SystemExit(1)
        seen_ihdr = True
    elif kind == b"IDAT":
        seen_idat = True
    elif kind == b"IEND":
        raise SystemExit(0 if seen_ihdr and seen_idat else 1)
    offset = end
raise SystemExit(1)
PY
}

valid_raw_screencap() {
  python3 - "$1" <<'PY'
import struct
import sys
from pathlib import Path

path = Path(sys.argv[1])
try:
    data = path.read_bytes()
except FileNotFoundError:
    raise SystemExit(1)
if len(data) < 16:
    raise SystemExit(1)
width, height, _pixel_format = struct.unpack_from("<III", data, 0)
if width <= 0 or height <= 0 or width > 10000 or height > 10000:
    raise SystemExit(1)
pixels = width * height
for header_size in (12, 16):
    payload = len(data) - header_size
    if payload in (pixels * 2, pixels * 3, pixels * 4):
        raise SystemExit(0)
raise SystemExit(1)
PY
}

capture_screenshot() {
  local output="$1"
  local raw="${output%.png}.raw-screencap"
  local visual_report="${output%.png}.visual.txt"
  local png_capture_status=0
  local raw_capture_status=0
  local captured=0
  local attempt
  local remote_png

  : > "$raw"
  : > "$output"
  for attempt in 1 2 3 4; do
    png_capture_status=0
    raw_capture_status=0
    rm -f "${output}.tmp" "${raw}.tmp"

    if [[ "${TRAJECTORY_ANDROID_SMOKE_FORCE_RAW_SCREENSHOT:-}" != "1" ]]; then
      if timeout 20s adb exec-out screencap -p > "${output}.tmp" 2> "${output}.stderr" &&
        valid_png_file "${output}.tmp"; then
        mv "${output}.tmp" "$output"
        captured=1
        break
      else
        png_capture_status=$?
      fi

      remote_png="/sdcard/trajectory-smoke-${RANDOM}-${RANDOM}.png"
      if timeout 20s adb shell screencap -p "$remote_png" >/dev/null 2> "${output}.device.stderr"; then
        if timeout 20s adb pull "$remote_png" "${output}.tmp" >/dev/null 2>> "${output}.device.stderr" &&
          valid_png_file "${output}.tmp"; then
          mv "${output}.tmp" "$output"
          adb shell rm -f "$remote_png" >/dev/null 2>&1 || true
          captured=1
          break
        fi
        adb shell rm -f "$remote_png" >/dev/null 2>&1 || true
      fi
    else
      png_capture_status=77
    fi

    if timeout 20s adb exec-out screencap > "${raw}.tmp" 2> "${raw}.stderr" &&
      valid_raw_screencap "${raw}.tmp"; then
      mv "${raw}.tmp" "$raw"
      captured=1
      break
    else
      raw_capture_status=$?
    fi

    echo "Android screenshot capture attempt ${attempt} failed; png_status=${png_capture_status}; raw_status=${raw_capture_status}" \
      >> "$visual_report.capture"
    adb_wait_ready || true
    sleep 1
  done

  if [[ "$captured" -ne 1 ]]; then
    echo "Android screenshot capture failed after retries; png_status=${png_capture_status}; raw_status=${raw_capture_status}" \
      >> "$visual_report.capture"
  fi
  python3 - "$raw" "$visual_report" "$output" "$png_capture_status" <<'PY'
import binascii
import struct
import sys
import zlib
from pathlib import Path

raw_path = Path(sys.argv[1])
report_path = Path(sys.argv[2])
png_path = Path(sys.argv[3])
png_capture_status = int(sys.argv[4])
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


def paeth(left: int, up: int, up_left: int) -> int:
    estimate = left + up - up_left
    left_distance = abs(estimate - left)
    up_distance = abs(estimate - up)
    up_left_distance = abs(estimate - up_left)
    if left_distance <= up_distance and left_distance <= up_left_distance:
        return left
    if up_distance <= up_left_distance:
        return up
    return up_left


def decode_png(path: Path):
    try:
        data = path.read_bytes()
    except FileNotFoundError:
        return None
    if not data.startswith(PNG_SIGNATURE):
        return None

    offset = len(PNG_SIGNATURE)
    width = height = bit_depth = color_type = interlace = None
    idat = bytearray()
    while offset + 12 <= len(data):
        length = struct.unpack(">I", data[offset : offset + 4])[0]
        kind = data[offset + 4 : offset + 8]
        payload = data[offset + 8 : offset + 8 + length]
        offset += 12 + length
        if kind == b"IHDR":
            width, height, bit_depth, color_type, _, _, interlace = struct.unpack(
                ">IIBBBBB", payload
            )
        elif kind == b"IDAT":
            idat.extend(payload)
        elif kind == b"IEND":
            break

    if (
        width is None
        or height is None
        or bit_depth != 8
        or color_type not in (2, 6)
        or interlace != 0
    ):
        return None

    channels = 4 if color_type == 6 else 3
    row_bytes = width * channels
    compressed = zlib.decompress(bytes(idat))
    rows = bytearray()
    previous = bytearray(row_bytes)
    position = 0
    for _ in range(height):
        filter_type = compressed[position]
        position += 1
        current = bytearray(compressed[position : position + row_bytes])
        position += row_bytes
        for index, value in enumerate(current):
            left = current[index - channels] if index >= channels else 0
            up = previous[index]
            up_left = previous[index - channels] if index >= channels else 0
            if filter_type == 1:
                current[index] = (value + left) & 0xFF
            elif filter_type == 2:
                current[index] = (value + up) & 0xFF
            elif filter_type == 3:
                current[index] = (value + ((left + up) // 2)) & 0xFF
            elif filter_type == 4:
                current[index] = (value + paeth(left, up, up_left)) & 0xFF
            elif filter_type != 0:
                return None
        rows.extend(current)
        previous = current
    return {
        "width": width,
        "height": height,
        "format": "png",
        "bytes_per_pixel": channels,
        "payload": bytes(rows),
        "source": "png",
    }


def parse_raw(path: Path):
    try:
        data = path.read_bytes()
    except FileNotFoundError:
        return None
    if len(data) < 16:
        return None
    width, height, pixel_format = struct.unpack_from("<III", data, 0)
    if width <= 0 or height <= 0 or width > 10000 or height > 10000:
        return None

    pixels = width * height
    for header_size in (12, 16):
        payload = data[header_size:]
        for candidate_bpp in (4, 3, 2):
            if len(payload) >= pixels * candidate_bpp:
                return {
                    "width": width,
                    "height": height,
                    "format": pixel_format,
                    "bytes_per_pixel": candidate_bpp,
                    "payload": payload[: pixels * candidate_bpp],
                    "source": f"raw-header-{header_size}",
                }
    return None


source = parse_raw(raw_path) or decode_png(png_path)
if source is None:
    raise SystemExit("could not decode PNG screenshot or raw screencap")

width = source["width"]
height = source["height"]
pixel_format = source["format"]
bytes_per_pixel = source["bytes_per_pixel"]
payload = source["payload"]
pixels = width * height
if pixels == 0:
    raise SystemExit("screenshot contained zero pixels")


def rgb_at(index: int) -> tuple[int, int, int]:
    offset = index * bytes_per_pixel
    if bytes_per_pixel == 2:
        value = payload[offset] | (payload[offset + 1] << 8)
        red = ((value >> 11) & 0x1F) * 255 // 31
        green = ((value >> 5) & 0x3F) * 255 // 63
        blue = (value & 0x1F) * 255 // 31
        return red, green, blue
    return payload[offset], payload[offset + 1], payload[offset + 2]

def png_chunk(kind: bytes, payload: bytes) -> bytes:
    checksum = binascii.crc32(kind)
    checksum = binascii.crc32(payload, checksum) & 0xFFFFFFFF
    return struct.pack(">I", len(payload)) + kind + payload + struct.pack(">I", checksum)


def synthesize_png_from_raw() -> str:
    try:
        existing = png_path.read_bytes()
    except FileNotFoundError:
        existing = b""
    if existing.startswith(PNG_SIGNATURE) and len(existing) > 32:
        return "adb-png"

    rows = bytearray()
    for y in range(height):
        rows.append(0)
        for x in range(width):
            rows.extend(rgb_at((y * width) + x))

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    png_path.write_bytes(
        PNG_SIGNATURE
        + png_chunk(b"IHDR", ihdr)
        + png_chunk(b"IDAT", zlib.compress(bytes(rows), level=1))
        + png_chunk(b"IEND", b"")
    )
    return "raw-synthesized"


png_source = synthesize_png_from_raw()
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
    if (index + 1) * bytes_per_pixel > len(payload):
        break
    red, green, blue = rgb_at(index)
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
    f"png_capture_status={png_capture_status}\n"
    f"png_source={png_source}\n"
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
  rm -f "${output}.stderr"
  rm -f "${output}.tmp" "${raw}.tmp" "${raw}.stderr"
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

tap_control() {
  local label="$1"
  local source_xml="$2"
  local prefix="$3"
  if tap_node "$label" "$source_xml"; then
    return 0
  fi
  for pass in 1 2 3 4; do
    adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
    sleep 1
    dump_screen "${prefix}_controls_${pass}"
    if tap_node "$label" "$artifact_dir/${prefix}_controls_${pass}.xml"; then
      return 0
    fi
  done
  return 1
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

tap_first_password_field() {
  local xml="$1"
  local coords
  if ! coords="$(python3 - "$xml" <<'PY'
import re
import sys
import xml.etree.ElementTree as ET

tree = ET.parse(sys.argv[1])

for node in tree.iter("node"):
    if node.attrib.get("password") != "true":
        continue
    match = re.match(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]", node.attrib.get("bounds", ""))
    if not match:
        continue
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

clear_focused_field() {
  adb shell 'input keyevent KEYCODE_MOVE_END >/dev/null 2>&1 || true; i=0; while [ "$i" -lt 96 ]; do input keyevent KEYCODE_DEL >/dev/null 2>&1 || true; i=$((i + 1)); done'
}

pick_local_port() {
  python3 - <<'PY'
import socket

for _ in range(100):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as stream:
        stream.bind(("127.0.0.1", 0))
        port = stream.getsockname()[1]
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as datagram:
            try:
                datagram.bind(("127.0.0.1", port))
            except OSError:
                continue
            print(port)
            raise SystemExit(0)
raise SystemExit("could not find a free TCP/UDP port")
PY
}

pick_host_ip() {
  local ip
  ip="$(
    (ip route get 8.8.8.8 2>/dev/null || true) |
      awk '{ for (i = 1; i <= NF; i++) if ($i == "src") { print $(i + 1); exit } }'
  )"
  if [[ -z "$ip" ]]; then
    ip="$(hostname -I 2>/dev/null | awk '{ print $1 }')"
  fi
  printf '%s\n' "${ip:-127.0.0.1}"
}

prepare_local_live_proxy_smoke() {
  if [[ "${TRAJECTORY_ANDROID_SMOKE_LOCAL_SERVER:-}" != "1" ]]; then
    return 0
  fi

  local_live_dir="$(mktemp -d)"
  local client_db="$local_live_dir/clients.json"
  local origin_dir="$local_live_dir/origin"
  local domain="${TRAJECTORY_ANDROID_SMOKE_LOCAL_DOMAIN:-t.android-smoke}"
  local dns_port="${TRAJECTORY_ANDROID_SMOKE_LOCAL_DNS_PORT:-$(pick_local_port)}"
  local origin_port="${TRAJECTORY_ANDROID_SMOKE_LOCAL_ORIGIN_PORT:-$(pick_local_port)}"
  local host_ip="${TRAJECTORY_ANDROID_SMOKE_LOCAL_HOST_IP:-$(pick_host_ip)}"

  mkdir -p "$origin_dir"
  local_origin_marker="trajectory-android-smoke-${RANDOM}-${RANDOM}"
  printf '%s\n' "$local_origin_marker" > "$origin_dir/trajectory-smoke.txt"
  python3 -u -m http.server "$origin_port" --bind 0.0.0.0 --directory "$origin_dir" \
    > "$artifact_dir/local-origin-server.log" 2>&1 &
  local_origin_pid=$!
  sleep 1
  if ! kill -0 "$local_origin_pid" >/dev/null 2>&1; then
    echo "Android local origin server exited early" >&2
    sed -n '1,160p' "$artifact_dir/local-origin-server.log" >&2 || true
    return 1
  fi

  cargo build --release -p trajectory-cli --bin trajectory-server --bin trajectory-admin \
    > "$artifact_dir/local-live-build.txt" 2>&1

  local access_key
  access_key="$(
    target/release/trajectory-admin create-client \
      --client-db "$client_db" \
      --label android-smoke \
      --format key
  )"

  target/release/trajectory-server \
    --domain "$domain" \
    --client-db "$client_db" \
    --bind 0.0.0.0 \
    --dns-listen-port "$dns_port" \
    --target-address socks5-direct \
    > "$artifact_dir/local-live-server.log" 2>&1 &
  local_live_server_pid=$!
  sleep 1
  if ! kill -0 "$local_live_server_pid" >/dev/null 2>&1; then
    echo "Android local live smoke server exited early" >&2
    sed -n '1,160p' "$artifact_dir/local-live-server.log" >&2 || true
    return 1
  fi

  export TRAJECTORY_ANDROID_SMOKE_DOMAIN="$domain"
  export TRAJECTORY_ANDROID_SMOKE_ACCESS_KEY="$access_key"
  export TRAJECTORY_ANDROID_SMOKE_RESOLVERS="10.0.2.2:${dns_port}"
  export TRAJECTORY_ANDROID_SMOKE_FETCH_URL="http://${host_ip}:${origin_port}/trajectory-smoke.txt"
  export TRAJECTORY_ANDROID_SMOKE_EXPECT_BODY="$local_origin_marker"
  echo "Android local live proxy smoke server ready on host DNS port ${dns_port}; origin http://${host_ip}:${origin_port}/trajectory-smoke.txt" \
    > "$artifact_dir/local-live-server-ready.txt"
}

start_live_profile_from_intent() {
  local domain="$1"
  local access_key="$2"
  local resolvers="$3"
  adb shell am force-stop "$package_name"
  adb shell am start -W -n "$activity" \
    --es trajectory_smoke_domain "$domain" \
    --es trajectory_smoke_access_key "$access_key" \
    --es trajectory_smoke_resolvers "$resolvers" \
    --es trajectory_smoke_resolver_transport auto \
    --es trajectory_smoke_transport_mode velocity \
    --es trajectory_smoke_socks_port "${TRAJECTORY_ANDROID_SMOKE_SOCKS_PORT:-7000}" \
    --es trajectory_smoke_http_port "${TRAJECTORY_ANDROID_SMOKE_HTTP_PORT:-7001}" \
    --es trajectory_smoke_fetch_url "${TRAJECTORY_ANDROID_SMOKE_FETCH_URL:-}" \
    > "$artifact_dir/live-smoke-intent-start.txt"
  sleep 2
  dump_screen "live_smoke_configured"
}

fetch_android_http_proxy() {
  local http_fetch_url="$1"
  local http_host="$2"
  local http_port="$3"
  local output="$4"
  timeout 25s adb shell "printf 'GET ${http_fetch_url} HTTP/1.1\r\nHost: ${http_host}\r\nConnection: close\r\n\r\n' | toybox nc -w 20 -q 2 127.0.0.1 ${http_port}" \
    > "$output" 2>&1
}

assert_http_proxy_response() {
  local output="$1"
  if ! grep -Eq '^HTTP/[0-9.]+ [23][0-9][0-9]' "$output"; then
    echo "Android live proxy smoke did not receive a 2xx/3xx HTTP response" >&2
    return 1
  fi
  if [[ -n "${TRAJECTORY_ANDROID_SMOKE_EXPECT_BODY:-}" ]] && ! grep -Fq "$TRAJECTORY_ANDROID_SMOKE_EXPECT_BODY" "$output"; then
    echo "Android live proxy smoke did not receive the deterministic local origin marker" >&2
    return 1
  fi
}

assert_socks_fetch() {
  local socks_port="${TRAJECTORY_ANDROID_SMOKE_SOCKS_PORT:-7000}"
  local forward_port="${TRAJECTORY_ANDROID_SMOKE_SOCKS_FORWARD_PORT:-$(pick_local_port)}"
  local output="$artifact_dir/live-proxy-socks-fetch.txt"
  adb forward "tcp:${forward_port}" "tcp:${socks_port}" >/dev/null
  python3 - "$forward_port" "${TRAJECTORY_ANDROID_SMOKE_FETCH_URL:-}" "$output" "${TRAJECTORY_ANDROID_SMOKE_EXPECT_BODY:-}" <<'PY'
import socket
import struct
import sys
from pathlib import Path
from urllib.parse import urlparse

forward_port = int(sys.argv[1])
url = urlparse(sys.argv[2])
output = Path(sys.argv[3])
expected = sys.argv[4]

if url.scheme != "http" or not url.hostname:
    raise SystemExit("SOCKS smoke requires a plain http:// URL")
host = url.hostname
port = url.port or 80
path = url.path or "/"
if url.query:
    path += "?" + url.query

with socket.create_connection(("127.0.0.1", forward_port), timeout=10) as stream:
    stream.settimeout(25)
    stream.sendall(b"\x05\x01\x00")
    greeting = stream.recv(2)
    if greeting != b"\x05\x00":
        raise SystemExit(f"unexpected SOCKS greeting response: {greeting!r}")
    encoded_host = host.encode("idna")
    if len(encoded_host) > 255:
        raise SystemExit("SOCKS host name is too long")
    request = b"\x05\x01\x00\x03" + bytes([len(encoded_host)]) + encoded_host + struct.pack("!H", port)
    stream.sendall(request)
    reply = stream.recv(10)
    if len(reply) < 2 or reply[1] != 0:
        raise SystemExit(f"SOCKS CONNECT failed: {reply!r}")
    http = f"GET {path} HTTP/1.1\r\nHost: {url.netloc}\r\nConnection: close\r\n\r\n".encode()
    stream.sendall(http)
    chunks = []
    while True:
        chunk = stream.recv(4096)
        if not chunk:
            break
        chunks.append(chunk)

response = b"".join(chunks)
output.write_bytes(response)
text = response.decode("utf-8", "replace")
status = text.splitlines()[0] if text.splitlines() else ""
if not status.startswith("HTTP/") or len(status.split()) < 2 or not status.split()[1].startswith(("2", "3")):
    raise SystemExit(f"SOCKS HTTP fetch returned non-success status: {status}")
if expected and expected not in text:
    raise SystemExit("SOCKS HTTP fetch missed expected body marker")
PY
  local status=$?
  adb forward --remove "tcp:${forward_port}" >/dev/null 2>&1 || true
  if [[ "$status" -ne 0 ]]; then
    echo "Android live proxy smoke could not fetch deterministic body through SOCKS on 127.0.0.1:${socks_port}" >&2
    return 1
  fi
}

assert_proxy_status_connected() {
  local source_xml="$1"
  if grep -Fq "Proxy connected" "$source_xml" ||
    grep -Fq "status.phase.proxy_connected" "$source_xml"; then
    return 0
  fi
  echo "Android live proxy data path worked, but UI did not show Proxy connected" >&2
  return 1
}

assert_vpn_status_connected() {
  local source_xml="$1"
  if grep -Fq "VPN connected" "$source_xml" ||
    grep -Fq "status.phase.vpn_connected" "$source_xml"; then
    return 0
  fi
  echo "Android VPN data path worked, but UI did not show VPN connected" >&2
  return 1
}

assert_socks_handshake() {
  if ! assert_socks_fetch; then
      return 1
  fi
}

port_is_closed() {
  local port="$1"
  ! timeout 5s adb shell "toybox nc -w 1 127.0.0.1 ${port} </dev/null >/dev/null 2>&1"
}

assert_no_android_crash() {
  local log="$1"
  if ! timeout 15s adb logcat -d > "$log" 2> "${log}.stderr"; then
    echo "Android logcat capture timed out or failed; continuing with UI/data-path assertions" > "${log}.warning"
    return 0
  fi
  if ! python3 - "$log" "$package_name" <<'PY'
import sys
from pathlib import Path

lines = Path(sys.argv[1]).read_text(errors="replace").splitlines()
package_name = sys.argv[2]
for index, line in enumerate(lines):
    if "FATAL EXCEPTION" not in line and "E AndroidRuntime" not in line:
        continue
    block = "\n".join(lines[index:index + 30])
    if package_name in block or "Process: Trajectory" in block:
        raise SystemExit(1)
raise SystemExit(0)
PY
  then
    echo "Android crash detected during UI smoke test" >&2
    return 1
  fi
}

assert_clean_proxy_shutdown() {
  local source_xml="$1"
  local shutdown_label="${2:-Proxy}"
  local marker="${3:-$artifact_dir/live-proxy-clean-shutdown.txt}"
  local http_port="${TRAJECTORY_ANDROID_SMOKE_HTTP_PORT:-7001}"
  local socks_port="${TRAJECTORY_ANDROID_SMOKE_SOCKS_PORT:-7000}"
  tap_control "Stop Trajectory" "$source_xml" "live_proxy_stop" || true
  for pass in $(seq 1 15); do
    sleep 1
    adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 500 >/dev/null 2>&1 || true
    adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 500 >/dev/null 2>&1 || true
    dump_screen "live_proxy_stopped_${pass}"
    assert_no_android_crash "$artifact_dir/live-proxy-stop-logcat-${pass}.txt"
    if grep -Fq "status.phase.disconnected" "$artifact_dir/live_proxy_stopped_${pass}.xml" &&
      port_is_closed "$http_port" &&
      port_is_closed "$socks_port" &&
      ! adb shell ps -A | grep -E 'trajectory_client|libtrajectory_client' > "$artifact_dir/live-proxy-sidecar-processes.txt"; then
      if [[ "$shutdown_label" == "VPN" ]]; then
        assert_vpn_network_stopped "$artifact_dir/vpn-shutdown-connectivity.txt"
      fi
      echo "${shutdown_label} stopped cleanly after ${pass}s" > "$marker"
      return 0
    fi
  done
  echo "Android ${shutdown_label} smoke did not prove clean shutdown" >&2
  return 1
}

accept_vpn_consent() {
  local probe="$artifact_dir/vpn-consent.raw.xml"
  local action
  local none_seen=0
  for _ in $(seq 1 25); do
    if ! timeout 5s adb exec-out uiautomator dump /dev/tty > "$probe" 2>/dev/null; then
      none_seen=0
      sleep 1
      continue
    fi
    if ! action="$(python3 - "$probe" <<'PY'
import re
import sys
import xml.etree.ElementTree as ET

raw = open(sys.argv[1], errors="replace").read()
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
dialog_present = any("Connection request" in text for text in texts) and any("VPN" in text for text in texts)
if not dialog_present:
    print("NONE")
    raise SystemExit(0)
for node in root.iter("node"):
    if node.attrib.get("text") in {"OK", "Connect", "Allow", "Continue"}:
        match = re.match(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]", node.attrib.get("bounds", ""))
        if match:
            x1, y1, x2, y2 = map(int, match.groups())
            print("TAP", (x1 + x2) // 2, (y1 + y2) // 2)
            raise SystemExit(0)
print("WAIT")
PY
    )"; then
      sleep 1
      continue
    fi
    if [[ "$action" == TAP* ]]; then
      none_seen=0
      adb shell input tap ${action#TAP }
      sleep 2
      continue
    fi
    if [[ "$action" == "NONE" ]]; then
      none_seen=$((none_seen + 1))
      if [[ "$none_seen" -ge 3 ]]; then
        return 0
      fi
      sleep 1
      continue
    fi
    if [[ "$action" == "WAIT" ]]; then
      none_seen=0
      sleep 1
      continue
    fi
    none_seen=0
    sleep 1
  done
  echo "Android VPN consent dialog did not dismiss" >&2
  return 1
}

resolve_smoke_probe_uid() {
  local output_prefix="$1"
  local uid_output="${output_prefix}-smokeprobe-uid.txt"
  local package_output="${output_prefix}-smokeprobe-package.txt"
  local probe_uid
  adb shell dumpsys package "$smoke_probe_package" > "$package_output" 2>&1 || true
  probe_uid="$(
    tr -d '\r' < "$package_output" |
      awk -F= '/userId=/ { print $2; exit }'
  )"
  if [[ -z "$probe_uid" ]]; then
    probe_uid="$(
      tr -d '\r' < "$package_output" |
        awk -F= '/appId=/ { print $2; exit }'
    )"
  fi
  if [[ -z "$probe_uid" || ! "$probe_uid" =~ ^[0-9]+$ ]]; then
    echo "Android VPN smoke could not resolve smoke probe UID" >&2
    return 1
  fi
  printf '%s\n' "$probe_uid" > "$uid_output"
  printf '%s\n' "$probe_uid"
}

resolve_trajectory_app_uid() {
  local output_prefix="$1"
  local uid_output="${output_prefix}-trajectory-uid.txt"
  local package_output="${output_prefix}-trajectory-package.txt"
  local app_uid
  adb shell dumpsys package "$package_name" > "$package_output" 2>&1 || true
  app_uid="$(
    tr -d '\r' < "$package_output" |
      awk -F= '/userId=/ { print $2; exit }'
  )"
  if [[ -z "$app_uid" ]]; then
    app_uid="$(
      tr -d '\r' < "$package_output" |
        awk -F= '/appId=/ { print $2; exit }'
    )"
  fi
  if [[ -z "$app_uid" || ! "$app_uid" =~ ^[0-9]+$ ]]; then
    echo "Android VPN smoke could not resolve Trajectory app UID" >&2
    return 1
  fi
  printf '%s\n' "$app_uid" > "$uid_output"
  printf '%s\n' "$app_uid"
}

assert_vpn_network_active() {
  local output="$1"
  local probe_uid
  local app_uid
  local summary_output="${output%.txt}-summary.txt"
  probe_uid="$(resolve_smoke_probe_uid "${output%.txt}")"
  app_uid="$(resolve_trajectory_app_uid "${output%.txt}")"
  timeout 10s adb shell dumpsys connectivity > "$output" 2>&1
  python3 - "$output" "$summary_output" "$probe_uid" "$app_uid" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(errors="replace")
summary_path = Path(sys.argv[2])
probe_uid = int(sys.argv[3])
app_uid = int(sys.argv[4])


def network_blocks(source: str) -> list[str]:
    matches = list(re.finditer(r"(?:^|\n)\s*NetworkAgentInfo\{", source))
    blocks = []
    for index, match in enumerate(matches):
        start = match.start()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        blocks.append(source[start:end])
    return blocks


def uid_in_ranges(block: str, uid: int) -> bool:
    match = re.search(r"Uids:\s*<\{([^}]*)\}>", block, re.S)
    if not match:
        return False
    for entry in match.group(1).split(","):
        entry = entry.strip()
        if not entry:
            continue
        if "-" in entry:
            start, end = [int(part) for part in entry.split("-", 1)]
        else:
            start = end = int(entry)
        if start <= uid <= end:
            return True
    return False


def owned_by_trajectory(block: str) -> bool:
    owner_matches = re.search(rf"\bOwnerUid:\s*{app_uid}\b", block) is not None
    admin_matches = re.search(rf"\bAdminUids:\s*\[[^\]]*\b{app_uid}\b", block) is not None
    establishing_matches = re.search(rf"\bEstablishingAppUid:\s*{app_uid}\b", block) is not None
    session_matches = "sessionId=Trajectory" in block
    return owner_matches or admin_matches or establishing_matches or session_matches


vpn_blocks = [
    block
    for block in network_blocks(text)
    if "VPN" in block and ("ni{VPN CONNECTED" in block or "Transports:" in block)
]
trajectory_blocks = [block for block in vpn_blocks if owned_by_trajectory(block)]
if not trajectory_blocks:
    summary_path.write_text(
        f"no Trajectory-owned VPN block found for app_uid={app_uid}; vpn_blocks={len(vpn_blocks)}\n",
        encoding="utf-8",
    )
    raise SystemExit("Android VPN network was not visible in dumpsys connectivity")

for block in trajectory_blocks:
    has_tun = re.search(r"InterfaceName:\s*tun\d+", block) is not None
    routes_probe = uid_in_ranges(block, probe_uid)
    validated = "VALIDATED" in block
    owner = re.search(r"\bOwnerUid:\s*([0-9]+)", block)
    establishing = re.search(r"\bEstablishingAppUid:\s*([0-9]+)", block)
    network = re.search(r"network\{([^}]+)\}", block)
    summary_path.write_text(
        "trajectory vpn candidate\n"
        f"network={network.group(1) if network else 'unknown'}\n"
        f"app_uid={app_uid}\n"
        f"owner_uid={owner.group(1) if owner else 'unknown'}\n"
        f"establishing_app_uid={establishing.group(1) if establishing else 'unknown'}\n"
        f"probe_uid={probe_uid}\n"
        f"has_tun={has_tun}\n"
        f"routes_probe_uid={routes_probe}\n"
        f"validated={validated}\n",
        encoding="utf-8",
    )
    if not has_tun:
        continue
    if not routes_probe:
        continue
    raise SystemExit(0)

raise SystemExit(f"smoke probe UID {probe_uid} is not routed through the Trajectory VPN")
PY
}

assert_vpn_network_stopped() {
  local output="$1"
  local probe_uid
  local app_uid
  local summary_output="${output%.txt}-summary.txt"
  probe_uid="$(resolve_smoke_probe_uid "${output%.txt}")"
  app_uid="$(resolve_trajectory_app_uid "${output%.txt}")"
  timeout 10s adb shell dumpsys connectivity > "$output" 2>&1
  python3 - "$output" "$summary_output" "$probe_uid" "$app_uid" <<'PY'
import re
import sys
from pathlib import Path

text = Path(sys.argv[1]).read_text(errors="replace")
summary_path = Path(sys.argv[2])
probe_uid = int(sys.argv[3])
app_uid = int(sys.argv[4])


def network_blocks(source: str) -> list[str]:
    matches = list(re.finditer(r"(?:^|\n)\s*NetworkAgentInfo\{", source))
    blocks = []
    for index, match in enumerate(matches):
        start = match.start()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        blocks.append(source[start:end])
    return blocks


def uid_in_ranges(block: str, uid: int) -> bool:
    match = re.search(r"Uids:\s*<\{([^}]*)\}>", block, re.S)
    if not match:
        return False
    for entry in match.group(1).split(","):
        entry = entry.strip()
        if not entry:
            continue
        if "-" in entry:
            start, end = [int(part) for part in entry.split("-", 1)]
        else:
            start = end = int(entry)
        if start <= uid <= end:
            return True
    return False


def owned_by_trajectory(block: str) -> bool:
    owner_matches = re.search(rf"\bOwnerUid:\s*{app_uid}\b", block) is not None
    admin_matches = re.search(rf"\bAdminUids:\s*\[[^\]]*\b{app_uid}\b", block) is not None
    establishing_matches = re.search(rf"\bEstablishingAppUid:\s*{app_uid}\b", block) is not None
    session_matches = "sessionId=Trajectory" in block
    return owner_matches or admin_matches or establishing_matches or session_matches


remaining = []
for block in network_blocks(text):
    if "VPN" not in block:
        continue
    if owned_by_trajectory(block):
        remaining.append("trajectory-owned")
    elif uid_in_ranges(block, probe_uid):
        remaining.append("probe-routed-through-other-vpn")

summary_path.write_text(
    f"app_uid={app_uid}\nprobe_uid={probe_uid}\nremaining_vpn_matches={','.join(remaining) or 'none'}\n",
    encoding="utf-8",
)
if remaining:
    raise SystemExit(
        f"Trajectory VPN network still present after shutdown or smoke probe UID {probe_uid} is still VPN-routed"
    )
raise SystemExit(0)
PY
}

run_vpn_probe_app() {
  local http_fetch_url="$1"
  local output_prefix="$2"
  install_smoke_probe_apk
  adb shell am force-stop "$smoke_probe_package" >/dev/null 2>&1 || true
  adb shell run-as "$smoke_probe_package" rm -f files/result.txt >/dev/null 2>&1 || true
  adb shell am start -W \
    -n "$smoke_probe_package/.SmokeProbeActivity" \
    --es trajectory_smoke_fetch_url "$http_fetch_url" \
    --es trajectory_smoke_expect_body "${TRAJECTORY_ANDROID_SMOKE_EXPECT_BODY:-}" \
    > "$artifact_dir/${output_prefix}-start.txt" 2>&1

  local pass
  for pass in $(seq 1 30); do
    sleep 1
    adb shell run-as "$smoke_probe_package" cat files/result.txt \
      > "$artifact_dir/${output_prefix}-result-${pass}.txt" 2>/dev/null || true
    if grep -Fq "passed" "$artifact_dir/${output_prefix}-result-${pass}.txt"; then
      dump_screen "${output_prefix}_passed_${pass}"
      adb shell am force-stop "$smoke_probe_package" >/dev/null 2>&1 || true
      return 0
    fi
    if grep -Fq "failed" "$artifact_dir/${output_prefix}-result-${pass}.txt"; then
      dump_screen "${output_prefix}_failed_${pass}"
      echo "Android VPN probe app failed to fetch deterministic body through VPN" >&2
      return 1
    fi
  done
  dump_screen "${output_prefix}_timeout"
  echo "Android VPN probe app timed out waiting for deterministic HTTP response" >&2
  return 1
}

run_vpn_smoke() {
  if [[ "${TRAJECTORY_ANDROID_SMOKE_REQUIRE_VPN:-}" != "1" ]]; then
    echo "Android VPN smoke skipped; TRAJECTORY_ANDROID_SMOKE_REQUIRE_VPN is not set" \
      > "$artifact_dir/vpn-smoke-skipped.txt"
    return 0
  fi
  if [[ -z "${TRAJECTORY_ANDROID_SMOKE_DOMAIN:-}" || -z "${TRAJECTORY_ANDROID_SMOKE_ACCESS_KEY:-}" ]]; then
    echo "Android VPN smoke requires the live/local smoke profile" >&2
    return 1
  fi
  local http_fetch_url="${TRAJECTORY_ANDROID_SMOKE_FETCH_URL:-}"
  if [[ "$http_fetch_url" != http://* ]]; then
    echo "Android VPN smoke requires a deterministic plain http:// fetch URL" >&2
    return 1
  fi

  start_live_profile_from_intent \
    "$TRAJECTORY_ANDROID_SMOKE_DOMAIN" \
    "$TRAJECTORY_ANDROID_SMOKE_ACCESS_KEY" \
    "${TRAJECTORY_ANDROID_SMOKE_RESOLVERS:-}"
  tap_control "Start VPN" "$artifact_dir/live_smoke_configured.xml" "vpn_live"
  accept_vpn_consent
  for pass in $(seq 1 45); do
    sleep 1
    adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 500 >/dev/null 2>&1 || true
    adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 500 >/dev/null 2>&1 || true
    dump_screen "vpn_live_${pass}"
    assert_no_android_crash "$artifact_dir/vpn-live-logcat-${pass}.txt"
    if grep -Fq 'package="com.android.vpndialogs"' "$artifact_dir/vpn_live_${pass}.xml"; then
      accept_vpn_consent
      continue
    fi
    if grep -Fq "VPN connected" "$artifact_dir/vpn_live_${pass}.xml" ||
      grep -Fq "status.phase.vpn_connected" "$artifact_dir/vpn_live_${pass}.xml"; then
      install_smoke_probe_apk
      assert_vpn_network_active "$artifact_dir/vpn-connectivity.txt"
      run_vpn_probe_app "$http_fetch_url" "vpn_probe"
      adb shell am start -W -n "$activity" > "$artifact_dir/vpn-return-main.txt" 2>&1
      sleep 1
      dump_screen "vpn_live_proven_${pass}"
      assert_vpn_status_connected "$artifact_dir/vpn_live_proven_${pass}.xml"
      echo "VPN connected after ${pass}s" > "$artifact_dir/vpn-smoke.txt"
      assert_clean_proxy_shutdown "$artifact_dir/vpn_live_proven_${pass}.xml" "VPN" "$artifact_dir/vpn-clean-shutdown.txt"
      return 0
    fi
    if grep -Fq "Failed" "$artifact_dir/vpn_live_${pass}.xml"; then
      echo "Android VPN smoke reached Failed state" >&2
      return 1
    fi
  done
  echo "Timed out waiting for Android VPN to connect" >&2
  return 1
}

run_live_proxy_smoke() {
  prepare_local_live_proxy_smoke
  local domain="${TRAJECTORY_ANDROID_SMOKE_DOMAIN:-}"
  local access_key="${TRAJECTORY_ANDROID_SMOKE_ACCESS_KEY:-}"
  local http_fetch_url="${TRAJECTORY_ANDROID_SMOKE_FETCH_URL:-http://example.com/}"
  local http_port="${TRAJECTORY_ANDROID_SMOKE_HTTP_PORT:-7001}"
  if [[ "$http_fetch_url" != http://* ]]; then
    echo "Android live proxy smoke requires a plain http:// URL; using http://example.com/ instead of $http_fetch_url" \
      > "$artifact_dir/live-proxy-fetch-url.txt"
    http_fetch_url="http://example.com/"
  fi
  local http_host="$http_fetch_url"
  http_host="${http_host#*//}"
  http_host="${http_host%%/*}"

  if [[ -z "$domain" || -z "$access_key" ]]; then
    if [[ "${TRAJECTORY_ANDROID_SMOKE_ALLOW_OFFLINE:-}" != "1" ]]; then
      echo "Android live proxy smoke requires TRAJECTORY_ANDROID_SMOKE_DOMAIN/ACCESS_KEY or TRAJECTORY_ANDROID_SMOKE_LOCAL_SERVER=1" >&2
      return 1
    fi
    echo "Android live proxy smoke skipped; TRAJECTORY_ANDROID_SMOKE_DOMAIN/ACCESS_KEY are not set" \
      > "$artifact_dir/live-proxy-smoke-skipped.txt"
    return 0
  fi

  if [[ "${TRAJECTORY_ANDROID_SMOKE_LOCAL_SERVER:-}" == "1" ]]; then
    start_live_profile_from_intent "$domain" "$access_key" "${TRAJECTORY_ANDROID_SMOKE_RESOLVERS:-}"
    tap_control "Start proxy" "$artifact_dir/live_smoke_configured.xml" "live_smoke"
    for pass in $(seq 1 60); do
      sleep 1
      dump_screen "live_proxy_${pass}"
      if fetch_android_http_proxy "$http_fetch_url" "$http_host" "$http_port" "$artifact_dir/live-proxy-http.txt" &&
        assert_http_proxy_response "$artifact_dir/live-proxy-http.txt" &&
        assert_socks_handshake; then
        dump_screen "live_proxy_proven_${pass}"
        assert_proxy_status_connected "$artifact_dir/live_proxy_proven_${pass}.xml"
        echo "Proxy connected after ${pass}s" > "$artifact_dir/live-proxy-smoke.txt"
        assert_clean_proxy_shutdown "$artifact_dir/live_proxy_proven_${pass}.xml"
        return 0
      fi
      if grep -Fq "Failed" "$artifact_dir/live_proxy_${pass}.xml"; then
        echo "Android live proxy smoke reached Failed state" >&2
        return 1
      fi
    done
    echo "Timed out waiting for Android live proxy to reach Proxy connected" >&2
    return 1
  fi

  tap_node "Profile tab" "$nav_source" || tap_node "Profile" "$nav_source"
  sleep 1
  dump_screen "live_profile"
  tap_first_text_field_after_label "Tunnel domain" "$artifact_dir/live_profile.xml"
  clear_focused_field
  adb_input_text "$domain"
  dump_screen "live_profile_domain"
  tap_first_text_field_after_label "Access key" "$artifact_dir/live_profile_domain.xml" \
    || tap_first_password_field "$artifact_dir/live_profile_domain.xml"
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
  tap_control "Start proxy" "$artifact_dir/live_saved.xml" "live_saved"

  for pass in $(seq 1 60); do
    sleep 1
    dump_screen "live_proxy_${pass}"
    if grep -Fq "Proxy connected" "$artifact_dir/live_proxy_${pass}.xml" ||
      grep -Fq "status.phase.proxy_connected" "$artifact_dir/live_proxy_${pass}.xml"; then
      echo "Proxy connected after ${pass}s" > "$artifact_dir/live-proxy-smoke.txt"
      if ! fetch_android_http_proxy "$http_fetch_url" "$http_host" "$http_port" "$artifact_dir/live-proxy-http.txt"; then
        echo "Android live proxy smoke could not fetch through HTTP proxy on 127.0.0.1:${http_port}" >&2
        return 1
      fi
      assert_http_proxy_response "$artifact_dir/live-proxy-http.txt"
      assert_socks_handshake
      assert_proxy_status_connected "$artifact_dir/live_proxy_${pass}.xml"
      assert_clean_proxy_shutdown "$artifact_dir/live_proxy_${pass}.xml"
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
wait_for_text "status.phase.disconnected" main
assert_texts "$artifact_dir/main.xml" "Trajectory" "Status" "status.phase.disconnected"
xml_files+=("$artifact_dir/main.xml")
adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
sleep 1
dump_screen "main_controls"
cp "$artifact_dir/main_controls.xml" "$artifact_dir/main_controls_all.xml"
for pass in 1 2 3 4; do
  if grep -Fq "Start proxy" "$artifact_dir/main_controls_all.xml" &&
    grep -Fq "Start VPN" "$artifact_dir/main_controls_all.xml"; then
    break
  fi
  adb shell input swipe "$swipe_x" "$swipe_start_y" "$swipe_x" "$swipe_end_y" 600
  sleep 1
  dump_screen "main_controls_${pass}"
  cat "$artifact_dir/main_controls.xml" "$artifact_dir/main_controls_"[0-9]*.xml > "$artifact_dir/main_controls_all.xml"
done
assert_texts "$artifact_dir/main_controls_all.xml" "Controls" "Start proxy" "Start VPN"
xml_files+=("$artifact_dir/main_controls_all.xml")
adb shell input swipe "$swipe_x" "$swipe_restore_start_y" "$swipe_x" "$swipe_restore_end_y" 600
sleep 1

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
    adb shell input swipe "$swipe_x" "$short_swipe_start_y" "$swipe_x" "$short_swipe_end_y" 350
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
  "Start VPN"

run_live_proxy_smoke
run_vpn_smoke

if timeout 15s adb logcat -d > "$artifact_dir/logcat.txt" 2> "$artifact_dir/logcat.stderr"; then
  final_logcat_available=1
else
  final_logcat_available=0
  echo "Android final logcat capture timed out or failed; UI/data-path smoke completed" > "$artifact_dir/logcat.warning"
fi
if [[ "$final_logcat_available" == "1" ]] && grep -E "FATAL EXCEPTION|E AndroidRuntime" "$artifact_dir/logcat.txt"; then
  echo "Android crash detected during UI smoke test" >&2
  exit 1
fi

find "$artifact_dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.xml' -o -name '*.txt' \) \
  -printf '%f\n' | sort > "$artifact_dir/manifest.txt"
