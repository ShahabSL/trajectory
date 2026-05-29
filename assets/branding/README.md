# Trajectory Branding

`trajectory-logo.png` is the canonical high-resolution source mark.

Generated app assets are derived from that mark:

- `trajectory-mark.png`: tighter mark on a white canvas for README and small surfaces
- `clients/desktop/public/favicon.png`: desktop web/Tauri favicon
- `clients/desktop/src-tauri/icons/*`: Tauri bundle icons
- `clients/android/app/src/main/res/**/ic_launcher*`: Android launcher icons

When changing the logo, regenerate every derived asset from the same source so the README, desktop client, Android launcher, and packaged apps remain visually consistent.

```bash
python3 scripts/generate_brand_assets.py
```

The script requires Pillow. The Tauri icon step also requires the desktop npm dependencies.
