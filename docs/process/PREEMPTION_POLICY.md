# SigmaOS Preemption Policy

## Principles

1. **Time-Slice Expiry**:
   - Ticks decrement `remaining_quantum_ticks`.
   - When quantum reaches 0, the task relinquishes its execution state and returns to `READY`.

2. **Interrupt-Driven Preemption**:
   - High-priority task wakeups or timer interrupts check for preemption needs and set `need_resched` flags.

3. **Voluntary Yield**:
   - Tasks calling `sys_yield` immediately reset quantum and yield control to the next highest priority runnable task.

4. **Interrupt Context Restrictions**:
   - Memory allocation and blocking locks are strictly forbidden within interrupt service routines.
