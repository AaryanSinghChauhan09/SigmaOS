#!/usr/bin/env bash
# SigmaOS Wallpaper Engine — animated and static wallpaper management.
set -euo pipefail
WALLPAPER_DIR="${SIGMA_WALLPAPER_DIR:-/usr/share/sigma/wallpapers}"
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command> [options]
Commands:
  list                 List available wallpapers
  set <path|name>      Set wallpaper (static)
  slideshow <dir>      Start slideshow from directory
  live <name>          Start animated/live wallpaper
  status               Show current wallpaper
EOF
exit 1; }
cmd_list() { echo "Available wallpapers in $WALLPAPER_DIR:"; ls "$WALLPAPER_DIR" 2>/dev/null || for w in sigma-dark.jpg sigma-gradient.png sigma-mountain.jpg arch-linux.png; do echo "  $w"; done; }
cmd_set() { local p="${1:-}"; echo "[wallpaper] Setting wallpaper: $p"; echo "  (Real impl: swaybg -i $p -m fill  OR  swww img $p --transition-type fade)"; }
cmd_slideshow() { local dir="${1:-$WALLPAPER_DIR}"; echo "[wallpaper] Starting slideshow from $dir (30s interval)"; echo "  (Real impl: while true; do sigma-wallpaper set $(ls $dir|shuf|head -1); sleep 30; done)"; }
cmd_live() { local name="${1:-}"; echo "[wallpaper] Starting live wallpaper: $name"; echo "  (Real impl: mpvpaper eDP-1 $WALLPAPER_DIR/$name.mp4 -o 'loop')"; }
cmd_status() { echo "Current wallpaper: $(cat /tmp/sigma-wallpaper 2>/dev/null || echo sigma-dark.jpg)"; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in list) cmd_list;; set) cmd_set "$@";; slideshow) cmd_slideshow "$@";; live) cmd_live "$@";; status) cmd_status;; *) usage;; esac; }
main "$@"
