#!/bin/bash
# SigmaOS forum-themes: forum and community website themes.
#
# Provides CSS stylesheets and template assets matching the SigmaOS brand.

set -euo pipefail

DEST="${DESTDIR:-/usr/share/sigma}"

usage() {
    echo "usage: $(basename "$0") {install|uninstall}" >&2
    exit 1
}

install_themes() {
    echo "Installing SigmaOS forum themes to $DEST/forum-themes"
}

uninstall_themes() {
    echo "Removing SigmaOS forum themes from $DEST/forum-themes"
}

case "${1:-}" in
    install)   install_themes ;;
    uninstall) uninstall_themes ;;
    *)         usage ;;
esac
