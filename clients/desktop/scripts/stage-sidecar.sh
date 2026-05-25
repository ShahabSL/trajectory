#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
desktop_dir="$(cd "$script_dir/.." && pwd)"
repo_root="$(cd "$desktop_dir/../.." && pwd)"
target_triple="$(rustc -vV | awk '/host:/ { print $2 }')"

if [[ -z "$target_triple" ]]; then
  echo "could not determine Rust host target triple" >&2
  exit 1
fi

binary_name="trajectory-client"
if [[ "$target_triple" == *windows* ]]; then
  binary_name="trajectory-client.exe"
fi

if [[ -n "${TRAJECTORY_CLIENT_BIN:-}" ]]; then
  source_path="$TRAJECTORY_CLIENT_BIN"
else
  cargo build --release -p trajectory-cli --bin trajectory-client --manifest-path "$repo_root/Cargo.toml"
  source_path="$repo_root/target/release/$binary_name"
fi

if [[ ! -f "$source_path" ]]; then
  echo "trajectory-client binary not found at $source_path" >&2
  exit 1
fi

mkdir -p "$desktop_dir/src-tauri/bin"
dest="$desktop_dir/src-tauri/bin/trajectory-client-$target_triple"
if [[ "$target_triple" == *windows* ]]; then
  dest="$dest.exe"
fi

cp "$source_path" "$dest"
chmod +x "$dest" 2>/dev/null || true
echo "staged $dest"
