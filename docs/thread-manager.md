# SigmaOS Thread Manager Contract & Architecture

## Overview

The SigmaOS Thread Manager is the core subsystem responsible for managing thread lifecycles, states, CPU context, stack limits, wait channels, and lock order tracking across SigmaOS processes and kernel threads.

## Thread States

A thread in SigmaOS moves through explicit, well-defined lifecycle states:

- **`New`**: Thread has been allocated and initialized, but has not yet been placed on a run queue.
- **`Ready`**: Thread is runnable and waiting in a scheduler run queue for CPU allocation.
- **`Running`**: Thread is actively executing on a CPU core.
- **`Blocked`**: Thread is sleeping on a wait channel waiting for an event, lock, or timer expire.
- **`Suspended`**: Thread is paused by explicit administrative/control action (e.g. signal or debugger).
- **`Zombie`**: Thread has terminated execution but its exit status has not yet been reaped by join/wait.
- **`Terminated`**: Thread resources (stack, ID, metadata) have been fully reclaimed and reaped.

### Legal State Transitions

```text
       ┌──────┐
       │ New  │
       └──┬───┘
          │ (enqueue)
          ▼
       ┌──────┐ (schedule) ┌─────────┐
    ┌─►│Ready ├───────────►│ Running │
    │  └──────┘ ◄──────────┤         │
    │     ▲     (preempt)  └──┬──┬───┘
    │     │                   │  │
    │     │ (wake/unblock)    │  │ (block/sleep)
    │  ┌──┴────┐              │  ▼
    │  │Blocked│◄─────────────┤┌─────────┐
    │  └───────┘              ││ Blocked │
    │                         │└─────────┘
    │  ┌─────────┐            │
    └──┤Suspended│◄───────────┤ (suspend)
       └─────────┘            │
                              │ (exit/terminate)
                              ▼
                         ┌────────┐
                         │ Zombie │
                         └───┬────┘
                             │ (reap/join)
                             ▼
                        ┌──────────┐
                        │Terminated│
                        └──────────┘
```

## Ownership Model

SigmaOS supports four thread ownership categories:

1. **Kernel Threads (`ThreadOwnership::Kernel`)**: Dedicated background execution contexts operating exclusively in kernel space without user address space mappings.
2. **User Threads (`ThreadOwnership::User`)**: Lightweight user-space threads bound to execution contexts within user address spaces.
3. **Process Threads (`ThreadOwnership::Process`)**: Primary process execution threads associated with a process group and thread group ID (`TGID`).
4. **Detached Threads (`ThreadOwnership::Detached`)**: Independent threads whose resources are automatically reclaimed upon termination without requiring an explicit `join()`.

## Guarantees & Constraints

- **Preemption Model**: Fully preemptive execution model with configurable quantum and timer-driven scheduler ticks.
- **Priority Ranges**: 140 priority levels (0–99 Real-Time, 100–139 Normal/Nice).
- **Affinity Semantics**: Mask-based CPU core binding with multi-core load balancer hooks.
- **Stack Limits**: Mandatory kernel stack (default 16KB/32KB) and user stack bounds validation with overflow protection guards.
- **Cancellation Behavior**: Deferred and asynchronous cancellation points with cleanup handler hooks.
- **Wait Channels**: Lost-wakeup prevention using atomic state-and-wait-channel lock synchronization.
- **Lock Order Tracking**: Runtime lock graph analysis to detect and warn on potential lock inversions / deadlocks.

## Non-Goals

- **POSIX Compliance Claim**: This subsystem does not claim full POSIX thread (`pthread`) compliance until libc-facing syscall wrappers and Ring 3 execution paths are end-to-end verified with integration tests.

## Acceptance Criteria

- `cargo check --lib` passes clean.
- `cargo test --lib` passes clean.
- `./run_sigma_tests.sh` passes clean.
- Every state transition is guarded by explicit validity checks and unit tests.
- Standalone unit tests pass via `rustc --test`.
