# SigmaOS Scheduler Policy and Contracts

## Overview

The SigmaOS scheduler governs process execution across multi-core systems, balancing CPU fairness, interactive responsiveness, priority levels, and preemption safety.

## Run Queue Structure and Priority Rules

1. **Multi-Level Priority Queues:** Run queues support 64 distinct priority levels (0 highest priority, 63 lowest).
2. **Bit Selection:** Standard 64-bit bitmap selection (`trailing_zeros`) locates non-empty priority queues in $O(1)$ time.
3. **Starvation Prevention:** Periodic starvation boosting elevates low-priority tasks that have been starved of CPU time past a defined threshold.

## Preemption and Fairness Metrics

1. **Preemption Points:** Context switching occurs on timer interrupts (quantum expiration), blocking operations (I/O, locks, sleep), or when a higher-priority thread becomes runnable.
2. **Time Slice / Quantum:** Adaptive timeslices based on nice level and task class.
3. **Lost Wakeup Prevention:** Enqueued tasks use wait-queue atomic guards to ensure wakeups occurring during transition to `SLEEPING` are not missed.
4. **Zero Runnable Tasks Behavior:** When all queues are empty, the CPU executes the kernel idle task (`hlt`/`wfi` wait loop).
