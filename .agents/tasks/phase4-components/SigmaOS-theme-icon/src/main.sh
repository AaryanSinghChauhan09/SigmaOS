#!/usr/bin/env bash
# SigmaOS-theme-icon installer
# Installs icon themes to $HOME/.local/share/icons/
set -euo pipefail

ICONS_DIR="${HOME}/.local/share/icons"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ICONS_SRC="${SCRIPT_DIR}/../icons"

install_icon_themes() {
    mkdir -p "${ICONS_DIR}"
    for icon_dir in "${ICONS_SRC}"/*/; do
        icon_name="$(basename "${icon_dir}")"
        echo "Installing icon theme: ${icon_name}"
        cp -r "${icon_dir}" "${ICONS_DIR}/${icon_name}"
    done
    echo "Icon themes installed to ${ICONS_DIR}"
}

install_icon_themes
