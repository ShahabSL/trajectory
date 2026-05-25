#!/usr/bin/env bash
set -euo pipefail

REPO_URL="${REPO_URL:-https://github.com/ShahabSL/trajectory.git}"
REPO_REF="${REPO_REF:-main}"
TARGET_ADDRESS="socks5-direct"
DOMAIN=""
CLIENT_LABEL="phone"
DISABLE_SYSTEMD_RESOLVED="false"
INSTALL_HEV="false"
RUN_USER="${SUDO_USER:-root}"

usage() {
  cat <<'EOF'
Usage: curl -fsSL https://raw.githubusercontent.com/ShahabSL/trajectory/main/deploy/bootstrap_server.sh | sudo bash -s -- --domain <DOMAIN> [options]

Required:
  --domain <DOMAIN>                 Authoritative domain served by this host

Optional:
  --target-address <HOST:PORT|socks5-direct>
                                    Server egress target (default: socks5-direct)
  --client-label <LABEL>            Initial client label (default: phone)
  --disable-systemd-resolved        Stop/disable systemd-resolved and free port 53
  --install-hev                     Reserved; not implemented by this bootstrap script
  --repo-url <URL>                  Git URL to clone (default: ShahabSL/trajectory)
  --repo-ref <REF>                  Git branch or tag to install (default: main)
  -h, --help                        Show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --domain)
      DOMAIN="${2:?missing domain}"
      shift 2
      ;;
    --target-address)
      TARGET_ADDRESS="${2:?missing target address}"
      shift 2
      ;;
    --client-label)
      CLIENT_LABEL="${2:?missing client label}"
      shift 2
      ;;
    --disable-systemd-resolved)
      DISABLE_SYSTEMD_RESOLVED="true"
      shift
      ;;
    --install-hev)
      INSTALL_HEV="true"
      shift
      ;;
    --repo-url)
      REPO_URL="${2:?missing repo url}"
      shift 2
      ;;
    --repo-ref)
      REPO_REF="${2:?missing repo ref}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ $EUID -ne 0 ]]; then
  echo "bootstrap_server.sh must run as root" >&2
  exit 1
fi

if [[ -z "$DOMAIN" ]]; then
  echo "--domain is required" >&2
  usage >&2
  exit 1
fi

install_packages() {
  if command -v apt-get >/dev/null 2>&1; then
    export DEBIAN_FRONTEND=noninteractive
    apt-get update
    apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      clang \
      cmake \
      curl \
      git \
      libssl-dev \
      pkg-config
    return
  fi

  if command -v dnf >/dev/null 2>&1; then
    dnf install -y \
      ca-certificates \
      clang \
      cmake \
      curl \
      gcc \
      gcc-c++ \
      git \
      openssl-devel \
      pkgconf-pkg-config
    return
  fi

  if command -v yum >/dev/null 2>&1; then
    yum install -y \
      ca-certificates \
      clang \
      cmake \
      curl \
      gcc \
      gcc-c++ \
      git \
      openssl-devel \
      pkgconfig
    return
  fi

  echo "Unsupported package manager. Install curl, git, clang, cmake, pkg-config, OpenSSL headers, and a C/C++ toolchain manually." >&2
  exit 1
}

ensure_rust() {
  if run_as_builder 'command -v cargo >/dev/null 2>&1 && command -v rustup >/dev/null 2>&1'; then
    return
  fi

  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | \
    run_as_builder 'sh -s -- -y --profile minimal'
}

run_as_builder() {
  local command="$1"
  if [[ "$RUN_USER" == "root" ]]; then
    bash -lc "$command"
  else
    sudo -u "$RUN_USER" bash -lc "$command"
  fi
}

WORK_DIR="$(mktemp -d /tmp/trajectory-bootstrap.XXXXXX)"
cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

install_packages
ensure_rust

chown -R "$RUN_USER":"$RUN_USER" "$WORK_DIR"

run_as_builder "git clone --depth 1 --branch '$REPO_REF' '$REPO_URL' '$WORK_DIR/repo'"
run_as_builder "cd '$WORK_DIR/repo' && source \"\$HOME/.cargo/env\" && cargo build --release -p trajectory-cli --bin trajectory-server --bin trajectory-admin"

cd "$WORK_DIR/repo"

INSTALL_ARGS=(
  --domain "$DOMAIN"
  --server-bin "$WORK_DIR/repo/target/release/trajectory-server"
  --admin-bin "$WORK_DIR/repo/target/release/trajectory-admin"
  --target-address "$TARGET_ADDRESS"
  --client-label "$CLIENT_LABEL"
)

if [[ "$DISABLE_SYSTEMD_RESOLVED" == "true" ]]; then
  INSTALL_ARGS+=(--disable-systemd-resolved)
fi

if [[ "$INSTALL_HEV" == "true" ]]; then
  echo "--install-hev currently expects you to provide or install hev-socks5-server separately." >&2
  exit 1
fi

"$WORK_DIR/repo/deploy/install_server.sh" "${INSTALL_ARGS[@]}"
