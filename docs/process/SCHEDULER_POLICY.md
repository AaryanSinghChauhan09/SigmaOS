# SigmaOS Scheduler Policy & Acceptance Criteria (`docs/process/SCHEDULER_POLICY.md`)

## 1. Scope & Objective

This document defines the scheduler contracts, run-queue data structures, priority preemption mechanics, starvation prevention, and CPU affinity rules for SigmaOS.

## 2. Run Queue Architecture

SigmaOS implements a multi-priority bounded run queue (`BoundedRunQueue`) featuring 64 priority levels:

- **Priority Levels**: 0 (highest / real-time) to 63 (lowest / idle).
- **Selection Optimization**: 64-bit priority bitmap (`u64`) enables $O(1)$ highest-priority runnable process selection via `trailing_zeros()` / `leading_zeros()`.
- **Time Slice**: Configurable quantum (e.g., 10 ms default per time slice).

```text
RunQueue (64 Priority Buckets)
 ├── Bitmask: 0b0000...00100 (Indicates active processes at Priority 2)
 ├── Priority 0: [ Task A ] -> [ Task B ]
 └── Priority 2: [ Task C ]
```

## 3. Preemption & Fairness Invariants

1. **Strict Priority Preemption**: A higher-priority process entering the `READY` state preempts lower-priority processes on the current CPU core.
2. **Starvation Boosting**: To prevent low-priority task starvation, the scheduler periodically increments wait-time counters and temporarily boosts starved processes to higher priorities.
3. **Zero Runnable Tasks**: When no processes are in the `READY` run queue, the CPU transitions to the idle thread executing power-saving wait loops (`hlt`).
4. **CPU Affinity**: Processes with assigned CPU core masks are enqueued and dispatched exclusively on matching core run queues.

## 4. Verification

Scheduler preemption, run queues, and starvation bounds are tested in `src/scheduler/` and `tests/scheduler/scheduler_test.rs`.
