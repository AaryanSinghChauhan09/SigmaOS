# DeckCraft: Presentation & Slide Design Suite

DeckCraft delivers clean, typography-focused presentation slides with smooth vector animations and presenter display.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Apple Keynote, Microsoft PowerPoint
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub enum DeckExportFormat { Pdf, SvgSlides, Html5, ZenithNative }
pub struct SlideElement { pub id: u64, pub x: f32, pub y: f32, pub width: f32, pub height: f32, pub z_index: u32 }
```

---

## ✨ Features & Capabilities
- **Fluid slide transitions powered by GPU shader pipeline**
- **Presenter screen with elapsed timer, speaker notes, and preview**
- **Direct vector asset embedding from VectorCraft and FigCraft**
- **PDF and interactive HTML5 self-contained export**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
