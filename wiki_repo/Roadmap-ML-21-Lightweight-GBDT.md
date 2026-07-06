# SigmaOS Roadmap: Lightweight Gradient Boosted Decision Trees
Implement a memory-gated GBDT implementation designed for microcontrollers and VMs.
## Goals
- Strict memory ceiling constraints on tree construction.
- No dynamic memory allocation during model evaluation.
## Key Milestones
- [ ] Memory-gated training allocator
- [ ] Fixed-point integer tree representations
- [ ] Execution verification under 128KB total RAM