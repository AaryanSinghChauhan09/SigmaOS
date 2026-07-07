# SigmaOS Zenith Desktop Specification

## Overview
Zenith Desktop is the native, Wayland-first desktop environment for SigmaOS. Built with performance and modern aesthetics in mind, Zenith features a hardware-accelerated compositor, native screen reading APIs, and deep localization with out-of-the-box support for major Indic languages.

## Compositor and Accessibility Flow
```
 [User Input (Wayland Events)]
               │
               ▼
   [Zenith Wayland Compositor] ◄──► [Accessibility screen reader daemon]
               │
               ▼
   [Indic Language Input IM] (IBus/Fcitx API)
               │
               ▼
   [Vulkan Desktop Layout Renderer]
```

## System Properties
Zenith compositor configurations are defined in `/etc/zenith/compositor.conf`:
```toml
[compositor]
renderer = "vulkan"
vsync = true
scaling = "hidpi"

[accessibility]
screen_reader = true
default_locale = "hi_IN" # Hindi (India)
input_method = "ibus-m17n"
```

## Technical Implementation
The compositor uses direct Vulkan rendering to draw layout components on the screen, skipping heavy server layers.

```rust
// userland/apps/zenith-compositor/src/compositor.rs
pub struct ZenithCompositor {
    pub vk_device: ash::Device,
    pub swapchain: ash::extensions::khr::Swapchain,
    pub screen_reader_active: bool,
}

impl ZenithCompositor {
    pub fn draw_desktop_elements(&self) -> Result<(), CompositorError> {
        // GPU accelerated composite rendering of panels, taskbars and windows
        self.render_panels()?;
        if self.screen_reader_active {
            self.announce_accessibility_focus();
        }
        Ok(())
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Core Wayland compositor protocols and Vulkan backend rendering.
- **Phase 2 (Months 3-6)**: IBus integration for Indic language text processing and layout engines.
- **Phase 3 (Months 6-9)**: Integrated speech synthesizer and screen reading engine.
- **Phase 4 (Months 9-12)**: Gestural accessibility control mapping and multi-display color profiles.
