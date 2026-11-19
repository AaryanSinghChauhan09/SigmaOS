#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# sigmaos_doctor.sh — Comprehensive system health checker for SigmaOS
# Checks display server, GPU, fonts, Python/GTK, Rust, SigmaOS binaries, and more.
# ---------------------------------------------------------------------------

set -uo pipefail

SIGMA_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# ── ANSI colours ────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

PASS="${GREEN}✔ PASS${RESET}"
WARN="${YELLOW}⚠ WARN${RESET}"
FAIL="${RED}✘ FAIL${RESET}"

pass_count=0
warn_count=0
fail_count=0

# ── Helpers ─────────────────────────────────────────────────────────────────
check() {
  local label="$1"
  local status="$2"   # pass | warn | fail
  local detail="${3:-}"

  local badge
  case "$status" in
    pass) badge="$PASS"; (( pass_count++ )) || true ;;
    warn) badge="$WARN"; (( warn_count++ )) || true ;;
    fail) badge="$FAIL"; (( fail_count++ )) || true ;;
  esac

  if [[ -n "$detail" ]]; then
    printf "  %-38s %b  %s\n" "$label" "$badge" "$detail"
  else
    printf "  %-38s %b\n" "$label" "$badge"
  fi
}

section() {
  echo ""
  echo -e "${CYAN}${BOLD}── $* ${RESET}"
}

cmd_exists() { command -v "$1" &>/dev/null; }

# ── Header ───────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}╔══════════════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}║             SigmaOS System Doctor                ║${RESET}"
echo -e "${BOLD}╚══════════════════════════════════════════════════╝${RESET}"
echo ""
echo -e "  Root  : ${SIGMA_ROOT}"
echo -e "  Host  : $(uname -r) / $(uname -m)"
echo -e "  Date  : $(date '+%Y-%m-%d %H:%M:%S')"

# ── 1. Display Server ────────────────────────────────────────────────────────
section "Display Server"

if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
  check "Wayland session" pass "${WAYLAND_DISPLAY}"
  check "X11 session" warn "not active (Wayland running)"
elif [[ -n "${DISPLAY:-}" ]]; then
  check "X11 session" pass "${DISPLAY}"
  check "Wayland session" warn "not active (X11 running)"
else
  check "Display server" fail "neither DISPLAY nor WAYLAND_DISPLAY set"
fi

if cmd_exists Xwayland; then
  check "XWayland bridge" pass "$(Xwayland --version 2>&1 | head -1)"
else
  check "XWayland bridge" warn "not installed"
fi

# ── 2. GPU Drivers ───────────────────────────────────────────────────────────
section "GPU Drivers"

if cmd_exists glxinfo; then
  renderer="$(glxinfo 2>/dev/null | grep 'OpenGL renderer' | sed 's/.*: //' | head -1)"
  if [[ -n "$renderer" ]]; then
    check "OpenGL renderer" pass "$renderer"
  else
    check "OpenGL renderer" warn "glxinfo found but no renderer reported"
  fi
else
  check "glxinfo (OpenGL)" warn "not installed (mesa-utils?)"
fi

if lsmod 2>/dev/null | grep -qE '^(nvidia|amdgpu|i915|radeon) '; then
  gpu_mod="$(lsmod 2>/dev/null | grep -Eo '^(nvidia|amdgpu|i915|radeon)' | head -1)"
  check "GPU kernel module" pass "$gpu_mod"
else
  check "GPU kernel module" warn "no discrete GPU module (nvidia/amdgpu/i915/radeon)"
fi

if cmd_exists vulkaninfo; then
  check "Vulkan" pass "vulkaninfo available"
else
  check "Vulkan" warn "vulkaninfo not found"
fi

# ── 3. Fonts ─────────────────────────────────────────────────────────────────
section "Fonts"

check_font() {
  local name="$1"; local pattern="$2"
  if (fc-list 2>/dev/null || true) | grep -iE "$pattern" >/dev/null; then
    check "Font: $name" pass
  else
    check "Font: $name" warn "not installed"
  fi
}

check_font "JetBrains Mono" "JetBrainsMono|JetBrains Mono"
check_font "Nerd Fonts (any)" "NerdFont|Nerd Font"
check_font "Noto Sans" "NotoSans|Noto Sans"
check_font "Ubuntu" "Ubuntu"
check_font "FiraCode" "FiraCode|Fira Code"

# ── 4. Python3 & GTK / WebKit2 ───────────────────────────────────────────────
section "Python3 & GTK / WebKit2"

if cmd_exists python3; then
  pyver="$(python3 --version 2>&1)"
  check "Python3" pass "$pyver"
else
  check "Python3" fail "python3 not found"
