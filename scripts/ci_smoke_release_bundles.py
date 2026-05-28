#!/usr/bin/env python3
"""Smoke-test packaged CLI release bundles by extracting and running help."""

from __future__ import annotations

import argparse
import subprocess
import tarfile
import tempfile
import zipfile
from pathlib import Path


EXPECTED_BINARIES = ("trajectory-client", "trajectory-admin", "trajectory-server")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_dir", help="Directory containing packaged release archives")
    return parser.parse_args()


def archive_paths(artifact_dir: Path) -> list[Path]:
    return sorted(
        [
            *artifact_dir.glob("trajectory-v*-cli.tar.gz"),
            *artifact_dir.glob("trajectory-v*-cli.zip"),
        ]
    )


def extract_archive(archive: Path, destination: Path) -> None:
    if archive.suffix == ".zip":
        with zipfile.ZipFile(archive) as bundle:
            bundle.extractall(destination)
        return

    if archive.name.endswith(".tar.gz"):
        with tarfile.open(archive, "r:gz") as bundle:
            try:
                bundle.extractall(destination, filter="data")
            except TypeError:
                bundle.extractall(destination)
        return

    raise SystemExit(f"unsupported archive format: {archive}")


def find_binary(extract_dir: Path, name: str) -> Path:
    candidates = [*extract_dir.rglob(name), *extract_dir.rglob(f"{name}.exe")]
    if len(candidates) != 1:
        raise SystemExit(f"expected one {name} binary in {extract_dir}, found {candidates}")
    return candidates[0]


def smoke_binary(binary: Path) -> None:
    result = subprocess.run(
        [str(binary), "--help"],
        check=True,
        capture_output=True,
        text=True,
    )
    if "Usage:" not in result.stdout:
        raise SystemExit(f"{binary} --help did not print usage text")


def main() -> None:
    artifact_dir = Path(parse_args().artifact_dir).resolve()
    archives = archive_paths(artifact_dir)
    if not archives:
        raise SystemExit(f"no CLI release archives found in {artifact_dir}")

    for archive in archives:
        with tempfile.TemporaryDirectory(prefix=f"{archive.stem}-") as temp_dir:
            extract_dir = Path(temp_dir)
            extract_archive(archive, extract_dir)
            for binary_name in EXPECTED_BINARIES:
                smoke_binary(find_binary(extract_dir, binary_name))
            print(f"smoked {archive.name}")


if __name__ == "__main__":
    main()
