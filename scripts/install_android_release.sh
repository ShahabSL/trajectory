#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APK_PATH="${APK_PATH:-$ROOT_DIR/clients/android/app/build/outputs/apk/release/app-release.apk}"

if ! command -v adb >/dev/null 2>&1; then
  echo "adb is required on PATH" >&2
  exit 1
fi

if [[ ! -f "$APK_PATH" ]]; then
  echo "APK not found at $APK_PATH" >&2
  exit 1
fi

adb install -r "$APK_PATH"
echo "Installed $APK_PATH"
