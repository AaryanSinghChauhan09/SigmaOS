# SigmaOS Zenith Desktop Environment Master Development Plan

## Executive Summary

The **Zenith Desktop Environment** is SigmaOS's flagship user interface and compositor architecture. It synthesizes the most powerful, beautiful, and performant user interface paradigms from major Linux distributions (**Hyprland**, **System76 COSMIC**, **KDE Plasma 6**, **GNOME 46**, **Sway**, **Linux Mint Cinnamon**, **Omarchy Omakase**) and BSD desktop ecosystems (**FreeBSD Lumina 2.0**, **OpenBSD cwm/fvwm**, **NetBSD wtwm**).

Zenith bridges hardware-accelerated 3D compositing, Wayland layer-shell protocol integration, zero-latency dynamic tiling, capability-sandboxed app portals, and multi-distro aesthetic personalization into a unified, sovereign Rust desktop.

---

## Architecture & Core Pillars

Zenith is organized around **6 Fundamental Pillars**:

```
+-----------------------------------------------------------------------------------+
|                        ZENITH DESKTOP ENVIRONMENT ARCHITECTURE                    |
+-----------------------------------------------------------------------------------+
| 1. Zenith Compositor & Window Management Core (Hyprland / Sway / Lumina 2.0)      |
| 2. Modular Shell & Control Center (COSMIC Rust Shell / GNOME Shell / Plasma)       |
| 3. Multi-Distro Aesthetic Engine (Omarchy Themes / Mint Accents / Kali Stealth)   |
| 4. XDG Desktop Portal Bridges (Flatpak / Snap / Landlock / Capability Prompts)    |
| 5. ScreenLocker & Session Management (ext-session-lock-v1 / PAM / FIDO2)          |
| 6. Hardware-Accelerated Rendering Pipeline (Vulkan / OpenGL ES 3.2 / DRM Scanout) |
+-----------------------------------------------------------------------------------+
```

---

### Pillar 1: Compositor & Window Management Core
- **Hybrid Layout Engine**:
  - **Dynamic Tiling Mode**: Master-Stack, Binary Space Partitioning (BSP), and Equal-Width Column Grid with auto-gaps and per-workspace rules.
  - **Floating & Stacking Mode**: Z-order window layering (`Background`, `Widgets`, `Application`, `AlwaysOnTop`, `Panel`, `OSD/Overlay`, `LockScreen`).
  - **Auto-Snap & Grid Snapping**: Quarter/half screen window tile snap zones.
- **Wayland Protocol Subsystem**:
  - Direct implementation of `wlr-layer-shell-v1`, `xdg-shell-v6`, `ext-session-lock-v1`, and `xdg-output-unstable-v1`.
  - Sway IPC command protocol parity (`zenith-ctl focus-window left`, `zenith-ctl set-layout bsp`).

---

### Pillar 2: Modular Shell & Control Center
- **Zenith Panel (`src/desktop/panel.rs`)**:
  - Modular top/bottom panel with launcher menu (`src/desktop/launcher.rs`), system tray indicators (`SNI` / `StatusNotifierItem`), and notification center (`src/desktop/notification.rs`).
  - Quick Settings Overlay: Wi-Fi, Bluetooth, Audio Volume/Device, Power Profiles (`Performance`, `Balanced`, `PowerSaver`), and Dark Mode.
- **Application Launcher**:
  - Instant fuzzy search over native applications, Flatpak/Snap apps, Web WASM apps, and terminal commands.
  - User-Defined Function (UDF) hotkey triggers and workflow actions.
- **Desktop Settings Manager (`src/desktop/settings.rs`)**:
  - Comprehensive control panel for displays, scaling, audio routing, input devices (touchpad gestures, mouse acceleration), themes, and power states.

---

### Pillar 3: Multi-Distro Aesthetic & Personalization Engine
- **Curated Theme Engine**:
  - Native themes inspired by Omarchy Linux: **Tokyo Night**, **Catppuccin Macchiato/Mocha**, **Gruvbox Dark**, **Nord**, **Ayu Dark**, and **Material Ocean**.
  - **Linux Mint Accent Color Selection**: Customize primary accent colors across window borders, buttons, and selection highlights.
  - **Kali Linux Undercover Disguise**: One-click stealth transformation converting Zenith UI into a standard corporate desktop theme.
