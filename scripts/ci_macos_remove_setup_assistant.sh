#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  exit 0
fi

for _ in {1..10}; do
  if ! pgrep -x "Setup Assistant" >/dev/null; then
    echo "Setup Assistant is not running"
    exit 0
  fi

  echo "Setup Assistant is running; trying to dismiss it"
  osascript \
    -e 'tell application "System Events"' \
    -e 'if exists process "Setup Assistant" then' \
    -e 'tell process "Setup Assistant"' \
    -e 'set frontmost to true' \
    -e 'delay 0.2' \
    -e 'try' \
    -e 'perform action "AXPress" of button "Continue" of window 1' \
    -e 'end try' \
    -e 'delay 0.5' \
    -e 'end tell' \
    -e 'key code 36' \
    -e 'end if' \
    -e 'end tell' >/dev/null 2>&1 || true
  sleep 2
done

echo "Setup Assistant did not go away; killing it"
pkill -9 -x "Setup Assistant" || true
sleep 2

if pgrep -x "Setup Assistant" >/dev/null; then
  echo "::error::Setup Assistant is still running"
  exit 1
fi
