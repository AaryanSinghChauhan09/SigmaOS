#!/bin/bash
# SigmaOS chromium: Chromium browser integration.
#
# Installs SigmaOS-specific preferences, policies, and launch wrappers.

set -euo pipefail

DEST="${DESTDIR:-/usr/lib/sigma}"

usage() {
    echo "usage: $(basename "$0") {install|uninstall}" >&2
    exit 1
}

install_config() {
    echo "Installing SigmaOS Chromium configuration to $DEST/chromium"
    # Real implementation drops policies, master_preferences, and wrapper.
}

uninstall_config() {
    echo "Removing SigmaOS Chromium configuration from $DEST/chromium"
}

case "${1:-}" in
    install)   install_config ;;
    uninstall) uninstall_config ;;
    *)         usage ;;
esac
