# SigmaOS Window Manager & Compositor Specification (`Zenith Compositor`)

## 1. Executive Summary

The Zenith Compositor combines window management logic with hardware-accelerated 3D compositing for SigmaOS. It handles window placement, window decoration framing, visual effects (drop shadows, gaussian blur, animations, acrylic transparency), multi-monitor workspace layout, and hardware display scanout.

## 2. Window Management Modes & Policies

Zenith supports hybrid window management workflows, seamlessly switching between tiling and floating paradigms:

### 2.1 Dynamic Tiling Engine
- **Master-Stack Layout**: One primary window occupies the main area, with remaining windows stacked vertically on the secondary pane.
- **Binary Space Partitioning (BSP)**: Automatic recursive splits along horizontal and vertical axes.
- **Grid / Columns Layout**: Equal-width vertical column layout tailored for ultra-wide displays.
- **Auto-Gaps & Margins**: Configurable inner and outer gaps with per-workspace layout rules.

### 2.2 Floating & Stacking Engine
- **Z-Order Management**: Windows are organized in discrete depth layers:
  1. **Background / Wallpaper Layer**
  2. **Bottom Desktop Widgets Layer**
  3. **Normal Application Layer**
  4. **Always-on-Top / Floating Layer**
  5. **Panel / System Bar Layer**
  6. **Overlay / Notification / OSD Layer**
  7. **Lock Screen / Security Prompt Layer**
- **Window Snapping**: Interactive edge snapping and quarter/half screen grid snap zones.

## 3. Rendering Architecture & Visual Effects Pipeline

```
+-----------------------------------------------------------+
|               Zenith Compositor Render Pipeline           |
+-----------------------------------------------------------+
| 1. Surface Damage Accumulation & Region Clipping          |
| 2. Offscreen Texture Pass (Client Surfaces)               |
| 3. Shader Effects Pass (Blur, Acrylic, Drop Shadow)       |
| 4. Frame Decoration Framing & Titlebars                   |
| 5. Workspace / Transition Animations                      |
| 6. DRM/KMS Hardware Plane Scanout (Direct DRM Primary)    |
+-----------------------------------------------------------+
```

### 3.1 Hardware Acceleration & Shaders
- **OpenGL ES 3.2 / Vulkan / DRM Backend**: Compositing is driven via modern hardware graphics pipelines.
- **Gaussian & Dual-Kawase Blur**: Real-time background blur filters applied to transparent overlay panels and floating windows.
- **Rounded Corner Clipping**: Anti-aliased corner rounding with dynamic shadow casting.
- **Damage Tracking**: Only dirty surface regions are re-rendered, reducing GPU power consumption on static desktop views.

## 4. Multi-Monitor & Fractional Scaling

- **Per-Output Workspaces**: Each physical display manages an independent set of workspaces or unified multi-monitor spans.
- **Fractional Scale Factors**: Supports crisp UI scaling on HiDPI/4K displays (e.g., 125%, 150%, 175%, 200%) using viewport scaling shaders and integer buffer sampling.
- **VRR & Adaptive Sync**: FreeSync / G-Sync variable refresh rate presentation for gaming and video playback surfaces.

## 5. Configuration & Scripting Engine

- **Hot-Reloadable Configuration**: Layout rules, keybindings, animations, and color schemes are defined in declarative configuration files (`~/.config/zenith/zenith.toml`).
- **IPC Scripting Interface**: External scripts and tools control windows using `zenith-ctl` commands (`zenith-ctl focus-window left`, `zenith-ctl set-layout bsp`).