- **Real-Time Visual Effects Pipeline**:
  - **Dual-Kawase Gaussian Blur**: Hardware-accelerated background blur applied to transparent panel overlays, popups, and terminal windows.
  - **Acrylic Translucency & Drop Shadows**: Variable alpha blending with dynamic anti-aliased drop shadows and rounded corner clipping.

---

### Pillar 4: XDG Desktop Portal & Security Capability Bridges
- **Portals Subsystem (`src/desktop/desktop_portal.rs`)**:
  - Native implementation of `org.freedesktop.impl.portal.*` interfaces:
    - `FileChooser`: Sandboxed file dialogs enforcing Landlock / Pledge path unveil rules.
    - `Screenshot` & `ScreenCast`: Wayland-native screen and window capture (`src/desktop/screenshot.rs`).
    - `Clipboard`: Bi-directional clipboard synchronization with primary/secondary selection rings (`src/desktop/clipboard.rs`).
    - `DarkLauncher` & `ColorScheme`: Dynamic system-wide light/dark mode event propagation.
- **Zenith Exec Guard Integration (`src/security/exec_guard.rs`)**:
  - Default-Deny capability prompt dialogs requesting explicit user authorization for privileged process execution or network access.

---

### Pillar 5: ScreenLocker & Session Lifecycle Management
- **Wayland Session Locker (`src/desktop/screensaver.rs`)**:
  - Enforces `ext-session-lock-v1` Wayland protocol ensuring client surfaces cannot obscure or bypass screen locking.
  - DBus `org.freedesktop.ScreenSaver` inhibitor tracking (`inhibit`/`uninhibit`) during video playback, presentations, or full-screen gaming.
- **Authentication Safeguards**:
  - Multi-factor authentication via PAM, TPM 2.0 PCR sealing, YubiKey HMAC-SHA1, and CTAP2/FIDO2 security keys.
  - Secure zeroing of password memory buffers immediately following authentication verification.

---

### Pillar 6: Hardware-Accelerated Rendering Pipeline
- **Multi-Monitor & Fractional Scaling**:
  - Independent per-display workspaces or unified display spans.
  - Crisp fractional UI scaling (125%, 150%, 175%, 200%) using viewport scaling shaders.
  - Variable Refresh Rate (VRR / FreeSync / G-Sync) adaptive presentation for zero-tear gaming and media playback.
- **Direct DRM/KMS Scanout**:
  - Bypasses compositing overhead for full-screen games and video surfaces via direct hardware plane scanout.

---

## 4-Phase Chronological Development Roadmap

### Phase 1: Bootable Wayland Core & Surface Rendering
- Complete `src/desktop/zenith_compositor.rs` surface damage tracking and DRM primary scanout.
- Verify Wayland `xdg-shell` window placement and basic floating/tiling layouts.

### Phase 2: Shell, Panel & Application Launcher
- Deploy modular `panel.rs` with `wlr-layer-shell-v1` panel anchoring.
- Implement system tray indicator bus, notification popup stack, and fuzzy application launcher.

### Phase 3: Desktop Settings, Portals & Aesthetics
- Implement `desktop_portal.rs` file chooser, screenshot, and dark preference portals.
- Integrate Omarchy theme suite, Mint accent selector, and Kali Undercover disguise modes.

### Phase 4: Session Locking, Power & Production Polish
- Enforce `ext-session-lock-v1` screensaver locker with PAM/FIDO2 authentication.
- Implement multi-monitor fractional scale shaders and VRR presentation.

---

## Verification Metrics & Test Benchmarks

| Component | Target Metric | Test Verification Command |
| :--- | :--- | :--- |
| Window Manager | <1ms Tiling Layout Calculation | `rustc --test src/desktop/zenith_compositor.rs --edition=2021` |
| Screen Locker | Zero Memory Leak & Instant Zeroing | `rustc --test src/desktop/screensaver.rs --edition=2021` |
| Notification Daemon | >1000 events/sec throughput | `rustc --test src/desktop/notification.rs --edition=2021` |
| Panel Shell | <5MB RAM footprint | `rustc --test src/desktop/panel.rs --edition=2021` |
| Full Desktop Suite | 100% Native Test Pass | `./run_sigma_tests.sh` |
