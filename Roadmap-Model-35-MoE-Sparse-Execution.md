# SigmaOS Roadmap: Mixture-of-Experts Sparse Weight Loader
Serve massive MoE models by dynamically loading only active expert weights from storage.
## Goals
- Zero-copy weight mapping using VFS mmap adapters.
- Maintain minimal RAM footprint for multi-expert configurations.
## Key Milestones
- [ ] Memory-mapped expert parameter registry
- [ ] Routing prediction cache
- [ ] Low-latency expert swap scheduler