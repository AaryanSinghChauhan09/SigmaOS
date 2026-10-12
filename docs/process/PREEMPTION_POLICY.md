# SigmaOS Preemption Policy Specification

## Preemption Rules
1. **Time-Slice Expiration**:
   - Each `RUNNING` process has a quantum counter (`remaining_quantum_ticks`, default 10 ticks).
   - Upon tick expiration (`remaining_quantum_ticks == 0`), task transitions back to `READY` and is re-enqueued at tail of its priority queue.

2. **Interrupt-Driven Preemption**:
   - High-priority task unblocked by an I/O or hardware interrupt preempts lower-priority running task at interrupt return point.

3. **Voluntary Yield**:
   - Tasks calling `sched_yield` forfeit remaining time-slice and are re-enqueued.

4. **Context Switch Restrictions**:
   - Interrupt service routines (ISRs) cannot invoke blocking scheduler calls.
   - Preemption is disabled while holding core run-queue or process-table locks.
