# SigmaOS-settings

System settings UI for SigmaOS. Panels for display, sound, network, privacy, and accessibility. Inspired by gnome-control-center, cinnamon-control-center, and xfce4-settings.

## Upstream Inspiration

- [gnome-control-center](https://gitlab.gnome.org/GNOME/gnome-control-center) — comprehensive GNOME settings
- [cinnamon-control-center](https://github.com/linuxmint/cinnamon-control-center) — Linux Mint settings
- [xfce4-settings](https://docs.xfce.org/xfce/xfce4-settings/start) — lightweight XFCE settings

## Purpose

- **Display**: resolution, orientation, refresh rate, night light
- **Sound**: volume, output/input device selection via PipeWire
- **Network**: connect/disconnect, scan Wi-Fi, VPN via NetworkManager
- **Privacy**: microphone/camera toggle, location, screen lock
- **Accessibility**: font scale, high contrast, keyboard layout, magnifier

## Build Instructions

```bash
sudo apt install libgtk-4-dev libpipewire-0.3-dev libnetworkmanager-dev

cargo build --release
cargo test
```

## API Overview

- `DisplayPanel` — display configuration state
- `SoundPanel` — PipeWire audio device management
- `NetworkPanel` — NetworkManager connection management
- `SettingsManager` — top-level settings coordinator
