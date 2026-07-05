# SigmaOS Roadmap: Direct Speech-To-Intent Parsing
Bypass intermediary text transcripts by mapping raw audio waveforms directly to OS intent commands.
## Goals
- Real-time end-to-end neural network translating voice directly to JSON structured actions.
- Target latency under 100ms on desktop-grade CPUs.
## Key Milestones
- [ ] Audio feature extraction (Log-Mel spectrograms in zero-alloc Rust)
- [ ] Custom lightweight sequence-to-intent model loader
- [ ] Intent dispatch loop routing directly to VFS or IPC