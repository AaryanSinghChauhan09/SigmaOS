# PrintCraft: Professional Desktop Publishing (DTP)

PrintCraft handles multi-page magazine, book, and brochure page layouts with professional prepress and CMYK color separation.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Adobe InDesign, QuarkXPress, Scribus
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct PageMargin { pub top: f32, pub bottom: f32, pub left: f32, pub right: f32 }
pub struct TextFrame { pub frame_id: u64, pub next_linked_frame: Option<u64>, pub text_content: String }
```

---

## ✨ Features & Capabilities
- **Multi-column text threading across non-contiguous frames**
- **Prepress CMYK plate separation and spot color Pantone tables**
- **Bleed, slug, and trim crop mark generation**
- **High-resolution PDF/X-1a and PDF/X-4 print output**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
