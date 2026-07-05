# SigmaOS Roadmap: State-Space Model (SSM) Hardware Accelerators
Interface Mamba SSM models directly with SIMD vector instructions on CPU/GPU.
## Goals
- Leverage hardware-level parallelism for selective scan operations.
- Under 1ms latency for processing log window buffers.
## Key Milestones
- [ ] AVX2/NEON intrinsics selective scan
- [ ] Model architecture config parser
- [ ] Execution benchmark comparison