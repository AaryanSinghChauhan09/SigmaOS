# AniCraft: 2D Animation & Motion Studio

AniCraft provides vector and frame-by-frame 2D animation with skeletal rigging, inverse kinematics, and onion skinning.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: OpenToonz, Adobe Animate, Moho
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct CubicBezier { pub p0: f32, pub p1: f32, pub p2: f32, pub p3: f32 }
pub struct Keyframe { pub frame_number: u64, pub value: f32, pub easing: CubicBezier }
pub struct IkJoint2D { pub position: [f32; 2], pub length: f32, pub angle_radians: f32 }
```

---

## ✨ Features & Capabilities
- **Multi-layer timing chart and frame sequencer**
- **Configurable onion skinning with forward/backward tinting**
- **2D inverse kinematics (IK) bone rigging**
- **Vector path tweening with cubic Bézier acceleration**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
