#!/usr/bin/env bash
# SigmaOS-theme-gtk3 installer
# Installs GTK3 themes to $HOME/.themes/
set -euo pipefail

THEMES_DIR="${HOME}/.themes"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEMES_SRC="${SCRIPT_DIR}/../themes"

install_gtk3_themes() {
    mkdir -p "${THEMES_DIR}"
    for theme_dir in "${THEMES_SRC}"/*/; do
        theme_name="$(basename "${theme_dir}")"
        echo "Installing GTK3 theme: ${theme_name}"
        cp -r "${theme_dir}" "${THEMES_DIR}/${theme_name}"
    done
    echo "GTK3 themes installed to ${THEMES_DIR}"
}

install_gtk3_themes
