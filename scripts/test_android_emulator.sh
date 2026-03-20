#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APK_PATH="${APK_PATH:-$ROOT_DIR/clients/android/app/build/outputs/apk/debug/app-debug.apk}"
PACKAGE_NAME="cc.sevenb.trajectorymobile"
ACTIVITY_NAME="$PACKAGE_NAME/.MainActivity"
CHROME_PACKAGE="com.android.chrome"
AUTOSTART_EXTRA="cc.sevenb.trajectorymobile.extra.AUTOSTART"
ACCESS_KEY_EXTRA="cc.sevenb.trajectorymobile.extra.ACCESS_KEY"
DOMAIN_EXTRA="cc.sevenb.trajectorymobile.extra.DOMAIN"
RESOLVERS_EXTRA="cc.sevenb.trajectorymobile.extra.RESOLVERS"
LISTEN_PORT_EXTRA="cc.sevenb.trajectorymobile.extra.LISTEN_PORT"
KEEP_ALIVE_EXTRA="cc.sevenb.trajectorymobile.extra.KEEP_ALIVE_MS"
CONNECTION_MODE_EXTRA="cc.sevenb.trajectorymobile.extra.CONNECTION_MODE"
TEST_MODE="${TRAJECTORY_TEST_MODE:-both}"
TEST_ACCESS_KEY="${TRAJECTORY_TEST_ACCESS_KEY:-traj1_00000001_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA}"
TEST_DOMAIN="${TRAJECTORY_TEST_DOMAIN:-t.7-b.cc}"
DEFAULT_TEST_RESOLVERS="1.1.1.1:53,1.0.0.1:53,8.8.8.8:53,8.8.4.4:53,9.9.9.9:53"
TEST_RESOLVERS="${TRAJECTORY_TEST_RESOLVERS:-$DEFAULT_TEST_RESOLVERS}"
TEST_LISTEN_PORT="${TRAJECTORY_TEST_LISTEN_PORT:-7000}"
TEST_KEEP_ALIVE_MS="${TRAJECTORY_TEST_KEEP_ALIVE_MS:-50}"
ANDROID_SERIAL="${ANDROID_SERIAL:-}"
HOST_FORWARD_PORT="${TRAJECTORY_TEST_FORWARD_PORT:-17000}"

extract_stat() {
  local line="$1"
  local field="$2"
  sed -n "s/.*${field}=\\([0-9][0-9]*\\).*/\\1/p" <<<"$line"
}

require_device() {
  if ! command -v adb >/dev/null 2>&1; then
    echo "adb is required on PATH" >&2
    exit 1
  fi

  if [[ ! -f "$APK_PATH" ]]; then
    echo "APK not found at $APK_PATH" >&2
    exit 1
  fi

  if [[ -z "$ANDROID_SERIAL" ]]; then
    ANDROID_SERIAL="$(adb devices | awk 'NR>1 && $2=="device" { print $1; exit }')"
  fi

  if [[ -z "$ANDROID_SERIAL" ]]; then
    echo "No online Android device or emulator found" >&2
    exit 1
  fi

  adb_cmd=(adb -s "$ANDROID_SERIAL")
  "${adb_cmd[@]}" wait-for-device >/dev/null
  until [[ "$("${adb_cmd[@]}" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" == "1" ]]; do
    sleep 2
  done
}

start_app() {
  local mode="$1"
  "${adb_cmd[@]}" shell am force-stop "$PACKAGE_NAME" >/dev/null || true
  "${adb_cmd[@]}" shell am force-stop "$CHROME_PACKAGE" >/dev/null || true
  "${adb_cmd[@]}" forward --remove "tcp:$HOST_FORWARD_PORT" >/dev/null 2>&1 || true
  "${adb_cmd[@]}" logcat -c

  "${adb_cmd[@]}" shell am start -n "$ACTIVITY_NAME" \
    --ez "$AUTOSTART_EXTRA" true \
    --es "$ACCESS_KEY_EXTRA" "$TEST_ACCESS_KEY" \
    --es "$DOMAIN_EXTRA" "$TEST_DOMAIN" \
    --es "$RESOLVERS_EXTRA" "$TEST_RESOLVERS" \
    --es "$LISTEN_PORT_EXTRA" "$TEST_LISTEN_PORT" \
    --es "$KEEP_ALIVE_EXTRA" "$TEST_KEEP_ALIVE_MS" \
    --es "$CONNECTION_MODE_EXTRA" "$mode" >/dev/null
  sleep 3
}

