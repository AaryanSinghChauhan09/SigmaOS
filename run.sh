#!/usr/bin/env bash
# ==============================================================================
# SigmaOS — Sovereign Operating System Runner
# Turnkey, zero-friction execution inspired by Omarchy Linux & Arch Linux.
#
# Usage:
#   ./run.sh              # Launch interactive sovereign workstation
#   ./run.sh --omarchy    # Launch Omarchy aesthetic persona (Hyprland / Wallust)
#   ./run.sh --arch       # Launch Arch Linux minimalist persona (Pacman / Rolling)
#   ./run.sh --doctor     # Run Omarchy diagnostic health checks
#   ./run.sh --check      # Run 21-point launch readiness validation
#   ./run.sh --iso        # Build & verify bootable ISO image
#   ./run.sh --qemu       # Boot ISO in QEMU virtual machine
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Ensure bin directory exists
mkdir -p "$SCRIPT_DIR/bin"

# Build workstation binary if missing or if source is newer
if [ ! -f "$SCRIPT_DIR/bin/sigmaos" ] || [ "$SCRIPT_DIR/src/bin/sigmaos.rs" -nt "$SCRIPT_DIR/bin/sigmaos" ]; then
    echo "==> Building native SigmaOS Workstation binary..."
    rustc -O "$SCRIPT_DIR/src/bin/sigmaos.rs" -o "$SCRIPT_DIR/bin/sigmaos"
fi

ACTION="${1:-}"

case "$ACTION" in
    --omarchy|omarchy)
        exec "$SCRIPT_DIR/bin/sigmaos" --omarchy
        ;;
    --arch|arch)
        exec "$SCRIPT_DIR/bin/sigmaos" --arch
        ;;
    --mint|mint)
        exec "$SCRIPT_DIR/bin/sigmaos" --mint
        ;;
    --doctor|doctor)
        exec "$SCRIPT_DIR/bin/sigmaos" --doctor
        ;;
    --check|check)
        exec "$SCRIPT_DIR/bin/sigmaos" --check
        ;;
    --craft|craft|-c)
        exec "$SCRIPT_DIR/bin/sigmaos" --craft
        ;;
    --distro|distro|-d)
        exec "$SCRIPT_DIR/bin/sigmaos" --distro
        ;;
    --status|status|-s)
        exec "$SCRIPT_DIR/bin/sigmaos" --status
        ;;
    --gui|gui|--desktop|desktop)
        exec python3 "$SCRIPT_DIR/scripts/sigmaos_gui.py"
        ;;
    --cli|cli|--tui|tui)
        if [ -t 0 ] && [ -f "$SCRIPT_DIR/bin/sigmaos-launch" ]; then
            exec "$SCRIPT_DIR/bin/sigmaos-launch"
        else
            exec "$SCRIPT_DIR/bin/sigmaos"
        fi
        ;;
    --iso|iso)
        if [ ! -f "$SCRIPT_DIR/build/sigmaos-desktop-preview.iso" ]; then
            echo "==> Synthesizing bootable ISO..."
            make iso-stub
        fi
        exec ls -lh "$SCRIPT_DIR/build/sigmaos-desktop-preview.iso"
        ;;
    --qemu|qemu)
        if [ ! -f "$SCRIPT_DIR/build/sigmaos-desktop-preview.iso" ]; then
            make iso-stub
        fi
        if command -v qemu-system-x86_64 >/dev/null 2>&1; then
            exec qemu-system-x86_64 -m 2048 -cdrom "$SCRIPT_DIR/build/sigmaos-desktop-preview.iso" -serial stdio -no-reboot
        else
            echo "==> QEMU not found in PATH. Launching native SigmaOS Workstation..."
            if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
                exec python3 "$SCRIPT_DIR/scripts/sigmaos_gui.py"
            else
                exec "$SCRIPT_DIR/bin/sigmaos-launch"
            fi
        fi
        ;;
    --help|-h|help)
        exec "$SCRIPT_DIR/bin/sigmaos" --help
        ;;
    "")
        # Direct, zero-friction launch: if in an interactive terminal, launch the live TUI menu!
        if [ -t 0 ] && [ -f "$SCRIPT_DIR/bin/sigmaos-launch" ]; then
            exec "$SCRIPT_DIR/bin/sigmaos-launch"
        elif [ -f "$SCRIPT_DIR/bin/sigmaos-launch" ]; then
            exec "$SCRIPT_DIR/bin/sigmaos-launch"
        else
            exec "$SCRIPT_DIR/bin/sigmaos"
        fi
        ;;
    *)
        exec "$SCRIPT_DIR/bin/sigmaos" "$@"
        ;;
esac
