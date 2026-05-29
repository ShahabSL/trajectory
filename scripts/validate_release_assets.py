#!/usr/bin/env python3
"""Validate that a release directory contains the expected downloadable assets."""

from __future__ import annotations

import argparse
import fnmatch
import hashlib
import re
import shutil
import subprocess
import tarfile
import zipfile
from pathlib import Path


TARGETS = {
    "x86_64-unknown-linux-gnu": ".tar.gz",
    "x86_64-pc-windows-msvc": ".zip",
    "x86_64-apple-darwin": ".tar.gz",
    "aarch64-apple-darwin": ".tar.gz",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("release_dir", help="Directory containing merged release artifacts")
    parser.add_argument("--release-tag", required=True, help="Release tag such as v0.1.12")
    return parser.parse_args()


def require(
    files: list[Path],
    pattern: str,
    label: str,
    failures: list[str],
    *,
    count: int = 1,
) -> list[Path]:
    regex = re.compile(pattern)
    matches = [path for path in files if regex.fullmatch(path.name)]
    if len(matches) != count:
        failures.append(f"{label}: expected {count} asset(s) matching {pattern}, found {len(matches)}")
        return []
    for match in matches:
        if match.stat().st_size <= 0:
            failures.append(f"{label}: {match.name} is empty")
    return matches


def optional(
    files: list[Path],
    pattern: str,
    label: str,
    failures: list[str],
    *,
    max_count: int = 1,
) -> list[Path]:
    regex = re.compile(pattern)
    matches = [path for path in files if regex.fullmatch(path.name)]
    if len(matches) > max_count:
        failures.append(f"{label}: expected at most {max_count} asset(s) matching {pattern}, found {len(matches)}")
        return []
    for match in matches:
        if match.stat().st_size <= 0:
            failures.append(f"{label}: {match.name} is empty")
    return matches


def main() -> None:
    args = parse_args()
    release_dir = Path(args.release_dir).resolve()
    if not release_dir.is_dir():
        raise SystemExit(f"release directory does not exist: {release_dir}")
    if not args.release_tag.startswith("v"):
        raise SystemExit(f"release tag must start with v: {args.release_tag}")

    version = re.escape(args.release_tag[1:])
    tag = re.escape(args.release_tag)
    files = [path for path in release_dir.rglob("*") if path.is_file()]
    failures: list[str] = []
    expected_assets: list[Path] = []

    for target, extension in TARGETS.items():
        expected_assets.extend(
            require(
                files,
                rf"trajectory-{tag}-{re.escape(target)}-cli{re.escape(extension)}",
                f"CLI bundle {target}",
                failures,
            )
        )
        expected_assets.extend(
            require(
                files,
                rf"trajectory-v{version}-{re.escape(target)}-SHA256SUMS\.txt",
                f"CLI checksum {target}",
                failures,
            )
        )

    expected_assets.extend(require(files, rf"Trajectory_{version}_.+\.deb", "Linux desktop .deb", failures))
    expected_assets.extend(require(files, rf"Trajectory-{version}-.+\.rpm", "Linux desktop .rpm", failures))
    expected_assets.extend(require(files, rf"Trajectory_{version}_.+\.AppImage", "Linux desktop AppImage", failures))
    expected_assets.extend(require(files, rf"Trajectory_{re.escape(args.release_tag)}_x86_64-apple-darwin\.app\.tar\.gz", "macOS Intel app bundle archive", failures))
    expected_assets.extend(require(files, rf"Trajectory_{re.escape(args.release_tag)}_aarch64-apple-darwin\.app\.tar\.gz", "macOS Apple Silicon app bundle archive", failures))
    expected_assets.extend(optional(files, rf"Trajectory_?{version}.*\.dmg", "macOS desktop .dmg", failures, max_count=2))
    expected_assets.extend(require(files, rf"Trajectory_?{version}.*\.msi", "Windows desktop .msi", failures))
    expected_assets.extend(require(files, rf"Trajectory_?{version}.*\.exe", "Windows desktop setup .exe", failures))
    expected_assets.extend(require(files, rf"trajectory-{tag}-android\.apk", "Android release APK", failures))
    merged_manifests = require(files, rf"{tag}-SHA256SUMS\.txt", "merged checksum manifest", failures)
    expected_assets.extend(merged_manifests)

    unexpected_cli = [
        path.name
        for path in files
        if path.name.startswith("trajectory-v")
        and args.release_tag not in path.name
        and path.suffix in {".zip", ".gz", ".txt"}
    ]
    if unexpected_cli:
        failures.append(
            "found version-mismatched trajectory assets: " + ", ".join(sorted(unexpected_cli))
        )

    expected_names = {path.name for path in expected_assets}
    unexpected_publishable = [
        path.name
        for path in files
        if is_release_asset(path.name) and path.name not in expected_names
    ]
    if unexpected_publishable:
        failures.append(
            "found unexpected publishable assets: " + ", ".join(sorted(unexpected_publishable))
        )

    if merged_manifests:
        verify_merged_checksums(merged_manifests[0], expected_assets, failures)
    verify_all_checksum_manifests(files, failures)
    verify_asset_structures(expected_assets, args.release_tag, failures)

    if failures:
        raise SystemExit("\n".join(failures))

    print(f"validated {len(files)} release files for {args.release_tag}")


def is_release_asset(name: str) -> bool:
    patterns = [
        "trajectory-v*.tar.gz",
        "trajectory-v*.zip",
        "*SHA256SUMS.txt",
        "*.deb",
        "*.rpm",
        "*.AppImage",
        "*.msi",
        "*.exe",
        "*.dmg",
        "*.app.tar.gz",
        "*.apk",
    ]
    return any(fnmatch.fnmatch(name, pattern) for pattern in patterns)


def verify_merged_checksums(
    manifest: Path,
    expected_assets: list[Path],
    failures: list[str],
) -> None:
    checksums: dict[str, str] = {}
    for line in manifest.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        parts = stripped.split(None, 1)
        if len(parts) != 2:
            failures.append(f"malformed checksum line in {manifest.name}: {line}")
            continue
        digest, filename = parts
        checksums[Path(filename).name] = digest

    assets_to_verify = [
        path for path in expected_assets if not path.name.endswith("SHA256SUMS.txt")
    ]
    expected_names = {path.name for path in assets_to_verify}
    if set(checksums) != expected_names:
        missing = sorted(expected_names - set(checksums))
        extra = sorted(set(checksums) - expected_names)
        if missing:
            failures.append("merged checksum manifest is missing: " + ", ".join(missing))
        if extra:
            failures.append("merged checksum manifest has unexpected entries: " + ", ".join(extra))

    for asset in assets_to_verify:
        actual = sha256(asset)
        expected = checksums.get(asset.name)
        if expected and actual != expected:
            failures.append(f"checksum mismatch for {asset.name}: expected {expected}, got {actual}")


def verify_all_checksum_manifests(files: list[Path], failures: list[str]) -> None:
    by_name = {path.name: path for path in files}
    manifests = sorted(path for path in files if path.name.endswith("SHA256SUMS.txt"))
    for manifest in manifests:
        for line in manifest.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if not stripped:
                continue
            parts = stripped.split(None, 1)
            if len(parts) != 2:
                failures.append(f"malformed checksum line in {manifest.name}: {line}")
                continue
            expected, filename = parts
            asset_name = Path(filename).name
            asset = by_name.get(asset_name)
            if asset is None:
                failures.append(f"{manifest.name} references missing asset: {asset_name}")
                continue
            actual = sha256(asset)
            if actual != expected:
                failures.append(
                    f"checksum mismatch for {asset_name} in {manifest.name}: expected {expected}, got {actual}"
                )


def verify_asset_structures(
    assets: list[Path],
    release_tag: str,
    failures: list[str],
) -> None:
    version = release_tag[1:]
    for asset in assets:
        name = asset.name
        try:
            if name.endswith(".zip"):
                with zipfile.ZipFile(asset) as bundle:
                    bad = bundle.testzip()
                    if bad:
                        failures.append(f"{name}: corrupt zip member {bad}")
                    require_archive_member(bundle.namelist(), "trajectory-client", name, failures)
                    require_archive_member(bundle.namelist(), "trajectory-server", name, failures)
                    require_archive_member(bundle.namelist(), "trajectory-admin", name, failures)
            elif name.endswith(".tar.gz"):
                with tarfile.open(asset, "r:gz") as bundle:
                    members = bundle.getnames()
                    if name.endswith(".app.tar.gz"):
                        require_archive_member(members, "Trajectory.app/Contents/MacOS", name, failures)
                        require_archive_member(members, "trajectory-client", name, failures)
                        require_archive_member_predicate(
                            members,
                            lambda member: (
                                "Trajectory.app/Contents/MacOS/" in member
                                and not Path(member).name.startswith("trajectory-client")
                                and Path(member).name not in {"", "MacOS"}
                            ),
                            "app launcher under Trajectory.app/Contents/MacOS",
                            name,
                            failures,
                        )
                    else:
                        require_archive_member(members, "trajectory-client", name, failures)
                        require_archive_member(members, "trajectory-server", name, failures)
                        require_archive_member(members, "trajectory-admin", name, failures)
            elif name.endswith(".deb"):
                require_magic(asset, b"!<arch>\n", name, ".deb ar container", failures)
                run_required(["dpkg-deb", "--info", str(asset)], f"{name} dpkg metadata", failures)
                contents = run_required(["dpkg-deb", "--contents", str(asset)], f"{name} dpkg contents", failures)
                if contents:
                    require_payload_members(
                        contents,
                        ["trajectory-desktop", "trajectory-client", "Trajectory.desktop"],
                        name,
                        failures,
                    )
            elif name.endswith(".rpm"):
                require_magic(asset, b"\xed\xab\xee\xdb", name, ".rpm lead", failures)
                contents = rpm_payload_contents(asset, name, failures)
                if contents:
                    require_payload_members(
                        contents,
                        ["trajectory-desktop", "trajectory-client", "Trajectory.desktop"],
                        name,
                        failures,
                    )
            elif name.endswith(".AppImage"):
                require_magic(asset, b"\x7fELF", name, "AppImage ELF header", failures)
                if version not in name:
                    failures.append(f"{name}: AppImage filename does not include version {version}")
            elif name.endswith(".msi"):
                require_magic(asset, b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1", name, "MSI OLE header", failures)
                run_required(["7z", "t", str(asset)], f"{name} 7z integrity", failures)
            elif name.endswith(".exe"):
                require_magic(asset, b"MZ", name, "Windows executable header", failures)
                run_required(["7z", "t", str(asset)], f"{name} setup archive integrity", failures)
            elif name.endswith(".dmg"):
                tail = asset.read_bytes()[-512:]
                if b"koly" not in tail:
                    failures.append(f"{name}: missing UDIF koly trailer")
            elif name.endswith(".apk"):
                with zipfile.ZipFile(asset) as apk:
                    bad = apk.testzip()
                    if bad:
                        failures.append(f"{name}: corrupt APK member {bad}")
                    apk_names = set(apk.namelist())
                    for member in [
                        "AndroidManifest.xml",
                        "lib/arm64-v8a/libtrajectory_client.so",
                        "lib/arm64-v8a/libtrajectory_vpn_bridge.so",
                        "lib/x86_64/libtrajectory_client.so",
                        "lib/x86_64/libtrajectory_vpn_bridge.so",
                    ]:
                        if member not in apk_names:
                            failures.append(f"{name}: missing {member}")
                if version not in name:
                    failures.append(f"{name}: APK filename does not include version {version}")
        except (tarfile.TarError, zipfile.BadZipFile, OSError) as error:
            failures.append(f"{name}: could not inspect asset structure: {error}")


def require_archive_member(
    members: list[str],
    needle: str,
    asset_name: str,
    failures: list[str],
) -> None:
    if not any(needle in member for member in members):
        failures.append(f"{asset_name}: missing archive member containing {needle}")


def require_archive_member_predicate(
    members: list[str],
    predicate,
    label: str,
    asset_name: str,
    failures: list[str],
) -> None:
    if not any(predicate(member) for member in members):
        failures.append(f"{asset_name}: missing archive member matching {label}")


def require_magic(
    path: Path,
    magic: bytes,
    asset_name: str,
    label: str,
    failures: list[str],
) -> None:
    if path.read_bytes()[: len(magic)] != magic:
        failures.append(f"{asset_name}: missing {label}")


def run_required(args: list[str], label: str, failures: list[str]) -> str | None:
    if not shutil.which(args[0]):
        failures.append(f"{label} skipped: required tool not found: {args[0]}")
        return None
    result = subprocess.run(
        args,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=30,
    )
    if result.returncode != 0:
        failures.append(f"{label} failed: {result.stderr.strip()}")
        return None
    return result.stdout


def rpm_payload_contents(asset: Path, asset_name: str, failures: list[str]) -> str | None:
    rpm2cpio = shutil.which("rpm2cpio")
    cpio = shutil.which("cpio")
    missing_tools = [tool for tool, path in [("rpm2cpio", rpm2cpio), ("cpio", cpio)] if path is None]
    if missing_tools:
        failures.append(
            f"{asset_name}: rpm payload validation skipped; required tool(s) not found: "
            + ", ".join(missing_tools)
        )
        return None

    rpm_process = subprocess.Popen(
        [rpm2cpio, str(asset)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    assert rpm_process.stdout is not None
    cpio_result = subprocess.run(
        [cpio, "-t"],
        stdin=rpm_process.stdout,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=30,
    )
    rpm_process.stdout.close()
    _, rpm_stderr = rpm_process.communicate(timeout=30)
    if cpio_result.returncode != 0:
        rpm_detail = rpm_stderr.decode("utf-8", errors="replace").strip()
        cpio_detail = cpio_result.stderr.decode("utf-8", errors="replace").strip()
        failures.append(
            f"{asset_name}: cpio payload listing failed: "
            + cpio_detail
            + (f"; rpm2cpio: {rpm_detail}" if rpm_detail else "")
        )
        return None
    contents = cpio_result.stdout.decode("utf-8", errors="replace")
    if not contents.strip():
        failures.append(f"{asset_name}: rpm payload listing was empty")
        return None
    return contents


def require_payload_members(
    contents: str,
    needles: list[str],
    asset_name: str,
    failures: list[str],
) -> None:
    for needle in needles:
        if needle not in contents:
            failures.append(f"{asset_name}: payload is missing {needle}")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


if __name__ == "__main__":
    main()
