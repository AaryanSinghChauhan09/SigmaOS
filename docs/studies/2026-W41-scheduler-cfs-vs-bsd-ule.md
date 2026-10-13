# Study: Scheduler Fairness Models (Linux CFS vs. FreeBSD ULE)

## Reference Systems

### Linux Completely Fair Scheduler (CFS)
- Uses a red-black tree indexed by virtual runtime (`vruntime`).
- Allocates CPU time dynamically based on task nice levels and latency targets.
- Enforces strict fairness; tasks with smaller `vruntime` are prioritized for execution.

### FreeBSD ULE Scheduler
- Uses multiple priority runqueues designed for SMP multicore systems.
- Differentiates between interactive and CPU-bound threads using an interactivity score based on sleep time versus runtime history.
- Minimizes CPU cache migration overhead via affinity steering.

## Mechanism & Tradeoffs

| Feature | Linux CFS | FreeBSD ULE |
|---|---|---|
| **Data Structure** | Red-Black Tree (`rb_node`) | Bounded Priority RunQueues |
| **Interactivity Detection** | Granularity & Latency Target | Sleep / Runtime History Score |
| **SMP Affinity** | Load Balancing Domains | Per-CPU Runqueues & CPU Topology Steering |
| **Selection Time Complexity** | $O(\log N)$ or $O(1)$ cached leftmost | $O(1)$ Bitmap Lookup |

## SigmaOS Current State

- **Implementation:** `src/scheduler/run_queue.rs` (`BoundedRunQueue` with 64 priority levels and 64-bit bitmap selection) and `src/scheduler/core.rs` (`SchedulerCore` with POSIX 6-state lifecycle tracking and CFS vruntime tracking in `src/kernel/tier2_usability_stack.rs`).
- **Tests:** `tests/scheduler/core.rs`, `tests/scheduler/fairness/fairness_test.rs`.
- **Gaps:** Starvation prevention boosting under extreme multi-threaded workloads requires continuous regression verification.

## Decision

- **Adapt:** Maintain $O(1)$ priority bitmap scheduling for real-time low-latency tasks while incorporating CFS `vruntime` tracking for dynamic batch process fairness.

## Follow-up Action Items
- [x] Implement starvation boosting in `BoundedRunQueue`.
- [x] Validate scheduler context switch latencies in `tests/scheduler/context_switch/context_switch_test.rs`.
