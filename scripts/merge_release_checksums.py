#!/usr/bin/env python3
"""Create one checksum manifest for all final release assets."""

from __future__ import annotations

import argparse
import fnmatch
import hashlib
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--input-dir",
        required=True,
        help="Directory containing per-target SHA256SUMS manifests",
    )
    parser.add_argument("--output", required=True, help="Combined checksum manifest path")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    input_dir = Path(args.input_dir)
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)

    checksums: dict[str, str] = {}
    manifests = sorted(input_dir.glob("trajectory-v*-SHA256SUMS.txt"))
    for manifest in manifests:
        if manifest.resolve() == output.resolve():
            continue
        for line in manifest.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if not stripped:
                continue
            digest, filename = stripped.split(None, 1)
            checksums[Path(filename).name] = digest

    for asset in sorted(input_dir.rglob("*")):
        if not asset.is_file() or asset.resolve() == output.resolve():
            continue
        if asset.name.endswith("SHA256SUMS.txt"):
            continue
        if not is_release_asset(asset.name):
            continue
        checksums[asset.name] = sha256(asset)

    if not checksums:
        raise SystemExit(f"no release assets found in {input_dir}")

    lines = [f"{digest}  {name}" for name, digest in sorted(checksums.items())]
    output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {len(lines)} checksums to {output}")


def is_release_asset(name: str) -> bool:
    patterns = [
        "trajectory-v*.tar.gz",
        "trajectory-v*.zip",
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


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


if __name__ == "__main__":
    main()