wait_for_listener() {
  local port_dump=""
  for _ in $(seq 1 15); do
    port_dump="$("${adb_cmd[@]}" shell toybox netstat -tnl 2>/dev/null | grep ":$TEST_LISTEN_PORT" || true)"
    if grep -q "127.0.0.1:$TEST_LISTEN_PORT" <<<"$port_dump"; then
      return 0
    fi
    sleep 2
  done
  echo "Tunnel is not listening on 127.0.0.1:$TEST_LISTEN_PORT" >&2
  echo "$port_dump" >&2
  exit 1
}

ensure_activity_present() {
  local activity_dump
  activity_dump="$("${adb_cmd[@]}" shell dumpsys activity activities | grep -i "$PACKAGE_NAME" || true)"
  if ! grep -q "$PACKAGE_NAME" <<<"$activity_dump"; then
    echo "Main activity did not resume" >&2
    echo "$activity_dump" >&2
    exit 1
  fi
}

ensure_no_crash() {
  local log_dump="$1"
  if grep -qE 'AndroidRuntime|FATAL EXCEPTION' <<<"$log_dump"; then
    echo "Crash detected in logcat" >&2
    echo "$log_dump" >&2
    exit 1
  fi
}

wait_for_log_pattern() {
  local pattern="$1"
  local filter="$2"
  local attempts="${3:-15}"
  local delay_seconds="${4:-2}"

  for _ in $(seq 1 "$attempts"); do
    local log_dump
    log_dump="$("${adb_cmd[@]}" logcat -d | grep -E "$filter" || true)"
    if grep -q "$pattern" <<<"$log_dump"; then
      printf '%s' "$log_dump"
      return 0
    fi
    ensure_no_crash "$log_dump"
    sleep "$delay_seconds"
  done

  return 1
}

run_vpn_mode() {
  start_app "vpn"

  for _ in $(seq 1 6); do
    log_dump="$("${adb_cmd[@]}" logcat -d | grep -E 'TrajectoryVpnService|TrajectoryViewModel|AndroidRuntime|FATAL EXCEPTION' || true)"
    if grep -q 'VPN active with SOCKS endpoint' <<<"$log_dump"; then
      break
    fi
    "${adb_cmd[@]}" shell input keyevent 20 >/dev/null || true
    "${adb_cmd[@]}" shell input keyevent 22 >/dev/null || true
    "${adb_cmd[@]}" shell input keyevent 66 >/dev/null || true
    sleep 1
  done

  ensure_activity_present
  wait_for_listener

  local log_dump
  log_dump="$("${adb_cmd[@]}" logcat -d | grep -E 'TrajectoryVpnService|TrajectoryViewModel|AndroidRuntime|FATAL EXCEPTION' || true)"
  ensure_no_crash "$log_dump"

  if ! grep -q 'Tunnel controller started successfully in vpn mode' <<<"$log_dump"; then
    echo "VPN mode never reported a successful controller start" >&2
    echo "$log_dump" >&2
    exit 1
  fi

  if ! grep -q 'VPN active with SOCKS endpoint' <<<"$log_dump"; then
    echo "VPN service never reported a successful start" >&2
    echo "$log_dump" >&2
    exit 1
  fi

  "${adb_cmd[@]}" shell am start -a android.intent.action.VIEW -d "https://example.com" "$CHROME_PACKAGE" >/dev/null
  sleep 8

  log_dump="$("${adb_cmd[@]}" logcat -d | grep -E 'TrajectoryVpnService|TrajectoryViewModel|AndroidRuntime|FATAL EXCEPTION' || true)"
  local latest_stats
  latest_stats="$(grep 'TrajectoryVpnService: VPN stats' <<<"$log_dump" | tail -n 1)"
  if [[ -z "$latest_stats" ]]; then
    echo "VPN service never reported traffic statistics" >&2
    echo "$log_dump" >&2
    exit 1
  fi

  local tx_bytes rx_bytes
  tx_bytes="$(extract_stat "$latest_stats" "tx_bytes")"
  rx_bytes="$(extract_stat "$latest_stats" "rx_bytes")"
  if [[ -z "$tx_bytes" || -z "$rx_bytes" || "$tx_bytes" -le 0 || "$rx_bytes" -le 0 ]]; then
    echo "VPN traffic never moved after launching Chrome" >&2
    echo "$latest_stats" >&2
    exit 1
  fi

  "${adb_cmd[@]}" shell input keyevent 3 >/dev/null
  sleep 5

  local service_after_home
  service_after_home="$("${adb_cmd[@]}" shell dumpsys activity services "$PACKAGE_NAME")"
  if ! grep -q 'TrajectoryVpnService' <<<"$service_after_home"; then
    echo "VPN foreground service died after backgrounding the app" >&2
    echo "$service_after_home" >&2
    exit 1
  fi

  if ! grep -q 'isForeground=true' <<<"$service_after_home"; then
    echo "VPN service is not running in foreground mode after backgrounding" >&2
    echo "$service_after_home" >&2
    exit 1
  fi

  echo "Android VPN mode smoke test passed"
}

