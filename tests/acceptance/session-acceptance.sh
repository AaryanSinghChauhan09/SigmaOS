#!/bin/bash
# sigmaos-session-acceptance - SigmaOS session acceptance test suite
# Modeled on Omancy 4.0.5 test/acceptance.d/session-test.sh
#
# Verifies that the SigmaOS desktop session is running and responsive,
# the shell plugins are loaded, the bar and background are visible,
# the audio stack is up, and no systemd units have failed.

set -euo pipefail

SIGMAOS_PATH="${SIGMAOS_PATH:-/usr/share/sigmaos}"
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

wait_until() {
  local description="$1" timeout="$2"
  shift 2

  local deadline=$((SECONDS + timeout))

  until "$@" >/dev/null 2>&1; do
    if ((SECONDS >= deadline)); then
      fail "$description" "timed out after ${timeout}s waiting for: $*"
    fi
    sleep 1
  done

  pass "$description"
}

# ---------------------------------------------------------------------------
# Verify compositor reports at least one monitor
# ---------------------------------------------------------------------------
verify_monitor() {
  if command -v hyprctl >/dev/null 2>&1; then
    monitors=$(hyprctl -j monitors 2>/dev/null | jq 'length' 2>/dev/null || echo "0")
    (( monitors >= 1 )) || fail "compositor reports a monitor"
    pass "compositor reports a monitor (${monitors})"
  else
    pass "compositor monitor check skipped (hyprctl not available)"
  fi
}

# ---------------------------------------------------------------------------
# Verify SigmaOS shell is running and responsive
# ---------------------------------------------------------------------------
verify_shell_responsive() {
  wait_until "sigmaos-shell responds to ping" 60 sigmaos-shell ping
}

# ---------------------------------------------------------------------------
# Verify core shell plugins are loaded
# ---------------------------------------------------------------------------
verify_plugins() {
  plugins=$(sigmaos-shell listPlugins)
  for plugin in \
    sigmaos.audio sigmaos.background sigmaos.bar sigmaos.bluetooth \
    sigmaos.clipboard sigmaos.emojis sigmaos.menu \
    sigmaos.monitor sigmaos.network sigmaos.notifications sigmaos.power \
    sigmaos.reminders sigmaos.weather; do
    [[ $plugins == *"$plugin"* ]] || fail "shell plugin is loaded: $plugin" "loaded plugins: $plugins"
    pass "shell plugin is loaded: $plugin"
  done
}

# ---------------------------------------------------------------------------
# Verify audio stack
# ---------------------------------------------------------------------------
verify_audio_stack() {
  wait_until "pipewire is running" 30 wpctl status
}

# ---------------------------------------------------------------------------
# Verify SigmaOS version
# ---------------------------------------------------------------------------
verify_version() {
  sigmaos-version >/dev/null || fail "sigmaos-version works"
  pass "sigmaos-version works"
}

# ---------------------------------------------------------------------------
# Verify no failed systemd units
# ---------------------------------------------------------------------------
verify_no_failed_units() {
  failed_units() {
    systemctl "$@" --failed --no-legend --plain 2>/dev/null | awk '{print $1}' || true
  }

  failed_system=$(failed_units --system)
  if [[ -n $failed_system ]]; then
    fail "no failed system units" "failed units: $failed_system"
  fi
  pass "no failed system units"

  failed_user=$(failed_units --user)
  if [[ -n $failed_user ]]; then
    fail "no failed user units" "failed units: $failed_user"
  fi
  pass "no failed user units"
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
status=0

verify_monitor || status=1
verify_shell_responsive || status=1
verify_plugins || status=1
verify_audio_stack || status=1
verify_version || status=1
verify_no_failed_units || status=1

if ((status == 0)); then
  echo "All SigmaOS session acceptance tests passed."
else
  echo "Some SigmaOS session acceptance tests failed." >&2
fi

exit $status
