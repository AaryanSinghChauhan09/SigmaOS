#!/usr/bin/env bash
# SigmaOS-theme-cursor: install cursor themes
set -euo pipefail

INSTALL_DIR="$HOME/.local/share/icons"
mkdir -p "$INSTALL_DIR"
cp -r ./cursors/* "$INSTALL_DIR/"
echo "SigmaOS cursor themes installed to $INSTALL_DIR"
