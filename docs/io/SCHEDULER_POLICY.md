# SigmaOS I/O Scheduler Policy

## Overview
The I/O Scheduler (`IoScheduler`) provides multi-queue I/O scheduling supporting `RealTime`, `BestEffort`, and `Idle` priority queues, strictly enforcing the 10-stage I/O lifecycle contract.

## Requirements & Enforcements
- **10-Stage Lifecycle Validation**: Requests transition linearly through `Submit -> Validate -> Enqueue -> Execute -> InterruptDma -> Complete -> Callback -> Reclaim -> Free`. Invalid jumps return `SchedulerError::InvalidStageTransition`.
- **Priority Queueing**: RealTime requests are dispatched ahead of BestEffort and Idle requests.
- **Queue Depth Bounds**: Enforces `max_queue_depth` limits returning `SchedulerError::QueueFull`.
- **Double-I/O Prevention**: Prevents duplicate request IDs and double-execution.
- **Resource Recovery**: Reclaims active request metadata upon completion or error recovery.
