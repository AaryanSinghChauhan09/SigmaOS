# SigmaOS I/O Scheduler Policy

## Objectives
Manage request submission, prioritization, execution dispatch, and completion reclamation across device storage and block queues.

## Multi-Queue Architecture
1. **Priority Classes**:
   - **RealTime (RT)**: High-priority, latency-sensitive I/O operations (e.g., paging, system log flushes).
   - **BestEffort (BE)**: Standard interactive and batch application I/O.
   - **Idle**: Background maintenance and opportunistic indexing I/O.
2. **10-Stage State Machine Enforcement**:
   - Requests transition sequentially through `Submit -> Validate -> Enqueue -> Execute -> InterruptDma -> Complete -> Callback -> Reclaim -> Free`.
   - Invalid stage skips or backward transitions fail with `InvalidStageTransition`.
3. **Queue Depth Bounds**: Enforcing total queued request caps across RT, BE, and Idle queues. Queue saturation returns `QueueFull`.
4. **Error Recovery**: Failed operations move to `ErrorRecovery` stage, recording error codes without corrupting scheduler state.
