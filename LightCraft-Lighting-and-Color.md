# LightCraft: Lighting Design, Stage Control & Color Grading

LightCraft provides stage lighting design, DMX512 universe control, and color temperature mastering.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: ETC Eos, Capture Visualizer, DaVinci Color
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct DmxChannelValue { pub universe: u16, pub channel: u16, pub value: u8 }
pub struct FixturePatch { pub fixture_id: u32, pub name: String, pub start_address: u16 }
```

---

## ✨ Features & Capabilities
- **DMX512 and Art-Net network lighting protocol dispatch**
- **Color temperature (Kelvin) and chromaticity coordinate adjustment**
- **Preset cue stack sequencer with crossfade timing**
- **Live hardware controller MIDI/OSC integration**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
