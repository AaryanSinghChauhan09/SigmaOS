# SigmaOS Roadmap: Memory-Safe Driver Framework
All device drivers written in safe Rust with formal verification stubs.
## Goals
- No unsafe in driver hot paths
- Kani verifier proofs for critical drivers
## Key Milestones
- [ ] Safe DMA abstraction layer
- [ ] Kani proofs for NVMe queue management
- [ ] MMIO bounds-check wrapper type