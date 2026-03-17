#!/usr/bin/env python3
"""Generate Kotlin and Swift UniFFI bindings for the mobile clients."""

from __future__ import annotations

import argparse
import platform
import shutil
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ANDROID_OUT = ROOT / "clients" / "android" / "app" / "src" / "main" / "java"
IOS_OUT = ROOT / "clients" / "ios" / "TrajectoryMobileApp" / "Sources" / "Generated"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Use an existing release build of trajectory-mobile",
    )
    parser.add_argument(
        "--profile",
        choices=("debug", "release"),
        default="debug",
        help="Cargo profile to use for the host library build",
    )
    return parser.parse_args()


def host_library_path(profile: str) -> Path:
    profile_dir = "debug" if profile == "debug" else "release"
    system = platform.system()
    if system == "Linux":
        return ROOT / "target" / profile_dir / "libtrajectory_mobile.so"
    if system == "Darwin":
        return ROOT / "target" / profile_dir / "libtrajectory_mobile.dylib"
    if system == "Windows":
        return ROOT / "target" / profile_dir / "trajectory_mobile.dll"
    raise SystemExit(f"unsupported host platform for bindgen: {system}")


def run(*command: str) -> None:
    subprocess.run(command, cwd=ROOT, check=True)


def main() -> None:
    args = parse_args()
    if not args.skip_build:
        command = ["cargo", "build", "-p", "trajectory-mobile"]
        if args.profile == "release":
            command.insert(2, "--release")
        run(*command)

    library = host_library_path(args.profile)
    if not library.exists():
        raise SystemExit(f"expected mobile library not found: {library}")

    kotlin_package_dir = ANDROID_OUT / "uniffi" / "trajectorymobile"
    if kotlin_package_dir.exists():
        shutil.rmtree(kotlin_package_dir)
    IOS_OUT.mkdir(parents=True, exist_ok=True)
    for generated in IOS_OUT.glob("trajectorymobile*"):
        generated.unlink()
    for generated in IOS_OUT.glob("Trajectorymobile*"):
        generated.unlink()

    run(
        "cargo",
        "run",
        "-p",
        "trajectory-mobile",
        "--bin",
        "uniffi-bindgen",
        "--",
        "generate",
        "--library",
        str(library),
        "--language",
        "kotlin",
        "--out-dir",
        str(ANDROID_OUT),
    )
    run(
        "cargo",
        "run",
        "-p",
        "trajectory-mobile",
        "--bin",
        "uniffi-bindgen",
        "--",
        "generate",
        "--library",
        str(library),
        "--language",
        "swift",
        "--out-dir",
        str(IOS_OUT),
    )

    print(f"generated Kotlin bindings in {kotlin_package_dir}")
    print(f"generated Swift bindings in {IOS_OUT}")


if __name__ == "__main__":
    main()
