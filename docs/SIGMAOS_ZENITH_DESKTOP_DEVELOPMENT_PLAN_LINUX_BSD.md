# SigmaOS Zenith Desktop Environment Development Plan

## Executive Overview

The **Zenith Desktop Environment** (`ZenithCompositor` in `src/desktop/zenith_compositor.rs`, `ZenithDesktopEngine` in `src/desktop/zenith.rs`, `WaylandProtocolEngine` in `src/desktop/wayland_protocol.rs`) is engineered as a modern, hardware-accelerated Wayland desktop environment for SigmaOS. It bridges the visual fluidity and dynamic layout paradigms of **Linux Wayland compositors** (Hyprland fluid animations & dynamic tiling, COSMIC Rust modular shell architecture, Sway i3-compatible IPC, KDE Plasma 6 KWin Wayland protocols, GNOME 46 xdg-desktop-portal integration) with the simplicity, security, and desktop innovations of **BSD desktop shells** (FreeBSD Lumina 2.0 desktop shell, OpenBSD cwm/fvwm lightweight window managers, and Omarchy Omakase curated desktop layouts).

This document establishes the master development plan for the Zenith Desktop Environment across 7 core architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │           Zenith Wayland Desktop Environment             │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│ Wayland Compositors (Linux)│           │   KDE Plasma & GNOME 46   │            │   BSD & Omarchy Shells    │
│ • Hyprland Dynamic Tiling │            │ • KWin Wayland Protocols  │            │ • FreeBSD Lumina 2.0 Shell│
│ • COSMIC Rust Modular Shell│           │ • xdg-desktop-portal API  │            │ • Omarchy Omakase Layouts │
│ • Sway i3-compatible IPC  │            │ • Multi-Display HDR & VRR │            │ • OpenBSD cwm/fvwm        │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Seven Core Zenith Desktop Development Pillars

### Pillar 1: Zenith Wayland Compositor Core & DRM/KMS Acceleration (`src/desktop/zenith_compositor.rs`)
* **DRM/KMS Atomic Modesetting & EGL/OpenGL Renderer**:
  - Direct hardware rendering on DRM/KMS card nodes (`/dev/dri/card0`) with hardware-accelerated EGL/GLES2 frame composition.
  - Zero-copy Wayland `wl_surface` buffer damage tracking and sub-surface commit management.
  - XWayland xcb event translation (`translate_x11_configure_notify`) and legacy X11 app surface bridging.

### Pillar 2: Dynamic Auto-Tiling & Stacking Window Manager (`src/desktop/zenith.rs`)
* **Multi-Layout Window Manager**:
  - **Dynamic Tiling Mode**: Master-stack, BSP (Binary Space Partitioning), and spiral auto-tiling with adjustable gaps and window smooth animations inspired by Hyprland and Sway.
  - **Floating / Stacking Mode**: Overlapping window management with snap-to-edge magnetic snapping and window titlebar controls.
  - **Omarchy Omakase Presets**: One-click switching between macOS-dock, Windows-11 taskbar, GNOME modern overview, and Hyprland tiling layouts (`src/desktop/omarchy_omakase.rs`).

### Pillar 3: XDG Desktop Portals & PipeWire Screencast Integration (`src/desktop/wayland_protocol.rs`)
* **Protocol & Portal Extensions (`src/desktop/desktop_portal.rs`)**:
  - Support core Wayland protocols (`wl_compositor`, `wl_shm`, `wl_seat`, `xdg_shell`, `zwp_linux_dmabuf_v1`).
  - Implement XDG desktop portal backends for PipeWire screen recording (`org.freedesktop.portal.ScreenCast`), file chooser dialogs, and wallpaper settings.

### Pillar 4: Shell Panel, Fuzzy Launcher & Notification Daemon (`panel.rs`, `launcher.rs`, `notifications.rs`)
* **Integrated Desktop Shell Components**:
  - **Zenith Panel (`panel.rs`)**: System tray, workspace pager, battery/power indicator, clock, and network status applet.
  - **Application Launcher (`launcher.rs`)**: Fuzzy-search application launcher, calculator, and command runner with sub-10ms query response.
  - **Notification Daemon (`notifications.rs`)**: Desktop notification server (`org.freedesktop.Notifications`) supporting action buttons and sound effects.

