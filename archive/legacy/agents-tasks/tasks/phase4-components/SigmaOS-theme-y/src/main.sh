#!/usr/bin/env bash
# SigmaOS-theme-y installer
# Installs Y theme series (light and dark) to $HOME/.themes/
set -euo pipefail

THEMES_DIR="${HOME}/.themes"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEMES_SRC="${SCRIPT_DIR}/../themes"

install_y_themes() {
    mkdir -p "${THEMES_DIR}"
    for theme_dir in "${THEMES_SRC}"/*/; do
        theme_name="$(basename "${theme_dir}")"
        echo "Installing Y theme: ${theme_name}"
        cp -r "${theme_dir}" "${THEMES_DIR}/${theme_name}"
    done
    echo "Y theme series installed to ${THEMES_DIR}"
}

install_y_themes
