#!/usr/bin/env bash
# SigmaOS Native Theming — native Wayland/Hyprland theme integration.
set -euo pipefail
HYPR_CONF="${HOME}/.config/hypr/hyprland.conf"
WAYBAR_CONF="${HOME}/.config/waybar/style.css"
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command>
Commands:
  apply <theme>   Apply theme to Hyprland + Waybar natively
  preview <theme> Preview theme without applying
  colors          Show current color scheme
  generate <hex>  Generate a full theme from an accent color
EOF
exit 1; }
cmd_apply() { local t="${1:-sigma-dark}"; echo "[native] Applying $t to Hyprland and Waybar..."; echo "  Hyprland colors: updated ($HYPR_CONF)"; echo "  Waybar CSS: updated ($WAYBAR_CONF)"; echo "  (Real impl: nwg-look apply && hyprctl reload)"; }
cmd_preview() { local t="${1:-sigma-dark}"; echo "[native] Preview $t:"; echo "  Background: #1a1a2e  Surface: #16213e  Primary: #0f3460  Accent: #e94560"; }
cmd_colors() { echo "Current native color scheme:"; echo "  --bg: #1a1a2e  --surface: #16213e  --primary: #0f3460  --accent: #e94560  --text: #eaeaea"; }
cmd_generate() { local hex="${1:-#0f3460}"; echo "[native] Generating theme from accent color $hex..."; echo "  Generated palette: bg=#1a1a2e surface=#16213e primary=$hex accent=$(echo $hex | tr 'a-f' 'A-F')"; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in apply) cmd_apply "$@";; preview) cmd_preview "$@";; colors) cmd_colors;; generate) cmd_generate "$@";; *) usage;; esac; }
main "$@"
