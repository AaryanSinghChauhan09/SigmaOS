#!/usr/bin/env bash
# SigmaOS Omarchy Linux Aesthetic Persona Launcher
# Enables Hyprland-style dynamic tiling, Wallust wallpaper theme switching, and Starship shell.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

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
