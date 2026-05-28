#!/usr/bin/env bash
set -euo pipefail

api_level="${1:?usage: ci_android_reclaim_emulator_disk.sh <api-level>}"

adb kill-server >/dev/null 2>&1 || true
rm -rf "${ANDROID_AVD_HOME:-$HOME/.android/avd}" "$HOME/.android/avd" || true

if [[ -n "${ANDROID_HOME:-}" ]]; then
  rm -rf "$ANDROID_HOME/system-images/android-${api_level}" || true
fi

df -h
