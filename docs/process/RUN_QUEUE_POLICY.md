# SigmaOS Run Queue Subsystem Policy

## Bounded Run Queue Capacity

1. **Capacity Limit**:
   Each CPU core run queue (`BoundedRunQueue`) is strictly bounded to `MAX_RUNQUEUE_CAPACITY` (1024 tasks). Attempting to enqueue beyond this capacity returns `RunQueueError::CapacityExceeded`.

2. **Race-Free Enqueue and Dequeue**:
   All run queue mutations are synchronized via core-local lock discipline or atomic state operations. Concurrent access is free of data races and lost-update bugs.

3. **Multi-Priority Queueing**:
   Supports 64 discrete priority levels (`0..63`). Tasks are enqueued in FIFO order per priority tier, selected using a 64-bit non-empty bitmap mask.

4. **Lost-Wakeup Prevention**:
   When sleeping on a wait queue, tasks check wake conditions under the wait queue lock before changing state to `SLEEPING`. Wakeup operations check state atomically and enqueue to `READY` if sleeping.

5. **Single-Scheduling Invariant**:
   A process descriptor can exist in at most one run queue at any given instant. Enqueue checks prevent double-scheduling of tasks.
