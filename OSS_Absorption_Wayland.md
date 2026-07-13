# OSS Absorption: Wayland Protocol & Compositor Stack

> **Status**: 🔄 Active | **Source Projects**: Wayland, Weston, wlroots, Sway, Mutter (GNOME) | **Target Shard**: `Zenith Display Server`

---

## 1. Executive Summary

The **Zenith Display Server** is SigmaOS's Wayland compositor — the single component that manages all screen rendering, input routing, and window management. It is built on the `wlroots` library (the foundation of compositors like Sway, Hyprland, and KDE's KWin port) and implements the full Wayland core protocol plus all standard extensions.

The Zenith compositor replaces X11 entirely, providing a modern, GPU-accelerated, security-isolated display architecture where applications cannot spy on each other's windows or keystrokes.

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    ZENITH DISPLAY STACK                          │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────────────────┐ │
│  │  Firefox    │  │  Terminal   │  │  GIMP (XWayland)         │ │
│  │  (Wayland)  │  │  (Wayland)  │  │  X11 app via XWayland    │ │
│  └──────┬──────┘  └──────┬──────┘  └────────┬─────────────────┘ │
│         │                │                   │                   │
│         └────────────────┼───────────────────┘                   │
│                          │ Wayland Socket (wl_display)           │
│  ┌───────────────────────▼──────────────────────────────────┐    │
│  │                ZENITH COMPOSITOR                         │    │
│  │  Scene Graph   │  Input Router  │  Output Manager        │    │
│  │  (wlr-scene)   │  (seat/libinput)│  (DRM/KMS)           │    │
│  └───────────────────────┬──────────────────────────────────┘    │
│                          │                                       │
│  ┌───────────────────────▼──────────────────────────────────┐    │
│  │          GPU RENDERING BACKEND (Mesa + Vulkan)           │    │
│  │  OpenGL ES 3.2   │   Vulkan 1.3   │   DRM KMS           │    │
│  └──────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Wayland Protocol Extensions Implemented

### 3.1 Core Protocol + Stability Extensions

| Protocol | Source | Purpose |
|:---------|:-------|:--------|
| `wayland-core` | Wayland project | Base window/surface/input protocol |
| `xdg-shell` | freedesktop.org | Application window management |
| `xdg-output` | freedesktop.org | Output/monitor information |
| `xdg-decoration` | freedesktop.org | Server-side window decorations |
| `wl-drm` | Mesa | DRM buffer sharing for zero-copy rendering |
| `linux-dmabuf` | freedesktop.org | GPU buffer import/export |
| `presentation-time` | Wayland | Frame timing for smooth animations |
| `viewporter` | Wayland | Scaling/cropping hint to compositor |

### 3.2 Additional Extensions (Zenith-specific)

| Protocol | Inspired By | Purpose |
|:---------|:-----------|:--------|
| `sigma-layer-shell` | wlr-layer-shell | Panels, docks, wallpapers, OSD |
| `sigma-screencopy` | wlr-screencopy | Screen recording/screenshot |
| `sigma-foreign-toplevel` | wlr-foreign-toplevel | Taskbar/alt-tab window listing |
| `sigma-gamma-control` | wlr-gamma-control | Night mode/colour temperature |
| `sigma-output-management` | wlr-output-management | Multi-monitor setup |
| `sigma-virtual-keyboard` | Weston | On-screen keyboard for tablets |
| `sigma-pointer-gestures` | libinput | Multi-touch gesture handling |

### 3.3 Rust Implementation (Zenith Core)

```rust
// zenith/compositor/mod.rs
// SPDX-License-Identifier: MIT

use smithay::{   // Rust Wayland compositor framework
    backend::drm::DrmDevice,
    desktop::{Space, Window},
    input::{Seat, SeatHandler},
    wayland::compositor::CompositorState,
};

pub struct ZenithCompositor {
    pub display:     Display<ZenithState>,
    pub space:       Space<Window>,
    pub seat:        Seat<ZenithState>,
    pub drm:         DrmDevice,
    pub gpu:         VulkanRenderer,
    pub scene_graph: SceneGraph,
}

impl ZenithCompositor {
    /// Render one frame to all connected outputs
    pub fn render_frame(&mut self) -> Result<()> {
        for output in self.space.outputs() {
            let damage = self.scene_graph.compute_damage(output);
            if damage.is_empty() { continue; }  // Skip if nothing changed

            self.gpu.begin_frame(output)?;
            self.scene_graph.render(&mut self.gpu, output, &damage)?;
            self.gpu.present(output)?;
            self.send_frame_callbacks(output);
        }
        Ok(())
    }

    /// Handle new application window creation
    fn on_new_surface(&mut self, surface: WlSurface) {
        let window = Window::new_wayland_window(surface);
        // Apply security policy: sandboxed app → restricted capabilities
        let caps = self.get_window_capabilities(&window);
        self.space.map_element(window, Position::default(), caps);
    }
}
```

---

## 4. XWayland Compatibility

Legacy X11 applications run through `sigma-xwayland` with zero user action required:

```bash
$ sigma run gimp         # GIMP is X11 — auto-launched via XWayland
Σ [XWAYLAND] Starting XWayland for legacy X11 application: gimp
Σ [INFO] DISPLAY=:1 set for this application
  (GIMP windows appear like any Wayland window to the compositor)
```

XWayland isolation: each X11 app gets its own isolated X server instance, so X11 apps cannot read keyboard input or spy on other windows (a critical security improvement over traditional X11 arrangements).

---

## 5. GPU Rendering Backend

```bash
# Check Zenith GPU backend status
$ sigma display status
Σ [ZENITH] Display Server Status:
  Compositor     : Zenith 1.0 (Wayland)
  Renderer       : Vulkan 1.3 (Mesa 24.0 — RADV, AMD RX 6700)
  VRAM           : 12GB GDDR6  (used: 890MB)
  Outputs:
    HDMI-1       : 2560x1440@144Hz   (primary, HDR10 enabled)
    DP-1         : 1920x1080@60Hz    (secondary)
  XWayland       : Active (:1)
  Frame timing   : 144fps / 6.9ms average

# Enable Night Mode (reduces blue light at night)
$ sigma display night-mode --temp 3500K --from 22:00 --to 07:00
```

---

## 6. References & Standards

- Wayland Protocol — `wayland.freedesktop.org` (MIT)
- wlroots — `gitlab.freedesktop.org/wlroots/wlroots` (MIT)
- Smithay (Rust compositor framework) — `github.com/Smithay/smithay` (MIT)
- Mesa OpenGL/Vulkan — `mesa3d.org` (MIT)
- libinput — `wayland.freedesktop.org/libinput` (MIT)
