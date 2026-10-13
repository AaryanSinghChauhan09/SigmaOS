# SigmaOS Run Queue Policy (`RUN_QUEUE_POLICY.md`)

## 1. Overview
This policy governs the design, capacity limits, concurrent safety, and priority indexing of the `BoundedRunQueue` data structure in SigmaOS.

## 2. Queue Structure & Bounds
- **Capacity Constraint**: Each CPU core maintains an independent `BoundedRunQueue` with a bounded capacity (default `1024` entries).
- **Priority Dimensions**: 64 discrete priority queues (`queues: [VecDeque<TaskRunRef>; 64]`).
- **Active Bitmap**: 64-bit integer (`active_bitmap: u64`) where bit $N$ is set if priority queue $N$ contains at least 1 task.

## 3. Operational Invariants
- **No Duplicate Enqueue**: A PID must exist at most once across all priority levels in a run queue. Enqueuing a duplicate PID returns `RunQueueError::DuplicateTask`.
- **Bounded Enqueue**: Attempting to enqueue when `total_tasks >= capacity` strictly returns `RunQueueError::CapacityExceeded`.
- **$O(1)$ Priority Dispatch**: `dequeue_next` uses `active_bitmap.leading_zeros()` / bitwise iteration to select the highest active priority queue in $O(1)$ time.

## 4. Starvation Prevention
- The run queue provides `boost_starved_tasks(current_tick, starvation_threshold_ticks)`.
- Tasks waiting in lower priority queues longer than the starvation threshold have their priority boosted by +10 levels (capped at level 62).
- The bitmap is updated immediately upon priority migration.
