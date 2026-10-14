# Study: Scheduler Fairness Models (Linux CFS vs. FreeBSD ULE)

## Reference Systems

- **Linux CFS (Completely Fair Scheduler)**:
  - Uses a red-black tree indexed by virtual runtime (`vruntime`).
  - Processes with lowest `vruntime` are selected first.
  - Weight is derived from `nice` values to scale `vruntime` progression.
- **FreeBSD ULE**:
  - Multi-queue architecture per CPU with interactivity scoring.
  - Interactivity heuristic compares CPU time vs. I/O sleep time.
  - Interactivity queues bypass batch queues for low latency response.
- **OpenBSD Scheduler**:
  - Simple round-robin runqueue with priority decay and starvation prevention.
- **Redox OS**:
  - Round-robin / priority queues in microkernel user space.

## Mechanism

- **Problem**: Balancing interactive responsiveness (desktop/GUI) with batch throughput while preventing process starvation under heavy load.
- **Data Structures**:
  - Red-black tree (`vruntime` ordered) for CFS.
  - Priority array with 64 priority levels and 64-bit bitmap selection for SigmaOS.
- **Scheduling Decision**:
  - Find highest priority non-empty queue in O(1) time using bitmask finding (`find_first_set` / `trailing_zeros`).
- **Priority Handling**:
  - Priority levels 0..63. Lower number = higher priority.
  - Starvation boosting moves aging low-priority threads to higher priority queues after starvation thresholds are reached.
- **Failure Modes**:
  - Priority inversion when high-priority tasks block on locks held by low-priority tasks.
  - Starvation of batch tasks under continuous interactive thread scheduling.

## SigmaOS Current State

- **Code**: `src/scheduler/run_queue.rs` (`BoundedRunQueue`) and `src/scheduler/core.rs` (`SchedulerCore`).
- **Tests**: `tests/scheduler/run_queue_test.rs`, `tests/scheduler/core.rs`, `tests/scheduler/fairness/fairness_test.rs`.
- **Runtime Evidence**: Standalone tests verify 64-priority level bitmap indexing and starvation boosting.
- **Gaps**: Lack of fine-grained `vruntime` latency target tracking like Linux CFS under multi-core SMP load.

## Decision

- **Adopt**: O(1) bitmap priority selection with 64 priority levels.
- **Adapt**: Interactive thread scoring inspired by FreeBSD ULE to dynamic dynamic priority calculation.
- **Reject**: Complex red-black tree insertion overhead in kernel hotpaths for early releases.

## Follow-up PRs

- [x] feat(scheduler): add BoundedRunQueue with 64 priority levels and bitmap selection
- [x] test(scheduler): add starvation boosting and lifecycle regression tests
- [ ] feat(scheduler): dynamic interactivity heuristic scoring for desktop threads
