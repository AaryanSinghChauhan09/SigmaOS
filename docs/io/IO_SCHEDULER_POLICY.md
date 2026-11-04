# SigmaOS I/O Scheduler Policy

## Objective

Schedule and prioritize block and character device requests across multiple queues to balance low-latency interactive responsiveness with high-throughput batch operations without starvation or lost requests.

## Requirements and Guarantees

### 1. Multi-Queue Priority Scheduling
- **RealTime (RT)**: Dispatched immediately with preemptive priority over standard tasks.
- **BestEffort (BE)**: Balanced round-robin or deadline-fair scheduling.
- **Idle**: Dispatched only when higher-priority queues are empty.

### 2. Race-Free Enqueue and Dequeue
- Submission and dispatch operate atomically or with synchronized locks.
- Request lifecycle transitions: `Submit -> Validate -> Enqueue -> Execute -> Complete -> Free`.

### 3. Request Reuse Policy
- Monotonic 64-bit request IDs.
- Completed requests transition to `Free` before ID slot reuse.

### 4. Bounded Metadata and Queue Depths
- Queue capacity is strictly bounded by `max_depth`.
- Allocations exceeding `max_depth` return `SchedulerError::QueueFull`.

### 5. Lost-I/O and Double-I/O Prevention
- In-flight requests are tracked in `active_requests`.
- Duplicate completions return `SchedulerError::RequestNotFound` or `InvalidStageTransition`.

### 6. Observable OOM and Starvation Telemetry
- Subsystem metrics report pending requests, active requests, and queue wait times.
