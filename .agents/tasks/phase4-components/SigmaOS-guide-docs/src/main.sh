#!/bin/bash
# SigmaOS guide-docs: documentation hub.
#
# User guides, release notes, FAQ, and component documentation.

set -euo pipefail

DEST="${DESTDIR:-/usr/share/sigma}"

usage() {
    echo "usage: $(basename "$0") {install|uninstall}" >&2
    exit 1
}

install_docs() {
    echo "Installing SigmaOS documentation to $DEST/docs"
}

uninstall_docs() {
    echo "Removing SigmaOS documentation from $DEST/docs"
}

case "${1:-}" in
    install)   install_docs ;;
    uninstall) uninstall_docs ;;
    *)         usage ;;
esac
