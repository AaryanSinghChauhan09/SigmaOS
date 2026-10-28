# SigmaOS Scheduler Subsystem Policy

## Overview & Objective
The SigmaOS scheduler is a **lifecycle subsystem** designed to maintain a safe, observable, testable scheduling environment with zero leaks, races, starvation, or undefined behavior, connecting kernel process models directly to a validated boot-to-interactive-shell path.

## Authoritative Lifecycle State Machine

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### State Behavior Requirements

| State | Required Behavior |
|---|---|
| **NEW** | PID allocated, address space and credentials initialized. |
| **READY** | Placed in scheduler run queue; ready for execution. |
| **RUNNING** | Executes on CPU core with defined preemption and quantum. |
| **SLEEPING** | Blocked on a wait queue with deterministic wakeup path & lost-wakeup protection. |
| **STOPPED** | Suspended by signal or debugger. |
| **ZOMBIE** | Process exit status retained until reaped by parent or init (PID 1). |
| **REAPED** | PID, kernel resources, memory pools, and accounting completely released. |

## Core Scheduler Dimensions

1. **Scheduling Class**: Hybrid Priority / CFS-like policy. High-priority real-time tasks take precedence via priority bitmaps; normal tasks are scheduled using virtual runtime (`vruntime`) tracking for weighted fairness.
2. **Run-Queue Structure**: Per-CPU bounded run queues supporting up to 64 priority levels with atomic bitmap selection and constant-time O(1) enqueue/dequeue operations.
3. **Time-Slice Policy**: Default quantum of 10 timer ticks (100ms at 100Hz tick frequency). Time-slice decrement occurs at timer interrupt; quantum expiration triggers yield and re-enqueue to `READY`.
4. **Priority and Nice Semantics**: Priority range from 0 (highest) to 63 (lowest). Nice levels map linearly onto priority shifts and quantum scaling.
5. **Preemption Points**: Interrupt handler returns, kernel preemptible sections, voluntary `sys_yield`, and blocking I/O transitions.
6. **Fairness Metric**: Measured via runnable-vs-waiting ratio and virtual runtime delta. `vruntime` accrual ensures fair CPU time distribution.
7. **Starvation Prevention**: Tasks waiting longer than `starvation_threshold_ticks` (default: 50 ticks / 500ms) receive temporary priority boosts.
8. **Priority-Inversion Handling**: Priority inheritance mechanism boosts lock-holder priority to match highest blocked waiter.
9. **CPU-Affinity Behavior**: Bitmask-based core affinity masks. Load balancer migrates tasks across cores while respecting affinity masks and cache locality cost models.
10. **Zero Runnable Tasks Handling**: Switches CPU core to idle loop (`hlt` instruction / power-aware wait state) with immediate interrupt wake capability.
