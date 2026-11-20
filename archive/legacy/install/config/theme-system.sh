#!/bin/bash
# SigmaOS theme system installer
# Inspired by Omarchy 4.0.5 install/config/theme-system.sh

set -euo pipefail

THEME_NAME="${1:-sigmaos}"
THEME_DIR="/usr/share/sigmaos/themes"
TARGET_THEME_DIR="/usr/share/sigmaos/themes/${THEME_NAME}"

# Install theme templates from default/themed/.
if [ -d /usr/share/sigmaos/default/themed ]; then
    mkdir -p "${TARGET_THEME_DIR}"
    cp -a /usr/share/sigmaos/default/themed/* "${TARGET_THEME_DIR}/"
    echo "[sigmaos-theme-system] Installed theme ${THEME_NAME} templates"
fi

# Apply theme to desktop components.
apply_theme() {
    local component="$1"
    local template="$2"
    local target="$3"

    if [ -f "${TARGET_THEME_DIR}/${template}" ]; then
        mkdir -p "$(dirname "${target}")"
        cp "${TARGET_THEME_DIR}/${template}" "${target}"
        echo "[sigmaos-theme-system] Applied ${template} to ${component}"
    fi
}

# Apply to terminal configs.
apply_theme "foot" "foot.ini.tpl" "$HOME/.config/foot/foot.ini" 2>/dev/null || true
apply_theme "ghostty" "ghostty.conf.tpl" "$HOME/.config/ghostty/config" 2>/dev/null || true

# Apply to Hyprland.
apply_theme "hyprland" "hyprland.lua.tpl" "$HOME/.config/hypr/hyprland.conf" 2>/dev/null || true

echo "[sigmaos-theme-system] Theme ${THEME_NAME} applied"
