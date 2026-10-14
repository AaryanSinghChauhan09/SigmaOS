# SigmaOS Run Queue Policy Specification

## Capacity & Bounds
- Every CPU run queue is bounded (`MAX_RUNQUEUE_CAPACITY = 1024` entries per core).
- Max process table size is bounded (`MAX_PROCESS_TABLE_SIZE = 4096`).
- Enqueue attempts exceeding queue capacity fail cleanly with `RunQueueError::CapacityExceeded`.

## Synchronization & Race-Freedom
- Run queue state updates are protected under scheduler locks.
- Duplicate task enqueue is prevented via task ID checks (`RunQueueError::DuplicateTask`).
- Task removal/dequeue explicitly updates total task counts and 64-bit priority selection bitmask.

## Lost-Wakeup Prevention
- Sleeping tasks maintain a wake notification flag (`lost_wakeup_signaled`).
- If a wakeup signal arrives while a process transitions from `RUNNING` to `SLEEPING`, the flag causes the process to immediately return to `READY` state without blocking.
