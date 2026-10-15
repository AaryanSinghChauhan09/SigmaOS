# SigmaOS Preemption Policy

## Time-Slice and Preemption Trigger Rules

1. **Quantum Expiry**: Tasks receive a default quantum of 10 ticks. At each timer tick, `remaining_quantum_ticks` decrements. When it reaches 0, the process yields back to `READY` state and is re-enqueued.
2. **Voluntary Yield**: Tasks can voluntarily yield execution or enter `SLEEPING`/`STOPPED` states.
3. **Nested Priority Preemption**: When a higher priority task becomes `READY` on a core executing a lower priority task, preemption is flagged for the next interrupt return point.
4. **Preemption Restrictions**:
   - Preemption is disabled during critical kernel spinlocks, interrupt context, or hardware context switches.
   - Core state transitions must remain atomic across preemption triggers.
