# Audio Subsystem & PipeWire Engine Component Agent

## Component Overview
Governs low-latency digital audio routing, software mixing, ALSA/PipeWire compatibility, and hardware codecs (Intel HDA, USB Audio, AC97).

## Linux Inspiration
Linux PipeWire graph-based multimedia routing, ALSA PCM interface, JACK low-latency audio server.

## BSD Inspiration
OpenBSD sndio audio framework (ultra-low latency and simple API), FreeBSD OSS (Open Sound System) v4.

## Current SigmaOS Status
Intel HDA driver initialized, basic PCM playback and software audio mixer functional. Multi-stream graph routing and microphone capture need completion.

## Critical Missing Features
PipeWire-compatible node graph routing engine, sub-2ms audio buffer scheduling, hardware mixer volume controls, and real-time noise suppression.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/audio/mixer.rs, src/audio/intel_hda.rs, src/audio/pcm.rs, src/audio/graph.rs

## Testing Strategy
Audio loopback latency tests, sample rate conversion fidelity tests, high-load buffer underrun stress suite.

## Dependencies
HARDWARE_HAL_DRIVERS_AGENT, INTERRUPT_APIC_AGENT, TIMEKEEPING_AGENT

## Success Criteria
Sub-3ms round-trip audio latency, zero audio crackle/glitch under 100% CPU load, automatic seamless headphone jack detection.

## Open Source Competitors Analysis
Combines the bulletproof stability of OpenBSD sndio with the versatile graph routing capabilities of Linux PipeWire.

## Future Enhancements
Spatial 3D audio HRTF synthesis directly processed in the OS audio engine.
