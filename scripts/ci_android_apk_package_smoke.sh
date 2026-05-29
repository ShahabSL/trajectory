#!/usr/bin/env bash
set -euo pipefail

apk="${1:?usage: ci_android_apk_package_smoke.sh <apk> [artifact-dir]}"
artifact_dir="${2:-${RUNNER_TEMP:-/tmp}/trajectory-android-apk-package}"

mkdir -p "$artifact_dir"
test -f "$apk"

unzip -l "$apk" > "$artifact_dir/unzip-list.txt"
zipinfo -1 "$apk" > "$artifact_dir/zip-entries.txt"

required_entries=(
  "AndroidManifest.xml"
  "classes.dex"
  "lib/arm64-v8a/libtrajectory_client.so"
  "lib/arm64-v8a/libtrajectory_vpn_bridge.so"
  "lib/x86_64/libtrajectory_client.so"
  "lib/x86_64/libtrajectory_vpn_bridge.so"
)

for entry in "${required_entries[@]}"; do
  if ! grep -Fxq "$entry" "$artifact_dir/zip-entries.txt"; then
    echo "Android APK package smoke missing required entry: $entry" >&2
    exit 1
  fi
done

native_extract_dir="$artifact_dir/native-libs"
mkdir -p "$native_extract_dir"
readelf_bin=""
if [[ -n "${ANDROID_NDK_HOME:-}" && -x "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf" ]]; then
  readelf_bin="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf"
fi
if [[ -z "$readelf_bin" && -n "${ANDROID_NDK_ROOT:-}" && -x "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf" ]]; then
  readelf_bin="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf"
fi
if [[ -z "$readelf_bin" && -n "${ANDROID_HOME:-}" && -d "$ANDROID_HOME/ndk" ]]; then
  latest_ndk="$(find "$ANDROID_HOME/ndk" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)"
  if [[ -n "${latest_ndk:-}" && -x "$latest_ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf" ]]; then
    readelf_bin="$latest_ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-readelf"
  fi
fi
if [[ -z "$readelf_bin" ]] && command -v llvm-readelf >/dev/null 2>&1; then
  readelf_bin="$(command -v llvm-readelf)"
fi
if [[ -z "$readelf_bin" ]] && command -v readelf >/dev/null 2>&1; then
  readelf_bin="$(command -v readelf)"
fi

if [[ -z "$readelf_bin" ]]; then
  echo "llvm-readelf or readelf is required for Android native alignment smoke" >&2
  exit 1
fi

while IFS= read -r entry; do
  lib_path="$native_extract_dir/$entry"
  mkdir -p "$(dirname "$lib_path")"
  unzip -p "$apk" "$entry" > "$lib_path"
  readelf_log="$artifact_dir/readelf-${entry//\//_}.txt"
  "$readelf_bin" -lW "$lib_path" > "$readelf_log"
  python3 - "$readelf_log" "$entry" <<'PY'
import sys
from pathlib import Path

readelf_log = Path(sys.argv[1])
entry = sys.argv[2]
for line in readelf_log.read_text(encoding="utf-8", errors="replace").splitlines():
    stripped = line.strip()
    if not stripped.startswith("LOAD"):
        continue
    align = stripped.split()[-1]
    if align.startswith("2**"):
        value = 1 << int(align[3:])
    else:
        value = int(align, 0)
    if value < 16384:
        raise SystemExit(f"{entry}: LOAD segment alignment {align} is below 16 KiB")
PY
done < <(grep -E '^lib/.+/.+\.so$' "$artifact_dir/zip-entries.txt")

{
  echo "required_entries=ok"
  echo "arm64_client_bytes=$(unzip -p "$apk" lib/arm64-v8a/libtrajectory_client.so | wc -c | tr -d ' ')"
  echo "arm64_bridge_bytes=$(unzip -p "$apk" lib/arm64-v8a/libtrajectory_vpn_bridge.so | wc -c | tr -d ' ')"
  echo "x86_64_client_bytes=$(unzip -p "$apk" lib/x86_64/libtrajectory_client.so | wc -c | tr -d ' ')"
  echo "x86_64_bridge_bytes=$(unzip -p "$apk" lib/x86_64/libtrajectory_vpn_bridge.so | wc -c | tr -d ' ')"
} > "$artifact_dir/native-libs.txt"
echo "elf_load_alignment_16k=ok" >> "$artifact_dir/native-libs.txt"

if [[ -n "${ANDROID_HOME:-}" && -d "$ANDROID_HOME/build-tools" ]]; then
  latest_build_tools="$(find "$ANDROID_HOME/build-tools" -mindepth 1 -maxdepth 1 -type d | sort -V | tail -n 1)"
  if [[ -n "$latest_build_tools" && -x "$latest_build_tools/apksigner" ]]; then
    if ! "$latest_build_tools/apksigner" verify --verbose --print-certs "$apk" \
      > "$artifact_dir/apksigner.txt" 2> "$artifact_dir/apksigner.stderr"; then
      if [[ "$apk" == *unsigned* ]]; then
        echo "signature=skipped_unsigned_local_apk" >> "$artifact_dir/native-libs.txt"
      else
        cat "$artifact_dir/apksigner.stderr" >&2
        exit 1
      fi
    fi
  fi
  if [[ -n "$latest_build_tools" && -x "$latest_build_tools/aapt" ]]; then
    "$latest_build_tools/aapt" dump badging "$apk" > "$artifact_dir/aapt-badging.txt"
    grep -Fq "package: name='app.trajectory.android'" "$artifact_dir/aapt-badging.txt"
    grep -Fq "'arm64-v8a'" "$artifact_dir/aapt-badging.txt"
    grep -Fq "'x86_64'" "$artifact_dir/aapt-badging.txt"
  fi
  if [[ -n "$latest_build_tools" && -x "$latest_build_tools/zipalign" ]]; then
    "$latest_build_tools/zipalign" -v -c -P 16 4 "$apk" > "$artifact_dir/zipalign-16k.txt"
    echo "zipalign_16k=ok" >> "$artifact_dir/native-libs.txt"
  fi
fi

echo "Android APK package smoke passed" > "$artifact_dir/package-smoke.txt"
