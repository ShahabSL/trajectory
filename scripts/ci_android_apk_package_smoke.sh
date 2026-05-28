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

{
  echo "required_entries=ok"
  echo "arm64_client_bytes=$(unzip -p "$apk" lib/arm64-v8a/libtrajectory_client.so | wc -c | tr -d ' ')"
  echo "arm64_bridge_bytes=$(unzip -p "$apk" lib/arm64-v8a/libtrajectory_vpn_bridge.so | wc -c | tr -d ' ')"
  echo "x86_64_client_bytes=$(unzip -p "$apk" lib/x86_64/libtrajectory_client.so | wc -c | tr -d ' ')"
  echo "x86_64_bridge_bytes=$(unzip -p "$apk" lib/x86_64/libtrajectory_vpn_bridge.so | wc -c | tr -d ' ')"
} > "$artifact_dir/native-libs.txt"

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
fi

echo "Android APK package smoke passed" > "$artifact_dir/package-smoke.txt"
