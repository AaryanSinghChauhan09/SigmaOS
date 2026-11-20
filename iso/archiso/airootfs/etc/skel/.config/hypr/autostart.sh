#!/bin/bash
# SigmaOS Hyprland Autostart

# Static SigmaOS wallpaper (wallust optional theming)
if [ -f "$HOME/.config/hypr/wallpaper.png" ]; then
    hyprctl hyprpaper wallpaper "all,$HOME/.config/hypr/wallpaper.png" 2>/dev/null
fi

# Optional wallust theme if installed
command -v wallust >/dev/null 2>&1 && wallust run sigmaos >/dev/null 2>&1 &

# Clipboard + notifications (best-effort)
command -v wl-paste >/dev/null 2>&1 && wl-paste --type text --watch cliphist store >/dev/null 2>&1 &
command -v dunst >/dev/null 2>&1 && dunst >/dev/null 2>&1 &
