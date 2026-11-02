#!/bin/bash
# SigmaOS github-action: reusable GitHub Actions.
#
# Provides build matrix templates, release automation, and cross-component
# CI helpers.

set -euo pipefail

DEST="${DESTDIR:-/usr/share/sigma}"

usage() {
    echo "usage: $(basename "$0") {install|uninstall}" >&2
    exit 1
}

install_actions() {
    echo "Installing SigmaOS GitHub Actions to $DEST/github-actions"
}

uninstall_actions() {
    echo "Removing SigmaOS GitHub Actions from $DEST/github-actions"
}

case "${1:-}" in
    install)   install_actions ;;
    uninstall) uninstall_actions ;;
    *)         usage ;;
esac
