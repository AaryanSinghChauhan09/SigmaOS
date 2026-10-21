#!/bin/bash
# sigmaos shell plugin: clipboard
# Modeled on Omancy 4.0.5 omancy.clipboard shell plugin
# Manages clipboard history stored in XDG state directory.

plugin_cmd="${1:-}"
payload="${2:-}"

CLIPBOARD_HISTORY="${XDG_STATE_HOME:-$HOME/.local/state}/sigmaos/clipboard-history.json"

case "${plugin_cmd}" in
  summon)
    mkdir -p "$(dirname "${CLIPBOARD_HISTORY}")"
    touch "${CLIPBOARD_HISTORY}" 2>/dev/null || true
    # Show clipboard history via wofi or fzf
    if command -v wofi >/dev/null 2>&1 && [ -s "${CLIPBOARD_HISTORY}" ]; then
      wofi --show dmenu < "${CLIPBOARD_HISTORY}" 2>/dev/null || true
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
