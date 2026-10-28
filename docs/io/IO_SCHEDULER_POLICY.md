# SigmaOS Multi-Queue I/O Scheduler Policy

## Objectives
- Multi-queue priority scheduling (RealTime, BestEffort, Idle).
- Strict adherence to the 10-stage lifecycle contract.
- Starvation prevention and bounded queue depths.

## Policy Rules
1. **Priority Servicing**: RealTime queue serviced before BestEffort, BestEffort serviced before Idle.
2. **Queue Depth**: Maximum total queued requests restricted by `max_queue_depth`; overflow returns `SchedulerError::QueueFull`.
3. **Stage Transitions**: Out-of-order stage transitions rejected with `SchedulerError::InvalidStageTransition`.
