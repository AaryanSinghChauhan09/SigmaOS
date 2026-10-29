# ArtCraft: Digital Painting & Concept Art Studio

ArtCraft offers a dynamic, latency-optimized raster painting engine with pressure curves, tilt support, and physical brush simulations.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Krita, Procreate, Adobe Photoshop
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct BrushDab { pub x: f32, pub y: f32, pub pressure: f32, pub tilt_angle: f32, pub color_rgba: [u8; 4] }
pub struct PaintLayer { pub name: String, pub opacity: f32, pub blend_mode: u8, pub is_visible: bool }
```

---

## ✨ Features & Capabilities
- **Sub-millisecond stylus input latency via direct event ring buffer**
- **Wet-on-wet paint mixing and dynamic watercolor diffusion**
- **Unlimited non-destructive layer stack with mask clipping**
- **PSD, ORA, and PNG raster format compatibility**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
