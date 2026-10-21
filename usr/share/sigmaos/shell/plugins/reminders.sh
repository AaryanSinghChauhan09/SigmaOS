#!/bin/bash
# sigmaos shell plugin: reminders
# Modeled on Omancy 4.0.5 omancy.reminders shell plugin
# Manages reminders stored in XDG runtime directory.

plugin_cmd="${1:-}"
payload="${2:-}"

REMINDER_DIR="${XDG_RUNTIME_DIR:-/tmp}/sigmaos-reminders"

case "${plugin_cmd}" in
  summon)
    mkdir -p "${REMINDER_DIR}"
    # List active reminders
    for timer in "${REMINDER_DIR}"/*.timer; do
      [ -f "${timer}" ] || continue
      echo "Reminder due at $(cat "${timer}" 2>/dev/null): $(cat "${timer%.timer}.message" 2>/dev/null)"
    done
    echo "reminders panel opened"
    ;;
  hide)
    echo "reminders panel closed"
    ;;
  *)
    echo "Usage: sigmaos.reminders <summon|hide>" >&2
    exit 1
    ;;
esac
