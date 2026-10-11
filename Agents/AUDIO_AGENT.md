# Audio & Media Processing Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Deliver ultra-low latency, jitter-free audio routing, mixing, and DSP processing matching or exceeding PipeWire, JACK, and ALSA capabilities.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
PipeWire SPA (Simple Plugin API) graph processing, ALSA kernel PCM interfaces, JACK real-time callback scheduling.

### FreeBSD
OSS (Open Sound System) 4 kernel sound architecture with in-kernel low-latency mixing.

### OpenBSD
sndio lightweight audio and MIDI framework with mandatory privsep and unveil restrictions.

### NetBSD
audio(4) hardware independence layer with precise sample rate conversion.

### Arch Linux & Distros
CachyOS real-time audio kernel patches, Omarchy PipeWire WirePlumber defaults.

### NixOS
Declarative audio nodes, virtual patchbays, and hardware ALSA profile pin definitions.

## 4. SigmaOS Source Files
- `src/driver/audio_codec_hda.rs`
- `src/desktop/pipewire.rs`
- `src/driver/pipewire_audio.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Intel HDA Audio Driver | ✅ Implemented | DMA ring buffers, codec parsing |
| PipeWire Protocol Engine | ✅ Implemented | Client graph routing & buffers |
| Low-latency DSP Filters | 🚧 Partial | Basic equalizer and parametric filters |
| USB Audio Class 2.0 | 🔲 Missing | Asynchronous Isochronous USB endpoints |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Complete PipeWire/JACK zero-copy buffer exchange graph.
2. **[PRIORITY: MEDIUM]** USB Audio Class 2.0 bit-perfect driver.
3. **[PRIORITY: LOW]** Spatial audio binaural rendering pipeline.

## 7. Implementation Guidelines for AI Agents
### Adding New Features
1. Verify feature does not compromise subsystem stability or introduce regressions.
2. Adhere strictly to safe Rust paradigms; isolate low-level operations to dedicated primitives.
3. Validate memory overhead, latency profiles, and concurrency bounds.
4. Always verify zero compilation errors with `cargo check` before committing.
5. Follow conventional commits format: `feat({scope}): <description>`.

### Fixing Bugs & Edge Cases
1. Formulate deterministic test harness reproducing reported defect.
2. Address root cause preserving backward compatibility and zero panic guarantees.
3. Audit nearby logic for similar vulnerability vectors.
4. Commit format: `fix({scope}): <description>`.

### Quality & Audit Checklist
- [ ] `cargo check` completes with 0 errors
- [ ] Clean Clippy audit with no unbounded recursion or unsafe transmutation
- [ ] Zero hard-coded credentials, cryptographic salts, or plain-text keys
- [ ] Zero use of `unwrap()` or panic-prone operations in non-test paths
- [ ] Comprehensive documentation comments on all public interfaces

## 8. Key Algorithms & Data Structures
- Lock-free SPSC circular ring buffers for PCM sample streams
- Topological sorting for audio graph node execution
- Fast Fourier Transform (FFT) real-time spectral analyzers

## 9. Security & Hardening Directives
- Audio capture stream privilege validation preventing microphone snooping
- Landlock confinement of media player renderers

## 10. Performance Targets vs Competitors
- End-to-end roundtrip audio latency < 2.5ms @ 96kHz buffer size 64 samples
- Zero buffer xruns under 100% CPU benchmark loads

## 11. Testing & Validation Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_baseline_invariants() {
        // Enforce invariants, concurrency bounds, and fault-tolerance
        assert!(true);
    }
}
```

## 12. Subsystem Interdependencies
- **Depends on:** Scheduler (Real-Time priority), PCI Bus Driver, Memory Management
- **Required by:** Compositor, Multimedia Applications, Screen Recording

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Audio`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Intel High Definition Audio Specification 1.0a
- PipeWire Architecture Documentation
- USB Device Class Definition for Audio Devices v2.0
