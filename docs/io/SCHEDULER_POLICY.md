# SigmaOS I/O Scheduler Policy

## 1. Overview
The I/O Scheduler subsystem (`src/io/scheduler.rs`, `src/io/scheduler_queue.rs`) enforces multi-queue request dispatching and tracks stage transitions across the authoritative 10-stage I/O lifecycle.

## 2. Priority Queues
Requests are scheduled according to priority levels:
1. **RealTime (`IoPriority::RealTime`):** Highest priority; evaluated first for latency-critical tasks.
2. **BestEffort (`IoPriority::BestEffort`):** Standard priority for userland reads/writes.
3. **Idle (`IoPriority::Idle`):** Low priority background synchronization and maintenance task processing.

## 3. Request Lifecycle State Machine
Each request moves sequentially through stages:
`Submit -> Validate -> Enqueue -> Execute -> Interrupt/DMA -> Complete -> Callback -> Reclaim -> Free` (or `Error`).

Attempting invalid stage jumps returns `SchedulerError::InvalidStageTransition`.

## 4. Limits and Bounded Queue Depth
- Total active and enqueued requests are capped by `max_queue_depth`.
- Submissions exceeding capacity return `SchedulerError::QueueFull`.
