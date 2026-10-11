# Study: Scheduler fairness and priority inversion

## References
- Linux: CFS (Completely Fair Scheduler) & EEVDF (Earliest Eligible Virtual Deadline First)
- FreeBSD: ULE Scheduler
- OpenBSD: Priority queue runqueue
- Redox/xv6: Round-Robin / Priority-based scheduling

## Mechanism
- Problem: Mitigating priority inversion and ensuring fair vruntime distribution across multithreaded workloads.
- Data structures: Red-Black tree tracking vruntime / deadline.
- Scheduling rule: Select process with minimum vruntime / earliest eligible deadline.
- Concurrency model: Per-CPU runqueues with lock-free atomic load balancing.
- Failure behavior: Starvation detection triggers boost policy.

## SigmaOS state
- Code: `src/kernel/sigma_scheduler_eevdf.rs`, `src/kernel/roundrobin.rs`
- Tests: Standalone scheduler unit tests
- Runtime evidence: Simulated task scheduling ticks
- Gaps: Multi-core IPI context switch benching under real hardware load

## Decision
- Adapt: EEVDF latency-sensitive deadline calculation for real-time threads combined with CFS vruntime tracking.

## PR plan
- [x] feat(scheduler): add EEVDF scheduling engine
- [x] test(scheduler): add runnable-vs-waiting fairness unit tests
- [ ] docs(scheduler): update scheduler integration contracts
