# Study: Scheduler fairness, Linux CFS vs FreeBSD ULE priority handling

## References
- Linux: Completely Fair Scheduler (`kernel/sched/fair.c`, virtual runtime tracking)
- FreeBSD: ULE Scheduler (`sys/kern/sched_ule.c`, dynamic interactivity scoring and priority queues)
- OpenBSD: Priority queue scheduler (`sys/kern/sched_bsd.c`)
- Redox / xv6: Simple round-robin with priority tiers

## Mechanism
- **Problem**: Balance CPU fairness, interactive latency, and prevent priority inversion and starvation under heavy load.
- **Data structures**: Red-black tree / min-heap ordered by virtual runtime (`vruntime`) or 64-level priority bitmaps with run queues.
- **Scheduling rule**: Select task with minimum `vruntime` or highest priority level; decay interactivity scores for CPU-bound tasks.
- **Concurrency model**: Per-CPU run queues with lock-free atomic selection or bounded spinlocks and inter-processor interrupts (IPI).
- **Failure behavior**: Fallback to idle loop on empty queues; decay starved tasks using age boosting.

## SigmaOS State
- **Code**: `src/scheduler/core.rs`, `src/scheduler/run_queue.rs`, `src/governance/kernel_lifecycle_program.rs`.
- **Tests**: `tests/scheduler/core.rs`, `tests/scheduler/fairness/fairness_test.rs`.
- **Runtime evidence**: Passes unit and standalone fairness test suites.
- **Gaps**: Needs further boot-time runtime evidence under QEMU multi-core SMP stress.

## Decision
- **Adapt**: Hybrid priority/vruntime scheduling class with 64 priority levels, 64-bit bitmap selection, and starvation boosting.

## PR Plan
- [x] `feat(scheduler)`: Implement bounded run queues with starvation boosting.
- [x] `test(scheduler)`: Add context switch and fairness integration tests.
- [ ] `docs(scheduler)`: Update scheduler integration notes in BOOT_MATRIX.md.
