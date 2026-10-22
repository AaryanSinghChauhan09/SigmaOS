# SigmaOS Preemption Subsystem Policy

## Preemption Rules

1. **Time-Slice Expiry**:
   When a running process exhausts its time-slice quantum (10 ticks by default), the timer interrupt flags `need_resched`. The scheduler triggers a voluntary context switch at the next kernel exit or preemption point.

2. **Interrupt-Driven Preemption**:
   When a higher-priority task transitions to `READY` (e.g. following I/O interrupt completion), preemption occurs immediately if the current process has lower priority.

3. **Voluntary Yield**:
   Tasks calling `sched_yield` relinquish remaining time-slice quantum, move to the tail of their priority queue in `READY` state, and trigger context switch.

4. **Interrupt Context Restrictions**:
   Interrupt handlers must never block, sleep, or trigger a direct context switch. Preemption requests from ISRs set deferred flags (`need_resched`) processed upon ISR return.

5. **Nested Priority and Inversion**:
   Mutex holders involved in priority inversion inherit the waiting task's priority dynamically until lock release.
