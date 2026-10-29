# PhotoCraft: Non-Destructive RAW Photo Processor

PhotoCraft develops digital camera RAW images with high dynamic range processing, demosaicing, and color calibration.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Adobe Lightroom, Darktable, RawTherapee
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct RawMetadata { pub camera_make: String, pub camera_model: String, pub iso: u32, pub shutter_speed: f64 }
pub struct ToneCurvePoint { pub input: f32, pub output: f32 }
```

---

## ✨ Features & Capabilities
- **Non-destructive pipeline keeping original RAW sensor bytes pristine**
- **High-fidelity demosaicing (AHD, AMAZE, RCD algorithms)**
- **Lens chromatic aberration, distortion, and vignetting correction**
- **Color tone curves with separate Luma and RGB channels**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
