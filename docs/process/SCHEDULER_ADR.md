# ADR-0004: SigmaOS Lifecycle-Driven Bounded Scheduler Subsystem

* **Status**: Accepted
* **Date**: 2026-10-15
* **Deciders**: Kernel Architecture Team
* **Rules File**: `SCHEDULER_RULES.toml`

## Context

SigmaOS requires a deterministic kernel process scheduler that guarantees bounded resource usage, lost-wakeup protection, starvation prevention, and strict POSIX process lifecycle transitions without relying on unbounded heap allocations or model-only simulations.

## Decision

We establish the SigmaOS Scheduler Subsystem as a lifecycle-driven core with the following key architectural decisions:

1. **7-State POSIX Process Lifecycle**: Strict state machine enforcing `NEW -> READY -> RUNNING -> SLEEPING -> READY`, `READY -> RUNNING -> ZOMBIE`, `RUNNING -> STOPPED -> READY`, and `ANY -> ZOMBIE -> REAPED`.
2. **Bounded Multi-Level Priority Run Queue**: 64 priority queues per CPU core backed by 64-bit priority bitmaps for O(1) scheduling selection and hard capacity limits (1024 tasks per queue).
3. **Lost-Wakeup Protection**: Flag-based tracking (`wake_event_pending`) ensuring a wake event delivered right before `sleep_process` keeps the process in `READY` state instead of blocking.
4. **Starvation Boosting**: Tasks waiting in run queues longer than `starvation_threshold_ticks` (default 50 ticks) receive temporary priority boosts (+10 levels, capped at 62).
5. **CPU Affinity Governance**: Bitmask-based CPU affinity matching with minimum-load core selection and validation.
6. **Orphan Reparenting**: When a parent process exits, its active child processes are atomically reparented to PID 1 (init).

## Consequences

- Prevents kernel memory exhaustion through static capacity bounds.
- Guarantees no thread lost or deadlock during sleep/wake races.
- Eliminates zombie process leaks and orphan accumulation.
- Connects validated unit tests directly to the boot-to-shell validation path.
