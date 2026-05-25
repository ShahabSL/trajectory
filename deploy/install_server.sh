#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INSTALL_DIR="/opt/trajectory"
ENV_DIR="/etc/trajectory"
SERVER_ENV_PATH="$ENV_DIR/server.env"
SERVER_BIN="${SERVER_BIN:-$ROOT_DIR/target/release/trajectory-server}"
ADMIN_BIN="${ADMIN_BIN:-$ROOT_DIR/target/release/trajectory-admin}"
HEV_BIN="${HEV_BIN:-}"
TARGET_ADDRESS="socks5-direct"
BIND_HOST="0.0.0.0"
DNS_PORT="53"
DOMAIN=""
CREATE_CLIENT_LABEL="phone"
DISABLE_SYSTEMD_RESOLVED="false"
START_SERVICES="true"

usage() {
  cat <<'EOF'
Usage: sudo deploy/install_server.sh --domain <DOMAIN> [options]

Required:
  --domain <DOMAIN>                 Authoritative domain served by this host

Optional:
  --server-bin <PATH>               trajectory-server binary
  --admin-bin <PATH>                trajectory-admin binary
  --hev-binary <PATH>               Optional hev-socks5-server binary to install locally
  --target-address <HOST:PORT|socks5-direct>
                                     Server egress target. Use socks5-direct for built-in
                                     SOCKS5 egress, or HOST:PORT for raw TCP upstream
                                     (default: socks5-direct)
  --bind-host <HOST>                Bind host for DNS service (default: 0.0.0.0)
  --dns-port <PORT>                 DNS listen port (default: 53)
  --install-dir <PATH>              Install directory (default: /opt/trajectory)
  --env-dir <PATH>                  Environment directory (default: /etc/trajectory)
  --client-label <LABEL>            Create and print an initial client key with this label
  --no-client                       Do not create an initial client key
  --disable-systemd-resolved        Stop/disable systemd-resolved, back up resolv.conf,
                                    and write a public resolv.conf
  --no-start                        Install files without enabling/starting services
  -h, --help                        Show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --domain)
      DOMAIN="${2:?missing domain}"
      shift 2
      ;;
    --server-bin)
      SERVER_BIN="${2:?missing server binary path}"
      shift 2
      ;;
    --admin-bin)
      ADMIN_BIN="${2:?missing admin binary path}"
      shift 2
      ;;
    --hev-binary)
      HEV_BIN="${2:?missing hev binary path}"
      shift 2
      ;;
    --target-address)
      TARGET_ADDRESS="${2:?missing target address}"
      shift 2
      ;;
    --bind-host)
      BIND_HOST="${2:?missing bind host}"
      shift 2
      ;;
    --dns-port)
      DNS_PORT="${2:?missing DNS port}"
      shift 2
      ;;
    --install-dir)
      INSTALL_DIR="${2:?missing install dir}"
      shift 2
      ;;
    --env-dir)
      ENV_DIR="${2:?missing env dir}"
      shift 2
      ;;
    --client-label)
      CREATE_CLIENT_LABEL="${2:?missing client label}"
      shift 2
      ;;
    --no-client)
      CREATE_CLIENT_LABEL=""
      shift
      ;;
    --disable-systemd-resolved)
      DISABLE_SYSTEMD_RESOLVED="true"
      shift
      ;;
    --no-start)
      START_SERVICES="false"
      shift
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
  echo "install_server.sh must run as root" >&2
  exit 1
fi

if [[ -z "$DOMAIN" ]]; then
  echo "--domain is required" >&2
  usage >&2
  exit 1
fi

if [[ ! -x "$SERVER_BIN" ]]; then
  echo "trajectory-server binary not found or not executable: $SERVER_BIN" >&2
  exit 1
fi

if [[ ! -x "$ADMIN_BIN" ]]; then
  echo "trajectory-admin binary not found or not executable: $ADMIN_BIN" >&2
  exit 1
fi

SERVER_ENV_PATH="$ENV_DIR/server.env"
CLIENT_DB_PATH="$INSTALL_DIR/trajectory-clients.json"

install -d -m 755 "$INSTALL_DIR" "$ENV_DIR"
install -m 755 "$SERVER_BIN" "$INSTALL_DIR/trajectory-server"
install -m 755 "$ADMIN_BIN" "$INSTALL_DIR/trajectory-admin"

