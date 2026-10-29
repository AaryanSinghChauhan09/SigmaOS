# CraftLibs: Unified Creative Framework & Color Engine

CraftLibs provides shared math, color management, and memory primitives across all 20 Craft Suite applications.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Skia Graphics Engine, Cairo, GEGL
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub enum ColorSpace { Srgb, DisplayP3, AdobeRgb, CmykSwop, LinearRgb }
pub struct ColorTransformMatrix { pub matrix: [[f32; 4]; 4] }
pub struct CanvasBuffer { pub width: u32, pub height: u32, pub stride: u32, pub data_ptr: usize }
```

---

## ✨ Features & Capabilities
- **High-speed ICC color management and gamut mapping**
- **SIMD-accelerated alpha blending and format conversion**
- **Lock-free undo/redo DAG history graph**
- **Shared zero-copy Wayland/Zenith memory sharing handles**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
