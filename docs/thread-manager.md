# SigmaOS Thread Manager Subsystem Contract

## Overview

The SigmaOS Thread Manager provides a unified, canonical `Thread` abstraction and lifecycle management interface across kernel and user execution context. It serves as the single source of truth for thread execution states, CPU context, priority, affinity, wait channel synchronization, and resource ownership.

## Thread Lifecycle States

SigmaOS thread execution is governed by an explicit finite state machine with 7 canonical states:

- **`New`**: Newly created thread object; metadata and CPU context initialized, but not yet enqueued in CPU run queues.
- **`Ready`**: Runnable state; thread is enqueued in a scheduler run queue waiting for CPU dispatch.
- **`Running`**: Actively executing on a CPU core.
- **`Blocked`**: Sleeping on a wait channel (`WaitChannelId`), timer, or I/O boundary.
- **`Suspended`**: Paused by explicit control signal or debug action; ineligible for scheduler dispatch.
- **`Zombie`**: Execution terminated; exit code recorded, awaiting parent/reaper process join or cleanup.
- **`Terminated`**: Final state; kernel and stack resources fully reclaimed.

### State Transitions

Legal state transitions are strictly validated by `ThreadState::is_valid_transition`:
- `New` -> `Ready`, `Terminated`
- `Ready` -> `Running`, `Suspended`, `Terminated`
- `Running` -> `Ready`, `Blocked`, `Suspended`, `Zombie`, `Terminated`
- `Blocked` -> `Ready`, `Suspended`, `Terminated`
- `Suspended` -> `Ready`, `Blocked`, `Terminated`
- `Zombie` -> `Terminated`
- `Terminated` -> None (terminal sink state)

## Ownership Categories

Every thread in SigmaOS belongs to one of four canonical ownership categories:

1. **Kernel Threads (`ThreadOwnership::Kernel`)**: Supervisor-mode threads executing internal kernel tasks (e.g. page reclaim, driver polling). No associated user address space.
2. **User Threads (`ThreadOwnership::User`)**: Executing in user mode within a distinct process address space.
3. **Process Threads (`ThreadOwnership::Process(Pid)`)**: Bound to a specific owning process identifier (`Pid`).
4. **Detached Threads (`ThreadOwnership::Detached`)**: Threads configured to auto-reclaim resources immediately upon exit without requiring explicit `join`.

## Core Guarantees & Invariants

1. **Stack Limits & Guard Zones**: Every thread specifies stack base address and total size (`StackMeta`). Stack overflow detection is enforced via explicit guard checks (`is_sp_in_guard_zone`) and boundary checks (`is_sp_valid`).
2. **Deterministic State Transitions**: Invalid state transitions return `Err(ThreadError::InvalidStateTransition)` and preserve previous state.
3. **Wait Channel Wakeup Safety**: Blocking on a wait channel (`WaitChannelId`) registers the thread atomically. Wakeup (`wake_wait_channel`) and timeout processing update thread state safely to prevent lost wakeups or race conditions.
4. **Priority & Affinity Semantics**: Priority range is bounded (0-255, default 128). `CpuAffinity` provides bitmask-based CPU pinning up to 64 cores.
5. **Lock-Order Tracking**: When lock-order diagnostics are enabled (`LockOrderTracker`), lock acquisitions are validated against hierarchical levels to prevent deadlocks.

## Non-Goals & Limitations

- **POSIX Compliance Scope**: The thread manager contract does NOT claim POSIX `pthread` compliance until all syscall and libc-facing behaviors (e.g. robust mutexes, signal delivery, cancellation points) are fully integrated and verified in userspace.
- **Hardware Integration**: Pure unit-tested logic; hardware CPU register context switching and trap handler return paths are validated in kernel architecture modules.

## Current Implementation Status

- **Status**: `unit-tested`
- **Primary Source File**: `src/kernel/thread.rs`
- **Verification Evidence**: 10 unit tests pass via `rustc --test src/kernel/thread.rs --edition=2021`.
- **Next Milestone**: Integrate canonical `Thread` objects into CPU run queue and scheduler dispatch path (`feat(sched): preemptive scheduler core`).
