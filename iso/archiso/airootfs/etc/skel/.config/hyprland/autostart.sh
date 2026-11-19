#!/bin/bash
# SigmaOS Hyprland Autostart Script

# Set wallpaper
hyprctl hyprpaper wallpaper "all,~/.config/wallust/wall.png"

# Start notification daemon
dunst &

# Start clipboard manager
wl-paste --type text --watch cliphist store &

# Load wallust theme
wallust run sigmaos
