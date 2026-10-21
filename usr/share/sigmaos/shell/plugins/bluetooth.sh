#!/bin/bash
# sigmaos shell plugin: bluetooth
# Modeled on Omancy 4.0.5 omancy.bluetooth shell plugin
# Manages bluetooth devices via bluetoothctl/blueman.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    if command -v blueman-manager >/dev/null 2>&1; then
      blueman-manager &
    elif command -v bluetoothctl >/dev/null 2>&1; then
      echo "Powered on" | bluetoothctl 2>/dev/null || true
      echo "Discoverable on" | bluetoothctl 2>/dev/null || true
    fi
    echo "bluetooth panel opened"
    ;;
  hide)
    if command -v pkill >/dev/null 2>&1; then
      pkill -f blueman-manager 2>/dev/null || true
    fi
    echo "bluetooth panel closed"
    ;;
  *)
    echo "Usage: sigmaos.bluetooth <summon|hide>" >&2
    exit 1
    ;;
esac
