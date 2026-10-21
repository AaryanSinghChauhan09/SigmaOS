#!/bin/bash
# sigmaos shell plugin: menu
# Modeled on Omancy 4.0.5 omancy.menu shell plugin
# Opens the application launcher (wofi drun).

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    # Pass payload as menu type if specified
    if [ "${payload}" = '{"menu":"system"}' ]; then
      # System menu - show power options via wofi
      if command -v wofi >/dev/null 2>&1; then
        echo -e "Shutdown\nReboot\nSuspend\nLogout\nLock" | wofi --show dmenu 2>/dev/null || true
      fi
    else
      # Regular app launcher
      if command -v wofi >/dev/null 2>&1; then
        wofi --show drun 2>/dev/null || true
      fi
    fi
    echo "menu opened"
    ;;
  hide)
    echo "menu closed"
    ;;
  *)
    echo "Usage: sigmaos.menu <summon|hide>" >&2
    exit 1
    ;;
esac
