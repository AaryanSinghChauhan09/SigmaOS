#!/usr/bin/env bash
# ==============================================================================
# Launch SigmaOS Interactive Live Session in a dedicated visible terminal window
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

export DISPLAY="${DISPLAY:-:0}"

if command -v gnome-terminal >/dev/null 2>&1; then
    exec gnome-terminal \
        --geometry=110x32 \
        --title="SigmaOS Sovereign Workstation (Arch & Omarchy)" \
        -- bash -c "$SCRIPT_DIR/sigmaos_interactive_shell.sh"
else
    exec "$SCRIPT_DIR/sigmaos_interactive_shell.sh"
fi
