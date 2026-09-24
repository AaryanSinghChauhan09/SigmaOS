# 🖼️ SigmaOS Desktop Backgrounds, Wallpapers & Dynamic Theme Extraction (`backgrounds`) Strategic Development Plan

## Executive Summary & Design Vision

Desktop backgrounds and wallpapers in modern operating systems are no longer static image buffers. They form the core of desktop visual identity, dynamic UI theme generation, time-of-day circadian lighting, multi-monitor workspace personalization, and live GPU-rendered animated backgrounds.

Drawing inspiration from modern Wayland wallpaper tools (**Hyprpaper**, **Swaybg**, **mpvpaper**), Linux distribution artwork infrastructure (Fedora dynamic day/night XML wallpapers, Ubuntu dynamic backgrounds, GNOME libadwaita Material You palette extraction, Zorin Chameleon Engine), and BSD desktop compositors, the **SigmaOS Background Subsystem** (`OmarchyHyprpaperWallpaperEngine`, `ZorinChameleonEngine`, `MaterialYouPaletteEngine`, `ZenithBackgroundCompositor`) provides a zero-dependency, high-performance background daemon in Safe Rust.

---

## 1. Multi-Distro & Multi-OS Background Inspirations

### 1.1 Hyprland Hyprpaper & Swaybg (Wayland `wlr-layer-shell` Protocol)
- **Inspirations**:
  - **IPC Wallpaper Control**: Rapid wallpaper preload and hot-swapping via IPC unix socket commands without desktop flicker.
  - **Per-Monitor Wallpaper Assignment**: Assigning distinct wallpapers or scaling modes (`fill`, `contain`, `cover`, `tile`, `center`) to individual physical display outputs.
- **SigmaOS Integration**: `OmarchyHyprpaperWallpaperEngine` in `src/distro/omarchy.rs` and `wlr_layer_shell` background surface rendering in Zenith compositor.

### 1.2 Fedora & Ubuntu Dynamic Day-Night & Seasonal XML Wallpapers
- **Inspirations**:
  - **Circadian Time-of-Day Transitions**: XML/JSON dynamic background manifests scheduling smooth cross-fades between morning, noon, sunset, and night artwork matching system clock and geographical coordinates.
- **SigmaOS Integration**: `DynamicDayNightWallpaperScheduler` in `src/desktop/` and `/usr/share/backgrounds/sigma/` metadata manifests.

### 1.3 Zorin OS Chameleon Engine & Android Material You Color Palette Extraction
- **Inspirations**:
  - **Wallpaper-Driven Dynamic UI Themes**: Extracting dominant, vibrant, and muted color palettes (RGB histogram clustering) from active wallpaper images to automatically dynamically tint window borders, taskbar accents, terminal color schemes, and application highlights.
- **SigmaOS Integration**: `ZorinChameleonEngine::calculate_accent_from_wallpaper` in `src/compatibility/zorin.rs` and Material You palette extraction in `src/distro/tech_media_distro_innovations.rs`.

### 1.4 EGL / Vulkan Live Animated Shaders & Video Wallpapers (`mpvpaper`)
- **Inspirations**: Live GPU GLSL shader backgrounds and zero-copy hardware-decoded video wallpaper looping with low CPU/RAM footprint.
- **SigmaOS Integration**: `LiveShaderWallpaperEngine` and hardware-accelerated video background surface loopers.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Zenith Wayland Compositor Layer Surface               │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              Wayland Background Daemon (`wlr-layer-shell`)                │
│     - Layer `ZwlrLayerShellV1::Layer::Background` surface creation       │
│     - Per-Monitor Wallpaper Assignment (Display-0, Display-1)            │
│     - Scaling & Crop Modes: `Cover`, `Contain`, `Center`, `Tile`, `Stretch`│
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│          Dynamic Material You & Chameleon Accent Color Extractor          │
│     - Fast RGB Histogram K-Means Color Clustering Engine                  │
│     - Generates Primary Accent, Surface Tint, Border Hue & Highlight RGB  │
│     - Auto-applies color scheme to Zenith Taskbar & Terminal Themes       │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│            Dynamic Day/Night Circadian & Animated Shader Engine           │
│     - Time-of-day solar position scheduling & smooth alpha blending      │
│     - EGL/Vulkan live GLSL ambient background shader pipeline             │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Color Extraction Performance Guarantee
- RGB histogram palette extraction target runtime: **< 10 milliseconds** for 4K images (using downsampled 256x256 thumbnail analysis).
- Seamless palette propagation across userland theme daemons via D-Bus / IPC.

### 2.2 Memory & CPU Footprint
- Background surface VRAM usage optimized with shared memory (`wl_shm`) or DRM DMA-BUFs.
- Live shader backgrounds throttled to 30 FPS with automatically paused rendering when full-screen applications or games are focused.

---

## 3. Phased Development Roadmap

### Phase 1: Wayland Background Layer Surface & Hyprpaper IPC (Q4 2026)
- Stabilize `wlr-layer-shell` background surface rendering in Zenith Wayland compositor.
- Implement `OmarchyHyprpaperWallpaperEngine` IPC command handler (`preload`, `wallpaper`, `unload`).
- Support per-monitor background assignment and image scaling algorithms.

### Phase 2: Dynamic Day/Night & Circadian Solar Engine (Q1 2027)
- Deploy XML/JSON dynamic background manifest parser in `/usr/share/backgrounds/`.
- Connect system clock and geographic location solar engine for smooth morning-to-night cross-fading.
- Implement background slideshow rotation timer.

### Phase 3: Material You & Chameleon Dynamic Palette Extraction (Q2 2027)
- Integrate `ZorinChameleonEngine` RGB histogram downsampling and color clustering.
- Automate real-time accent color synchronization across Zenith taskbar, window borders, and terminal color profiles.
- Add user UI toggle for dynamic wallpaper accent matching.

### Phase 4: Live GPU Shaders, Video Wallpapers & Benchmarks (Q3 2027+)
- Implement EGL/Vulkan GLSL live ambient shader background renderer.
- Add automatic pause/resume triggers on full-screen app focus to save GPU power.
- Include background rendering benchmarks in `scripts/tech_media_benchmark_suite.sh`.

---

## 4. Verification & Testing Standards

All background components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/distro/omarchy.rs` (`OmarchyHyprpaperWallpaperEngine`)
- `src/compatibility/zorin.rs` (`ZorinChameleonEngine` accent calculation)
- `src/desktop/moksha.rs` (Wallpaper transitions & active background state)
- `src/desktop/ultimate_distro_desktop.rs` (Wallpaper accent color extraction)
