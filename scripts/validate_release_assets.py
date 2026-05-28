#!/usr/bin/env python3
"""Validate that a release directory contains the expected downloadable assets."""

from __future__ import annotations

import argparse
import re
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
) -> None:
    regex = re.compile(pattern)
    matches = [path for path in files if regex.fullmatch(path.name)]
    if len(matches) != count:
        failures.append(f"{label}: expected {count} asset(s) matching {pattern}, found {len(matches)}")
        return
    for match in matches:
        if match.stat().st_size <= 0:
            failures.append(f"{label}: {match.name} is empty")


def main() -> None:
    args = parse_args()
    release_dir = Path(args.release_dir)
    if not release_dir.is_dir():
        raise SystemExit(f"release directory does not exist: {release_dir}")
    if not args.release_tag.startswith("v"):
        raise SystemExit(f"release tag must start with v: {args.release_tag}")

    version = re.escape(args.release_tag[1:])
    tag = re.escape(args.release_tag)
    files = [path for path in release_dir.rglob("*") if path.is_file()]
    failures: list[str] = []

    for target, extension in TARGETS.items():
        require(
            files,
            rf"trajectory-{tag}-{re.escape(target)}-cli{re.escape(extension)}",
            f"CLI bundle {target}",
            failures,
        )
        require(
            files,
            rf"trajectory-v{version}-{re.escape(target)}-SHA256SUMS\.txt",
            f"CLI checksum {target}",
            failures,
        )

    require(files, rf"Trajectory_{version}_.+\.deb", "Linux desktop .deb", failures)
    require(files, rf"Trajectory-{version}-.+\.rpm", "Linux desktop .rpm", failures)
    require(files, rf"Trajectory_{version}_.+\.AppImage", "Linux desktop AppImage", failures)
    require(files, rf"Trajectory_?{version}.*\.dmg", "macOS desktop .dmg", failures, count=2)
    require(files, rf"Trajectory_{re.escape(args.release_tag)}_x86_64-apple-darwin\.app\.tar\.gz", "macOS Intel app bundle archive", failures)
    require(files, rf"Trajectory_{re.escape(args.release_tag)}_aarch64-apple-darwin\.app\.tar\.gz", "macOS Apple Silicon app bundle archive", failures)
    require(files, rf"Trajectory_?{version}.*\.msi", "Windows desktop .msi", failures)
    require(files, rf"Trajectory_?{version}.*\.exe", "Windows desktop setup .exe", failures)
    require(files, rf"trajectory-{tag}-android\.apk", "Android release APK", failures)
    require(files, rf"{tag}-SHA256SUMS\.txt", "merged checksum manifest", failures)

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

    if failures:
        raise SystemExit("\n".join(failures))

    print(f"validated {len(files)} release files for {args.release_tag}")


if __name__ == "__main__":
    main()
