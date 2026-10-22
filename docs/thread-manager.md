# SigmaOS Thread Manager Architectural Specification & Contract

## Overview

The SigmaOS Thread Manager provides the canonical execution entity abstraction (`Thread`) and lifecycle manager for SigmaOS. It defines strict state transitions, ownership classifications, context preservation, stack security bounds, wait-channel wakeup mechanics, and lock-order tracking.

---

## 1. Thread States & Lifecycle Model

SigmaOS threads move through seven explicit, deterministic states:

```
                  +--------------------------------+
                  |              New               |
                  +---------------+----------------+
                                  |
                                  v
                  +--------------------------------+
         +------->|             Ready              |<-------+
         |        +---------------+----------------+        |
         |                        |                         |
         |                        v                         |
         |        +--------------------------------+        |
         +--------|            Running             |--------+
                  +---------------+----------------+
                     |            |            |
                     v            v            v
            +------------+  +-----------+  +-----------+
            |  Blocked   |  | Suspended |  |  Zombie   |
            +------------+  +-----------+  +-----------+
                 |                |              |
                 +----------------+              v
                         |                 +-----------+
                         +---------------->|Terminated |
                                           +-----------+
```

### State Definitions

| State | Encodable Value | Description |
|---|---|---|
| `New` | `0` | Newly created thread structure, memory/stack allocated, not yet enqueued in scheduler. |
| `Ready` | `1` | Runnable state; queued in scheduler run-queue waiting for CPU execution time. |
| `Running` | `2` | Actively executing on a physical CPU core. |
| `Blocked` | `3` | Sleeping on a wait channel or timer, awaiting I/O or event notification. |
| `Suspended` | `4` | Execution paused via signal (`SIGSTOP`) or administrative control action. |
| `Zombie` | `5` | Exited execution, retains exit status awaiting join/reaping by parent process. |
| `Terminated` | `6` | Execution finished and all resources (stacks, TCB) completely reclaimed. |

### Valid State Transitions

The state machine enforces valid transitions via `ThreadState::is_valid_transition`:

- `New` -> `Ready`, `Terminated`
- `Ready` -> `Running`, `Suspended`, `Zombie`
- `Running` -> `Ready`, `Blocked`, `Suspended`, `Zombie`
- `Blocked` -> `Ready`, `Suspended`, `Zombie`
- `Suspended` -> `Ready`, `Zombie`
- `Zombie` -> `Terminated`

---

## 2. Ownership Categories

1. **`Kernel`**: Pure kernel background thread running entirely within kernel address space (`PID 0` or system context, no user space memory mappings).
2. **`User`**: Application thread bound to a specific user process (`PID > 0`), sharing address space and file descriptors with peer process threads.
3. **`Process`**: Primary thread / thread-group leader of a process group.
4. **`Detached`**: Independent thread whose resources are automatically reclaimed upon exit without requiring explicit `join()`.

---

## 3. Guarantees & Constraints

### Preemption Model
- Preemptive scheduling with quantum-driven timer interrupts.
- Voluntary yields via `sched_yield()` or blocking I/O transitions immediately to `Ready` or `Blocked`.

### Priority Ranges & Scheduling Classes
- **Priority Range**: `0` (highest) through `139` (lowest). Default normal priority is `120`.
- **Nice Values**: `-20` (highest user priority) through `19` (lowest).
- **Scheduling Classes**: `Normal` (CFS/EEVDF), `Fifo` (Real-Time SCHED_FIFO), `RoundRobin` (SCHED_RR), `Batch`, `Idle`, `Deadline`.

### CPU Affinity
- Machine CPU affinity represented via `CpuAffinity` u64 bitmask (supporting up to 64 CPU cores).
- Default affinity is `CpuAffinity::all()`.

### Stack Limits & Guard Zones
- Kernel and user stacks tracked via `StackMeta` containing `base_addr`, `stack_size`, and `guard_size`.
- `StackMeta::is_in_guard_zone()` identifies stack overflow / guard-page violations before memory corruption.

### Lock-Order Tracking
- Each thread tracks acquired lock addresses in `locks_held: Vec<u64>`.
- Attempts to acquire locks out of hierarchical address order emit lock-inversion warnings.

### Cancellation & Join Mechanics
- Non-detached threads must be joined via `join()` to harvest exit code and transition `Zombie` -> `Terminated`.
- Joining a detached thread returns an error (`Err("cannot join a detached thread")`).

---

## 4. Non-Goals & Boundaries

- **POSIX Compliance Scope**: Full `pthread` compliance claims are restricted to verified POSIX API tests. The kernel Thread object models baseline primitives upon which libc/pthread handlers build.
- **Hardware Multiprocessing Integration**: Multi-core APIC scheduling is tested in isolated unit/integration harnesses prior to bare-metal SMP validation.

---

## 5. Verification Commands

```bash
cargo check --lib
cargo test --lib
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
./run_sigma_tests.sh
```

---

## 6. Implementation Status

- **Status**: `Unit-tested`
- **Source Module**: `src/kernel/thread.rs`
- **Feature Matrix Entry**: `[kernel.thread_manager]` in `FEATURE_STATUS.toml`
