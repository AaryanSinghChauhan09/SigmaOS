# SigmaOS Scheduler Policy Specification

## 1. Subsystem Overview

SigmaOS implements a multi-level priority hybrid scheduler designed for low latency, predictable preemption, lost-wakeup protection, and starvation prevention.

## 2. Authoritative Process Lifecycle State Machine

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### State Contract

| State | Required Behavior |
| :--- | :--- |
| **NEW** | PID allocated, address space and credentials initialized. |
| **READY** | Placed in bounded scheduler run queue for selection. |
| **RUNNING** | Executes on CPU core with preemption and time-slice tracking defined. |
| **SLEEPING** | Blocked on a wait queue with explicit wakeup path and lost-wakeup protection. |
| **STOPPED** | Suspended by signal (`SIGSTOP`/`SIGTSTP`) or debugger. |
| **ZOMBIE** | Execution completed; exit status retained until parent reaps process. |
| **REAPED** | PID, kernel resources, page tables, and accounting records fully released. |

A process must **never** be scheduled after it exits, and every zombie process must eventually be reaped or adopted by `init` (PID 1).

## 3. Run Queue & Scheduling Policies

- **Scheduling Class**: Hybrid Multi-Priority Round-Robin with EEVDF/CFS-inspired virtual runtime (`vruntime`) tracking.
- **Run-Queue Structure**: Bounded array of 64 priority queues (0 = lowest/idle, 63 = real-time) with 64-bit priority bitmask for $O(1)$ queue selection.
- **Time-Slice Policy**: Dynamic quantum assignment based on priority level (10ms base quantum, scaled by nice level).
- **Priority & Nice Semantics**: Standard POSIX nice range mapped to priority levels 0 to 63.
- **Preemption Points**:
  1. Quantum expiration on periodic timer tick (100 Hz / 1000 Hz).
  2. Direct voluntary yield (`sched_yield`).
  3. Wakeup of higher-priority task during I/O or signal delivery.
  4. Explicit preemption requested during kernel entry/exit.
- **Fairness Metric**: Monotonically increasing `vruntime` tracked per task. Lower `vruntime` tasks receive scheduling preference within equal priority bands.
- **Starvation Prevention**: Automatic starvation boosting. Tasks waiting in run queues longer than `DEFAULT_STARVATION_THRESHOLD_TICKS` (50 ticks / 500ms) receive temporary priority elevation.
- **Priority-Inversion Handling**: Priority inheritance protocol applied on mutex and wait-queue contend points.
- **CPU-Affinity Behavior**: Core affinity mask (`u64`) governs task-to-CPU assignment. Tasks with affinity restrictions are only enqueued on matching core run queues.
- **Zero-Runnable Behavior**: CPU enters idle state (`hlt` / low-power C-state) when run queues are empty.
