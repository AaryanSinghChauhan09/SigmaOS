# SigmaOS Thread Manager Architecture & Subsystem Contract

The SigmaOS Thread Manager is a formal kernel subsystem responsible for thread lifecycle management, state transitions, CPU context tracking, stack guard protection, and wait-channel synchronization.

---

## Thread Lifecycle States

The canonical `ThreadState` state machine consists of 7 explicit states:

| State | Description |
|---|---|
| `New` | Thread object instantiated, stack and CPU context allocated, not yet enqueueable |
| `Ready` | Enqueued in CPU run queue, eligible for scheduler selection |
| `Running` | Currently executing on a CPU core |
| `Blocked` | Waiting on an explicit wait channel, event, timer, or lock wakeup |
| `Suspended` | Explicitly paused by user or administrative action |
| `Zombie` | Execution completed, stack reclaimed, awaiting parent process join/reaping |
| `Terminated` | Fully joined, reaped, and ready for memory deallocation |

---

## Ownership Categories

Threads in SigmaOS belong to one of four ownership models:

1. **Kernel Threads (`ThreadOwner::Kernel`):** Background kernel tasks executing in kernel space without a user context or user FD table.
2. **User Threads (`ThreadOwner::User`):** Ring-3 user threads executing within a user process boundary.
3. **Process Threads (`ThreadOwner::Process(ProcessId)`):** Threads explicitly tied to an owning `ProcessId` and sharing its virtual memory map and FD table.
4. **Detached Threads (`ThreadOwner::Detached`):** Autonomous threads whose resources are automatically reclaimed upon termination without requiring `join()`.

---

## Architectural Guarantees

- **Preemption Model:** Deterministic timer-driven preemption with priority-slice calculation and voluntary yield (`sched_yield`).
- **Priority Ranges:** 128 priority levels (0 = lowest background priority, 127 = highest real-time priority; default nice = 0 mapped to priority 64).
- **Affinity Semantics:** Per-thread CPU affinity mask (`cpu_affinity: u64`) constraining run-queue placement to specified logical cores.
- **Stack Limits & Guard Pages:** Minimum 16 KiB kernel stack allocation featuring non-executable stack guard regions and top/bottom boundary checks.
- **Cancellation Behavior:** Synchronous and asynchronous thread cancellation with state verification.

---

## Non-Goals & Compatibility Boundaries

- **No Unvalidated POSIX Claims:** SigmaOS does not claim POSIX `pthread` compliance until the corresponding syscall dispatch (`sys_clone`, `sys_futex`, `sys_pthread_*`) and libc ABI wrappers are fully integrated and verified with conformance tests.
- **Unit-Tested Isolation:** Unit-tested component status establishes module-level state transition and wait-channel logic, not proof of a bootable kernel session.

---

## Subsystem Acceptance Criteria & Verification

Every PR modifying the Thread Manager must satisfy:

1. `cargo check --lib` passes clean without errors.
2. `cargo test --lib` passes cleanly.
3. `./run_sigma_tests.sh` passes completely.
4. Every `ThreadState` transition has an explicit unit test in `src/kernel/thread.rs`.
5. Standalone verification passes via `rustc --test src/kernel/thread.rs`.
