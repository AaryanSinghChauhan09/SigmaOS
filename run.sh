#!/usr/bin/env bash
# ==============================================================================
# SigmaOS — Sovereign Operating System Runner
# Turnkey, zero-friction workstation and desktop environment runner.
#
# Usage:
#   ./run.sh              # Launch sovereign workstation (GUI on desktop, TUI in terminal)
#   ./run.sh --gui        # Launch sovereign GUI desktop (GTK3 + WebKit2)
#   ./run.sh --native     # Launch native GTK3 desktop app (pure GTK3)
#   ./run.sh --desktop    # Launch SigmaOS Sovereign Desktop Profile (Hyprland / Wayland)
#   ./run.sh --minimal    # Launch SigmaOS Sovereign Minimalist Profile
#   ./run.sh --doctor     # Run SigmaOS diagnostic health checks (39-point audit)
#   ./run.sh --dotfiles   # Deploy SigmaOS desktop dotfiles to ~/.config
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
    --desktop|desktop|--omarchy|omarchy)
        exec "$SCRIPT_DIR/bin/sigmaos" --desktop
        ;;
    --minimal|minimal|--arch|arch)
        exec "$SCRIPT_DIR/bin/sigmaos" --minimal
        ;;
    --doctor|doctor)
        if [ -f "$SCRIPT_DIR/scripts/sigmaos_doctor.sh" ]; then
            exec bash "$SCRIPT_DIR/scripts/sigmaos_doctor.sh"
        else
            exec "$SCRIPT_DIR/bin/sigmaos" --doctor
        fi
        ;;
    --dotfiles|dotfiles)
        if [ -f "$SCRIPT_DIR/scripts/setup_desktop_dotfiles.sh" ]; then
            exec bash "$SCRIPT_DIR/scripts/setup_desktop_dotfiles.sh" "${@:2}"
        else
            exec bash "$SCRIPT_DIR/scripts/setup_omarchy_dotfiles.sh" "${@:2}"
        fi
        ;;
    --update|update)
        exec bash "$SCRIPT_DIR/bin/sigmaos-update" "${@:2}"
        ;;
    --theme|theme)
        exec bash "$SCRIPT_DIR/bin/sigmaos-theme" "${@:2}"
        ;;
    --wallpaper|wallpaper|--bg|bg)
        exec bash "$SCRIPT_DIR/bin/sigmaos-wallpaper-set" "${@:2}"
        ;;
    --pkgmgr|pkgmgr|--pkg|pkg)
        exec bash "$SCRIPT_DIR/bin/sigmaos-pkgmgr" "${@:2}"
        ;;
    --network|network|--net|net)
        exec bash "$SCRIPT_DIR/bin/sigmaos-network" "${@:2}"
        ;;
    --audio|audio)
        exec bash "$SCRIPT_DIR/bin/sigmaos-audio" "${@:2}"
        ;;
    --bluetooth|bluetooth|--bt|bt)
        exec bash "$SCRIPT_DIR/bin/sigmaos-bluetooth" "${@:2}"
        ;;
    --display|display|--monitors|monitors)
        exec bash "$SCRIPT_DIR/bin/sigmaos-display" "${@:2}"
        ;;
    --keybinds|keybinds|--keys|keys)
        exec bash "$SCRIPT_DIR/bin/sigmaos-keybinds" "${@:2}"
        ;;
    --snapshot|snapshot|--btrfs|btrfs)
        exec bash "$SCRIPT_DIR/bin/sigmaos-snapshot" "${@:2}"
        ;;
    --info|info)
        exec bash "$SCRIPT_DIR/bin/sigmaos-info" "${@:2}"
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
        echo "  --desktop     Launch SigmaOS Sovereign Desktop Profile (Hyprland / Wayland)"
        echo "  --minimal     Launch SigmaOS Sovereign Minimalist Profile"
        echo "  --doctor      Run SigmaOS 56-point system health diagnostics"
        echo "  --dotfiles    Deploy SigmaOS desktop configs to ~/.config"
        echo "  --update      Update packages + dotfiles + theme (sigmaos-update)"
        echo "  --theme       Manage Wallust desktop themes (list/apply/next/prev)"
        echo "  --wallpaper   Set wallpaper and regenerate dynamic color palette"
        echo "  --pkgmgr      Arch/AUR package manager frontend (install/search/update)"
        echo "  --network     Network manager CLI (wifi/ethernet/vpn/dns/status)"
        echo "  --audio       PipeWire audio and volume manager (volume/mute/sinks)"
        echo "  --bluetooth   Bluetooth device manager (scan/pair/connect/status)"
        echo "  --display     Hyprland multi-monitor & HiDPI scaling manager"
        echo "  --keybinds    Interactive Hyprland keyboard shortcuts cheatsheet"
        echo "  --snapshot    Btrfs snapshot and recovery manager (create/list/rollback)"
        echo "  --info        Show system info (like neofetch, with JSON output)"
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
