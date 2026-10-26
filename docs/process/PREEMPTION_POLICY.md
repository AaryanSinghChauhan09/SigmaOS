# SigmaOS Preemption Policy Specification

## Purpose
This document defines time-slice expiration, interrupt-driven preemption, voluntary yield, and preemption safety boundaries in SigmaOS.

---

## 1. Time-Slice Quantum Expiry
- Every timer interrupt decrements the remaining time-slice ticks of the active running task on that CPU core.
- When ticks reach zero, the task's state is changed from `ProcessState::Running` to `ProcessState::Ready`, it is re-enqueued into its run queue, and `schedule_next` picks the next candidate.

## 2. Interrupt-Driven Preemption
- Arrival of a higher-priority task in a run queue triggers immediate preemption of lower-priority running tasks.
- Critical kernel sections guarded by `PreemptGuard` defer preemption until the guard drops.

## 3. Voluntary Yield Behavior
- A task calling `sys_yield()` voluntarily surrenders its CPU quantum, moving to `ProcessState::Ready` and allowing equal or higher priority tasks to execute.

## 4. Nested Priorities and Inversion Scenarios
- High-priority tasks preempt lower-priority tasks immediately unless a lock held by a lower-priority task triggers priority inheritance.
- Interrupt contexts (ISRs) execute on dedicated interrupt stacks and cannot be preempted by user-level threads.
