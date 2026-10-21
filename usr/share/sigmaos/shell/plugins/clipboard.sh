#!/bin/bash
# sigmaos clipboard plugin - clipboard history viewer
# Modeled on Omancy 4.0.5 omancy.clipboard shell plugin
#
# Shows clipboard history from JSON file and allows selecting entries
# to copy back to the clipboard.

set -euo pipefail

CLIPBOARD_JSON="${XDG_STATE_HOME:-$HOME/.local/state}/sigmaos/clipboard-history.json"

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    mkdir -p "$(dirname "${CLIPBOARD_JSON}")"
    touch "${CLIPBOARD_JSON}" 2>/dev/null || true

    if command -v wofi >/dev/null 2>&1 && [ -s "${CLIPBOARD_JSON}" ]; then
      # Extract clipboard entries from JSON and show in wofi
      selection=$(grep -o '"content": "[^"]*"' "${CLIPBOARD_JSON}" 2>/dev/null | sed 's/"content": //; s/^"//; s/"$//' | wofi --show dmenu --prompt "Clipboard" 2>/dev/null || true)
      if [ -n "${selection}" ]; then
        if command -v wl-copy >/dev/null 2>&1; then
          echo -n "${selection}" | wl-copy
        fi
        notify-send "SigmaOS Clipboard" "Copied: ${selection}" 2>/dev/null || true
      fi
    else
      echo "Clipboard history:"
      if [ -s "${CLIPBOARD_JSON}" ]; then
        grep -o '"content": "[^"]*"' "${CLIPBOARD_JSON}" 2>/dev/null | sed 's/"content": //; s/^"//; s/"$//' | head -20
      else
        echo "(empty)"
      fi
    fi
    echo "clipboard panel opened"
    ;;
  hide)
    echo "clipboard panel closed"
    ;;
  *)
    echo "Usage: sigmaos.clipboard <summon|hide>" >&2
    exit 1
    ;;
esac
