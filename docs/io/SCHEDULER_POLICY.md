# SigmaOS I/O Scheduler Policy

## Subsystem Overview
The I/O Scheduler (`src/io/scheduler.rs`, `src/io/scheduler_queue.rs`) organizes, prioritizes, merges, and dispatches pending I/O requests to target devices using pluggable scheduling algorithms (e.g., Deadline, BFQ/Elevator, FIFO, Priority).

## Key Requirements & Invariants

1. **Race-Free Enqueue / Dequeue**: Concurrent request enqueueing and dequeueing are synchronized without deadlock risk.
2. **Bounded Request Queue**: Total queued requests cannot exceed `max_capacity`. Enqueueing to a full queue returns `Err(SchedulerError::QueueFull)`.
3. **Starvation & Lost-I/O Prevention**: Deadline bounds (e.g., 500ms read deadline, 5,000ms write deadline) prevent starvation of low-priority requests.
4. **Request Merging & Sorting**: Sector-adjacent requests are merged to optimize sequential throughput for block storage devices.
5. **Exact-Once Execution**: Requests dequeued for execution are marked active; duplicate execution returns `Err(SchedulerError::AlreadyScheduled)`.
