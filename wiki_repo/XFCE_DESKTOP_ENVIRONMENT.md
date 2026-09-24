# SigmaOS XFCE Desktop Environment Integration Specification (`Zenith XFCE Engine`)

## 1. Executive Summary

The Zenith XFCE Engine provides full desktop environment integration for the lightweight, modular XFCE 4 desktop suite in SigmaOS. Drawing inspiration from XFCE 4.18/4.20 on Arch Linux, Xubuntu, and FreeBSD Desktop setups, it bridges core XFCE components (`xfwm4`, `xfce4-panel`, `thunar`, `xfconf`, `xfce4-session`) with the native Zenith Display Server and Wayland IPC layer.

## 2. Desktop Subsystem Architecture

```
+------------------------------------------------------------------+
|                   XFCE Desktop Application Suite                 |
|  +---------------+  +---------------+  +---------------+  +----+ |
|  | xfce4-panel   |  | Thunar FM     |  | Whiskermenu   |  |App | |
|  | (Panel Bars)  |  | File Manager  |  | Launcher      |  |lets| |
|  +-------+-------+  +-------+-------+  +-------+-------+  +--+-+ |
+----------|------------------|------------------|-------------|---+
           |                  |                  |             |
+----------v------------------v------------------v-------------v---+
|                   Zenith XFCE Integration Engine                |
|  +-------------------------------------------------------------+ |
|  | 1. xfwm4 Window Decorator & Compositor Frame Bridge        | |
|  | 2. xfconf Configuration Registry Daemon (/etc/xfconf/)      | |
|  | 3. XFCE Panel Layer-Shell Anchor Manager                    | |
|  | 4. Thunar VFS & Custom Action Plugin Router                 | |
|  +-------------------------------+-----------------------------+ |
+----------------------------------|-------------------------------+
                                   |
+----------------------------------v-------------------------------+
|                Zenith Display Server / Wayland IPC               |
+------------------------------------------------------------------+
```

## 3. Core Component Bridges

### 3.1 `xfwm4` Window Manager Bridge
- Maps `xfwm4` titlebar decorations, button layouts (minimize, maximize, close), and window snapping vectors to Zenith surface geometry.
- Enforces low-latency 2D/3D compositing shaders with anti-aliased window corner rounding.

### 3.2 `xfce4-panel` Applet Protocol
- Panel items (Whiskermenu launcher, Window Buttons tasklist, System Tray / SNI, Battery Monitor, CPU/Mem Graph) are anchored to screen edges using `zenith_layer_shell`.
- Autohide and intelligent panel dodge modes are supported with zero CPU idle overhead.

### 3.3 `xfconf` Channel Registry
- Replaces traditional monolithic XML storage with a high-performance in-memory key-value channel registry (`xfconfd`).
- Configuration channels (`xsettings`, `xfce4-desktop`, `xfwm4`, `displays`) update live via IPC event broadcasts.

### 3.4 Thunar File Manager Integration
- Integrates Thunar with the SigmaOS VFS layer (`@root`, `@home`, `@store`).
- Supports Thunar custom actions (Open Terminal Here, Hash Verification, Fast Downloader).

## 4. Performance & Memory Benchmarks

- **RAM Consumption**: Base desktop cold boot footprint < 180MB RAM.
- **Cold Startup Latency**: Full desktop shell session ready in < 150ms.
