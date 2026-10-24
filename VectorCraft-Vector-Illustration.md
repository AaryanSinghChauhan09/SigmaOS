# VectorCraft: Scalable Vector Graphics Studio

VectorCraft provides precision vector illustration with path Boolean operations, gradient meshes, and SVG2 compatibility.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Adobe Illustrator, Inkscape, Affinity Designer
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct BezierPathNode { pub point: [f32; 2], pub control_in: [f32; 2], pub control_out: [f32; 2] }
pub struct GradientMeshPatch { pub control_grid: [[f32; 2]; 16], pub corner_colors: [[u8; 4]; 4] }
```

---

## ✨ Features & Capabilities
- **Bézier path Boolean operations (Union, Intersection, Difference, Exclusion)**
- **Multi-stop linear, radial, and multi-point gradient meshes**
- **Pixel-perfect grid alignment and dynamic angle snapping**
- **Clean W3C SVG2 import, editing, and lossless export**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
