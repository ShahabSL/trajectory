#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${TRAJECTORY_E2E_DOMAIN:-}" ]]; then
  echo "TRAJECTORY_E2E_DOMAIN is required" >&2
  exit 2
fi

if [[ -z "${TRAJECTORY_E2E_ACCESS_KEY:-}" ]]; then
  echo "TRAJECTORY_E2E_ACCESS_KEY is required" >&2
  exit 2
fi

if [[ -z "${TRAJECTORY_E2E_RESOLVERS:-}" ]]; then
  echo "TRAJECTORY_E2E_RESOLVERS is required" >&2
  exit 2
fi

FETCH_URL="${TRAJECTORY_E2E_FETCH_URL:-https://www.wikipedia.org/}"
LISTEN_HOST="${TRAJECTORY_E2E_LISTEN_HOST:-127.0.0.1}"
LISTEN_PORT="${TRAJECTORY_E2E_LISTEN_PORT:-7000}"
HTTP_LISTEN_PORT="${TRAJECTORY_E2E_HTTP_LISTEN_PORT:-7001}"
CONNECT_TIMEOUT="${TRAJECTORY_E2E_CONNECT_TIMEOUT:-20}"
MAX_TIME="${TRAJECTORY_E2E_MAX_TIME:-90}"
if [[ -n "${TRAJECTORY_E2E_RESOLVER_SOCKS_PROXY:-}" ]]; then
  STARTUP_TIMEOUT="${TRAJECTORY_E2E_STARTUP_TIMEOUT:-180}"
else
  STARTUP_TIMEOUT="${TRAJECTORY_E2E_STARTUP_TIMEOUT:-30}"
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  if [[ -n "${client_pid:-}" ]] && kill -0 "$client_pid" >/dev/null 2>&1; then
    kill "$client_pid" >/dev/null 2>&1 || true
    wait "$client_pid" >/dev/null 2>&1 || true
  fi
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

umask 077
resolver_file="$tmp_dir/resolvers.txt"
client_log="$tmp_dir/client.log"
admission_report="$tmp_dir/admission.jsonl"
body_path="$tmp_dir/body.out"

printf '%s\n' "$TRAJECTORY_E2E_RESOLVERS" \
  | tr ', ' '\n' \
  | sed '/^[[:space:]]*$/d' \
  >"$resolver_file"

if [[ ! -s "$resolver_file" ]]; then
  echo "TRAJECTORY_E2E_RESOLVERS did not contain any resolvers" >&2
  exit 2
fi

cargo build --release -p trajectory-cli --bin trajectory-client

client_args=(
  "target/release/trajectory-client"
  "--listen" "${LISTEN_HOST}:${LISTEN_PORT}"
  "--http-listen" "${LISTEN_HOST}:${HTTP_LISTEN_PORT}"
  "--domain" "$TRAJECTORY_E2E_DOMAIN"
  "--resolver-file" "$resolver_file"
  "--admission-report" "$admission_report"
)

if [[ -n "${TRAJECTORY_E2E_RESOLVER_SOCKS_PROXY:-}" ]]; then
  client_args+=("--resolver-socks-proxy" "$TRAJECTORY_E2E_RESOLVER_SOCKS_PROXY")
fi

if [[ -n "${TRAJECTORY_E2E_DNS_MAX_PAYLOAD:-}" ]]; then
  client_args+=("--dns-max-payload" "$TRAJECTORY_E2E_DNS_MAX_PAYLOAD")
fi

TRAJECTORY_ACCESS_KEY="$TRAJECTORY_E2E_ACCESS_KEY" \
  "${client_args[@]}" >"$client_log" 2>&1 &
client_pid=$!

ready_deadline=$((SECONDS + STARTUP_TIMEOUT))
while (( SECONDS < ready_deadline )); do
  if ! kill -0 "$client_pid" >/dev/null 2>&1; then
    echo "trajectory-client exited before it became ready" >&2
    exit 1
  fi
  if grep -q "trajectory client listening" "$client_log"; then
    break
  fi
  sleep 0.1
done

if ! grep -q "trajectory client listening" "$client_log"; then
  echo "trajectory-client did not become ready within ${STARTUP_TIMEOUT}s" >&2
  exit 1
fi

if [[ -n "${TRAJECTORY_E2E_RESOLVER_SOCKS_PROXY:-}" ]]; then
  python3 - "$admission_report" <<'PY'
import json
import sys

path = sys.argv[1]
summary = None
with open(path, "r", encoding="utf-8") as handle:
    for line in handle:
        event = json.loads(line)
        if event.get("event") == "admission_summary":
            summary = event
            break

if not summary:
    raise SystemExit("missing admission summary for SOCKS-gated live e2e")
if not summary.get("tcp_path"):
    raise SystemExit("live e2e did not exercise SOCKS-gated DNS-over-TCP")
if int(summary.get("selected_count") or 0) < 1:
    raise SystemExit("no resolvers passed SOCKS-gated admission")
PY
fi

curl_output="$(
  curl -L \
    --socks5-hostname "${LISTEN_HOST}:${LISTEN_PORT}" \
    --connect-timeout "$CONNECT_TIMEOUT" \
    --max-time "$MAX_TIME" \
    --output "$body_path" \
    --write-out 'code=%{http_code} time=%{time_total} size=%{size_download}' \
    "$FETCH_URL"
)"

case "$curl_output" in
  code=2*|code=3*) ;;
  *)
    echo "live e2e fetch failed: $curl_output" >&2
    exit 1
    ;;
esac

if [[ ! -s "$body_path" ]]; then
  echo "live e2e fetch returned an empty body: $curl_output" >&2
  exit 1
fi

echo "live e2e passed: $curl_output"

http_proxy_output="$(
  curl -L \
    --proxy "http://${LISTEN_HOST}:${HTTP_LISTEN_PORT}" \
    --connect-timeout "$CONNECT_TIMEOUT" \
    --max-time "$MAX_TIME" \
    --output "$body_path.http" \
    --write-out 'code=%{http_code} time=%{time_total} size=%{size_download}' \
    "$FETCH_URL"
)"

case "$http_proxy_output" in
  code=2*|code=3*) ;;
  *)
    echo "live HTTP proxy e2e fetch failed: $http_proxy_output" >&2
    exit 1
    ;;
esac

if [[ ! -s "$body_path.http" ]]; then
  echo "live HTTP proxy e2e fetch returned an empty body: $http_proxy_output" >&2
  exit 1
fi

echo "live HTTP proxy e2e passed: $http_proxy_output"
