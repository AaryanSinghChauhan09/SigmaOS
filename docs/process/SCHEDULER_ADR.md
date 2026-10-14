# Architectural Decision Record: SigmaOS Scheduler Subsystem

## Status
Approved / Active

## Context
SigmaOS requires a kernel scheduler capable of low-latency task dispatching, explicit memory upper-bounds, preemption safety, lost-wakeup prevention, starvation elimination, and CPU affinity governance without incurring race conditions, memory leaks, or non-deterministic lifecycle states.

## Key Decisions

1. **Explicit 6-State Process Lifecycle Machine**:
   - Machine transitions explicitly strictly between `NEW`, `READY`, `RUNNING`, `SLEEPING`, `STOPPED`, `ZOMBIE`, and `REAPED`.
   - ZOMBIE states retain exit codes until parent reap or init adoption; double-reaping is prevented by state checking.

2. **Bounded BoundedRunQueue with 64 Priority Levels**:
   - Run queue capacity per core is explicitly capped (`MAX_RUNQUEUE_CAPACITY = 1024`).
   - Bitmask $O(1)$ priority selection over 64 levels prevents unbounded traversal overhead.

3. **Lost-Wakeup Protection**:
   - `lost_wakeup_signaled` atomic boolean flag prevents race conditions where a task is woken up between checking sleep condition and being placed on a wait queue.

4. **Starvation Boosting & Fairness**:
   - Every enqueue records enqueue tick. Tasks exceeding starvation threshold receive boost flags elevating priority to ensure bounded latency under heavy load.

5. **CPU Affinity & Migration Limits**:
   - 64-core affinity mask (`cpu_affinity: u64`) enforces task placement boundaries. Core migration validates target CPU affinity.
