#!/bin/bash
# sigmaos shell plugin: monitor
# Modeled on Omancy 4.0.5 omancy.monitor shell plugin
# Manages display outputs and monitor configuration via hyprctl.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    if command -v hyprctl >/dev/null 2>&1; then
      hyprctl monitors 2>/dev/null || true
    fi
    echo "monitor panel opened"
    ;;
  hide)
    echo "monitor panel closed"
    ;;
  *)
    echo "Usage: sigmaos.monitor <summon|hide>" >&2
    exit 1
    ;;
esac
