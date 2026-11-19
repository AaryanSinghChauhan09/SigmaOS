# Hyprland Configuration Guide

SigmaOS uses Hyprland as the default Wayland tiling compositor with custom SigmaOS branding.

## Configuration Files

### Main Config
Location: `iso/archiso/airootfs/etc/skel/.config/hyprland/hyprland.conf`

Key features:
- Monitor setup with preferred resolution
- Input settings (keyboard, touchpad)
- Window decoration with blur and shadows
- Dwindle layout
- Animations
- Keybindings (SUPER key as main modifier)

### Autostart Script
Location: `iso/archiso/airootfs/etc/skel/.config/hyprland/autostart.sh`

Executed on Hyprland startup:
- Sets wallpaper with hyprpaper
- Starts notification daemon (dunst)
- Initializes clipboard manager (cliphist)
- Loads Wallust theme

## Keybindings

| Keybinding | Action |
|-----------|--------|
| SUPER + RETURN | Open terminal (foot) |
| SUPER + Q | Close active window |
| SUPER + M | Exit Hyprland |
| SUPER + E | Open file manager (thunar) |
| SUPER + V | Toggle floating mode |
| SUPER + SPACE | Toggle split |
| SUPER + [ARROWS] | Move focus |
| SUPER + SHIFT + [ARROWS] | Move window |
| SUPER + 1-0 | Switch workspace |
| SUPER + SHIFT + 1-0 | Move to workspace |
| SUPER + S | Screenshot (hyprshot) |
| SUPER + D | Launcher (rofi) |

## Theming

SigmaOS uses Wallust for dynamic theming:

1. Install wallpaper: `hyprpaper` or set manually
2. Generate colors: `wallust run sigmaos`
3. Applies to: Hyprland, Waybar, terminal

## Multi-Monitor

Default configuration:
- Uses preferred resolution for each monitor
- Unified keybindings across monitors
- Per-monitor scaling (via fractional scaling)

## Troubleshooting

### Touchpad not working
Check input settings in hyprland.conf:
```
input {
    touchpad {
        natural_scroll=no
        disable_while_typing=1
        tap-to-click=1
    }
}
```

### No wallpaper on boot
Ensure autostart.sh is executable and runs:
```bash
chmod +x ~/.config/hyprland/autostart.sh
```

### Keybindings not working
Check Hyprland logs:
```bash
hyprctl monitors
hyprctl clients
```
