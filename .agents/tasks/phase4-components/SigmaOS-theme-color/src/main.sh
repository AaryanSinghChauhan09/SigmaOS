#!/usr/bin/env bash
# SigmaOS-theme-color: install color theme definitions
set -euo pipefail

INSTALL_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/sigma/colors"
mkdir -p "$INSTALL_DIR"
cp -r ./colors/* "$INSTALL_DIR/"
echo "SigmaOS color themes installed to $INSTALL_DIR"
