# SigmaOS Scheduler Policy Specification

## Architectural Overview
The SigmaOS Scheduler Subsystem operates as an authoritative, lifecycle-managed kernel component responsible for preemptive scheduling, process lifecycle governance, SMP run-queue management, fairness guarantees, and low-latency context switching across active CPU cores.

## Authoritative Lifecycle State Machine

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### State Contracts and Required Behavior

| State | Required Behavior |
|---|---|
| `NEW` | PID allocated via race-free `PidAllocator`, address space initialized, credentials/capability tokens bound. |
| `READY` | Placed in bounded SMP scheduler run queue (`BoundedRunQueue`), eligible for scheduling execution. |
| `RUNNING` | Executes on assigned CPU core with defined preemption, time-slice quantum countdown, and interrupt safety. |
| `SLEEPING` | Blocked on wait queue with explicit wake-event check; lost-wakeup protection active. |
| `STOPPED` | Suspended by signal or debugger; removed from run queue until explicitly resumed. |
| `ZOMBIE` | Process execution terminated; exit status and resource metrics retained until parent reaps. |
| `REAPED` | PID, kernel stack, page tables, file descriptors, and accounting metrics released. |

## Core Scheduling Dimensions

### 1. Scheduling Class Architecture
- Hybrid Priority / CFS-like policy: Tasks are organized into 64 priority bands (0 = lowest, 63 = highest).
- Real-time priority tasks (prio 48–63) receive preemptive fixed-priority scheduling.
- Normal tasks (prio 0–47) receive dynamic time-slice scheduling based on vruntime tracking and nice level adjustments.

### 2. Run-Queue Structure
- Bounded capacity per CPU core (`MAX_RUNQUEUE_CAPACITY = 1024` or `4096`).
- Bitmap-indexed priority arrays allowing $O(1)$ selection of highest-priority runnable process.
- Lock-free / fine-grained spinlock protection per run queue to eliminate cross-core lock contention.

### 3. Time-Slice Policy
- Default quantum: 10 timer ticks (100ms at 100Hz tick rate).
- Real-time tasks receive configurable quanta (up to infinity for `SCHED_FIFO` or round-robin rotation for `SCHED_RR`).
- Quanta are refreshed upon state transition back to `READY`.

### 4. Priority and Nice Semantics
- POSIX-compliant nice range (-20 to +19) mapped onto kernel priority bands.
- Priority boosting applied automatically to starved tasks in lower bands.

### 5. Preemption Points
- Timer interrupt ticks (`timer_tick`).
- Explicit voluntary yields (`sched_yield`).
- Task wakeup on higher-priority events (`wake_process`).
- Syscall return boundaries (`sys_exit`, `sys_yield`).

### 6. Fairness Metric and Starvation Prevention
- Virtual runtime tracking (`vruntime`) guarantees fair CPU time distribution.
- Starvation threshold (`DEFAULT_STARVATION_THRESHOLD_TICKS = 50` ticks).
- Periodic priority boosting shifts waiting processes to higher priority bands after threshold expiry.

### 7. Priority Inversion Handling
- Priority inheritance protocol (PIP) elevates lock holder priority to match highest waiter priority.

### 8. CPU Affinity Behavior
- 64-bit CPU affinity bitmask (`cpu_affinity`) per PCB.
- Scheduler core selects least-loaded legal CPU core during `mark_ready`.
- Strict affinity enforcement prevents illegal cross-CPU migration.

### 9. Zero Runnable Tasks Behavior
- When no task is in `READY` state for a CPU core, the scheduler executes the CPU idle loop (`hlt` instruction on x86_64, `wfi` on ARM64) to save energy.