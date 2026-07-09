# SigmaOS Roadmap: Active Learning for Kernel Parameter Tuning
Use active learning loops to discover optimal system limits (TCP window sizes, task quotas).
## Goals
- Query system configurations to find parameters that yield maximum throughput.
- Target zero user intervention during optimization cycles.
## Key Milestones
- [ ] Parameter configuration sampler
- [ ] Latency/throughput reward calculator
- [ ] Kernel parameter updating daemon