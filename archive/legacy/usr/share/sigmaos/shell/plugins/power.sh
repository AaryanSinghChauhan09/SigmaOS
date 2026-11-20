#!/bin/bash
# sigmaos shell plugin: power
# Modeled on Omancy 4.0.5 omancy.power shell plugin
# Manages power profiles and battery status via powerprofilesctl.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    if command -v powerprofilesctl >/dev/null 2>&1; then
      powerprofilesctl list 2>/dev/null || true
    fi
    if command -v cat >/dev/null 2>&1; then
      for bat in /sys/class/power_supply/BAT*; do
        [ -d "${bat}" ] || continue
        echo "Battery: $(cat ${bat}/capacity 2>/dev/null)% $(cat ${bat}/status 2>/dev/null)"
      done
    fi
    echo "power panel opened"
    ;;
  hide)
    echo "power panel closed"
    ;;
  *)
    echo "Usage: sigmaos.power <summon|hide>" >&2
    exit 1
    ;;
esac
