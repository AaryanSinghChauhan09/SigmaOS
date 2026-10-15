# SigmaOS Run Queue Policy

## Structure and Capacity Bounds

1. **Per-CPU Run Queues**: Each CPU core maintains a `BoundedRunQueue`.
2. **Capacity Limit**: Maximum 1024 tasks per core queue. Attempts to enqueue past capacity return `RunQueueError::CapacityExceeded`.
3. **Priority Array**: 64 priority queues (`0..63`). Higher numbers indicate higher priority.
4. **Active Bitmap**: 64-bit integer (`u64`) where bit `N` is set if priority queue `N` contains runnable tasks, enabling O(1) highest-priority selection via `leading_zeros()` / `rev()`.

## Core Invariants

- **Duplicate Prevention**: A task with PID `P` cannot be enqueued twice in any run queue.
- **Lost-Wakeup Protection**: `sleep_process` checks `wake_event_pending` before transitioning to `SLEEPING`. If set, the process remains in `READY` state.
- **Race-Free State Handling**: State transitions to `READY` occur atomically after successful run-queue enqueue.
- **No Task Lost**: Every process context switch saves task state back into process table or run queue.
