#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# ----------------------------------------------------------------------------
# setup_omarchy_dotfiles.sh: Deploy Omarchy Linux desktop configs to ~/.config
# ----------------------------------------------------------------------------

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMA_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_SRC="$SIGMA_ROOT/config/omarchy"
TARGET_DIR="${XDG_CONFIG_HOME:-$HOME/.config}"

echo "============================================================"
echo "  Deploying Omarchy Linux Desktop Configurations to ~/.config"
echo "============================================================"

# Ensure directories exist
mkdir -p "$TARGET_DIR"/{hypr,waybar,wallust,rofi,mako,dunst,foot,alacritty,kitty,wlogout,nvim/colors,fish,omarchy/plugins}

# 1. Hyprland suite
echo "[+] Deploying Hyprland, Hypridle, Hyprlock..."
cp "$CONFIG_SRC/hyprland/hyprland.conf" "$TARGET_DIR/hypr/hyprland.conf"
cp "$CONFIG_SRC/hyprland/hyprland-binds.conf" "$TARGET_DIR/hypr/hyprland-binds.conf"
cp "$CONFIG_SRC/hyprland/hypridle.conf" "$TARGET_DIR/hypr/hypridle.conf"
cp "$CONFIG_SRC/hyprland/hyprlock.conf" "$TARGET_DIR/hypr/hyprlock.conf"

# 2. Waybar status bar
echo "[+] Deploying Waybar..."
cp "$CONFIG_SRC/waybar/config.jsonc" "$TARGET_DIR/waybar/config.jsonc"
cp "$CONFIG_SRC/waybar/style.css" "$TARGET_DIR/waybar/style.css"
if [[ -f "$CONFIG_SRC/waybar/wallust-colors.css" ]]; then
    cp "$CONFIG_SRC/waybar/wallust-colors.css" "$TARGET_DIR/waybar/wallust-colors.css"
fi

# 3. Wallust dynamic palette generator
echo "[+] Deploying Wallust..."
cp "$CONFIG_SRC/wallust/wallust.toml" "$TARGET_DIR/wallust/wallust.toml"
mkdir -p "$TARGET_DIR/wallust/templates"
cp -r "$CONFIG_SRC/wallust/templates/"* "$TARGET_DIR/wallust/templates/"

# 4. Terminals (Foot, Alacritty, Kitty)
echo "[+] Deploying Terminals (Foot, Alacritty, Kitty)..."
cp "$CONFIG_SRC/foot/foot.ini" "$TARGET_DIR/foot/foot.ini"
cp "$CONFIG_SRC/alacritty/alacritty.toml" "$TARGET_DIR/alacritty/alacritty.toml"
cp "$CONFIG_SRC/kitty/kitty.conf" "$TARGET_DIR/kitty/kitty.conf"

# 5. Application Launchers & Notifications
echo "[+] Deploying Rofi, Mako, Dunst..."
cp "$CONFIG_SRC/rofi/sigmaos.rasi" "$TARGET_DIR/rofi/sigmaos.rasi"
cp "$CONFIG_SRC/mako/config" "$TARGET_DIR/mako/config"
cp "$CONFIG_SRC/dunst/dunstrc" "$TARGET_DIR/dunst/dunstrc"

# 6. Wlogout & Starship
echo "[+] Deploying Wlogout & Starship..."
cp "$CONFIG_SRC/wlogout/layout" "$TARGET_DIR/wlogout/layout"
cp "$CONFIG_SRC/wlogout/style.css" "$TARGET_DIR/wlogout/style.css"
cp "$CONFIG_SRC/starship/starship.toml" "$TARGET_DIR/starship.toml"

# 7. Neovim & Colorschemes (Aether, Lumon)
echo "[+] Deploying Neovim configuration & dynamic colorschemes..."
cp "$CONFIG_SRC/nvim/init.lua" "$TARGET_DIR/nvim/init.lua"
cp "$CONFIG_SRC/nvim/colors/"*.lua "$TARGET_DIR/nvim/colors/"

# 8. Shell Environments (Zsh & Fish)
echo "[+] Deploying Zsh & Fish shells..."
cp "$CONFIG_SRC/fish/config.fish" "$TARGET_DIR/fish/config.fish"
if [[ -f "$CONFIG_SRC/zsh/.zshrc" ]]; then
    cp "$CONFIG_SRC/zsh/.zshrc" "$HOME/.zshrc"
fi

# 9. Omarchy Plugins (Theme Sync, Local AI, Port Forward)
echo "[+] Deploying Omarchy Plugins..."
cp "$CONFIG_SRC/plugins/"* "$TARGET_DIR/omarchy/plugins/"

echo "============================================================"
echo "  [✓] All Omarchy configurations successfully deployed!"
echo "============================================================"
