# SigmaOS Run Queue Policy Specification

## Run Queue Architecture
The run queue is a core data structure (`BoundedRunQueue`) instantiated per CPU core to manage tasks eligible for execution (`READY` state).

## Key Policy Rules

### 1. Bounded Capacity
- Every run queue has a strict maximum capacity limit (`MAX_RUNQUEUE_CAPACITY = 1024` or `4096`).
- Attempting to enqueue a task into a full queue returns `RunQueueError::CapacityExceeded`.
- Run queues never dynamically allocate unbounded heap memory during interrupt or scheduling contexts.

### 2. Race-Free Operations
- Enqueue and dequeue operations are guaranteed race-free.
- Each CPU core owns its primary run queue; work-stealing across cores acquires fine-grained per-queue spinlocks following strict lock ordering rules.

### 3. Scheduling Fairness & Selection
- Tasks are indexed across 64 priority levels using a 64-bit bitmap for $O(1)$ priority selection.
- Starved tasks in lower priority levels receive periodic priority boosts during `timer_tick` when waiting time exceeds `DEFAULT_STARVATION_THRESHOLD_TICKS`.

### 4. Lost-Wakeup Prevention
- When a task transitions from `SLEEPING` or `RUNNING` to `READY`, the run queue verifies pending wake events (`wake_event_pending`) to prevent missed wakeups.
