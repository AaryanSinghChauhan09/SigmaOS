# Study: Scheduler fairness models

## Reference systems
- Linux CFS: Completely Fair Scheduler using Red-Black trees tracking virtual runtime (`vruntime`) and latency targets.
- FreeBSD ULE: Multi-queue scheduler with interactive, non-interactive, and realtime queues with decay-based priority calculation.
- OpenBSD scheduler: Dual run-queue layout with randomized slice quantum adjustments to mitigate timing side channels.
- Redox: Cooperative/preemptive hybrid scheme utilizing event-driven async yield triggers.

## Mechanism
- **Problem**: Balancing low interactive latency with starvation-free compute throughput across heterogeneous task priorities.
- **Data structures**: Priority-indexed run-queue bitmaps, per-CPU `vruntime` trees, and starvation boost counters.
- **Scheduling decision**: Pick lowest `vruntime` task; if equal priority, fall back to round-robin quantum allocation.
- **Priority handling**: POSIX nice levels map to time-slice multipliers and decay rates.
- **Failure modes**: Priority inversion under locked resource contention; starvation of low-priority background threads during heavy I/O loops.

## SigmaOS current state
- **Code**: `src/scheduler/run_queue.rs` (`BoundedRunQueue` with 64 priority bitmaps) and `src/scheduler/core.rs` (`SchedulerCore`).
- **Tests**: `tests/scheduler/fairness/fairness_test.rs`, `tests/scheduler/preemption/preemption_test.rs`.
- **Runtime evidence**: Verified 7-state POSIX transitions and starvation boosting in standalone integration test suite.
- **Gaps**: Per-CPU load balancing across multi-socket NUMA domains needs hardware topology affinity feeds.

## Decision
- **Adopt**: Linux CFS virtual runtime accounting semantics for fair-share CPU allocation.
- **Adapt**: FreeBSD decay-based interactivity score calculation for low-latency desktop task scheduling.
- **Reject**: Unbounded priority decay vectors that introduce non-deterministic execution times in kernel space.

## Follow-up PRs
- [x] feat(scheduler): add starvation boost counter and priority bitmap indexing
- [x] test(scheduler): add starvation regression test suite in `tests/scheduler/`
