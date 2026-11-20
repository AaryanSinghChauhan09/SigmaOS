#!/bin/bash
# sigmaos shell plugin: network
# Modeled on Omancy 4.0.5 omancy.network shell plugin
# Manages network connections via NetworkManager/nmcli.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    if command -v nmcli >/dev/null 2>&1; then
      nmcli device wifi list 2>/dev/null || true
      nmcli connection show 2>/dev/null || true
    fi
    if command -v nm-connection-editor >/dev/null 2>&1; then
      nm-connection-editor &
    fi
    echo "network panel opened"
    ;;
  hide)
    if command -v pkill >/dev/null 2>&1; then
      pkill -f nm-connection-editor 2>/dev/null || true
    fi
    echo "network panel closed"
    ;;
  *)
    echo "Usage: sigmaos.network <summon|hide>" >&2
    exit 1
    ;;
esac
