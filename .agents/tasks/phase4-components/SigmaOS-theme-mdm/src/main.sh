#!/usr/bin/env bash
# SigmaOS-theme-mdm installer
# Installs MDM themes to /usr/share/mdm/themes/ (requires sudo)
set -euo pipefail

MDM_THEMES_DIR="/usr/share/mdm/themes"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THEMES_SRC="${SCRIPT_DIR}/../themes"

check_sudo() {
    if [[ "${EUID}" -ne 0 ]]; then
        echo "Error: MDM theme installation requires root privileges." >&2
        echo "Please re-run with sudo: sudo $0" >&2
        exit 1
    fi
}

install_mdm_themes() {
    check_sudo
    mkdir -p "${MDM_THEMES_DIR}"
    for theme_dir in "${THEMES_SRC}"/*/; do
        theme_name="$(basename "${theme_dir}")"
        echo "Installing MDM theme: ${theme_name}"
        cp -r "${theme_dir}" "${MDM_THEMES_DIR}/${theme_name}"
    done
    echo "MDM themes installed to ${MDM_THEMES_DIR}"
}

install_mdm_themes
