#!/bin/bash
# sigmaos-apps-acceptance - SigmaOS apps acceptance test suite
# Modeled on Omancy 4.0.5 test/acceptance.d/apps-test.sh
#
# Verifies that core applications can be launched and closed.

set -euo pipefail

ARTIFACTS="${SIGMAOS_ACCEPTANCE_DIR:-/tmp/sigmaos-acceptance}"

mkdir -p "$ARTIFACTS"

pass() {
  printf 'ok - %s\n' "$1"
}

fail() {
  local description="$1"
  local detail="${2:-}"
  [[ -n $detail ]] && printf '%s\n' "$detail" >&2
  printf 'not ok - %s\n' "$description" >&2
  exit 1
}

window_present() {
  local class="$1"
  if command -v hyprctl >/dev/null 2>&1; then
    hyprctl -j clients 2>/dev/null | jq -e --arg class "$class" '[.[] | select(.class | test($class))] | length > 0' >/dev/null 2>&1
  else
    # Fallback: assume window is present if we can't check
    return 0
  fi
}

close_windows() {
  local class="$1"
  if command -v hyprctl >/dev/null 2>&1; then
    hyprctl -j clients 2>/dev/null | jq -r --arg class "$class" '.[] | select(.class | test($class)) | .address' 2>/dev/null | while read -r addr; do
      [ -n "$addr" ] && hyprctl dispatch killaddress "$addr" 2>/dev/null || true
    done
  fi
}

launch_and_verify() {
  local name="$1" command="$2" class="$3" timeout="${4:-45}"

  if window_present "$class" >/dev/null 2>&1; then
    fail "$name starts with no pre-existing window" "a window matching $class is already open"
  fi

  # Launch the app
  $command &

  # Wait for window to appear
  local deadline=$((SECONDS + timeout))
  while ((SECONDS < deadline)); do
    if window_present "$class" >/dev/null 2>&1; then
      break
    fi
    sleep 1
  done

  if window_present "$class" >/dev/null 2>&1; then
    pass "$name opens a window"
  else
    fail "$name opens a window" "timed out after ${timeout}s waiting for $command"
  fi

  sleep 1

  # Close the app
  close_windows "$class"

  local deadline2=$((SECONDS + 30))
  while window_present "$class" >/dev/null 2>&1; do
    if ((SECONDS >= deadline2)); then
      fail "$name window closes"
    fi
    sleep 2
  done
  pass "$name window closes"
}

# name|command|window class regex|launch timeout
apps='terminal|foot|^foot$|45
browser|chromium --new-window|(?i)chromium|45
editor|xdg-terminal-exec --app-id=org.sigmaos.nvim nvim|org.sigmaos.nvim|45'

status=0
while IFS='|' read -r name command class timeout; do
  if ! (launch_and_verify "$name" "$command" "$class" "${timeout:-45}"); then
    status=1
  fi
done <<<"$apps"

exit $status
