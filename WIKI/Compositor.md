# Compositor

SigmaOS ships a Wayland-native compositor written entirely in Rust, designed to outperform Mutter (GNOME/Mint), KWin, and Hyprland's Wayland compositor on latency, frame consistency, and power efficiency.

---

## Architecture Overview

```
 ┌──────────────────────────────────────────────────────┐
 │                Wayland Clients (Apps)                 │
 └────────────────────┬─────────────────────────────────┘
                      │ Wayland protocol (wl_compositor)
 ┌────────────────────▼─────────────────────────────────┐
 │             SigmaCompositor Core                      │
 │  Surface Manager │ Scene Graph │ Damage Tracking       │
 │  Input Router    │ XWayland    │ Protocol Extensions   │
 └────────────────────┬─────────────────────────────────┘
                      │ DRM/KMS
 ┌────────────────────▼─────────────────────────────────┐
 │           GPU / Display Engine                        │
 │  DRM Lease │ Atomic Modesetting │ Direct Scanout      │
 └──────────────────────────────────────────────────────┘
```

---

## Components

### 1. Surface Manager (`src/compositor/surface.rs`)
- wl_surface lifecycle: attach, damage, commit
- Buffer management: SHM + DMA-BUF
- Subsurfaces and surface roles

### 2. Scene Graph
- Retained-mode scene tree
- Damage-region-aware: only re-composite changed areas
- Layer shell (wlr-layer-shell) for panels, wallpapers, overlays

### 3. Window Manager
- Tiling layouts: BSP, master-stack, grid
- Floating mode with magnetic snapping
- Gaps, borders, animations (configurable easing curves)
- Workspace system: 10 virtual desktops per monitor

### 4. Input Routing (`src/input/`)
- libinput backend for keyboard, mouse, touch, stylus
- Keyboard shortcuts remapping in Rust config (no Lua required)
- Per-window input focus with pointer constraint support

### 5. XWayland Bridge
- Seamless X11 app support via XWayland
- DRI3 for hardware-accelerated X clients
- Clipboard and drag-and-drop interop

### 6. Direct Scanout
- Bypasses compositor for fullscreen apps (games, video)
- Sub-1ms frame delivery for 144Hz/240Hz panels
- Explicit sync with `drm_syncobj` (no tearing)

### 7. Protocol Extensions
- `xdg-shell`: standard app window management
- `xdg-output`: multi-monitor layout
- `wp-presentation-time`: precise frame timing
- `sigma-ext-*`: SigmaOS-specific extensions for AI overlays

---

## Performance Targets

| Metric | Mutter | KWin | Hyprland | **SigmaCompositor** |
|--------|--------|------|----------|---------------------|
| Input latency | ~4 ms | ~3 ms | ~2 ms | **< 1.5 ms** |
| Frame consistency (σ) | 0.8 ms | 0.6 ms | 0.4 ms | **< 0.3 ms** |
| GPU idle power | 8W | 7W | 5W | **< 4W** |
| 4K@60Hz CPU usage | 12% | 9% | 6% | **< 4%** |

---

## Configuration

Configuration is in TOML (no Lua, no XML):

```toml
[compositor]
backend = "drm"          # or "winit" for VM/testing
vsync = true
direct_scanout = true

[layout]
mode = "bsp"             # bsp | master-stack | grid | float
gaps_inner = 6
gaps_outer = 12
border_width = 2
border_color = "#8be9fd"

[animations]
enable = true
duration_ms = 180
easing = "ease-out-cubic"
```

---

## Comparison vs Linux Desktop Compositors

| Feature | Mutter | KWin | Hyprland | **SigmaCompositor** |
|---------|--------|------|----------|---------------------|
| Language | C | C++ | C++ | **Rust** |
| Wayland-native | ✅ | ✅ | ✅ | ✅ |
| Direct scanout | ✅ | ✅ | ✅ | ✅ |
| Tiling | ❌ | Plugin | ✅ | ✅ Built-in |
| AI overlays | ❌ | ❌ | ❌ | ✅ |
| Config language | GSettings | KConfig | hyprlang | **TOML** |

---

## Source Files

| File | Description |
|------|-------------|
| `src/compositor/` | Compositor core |
| `src/desktop/` | Desktop shell, panels, launcher |
| `src/graphics/` | GPU abstraction layer |
| `src/input/` | Input routing |
| `src/gpu/` | DRM/KMS, GPU memory |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/compositor/`, `src/desktop/`, `src/graphics/`, `src/gpu/`
> - Update performance table when benchmarks are run against new compositor versions
> - Add new protocol extensions as they are implemented
> - Keep direct scanout / explicit sync status current
