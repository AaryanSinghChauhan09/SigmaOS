# CADCraft: Parametric 3D Engineering & Solid Modeling

CADCraft is a parametric 2D constraint solver and 3D boundary representation (B-Rep) solid modeling application for mechanical design.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: FreeCAD, SolidWorks, Autodesk Fusion 360
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct NurbsControlPoint3D { pub x: f32, pub y: f32, pub z: f32, pub weight: f32 }
pub enum SketchConstraint { Coincident(u32, u32), Parallel(u32, u32), Perpendicular(u32, u32), Distance(u32, u32, f32) }
```

---

## ✨ Features & Capabilities
- **2D geometric constraint solving (coincident, horizontal, tangent, dimension)**
- **3D CSG operations: Boolean union, difference, and intersection**
- **Parametric feature history tree with live rollback**
- **STEP, IGES, and STL export for 3D printing and CNC**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
