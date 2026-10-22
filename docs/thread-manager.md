# SigmaOS Thread Manager Architecture & Specification

## Executive Summary

The SigmaOS Thread Manager provides the canonical `Thread` abstraction and lifecycle engine (`src/kernel/thread.rs`). It governs thread states, ownership categories, saved CPU registers, kernel and user stack boundaries with guard zones, scheduling class attributes, CPU affinity, signal delivery masks, file-descriptor table references, resource limits, capabilities, and wait-channel synchronization.

---

## Thread Lifecycle Contract

### 1. States & Valid Transitions

Threads progress through seven canonical lifecycle states:

| State | Numerical Code | Description |
|---|---|---|
| `New` | `0` | Freshly allocated, stack prepared, awaiting placement in run queue |
| `Ready` | `1` | Runnable and waiting in run queue for CPU slice allocation |
| `Running` | `2` | Actively executing on CPU core |
| `Blocked` | `3` | Sleeping on wait channel or timer, awaiting event wakeup |
| `Suspended` | `4` | Execution paused via signal, debugger, or system action |
| `Zombie` | `5` | Exited execution; retaining exit status code until joined/reaped |
| `Terminated` | `6` | All resources reclaimed; entry eligible for thread manager purging |

```text
[New] -------> [Ready] <=======> [Running] -------> [Zombie] -------> [Terminated]
                 ^                    |                ^                  ^
                 |                    v                |                  |
                 +---- [Blocked] <----+                +---- (Detached) --+
                 |                    |
                 +--- [Suspended] <---+
```

#### Transition Invariants:
- `New` -> `Ready`, `Terminated`
- `Ready` -> `Running`, `Suspended`, `Zombie`
- `Running` -> `Ready`, `Blocked`, `Suspended`, `Zombie`
- `Blocked` -> `Ready`, `Suspended`, `Zombie`
- `Suspended` -> `Ready`, `Zombie`
- `Zombie` -> `Terminated`
- Any illegal state transition attempts fail immediately with an `Err(&str)`.

---

## Ownership Categories

1. **`Kernel`**: Pure kernel background thread with no user address space or POSIX context.
2. **`User`**: User-space thread bound to a process address space.
3. **`Process`**: Primary thread / leader of a process thread group.
4. **`Detached`**: Autonomous thread whose resources are automatically reclaimed on exit without requiring a parent join.

---

## Guarantees & Constraints

- **Preemption Model**: Fully preemptive in user mode; kernel preemption points enforced via safe preemption checks.
- **Priority Range**: Static priority values scaled 0–139 (120 default for normal scheduling).
- **Affinity**: Bitmask `CpuAffinity` supporting up to 64 logical CPU cores.
- **Stack Guards**: Mandatory `StackMeta` bounds checking and guard page protection against stack overflow.
- **Wait Channels**: Lost-wakeup prevention via single wait-channel state transitions and atomic wakeup queues.
- **Lock-Order Tracking**: Automatic recording of held lock addresses in `locks_held` for deadlock detection.

---

## Non-Goals & Scope Boundaries

- **Pthreads Conformance**: Full POSIX `pthread_*` binary runtime compatibility is provided by higher-level translation layers; kernel thread primitives enforce pure core safety invariants.
- **Direct Hardware Context Switching**: Hardware-level assembly register switching is isolated within CPU architecture crates (`src/arch/x86_64/`).

---

## Verification & Status

- **Status**: `Unit-tested`
- **Implementation**: `src/kernel/thread.rs`
- **Tests**: `rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread`
