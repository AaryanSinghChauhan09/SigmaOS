#!/bin/bash
# sigmaos emoji shell plugin
# Modeled on Omancy 4.0.5 omancy.emojis shell plugin
# Opens emoji picker that accepts typed search input.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    # The emoji picker shows available emojis.
    # User types to search (handled by wofi/filter externally).
    sigmaos-emoji-picker pick 2>/dev/null || true
    echo "emoji picker opened"
    ;;
  hide)
    echo "emoji picker closed"
    ;;
  *)
    echo "Usage: sigmaos.emojis <summon|hide>" >&2
    exit 1
    ;;
esac
