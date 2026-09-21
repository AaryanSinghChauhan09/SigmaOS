# XFCE Desktop Guide: Lightweight Desktop Session in SigmaOS

## Introduction

SigmaOS includes native support for the XFCE desktop environment (`Zenith XFCE Engine`). Renowned for its low memory usage, modular design, and classic desktop layout, XFCE is ideal for workstations, laptops, and resource-constrained hardware.

## Key Features in SigmaOS

1. **Ultra-Low Memory Footprint**: Uses less than 180MB RAM on cold boot.
2. **Whiskermenu Launcher**: Fast application searching, category browsing, and recent file tracking.
3. **Thunar File Manager**: Dual-pane file browsing, tabbed navigation, and custom right-click actions.
4. **xfwm4 Window Snapping**: Edge snapping, quarter/half screen tile grid shortcuts (`Super + Left/Right/Up/Down`).

## Customizing XFCE in SigmaOS

### 1. Panel Applets & Layout
To add or configure panel applets:
1. Right-click any empty space on the panel -> **Panel** -> **Panel Preferences**.
2. Go to the **Items** tab to add applets like CPU Graph, Network Monitor, Clipboard Manager (`Clipman`), or PulseAudio/PipeWire Volume Control.

### 2. Changing Window Themes & Appearance
Use `xfconf` settings dialogs or CLI commands:
```bash
# Set GTK theme to Dark Mode
xfconf-query -c xsettings -p /Net/ThemeName -s "Sigma-Dark"

# Set Window Manager Titlebar Theme
xfconf-query -c xfwm4 -p /general/theme -s "Sigma-Dark"
```

### 3. Thunar Custom Actions
To add custom right-click actions in Thunar (e.g. "Open Terminal Here"):
- Open Thunar -> **Edit** -> **Configure custom actions...**
- Command: `zenith-terminal --working-directory=%f`
- Pattern: `*` (Directories)

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Super` or `Alt + F1` | Open Whiskermenu Application Launcher |
| `Alt + Tab` | Switch Between Active Windows |
| `Super + Left / Right` | Snap Window to Left / Right Half Screen |
| `Ctrl + Alt + T` | Open Zenith Terminal |
| `Super + L` | Lock Screen |
