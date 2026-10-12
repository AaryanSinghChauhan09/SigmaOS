# SigmaOS ADR 0002: Scheduler Subsystem Architecture & Lifecycle Governance

## Context
SigmaOS requires an authoritative, safe, and testable scheduling system that eliminates process leaks, data races, starvation, and priority inversions while maintaining a validated boot-to-shell execution path.

## Decision
1. **Authoritative Lifecycle State Machine**:
   ```text
   NEW -> READY -> RUNNING -> SLEEPING -> READY
   READY -> RUNNING -> ZOMBIE
   RUNNING -> STOPPED -> READY
   ANY -> ZOMBIE -> REAPED
   ```
2. **State Transition Guarantees**:
   - `NEW`: PID allocated, address space & credentials initialized.
   - `READY`: Enqueued into CPU `BoundedRunQueue`.
   - `RUNNING`: Executing on CPU with time-slice preemption.
   - `SLEEPING`: Blocked on wait queue with lost-wakeup protection.
   - `STOPPED`: Suspended by signal or debugger.
   - `ZOMBIE`: Process exited; exit code retained until parent reaps.
   - `REAPED`: PID and kernel process table slot freed; orphans reparented to init (PID 1).

3. **Concurrency & Synchronization**:
   - Per-CPU run-queue lock ordering strictly specified.
   - Lost-wakeup prevention flags (`wake_event_pending`, `pending_signals`) prevent race conditions when transitioning to sleep.

4. **Resource Constraints**:
   - Bounded process table (`MAX_PROCESS_TABLE_SIZE = 4096`).
   - Bounded run-queue capacity (`MAX_RUNQUEUE_CAPACITY = 2048`).
   - OOM treated as explicit failure state rather than panic.

## Status
Accepted and Enforced by `SCHEDULER_RULES.toml`.
