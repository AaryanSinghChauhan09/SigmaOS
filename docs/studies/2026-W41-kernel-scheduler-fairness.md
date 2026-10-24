# Study: Scheduler fairness and priority inversion

## References
- Linux: CFS (`sched/fair.c`), EEVEF/sched_ext policy switching, vruntime tracking.
- FreeBSD: ULE scheduler (`sched_ule.c`), interactivity scoring, priority boosting.
- OpenBSD: Priority queue runqueue with lockless bitmap selection.
- Redox/xv6: Round-robin runqueue with process priority caps.

## Mechanism
- Problem: Mitigating priority inversion and ensuring bounded delay for runnable tasks without starvation.
- Data structures: 64-level priority runqueue bitmap, red-black tree / min-heap for virtual runtime tracking.
- Scheduling rule: Pick task with lowest vruntime; boost starved tasks after `STARVATION_THRESHOLD_TICKS`.
- Concurrency model: Per-CPU runqueue locks with atomic bitmap updates.
- Failure behavior: CPU idle task fallback when runqueue is empty.

## SigmaOS State
- Code: `src/scheduler/core.rs`, `src/scheduler/run_queue.rs`.
- Tests: `tests/scheduler/fairness/fairness_test.rs`, `tests/scheduler/core.rs`.
- Runtime evidence: `run_sigma_tests.sh` pass status.
- Gaps: Real-time deadline scheduling integration in kernel boot path.

## Decision
- Adopt: CFS-style vruntime tracking combined with BSD-style interactivity priority boosting.
- Adapt: 64-level priority runqueue with starvation boosting for no_std deterministic runtime.
- Reject: Unbounded dynamic memory allocation inside scheduler tick interrupt handlers.

## PR Plan
- [x] `feat(scheduler)`: Implement `BoundedRunQueue` and `SchedulerCore`.
- [x] `test(scheduler)`: Add unit and integration tests for priority boosting and context switches.
- [x] `docs(scheduler)`: Document scheduler policy and fairness ADR in `docs/process/SCHEDULER_POLICY.md`.
