# EffectCraft: Node-Based Visual Effects & Motion FX

EffectCraft is a high-performance node-based compositor for motion graphics, VFX, and post-production.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Adobe After Effects, Foundry Nuke, Blackmagic Fusion
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub enum EffectNodeCategory { Generator, ColorCorrection, BlurSharpen, Keying, Transform }
pub struct OpticalFlowVector { pub dx: f32, pub dy: f32, pub confidence: f32 }
```

---

## ✨ Features & Capabilities
- **Node graph architecture with infinite branching and caching**
- **Green screen Chroma/Luma keying with spill suppression**
- **Optical flow motion vector estimation and frame interpolation**
- **High-performance particle engine with gravity and wind fields**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
