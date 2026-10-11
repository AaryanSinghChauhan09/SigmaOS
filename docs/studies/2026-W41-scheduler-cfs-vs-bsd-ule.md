# Study: Scheduler Fairness Models

## Reference Systems

### Linux CFS (Completely Fair Scheduler)
- **Concept**: Models an ideal multi-tasking CPU on hardware by assigning virtual runtime (`vruntime`) to tasks.
- **Data Structures**: Red-Black Tree indexed by `vruntime` (`rb_node`).
- **Priority**: Translates `nice` values (-20 to +19) into weights (`sched_prio_to_weight`). Higher weight means `vruntime` increases more slowly.
- **Latency / Granularity**: Configurable target latency (`sysctl_sched_latency`) and min granularity.

### FreeBSD ULE
- **Concept**: Multi-queue scheduler designed for SMP scale and interactive responsiveness.
- **Data Structures**: Three priority queues per CPU (Current, Next, Idle) with interactive runqueues based on priority scores.
- **Priority**: Priority ranges (0-255): Realtime (0-63), Interrupt (64-159), Timeshare (160-223), Idle (224-255).
- **Interactivity**: Dynamic interactivity score computed from sleep time vs. run time. High score bypasses timeshare queue.

### OpenBSD Scheduler
- **Concept**: Simple priority-based round-robin scheduler with decay-based priority adjustment.
- **Data Structures**: Array of run queues per priority level.
- **Priority**: Adjusted periodically based on CPU usage (`estcpu`).

### Redox OS Scheduler
- **Concept**: Capability-aware, scheme-centric cooperative/preemptive event-driven scheduler.
- **Data Structures**: Round-robin run queue per priority class with kernel scheme event triggers.

---

## Mechanism Comparison & Technical Analysis

| Feature | Linux CFS | FreeBSD ULE | OpenBSD | Redox OS | SigmaOS Current |
|---|---|---|---|---|---|
| **Primary Metric** | `vruntime` | Priority score + Interactivity | `estcpu` decay | Scheme event / Queue | `vruntime` + priority |
| **Data Structure** | Red-Black Tree | 3-Queue array / CPU | Multi-level runqueue | Priority list | RB-tree / Ring queue |
| **Interactive Heuristic** | Latency target / min granularity | Sleep/Run ratio score | CPU usage history | Scheme wake events | Yield timer score |
| **Preemption** | `vruntime` delta > granularity | Interactive score threshold | Quantum expiration | Co-operative / Timer | Quantum + preemption |

---

## SigmaOS Current State

- **Implementation**: `src/kernel/tier2_usability_stack.rs` (`SovereignCfsSchedulerEngine`).
- **Data Structures**: Red-Black tracking model with task states, `vruntime` tracking, and weight scaling.
- **Unit Tests**: Covered in `tests/os_components_tests.rs` and `src/kernel/tier2_usability_stack.rs`.
- **Runtime Evidence**: Verified via standalone unit tests (`rustc --test`).
- **Gaps**:
  1. Lacks FreeBSD-style interactive sleep/run dynamic scoring for desktop input thread prioritization.
  2. Needs starvation prevention metric verification under asymmetric thread loads.

---

## Decision

- **Adopt**: Linux CFS `vruntime` model as primary CPU scheduling foundation for predictable fair-share execution.
- **Adapt**: FreeBSD ULE interactivity scoring for user interface threads (Zenith Desktop / input event loops) to bypass batch queues.
- **Reject**: Unweighted round-robin scheduling for general kernel processes.

---

## Follow-up Action Items & PR Backlog

- [x] `feat(scheduler): implement vruntime tracking and weight scaling`
- [ ] `feat(scheduler): add FreeBSD ULE interactive dynamic score boost`
- [ ] `test(scheduler): add starvation regression test under asymmetric load`
- [ ] `docs(scheduler): update kernel scheduler architecture specification`
