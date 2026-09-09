# AI Agent Time Sharing System Management Architecture

## Executive Overview

Time Sharing System Management in SigmaOS divides CPU processing time into discrete quanta (time slices) allocated across concurrent processes and threads. Implemented across `src/kernel/roundrobin.rs`, `src/process/scheduler.rs`, `src/performance/eevdf.rs`, and `src/kernel/sched/sigma_mlfq.rs`, SigmaOS combines Round-Robin quantum time-slicing, EEVDF (Earliest Eligible Virtual Deadline First) virtual runtime tracking (`vruntime_us`), and MLFQ (Multi-Level Feedback Queue) exponential backoff priority decay to deliver low-latency interactive execution and fair CPU resource distribution.

This document serves as the architectural reference for AI coding agents inspecting, configuring, or extending CPU time sharing and time-slicing algorithms in SigmaOS.

---

## Time Sharing Architecture & Scheduling Engines

```
                                +-----------------------------------+
                                |      Task Creation / Submission   |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    EEVDF / MLFQ Scheduler         |
                                |  (Virtual Deadline Calculation)   |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | High-Priority Quantum |   | Normal Quantum    |   | Low-Priority Quantum  |
            | (e.g., 20ms)          |   | (e.g., 10ms)      |   | (e.g., 5ms)           |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |   Preemption Timer Interrupt (LAPIC)|
                                |  time_slice_remaining_ms Decrement|
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                | Context Switch / Queue Rotation   |
                                +-----------------------------------+
```

### Core Time Sharing Components

1. **Round-Robin Quantum Engine (`src/kernel/roundrobin.rs`)**:
   - `calculate_sched_rr_quantum(priority)`: Calculates quantum based on process priority (e.g., high priority tasks receive larger quanta, e.g. 20ms, while background tasks receive 5ms).
   - `time_slice_remaining_ms`: Decremented on every timer interrupt tick (1ms resolution). When it reaches 0, preemption triggers and task moves to the back of the run queue.

2. **EEVDF Virtual Runtime Engine (`src/performance/eevdf.rs`)**:
   - `virtual_deadline = virtual_runtime + (time_slice / weight)`
   - Dynamically adjusts time-slice length based on `latency_nice` hints. Lower `latency_nice` values yield smaller quanta for rapid response times.

3. **Multi-Level Feedback Queue (MLFQ) (`src/kernel/sched/sigma_mlfq.rs`)**:
   - Multi-queue hierarchy where Queue $i$ has quantum length $2^i \text{ ms}$. Tasks that exhaust their full quantum without blocking are demoted to lower-priority queues with larger quanta.

---

## Quantum Formula & Fair Share Bounds

For $N$ runnable tasks with weights $w_1, w_2, \dots, w_N$, the allocated time slice quantum $Q_i$ for task $i$ during target scheduling period $T_{sched}$ is:

$$Q_i = T_{sched} \times \frac{w_i}{\sum_{j=1}^{N} w_j}$$

When $Q_i < Q_{min}$ (default $1\text{ms}$), $Q_i$ is clamped to $Q_{min}$ to limit context-switch overhead.

---

## Zero-Allocation Guardrails

AI agents tuning time-sharing algorithms must follow these zero-allocation rules:
- Preemption tick decrements operate in $O(1)$ atomic or stack-register operations.
- Queue rotations inside timer ISRs operate via array or ring-buffer pointer updates without heap allocation.
- `vruntime_us` math uses integer fixed-point arithmetic (`u64`).

---

## Related Architectural References
- `src/kernel/roundrobin.rs` - Round-Robin and POSIX `SCHED_RR` implementation.
- `src/performance/eevdf.rs` - EEVDF scheduler and virtual deadline tracking.
- `src/kernel/sched/sigma_mlfq.rs` - Multi-Level Feedback Queue scheduler.
- `docs/AI_AGENT_KERNEL_MANAGEMENT_ARCHITECTURE.md` - Overall kernel scheduler architecture.
