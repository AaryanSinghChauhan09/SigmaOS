# SigmaOS Desktop Themes & Aesthetic Engine Development Master Plan

## Executive Summary

The **SigmaOS Desktop Themes Subsystem** (`ThemeEngine` in `src/theming/theme_engine.rs`, `OmarchyTheme` in `src/distro/omarchy.rs`, `KaliUndercoverThemeMode` in `src/security/kali_stack.rs`) is engineered as a unified aesthetic engine combining Omarchy Linux curated theme switching (Tokyo Night, Catppuccin, Gruvbox, Nord, Ayu, Material Ocean) with Linux Mint accent color customization, Kali Linux Undercover disguise modes, Garuda Linux Neon GL compositor shaders, Pop!_OS Cosmic auto dark-mode scheduling, and FreeBSD Lumina/qt5ct cross-toolkit styling.

This document defines the master development plan for the SigmaOS desktop theming engine across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │            SigmaOS Universal Theme Subsystem             │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│       Omarchy Linux       │            │  Linux Mint & Pop!_OS     │            │   Kali & Garuda Linux     │
│ • Omakase Curated Themes  │            │ • Dynamic Accent Color    │            │ • Undercover Disguise Mode│
│ • Tokyo Night & Catppuccin│            │ • GTK3/4 & Qt Transpiling │            │ • Neon GL Shader Blurs    │
│ • Sub-100ms Live Cycle    │            │ • Time-Based Auto Dark-Mode│            │ • Glassmorphism Compositor│
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core Desktop Theming Development Pillars

### Pillar 1: Dynamic Color Palette Engine & Accent Color Pipeline (`src/theming/theme_engine.rs`)
* **Omakase Curated Palette Engine**:
  - Maintain high-contrast, accessibility-audited color palettes: Tokyo Night, Catppuccin Mocha/Latte, Gruvbox Dark/Light, Nord, Ayu, and Material Ocean (`OmarchyTheme` in `src/distro/omarchy.rs`).
* **Dynamic Accent Color Injection**:
  - Automatically derive active accent colors, selection highlights, and status indicators across all desktop components.

### Pillar 2: Multi-Toolkit Theme Transpiler (GTK3/4, Qt5/6, Wayland Zenith, Terminal ANSI)
* **Cross-Toolkit Style Generation**:
  - Automatically transpile theme palettes into CSS for GTK3 (`~/.config/gtk-3.0/gtk.css`), GTK4 (`libadwaita` color schemes), Qt KStyle/qt5ct configurations, and Zenith Wayland compositor sub-surfaces.
* **ANSI Terminal Color Mapping**:
  - Dynamically generate 16-color ANSI terminal palettes for `sigma_sh` and Zenith terminal emulators.

### Pillar 3: Kali Undercover Disguise Engine (`KaliUndercoverThemeMode` in `src/security/kali_stack.rs`)
* **Stealth Disguise Mode Switcher**:
  - Instant one-click transformation (`toggle_undercover`) morphing the Zenith desktop UI into familiar OS disguises (`Windows10Disguise`, `Windows11Disguise`, `MacOsDisguise`) for stealth operations in public environments.

### Pillar 4: Wallpaper Shaders & Glassmorphism Compositor Effects
* **Hardware-Accelerated GL Shaders**:
  - Compute shader wallpaper blurs, animated gradient waves, and frosted glass window titlebars rendered directly on VirtIO / DRM/KMS GPU framebuffers.

### Pillar 5: Cursor, Font & Icon Set Packaging (`sigpkg`)
* **Icon Set & Cursor Vector Management**:
  - Package SVG vector icon themes and cursor animations into atomic `sigpkg` bundles (`/usr/share/icons/sovereign-icons`).

### Pillar 6: Time-Based Auto Dark-Mode & Solar Calculator
* **Diurnal Day/Night Transition**:
  - Automatically transition UI color schemes between light and dark variants based on local solar sunrise/sunset schedules or ambient light sensors.

---

## 3. Four-Phase Chronological Roadmap

```
  Phase 1: Omakase Palette Engine & Accent Color Transpilation (Months 1–3)
  ├── Omakase Palette Catalog (Tokyo Night, Catppuccin, Gruvbox, Nord)
  ├── Dynamic Accent Color Generator & Selection Highlight Engine
  └── ANSI 16-Color Terminal Palette Transpiler

  Phase 2: Cross-Toolkit GTK/Qt/Zenith Style Generator (Months 3–6)
  ├── GTK3/GTK4 CSS Style Transpiler (`gtk-3.0/gtk.css`, `libadwaita`)
  ├── Qt5/Qt6 KStyle & qt5ct Color Scheme Exporter
  └── Zenith Wayland Compositor Surface Border & Shadow Styling

  Phase 3: Kali Undercover Disguise & Glassmorphism Shaders (Months 6–9)
  ├── `KaliUndercoverThemeMode` Disguise Switcher (Windows 10/11, macOS)
  ├── DRM/KMS GPU Compute Shader Glassmorphism Blurs
  └── SVG Vector Icon & Cursor Theme `sigpkg` Package Installer

  Phase 4: Auto Dark-Mode & Theme Marketplace Integration (Months 9–12)
  ├── Solar Day/Night Schedule Auto Dark-Mode Engine
  ├── Community Theme Marketplace Sync (`sigpkg install theme-tokyonight`)
  └── Full Sub-100ms Live Theme Cycling Test Suite
```

---

## 4. Verification and Benchmark Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **Theme Switching Speed** | Live Theme Cycle Benchmark | < 100ms propagation across all running GTK, Qt, and Zenith windows |
| **Transpiler Latency** | Palette-to-CSS Generation Test | < 5ms generation time per toolkit stylesheet |
| **Disguise Toggle Speed** | `toggle_undercover` Test | Instant < 50ms UI layout and icon swap |
| **Compositor Memory Overhead** | Zenith Glassmorphism Render | < 2% additional GPU VRAM allocation for frosted glass blurs |
