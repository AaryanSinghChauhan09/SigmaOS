# SigmaOS Roadmap: Yi 6B Long-Context Engine
Enable large document understanding inside Zenith apps using Yi 6B.
## Goals
- Support up to 64K token contexts.
- FlashAttention-2 CPU implementation for fast long-context computation.
## Key Milestones
- [ ] FlashAttention kernel in Rust
- [ ] Rotary position embedding scaling
- [ ] Context caching engine