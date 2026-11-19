#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# ----------------------------------------------------------------------------
# setup_desktop_dotfiles.sh: Deploy SigmaOS Sovereign Desktop configs to ~/.config
# Idempotent — safe to run multiple times. Uses cp -n (no-clobber) by default.
# Flags:
#   --force    Overwrite existing target configuration files
#   --dry-run  Simulate deployment without modifying the filesystem
# ----------------------------------------------------------------------------
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMA_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_SRC="$SIGMA_ROOT/config/omarchy"
TARGET_DIR="${XDG_CONFIG_HOME:-$HOME/.config}"
FORCE=false
DRY_RUN=false

for arg in "$@"; do
  [[ "$arg" == "--force" ]] && FORCE=true
  [[ "$arg" == "--dry-run" ]] && DRY_RUN=true
done

echo "============================================================"
echo "  Deploying SigmaOS Sovereign Desktop Configurations"
echo "============================================================"
echo "  Source  : $CONFIG_SRC"
echo "  Target  : $TARGET_DIR"
echo "  Force   : $FORCE"
echo "  Dry-run : $DRY_RUN"
echo ""

safe_cp() {
  local src="$1" dst="$2"
  if [[ "$DRY_RUN" == true ]]; then
    echo "  [dry-run] would deploy: $(basename "$src") -> $dst"
    return 0
  fi
  mkdir -p "$(dirname "$dst")"
  if [[ "$FORCE" == true ]]; then
    cp "$src" "$dst"
    echo "  [updated] $dst"
  else
    if [[ -f "$dst" ]]; then
      echo "  [skipped] $dst (exists; use --force to overwrite)"
    else
      cp "$src" "$dst"
      echo "  [created] $dst"
    fi
  fi
}

# 1. Base Target Directories
if [[ "$DRY_RUN" == false ]]; then
  mkdir -p "$TARGET_DIR"/{hypr,waybar,wallust,rofi,mako,dunst,foot,alacritty,kitty,wlogout,nvim/colors,fish,sigmaos,omarchy,starship}
fi

# 2. SigmaOS Desktop Metadata
echo "[+] Deploying SigmaOS Desktop Profile metadata..."
[[ -f "$CONFIG_SRC/shell.json" ]]        && safe_cp "$CONFIG_SRC/shell.json"        "$TARGET_DIR/sigmaos/shell.json"
[[ -f "$CONFIG_SRC/theme.conf" ]]        && safe_cp "$CONFIG_SRC/theme.conf"        "$TARGET_DIR/sigmaos/theme.conf"
[[ -f "$CONFIG_SRC/keybindings.conf" ]]  && safe_cp "$CONFIG_SRC/keybindings.conf"  "$TARGET_DIR/sigmaos/keybindings.conf"
# Mirror to legacy path for compatibility
[[ -f "$CONFIG_SRC/shell.json" ]]        && safe_cp "$CONFIG_SRC/shell.json"        "$TARGET_DIR/omarchy/shell.json"
[[ -f "$CONFIG_SRC/theme.conf" ]]        && safe_cp "$CONFIG_SRC/theme.conf"        "$TARGET_DIR/omarchy/theme.conf"
[[ -f "$CONFIG_SRC/keybindings.conf" ]]  && safe_cp "$CONFIG_SRC/keybindings.conf"  "$TARGET_DIR/omarchy/keybindings.conf"

# 3. Hyprland Suite
echo "[+] Deploying Hyprland Wayland compositor configurations..."
for f in hyprland.conf hyprland-binds.conf hypridle.conf hyprlock.conf; do
  src="$CONFIG_SRC/hyprland/$f"
  [[ -f "$src" ]] && safe_cp "$src" "$TARGET_DIR/hypr/$f"
done

# 4. Waybar Status Bar
echo "[+] Deploying Waybar status line configurations..."
for f in config.jsonc style.css wallust-colors.css; do
  src="$CONFIG_SRC/waybar/$f"
  [[ -f "$src" ]] && safe_cp "$src" "$TARGET_DIR/waybar/$f"
