# SigmaOS Creative & Design Tools Suite

## Overview
SigmaOS aims to be a complete creative workstation, offering out-of-the-box alternatives to industry-standard commercial creative suites (such as Adobe, Autodesk, and Figma). The OS natively packages and accelerates vector graphics (Inkscape), raster painting (Krita), 3D modeling/rendering (Blender), video/audio editing (Kdenlive, LMMS, Ardour), and UI/UX workspace design (Penpot).

## Design System & GPU Acceleration
Natively integrated graphics packages use custom compositor plugins for Zenith Desktop, routing UI frames through specialized low-overhead Vulkan pipelines.

```
 [Vector / Raster / 3D App]
             │
             ▼
   [Custom Vulkan Pipeline]
             │
             ▼
   [Zenith Compositor] ◄──► [Low-Latency Audio (Ardour/LMMS)]
             │
             ▼
      [Display Output]
```

## System Properties
Design configurations and path variables are configured in `/etc/sigma/creative.conf`:
```toml
[creative]
vulkan_acceleration = true
default_color_space = "sRGB"
audio_backend = "pipewire"

[penpot]
local_server_enabled = true
port = 8080
```

## Technical Implementation
SigmaOS uses memory-safe graphics buffers to translate native image objects between creative tools and Zenith Desktop widgets.

```rust
// userland/apps/sigma-studio/src/main.rs
pub struct CreativeCanvas {
    pub width: u32,
    pub height: u32,
    pub buffer: *mut u8,
}

impl CreativeCanvas {
    pub fn render_frame(&self) -> Result<(), RenderError> {
        // Direct Vulkan swapchain blit for low-latency painting/modeling
        unsafe {
            blit_canvas_to_compositor(self.buffer, self.width, self.height)?;
        }
        Ok(())
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Package compilation of Blender and Krita optimized with PGO (Profile-Guided Optimization) and SIMD for SigmaOS.
- **Phase 2 (Months 3-6)**: Wayland compositor integrations for tablet pressure sensitivity and multi-monitor color profiles.
- **Phase 3 (Months 6-9)**: Local Penpot container pre-configuration for offline UI design.
- **Phase 4 (Months 9-12)**: Open-source asset management suite and project pipeline version control.
