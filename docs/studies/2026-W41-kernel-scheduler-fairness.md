# Study: Scheduler Fairness and Priority Inversion

## References
- **Linux:** Completely Fair Scheduler (CFS) vruntime tracking & EEVDF scheduler.
- **FreeBSD:** ULE Scheduler multi-queue interactivity heuristics & BSD priority bands.
- **OpenBSD:** Traditional BSD decay scheduler with strict priority preemption.
- **Redox / xv6:** Simple round-robin runqueue with process priority decay.

---

## Mechanism

### Problem
Prevent high-priority task starvation while ensuring I/O bound tasks remain interactive without suffering from priority inversion.

### Data Structures
- 64-level priority bitmap runqueue (`BoundedRunQueue`).
- Process vruntime tracking in `SchedulerProcessControlBlock`.

### Scheduling Rule
High-priority non-empty queues are serviced first. Starvation prevention mechanisms periodically boost tasks waiting in low-priority runqueues.

### Concurrency Model
Spinlock-protected runqueue modifications with lock-free atomic state checks for task state transitions.

### Failure Behavior
Task priority degradation under CPU-bound execution; starvation boost applied when waiting time exceeds maximum quantum threshold.

---

## SigmaOS State
- **Code:** `src/scheduler/run_queue.rs`, `src/scheduler/core.rs`.
- **Tests:** `tests/scheduler/core.rs`, `tests/scheduler/fairness/fairness_test.rs`.
- **Runtime Evidence:** Bounded runqueue unit and priority level tests pass cleanly.
- **Gaps:** Runtime context switch integration test under heavy multicore contention needs expansion.

---

## Decision
- **Adopt:** Multi-queue priority structure inspired by BSD ULE with vruntime fairness metrics from Linux CFS.
- **Adapt:** Bound maximum runqueue depth to 64 levels for no_std stack predictability.
- **Reject:** Dynamic priority calculations that require floating-point arithmetic in kernel mode.

---

## PR Plan
- [x] `feat(scheduler): add BoundedRunQueue priority bitmap`
- [x] `test(scheduler): add priority starvation boost tests`
- [x] `docs(studies): add 2026-W41 scheduler study`
