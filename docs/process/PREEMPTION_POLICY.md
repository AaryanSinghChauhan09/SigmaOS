# SigmaOS Preemption Policy (`PREEMPTION_POLICY.md`)

## 1. Overview
Preemption guarantees that high-priority tasks and interactive user applications obtain prompt CPU execution without being blocked by low-priority compute tasks.

## 2. Preemption Rules
- **Time-Slice Expiration**: Each process receives a quantum of 10 ticks. When quantum expires, preemption flag is set and `schedule_next()` yields the CPU.
- **Interrupt-Driven Preemption**: Timer interrupt and device ISRs evaluate if a newly woken task has higher priority than the currently running task. If so, `need_resched` flag triggers context switch upon return from interrupt.
- **Voluntary Yield**: System calls (`sys_sched_yield`) allow processes to yield remaining quantum voluntarily.

## 3. Nested Priorities & Inversion
- Real-time priority levels (60..63) preempt normal interactive tasks (20..59) and batch background tasks (0..19).
- Priority inversion scenarios are mitigated by temporary priority inheritance / starvation boosting.
