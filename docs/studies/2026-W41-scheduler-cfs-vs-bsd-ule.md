# Study: Completely Fair Scheduler (CFS) vs. FreeBSD ULE Scheduler

## Reference systems
- Linux: Completely Fair Scheduler (CFS) / EEVDF (`kernel/sched/fair.c`) - Red-black tree tracking vruntime.
- FreeBSD/OpenBSD: ULE Scheduler (`sys/kern/sched_ule.c`) - Multi-queue per-CPU interactivity queues with decay.
- Redox/xv6/other: Simple round-robin with priority run-queues.

## Mechanism under study
- **Problem**: Fair CPU allocation across interactive desktop tasks and background compute tasks without priority starvation or priority inversion.
- **Interface**: `sched_yield()`, `nice()`, `sched_setaffinity()`.
- **Data structures**: CFS uses red-black trees sorted by virtual runtime (`vruntime`); ULE uses 64-level runqueues with interleaved time slices.
- **Concurrency model**: Per-CPU runqueues with lockless/atomic task migration and load balancing interrupts.
- **Failure behavior**: Unbounded starvation if priority boosting is omitted; latency spikes if context switch overhead dominates.

## SigmaOS current state
- **Code location**: `src/scheduler/core.rs`, `src/scheduler/run_queue.rs`.
- **Existing tests**: `tests/scheduler/core.rs`, `tests/scheduler/scheduler_test.rs`.
- **Verified runtime behavior**: 64 priority level `BoundedRunQueue` with starvation boosting and CPU affinity support.
- **Known gaps**: Lack of dynamic vruntime tracking for interactive tasks.

## Adaptation decision
- **Adapt**: Hybrid Bounded RunQueue + CFS vruntime tracking.
- **Reason**: Maintains `no_std` determinism while providing fair CPU sharing across interactive shell tasks and background services.

## Proposed PRs
- [ ] PR 1: Add dynamic vruntime tracking to `BoundedRunQueue`.
- [ ] PR 2: Integrate latency target smoothing in `SchedulerCore`.

## Acceptance evidence
- [ ] `cargo check --lib`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] QEMU/boot or integration evidence, if applicable
