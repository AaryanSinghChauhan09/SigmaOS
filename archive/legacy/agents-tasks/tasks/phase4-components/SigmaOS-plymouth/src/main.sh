#!/usr/bin/env bash
# SigmaOS Plymouth — boot splash integration and theme manager scaffold
# Upstream: Plymouth (https://www.freedesktop.org/wiki/Software/Plymouth/)
set -euo pipefail

PLYMOUTH_THEMES_DIR="${SIGMA_PLYMOUTH_DIR:-/usr/share/plymouth/themes}"
DEFAULT_THEME="sigmaos"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS Plymouth boot splash manager

Subcommands:
  list              List installed Plymouth themes
  active            Show the currently active theme
  set   <theme>     Set the active boot splash theme
  install <pkg>     Install a Plymouth theme package
  remove  <theme>   Remove a Plymouth theme
  preview <theme>   Preview a theme (simulated, needs Plymouth)
  initramfs         Rebuild initramfs to apply theme change
  status            Show Plymouth installation state
  version           Print manager version

EOF
}

cmd_list() {
    echo "Installed Plymouth themes:"
    if [ -d "$PLYMOUTH_THEMES_DIR" ]; then
        for d in "$PLYMOUTH_THEMES_DIR"/*/; do
            local name
            name=$(basename "$d")
            echo "  $name"
        done
    else
        # Scaffold: show expected default themes
        echo "  sigmaos    (SigmaOS default)"
        echo "  bgrt       (vendor logo)"
        echo "  spinner    (spinner fallback)"
        echo "  text       (text mode)"
        echo "  (themes dir: $PLYMOUTH_THEMES_DIR)"
    fi
}

cmd_active() {
    if command -v plymouth-set-default-theme &>/dev/null; then
        echo "Active theme: $(plymouth-set-default-theme)"
    else
        echo "Active theme: $DEFAULT_THEME (scaffold default)"
    fi
}

cmd_set() {
    local theme="${1:?'theme name required'}"
    echo "[plymouth] Setting theme: $theme"
    if command -v plymouth-set-default-theme &>/dev/null; then
        plymouth-set-default-theme "$theme"
        echo "[plymouth] Theme set. Run: $(basename "$0") initramfs  to apply"
    else
        echo "[plymouth] plymouth-set-default-theme not found"
        echo "[plymouth] Manual: edit /etc/plymouth/plymouthd.conf → Theme=$theme"
        echo "[plymouth] Then rebuild initramfs: mkinitcpio -P  or  sigmaos-mkinitramfs --all-kernels"
    fi
}

cmd_install() {
    local pkg="${1:?'theme package required'}"
    echo "[plymouth] Installing theme package: $pkg"
    echo "  sigmaos-pkg install $pkg"
    echo "  (scaffold: printed, not executed)"
}

cmd_remove() {
    local theme="${1:?'theme name required'}"
    echo "[plymouth] Removing theme: $theme"
    if [ "$theme" = "$DEFAULT_THEME" ]; then
        echo "[plymouth] Cannot remove the default SigmaOS theme" >&2; exit 1
    fi
    echo "  sigmaos-pkg remove plymouth-theme-$theme"
    echo "  (scaffold: printed, not executed)"
}

cmd_preview() {
    local theme="${1:?'theme name required'}"
    echo "[plymouth] Previewing theme: $theme"
    if command -v plymouthd &>/dev/null && command -v plymouth &>/dev/null; then
        plymouthd --debug --attach-to-session
        sleep 1
        plymouth --show-splash
        sleep 3
        plymouth --quit
    else
        echo "[plymouth] plymouthd not available in this env"
        echo "[plymouth] To preview: boot with plymouth enabled kernel"
    fi
}

cmd_initramfs() {
    echo "[plymouth] Rebuilding initramfs to apply Plymouth theme..."
    if command -v sigmaos-mkinitramfs &>/dev/null; then
        sigmaos-mkinitramfs --all-kernels
    elif command -v mkinitcpio &>/dev/null; then
        mkinitcpio -P
    elif command -v dracut &>/dev/null; then
        dracut --force
    else
        echo "[plymouth] No initramfs builder found" >&2
        echo "[plymouth] Install sigmaos-initramfs and rerun" >&2
        exit 1
    fi
    echo "[plymouth] Initramfs rebuilt. Reboot to see new theme."
}

cmd_status() {
    echo "Plymouth dir    : $PLYMOUTH_THEMES_DIR"
    if command -v plymouth &>/dev/null; then
        echo "Plymouth binary : $(command -v plymouth)"
    else
        echo "Plymouth binary : not found (install: sigmaos-pkg install plymouth)"
    fi
    echo "Default theme   : $DEFAULT_THEME"
    cmd_active
}

case "${1:-}" in
    list)       cmd_list ;;
    active)     cmd_active ;;
    set)        cmd_set "${2:-}" ;;
    install)    cmd_install "${2:-}" ;;
    remove)     cmd_remove "${2:-}" ;;
    preview)    cmd_preview "${2:-}" ;;
    initramfs)  cmd_initramfs ;;
    status)     cmd_status ;;
    version)    echo "SigmaOS Plymouth manager v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
