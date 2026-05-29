#!/usr/bin/env python3
"""Regenerate Trajectory brand assets from the canonical logo."""

from __future__ import annotations

import argparse
import shutil
import subprocess
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "assets" / "branding" / "trajectory-logo.png"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--skip-tauri",
        action="store_true",
        help="Only generate repo, desktop web, and Android native assets.",
    )
    return parser.parse_args()


def source_mark() -> Image.Image:
    source = Image.open(SOURCE).convert("RGBA")
    gray = source.convert("L")
    alpha = Image.eval(gray, lambda px: max(0, min(255, 255 - px)))
    alpha = Image.eval(alpha, lambda px: 0 if px < 8 else px)
    mark = Image.new("RGBA", source.size, (0, 0, 0, 0))
    mark.putalpha(alpha)
    bbox = mark.getbbox()
    if bbox is None:
        raise SystemExit(f"source logo has no visible mark: {SOURCE}")
    return mark.crop(bbox)


def render(mark: Image.Image, size: int, occupancy: float, background: tuple[int, int, int, int]) -> Image.Image:
    canvas = Image.new("RGBA", (size, size), background)
    target = max(1, int(size * occupancy))
    scale = min(target / mark.width, target / mark.height)
    resized = mark.resize(
        (max(1, round(mark.width * scale)), max(1, round(mark.height * scale))),
        Image.Resampling.LANCZOS,
    )
    canvas.alpha_composite(resized, ((size - resized.width) // 2, (size - resized.height) // 2))
    return canvas


def generate_static_assets(mark: Image.Image) -> None:
    render(mark, 1024, 0.58, (255, 255, 255, 255)).save(
        ROOT / "assets" / "branding" / "trajectory-mark.png"
    )
    render(mark, 256, 0.58, (255, 255, 255, 255)).save(
        ROOT / "clients" / "desktop" / "public" / "favicon.png"
    )
    render(mark, 512, 0.58, (255, 255, 255, 255)).save(
        ROOT / "clients" / "desktop" / "src" / "assets" / "trajectory-logo.png"
    )

    foreground = Image.new("RGBA", (432, 432), (0, 0, 0, 0))
    resized = mark.resize((286, 286), Image.Resampling.LANCZOS)
    foreground.alpha_composite(resized, ((432 - resized.width) // 2, (432 - resized.height) // 2))
    foreground.save(
        ROOT / "clients" / "android" / "app" / "src" / "main" / "res" / "drawable" / "ic_launcher_foreground.png"
    )

    legacy_sizes = {
        "mipmap-mdpi": 48,
        "mipmap-hdpi": 72,
        "mipmap-xhdpi": 96,
        "mipmap-xxhdpi": 144,
        "mipmap-xxxhdpi": 192,
    }
    res_dir = ROOT / "clients" / "android" / "app" / "src" / "main" / "res"
    for folder, size in legacy_sizes.items():
        out_dir = res_dir / folder
        out_dir.mkdir(parents=True, exist_ok=True)
        icon = render(mark, size, 0.58, (255, 255, 255, 255))
        icon.save(out_dir / "ic_launcher.png")
        icon.save(out_dir / "ic_launcher_round.png")


def generate_tauri_icons() -> None:
    desktop_dir = ROOT / "clients" / "desktop"
    subprocess.run(
        ["npm", "run", "tauri", "--", "icon", "../../assets/branding/trajectory-mark.png"],
        cwd=desktop_dir,
        check=True,
    )
    shutil.rmtree(desktop_dir / "src-tauri" / "icons" / "android", ignore_errors=True)
    shutil.rmtree(desktop_dir / "src-tauri" / "icons" / "ios", ignore_errors=True)


def main() -> None:
    args = parse_args()
    mark = source_mark()
    generate_static_assets(mark)
    if not args.skip_tauri:
        generate_tauri_icons()


if __name__ == "__main__":
    main()
