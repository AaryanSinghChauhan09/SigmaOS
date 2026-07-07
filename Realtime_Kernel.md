# SigmaOS Real-Time Kernel (PREEMPT_RT)

> Deterministic scheduling for robotics, industrial, and audio workloads.

## Overview

SigmaOS ships an optional **PREEMPT_RT kernel variant** that provides hard real-time guarantees. It implements priority inheritance mutexes, Earliest Deadline First (EDF) scheduling, and threaded interrupt handlers to minimize worst-case latency.

## Scheduling Classes

| Class        | Algorithm           | Use Case                    |
|-------------|---------------------|-----------------------------|
| FIFO        | First-in-first-out  | Audio DSP, PLC control      |
| RoundRobin  | Time-sliced FIFO    | Multiple RT tasks, fairness |
| Deadline    | Earliest Deadline   | Robotics, sensor fusion     |
| Normal      | CFS (default)       | Desktop workloads           |

## Priority Inheritance

The RT kernel implements the **Priority Inheritance Protocol** to prevent priority inversion:

1. When a high-priority task blocks on a mutex held by a low-priority task
2. The low-priority holder is **temporarily boosted** to the waiter's priority
3. On unlock, the holder's priority is **restored** to its original value
4. The highest-priority waiter is **woken next**

## Deadline Scheduler (EDF)

- **Admission control**: Verifies `Σ(runtime/period) ≤ 1.0` per CPU
- **Earliest Deadline First**: Always runs the task with the nearest deadline
- **Runtime budgeting**: Tasks are throttled if they exceed their runtime budget

## Threaded IRQs

All hardware interrupts are converted to **kernel threads** with configurable priorities:
- Allows preemption of interrupt handlers by higher-priority RT tasks
- Eliminates interrupt-induced latency spikes
- Each IRQ thread has CPU affinity for NUMA-aware scheduling

## Implementation

- **Source**: `kernel/rt/sigma_realtime_kernel.rs`
- **Language**: Rust (`no_std`)
- **Key APIs**:
  - `PriorityMutex::lock(task)` — lock with priority inheritance
  - `DeadlineScheduler::admit(task)` — admission control
  - `DeadlineScheduler::pick_next()` — EDF scheduling decision
  - `make_irq_threaded(irq, handler, priority)` — convert hardirq to thread

## Target Latency

| Metric            | Target        | Measured       |
|-------------------|---------------|----------------|
| Worst-case jitter | < 50 µs       | In development |
| IRQ response      | < 10 µs       | In development |
| Context switch    | < 5 µs        | In development |
