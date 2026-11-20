#!/bin/bash
# sigmaos shell plugin: bar
# Modeled on Omancy 4.0.5 omancy.bar shell plugin
# Controls waybar visibility and configuration.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    # Start/restart waybar
    if command -v waybar >/dev/null 2>&1; then
      pkill waybar 2>/dev/null || true
      sleep 0.5
      waybar &
    fi
    echo "bar opened"
    ;;
  hide)
    # Hide bar (signal waybar)
    if command -v pkill >/dev/null 2>&1; then
      pkill -USR1 waybar 2>/dev/null || true
    fi
    echo "bar closed"
    ;;
  *)
    echo "Usage: sigmaos.bar <summon|hide>" >&2
    exit 1
    ;;
esac
