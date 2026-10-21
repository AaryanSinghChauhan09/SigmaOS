#!/bin/bash
# sigmaos-base-acceptance - SigmaOS base acceptance test suite
# Modeled on Omancy 4.0.5 test/acceptance.d/base-test.sh
#
# Verifies that SigmaOS is installed and configured correctly,
# matching Omancy's acceptance criteria.

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

# ---------------------------------------------------------------------------
# Verify core packages are installed
# ---------------------------------------------------------------------------
verify_core_packages() {
  local package
  local manifest="${SIGMAOS_PATH}/install/packages/sigmaos-base.packages"
  local -a missing=()

  [[ -f $manifest ]] || fail "all SigmaOS core packages are installed" "package manifest not found: $manifest"

  while IFS= read -r package; do
    [[ -z $package || $package == \#* ]] && continue
    pacman -Q "$package" >/dev/null 2>&1 || missing+=("$package")
  done <"$manifest"

  (( ${#missing[@]} == 0 )) || fail "all SigmaOS core packages are installed" "missing packages: ${missing[*]}"
  pass "all SigmaOS core packages are installed (${#missing[@]} missing)"
}

# ---------------------------------------------------------------------------
# Verify defaults
# ---------------------------------------------------------------------------
verify_defaults() {
  [[ $(sigmaos-default-browser) == "chromium" ]] || fail "Chromium is the default browser"
  pass "Chromium is the default browser"

  [[ $(sigmaos-default-terminal) == "foot" ]] || fail "Foot is the default terminal"
  pass "Foot is the default terminal"

  [[ $(sigmaos-default-editor) == "nvim" ]] || fail "Neovim is the default editor"
  pass "Neovim is the default editor"

  [[ $(sigmaos-theme-current) != "Unknown" ]] || fail "a current theme is configured"
  pass "a current theme is configured"

  [[ $(sigmaos-theme-bg-current) != "Unknown" ]] || fail "a current background is configured"
  pass "a current background is configured"

  [[ -n $(sigmaos-font-current) ]] || fail "a monospace font is configured"
  pass "a monospace font is configured"

  [[ $(xdg-mime query default x-scheme-handler/http 2>/dev/null) == "chromium.desktop" ]] || \
    [[ $(sigmaos-default-browser) == "chromium" ]] || fail "HTTP MIME handling uses Chromium"
  pass "HTTP MIME handling uses Chromium"
}

# ---------------------------------------------------------------------------
# Verify shell
# ---------------------------------------------------------------------------
verify_shell() {
  # SigmaOS shell responds to ping
  wait_until "sigmaos-shell responds to ping" 30 sigmaos-shell ping

  # Core shell plugins are loaded
  plugins=$(sigmaos-shell listPlugins)
  for plugin in \
    audio background bar bluetooth clipboard emojis menu \
    monitor network notifications power reminders weather; do
    [[ $plugins == *"$plugin"* ]] || fail "shell plugin is loaded: $plugin" "loaded plugins: $plugins"
    pass "shell plugin is loaded: $plugin"
  done
}

# ---------------------------------------------------------------------------
# Verify audio stack
# ---------------------------------------------------------------------------
verify_audio() {
  wait_until "pipewire is running" 30 wpctl status
  pass "pipewire is running"
}

# ---------------------------------------------------------------------------
# Verify version
# ---------------------------------------------------------------------------
verify_version() {
  sigmaos-version >/dev/null || fail "sigmaos-version works"
  pass "sigmaos-version works"
}

# ---------------------------------------------------------------------------
# Helper: poll a command until it succeeds
# ---------------------------------------------------------------------------
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
# Main
# ---------------------------------------------------------------------------
status=0

verify_core_packages || status=1
verify_defaults || status=1
verify_shell || status=1
verify_audio || status=1
verify_version || status=1

if ((status == 0)); then
  echo "All SigmaOS base acceptance tests passed."
else
  echo "Some SigmaOS base acceptance tests failed." >&2
fi

exit $status
