# Upstream Study: Linux Kernel Scheduler (EEVDF)

1. **Upstream Problem**: Multi-core, latency-sensitive CPU scheduling with virtual deadlines and fair queueing.
2. **Data Structure & Control Flow**: Earliest Eligible Virtual Deadline First (`cfs_rq`), Red-Black Tree virtual runtime tracking.
3. **Security & Failure Assumptions**: cgroups v2 CPU quota enforcement; starvation prevention.
4. **Direct Adoption**: EEVDF virtual deadline and eligibility calculation rules.
5. **Rust/SigmaOS Adaptation**: Hybrid EEVDF/Round-Robin scheduler wrapped in `Spinlock<BTreeMap<u64, Task>>`.
6. **Exit Test Criteria**: Task priority latency test proves zero task starvation across 100 concurrent threads.
7. **Licensing & Attribution**: GPL-2.0 (Clean-room specification in SigmaOS).
