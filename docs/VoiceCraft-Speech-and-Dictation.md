# VoiceCraft: Speech Synthesis, Audio Dictation & Voice Control

VoiceCraft provides real-time on-device speech recognition and neural text-to-speech with zero cloud dependencies.

---

## 🎯 Parity Targets & Inspiration
- **Primary Targets**: Whisper AI, ElevenLabs, Apple Dictation
- **Architectural Philosophy**: Built in `#![no_std]` Rust; zero telemetry, zero electron wrappers, sub-millisecond input response, native shared memory pipeline with the Zenith compositor.

---

## 🏛️ Key Structures & Types
```rust
pub struct VoiceSynthesisOptions { pub pitch_multiplier: f32, pub speed_multiplier: f32, pub volume: f32 }
pub struct DictationResult { pub text: String, pub confidence: f32, pub timestamp_start_ms: u64 }
```

---

## ✨ Features & Capabilities
- **On-device Whisper neural speech recognition with low latency**
- **Natural expressive text-to-speech synthesis engine**
- **System-wide speech dictation in all text inputs (`Super + D`)**
- **Customizable voice command trigger actions**

---

## ⚡ Integration with SigmaOS
This module shares the unified `CraftLibs` foundation, allowing direct drag-and-drop asset sharing with other Craft applications, instant theme adaptation, and GPU-accelerated drawing.
