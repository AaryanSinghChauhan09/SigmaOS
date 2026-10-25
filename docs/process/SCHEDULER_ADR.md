# ADR-0004: SigmaOS Scheduler Lifecycle Subsystem Architecture

## Status
Accepted

## Context
SigmaOS requires a deterministic, safe, and observable scheduler subsystem to manage process states, run queues, context switching, preemption, fairness, and multi-core CPU affinity.

## Decision
1. Implement scheduler state transitions according to POSIX lifecycle state machine (`NEW`, `READY`, `RUNNING`, `SLEEPING`, `STOPPED`, `ZOMBIE`, `REAPED`).
2. Enforce bounded run queues per CPU core with active priority bitmaps (64 levels) to bound kernel memory allocation.
3. Incorporate lost-wakeup prevention mechanisms (`wake_event_pending` flags) across sleep and wake pathways.
4. Reparent orphan zombie processes to Init (PID 1) upon parent termination to eliminate memory leaks.
5. Provide automatic starvation boosting for tasks exceeding waiting thresholds.

## Consequences
- Guarantees zero zombie leaks and race-free PID lifecycle management.
- Bounded memory footprint prevents kernel stack/heap exhaustion under heavy process creation.
