# 🧩 Cleanroom Absorption: Ardour Audio Studio

SigmaOS integrates a high-performance audio engine, **SigmaSound Studio**, serving as a sovereign alternative to Ardour DAW.

---

## 🎯 Target Architecture: Ardour

Ardour is a professional digital audio workstation (DAW) featuring multi-channel recording, precise non-destructive editing, MIDI tracking, and low-latency audio plugin hosting.

### Gaps in Legacy Ardour:
- Heavy dependency on ALSA, JACK, and manual PipeWire configurations.
- Highly vulnerable to scheduling jitter on standard kernels.

---

## 🎛️ SigmaOS Sovereign Features

### 1. Preemptive Jitter-Free Scheduler
- The microkernel provides real-time high-priority preemptive thread execution with scheduling jitter guarantees of less than 1 microsecond.

### 2. Built-in Stem Isolation
- Leverages our local AI models to isolate vocals, drums, bass, and instruments cleanly on-device with zero user configuration.

### 3. Spatial HRTF Audio
- Automatically renders multi-channel spatial audio into binaural HRTF output for standard consumer headphones.

---

## 📊 Absorption Matrix

| Capability | Ardour DAW | SigmaSound Studio |
|------------|------------|-------------------|
| Multi-channel Recording | ✅ | ✅ |
| Non-destructive Editing | ✅ | ✅ |
| Plugin Support | ✅ (VST3, LV2) | ✅ VST3/LV2 + Rust Native |
| Latency Jitter | Millisecond level | ✅ Sub-microsecond (kernel-level) |
| Audio Stem Isolation | External Plugin | ✅ SovereignML |
| Binaural HRTF Spatialization | ❌ | ✅ Built-in |
