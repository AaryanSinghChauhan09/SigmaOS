# SigmaOS Thread Manager Architecture & Contract Specification

## Overview

The **SigmaOS Thread Manager** is a core kernel subsystem responsible for thread creation, lifecycle state machine tracking, CPU context frame setup, stack management, wait-channel blocking/wakeup, and join/detach reaping operations.

This document serves as the canonical contract for Phase 1 of the Thread Manager development plan.

---

## 1. Thread Lifecycle States

The Thread Manager strictly enforces an 7-stage deterministic thread state machine:

```text
       ┌───────────┐
       │    New    │
       └─────┬─────┘
             │ (enqueue / wake)
             ▼
  ┌─────────────────────┐
  │        Ready        │◄──────────────┐
  └──────────┬──────────┘               │
             │ (schedule dispatch)      │ (wakeup / unblock)
             ▼                          │
  ┌─────────────────────┐               │
  │       Running       ├───────────────┼───────────────┐
  └──────────┬──────────┴───────────────┘               │
             │                          ▲               │
             │ (wait / block)           │ (resume)      │
             ▼                          │               │
  ┌─────────────────────┐      ┌────────┴────────┐      │ (exit / terminate)
  │       Blocked       │      │    Suspended    │      │
  └─────────────────────┘      └─────────────────┘      │
             │                                          │
             └──────────────────────────────────────────┼────────┐
                                                        ▼        ▼
                                                ┌────────┐  ┌───────────┐
                                                │ Zombie │  │Terminated │
                                                └────────┘  └───────────┘
```

| State | Description | Invariants |
|---|---|---|
| `New` | Thread object allocated, CPU context initialized, stack guard pages configured. | Not yet in any CPU run queue. |
| `Ready` | Enqueued in scheduler run queue, eligible for CPU dispatch. | Holds valid saved `CpuContext`. |
| `Running` | Currently executing on an active CPU core. | Stack guard intact; lock order tracking active. |
| `Blocked` | Waiting on a wait channel (futex, timer, I/O event, or signal). | Associated with a specific wait channel ID. |
| `Suspended` | Explicitly paused by kernel debug, process trace, or job control (`SIGSTOP`). | Unreachable by standard scheduler dispatch until resumed. |
| `Zombie` | Execution completed (`exit`), awaiting join call or parent reaping. | Holds exit code; stack unmapped or ready for reclamation. |
| `Terminated` | Fully reaped and decoupled; resources marked for reclamation. | Terminal state; no further state transitions permitted. |

---

## 2. Ownership Model

Threads in SigmaOS are classified under four explicit ownership categories:

1. **Kernel Threads (`ThreadOwnership::Kernel`)**:
   - Execute strictly in Ring 0 with kernel stack privileges.
   - Do not possess a userland address space (CR3 points to kernel page directory).
2. **User Threads (`ThreadOwnership::User`)**:
   - Execute in Ring 3 with isolated user virtual address space.
   - Unbounded by parent process metadata.
3. **Process Threads (`ThreadOwnership::Process(Pid)`)**:
   - Associated with a parent process ID (`Pid`).
   - Share file descriptor table, address space, and resource limits with sibling process threads.
4. **Detached Threads (`ThreadOwnership::Detached`)**:
   - Automatically reaped upon termination without requiring an explicit `join()` call.

---

## 3. Subsystem Guarantees & Safety Invariants

### Preemption Model
- Voluntary yield (`sched_yield`) or timer-driven tick preemption (`PIT`/`APIC` timer interrupts).
- Preemption disabled during critical lock acquisitions or hard IRQ handlers.

### Priority Ranges
- Priority spectrum: `0` (highest real-time priority) to `139` (lowest nice priority).
- Real-Time (RT) class: `0..99` (`SCHED_FIFO` / `SCHED_RR`).
- Fair / Normal class: `100..139` (default nice level = `120`).

### Affinity Semantics
- Bitmask representing allowed CPU core IDs (`0..63`).
- Re-evaluated on every scheduler run queue selection.

### Stack Limits & Guard Pages
- Default kernel stack size: 16 KiB (4 pages).
- Lower boundary protected by an unmapped stack guard page (`STACK_GUARD_PATTERN = 0xDEADBEEF_CAFEBABEDCBA`).
- Underflow/overflow triggers immediate panic or task termination.

### Cancellation Behavior
- Supports deferred cancellation points (at syscall or wait-channel boundary).
- Asynchronous cancellation permitted only for kernel worker threads.

---

## 4. Explicit Non-Goals (Phase 1)

- **POSIX `pthread` Compliance**: Full POSIX `pthread_*` ABI compliance (e.g., `pthread_cancel`, `pthread_key_create`, `pthread_atfork`) is NOT claimed until libc-facing syscall wrappers and runtime tests are integrated.
- **Hardware Multiprocessing (SMP) Lock-Free Queues**: SMP lock-free queues and cross-core IPI preemption are modeled in unit tests, pending QEMU multi-core SMP boot validation.

---

## 5. Acceptance Criteria & Verification

- `cargo check --lib` builds cleanly without warnings.
- `cargo test --lib` passes core library tests.
- `./run_sigma_tests.sh` executes full workspace test suites without errors.
- Unit test suite in `src/kernel/thread.rs` verifies every state transition, stack guard check, wait-channel wakeup, and join/detach operation.
- Capability status updated in `FEATURE_STATUS.toml` under `[kernel.thread_manager]`.