### Pillar 5: Zenith Dual-Pane File Manager & Settings Control Center (`filemanager.rs`, `settings.rs`)
* **Userland Desktop Apps**:
  - **Zenith File Manager (`filemanager.rs`)**: Dual-pane, tabbed, and Miller columns file browsing with inline image previews, path sandboxing (`PathSandboxGuard`), directory caching (`DirectoryCache`), and archive extraction.
  - **Zenith Control Center (`settings.rs`)**: Unified GUI control panel managing display resolution, fractional scaling, dark mode scheduling, audio output, network connections, and bluetooth devices.

### Pillar 6: Multi-Display HDR, Variable Refresh Rate (VRR) & Touch Gestures (`zenith_advanced_features.rs`)
* **Advanced Display & Touch Subsystem**:
  - Multi-monitor display routing with independent per-monitor refresh rates and fractional DPI scaling.
  - Wayland `wp_color_management_v1` HDR color grading and Adaptive Sync / FreeSync VRR support.
  - 1:1 touchpad and touchscreen multi-finger gesture navigation (swipe to change workspace, pinch to zoom).

### Pillar 7: Omarchy Omakase Curated Theme Studio & Aesthetic Engine (`src/theming/theme_engine.rs`)
* **Aesthetic & Theme Manager**:
  - Curated color scheme engine supporting Tokyo Night, Catppuccin, Gruvbox, Nord, Ayu, and Material Ocean themes (`OmarchyTheme` in `src/distro/omarchy.rs`).
  - Dark mode scheduling, wallpaper transition engine, and cross-toolkit GTK/Qt/Wayland theme synchronization.

---

## 3. Four-Phase Chronological Development Roadmap

```
  Phase 1: Zenith Compositor Core & Dynamic Tiling Engine (Months 1–3)
  ├── DRM/KMS Atomic Modesetting & EGL Surface Renderer
  ├── Master-Stack & BSP Dynamic Auto-Tiling Window Layouts
  └── Omarchy Omakase Desktop Layout Presets (macOS, Windows 11, Tiling)

  Phase 2: XDG Desktop Portals & Shell Applet Suite (Months 3–6)
  ├── Wayland `xdg_shell`, `zwp_linux_dmabuf_v1`, & XDG Desktop Portals
  ├── Zenith Panel, Fuzzy Launcher, & Notification Daemon
  └── PipeWire Screencast & Wayland Clipboard Sync

  Phase 3: File Manager, Settings Center & Accessibility (Months 6–9)
  ├── Zenith Dual-Pane File Manager & Archive Preview
  ├── Zenith System Control Center (Display, Audio, Wi-Fi, Bluetooth)
  └── AT-SPI2 Screen Reader Accessibility Bus Integration

  Phase 4: Multi-Display HDR/VRR & Sub-16ms Latency Verification (Months 9–12)
  ├── Variable Refresh Rate (Adaptive Sync) & Multi-Monitor Fractional Scaling
  ├── 1:1 Touchpad Multi-Finger Gestures & Wayland HDR Color Management
  └── Sub-16ms Frame Latency Benchmark & Zero Frame Tearing Test Suite
```

---

## 4. Benchmark & Acceptance Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **Compositor Frame Latency** | `weston-simple-egl` / `vkmark` | < 16ms frame time at 60Hz / < 7ms at 144Hz |
| **Launcher Query Speed** | Fuzzy Search Test (1000 Desktop Files) | Sub-10ms search query and result rendering |
| **Idle Memory Footprint** | System RAM Usage Metric | < 250 MB total RAM for Zenith compositor + panel + notification daemon |
| **Multi-Monitor Display Sync** | Dual 4K Monitor DRM Test | Zero tearing; independent per-monitor VRR refresh rates |
