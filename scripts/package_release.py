#!/usr/bin/env python3
"""Build and package Trajectory release artifacts for one target triple."""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import tarfile
import tempfile
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_COMPONENTS = ("cli", "desktop")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True, help="Rust target triple to build/package")
    parser.add_argument(
        "--output-dir",
        default="dist",
        help="Directory where release artifacts will be written",
    )
    parser.add_argument(
        "--component",
        action="append",
        choices=DEFAULT_COMPONENTS,
        dest="components",
        help="Limit packaging to one or more components",
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Package from existing release binaries instead of building first",
    )
    parser.add_argument(
        "--release-tag",
        help="Optional git tag or label to record in bundled release notes",
    )
    return parser.parse_args()


def cargo_metadata() -> dict:
    command = ["cargo", "metadata", "--format-version", "1", "--no-deps"]
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(result.stdout)


def workspace_version() -> str:
    packages = cargo_metadata()["packages"]
    versions = {
        package["version"]
        for package in packages
        if package["name"] in {"trajectory-core", "trajectory-cli", "trajectory-desktop"}
    }
    if len(versions) != 1:
        raise SystemExit(f"expected one workspace version, found: {sorted(versions)}")
    return versions.pop()


def artifact_manifest(target: str) -> dict[str, dict]:
    binary_ext = ".exe" if "windows" in target else ""
    release_dir = ROOT / "target" / target / "release"
    return {
        "cli": {
            "display": "CLI",
            "binaries": [
                release_dir / f"trajectory-client{binary_ext}",
                release_dir / f"trajectory-server{binary_ext}",
            ],
            "readme": cli_readme(target),
        },
        "desktop": {
            "display": "Desktop",
            "binaries": [release_dir / f"trajectory-desktop{binary_ext}"],
            "readme": desktop_readme(target),
        },
    }


def cli_readme(target: str) -> str:
    return f"""Trajectory CLI
================

Target: {target}

Included binaries:
- trajectory-client
- trajectory-server

Quick start:
1. Run trajectory-client locally and point it at your recursive resolvers.
2. Connect to 127.0.0.1:7000 with SSH, or layer a SOCKS proxy with:
   ssh -N -D 127.0.0.1:1080 -p 7000 root@127.0.0.1
3. Point Firefox at SOCKS5 127.0.0.1:1080 with proxy DNS enabled.

See the repository README for the full tunnel and deployment flow.
"""


def desktop_readme(target: str) -> str:
    return f"""Trajectory Desktop
==================

Target: {target}

Included binary:
- trajectory-desktop

The desktop client wraps the shared pure-Rust tunnel core with:
- resolver and domain configuration
- local listen port configuration
- start/stop controls
- diagnostics log and status cards

The desktop app drives the same transport implementation used by the CLI.
"""


def build_component(target: str, component: str) -> None:
    if component == "cli":
        command = [
            "cargo",
            "build",
            "--release",
            "--target",
            target,
            "-p",
            "trajectory-cli",
            "--bins",
        ]
    elif component == "desktop":
        command = [
            "cargo",
            "build",
            "--release",
            "--target",
            target,
            "-p",
            "trajectory-desktop",
        ]
    else:
        raise SystemExit(f"unknown component: {component}")

    subprocess.run(command, cwd=ROOT, check=True)


def archive_extension(target: str) -> str:
    return ".zip" if "windows" in target else ".tar.gz"


def archive_directory(source_dir: Path, archive_path: Path) -> None:
    if archive_path.exists():
        archive_path.unlink()
    if archive_path.suffix == ".zip":
        with zipfile.ZipFile(archive_path, "w", compression=zipfile.ZIP_DEFLATED) as bundle:
            for path in sorted(source_dir.rglob("*")):
                if path.is_file():
                    bundle.write(path, path.relative_to(source_dir.parent))
    else:
        with tarfile.open(archive_path, "w:gz") as bundle:
            bundle.add(source_dir, arcname=source_dir.name)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_bundle_notes(bundle_dir: Path, version: str, target: str, component: str, tag: str | None) -> None:
    notes = [
        f"Trajectory release bundle",
        f"Version: {version}",
        f"Target: {target}",
        f"Component: {component}",
    ]
    if tag:
        notes.append(f"Release tag: {tag}")
    notes.extend(
        [
            "",
            "Repository layout:",
            "- crates/trajectory-core: shared transport engine",
            "- crates/trajectory-cli: CLI binaries",
            "- clients/desktop: end-user desktop UI",
        ]
    )
    (bundle_dir / "BUILD-INFO.txt").write_text("\n".join(notes) + "\n", encoding="utf-8")


def package_component(
    *,
    version: str,
    target: str,
    component: str,
    config: dict,
    output_dir: Path,
    release_tag: str | None,
) -> tuple[Path, str]:
    bundle_name = f"trajectory-v{version}-{target}-{component}"
    extension = archive_extension(target)
    archive_path = output_dir / f"{bundle_name}{extension}"

    with tempfile.TemporaryDirectory(prefix=f"{bundle_name}-", dir=output_dir) as temp_dir:
        temp_root = Path(temp_dir)
        bundle_dir = temp_root / bundle_name
        bundle_dir.mkdir(parents=True, exist_ok=True)

        for binary in config["binaries"]:
            if not binary.exists():
                raise SystemExit(f"expected built binary not found: {binary}")
            shutil.copy2(binary, bundle_dir / binary.name)

        (bundle_dir / "README.txt").write_text(config["readme"], encoding="utf-8")
        shutil.copy2(ROOT / "README.md", bundle_dir / "README-project.md")
        write_bundle_notes(bundle_dir, version, target, component, release_tag)
        archive_directory(bundle_dir, archive_path)

    digest = sha256(archive_path)
    return archive_path, digest


def main() -> None:
    args = parse_args()
    version = workspace_version()
    target = args.target
    output_dir = (ROOT / args.output_dir).resolve()
    output_dir.mkdir(parents=True, exist_ok=True)
    components = args.components or list(DEFAULT_COMPONENTS)
    manifest = artifact_manifest(target)

    if not args.skip_build:
        for component in components:
            build_component(target, component)

    checksum_lines: list[str] = []
    for component in components:
        archive_path, digest = package_component(
            version=version,
            target=target,
            component=component,
            config=manifest[component],
            output_dir=output_dir,
            release_tag=args.release_tag,
        )
        checksum_lines.append(f"{digest}  {archive_path.name}")

    checksum_path = output_dir / f"trajectory-v{version}-{target}-SHA256SUMS.txt"
    checksum_path.write_text("\n".join(checksum_lines) + "\n", encoding="utf-8")

    print(f"wrote {len(checksum_lines)} artifacts to {output_dir}")
    for line in checksum_lines:
        print(line)
    print(checksum_path.name)


if __name__ == "__main__":
    main()
