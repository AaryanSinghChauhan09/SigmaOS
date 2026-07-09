# SigmaOS Roadmap: Comprehensive Benchmarking Suite
Extend sigma_bench.rs to cover all subsystems with reproducible results.
## Goals
- IPC throughput, scheduler latency, memory bandwidth
- ML inference tokens/sec across model sizes
## Key Milestones
- [ ] Benchmark harness with warm-up and cooldown
- [ ] Results stored in sigma_db with timestamps
- [ ] CI badge generation from benchmark output