fi

check_py_module() {
  local label="$1"; local module="$2"
  if python3 -c "import $module" 2>/dev/null; then
    check "$label" pass
  else
    check "$label" fail "module '$module' not importable"
  fi
}

check_py_module "gi (PyGObject)" "gi"

# GTK3
if python3 -c "import gi; gi.require_version('Gtk','3.0'); from gi.repository import Gtk" 2>/dev/null; then
  check "GTK3 bindings" pass
else
  check "GTK3 bindings" fail "gir1.2-gtk-3.0 / python3-gi missing?"
fi

# WebKit2 4.1 (preferred), fall back to 4.0
if python3 -c "import gi; gi.require_version('WebKit2','4.1'); from gi.repository import WebKit2" 2>/dev/null; then
  check "WebKit2 4.1 bindings" pass
elif python3 -c "import gi; gi.require_version('WebKit2','4.0'); from gi.repository import WebKit2" 2>/dev/null; then
  check "WebKit2 4.0 bindings" warn "4.1 not available, 4.0 found"
else
  check "WebKit2 bindings" fail "gir1.2-webkit2-4.1 / webkit2gtk missing?"
fi

# ── 5. SigmaOS Binaries ───────────────────────────────────────────────────────
section "SigmaOS Binaries"

for bin in sigmaos sigmaos-shell sigmaos-bar sigmaos-settings sigmaos-version; do
  bin_path="$SIGMA_ROOT/bin/$bin"
  if [[ -x "$bin_path" ]]; then
    check "bin/$bin" pass
  elif [[ -f "$bin_path" ]]; then
    check "bin/$bin" warn "exists but not executable"
  elif cmd_exists "$bin"; then
    check "$bin (PATH)" pass "$(command -v "$bin")"
  else
    check "bin/$bin" fail "not found"
  fi
done

# ── 6. SigmaOS Shell & Desktop Tools ─────────────────────────────────────────
section "Desktop & Shell Tools"

if [[ -x "$SIGMA_ROOT/bin/sigmaos-shell" ]] || cmd_exists sigmaos-shell; then
  check "sigmaos-shell" pass
else
  check "sigmaos-shell" warn "not installed (optional)"
fi

for tool in hyprland waybar rofi wallust kitty; do
  if cmd_exists "$tool"; then
    check "$tool" pass "$(command -v "$tool")"
  else
    check "$tool" warn "not installed (Wayland-specific, optional)"
  fi
done

# ── 7. Rust toolchain ─────────────────────────────────────────────────────────
section "Rust Toolchain"

if cmd_exists rustc; then
  rustver="$(rustc --version 2>&1)"
  check "rustc" pass "$rustver"
else
  check "rustc" fail "not found (curl https://sh.rustup.rs | sh)"
fi

if cmd_exists cargo; then
  cargover="$(cargo --version 2>&1)"
  check "cargo" pass "$cargover"
else
  check "cargo" fail "not found"
fi

if cmd_exists rust-analyzer; then
  check "rust-analyzer (LSP)" pass
else
  check "rust-analyzer (LSP)" warn "not installed (optional)"
fi

# ── 8. SigmaOS UI Files ────────────────────────────────────────────────────────
section "SigmaOS UI Files"

check_file() {
  local label="$1"; local path="$2"
  if [[ -f "$path" ]]; then
    check "$label" pass
  else
    check "$label" fail "missing: $path"
  fi
}

check_file "ui/desktop/index.html"      "$SIGMA_ROOT/ui/desktop/index.html"
check_file "scripts/sigmaos_gui.py"     "$SIGMA_ROOT/scripts/sigmaos_gui.py"
check_file "scripts/sigmaos_native_app.py" "$SIGMA_ROOT/scripts/sigmaos_native_app.py"
check_file "scripts/sigmaos_startup_diagnostics.py" "$SIGMA_ROOT/scripts/sigmaos_startup_diagnostics.py"
check_file "run.sh"                     "$SIGMA_ROOT/run.sh"
check_file "Makefile"                   "$SIGMA_ROOT/Makefile"

# Config omarchy structure
if [[ -d "$SIGMA_ROOT/config/omarchy" ]]; then
  check "config/omarchy/ directory"    pass
else
  check "config/omarchy/ directory"   fail "missing"
fi

for cf in shell.json theme.conf keybindings.conf; do
  if [[ -f "$SIGMA_ROOT/config/omarchy/$cf" ]]; then
    check "config/omarchy/$cf" pass
  else
    check "config/omarchy/$cf" warn "missing (run scripts/setup_desktop_dotfiles.sh)"
  fi
done

