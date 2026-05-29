#!/usr/bin/env python3
"""Fail release packaging when versioned project files disagree with the tag."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--release-tag", required=True, help="Release tag such as v0.1.12")
    return parser.parse_args()


def read_json(path: str) -> dict:
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def read_toml(path: str) -> dict:
    return tomllib.loads((ROOT / path).read_text(encoding="utf-8"))


def cargo_versions() -> dict[str, str]:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    metadata = json.loads(result.stdout)
    wanted = {
        "trajectory-core",
        "trajectory-cli",
        "trajectory-vpn-bridge",
        "trajectory-desktop",
    }
    return {
        package["name"]: package["version"]
        for package in metadata["packages"]
        if package["name"] in wanted
    }


def android_version_name() -> str:
    content = (ROOT / "clients/android/app/build.gradle.kts").read_text(encoding="utf-8")
    match = re.search(r'^\s*versionName\s*=\s*"([^"]+)"', content, re.MULTILINE)
    if not match:
        raise SystemExit("clients/android/app/build.gradle.kts is missing versionName")
    return match.group(1)


def main() -> None:
    args = parse_args()
    if not args.release_tag.startswith("v"):
        raise SystemExit(f"release tag must start with v: {args.release_tag}")
    expected = args.release_tag[1:]

    checks = {
        "Cargo workspace": cargo_versions(),
        "package.json": {
            "trajectory": read_json("package.json")["version"],
        },
        "clients/desktop/package.json": {
            "@trajectory/desktop": read_json("clients/desktop/package.json")["version"],
        },
        "clients/desktop/src-tauri/tauri.conf.json": {
            "Trajectory": read_json("clients/desktop/src-tauri/tauri.conf.json")["version"],
        },
        "clients/desktop/src-tauri/Cargo.toml": {
            "trajectory-desktop": read_toml("clients/desktop/src-tauri/Cargo.toml")["package"][
                "version"
            ],
        },
        "clients/android/app/build.gradle.kts": {
            "app.trajectory.android": android_version_name(),
        },
    }

    failures: list[str] = []
    for source, versions in checks.items():
        for name, version in versions.items():
            if version != expected:
                failures.append(f"{source}: {name} is {version}, expected {expected}")

    if failures:
        raise SystemExit("\n".join(failures))

    print(f"all release versions match {args.release_tag}")


if __name__ == "__main__":
    main()
