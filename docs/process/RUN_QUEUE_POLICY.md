# SigmaOS Run Queue Policy Specification

## Bounded Run-Queue Guarantees
1. **Bounded Queue Capacity**:
   - Each CPU core maintains a `BoundedRunQueue` with a fixed max capacity of 2048 tasks.
   - Reaching capacity returns `RunQueueError::RunQueueFull`.

2. **Race-Free Enqueue and Dequeue**:
   - Enqueue operations perform duplicate PID checks (`TaskAlreadyEnqueued`).
   - Bitmapped priority selection provides $O(1)$ lookup for non-empty queues.

3. **Task Single-Scheduling Invariant**:
   - A process PID can exist in at most one CPU run queue at any given moment.
   - Context switches atomically remove active task from run queue before marked `RUNNING`.

4. **Starvation Prevention Algorithm**:
   - `boost_starved_tasks` scans tasks waiting longer than `starvation_threshold_ticks` (default 50 ticks / 500ms).
   - Starved tasks are promoted to priority level 0 (highest) to guarantee immediate execution on next schedule tick.
