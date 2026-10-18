# SigmaOS Scheduler Policy & Specification (`docs/process/SCHEDULER_POLICY.md`)

## 1. Scope & Objective

This document defines the authoritative scheduling policies, run-queue mechanics, priority and nice semantics, preemption points, fairness metrics, starvation bounds, priority-inversion prevention, CPU affinity governance, and idle loop behavior for the SigmaOS kernel process scheduler.

## 2. Standard Scheduler Contracts

### 2.1 Scheduling Class
SigmaOS implements a **hybrid priority / CFS-like scheduler**:
- **Multi-Level Priority Queues**: 64 priority levels (`0` highest/real-time to `63` lowest/idle).
- **Virtual Runtime (vruntime) Tracking**: Within each priority queue level, processes track accumulated `vruntime` and total scheduled ticks to preserve CFS-style execution fairness among peer tasks.

### 2.2 Run-Queue Structure
- **Bounded Multi-Level Run Queue (`BoundedRunQueue`)**: Static capacity limit of 1024 tasks per core queue.
- **Priority Bitmap**: 64-bit integer (`u64`) mask where bit `N` is set if priority bucket `N` contains active runnable processes, guaranteeing $O(1)$ selection via `leading_zeros()`.
- **Race-Free Enqueue/Dequeue**: Atomic run queue state updates and duplicate task prevention.

### 2.3 Time-Slice Policy
- **Quantum Duration**: Default default quantum of 10 ticks (100 ms at 100 Hz timer frequency).
- **Quantum Expiry**: At each timer tick, `remaining_quantum_ticks` decrements by 1. When `remaining_quantum_ticks == 0`, the process yields back to `READY` state and is re-enqueued at the tail of its priority queue with a refreshed quantum.

### 2.4 Priority and Nice Semantics
- **Priority Range**: 0 to 63. Real-time priorities span `0..15`, interactive priorities `16..47`, and background/idle priorities `48..63`.
- **Nice Conversion**: POSIX `nice` values (`-20` to `+19`) map linearly onto priority range `16..47`, adjusting default time-slice allocations and vruntime scaling.

### 2.5 Preemption Points
- **Interrupt Return Points**: Preemption checks occur upon returning from kernel timer interrupts or hardware I/O interrupts.
- **Voluntary Yield**: Process calls `sched_yield()` or enters `SLEEPING`/`STOPPED` state.
- **Higher Priority Arrival**: When a higher-priority task enters `READY` state on a CPU executing a lower-priority task, a preemption request flag is set for the next interrupt boundary.
- **Interrupt Preemption Restrictions**: Preemption is strictly disabled inside critical spinlock regions, interrupt service routines (ISRs), and context switch trampolines.

### 2.6 Fairness Metric
- **Runnable vs. Waiting Fairness**: Measured as the ratio of scheduled CPU ticks ($T_{scheduled}$) to waiting run-queue ticks ($T_{waiting}$) for processes of equal priority level:
  $$\text{Fairness Ratio} = \frac{T_{scheduled}}{T_{waiting} + 1}$$
- **vruntime Convergence**: Processes with lower `vruntime` are prioritized during run-queue selection to ensure balanced progress.

### 2.7 Starvation Prevention
- **Starvation Metric**: Measured by `current_tick - enqueue_tick`.
- **Starvation Threshold**: Default 50 ticks (500 ms at 100 Hz).
- **Priority Boosting**: During `timer_tick()`, any task waiting in a queue $\ge 50$ ticks receives a temporary priority boost (+10 levels, capped at 62) and its `enqueue_tick` is reset to `current_tick`.

### 2.8 Priority-Inversion Handling
- **Priority Inheritance Protocol (PIP)**: When a high-priority task blocks waiting for a lock held by a lower-priority task, the lower-priority lock holder temporarily inherits the priority of the waiting high-priority task until it releases the resource.

### 2.9 CPU Affinity Behavior
- **Bitmask Representation**: 64-bit integer (`u64`) affinity mask where bit `C` permits scheduling on CPU core `C`.
- **Load Balancing**: `select_best_cpu` evaluates all cores permitted by `cpu_affinity` and selects the core with minimal run-queue depth.
- **Affinity Exhaustion**: Enqueuing a process with a mask matching no online CPUs fails with `SchedulerError::InvalidCpuAffinity`.

### 2.10 Zero Runnable Tasks Behavior
- **Idle Loop State**: When all run queues on a CPU core are empty, the scheduler transitions the CPU to the kernel idle task (`PID 0`).
- **Power Management**: The idle task executes low-power halt instructions (`hlt` or `wfi`) in a loop until the next hardware timer or I/O interrupt wakes the CPU core.

## 3. POSIX State Machine Enforcement

SigmaOS enforces the 7-state POSIX process lifecycle:
```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

- **NEW**: PID allocated, address space & credentials initialized.
- **READY**: Placed in `BoundedRunQueue`.
- **RUNNING**: Executes on CPU with time-slice preemption.
- **SLEEPING**: Blocked on wait queue with lost-wakeup protection.
- **STOPPED**: Suspended by SIGSTOP or debugger.
- **ZOMBIE**: Exit code retained until reaped; orphans reparented to PID 1.
- **REAPED**: PID, process control block, and kernel accounting released.

## 4. Verification

Scheduler policy compliance, preemption safety, context switch integrity, fairness benchmarks, and affinity semantics are continuously validated by unit and integration suites under `tests/scheduler/` and `./run_sigma_tests.sh`.
