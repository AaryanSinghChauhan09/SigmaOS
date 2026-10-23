# SigmaOS Thread Manager Specification & Contract

## Overview

The SigmaOS Thread Manager provides the canonical thread lifecycle, context management, synchronization primitives, and scheduling contracts for kernel and userland threads. It transforms isolated threading experiments into a deterministic, auditable subsystem with explicit lifecycle invariants.

---

## 1. Thread States & Lifecycle Contract

The thread lifecycle enforces strict state transitions. Illegal state jumps (such as `New` directly to `Running` or `Terminated` directly to `Ready`) are rejected at runtime.

### State Vocabulary

| State | Encoding | Description | Legal Next States |
|---|---|---|---|
| `New` | 0 | Newly created thread object; metadata and stacks initialized, not yet enqueued in run queue. | `Ready`, `Terminated` |
| `Ready` | 1 | Runnable and placed in CPU run queue waiting for scheduler dispatch. | `Running`, `Suspended`, `Zombie` |
| `Running` | 2 | Actively executing on CPU core. | `Ready`, `Blocked`, `Suspended`, `Zombie` |
| `Blocked` | 3 | Sleeping on wait channel, timer, or lock. | `Ready`, `Suspended`, `Zombie` |
| `Suspended` | 4 | Paused by signal, debugger, or control API. | `Ready`, `Zombie` |
| `Zombie` | 5 | Exited execution; retained in process table awaiting join/reaping. | `Terminated` |
| `Terminated` | 6 | Resources fully reclaimed; TID safe for recycling. | *Terminal state* |

---

## 2. Ownership Categories

SigmaOS categorizes thread ownership to dictate stack lifetime, address space association, and reaping mechanics:

1. **Kernel Threads (`ThreadOwnership::Kernel`):**
   - Pure kernel background workers executing within kernel space without a user address space.
   - Stack memory allocated from kernel heap; non-swappable.
2. **User Threads (`ThreadOwnership::User`):**
   - Bound to an owning process PID. Stack metadata tracks kernel stack and user space stack memory.
3. **Process Primary Threads (`ThreadOwnership::Process`):**
   - Leader thread of a process group. Termination triggers process group teardown unless reparented.
4. **Detached Threads (`ThreadOwnership::Detached`):**
   - Independent execution threads that automatically transition from `Zombie` to `Terminated` on exit without requiring an explicit `join()` call.

---

## 3. Subsystem Guarantees & Constraints

- **Preemption Model:** Deterministic preemption support with priority levels 0 (highest) through 139 (lowest). Default normal priority is 120; default nice value is 0 (-20 to +19).
- **CPU Affinity:** 64-bit affinity bitmask (`CpuAffinity`) restricting thread execution to designated CPU cores.
- **Stack Memory & Bounds Guards:** `StackMeta` maintains explicit base addresses, stack sizes, and guard zones to detect stack overflows before corrupting kernel structures.
- **Lock-Order Tracking:** Every lock acquisition (`acquire_lock`) and release (`release_lock`) is recorded in `locks_held` to detect lock inversion and potential deadlocks early.
- **Wait Channels & Wakeup Safety:** Wait channel IDs (`WaitChannelId`) wrap sleeping threads; `wake_wait_channel` atomically transitions blocked threads back to `Ready`.

---

## 4. Non-Goals & Compatibility Disclaimers

> **IMPORTANT:**
> SigmaOS explicitly distinguishes unit-tested data structures from complete, bootable operating system features.
> - **POSIX Compatibility:** Do not claim POSIX `pthread` compliance until the corresponding syscall surface, signal delivery matrix, and C library integration are fully tested and verified.
> - **Real-Time Guarantees:** Hard real-time scheduling ("SovereignSched") requires hardware evidence on target architectures before release claims.

---

## 5. Development Roadmap & First 10 PR Sequence

1. `docs(thread): define thread-manager contract and acceptance criteria`
2. `feat(thread): add canonical Thread object and lifecycle states`
3. `test(thread): add lifecycle, join, detach, and exit tests`
4. `feat(sched): add deterministic ready queue and scheduling-class hooks`
5. `feat(sched): add preemptive scheduling and context-switch test harness`
6. `fix(thread): add wait-channel wakeup and timeout race tests`
7. `feat(signal): route signals to the correct target thread`
8. `feat(proc): add process exit, orphan reparenting, and thread cleanup tests`
9. `ci: enforce fmt, clippy, unit tests, and standalone SigmaOS suites`
10. `docs(status): connect FEATURE_STATUS.toml to verified test evidence`

---

## 6. Verification Commands

```bash
cargo check --lib
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
./run_sigma_tests.sh
```
