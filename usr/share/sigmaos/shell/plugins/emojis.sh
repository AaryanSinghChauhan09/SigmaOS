#!/bin/bash
# sigmaos shell plugin: emojis
# Modeled on Omancy 4.0.5 omancy.emojis shell plugin
# Opens an emoji picker via wofi or similar.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    if command -v wofi >/dev/null 2>&1; then
      # Simple emoji list
      echo -e "😀\n😃\n😄\n😁\n😆\n😅\n😂\n🤣\n😊\n😇" | wofi --show dmenu 2>/dev/null || true
    fi
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
