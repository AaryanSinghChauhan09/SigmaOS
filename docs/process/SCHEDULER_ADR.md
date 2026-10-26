# ADR 002: SigmaOS Scheduler Subsystem Architecture

## Status
Accepted

## Context
SigmaOS requires a safe, observable, and testable scheduling subsystem operating under `#![no_std]` constraints. Prior experiments lacked formal contracts for lost-wakeup prevention, run-queue capacity limits, and lifecycle invariants required for a deterministic boot-to-interactive-shell path.

## Decision
1. **Canonical State Machine**:
   Adopt an authoritative 7-state process lifecycle state machine (`NEW`, `READY`, `RUNNING`, `SLEEPING`, `STOPPED`, `ZOMBIE`, `REAPED`). No process can be scheduled in ZOMBIE or REAPED states.
2. **Bounded Per-CPU Run Queues**:
   Implement per-CPU `BoundedRunQueue` instances with hard limits (`MAX_RUNQUEUE_CAPACITY = 1024`) and 64 priority levels indexed by a 64-bit bitmap for $O(1)$ highest-priority lookup.
3. **Lost-Wakeup Prevention**:
   Maintain a `wake_event_pending` boolean inside process control blocks. If a wakeup signal occurs prior to sleep state commit, `sleep_process` clears `wake_event_pending` and returns immediately in `READY` state.
4. **Starvation Prevention & Fairness**:
   Track process `vruntime` and tick count. Tasks remaining unexecuted beyond `DEFAULT_STARVATION_THRESHOLD_TICKS` (50 ticks) receive temporary priority elevation.
5. **CPU Affinity & Migration**:
   Enforce `CpuAffinityMask` validation upon process creation and state changes.

## Consequences
- Unbounded queue allocation is eliminated; queue overflow returns `RunQueueError::CapacityExceeded`.
- Race conditions between sleep state transitions and wakeup interrupts are structurally impossible.
- All scheduler changes must supply evidence-backed tests and pass `./run_sigma_tests.sh`.
