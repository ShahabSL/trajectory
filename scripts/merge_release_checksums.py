#!/usr/bin/env python3
"""Merge per-target release checksum manifests into one sorted file."""

from __future__ import annotations

import argparse
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
    manifests = sorted(input_dir.glob("trajectory-v*-SHA256SUMS.txt"))
    if not manifests:
        raise SystemExit(f"no checksum manifests found in {input_dir}")

    lines: list[str] = []
    for manifest in manifests:
        for line in manifest.read_text(encoding="utf-8").splitlines():
            stripped = line.strip()
            if stripped:
                lines.append(stripped)

    unique_lines = sorted(set(lines), key=lambda item: item.split(None, 1)[1])
    output.write_text("\n".join(unique_lines) + "\n", encoding="utf-8")
    print(f"merged {len(manifests)} manifests into {output}")


if __name__ == "__main__":
    main()
