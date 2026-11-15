#!/usr/bin/env bash
# ==============================================================================
# SigmaOS — Workstation & Omarchy Desktop Installer
# Turnkey installer: deploys binaries, dotfiles, and desktop entries.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DEST="${HOME}/.local/bin"
CONFIG_DEST="${HOME}/.config/sigmaos"
APPS_DEST="${HOME}/.local/share/applications"

echo "╔══════════════════════════════════════════════════════════╗"
echo "║          Installing SigmaOS Sovereign Workstation        ║"
echo "╚══════════════════════════════════════════════════════════╝"

# 1. Build binary
echo "--> Compiling SigmaOS workstation binary..."
mkdir -p "$SCRIPT_DIR/bin"
rustc -O "$SCRIPT_DIR/src/bin/sigmaos.rs" -o "$SCRIPT_DIR/bin/sigmaos"

# 2. Install to PATH
echo "--> Installing binaries to $BIN_DEST..."
mkdir -p "$BIN_DEST"
cp -f "$SCRIPT_DIR/bin/sigmaos" "$BIN_DEST/sigmaos"
chmod +x "$BIN_DEST/sigmaos"

# Symlink omarchy CLI alias
ln -sf "$BIN_DEST/sigmaos" "$BIN_DEST/sigmaos-omarchy"

# 3. Deploy default configurations
echo "--> Deploying Omarchy configuration dotfiles to $CONFIG_DEST..."
mkdir -p "$CONFIG_DEST/omarchy/hyprland"
mkdir -p "$CONFIG_DEST/omarchy/waybar"
mkdir -p "$CONFIG_DEST/omarchy/wallust"

cat <<'EOF' > "$CONFIG_DEST/omarchy/hyprland/hyprland.conf"
# SigmaOS Omarchy Linux Aesthetic Persona — Hyprland Configuration
monitor=,preferred,auto,1
input {
    kb_layout = us
    follow_mouse = 1
}
general {
    gaps_in = 6
    gaps_out = 12
    border_size = 2
    col.active_border = rgba(33ccffee) rgba(00ff99ee) 45deg
    col.inactive_border = rgba(595959aa)
    layout = dwindle
}
decoration {
    rounding = 8
    blur {
        enabled = true
        size = 4
        passes = 2
    }
}
bind = SUPER, Return, exec, alacritty || kitty || xterm
bind = SUPER, Q, killactive,
bind = SUPER, M, exit,
bind = SUPER, D, exec, wofi --show drun
EOF

cat <<'EOF' > "$CONFIG_DEST/omarchy/waybar/config"
{
    "layer": "top",
    "position": "top",
    "height": 32,
    "modules-left": ["hyprland/workspaces", "hyprland/window"],
    "modules-center": ["clock"],
    "modules-right": ["pulseaudio", "network", "cpu", "memory", "tray"]
}
EOF

# 4. Install Desktop Entry
echo "--> Installing Desktop Entry to $APPS_DEST..."
mkdir -p "$APPS_DEST"
cat <<EOF > "$APPS_DEST/sigmaos.desktop"
[Desktop Entry]
Name=SigmaOS Workstation
Comment=Sovereign Operating System Workstation (Omarchy / Arch Linux)
Exec=$BIN_DEST/sigmaos
Icon=utilities-terminal
Terminal=true
Type=Application
Categories=System;Utility;
Keywords=SigmaOS;Arch;Omarchy;Hyprland;Workstation;
EOF

echo ""
echo -e "\033[32m[✓] SigmaOS successfully installed!\033[0m"
echo ""
echo "You can now run:"
echo "  sigmaos            # Interactive Workstation"
echo "  sigmaos --omarchy  # Omarchy Aesthetic Persona"
echo "  sigmaos --arch     # Arch Linux Minimalist Persona"
echo "  sigmaos --doctor   # System Health Diagnostics"
echo ""
if [[ ":$PATH:" != *":$BIN_DEST:"* ]]; then
    echo "Notice: Make sure $BIN_DEST is in your PATH, e.g.:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi
