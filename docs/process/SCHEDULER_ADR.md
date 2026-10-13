# Architecture Decision Record: SigmaOS Scheduler Subsystem Lifecycle (`SCHEDULER_ADR.md`)

## Status
Accepted

## Context
Kernel experiments previously allowed ad-hoc scheduling models without formal contracts for process lifecycle, bounded queues, or lost-wakeup protection. Passing unit tests alone does not establish kernel integration, safety, or hardware predictability. A rigorous lifecycle state machine and bounded run queue architecture are necessary to ensure SigmaOS creates no resource leaks, races, starvation, or undefined behavior.

## Decisions

### 1. Authoritative 7-State Process Lifecycle Machine
SigmaOS defines a single authoritative lifecycle state machine across all process and scheduler APIs:
```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```
- No task shall be scheduled after it enters `ZOMBIE` or `REAPED`.
- All `ZOMBIE` tasks must eventually be reaped to release PID and accounting metadata.
- Parent termination automatically reparents orphan processes to PID 1 (`init`).

### 2. Multi-Level Bounded Run Queue
- Per-CPU run queues (`BoundedRunQueue`) enforce a maximum capacity limit (`MAX_RUNQUEUE_CAPACITY = 1024`).
- Priority levels are strictly bounded to 64 queues (`NUM_PRIORITY_LEVELS = 64`), indexed via a 64-bit active bitmap.
- Enqueue returns `RunQueueError::CapacityExceeded` when full, treating queue exhaustion as a designed condition rather than an unhandled panic.

### 3. Lost-Wakeup Protection
- `SchedulerProcessControlBlock` maintains a `wake_event_pending` boolean and `pending_signals` mask.
- If `wake_process` is invoked on a process before `sleep_process` finishes entering sleep state, `wake_event_pending` is set to `true`.
- Subsequent `sleep_process` calls observe `wake_event_pending`, clear the flag, and immediately return the task to `READY` state without blocking.

### 4. SMP Load Balancing & CPU Affinity
- `SchedulerCore` manages per-CPU run queues and active PID slots (`active_pids`).
- CPU affinity masks (`u64`) are respected during core selection (`select_best_cpu`), selecting the target core with the lowest current queue load.

## Consequences
- **Pros**: Deterministic state transitions, race-free lost-wakeup protection, bounded memory footprint, zero zombie orphan leaks.
- **Cons**: Requires explicit handling of capacity errors on task creation/enqueue.
