#!/usr/bin/env bash
# SigmaOS Omarchy Linux Aesthetic Persona Launcher
# Enables Hyprland-style dynamic tiling, Wallust wallpaper theme switching, and Starship shell.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Direct subcommand dispatch (avoids banner when used as CLI tool)
if [ $# -gt 0 ]; then
    case "${1:-}" in
        doctor)
            exec "$ROOT_DIR/bin/sigmaos-doctor" "${@:2}"
            ;;
        theme)
            if [ "${2:-}" = "list" ] || [ -z "${2:-}" ]; then
                echo "Available Wallust Themes: Catppuccin Mocha, Tokyo Night, Nord, Gruvbox, Cyber, Mint, Arch"
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
        version|--version|-v)
            exec "$ROOT_DIR/bin/sigmaos-version"
            ;;
        help|--help|-h)
            echo "Omarchy Aesthetic Persona CLI"
            echo "Usage: omarchy [doctor | theme [list|<name>] | gui | native | settings | version | help]"
            exit 0
            ;;
        *)
            echo "Unknown subcommand: $1" >&2
            echo "Usage: omarchy [doctor | theme [list|<name>] | gui | native | settings | version | help]" >&2
            exit 1
            ;;
    esac
fi

# Auto-launch graphical desktop if display environment is active and not explicitly CLI mode
if [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; then
    if [ "${1:-}" != "--cli" ] && [ "${1:-}" != "--tui" ] && [ "${1:-}" != "-i" ]; then
        echo -e "\033[1;35m==> Graphical display detected. Launching SigmaOS Sovereign Desktop (Omarchy)...\033[0m"
        exec python3 "$ROOT_DIR/scripts/sigmaos_gui.py" --persona omarchy "$@"
    fi
fi

echo -e "\033[1;35m"
cat <<'EOF'
 ╭──────────────────────────────────────────────────────────────────╮
 │               SIGMAOS · OMARCHY LINUX AESTHETIC PERSONA          │
 │    Hyprland Wayland · Wallust Color Harmony · Starship · Aether   │
 ╰──────────────────────────────────────────────────────────────────╯
EOF
echo -e "\033[0m"

echo "--> Initializing Omarchy Linux Persona on SigmaOS Substrate..."
echo "  [✓] Compositor      : Zenith Wayland Compositor (Hyprland dynamic tiling)"
echo "  [✓] Palette Engine  : Wallust Dynamic Wallpaper-Driven Theme Harmonizer"
echo "  [✓] Status Layer    : Waybar / Quickshell Aesthetic Bar"
echo "  [✓] Shell Runtime   : Starship Prompt + Fish/Zsh Syntax Highlighting"
echo "  [✓] Editor Studio   : Aether.nvim (Neovim Lua-based IDE with LSP)"
echo "  [✓] Terminal Engine : Modern GPU Block Terminal (Alacritty / Kitty / Warp)"
echo ""

# Activate shell plugins for Omarchy layout
if [ -f "$ROOT_DIR/bin/sigmaos-shell" ]; then
    bash "$ROOT_DIR/bin/sigmaos-shell" summon bar "top-waybar" || true
    bash "$ROOT_DIR/bin/sigmaos-shell" summon background "wallust-dynamic" || true
    bash "$ROOT_DIR/bin/sigmaos-shell" summon notifications "mako-aesthetic" || true
fi

echo -e "\033[1mOmarchy Linux Persona Active.\033[0m Fast, aesthetic, keyboard-driven."
echo "Keybindings:"
echo "  Super + Return : Open GPU Terminal"
echo "  Super + D      : Wofi Application Launcher"
echo "  Super + Q      : Close Focused Window"
echo "  Super + M      : Exit Session"
echo ""

# Live interactive session launcher
if [ -t 0 ] && [ "${SIGMAOS_NON_INTERACTIVE:-0}" != "1" ]; then
    echo -e "\033[1;35m==> Dropping into live Omarchy interactive environment...\033[0m"
    echo -e "    Type \033[1;32mexit\033[0m to return from the Omarchy persona session.\n"
    
    TEMP_RC="$(mktemp /tmp/omarchy_rc.XXXXXX)"
    cat << EOF > "$TEMP_RC"
[ -f ~/.bashrc ] && source ~/.bashrc 2>/dev/null
export PATH="${HOME}/.local/bin:${ROOT_DIR}/scripts:${PATH}"
export PS1="\[\033[1;35m\]󰣇 omarchy\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]\$ "
export SIGMAOS_PERSONA="omarchy"
alias pacman="sigpkg pacman"
alias yay="sigpkg"
alias paru="sigpkg"
alias aur="sigpkg aur"
alias update="sigpkg update"
alias doctor="omarchy doctor"
alias theme="omarchy theme"
alias fastfetch="neofetch 2>/dev/null || echo 'SigmaOS Omarchy Workstation'"
alias neofetch="neofetch 2>/dev/null || echo 'SigmaOS Omarchy Workstation'"
EOF
    trap 'rm -f "$TEMP_RC"' EXIT
    bash --rcfile "$TEMP_RC" -i
fi
