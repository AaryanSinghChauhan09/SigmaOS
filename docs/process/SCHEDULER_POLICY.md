# SigmaOS Scheduler Policy Specification

## Overview

The SigmaOS Scheduler operates as a safe, observable, testable lifecycle subsystem designed to prevent leaks, races, starvation, and undefined behavior while driving the kernel's boot-to-interactive-shell path.

## Core Architectural Dimensions

### 1. Scheduling Class
SigmaOS implements a **Hybrid Priority / CFS Scheduling Class**:
- Real-time and elevated system tasks use a 64-level multi-level queue structure with bitmap selection.
- Normal user tasks utilize virtual runtime (`vruntime`) tracking similar to Completely Fair Scheduler (CFS) for proportional CPU time distribution.

### 2. Run-Queue Structure
- Run queues are bounded per CPU core with a default maximum capacity (`MAX_RUNQUEUE_CAPACITY = 1024`).
- Enqueue and dequeue operations are constant-time $O(1)$ or bounded $O(\text{priority levels})$ via bitmask operations (`active_bitmap`).

### 3. Time-Slice Policy
- Each task receives a default quantum of 10 timer ticks upon being scheduled (`remaining_quantum_ticks`).
- Time-slice expiration triggers a voluntary yield or preemptive reschedule to ensure task responsiveness.

### 4. Priority and Nice Semantics
- Priority values range from 0 (idle/lowest) to 63 (real-time/highest).
- Priority determines the initial run-queue level and relative time-slice scaling.

### 5. Preemption Points
- Preemption occurs at timer tick interrupts, voluntary yielding (`sys_yield`), I/O blockage, and higher-priority task enqueue events.

### 6. Fairness Metric
- Fairness is tracked via `vruntime` accumulator.
- The task with the lowest accumulated `vruntime` in a given priority class is selected for execution first.

### 7. Starvation Prevention
- Multi-level queues execute periodic starvation checks (`boost_starved_tasks`).
- Tasks waiting longer than `DEFAULT_STARVATION_THRESHOLD_TICKS` (50 ticks) have their priority boosted dynamically.

### 8. Priority Inversion Handling
- Priority inheritance protocols (PIP) elevate lower-priority tasks holding mutexes needed by higher-priority tasks.

### 9. CPU Affinity Behavior
- Each processPCB retains a 64-bit affinity bitmask (`cpu_affinity`).
- Scheduler routes task placement to the best CPU core matching the bitmask while balancing queue load.

### 10. Zero Runnable Task Behavior
- When no runnable tasks exist on a CPU core, the scheduler selects the kernel idle task (`hlt` / CPU energy save loop).

## Authoritative Process Lifecycle

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```
