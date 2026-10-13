# SigmaOS Thread Manager Subsystem Specification

## Overview

The SigmaOS Thread Manager subsystem provides thread creation, context management, state transitions, wait-channel wakeups, stack protection, and lifecycle tracking.

This document serves as the formal subsystem contract and capability status reference for the thread manager in `src/kernel/thread.rs`.

---

## Subsystem Contract

### 1. Thread States

Every thread transitions through a deterministic finite-state machine:

- **`New`**: Thread object allocated and initialized; not yet placed on any ready queue.
- **`Ready`**: Thread is runnable and waiting for scheduler CPU dispatch.
- **`Running`**: Thread is currently executing on a CPU core.
- **`Blocked`**: Thread is waiting on a wait channel or synchronization primitive (e.g., mutex, semaphore, timer, or I/O).
- **`Suspended`**: Thread execution is paused administratively or by control signals.
- **`Zombie`**: Thread has exited or terminated; context destroyed, but exit code retained pending parent/join reaping.
- **`Terminated`**: Thread resources (including kernel/user stacks) are fully reclaimed.

### 2. Ownership Categories

- **Kernel Threads (`ThreadOwnership::Kernel`)**: Independent kernel-mode execution entities without userland memory space.
- **User Threads (`ThreadOwnership::User`)**: User-mode execution entities bound to a specific process address space.
- **Process Threads (`ThreadOwnership::Process`)**: Main process threads holding process group leadership.
- **Detached Threads (`ThreadOwnership::Detached`)**: Threads whose exit resources are auto-reclaimed upon exit without requiring explicit `join`.

### 3. Subsystem Guarantees

- **Preemption Model**: Supports preemptive scheduling; saved `CpuContext` guarantees safe context save/restore across interrupt and timer boundaries.
- **Priority & Nice Ranges**: Priority scale `0..=139` with nice values `-20..=19` mapping onto real-time and fair scheduling classes.
- **Affinity Semantics**: Per-CPU bitmasks (`cpu_affinity: u64`) enforcing CPU pinning and migration constraints.
- **Stack Protection**: Stack metadata includes base, size, and guard page boundaries (`stack_guard_size`). Stack overflow checks fail closed before state transition.
- **Cancellation & Join Safety**: Lock-free or lock-ordered state transitions prevent lost wakeups and double-reaping races on `join` or exit.

### 4. Non-Goals

- **POSIX `pthread` Conformance**: While compatible primitives exist, this module does not claim full POSIX `pthread` compliance until all syscall boundaries and libc shim behaviors pass conformance suites.
- **Hardware Multiprocessing Proof**: Unit-tested state logic does not constitute multi-socket SMP hardware validation under full QEMU or bare-metal execution.

---

## Unit Test Evidence

Standalone unit tests in `src/kernel/thread.rs` verify:

- `test_valid_state_transitions`: Legal transitions across `New` -> `Ready` -> `Running` -> `Blocked` -> `Zombie` -> `Terminated`.
- `test_invalid_state_transitions`: Illegal state jumps (e.g. `Terminated` -> `Running`) are rejected.
- `test_detach_and_join`: Proper join waiting and auto-reclaim for detached threads.
- `test_thread_manager_wait_channel_wake`: Concurrent wait-channel registration and loss-free wakeups.
- `test_stack_bounds_and_guards`: Stack overflow guard detection.
- `test_lock_order_tracker`: Lock dependency tracking and acquisition validation.
- `test_suspension_and_resume_lifecycle`: Admin suspend and resume logic.
- `test_reap_terminated_threads`: Batch cleanup of terminated thread objects.

Execution command:
```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```
