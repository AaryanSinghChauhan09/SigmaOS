#!/bin/bash
# sigmaos shell plugin: audio
# Modeled on Omancy 4.0.5 omancy.audio shell plugin
# Controls audio volume, mute status, and output device via pipewire/pulseaudio.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    # Show the audio panel/overlay
    if command -v pavucontrol >/dev/null 2>&1; then
      pavucontrol &
    elif command -v pamixer >/dev/null 2>&1; then
      # Toggle mute as a quick action
      pamixer -t 2>/dev/null || true
    fi
    echo "audio panel opened"
    ;;
  hide)
    # Close audio panel
    if command -v pkill >/dev/null 2>&1; then
      pkill -f pavucontrol 2>/dev/null || true
    fi
    echo "audio panel closed"
    ;;
  *)
    echo "Usage: sigmaos.audio <summon|hide>" >&2
    exit 1
    ;;
esac
