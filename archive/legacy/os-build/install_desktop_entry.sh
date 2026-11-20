#!/usr/bin/env bash
# SigmaOS Desktop Entry Installer
# Installs the .desktop launcher so SigmaOS appears in the app menu
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
APPS_DIR="${HOME}/.local/share/applications"
ICONS_DIR="${HOME}/.local/share/icons/hicolor/scalable/apps"

mkdir -p "$APPS_DIR" "$ICONS_DIR"

# Copy icon
cp "$ROOT_DIR/assets/icons/sigmaos.svg" "$ICONS_DIR/sigmaos.svg"
echo "✓ Icon installed: $ICONS_DIR/sigmaos.svg"

# Write .desktop entry with correct paths
cat > "$APPS_DIR/sigmaos.desktop" << DESKTOP
[Desktop Entry]
Version=1.0
Type=Application
Name=SigmaOS
GenericName=Sovereign Operating System
Comment=Arch Linux ALPM substrate with Omarchy desktop aesthetic — built in Rust
Exec=python3 $ROOT_DIR/scripts/sigmaos_gui.py
Icon=$ICONS_DIR/sigmaos.svg
Terminal=false
Categories=System;
Keywords=arch;linux;os;pacman;omarchy;hyprland;catppuccin;rust;
StartupNotify=true
StartupWMClass=SigmaOS
Actions=REPL;Arch;Omarchy;Doctor;Diagnostics;

[Desktop Action REPL]
Name=Open SigmaOS REPL
Exec=gnome-terminal -- $ROOT_DIR/bin/sigmaos

[Desktop Action Arch]
Name=Arch Persona
Exec=gnome-terminal -- bash $ROOT_DIR/scripts/launch_arch_persona.sh

[Desktop Action Omarchy]
Name=Omarchy Persona
Exec=gnome-terminal -- bash $ROOT_DIR/scripts/launch_omarchy_persona.sh

[Desktop Action Doctor]
Name=Run Health Diagnostics
Exec=gnome-terminal -- python3 $ROOT_DIR/scripts/sigmaos_startup_diagnostics.py

[Desktop Action Diagnostics]
Name=Full Startup Diagnostics
Exec=gnome-terminal -- bash -c "python3 $ROOT_DIR/scripts/sigmaos_startup_diagnostics.py; read -p 'Press Enter to close...'"
DESKTOP

echo "✓ Desktop entry installed: $APPS_DIR/sigmaos.desktop"

# Update icon cache if gtk-update-icon-cache is available
if command -v gtk-update-icon-cache &>/dev/null; then
    gtk-update-icon-cache -f -t "${HOME}/.local/share/icons/hicolor" 2>/dev/null && echo "✓ Icon cache updated"
fi

# Update desktop database
if command -v update-desktop-database &>/dev/null; then
    update-desktop-database "$APPS_DIR" 2>/dev/null && echo "✓ Desktop database updated"
fi

echo ""
echo "✓ SigmaOS is now available in your application menu!"
echo "  Right-click icon for: REPL · Arch · Omarchy · Diagnostics"
