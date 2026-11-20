#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# ----------------------------------------------------------------------------
# setup_omarchy_dotfiles.sh: Deploy Omarchy Linux desktop configs to ~/.config
# Idempotent — safe to run multiple times. Uses cp -n (no-clobber) by default
# and --force flag to overwrite existing files.
# ----------------------------------------------------------------------------

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SIGMA_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_SRC="$SIGMA_ROOT/config/omarchy"
TARGET_DIR="${XDG_CONFIG_HOME:-$HOME/.config}"
FORCE=false

for arg in "$@"; do
  [[ "$arg" == "--force" ]] && FORCE=true
done

# ── Copy helper (honours idempotency) ────────────────────────────────────────
safe_cp() {
  local src="$1" dst="$2"
  if [[ "$FORCE" == true ]]; then
    cp "$src" "$dst"
  else
    cp -n "$src" "$dst" 2>/dev/null || true
  fi
}

echo "============================================================"
echo "  Deploying Omarchy Linux Desktop Configurations to ~/.config"
echo "============================================================"
echo "  Source : $CONFIG_SRC"
echo "  Target : $TARGET_DIR"
echo "  Force  : $FORCE"
echo ""

# ── Ensure base directories exist ────────────────────────────────────────────
mkdir -p "$TARGET_DIR"/{hypr,waybar,wallust,rofi,mako,dunst,foot,alacritty,kitty,wlogout,nvim/colors,fish,omarchy/plugins,starship}

# ── 0. SigmaOS Omarchy persona metadata ──────────────────────────────────────
echo "[+] Ensuring ~/.config/omarchy/ structure..."
mkdir -p "$TARGET_DIR/omarchy/plugins"

# shell.json — SigmaOS persona descriptor
SHELL_JSON_DST="$TARGET_DIR/omarchy/shell.json"
if [[ ! -f "$SHELL_JSON_DST" ]] || [[ "$FORCE" == true ]]; then
  cp "$CONFIG_SRC/shell.json" "$SHELL_JSON_DST"
  echo "    wrote $SHELL_JSON_DST"
else
  echo "    skipped $SHELL_JSON_DST (exists; use --force to overwrite)"
fi

# theme.conf — Catppuccin Mocha defaults
THEME_DST="$TARGET_DIR/omarchy/theme.conf"
if [[ ! -f "$THEME_DST" ]] || [[ "$FORCE" == true ]]; then
  cp "$CONFIG_SRC/theme.conf" "$THEME_DST"
  echo "    wrote $THEME_DST"
else
  echo "    skipped $THEME_DST (exists; use --force to overwrite)"
fi

# keybindings.conf — reference doc
KB_DST="$TARGET_DIR/omarchy/keybindings.conf"
if [[ ! -f "$KB_DST" ]] || [[ "$FORCE" == true ]]; then
  cp "$CONFIG_SRC/keybindings.conf" "$KB_DST"
  echo "    wrote $KB_DST"
else
  echo "    skipped $KB_DST (exists)"
fi

# ── 1. Hyprland suite ────────────────────────────────────────────────────────
echo "[+] Deploying Hyprland, Hypridle, Hyprlock..."
for f in hyprland.conf hyprland-binds.conf hypridle.conf hyprlock.conf; do
  src="$CONFIG_SRC/hyprland/$f"
  [[ -f "$src" ]] && safe_cp "$src" "$TARGET_DIR/hypr/$f" || true
done

# ── 2. Waybar status bar ─────────────────────────────────────────────────────
echo "[+] Deploying Waybar..."
for f in config.jsonc style.css wallust-colors.css; do
  src="$CONFIG_SRC/waybar/$f"
  [[ -f "$src" ]] && safe_cp "$src" "$TARGET_DIR/waybar/$f" || true
done

# ── 3. Wallust dynamic palette generator ─────────────────────────────────────
echo "[+] Deploying Wallust..."
if [[ -f "$CONFIG_SRC/wallust/wallust.toml" ]]; then
  safe_cp "$CONFIG_SRC/wallust/wallust.toml" "$TARGET_DIR/wallust/wallust.toml"
fi
if [[ -d "$CONFIG_SRC/wallust/templates" ]]; then
  mkdir -p "$TARGET_DIR/wallust/templates"
  for tmpl in "$CONFIG_SRC/wallust/templates/"*; do
    [[ -f "$tmpl" ]] && safe_cp "$tmpl" "$TARGET_DIR/wallust/templates/$(basename "$tmpl")" || true
  done
fi

# ── 4. Terminals (Foot, Alacritty, Kitty) ────────────────────────────────────
echo "[+] Deploying Terminals (Foot, Alacritty, Kitty)..."
[[ -f "$CONFIG_SRC/foot/foot.ini" ]]               && safe_cp "$CONFIG_SRC/foot/foot.ini"             "$TARGET_DIR/foot/foot.ini"
[[ -f "$CONFIG_SRC/alacritty/alacritty.toml" ]]   && safe_cp "$CONFIG_SRC/alacritty/alacritty.toml"  "$TARGET_DIR/alacritty/alacritty.toml"
[[ -f "$CONFIG_SRC/kitty/kitty.conf" ]]            && safe_cp "$CONFIG_SRC/kitty/kitty.conf"           "$TARGET_DIR/kitty/kitty.conf"

# ── 5. Application Launchers & Notifications ─────────────────────────────────
echo "[+] Deploying Rofi, Mako, Dunst..."
[[ -f "$CONFIG_SRC/rofi/sigmaos.rasi" ]]  && safe_cp "$CONFIG_SRC/rofi/sigmaos.rasi"  "$TARGET_DIR/rofi/sigmaos.rasi"
[[ -f "$CONFIG_SRC/mako/config" ]]         && safe_cp "$CONFIG_SRC/mako/config"         "$TARGET_DIR/mako/config"
[[ -f "$CONFIG_SRC/dunst/dunstrc" ]]       && safe_cp "$CONFIG_SRC/dunst/dunstrc"       "$TARGET_DIR/dunst/dunstrc"

# ── 6. Wlogout & Starship ────────────────────────────────────────────────────
echo "[+] Deploying Wlogout & Starship..."
[[ -f "$CONFIG_SRC/wlogout/layout" ]]    && safe_cp "$CONFIG_SRC/wlogout/layout"    "$TARGET_DIR/wlogout/layout"
[[ -f "$CONFIG_SRC/wlogout/style.css" ]] && safe_cp "$CONFIG_SRC/wlogout/style.css" "$TARGET_DIR/wlogout/style.css"
[[ -f "$CONFIG_SRC/starship/starship.toml" ]] && safe_cp "$CONFIG_SRC/starship/starship.toml" "$TARGET_DIR/starship.toml"

# ── 7. Neovim & Colorschemes ─────────────────────────────────────────────────
echo "[+] Deploying Neovim configuration..."
mkdir -p "$TARGET_DIR/nvim/lua/plugins"
[[ -f "$CONFIG_SRC/nvim/init.lua" ]] && safe_cp "$CONFIG_SRC/nvim/init.lua" "$TARGET_DIR/nvim/init.lua"
if [[ -d "$CONFIG_SRC/nvim/colors" ]]; then
  for f in "$CONFIG_SRC/nvim/colors/"*.lua; do
    [[ -f "$f" ]] && safe_cp "$f" "$TARGET_DIR/nvim/colors/$(basename "$f")" || true
  done
fi
if [[ -d "$CONFIG_SRC/nvim/lua/plugins" ]]; then
  for f in "$CONFIG_SRC/nvim/lua/plugins/"*.lua; do
    [[ -f "$f" ]] && safe_cp "$f" "$TARGET_DIR/nvim/lua/plugins/$(basename "$f")" || true
  done
fi

# ── 8. Shell Environments (Zsh & Fish) ───────────────────────────────────────
echo "[+] Deploying Zsh & Fish shells..."
[[ -f "$CONFIG_SRC/fish/config.fish" ]] && safe_cp "$CONFIG_SRC/fish/config.fish" "$TARGET_DIR/fish/config.fish"
if [[ -f "$CONFIG_SRC/zsh/.zshrc" ]]; then
  if [[ "$FORCE" == true ]] || [[ ! -f "$HOME/.zshrc" ]]; then
    cp "$CONFIG_SRC/zsh/.zshrc" "$HOME/.zshrc"
    echo "    wrote ~/.zshrc"
  else
    echo "    skipped ~/.zshrc (exists; use --force)"
  fi
fi

# ── 9. Omarchy Plugins ───────────────────────────────────────────────────────
echo "[+] Deploying Omarchy Plugins..."
if [[ -d "$CONFIG_SRC/plugins" ]]; then
  for f in "$CONFIG_SRC/plugins/"*; do
    [[ -f "$f" ]] && safe_cp "$f" "$TARGET_DIR/omarchy/plugins/$(basename "$f")" || true
  done
fi

echo ""
echo "============================================================"
echo "  [✓] Omarchy configurations deployed!"
echo "  Run 'bash scripts/sigmaos_doctor.sh' to verify your setup."
echo "============================================================"