# ── Sovereign Desktop Tooling ─────────────────────────────────────────────────
section "Sovereign Desktop Tooling"

check_file "scripts/setup_desktop_dotfiles.sh"  "$SIGMA_ROOT/scripts/setup_desktop_dotfiles.sh"
check_file "scripts/launch_desktop_persona.sh"  "$SIGMA_ROOT/scripts/launch_desktop_persona.sh"
check_file "bin/sigmaos-desktop"                "$SIGMA_ROOT/bin/sigmaos-desktop"
check_file "bin/sigmaos-update"                 "$SIGMA_ROOT/bin/sigmaos-update"
check_file "bin/sigmaos-theme"                  "$SIGMA_ROOT/bin/sigmaos-theme"
check_file "bin/sigmaos-wallpaper-set"          "$SIGMA_ROOT/bin/sigmaos-wallpaper-set"
check_file "bin/sigmaos-pkgmgr"                 "$SIGMA_ROOT/bin/sigmaos-pkgmgr"
check_file "bin/sigmaos-network"                "$SIGMA_ROOT/bin/sigmaos-network"
check_file "bin/sigmaos-audio"                  "$SIGMA_ROOT/bin/sigmaos-audio"
check_file "bin/sigmaos-bluetooth"              "$SIGMA_ROOT/bin/sigmaos-bluetooth"
check_file "bin/sigmaos-snapshot"               "$SIGMA_ROOT/bin/sigmaos-snapshot"
check_file "bin/sigmaos-display"                "$SIGMA_ROOT/bin/sigmaos-display"
check_file "bin/sigmaos-keybinds"               "$SIGMA_ROOT/bin/sigmaos-keybinds"
check_file "bin/sigmaos-quickshell"             "$SIGMA_ROOT/bin/sigmaos-quickshell"
check_file "bin/sigmaos-agent"                  "$SIGMA_ROOT/bin/sigmaos-agent"
check_file "bin/sigmaos-agent-usage"            "$SIGMA_ROOT/bin/sigmaos-agent-usage"
check_file "bin/sigmaos-crash-diagnostics"      "$SIGMA_ROOT/bin/sigmaos-crash-diagnostics"
check_file "bin/sigmaos-local-ai"               "$SIGMA_ROOT/bin/sigmaos-local-ai"
check_file "bin/sigmaos-screenrecord"           "$SIGMA_ROOT/bin/sigmaos-screenrecord"
check_file "bin/sigmaos-installer-tui"          "$SIGMA_ROOT/bin/sigmaos-installer-tui"
check_file "config/omarchy/quickshell/shell.qml" "$SIGMA_ROOT/config/omarchy/quickshell/shell.qml"
check_file "bin/sigmaos-info"                   "$SIGMA_ROOT/bin/sigmaos-info"

# config/sigmaos mirrors
if [[ -d "$HOME/.config/sigmaos" ]]; then
  check "~/.config/sigmaos/ deployed"    pass
else
  check "~/.config/sigmaos/ deployed"    warn "not deployed — run: sigmaos-update --dotfiles-only"
fi

# launch_desktop_persona target validity
if [[ -f "$SIGMA_ROOT/scripts/launch_desktop_persona.sh" ]]; then
  check "launch_desktop_persona.sh exists" pass
else
  check "launch_desktop_persona.sh exists" fail "missing sovereign desktop launcher"
fi

# sigmaos-update sanity
if [[ -x "$SIGMA_ROOT/bin/sigmaos-update" ]]; then
  check "sigmaos-update executable"       pass
else
  check "sigmaos-update executable"       warn "not executable (chmod +x bin/sigmaos-update)"
fi

# ── Summary ───────────────────────────────────────────────────────────────────
total=$(( pass_count + warn_count + fail_count ))
echo ""
echo -e "${BOLD}╔══════════════════════════════════════════════════╗${RESET}"
printf    "${BOLD}║  Results: %d total  ${GREEN}%d pass${RESET}${BOLD}  ${YELLOW}%d warn${RESET}${BOLD}  ${RED}%d fail${RESET}${BOLD}  ║${RESET}\n" \
          "$total" "$pass_count" "$warn_count" "$fail_count"
echo -e "${BOLD}╚══════════════════════════════════════════════════╝${RESET}"
echo ""

if [[ $fail_count -gt 0 ]]; then
  echo -e "  ${RED}Some checks failed. Review the FAIL items above.${RESET}"
  exit 1
elif [[ $warn_count -gt 0 ]]; then
  echo -e "  ${YELLOW}System functional with warnings. See WARN items above.${RESET}"
  exit 0
else
  echo -e "  ${GREEN}All checks passed! SigmaOS is ready.${RESET}"
  exit 0
fi
