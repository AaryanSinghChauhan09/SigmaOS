# SigmaOS Run Queue Subsystem Policy

## Bounded Run Queue Invariants

1. **Bounded Capacity**:
   - Each CPU core maintains a `BoundedRunQueue` with a fixed maximum capacity (`MAX_RUNQUEUE_CAPACITY = 1024`).
   - Attempts to enqueue beyond capacity return `RunQueueError::CapacityExceeded`.

2. **Race-Free State**:
   - Enqueue/dequeue operations maintain exact internal counters (`total_tasks`).
   - Bitwise operations on `active_bitmap` guarantee atomic priority lookup.

3. **Single Scheduling Guard**:
   - A task cannot exist in multiple run queues simultaneously or be scheduled twice.
   - Enqueue checks for duplicates returning `RunQueueError::DuplicateTask`.

4. **Starvation Prevention**:
   - `boost_starved_tasks` scans tasks waiting longer than `starvation_threshold_ticks` and elevates their priority queue level.
