#!/bin/bash
# sigmaos shell plugin: settings
# Opens SigmaOS settings panel for configuring bar position, theme, etc.

plugin_cmd="${1:-}"
payload="${2:-}"

case "${plugin_cmd}" in
  summon)
    if command -v wofi >/dev/null 2>&1; then
      selection=$(echo -e "Bar Position: Top\nTheme: sigmaos\nBackground: default\nFont: JetBrainsMono Nerd Font\nBack" | wofi --show dmenu --prompt "Settings" 2>/dev/null || true)
      case "${selection}" in
        "Bar Position: Top")
          notify-send "SigmaOS Settings" "Bar position set to top" 2>/dev/null || true
          ;;
        "Theme: sigmaos")
          sigmaos-theme-system sigmaos 2>/dev/null || true
          ;;
        "Background: default")
          sigmaos-theme-bg-switcher next 2>/dev/null || true
          ;;
        "Back")
          sigmaos-shell shell summon sigmaos.menu '{"menu":"root"}' 2>/dev/null || true
          ;;
      esac
    fi
    echo "settings panel opened"
    ;;
  hide)
    echo "settings panel closed"
    ;;
  *)
    echo "Usage: sigmaos.settings <summon|hide>" >&2
    exit 1
    ;;
esac
