#!/bin/bash
# SigmaOS Hyprland Autostart

# Static SigmaOS wallpaper (wallust optional theming). hyprpaper runs as a
# daemon — exec-once may not have started it yet when this script runs, so
# ensure it is up before issuing hyprctl commands.
if command -v hyprpaper >/dev/null 2>&1; then
    pgrep -x hyprpaper >/dev/null 2>&1 || hyprpaper >/dev/null 2>&1 &
    sleep 0.5
    if [ -f "$HOME/.config/hypr/wallpaper.png" ]; then
        hyprctl hyprpaper wallpaper "all,$HOME/.config/hypr/wallpaper.png" 2>/dev/null
    fi
fi

# Wallust dynamic theming: derive the palette from the wallpaper and render
# the Hyprland/Waybar templates. Then reload both so new colors apply live.
if command -v wallust >/dev/null 2>&1 && [ -f "$HOME/.config/hypr/wallpaper.png" ]; then
    if wallust run "$HOME/.config/hypr/wallpaper.png" >/dev/null 2>&1; then
        hyprctl reload >/dev/null 2>&1 || true
        pkill -RTMIN+1 waybar >/dev/null 2>&1 || true
    fi
fi &

# Clipboard + notifications (best-effort)
command -v wl-paste >/dev/null 2>&1 && wl-paste --type text --watch cliphist store >/dev/null 2>&1 &
command -v dunst >/dev/null 2>&1 && dunst >/dev/null 2>&1 &
