#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS Sovereign Desktop Profile Launcher (Zenith Edition)
# Enables Hyprland dynamic tiling, Wallust wallpaper theme switching, and Starship prompt.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Direct subcommand dispatch
if [ $# -gt 0 ]; then
    case "${1:-}" in
        doctor)
            exec "$ROOT_DIR/bin/sigmaos-doctor" "${@:2}"
            ;;
        theme)
            if [ "${2:-}" = "list" ] || [ -z "${2:-}" ]; then
                echo "Available Desktop Themes: Catppuccin Mocha, Tokyo Night, Nord, Gruvbox, Cyber, Zenith"
                exec "$ROOT_DIR/bin/sigmaos-theme-current"
            else
                exec "$ROOT_DIR/bin/sigmaos-theme-bg-switcher" "${2:-}"
            fi
            ;;
        gui|desktop)
            exec "$ROOT_DIR/bin/sigmaos-gui" "${@:2}"
            ;;
        native)
            exec "$ROOT_DIR/bin/sigmaos-native" "${@:2}"
            ;;
        settings)
            exec "$ROOT_DIR/bin/sigmaos-settings" "${@:2}"
            ;;
        dotfiles)
            exec "$ROOT_DIR/scripts/setup_desktop_dotfiles.sh" "${@:2}"
            ;;
        version|--version|-v)
            exec "$ROOT_DIR/bin/sigmaos-version"
            ;;
        help|--help|-h)
            echo "SigmaOS Sovereign Desktop Profile CLI"
            echo "Usage: sigmaos-desktop [doctor | theme [list|<name>] | gui | native | settings | dotfiles | version | help]"
            exit 0
            ;;
        *)
            echo "Unknown subcommand: $1" >&2
            echo "Usage: sigmaos-desktop [doctor | theme [list|<name>] | gui | native | settings | dotfiles | version | help]" >&2
            exit 1
            ;;
    esac
fi

# Auto-launch graphical desktop if display environment is active and not explicitly CLI mode
if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
    if [ "${1:-}" != "--cli" ] && [ "${1:-}" != "--tui" ] && [ "${1:-}" != "-i" ]; then
        echo -e "\033[1;36m==> Graphical display detected. Launching SigmaOS Sovereign Desktop (Zenith)...\033[0m"
        exec python3 "$ROOT_DIR/scripts/sigmaos_gui.py" --persona sovereign "$@"
    fi
fi

# Terminal Persona / TUI Experience
echo -e "\033[1;36m"
cat << "EOF"
 ╭─────────────────────────────────────────────────────────────────╮
 │          SIGMAOS · SOVEREIGN DESKTOP PROFILE (ZENITH)           │
 │       Dynamic Tiling · Dynamic Palette · High-Performance       │
 ╰─────────────────────────────────────────────────────────────────╯
EOF
echo -e "\033[0m"

echo "--> Initializing Sovereign Desktop Profile on SigmaOS Substrate..."
export PATH="$ROOT_DIR/bin:$ROOT_DIR/scripts:${PATH}"
export HYPRLAND_CONFIG="${HOME}/.config/hypr/hyprland.conf"
export WAYBAR_CONFIG="${HOME}/.config/waybar/config.jsonc"
export WALLUST_CONFIG="${HOME}/.config/wallust/wallust.toml"
export STARSHIP_CONFIG="${HOME}/.config/starship.toml"

echo -e "\033[1mSovereign Desktop Profile Active.\033[0m Fast, aesthetic, keyboard-driven."
echo ""
echo "  [Commands]:"
echo "    doctor         - Run 39-point system health audit"
echo "    theme list     - Display available wallpaper themes"
echo "    theme <name>   - Switch wallpaper theme live"
echo "    gui            - Open full graphical desktop window"
echo "    dotfiles       - Deploy desktop dotfiles to ~/.config"
echo "    exit           - Return to standard workstation REPL"
echo ""

if [ -t 0 ]; then
    echo -e "\033[1;36m==> Entering live Sovereign Desktop shell environment...\033[0m"
    echo -e "    Type \033[1;32mexit\033[0m to return from the session.\n"

    TEMP_RC="$(mktemp /tmp/sigmaos_rc.XXXXXX)"
    trap 'rm -f "$TEMP_RC"' EXIT

    cat << 'EOF_RC' > "$TEMP_RC"
export PS1="\[\033[1;36m\]󰣇 sigmaos:desktop\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]\$ "
export SIGMAOS_PERSONA="desktop"
alias doctor="sigmaos-doctor"
alias theme="sigmaos-theme-bg-switcher"
alias fastfetch="sigmafetch"
alias dotfiles="setup_desktop_dotfiles.sh"
EOF_RC

    bash --rcfile "$TEMP_RC" -i
fi
