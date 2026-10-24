# SigmaOS Scheduler Architectural Decision Record (ADR)

## Status
Accepted

## Context
SigmaOS requires a safe, deterministic, and highly observable scheduler subsystem that connects process models to the boot-to-shell path without memory leaks, race conditions, priority inversion, or process starvation.

## Decisions

### 1. Hybrid Priority / CFS Scheduling Policy
- **Decision**: Combine $O(1)$ multi-queue priority scheduling with $O(\log N)$ / bitmap-accelerated Virtual Runtime (`vruntime`) tracking.
- **Rationale**: Real-time tasks demand bounded deterministic latency ($O(1)$ scheduling), while general userland workloads benefit from CFS-style fairness and nice levels.

### 2. Explicit Bounded Run Queues
- **Decision**: All scheduler run queues (`BoundedRunQueue`) enforce a strict maximum capacity limit (`MAX_RUNQUEUE_CAPACITY`).
- **Rationale**: Prevents unbounded kernel memory growth under heavy thread creation or fork-bomb attacks. Exceeding capacity returns `RunQueueError::CapacityExceeded` cleanly without panicking.

### 3. POSIX Lifecycle State Machine
- **Decision**: Strictly enforce 7 process states: `NEW`, `READY`, `RUNNING`, `SLEEPING`, `STOPPED`, `ZOMBIE`, `REAPED`.
- **Rationale**: Guarantees process memory and PIDs are never reused prematurely or scheduled after termination, eliminating use-after-free and zombie leaks.

### 4. Lost-Wakeup Protection
- **Decision**: Incorporate `wake_event_pending` flags and atomic check-before-sleep checks in `sleep_process` and `wake_process`.
- **Rationale**: Eliminates race conditions where an interrupt or signal wakes a process immediately before `sleep_process` updates state to `SLEEPING`.

### 5. Automatic Orphan Reparenting
- **Decision**: When a parent process terminates (`exit_process`), all active children are automatically reparented to `init` (PID 1).
- **Rationale**: Guarantees all zombie processes have a valid parent capable of reaping them via `waitpid`.