run_proxy_mode() {
  start_app "proxy"

  ensure_activity_present
  wait_for_listener

  local log_dump
  if ! log_dump="$(wait_for_log_pattern \
    'Tunnel controller started successfully in proxy mode' \
    'TrajectoryProxyService|TrajectoryViewModel|AndroidRuntime|FATAL EXCEPTION' \
    12 \
    1)"; then
    echo "Proxy mode never reported a successful controller start" >&2
    log_dump="$("${adb_cmd[@]}" logcat -d | grep -E 'TrajectoryProxyService|TrajectoryViewModel|AndroidRuntime|FATAL EXCEPTION' || true)"
    echo "$log_dump" >&2
    exit 1
  fi

  "${adb_cmd[@]}" forward "tcp:$HOST_FORWARD_PORT" "tcp:$TEST_LISTEN_PORT" >/dev/null
  curl -I --max-time 20 --socks5-hostname "127.0.0.1:$HOST_FORWARD_PORT" https://example.com >/dev/null

  "${adb_cmd[@]}" shell input keyevent 3 >/dev/null
  sleep 5

  local service_after_home
  service_after_home="$("${adb_cmd[@]}" shell dumpsys activity services "$PACKAGE_NAME")"
  if ! grep -q 'TrajectoryProxyService' <<<"$service_after_home"; then
    echo "Proxy foreground service died after backgrounding the app" >&2
    echo "$service_after_home" >&2
    exit 1
  fi

  if ! grep -q 'isForeground=true' <<<"$service_after_home"; then
    echo "Proxy service is not running in foreground mode after backgrounding" >&2
    echo "$service_after_home" >&2
    exit 1
  fi

  echo "Android proxy mode smoke test passed"
}

require_device
"${adb_cmd[@]}" install -r "$APK_PATH" >/dev/null

case "$TEST_MODE" in
  vpn)
    run_vpn_mode
    ;;
  proxy)
    run_proxy_mode
    ;;
  both)
    run_proxy_mode
    run_vpn_mode
    ;;
  *)
    echo "Unsupported TRAJECTORY_TEST_MODE: $TEST_MODE" >&2
    exit 1
    ;;
esac

"${adb_cmd[@]}" forward --remove "tcp:$HOST_FORWARD_PORT" >/dev/null 2>&1 || true
echo "Android emulator smoke tests passed for mode=$TEST_MODE"
