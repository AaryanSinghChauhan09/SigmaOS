#!/usr/bin/env bash
# ==============================================================================
# SigmaOS Live Desktop Environment Launcher
# Opens the native SigmaOS Sovereign Desktop Window on current display (:0)
# Uses GTK 3.0 + WebKit2 4.1 hardware accelerated engine with Chrome/Firefox app fallback.
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "\033[1;36m==> Starting SigmaOS Desktop Experience...\033[0m"

# Export standard environment
export DISPLAY="${DISPLAY:-:0}"
export PATH="${HOME}/.local/bin:${PATH}"

# Launch the graphical desktop directly
exec python3 "$ROOT_DIR/scripts/sigmaos_gui.py"
