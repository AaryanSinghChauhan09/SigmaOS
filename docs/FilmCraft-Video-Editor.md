# FilmCraft: Non-Linear Video Editor (NLE)

FilmCraft provides multi-track video editing, proxy generation, audio synchronization, and color grading.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: DaVinci Resolve, Adobe Premiere Pro, Kdenlive
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct VideoTrack { pub track_id: u32, pub is_muted: bool, pub is_locked: bool }
pub struct TimelineClip { pub clip_id: u64, pub start_time_ms: u64, pub duration_ms: u64, pub source_path: String }
```

---

## ✨ Features & Capabilities
- **Zero-drop-frame multi-track 4K/8K timeline playback**
- **Automatic multicam audio waveform alignment**
- **3D LUT color grading and HDR waveform monitors**
- **Hardware video decoding via VA-API, NVDEC, and AMF**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
