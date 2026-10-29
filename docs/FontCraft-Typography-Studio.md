# FontCraft: Vector Font Design & Typography Studio

FontCraft allows type designers to create vector glyphs, define OpenType features, and compile installable fonts.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Glyphs, FontForge, RoboFont
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct GlyphContour { pub points: Vec<[f32; 2]>, pub is_closed: bool }
pub struct KerningPair { pub left_glyph: u16, pub right_glyph: u16, pub kerning_offset: i16 }
```

---

## ✨ Features & Capabilities
- **Bézier curve outline drawing with smooth tangent snapping**
- **OpenType layout table compiler (GSUB, GPOS, kern)**
- **TrueType bytecode hinting and grid-fitting previews**
- **Direct TTF, OTF, and WOFF2 generation**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
