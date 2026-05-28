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

python3 - "$apk" "$artifact_dir/native-libs.txt" <<'PY'
import struct
import sys
import zipfile
from pathlib import Path

apk = Path(sys.argv[1])
report = Path(sys.argv[2])
expected = {
    "lib/arm64-v8a/libtrajectory_client.so": 183,
    "lib/arm64-v8a/libtrajectory_vpn_bridge.so": 183,
    "lib/x86_64/libtrajectory_client.so": 62,
    "lib/x86_64/libtrajectory_vpn_bridge.so": 62,
}

lines = []
with zipfile.ZipFile(apk) as archive:
    names = set(archive.namelist())
    missing = sorted(set(expected) - names)
    if missing:
        raise SystemExit(f"APK missing native libraries: {', '.join(missing)}")
    for name, machine in expected.items():
        data = archive.read(name)
        if len(data) < 20:
            raise SystemExit(f"{name} is too small to be an ELF shared object")
        if data[:4] != b"\x7fELF":
            raise SystemExit(f"{name} does not start with ELF magic")
        if data[4] != 2:
            raise SystemExit(f"{name} is not a 64-bit ELF object")
        if data[5] != 1:
            raise SystemExit(f"{name} is not little-endian ELF")
        actual_machine = struct.unpack_from("<H", data, 18)[0]
        if actual_machine != machine:
            raise SystemExit(
                f"{name} has ELF machine {actual_machine}, expected {machine}"
            )
        lines.append(f"{name}: elf64 machine={actual_machine} bytes={len(data)}")

report.write_text("\n".join(lines) + "\n", encoding="utf-8")
PY

printf 'validated Android release APK %s for %s\n' "$apk" "$release_tag" > "$artifact_dir/summary.txt"
