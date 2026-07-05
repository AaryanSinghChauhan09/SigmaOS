# SigmaOS Roadmap: Transparent Memory Compression (zRAM)
Compress cold memory pages in RAM using LZ4 before swapping.
## Goals
- LZ4-compressed swap device in kernel
- Adaptive compression threshold based on memory pressure
## Key Milestones
- [ ] LZ4 streaming compressor (no_std)
- [ ] Virtual swap device driver
- [ ] Memory pressure telemetry integration