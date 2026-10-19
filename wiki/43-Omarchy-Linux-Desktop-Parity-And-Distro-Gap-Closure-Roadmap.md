# 43. Omarchy Linux Desktop Parity & Distro Gap Closure Roadmap

## Overview
This roadmap establishes the architecture, component specs, and pull-request verification workflows for complete userland and desktop environment parity with **Omarchy Linux**.

---

## Subsystem Gap Closure Matrix

### 1. Wallpaper & Dynamic Background Management (`Hyprpaper` & `Waypaper`)
- **Preload Memory Pool**: Wallpapers pre-loaded into zero-copy RAM buffers to ensure instantaneous wallpaper transitions across multiple displays without disk latency.
- **IPC Interface**: Active IPC listener supporting dynamic configuration updates from Waypaper and Wallust.

### 2. Idle Timer & Screen Shield (`Hypridle` & `Hyprlock`)
- **Power Management**: Multi-stage idle handlers (screen dimming, session locking via Hyprlock, DPMS screen sleep, system suspend).
- **Biometric PAM Support**: Unified PAM authentication pipeline hardened against side-channel username enumeration.

### 3. Notification Center (`Swaync`)
- **Widget Stack**: Integrated control center with MPRIS audio controls, Do Not Disturb toggles, backlight sliders, and volume indicators.
- **Suppression State**: Instant toggle for full-screen gaming and presentation mode notification suppression.

### 4. Dynamic Color Palette Extraction (`Wallust`)
- **Color Syncing**: Pywal-compatible palette extraction generating matching 16-color themes for Ghostty terminal, Waybar status bar, Swaync control center, and GTK applications.

---

## Verification & Execution Commands
```bash
# Compile and run Omarchy Linux V61 standalone tests
mkdir -p build
rustc --test --edition=2021 src/distro/omarchy_linux_missing_components_v61_pr.rs -o build/omarchy_v61_test
./build/omarchy_v61_test
```
