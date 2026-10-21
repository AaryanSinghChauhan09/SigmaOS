#!/bin/bash
# sigmaos shell plugin: background
# Modeled on Omancy 4.0.5 omancy.background shell plugin
# Controls wallpaper and background settings via swww.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    # Open background selector
    if command -v swww >/dev/null 2>&1; then
      # List available wallpapers
      local wallpaper_dir="${HOME}/Pictures/Wallpapers"
      if [ -d "${wallpaper_dir}" ]; then
        ls "${wallpaper_dir}" 2>/dev/null || true
      fi
    fi
    echo "background panel opened"
    ;;
  hide)
    echo "background panel closed"
    ;;
  *)
    echo "Usage: sigmaos.background <summon|hide>" >&2
    exit 1
    ;;
esac
