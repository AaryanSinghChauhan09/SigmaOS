#!/usr/bin/env bash
# ==============================================================================
# SigmaOS — Sovereign Operating System Runner
# Turnkey, zero-friction execution inspired by Omarchy Linux & Arch Linux.
#
# Usage:
#   ./run.sh              # Launch sovereign workstation (GUI on desktop, TUI in terminal)
#   ./run.sh --gui        # Launch sovereign GUI desktop (GTK3 + WebKit2)
#   ./run.sh --native     # Launch native GTK3 desktop app (pure GTK3)
#   ./run.sh --omarchy    # Launch Omarchy aesthetic persona (Hyprland / Wallust)
#   ./run.sh --arch       # Launch Arch Linux minimalist persona (Pacman / Rolling)
#   ./run.sh --doctor     # Run Omarchy diagnostic health checks (39-point audit)
#   ./run.sh --dotfiles   # Deploy Omarchy dotfiles to ~/.config
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
        if [ -f "$SCRIPT_DIR/scripts/sigmaos_doctor.sh" ]; then
            exec bash "$SCRIPT_DIR/scripts/sigmaos_doctor.sh"
        else
            exec "$SCRIPT_DIR/bin/sigmaos" --doctor
        fi
        ;;
    --dotfiles|dotfiles)
        exec bash "$SCRIPT_DIR/scripts/setup_omarchy_dotfiles.sh"
        ;;
    --sandbox|sandbox)
        exec bash "$SCRIPT_DIR/scripts/sigmaos_sandbox_container.sh" "${@:2}"
        ;;
    --version|-v|version)
        exec "$SCRIPT_DIR/bin/sigmaos" --version
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
        exec python3 "$SCRIPT_DIR/scripts/sigmaos_gui.py" "${@:2}"
        ;;
    --native|native)
        exec python3 "$SCRIPT_DIR/scripts/sigmaos_native_app.py"
        ;;
    --web|web|--server|server)
        exec python3 "$SCRIPT_DIR/scripts/sigmaos_gui.py" --web "${@:2}"
        ;;
    --browser|browser)
        exec python3 "$SCRIPT_DIR/scripts/sigmaos_gui.py" --browser "${@:2}"
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
        echo "SigmaOS — Sovereign Operating System Runner"
        echo ""
        echo "Usage: ./run.sh [OPTION]"
        echo ""
        echo "Options:"
        echo "  (none)        Launch GUI desktop if DISPLAY/WAYLAND available, else interactive TUI"
        echo "  --gui         Launch full graphical desktop (GTK3 + WebKit2 + IPC)"
        echo "  --native      Launch pure GTK3 desktop interface (no WebKit2 requirement)"
        echo "  --cli, --tui  Launch interactive terminal menu"
        echo "  --omarchy     Launch Omarchy aesthetic persona (Hyprland / Wallust)"
        echo "  --arch        Launch Arch Linux minimalist persona (Pacman / Rolling)"
        echo "  --mint        Launch Linux Mint ergonomic persona"
        echo "  --doctor      Run Omarchy 39-point system health diagnostics"
        echo "  --dotfiles    Deploy Omarchy desktop configs to ~/.config"
        echo "  --sandbox     Launch isolated bubblewrap sandbox with GUI passthrough"
        echo "  --version     Display SigmaOS version"
        echo "  --check       Run 21-point launch readiness validation"
        echo "  --status      Display system and subsystem status"
        echo "  --iso         Build and verify bootable ISO image"
        echo "  --qemu        Boot ISO in QEMU virtual machine"
        echo "  --help        Show this help message"
        exit 0
        ;;
    "")
        # Direct, zero-friction launch:
        # If graphical display is active, launch GUI desktop!
        if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
            echo "==> Graphical display detected ($DISPLAY). Launching SigmaOS Desktop GUI..."
            exec python3 "$SCRIPT_DIR/scripts/sigmaos_gui.py"
        elif [ -t 0 ] && [ -f "$SCRIPT_DIR/bin/sigmaos-launch" ]; then
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