cat >/etc/systemd/system/trajectory.service <<EOF
[Unit]
Description=Trajectory high-throughput DNS tunnel server
After=network-online.target
Wants=network-online.target
Conflicts=systemd-resolved.service

[Service]
Type=simple
EnvironmentFile=$SERVER_ENV_PATH
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/trajectory-server --bind \${TRAJECTORY_BIND_HOST} --dns-listen-port \${TRAJECTORY_DNS_LISTEN_PORT} --target-address \${TRAJECTORY_TARGET_ADDRESS} --domain \${TRAJECTORY_DOMAIN} --client-db \${TRAJECTORY_CLIENT_DB}
Restart=always
RestartSec=2

[Install]
WantedBy=multi-user.target
EOF

cat >"$SERVER_ENV_PATH" <<EOF
TRAJECTORY_BIND_HOST=$BIND_HOST
TRAJECTORY_DNS_LISTEN_PORT=$DNS_PORT
TRAJECTORY_TARGET_ADDRESS=$TARGET_ADDRESS
TRAJECTORY_DOMAIN=$DOMAIN
TRAJECTORY_CLIENT_DB=$CLIENT_DB_PATH
EOF
chmod 640 "$SERVER_ENV_PATH"

if [[ -n "$HEV_BIN" ]]; then
  if [[ ! -x "$HEV_BIN" ]]; then
    echo "hev-socks5-server binary not found or not executable: $HEV_BIN" >&2
    exit 1
  fi
  install -m 755 "$HEV_BIN" "$INSTALL_DIR/hev-socks5-server"
  install -m 644 "$ROOT_DIR/deploy/hev-socks5-server.yml" "$INSTALL_DIR/hev-socks5-server.yml"
  cat >/etc/systemd/system/trajectory-socks.service <<EOF
[Unit]
Description=Trajectory local SOCKS5 upstream
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/hev-socks5-server $INSTALL_DIR/hev-socks5-server.yml
Restart=always
RestartSec=2

[Install]
WantedBy=multi-user.target
EOF
fi

if [[ "$CREATE_CLIENT_LABEL" != "" ]]; then
  ACCESS_KEY="$(umask 077; "$INSTALL_DIR/trajectory-admin" create-client \
    --client-db "$CLIENT_DB_PATH" \
    --label "$CREATE_CLIENT_LABEL" \
    --format key)"
  chmod 600 "$CLIENT_DB_PATH"
fi

if [[ "$DISABLE_SYSTEMD_RESOLVED" == "true" ]] && systemctl list-unit-files systemd-resolved.service >/dev/null 2>&1; then
  systemctl disable --now systemd-resolved.service || true
  if [[ -e /etc/resolv.conf || -L /etc/resolv.conf ]]; then
    cp -a /etc/resolv.conf "/etc/resolv.conf.trajectory.bak.$(date +%s)" || true
  fi
  rm -f /etc/resolv.conf
  cat >/etc/resolv.conf <<'EOF'
nameserver 1.1.1.1
nameserver 8.8.8.8
nameserver 9.9.9.9
options edns0
EOF
fi

systemctl daemon-reload

if [[ "$START_SERVICES" == "true" ]]; then
  if [[ -n "$HEV_BIN" ]]; then
    systemctl enable --now trajectory-socks.service
  fi
  systemctl enable --now trajectory.service
fi

echo "Installed Trajectory server to $INSTALL_DIR"
echo "Server configuration: $SERVER_ENV_PATH"
echo "Client registry: $CLIENT_DB_PATH"

if [[ "${ACCESS_KEY:-}" != "" ]]; then
  echo
  echo "Initial client access key:"
  echo "$ACCESS_KEY"
  echo
  echo "Use this key with domain: $DOMAIN"
  echo
  echo "Client quickstart:"
  cat <<EOF
read -rsp 'Trajectory access key: ' TRAJECTORY_ACCESS_KEY; echo
export TRAJECTORY_ACCESS_KEY
trajectory-client \\
  --listen 127.0.0.1:7000 \\
  --http-listen 127.0.0.1:7001 \\
  --domain $DOMAIN \\
  --resolver 1.1.1.1:53 \\
  --resolver 8.8.8.8:53

curl -I --max-time 20 --socks5-hostname 127.0.0.1:7000 https://example.com
curl -I --max-time 20 --proxy http://127.0.0.1:7001 https://example.com
EOF
fi
