# Architecture Decision Record (ADR): SigmaOS Scheduler Subsystem

## Status

Accepted

## Context

SigmaOS requires a safe, deterministic, multi-core scheduler subsystem that prevents resource leaks, race conditions, starvation, and undefined state transitions while connecting process models to the boot-to-interactive-shell path.

## Decision

1. **Explicit Lifecycle State Machine**:
   Adopt a 7-state process lifecycle (`NEW`, `READY`, `RUNNING`, `SLEEPING`, `STOPPED`, `ZOMBIE`, `REAPED`) with atomic state transitions and lost-wakeup protection.

2. **Bounded Run-Queue Architecture**:
   Implement `BoundedRunQueue` capped at 1024 tasks per core across 64 priority levels with 64-bit bitmap selection, guaranteeing bound memory usage and $O(1)$ selection complexity.

3. **Hybrid CFS/EEVDF Scheduling Policy**:
   Combine CFS virtual runtime fairness with EEVDF deadline eligibility and BORE interactive burst detection.

4. **Starvation Prevention via Priority Boosting**:
   Enforce a 50-tick starvation limit that dynamically boosts waiting tasks to avoid starvation under heavy workloads.

5. **Lock Ordering Discipline**:
   Establish clear lock hierarchy: `ProcessTable` -> `RunQueue` -> `WaitQueue` -> `SignalState`.

## Consequences

- Prevents kernel stack overflow and memory exhaustion under thread creation bursts.
- Ensures all zombie processes are reaped without orphaned PID retention.
- Guarantees deterministic preemption and context-switch integrity.