done

# 5. Wallust Dynamic Palette Generator
echo "[+] Deploying Wallust dynamic palette generator..."
[[ -f "$CONFIG_SRC/wallust/wallust.toml" ]] && safe_cp "$CONFIG_SRC/wallust/wallust.toml" "$TARGET_DIR/wallust/wallust.toml"
if [[ -d "$CONFIG_SRC/wallust/templates" ]]; then
  for tmpl in "$CONFIG_SRC/wallust/templates/"*; do
    [[ -f "$tmpl" ]] && safe_cp "$tmpl" "$TARGET_DIR/wallust/templates/$(basename "$tmpl")"
  done
fi

# 6. Terminal Emulators
echo "[+] Deploying GPU-accelerated terminal configurations..."
[[ -f "$CONFIG_SRC/foot/foot.ini" ]]             && safe_cp "$CONFIG_SRC/foot/foot.ini"             "$TARGET_DIR/foot/foot.ini"
[[ -f "$CONFIG_SRC/alacritty/alacritty.toml" ]] && safe_cp "$CONFIG_SRC/alacritty/alacritty.toml" "$TARGET_DIR/alacritty/alacritty.toml"
[[ -f "$CONFIG_SRC/kitty/kitty.conf" ]]          && safe_cp "$CONFIG_SRC/kitty/kitty.conf"          "$TARGET_DIR/kitty/kitty.conf"

# 7. Launchers, Notifications & Session Power Menu
echo "[+] Deploying Rofi, Mako, Dunst, and Wlogout..."
[[ -f "$CONFIG_SRC/rofi/sigmaos.rasi" ]] && safe_cp "$CONFIG_SRC/rofi/sigmaos.rasi" "$TARGET_DIR/rofi/sigmaos.rasi"
[[ -f "$CONFIG_SRC/mako/config" ]]        && safe_cp "$CONFIG_SRC/mako/config"        "$TARGET_DIR/mako/config"
[[ -f "$CONFIG_SRC/dunst/dunstrc" ]]      && safe_cp "$CONFIG_SRC/dunst/dunstrc"      "$TARGET_DIR/dunst/dunstrc"
[[ -f "$CONFIG_SRC/wlogout/layout" ]]     && safe_cp "$CONFIG_SRC/wlogout/layout"     "$TARGET_DIR/wlogout/layout"
[[ -f "$CONFIG_SRC/wlogout/style.css" ]]  && safe_cp "$CONFIG_SRC/wlogout/style.css"  "$TARGET_DIR/wlogout/style.css"

# 8. Neovim & Starship Prompt
echo "[+] Deploying Neovim and Starship prompt configurations..."
[[ -f "$CONFIG_SRC/starship/starship.toml" ]] && safe_cp "$CONFIG_SRC/starship/starship.toml" "$TARGET_DIR/starship.toml"
[[ -f "$CONFIG_SRC/nvim/init.lua" ]]          && safe_cp "$CONFIG_SRC/nvim/init.lua"          "$TARGET_DIR/nvim/init.lua"
if [[ -d "$CONFIG_SRC/nvim/colors" ]]; then
  for f in "$CONFIG_SRC/nvim/colors/"*.lua; do
    [[ -f "$f" ]] && safe_cp "$f" "$TARGET_DIR/nvim/colors/$(basename "$f")"
  done
fi

# 9. Shells (Fish & Zsh)
echo "[+] Deploying Fish shell configurations..."
[[ -f "$CONFIG_SRC/fish/config.fish" ]] && safe_cp "$CONFIG_SRC/fish/config.fish" "$TARGET_DIR/fish/config.fish"

echo ""
echo "============================================================"
echo "  [✓] SigmaOS desktop configurations deployed successfully!"
echo "  Run './run.sh --doctor' to audit your environment."
echo "============================================================"
