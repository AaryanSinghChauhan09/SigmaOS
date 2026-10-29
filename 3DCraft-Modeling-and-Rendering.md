# 3DCraft: 3D Modeling & Rendering Engine

3DCraft provides professional polygon modeling, subdivision surfaces, NURBS curves, and real-time PBR rendering directly integrated into SigmaOS.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Blender, Autodesk Maya, Cinema 4D
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct MeshVertex { pub position: [f32; 3], pub normal: [f32; 3], pub uv: [f32; 2] }
pub struct BvhNode { pub aabb_min: [f32; 3], pub aabb_max: [f32; 3], pub left_child: u32, pub right_child: u32 }
pub struct SceneCamera { pub fov_degrees: f32, pub near_clip: f32, pub far_clip: f32, pub transform_matrix: [[f32; 4]; 4] }
```

---

## ✨ Features & Capabilities
- **Parametric polygon mesh generation and subdivision**
- **BVH ray-triangle intersection acceleration**
- **PBR material shaders with metallic-roughness workflow**
- **OBJ, STL, and glTF 2.0 asset import/export**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
