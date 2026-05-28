#!/usr/bin/env bash
set -euo pipefail

apk="${1:?usage: validate_android_release_apk.sh <apk> <release-tag> [artifact-dir]}"
release_tag="${2:?usage: validate_android_release_apk.sh <apk> <release-tag> [artifact-dir]}"
artifact_dir="${3:-${RUNNER_TEMP:-/tmp}/trajectory-android-release-validation}"
expected_version="${release_tag#v}"
expected_package="app.trajectory.android"
expected_min_sdk="26"
expected_target_sdk="35"

mkdir -p "$artifact_dir"
test -f "$apk"

build_tools="${ANDROID_HOME:?ANDROID_HOME is required}/build-tools/35.0.0"
apksigner="$build_tools/apksigner"
aapt2="$build_tools/aapt2"
test -x "$apksigner"
test -x "$aapt2"

"$apksigner" verify --verbose --print-certs "$apk" > "$artifact_dir/apksigner.txt"
"$aapt2" dump badging "$apk" > "$artifact_dir/badging.txt"
"$aapt2" dump permissions "$apk" > "$artifact_dir/permissions.txt"
"$aapt2" dump xmltree "$apk" --file AndroidManifest.xml > "$artifact_dir/manifest.xmltree.txt"

grep -Fq "package: name='$expected_package'" "$artifact_dir/badging.txt"
grep -Fq "versionName='$expected_version'" "$artifact_dir/badging.txt"
grep -Eq "(sdkVersion|minSdkVersion):'$expected_min_sdk'" "$artifact_dir/badging.txt"
grep -Fq "targetSdkVersion:'$expected_target_sdk'" "$artifact_dir/badging.txt"
grep -Fq "launchable-activity: name='$expected_package.MainActivity'" "$artifact_dir/badging.txt"

grep -Fq "android.permission.INTERNET" "$artifact_dir/permissions.txt"
grep -Fq "android.permission.FOREGROUND_SERVICE" "$artifact_dir/permissions.txt"
grep -Fq "android.permission.FOREGROUND_SERVICE_SPECIAL_USE" "$artifact_dir/permissions.txt"

grep -Fq "android.permission.BIND_VPN_SERVICE" "$artifact_dir/manifest.xmltree.txt"
grep -Fq "android.net.VpnService" "$artifact_dir/manifest.xmltree.txt"
grep -Fq "android.net.VpnService.SUPPORTS_ALWAYS_ON" "$artifact_dir/manifest.xmltree.txt"
grep -Fq "android.app.PROPERTY_SPECIAL_USE_FGS_SUBTYPE" "$artifact_dir/manifest.xmltree.txt"
grep -Fq "extractNativeLibs" "$artifact_dir/manifest.xmltree.txt"

printf 'validated Android release APK %s for %s\n' "$apk" "$release_tag" > "$artifact_dir/summary.txt"
