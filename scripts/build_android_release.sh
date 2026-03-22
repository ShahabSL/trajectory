#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANDROID_PROJECT_DIR="$ROOT_DIR/clients/android"
WRAPPER_GRADLE="$ANDROID_PROJECT_DIR/gradlew"
DEFAULT_GRADLE="$ROOT_DIR/.tooling/gradle-8.13/bin/gradle"

if [[ -z "${GRADLE_BIN:-}" ]]; then
  if [[ -x "$WRAPPER_GRADLE" ]]; then
    GRADLE_BIN="$WRAPPER_GRADLE"
  elif [[ -x "$DEFAULT_GRADLE" ]]; then
    GRADLE_BIN="$DEFAULT_GRADLE"
  elif command -v gradle >/dev/null 2>&1; then
    GRADLE_BIN="$(command -v gradle)"
  else
    echo "gradle not found; set GRADLE_BIN or install Gradle" >&2
    exit 1
  fi
fi

if [[ -z "${ANDROID_SDK_ROOT:-}" ]]; then
  if [[ -d "$ROOT_DIR/.tooling/android-sdk" ]]; then
    export ANDROID_SDK_ROOT="$ROOT_DIR/.tooling/android-sdk"
  else
    echo "ANDROID_SDK_ROOT is not set" >&2
    exit 1
  fi
fi
export ANDROID_HOME="${ANDROID_HOME:-$ANDROID_SDK_ROOT}"

if [[ -z "${JAVA_HOME:-}" ]]; then
  if command -v java >/dev/null 2>&1; then
    export JAVA_HOME="$(dirname "$(dirname "$(readlink -f "$(command -v java)")")")"
  else
    echo "JAVA_HOME is not set and java is not on PATH" >&2
    exit 1
  fi
fi

if [[ -z "${GRADLE_USER_HOME:-}" ]]; then
  mkdir -p "$ROOT_DIR/.tooling"
  GRADLE_USER_HOME="$ROOT_DIR/.tooling/gradle-user-home"
else
  mkdir -p "$GRADLE_USER_HOME"
fi
export GRADLE_USER_HOME

(
  cd "$ANDROID_PROJECT_DIR"
  "$GRADLE_BIN" --no-daemon assembleRelease
)

echo "Built Android release APK:"
echo "$ANDROID_PROJECT_DIR/app/build/outputs/apk/release/app-release.apk"
