# SigmaOS Scheduler Subsystem Policy

## Overview

The SigmaOS scheduler is governed as a **lifecycle subsystem**, not as a single task. It manages process execution, time-slicing, priority ordering, fairness, preemption, CPU affinity, and state transitions across the boot-to-interactive-shell runtime path and multi-core operations.

## Process State Machine Contract

SigmaOS defines one authoritative process lifecycle:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

| State | Required Behavior | Invariants |
|---|---|---|
| `NEW` | PID allocated, address space and credentials initialized | Must transition to `READY` before execution. |
| `READY` | Placed in scheduler run queue | Validated priority, non-zero remaining quantum. |
| `RUNNING` | Executes on CPU core with active preemption timer | Only 1 process running per CPU core. |
| `SLEEPING` | Blocked on wait queue with explicit wakeup path | Lost-wakeup protection via mutex/wait-queue lock. |
| `STOPPED` | Suspended by signal or debugger | Will not consume CPU cycles while stopped. |
| `ZOMBIE` | Exit status retained until reaped | Released process memory, PID retained for parent reaping. |
| `REAPED` | PID, kernel resources, and accounting released | Cannot be scheduled after transition to `REAPED`. |

## 10 Core Subsystem Dimensions

1. **Scheduling Class**:
   SigmaOS employs a hybrid priority-based CFS (Completely Fair Scheduler) model with interactive BORE (Burst-Oriented Response Enhancer) and EEVDF (Earliest Eligible Virtual Deadline First) extensions. High-priority real-time tasks take precedence, while normal tasks are scheduled based on virtual runtime (`vruntime`) and eligible deadlines.

2. **Run-Queue Structure**:
   Bounded multi-level priority run queue per core (`BoundedRunQueue`) with 64 priority levels (`0..63`). A 64-bit bitmap index enables O(1) non-empty queue selection.

3. **Time-Slice Policy**:
   Default time-slice quantum is 10 ticks (100ms at 100Hz timer frequency). Real-time tasks receive configurable quanta based on priority class.

4. **Priority and Nice Semantics**:
   Priorities range from 0 (idle) to 63 (real-time/critical). Standard POSIX `nice` values (-20 to +19) map inversely to user priorities (0 to 39).

5. **Preemption Points**:
   Preemption occurs on timer interrupts (quantum expiration), higher-priority task enqueuing, voluntary yield (`sched_yield`), and I/O wait state entries.

6. **Fairness Metric**:
   Runnable-vs-waiting ratio computed via normalized `vruntime` and weighted execution tracking. Standard deviation of wait times across runnable processes must remain under 15%.

7. **Starvation Prevention**:
   Starvation threshold set to 50 ticks (500ms). Tasks waiting longer than the starvation threshold receive dynamic priority boosting to priority 62 until executed.

8. **Priority Inversion Handling**:
   Priority Inheritance Protocol (PIP) boosts mutex holders' dynamic priorities to match the highest priority waiting thread.

9. **CPU Affinity Behavior**:
   Bitmask set representing eligible CPUs. Migration occurs only when target core load imbalance exceeds 25% or hard affinity requires rebalancing.

10. **Zero Runnable Task Handling**:
    When no runnable tasks exist, the core switches to an architecture-specific idle state (`hlt` / `wfi`), conserving power until the next timer or peripheral interrupt.
