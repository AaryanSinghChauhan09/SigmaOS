# SigmaOS Scheduler Policy Specification (`SCHEDULER_POLICY.md`)

## 1. Overview
This policy defines the authoritative operational model for the SigmaOS CPU Scheduler subsystem. The scheduler governs thread/process execution across single-core and multi-core (SMP) systems, balancing interactive responsiveness, throughput, fairness, and strict preemption safety.

## 2. Authoritative Lifecycle State Machine
Every process or thread managed by the SigmaOS scheduler MUST adhere to the 7-state lifecycle transition model:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### Required Behavior Per State:
| State | Behavior & Requirements |
|---|---|
| `NEW` | PID allocated, address space & process control block (PCB) initialized. Not yet in run queue. |
| `READY` | Inserted into the active `BoundedRunQueue` for a CPU core. Eligible for dispatch. |
| `RUNNING` | Dispatched to CPU. Time-slice quantum decrements on each timer tick. Preemption active. |
| `SLEEPING` | Blocked waiting on queue or wake event. Includes lost-wakeup protection flags. |
| `STOPPED` | Suspended by signal (e.g. `SIGSTOP`) or debugger. Removed from run queues. |
| `ZOMBIE` | Terminated via `exit_process`. Exit code retained; parent process notified; resources reclaimed upon reap. |
| `REAPED` | PID, kernel stack, PCB, and accounting structures fully released and returned to pool. |

## 3. Scheduling Class & Architecture
SigmaOS employs a **hybrid priority-based multi-level run queue with Virtual Runtime (vruntime) tracking**.

- **Priority Levels**: 64 priority levels (0 = lowest/idle, 63 = real-time / urgent).
- **Time-Slice Quantum**: Default quantum is 10 timer ticks per schedule window.
- **Priority & Nice Semantics**: Priority maps directly to run queue array indices (0..63).
- **Active Bitmap**: A 64-bit bitmap (`u64`) accelerates $O(1)$ lookup of highest non-empty priority queue.

## 4. Preemption Points & Safety
- **Quantum Expiration**: On timer tick, if remaining quantum hits 0, process transitions `RUNNING -> READY` and yields to higher or equal priority tasks.
- **Voluntary Yield**: Process can yield CPU early, transitioning to `READY` and re-entering run queue at its priority tier.
- **Interrupt Preemption**: Higher priority tasks unblocked by ISRs trigger immediate preemptive rescheduling on timer or IPI return.

## 5. Fairness & Starvation Prevention
- **Fairness Metric**: Monitored via `vruntime` and ratio of time spent `RUNNING` vs `READY`.
- **Starvation Threshold**: 50 timer ticks (default ~500ms at 100Hz).
- **Starvation Boost**: Periodic timer tick inspects tasks enqueued longer than threshold and temporarily boosts priority by +10 levels (up to max 62).

## 6. CPU Affinity & Zero Runnable Tasks
- **Affinity Bitmask**: 64-bit CPU mask (`u64`) restricts tasks to assigned CPU cores.
- **Zero Runnable Tasks**: When run queue is empty, CPU enters idle state (`schedule_next` returns `None`), triggering power-saving `hlt`/`cpuidle`.
