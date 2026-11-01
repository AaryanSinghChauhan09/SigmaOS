#!/usr/bin/env bash
# SigmaOS-theme-desktop-environment installer
# Installs DE themes to $HOME/.themes/
set -euo pipefail

THEMES_DIR="${HOME}/.themes"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEMES_SRC="${SCRIPT_DIR}/../themes"

install_de_themes() {
    mkdir -p "${THEMES_DIR}"
    for theme_dir in "${THEMES_SRC}"/*/; do
        theme_name="$(basename "${theme_dir}")"
        echo "Installing DE theme: ${theme_name}"
        cp -r "${theme_dir}" "${THEMES_DIR}/${theme_name}"
    done
    echo "Desktop environment themes installed to ${THEMES_DIR}"
}

install_de_themes
