# SigmaOS Roadmap: Falcon Lite 1B Optimizer
Incorporate Falcon Lite 1B as a low-memory OS orchestration fallback.
## Goals
- Optimise attention key-value caching to support multiple concurrent users on minimal RAM.
- Strict sub-2GB RAM allocation boundary.
## Key Milestones
- [ ] KV-cache quantization to 4-bit
- [ ] Falcon attention head parallelizer
- [ ] Deployment validator script