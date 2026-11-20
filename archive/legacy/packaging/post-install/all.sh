#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# post-install/all.sh — SigmaOS post-installation setup
# Run once after base system install to configure the full sovereign desktop.
#
# Usage:
#   sudo bash post-install/all.sh [--dry-run] [-h|--help]
#
# What it does:
#   1. Enable core systemd services (NetworkManager, pipewire, sddm)
#   2. Enable SigmaOS systemd units (provision-owner, update timer)
#   3. Set Fish as default shell for the target user
#   4. Deploy dotfiles via setup_desktop_dotfiles.sh
#   5. Enable pacman hook for auto dotfile sync on hyprland/waybar upgrades
#   6. Print next-steps summary

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[post-install]${RESET} $*"; }
success() { echo -e "${GREEN}[post-install]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[post-install]${RESET} $*"; }
err()     { echo -e "${RED}[post-install]${RESET} $*" >&2; }

usage() {
  cat <<'EOF'
post-install/all.sh — SigmaOS post-installation setup

USAGE
  sudo bash post-install/all.sh [OPTIONS]

OPTIONS
  --dry-run   Print what would be done; make no changes
  --user <u>  Target user account (default: $SUDO_USER or first non-root user)
  -h, --help  Show this help
EOF
}

DRY_RUN=0
TARGET_USER="${SUDO_USER:-}"

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=1 ;;
    --user)    shift; TARGET_USER="$1" ;;
    -h|--help) usage; exit 0 ;;
  esac
done

# Discover target user if not set
if [ -z "$TARGET_USER" ]; then
  TARGET_USER=$(awk -F: '$3>=1000 && $1!="nobody"{print $1; exit}' /etc/passwd 2>/dev/null || echo "")
fi
if [ -z "$TARGET_USER" ]; then
  err "Cannot determine target user. Pass --user <username>"
  exit 1
fi

run() {
  if [ "$DRY_RUN" = "1" ]; then
    warn "[DRY-RUN] $*"
  else
    eval "$@"
  fi
}

ERRORS=0

echo -e "${BOLD}${CYAN}"
echo "╔══════════════════════════════════════════════╗"
echo "║   SigmaOS Post-Installation Setup            ║"
echo "╚══════════════════════════════════════════════╝"
echo -e "${RESET}"
[ "$DRY_RUN" = "1" ] && warn "DRY-RUN mode active — no changes will be made"
info "Target user: $TARGET_USER"
echo ""

# ── Step 1: Systemd core services ────────────────────────────────────────────
info "1/6 Enabling core systemd services..."
for svc in NetworkManager bluetooth pipewire pipewire-pulse wireplumber; do
  if systemctl list-unit-files "$svc.service" &>/dev/null; then
    run "systemctl enable $svc.service" && success "  enabled: $svc" || warn "  skipped: $svc"
  fi
done
# Display manager
for dm in sddm gdm ly; do
  if systemctl list-unit-files "$dm.service" &>/dev/null; then
    run "systemctl enable $dm.service" && success "  display manager enabled: $dm"
    break
  fi
done

# ── Step 2: SigmaOS systemd units ────────────────────────────────────────────
info "2/6 Enabling SigmaOS systemd units..."
SERVICE_SRC="$ROOT_DIR/iso/archiso/airootfs/usr/lib/systemd/system"
if [ -d "$SERVICE_SRC" ]; then
  for unit in sigmaos-provision-owner.service sigmaos-update.timer; do
    if [ -f "$SERVICE_SRC/$unit" ]; then
      run "cp '$SERVICE_SRC/$unit' /usr/lib/systemd/system/"
      run "systemctl enable $unit" && success "  enabled: $unit"
    fi
  done
  run "systemctl daemon-reload"
else
  warn "  Service source not found: $SERVICE_SRC"
fi

# ── Step 3: Set Fish as default shell ─────────────────────────────────────────
info "3/6 Setting Fish as default shell for $TARGET_USER..."
FISH_PATH=$(command -v fish 2>/dev/null || echo "")
if [ -n "$FISH_PATH" ]; then
  if ! grep -q "^$FISH_PATH" /etc/shells 2>/dev/null; then
    run "echo '$FISH_PATH' >> /etc/shells"
  fi
  run "chsh -s '$FISH_PATH' '$TARGET_USER'" && success "  Fish set as default shell"
else
  warn "  Fish not installed (pacman -S fish)"
  ERRORS=$((ERRORS+1))
fi

# ── Step 4: Deploy dotfiles ───────────────────────────────────────────────────
info "4/6 Deploying sovereign desktop dotfiles..."
DOTFILE_SCRIPT="$ROOT_DIR/scripts/setup_desktop_dotfiles.sh"
if [ -f "$DOTFILE_SCRIPT" ]; then
  DRY_FLAG=""
  [ "$DRY_RUN" = "1" ] && DRY_FLAG="--dry-run"
  run "sudo -u '$TARGET_USER' bash '$DOTFILE_SCRIPT' $DRY_FLAG" && success "  Dotfiles deployed"
else
  err "  setup_desktop_dotfiles.sh not found"
  ERRORS=$((ERRORS+1))
fi

# ── Step 5: Wayland session entry ─────────────────────────────────────────────
info "5/6 Installing Wayland session .desktop entry..."
DESKTOP_SRC="$ROOT_DIR/iso/archiso/airootfs/usr/share/wayland-sessions/sigmaos-hyprland.desktop"
if [ -f "$DESKTOP_SRC" ]; then
  run "mkdir -p /usr/share/wayland-sessions"
  run "cp '$DESKTOP_SRC' /usr/share/wayland-sessions/"
  success "  SigmaOS Hyprland session entry installed"
else
  warn "  Desktop session file not found: $DESKTOP_SRC"
fi

# ── Step 6: Pacman hook ───────────────────────────────────────────────────────
info "6/6 Installing pacman post-upgrade hook..."
HOOK_SRC="$ROOT_DIR/iso/archiso/airootfs/etc/pacman.d/hooks/sigmaos-post-upgrade.hook"
if [ -f "$HOOK_SRC" ]; then
  run "mkdir -p /etc/pacman.d/hooks"
  run "cp '$HOOK_SRC' /etc/pacman.d/hooks/"
  success "  pacman hook installed"
else
  warn "  Hook source not found: $HOOK_SRC"
fi

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
if [ "$ERRORS" -gt 0 ]; then
  err "Post-install finished with $ERRORS error(s)."
  err "Review the output above and re-run after fixing."
  exit 1
else
  success "SigmaOS post-installation complete ✓"
  echo ""
  echo -e "  ${BOLD}Next steps:${RESET}"
  echo "   1. Reboot: sudo reboot"
  echo "   2. At login screen, select 'SigmaOS Hyprland' session"
  echo "   3. Run health check: sigmaos-doctor"
  echo "   4. Update: sigmaos-update"
  echo ""
fi
