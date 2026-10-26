# Study: Scheduler fairness and priority inversion

## References
- Linux: CFS (Completely Fair Scheduler) red-black tree vruntime tracking
- FreeBSD: ULE / BSD priority run-queue scheduling with decay-based interactive boosting
- OpenBSD: Priority queue with nice value transformation and time-slice degradation
- Redox/xv6: Round-robin priority queue with simple process table iteration

## Mechanism
- Problem: Mitigate process starvation and priority inversion in multi-tasking kernel run queues
- Data structures: Priority array bitmap + double-ended queue per priority level
- Scheduling rule: Constant-time O(1) highest-priority runnable selection with starvation boosting counter
- Concurrency model: Spinlock-protected run-queue access with preemption locks
- Failure behavior: Task state fallback to safe run-queue on priority overflow

## SigmaOS state
- Code: `src/scheduler/run_queue.rs`, `src/scheduler/core.rs`
- Tests: `tests/scheduler/core.rs`, `tests/scheduler/fairness/fairness_test.rs`
- Runtime evidence: Unit tests pass; boot-to-shell smoke test validates scheduler state machine
- Gaps: Multi-core NUMA topology load balancing, real-time deadline scheduling classes

## Decision
- Adopt: 64-level priority bitmap selection with O(1) lookup
- Adapt: Exponential starvation decay counter for interactive tasks
- Reject: Unbounded dynamic priority adjustments without static bounds

## PR plan
- [x] feat(scheduler): add BoundedRunQueue bitmap selection
- [x] test(scheduler): add starvation boosting and context-switch integration tests
- [x] docs(scheduler): update scheduler fairness policy and study notes
