# SigmaOS Roadmap: PipeWire-Equivalent Audio Server
Low-latency audio server replacing ALSA/PulseAudio.
## Goals
- Graph-based audio routing (sigma-pipe)
- <5ms round-trip latency on ALSA hardware
## Key Milestones
- [ ] ALSA PCM driver integration in HAL
- [ ] Graph scheduler with real-time priority
- [ ] JACK compatibility shim