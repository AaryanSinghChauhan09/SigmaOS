#!/bin/bash
# sigmaos shell plugin: notifications
# Modeled on Omancy 4.0.5 omancy.notifications shell plugin
# Shows notification history and settings via mako.

plugin_cmd="${1:-}"
payload="${2:-}"

NOTIFICATION_HISTORY="${XDG_STATE_HOME:-$HOME/.local/state}/sigmaos/notifications.json"

case "${plugin_cmd}" in
  summon)
    mkdir -p "$(dirname "${NOTIFICATION_HISTORY}")"
    touch "${NOTIFICATION_HISTORY}" 2>/dev/null || true
    if command -v makoctl >/dev/null 2>&1; then
      makoctl list 2>/dev/null || true
    fi
    echo "notifications panel opened"
    ;;
  hide)
    echo "notifications panel closed"
    ;;
  *)
    echo "Usage: sigmaos.notifications <summon|hide>" >&2
    exit 1
    ;;
esac
