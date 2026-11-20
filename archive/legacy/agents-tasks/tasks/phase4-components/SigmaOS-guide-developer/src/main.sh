#!/bin/bash
# SigmaOS guide-developer: developer guide.
#
# Architecture, contributing workflows, coding standards, component APIs,
# and build system documentation.

set -euo pipefail

DEST="${DESTDIR:-/usr/share/sigma}"

usage() {
    echo "usage: $(basename "$0") {install|uninstall}" >&2
    exit 1
}

install_guide() {
    echo "Installing SigmaOS developer guide to $DEST/dev-guide"
}

uninstall_guide() {
    echo "Removing SigmaOS developer guide from $DEST/dev-guide"
}

case "${1:-}" in
    install)   install_guide ;;
    uninstall) uninstall_guide ;;
    *)         usage ;;
esac
