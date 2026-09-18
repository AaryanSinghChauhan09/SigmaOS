# AI Agent Audio & Sound Subsystem Maintenance & Development Guidelines

This document provides engineering standards, API compatibility rules, and maintenance protocols for AI agents developing and maintaining the sound and audio processing subsystem in SigmaOS.

---

## 1. Subsystem Overview & Core Architecture

SigmaOS implements a unified audio processing pipeline supporting PipeWire, PulseAudio, and ALSA API emulation, low-latency DSP node graph routing, PC speaker internal tone generation, and USB Video Class (UVC) webcam audio stream capture.

### Core Modules:
- `src/audio/` / `src/audio/pipewire.rs`: Real-time DSP node graph audio mixer.
- `src/audio/alsa_compat.rs`: ALSA `/dev/snd/pcm*` emulation layer.
- `src/drivers/distro_device_expansion.rs`: `PcSpeakerInternalAudioDriver` and `UvcWebcamVideoCameraDriver`.

---

## 2. Maintenance & Development Guidelines for AI Agents

### 2.1 Low-Latency DSP Graph & Buffer Rules
1. **Real-Time Thread Invariants:** Real-time audio threads must execute with lock-free ring buffers (`AudioRingBuffer`) and strictly avoid heap allocations, disk I/O, or blocking lock acquisitions.
2. **Sample Format Conversion:** PCM audio buffers must support $16$-bit signed integer and $32$-bit floating point PCM formats with dynamic sample rate conversion ($44.1 \text{ kHz} \leftrightarrow 48 \text{ kHz} \leftrightarrow 96 \text{ kHz}$).

### 2.2 Driver Integration Standards
1. **PC Speaker Audio Driver:** Tone frequencies must be validated between $20 \text{ Hz}$ and $20,000 \text{ Hz}$ before programming timer channel 2 registers.
2. **UVC Audio Streams:** USB Video Class webcam audio descriptors must parse ISOCHRONOUS endpoint descriptors and buffer audio samples cleanly without frame drops.

### 2.3 Diagnostic Protocol & Self-Verification
AI agents must verify sound subsystem modifications using the following test runner commands:
```bash
# Verify audio subsystem expansion drivers
cargo test --lib drivers::distro_device_expansion::tests::test_pc_speaker_internal_audio_driver

# Run full test suite
./run_sigma_tests.sh
```

---

## 3. Safe Rust Code Blueprints

### 3.1 Lock-Free Real-Time Audio Buffer Blueprint
```rust
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct AudioPcmRingBuffer<const N: usize> {
    pub buffer: [f32; N],
    pub read_ptr: AtomicUsize,
    pub write_ptr: AtomicUsize,
}

impl<const N: usize> AudioPcmRingBuffer<N> {
    pub const fn new() -> Self {
        Self {
            buffer: [0.0; N],
            read_ptr: AtomicUsize::new(0),
            write_ptr: AtomicUsize::new(0),
        }
    }

    pub fn available_samples(&self) -> usize {
        let w = self.write_ptr.load(Ordering::Relaxed);
        let r = self.read_ptr.load(Ordering::Relaxed);
        w.wrapping_sub(r)
    }
}
```

---

*Verified & Enforced for SigmaOS AI Agents.*
