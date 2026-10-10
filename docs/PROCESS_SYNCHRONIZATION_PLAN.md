# SigmaOS Process Synchronization Master Architecture & PR Proposal Plan

## Executive Overview

SigmaOS contains synchronization-related primitives across kernel, IPC, and userland compatibility modules (`src/kernel/wait_queue.rs`, `src/kernel/sigma_spinlock.rs`, `src/kernel/semaphore.rs`, `src/kernel/flock.rs`, `src/kernel/futex.rs`, `src/kernel/lockdep.rs`, `src/kernel/pipe.rs`, `src/kernel/msgqueue.rs`, `src/kernel/shm.rs`, `src/kernel/eventfd.rs`, `src/ipc/futex.rs`, `src/process/blocked_state.rs`). However, code presence and unit test execution do not constitute a fully integrated, runtime-verified process synchronization subsystem.

This document establishes the official **SigmaOS Process Synchronization Architecture & Development Decision Plan**. It defines a coherent progression from low-level kernel wait/wakeup primitives to process-shared synchronization, UAPI futex syscall interfaces, and runtime concurrency diagnostics, providing explicit rules for locking contexts, memory ordering, ownership semantics, and testing gates.

---

## Architecture & Progression Pathway

Process synchronization in SigmaOS follows a strict 5-tier layered progression:

```
+-----------------------------------------------------------------------+
| 5. Concurrency Diagnostics & Lockdep                                  |
|    (Lock ordering graph, held-lock assertions, race detection)        |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| 4. Process-Shared IPC Synchronization                                 |
|    (Shared memory futexes, POSIX semaphores, pipes, eventfd, mqueue)  |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| 3. Userspace Synchronization APIs (UAPI & Libc Shim)                  |
|    (Futex wait/wake syscalls, robust user mutexes, condvars)          |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| 2. Kernel Locks & Completions                                         |
|    (Sleeping mutex, ticket spinlock, RW lock, completions, semaphores)|
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| 1. Kernel Wait/Wakeup Primitives                                      |
|    (Wait queues, task state transition, scheduler sleep/wake)          |
+-----------------------------------------------------------------------+
```

---

## Core Design Rules

1. **Name the Synchronization Domain:**
   Each primitive must explicitly declare whether it synchronizes:
   - Kernel threads within kernel space.
   - User threads within a single process.
   - Independent processes sharing virtual memory mappings.
   - Asynchronous operations across an IPC channel.

2. **Separate Lock from Event:**
   - Use **mutexes** strictly for mutual exclusion with owner tracking.
   - Use **condition variables, completions, and wait queues** for waiting on state changes or external predicates.
   - Do not substitute semaphores as general-purpose ownership locks.

3. **Never Sleep in Atomic Context:**
   - Prohibit sleeping or blocking locks in hard-interrupt, non-preemptible, or RCU critical sections.
   - Distinguish sleeping locks (mutexes, semaphores, condvars) from spinning locks (spinlocks, RW-spinlocks).
   - Keep IRQ-safe spinlocks short, bounded, and allocation-free.

4. **Specify Ownership and Failure Semantics:**
   - Define recursive, error-checking, and robust behavior explicitly.
   - Enforce owner-only unlock rules for mutexes.
   - Specify error returns on invalid object destruction while waiters exist or when an owner dies.

5. **Wait on Predicates, Not Notifications:**
   - Callers of condition variables or wait queues must check shared state in a loop (`while !predicate()`).
   - Permit spurious wakeups.
   - Ensure wait operations atomically release the associated lock and block, then reacquire the lock prior to return.

6. **Prevent Lost Wakeups:**
   - Registration of a waiter in a wait queue must occur before releasing the predicate lock or entering the sleep state.
   - State re-evaluation and sleep transitions must be atomic relative to incoming notifications, timeouts, signals, and task cancellations.

7. **Use Explicit Memory Ordering:**
   - Document the happens-before invariant for every atomic operation.
   - Use `Acquire`/`Release` or `SeqCst` when publishing state or transferring ownership. Avoid unexplained `Relaxed` atomics in synchronization fast paths.

8. **Address Priority Inversion and Starvation:**
   - Define clear fairness policies for mutexes, semaphores, and read-write locks.
   - Evaluate priority inheritance (PI) for mutexes after scheduler priority tracking is integrated. Counting semaphores do not track an owner and cannot support owner-based PI.

9. **Treat User-Memory Objects as Untrusted:**
   - Validate shared-memory virtual addresses, object handles, ownership, and process lifetimes at every syscall boundary.
   - Prevent user processes from passing raw kernel pointers or corrupting kernel waiter lists.

10. **Test Races Systematically:**
    - Combine deterministic unit state-machine tests, Loom concurrency model checking, stress benchmarks, and booted QEMU multi-vCPU validation.

---

## Proposed 17-PR Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
| :--- | :--- | :--- | :--- |
| **PR 1** | `docs(sync): inventory primitives and define subsystem boundaries` | Audit scheduler wait queues, atomics, spinlocks, semaphores, flock, pipes, message queues, shared memory, signals, event notifications, syscalls, and libc wrappers. | Inventory distinguishes kernel-internal, process-local, process-shared, and modeled code; no module is described as runtime-supported without evidence. |
| **PR 2** | `docs(sync): define synchronization contracts and lock-context rules` | Publish primitive selection guide, lock hierarchy, blocking-context matrix, fairness expectations, memory-order rules, and lifecycle/destruction contract. | Every primitive has documented owner, valid contexts, wake policy, timeout/cancellation behavior, destruction behavior, and test obligations. |
| **PR 3** | `test(sync): add concurrency model and invariant harness` | Add host-side deterministic tests and optional Loom models for atomic fast paths, waiter registration, wakeup, timeout, cancellation, and object destruction. | Models use Loom-aware atomics/locks; bounds are explicit; tests cover lost wakeup and ABA/reuse cases; model tests are not presented as kernel runtime evidence. |
| **PR 4** | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Implement the base wait queue used by sleeping synchronization primitives; connect task states, scheduler enqueue/dequeue, timeout, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; notification cannot be lost between predicate check and sleep; task exits remove wait-queue references safely. |
| **PR 5** | `feat(sync): add kernel mutex with ownership and debug checks` | Implement a sleeping mutex for task context; define ownership, try-lock, recursion policy, and error behavior. | Contended lock sleeps instead of spinning; non-owner unlock fails in debug/test mode; interrupt/atomic context use is rejected; owner and waiter cleanup survive task exit. |
| **PR 6** | `feat(sync): add bounded spinlock and IRQ-safe variants` | Add minimal spinlock/IRQ-save primitives for tiny critical sections; document per-CPU and SMP semantics. | Lock/unlock barriers are tested; debug mode detects recursive acquisition and invalid context; no allocation, blocking, or unbounded work occurs while held. |
| **PR 7** | `feat(sync): add completion and one-shot event primitive` | Add a completion/event for “wait until operation reaches state” use cases; integrate with wait queues and timeout. | Completion before wait is retained; multiple waiters follow documented semantics; repeated complete/reinit behavior is specified; tests cover race and teardown cases. |
| **PR 8** | `feat(sync): add counting semaphore with bounded counter` | Implement wait/post/try-wait/timed-wait with overflow, underflow, and waiter semantics. | Counter never underflows or exceeds configured maximum; waking a waiter transfers availability correctly; interrupt-context post rules are explicit; process sharing is not claimed yet. |
| **PR 9** | `feat(sync): add condition variable with atomic unlock-and-wait` | Implement condition wait/signal/broadcast tied to a mutex; add monotonic timed wait if clock support exists. | Mutex is released atomically with waiter registration; it is reacquired before return; callers are required to re-check predicates; spurious wakeups and timeout races are tested. |
| **PR 10** | `feat(sync): add read-write lock with starvation policy` | Add read/write lock only after mutex/wait queues work; specify reader/writer preference or fairness and recursion policy. | Tests cover concurrent readers, exclusive writer, downgrade/upgrade policy, writer starvation bounds, and cancellation/exit cleanup. |
| **PR 11** | `feat(sync): add robust process-local mutex recovery` | Add owner-death handling for user mutexes where thread lifecycle and shared synchronization objects support it. | A successor receives an owner-dead/inconsistent indication; recovery/consistent/unrecoverable semantics are documented and tested against POSIX expectations. |
| **PR 12** | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Add a word-based wait/wake ABI only after syscall dispatch, safe user-memory access, scheduler wait queues, and process identity are integrated. Use compare-and-block semantics to avoid the check-to-sleep race. | Wait only blocks when the user value still matches; wake targets the correct waiters; invalid pointers and key reuse are safe; ABI is documented and architecture-tested. |
| **PR 13** | `feat(sync): integrate process-shared synchronization with shared memory` | Connect semaphore/mutex objects to shared-memory mappings and object lifetimes; define handle creation, permissions, unlink, and owner cleanup. | Independent processes synchronize through a real shared mapping; permissions and namespace boundaries are enforced; object destruction cannot free state while waiters reference it. |
| **PR 14** | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and event notifications with the same wait/wakeup and cancellation contracts. | Read/write readiness, queue full/empty, close, peer exit, timeout, and signal interactions are tested; no busy-wait loop is required for blocking operations. |
| **PR 15** | `debug(sync): add lock-order checker and wait diagnostics` | Add debug-only lock classes, lock-order graph, owner/waiter diagnostics, held-lock assertions, and deadlock reports. | Debug mode catches inverse lock ordering, recursive misuse, sleeping lock in interrupt context, and wait-while-held violations; diagnostics are bounded and do not expose sensitive data. |
| **PR 16** | `test(sync): add race detector and stress configuration` | Add debug builds or instrumentation for data races where practical; add lock torture, producer/consumer, timeout, and high-contention tests. | Tests run on single-core and SMP QEMU when available; race reports include task/CPU/access context; limitations and false-negative risks are stated. |
| **PR 17** | `docs(sync): publish API, context matrix, and compatibility status` | Document kernel APIs, UAPI/libc APIs, errno/status, object lifecycle, process sharing, architecture support, and limitations. | Every API points to implementation and tests; Linux/BSD/POSIX references are clearly differentiated; capability matrix matches runtime evidence. |

---

## Priority Order & Prerequisite Gates

1. **Phase 1 (PRs 1–8): Core Primitives & Scheduler Integration**
   - Complete PRs 1–8 first.
   - **Hard Gate:** Implementation of blocking mutex, condition variable, and semaphore behavior MUST be gated on the scheduler's real task wait/wakeup integration (`src/scheduler/eevdf.rs`, `src/kernel/wait_queue.rs`).

2. **Phase 2 (PRs 9–11): Complex Locks & Robust Recovery**
   - Add condition variables, read/write locks, and robust process-local mutex owner-death recovery.

3. **Phase 3 (PRs 12–14): Userspace & Shared IPC Synchronization**
   - Integrate futex wait/wake syscalls, shared-memory synchronization mappings, and interprocess message queues.
   - **Prerequisites:** System call dispatch (`src/syscall/abi.rs`), Ring 3 execution (`src/arch/x86_64/tss_ring3_user_mode.rs`), process spawning, and virtual memory page validation are mandatory prerequisites before claiming userspace process-shared support.

4. **Phase 4 (PRs 15–17): Diagnostics, Torture Stress, & Documentation Certification**
   - Implement lockdep graph tracking, sampling race detection, SMP torture benchmarks, and publish final support claims.

---

## Subsystem Inventory Audit Matrix

| Module File | Domain | Runtime Status | Current Evidence | Ownership & Dependencies |
| :--- | :--- | :--- | :--- | :--- |
| `src/kernel/wait_queue.rs` | Kernel Threads | Prototype | Unit tests for entries, flags, wake_one/all, completion | Requires EEVDF scheduler task enqueue/dequeue |
| `src/kernel/sigma_spinlock.rs` | Kernel Threads (SMP) | Working | Unit tests for ticket allocation and RW spinlock | CPU spin loop, no heap allocation |
| `src/kernel/semaphore.rs` | Kernel / Thread | Prototype | Unit tests for counter wait/post | Depends on `WaitQueue` |
| `src/kernel/futex.rs` | Thread / Process | Prototype | Unit tests for hash key lookup and queue management | Requires page table validation & syscall dispatch |
| `src/ipc/futex.rs` | Process-Shared | Prototype | Unit tests for PI state and waiter hash table | Requires shared memory mapping lifecycle |
| `src/kernel/flock.rs` | Process File Lock | Prototype | Unit tests for advisory lock conflicts | Requires VFS inode fd table integration |
| `src/kernel/pipe.rs` | IPC / Process | Prototype | Unit tests for ring buffer read/write | Requires VFS file descriptor binding |
| `src/kernel/msgqueue.rs` | IPC / Process | Prototype | Unit tests for message queue prioritization | Depends on `WaitQueue` and POSIX UAPI |
| `src/kernel/shm.rs` | Process-Shared | Prototype | Unit tests for segment creation and permissions | Requires page table mapping & MMU |
| `src/kernel/eventfd.rs` | IPC / Process | Prototype | Unit tests for counter signal/wait | Requires epoll / kqueue event loop integration |
| `src/kernel/lockdep.rs` | Diagnostic | Prototype | Unit tests for dependency graph tracking | Debug mode annotation |
| `src/process/blocked_state.rs` | Process State | Prototype | Unit tests for state transitions | Connects to scheduler process list |

---

## Component-Specific Rules

- **Scheduler/Wait Queues:** Register the waiter in the queue before releasing the predicate lock. Make sleep and wake transitions atomic from the caller's perspective. Cleanly remove waiters on timeout, signal delivery, or task exit.
- **Mutexes:** Enforce owner task tracking. Enforce owner-only unlock rules and reject non-owner releases in debug mode. Reject acquisition or sleeping in interrupt/atomic context.
- **Condition Variables:** Always pair with a mutex and shared predicate. Callers must wait in a `while !predicate()` loop to handle spurious wakeups. Release lock atomically with waiter registration; reacquire lock before return.
- **Semaphores:** Bounded counter limits. Do not claim process-shared semantics until shared memory mappings and object handle lifecycles are integrated.
- **Spinlocks and IRQ Locks:** Use exclusively for tiny bounded critical sections. Prohibit heap allocation, blocking calls, or scheduler yields while held.
- **Read/Write Locks:** Document writer starvation policies and upgrade/downgrade rules.
- **Atomics:** Use explicit `Acquire`/`Release` ordering with documented happens-before arguments.
- **Shared Memory / Futex:** Tie wait keys to stable shared object identity (physical page / inode offset), not raw user virtual addresses alone.
- **IPC Queues and eventfd:** Standardize readiness flags, queue saturation, timeout, and cancellation behavior.

---

## PR Review Template & Merge Gates

```markdown
## Summary
What synchronization behavior or correctness gap does this PR address?

## Primitive and domain
Mutex | spinlock | semaphore | condition | completion | wait queue | IPC/futex
Kernel-only | thread-shared | process-shared

## Contract
Ownership, blocking context, memory ordering, fairness, timeout/cancellation,
destruction, error behavior, and supported architecture.

## Concurrency hazards
Lost wakeup, lock ordering, priority inversion, reentrancy, object reuse,
task/process exit, and interrupt interaction.

## Runtime evidence
Unit/model | Loom | QEMU single CPU | QEMU SMP | physical hardware
List target, workload, and what the test proves.

## Tests
Normal, contention, timeout, owner death, invalid context, cleanup,
race/deadlock diagnostics, and known untested interleavings.

## Status and docs
Proposed | Prototype | Integrated | Supported
Link API docs, compatibility notes, feature inventory, and test evidence.

## Checklist
- [ ] Lock/context rules documented and enforced
- [ ] Wait registration cannot race with wakeup
- [ ] Memory orderings have a written rationale
- [ ] Failure/timeout/exit paths release waiters and references
- [ ] Tests cover race and lifecycle cases
- [ ] ABI/layout changes are reviewed and documented
- [ ] Required CI passes; review discussions resolved
```

### Review Approval & Completion Gates

- **Approval Requirement:** Isolated documentation or model tests require 1 maintainer approval. Core atomic algorithms, scheduler wait queues, IRQ-safe locks, process-shared objects, or syscall UAPI changes require 2 qualified maintainer approvals.
- **Prototype Gate:** Data structures or lock models pass unit tests, but are not connected to real tasks.
- **Integrated Gate:** Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
- **Supported Gate:** Repeatable tests cover lifecycle, ownership, interrupt context, process sharing where claimed, starvation/priority behavior, architecture-specific memory ordering, and documented error semantics.

---

## Periodic Maintenance PR Cadence

- **Every PR:** `ci(sync)` - Run primitive-specific tests, invariant checks, and Loom models for changed code.
- **Weekly:** `maint(sync)` - Triage deadlocks, stalls, and race reports with task traces.
- **Weekly:** `test(sync)` - Review wakeup and timeout regressions under fixed workloads.
- **Monthly:** `audit(sync)` - Review lock ordering graph, interrupt/preemption contexts, and held-lock durations.
- **Monthly:** `test(sync)` - Run mutex/semaphore/condition stress matrix with contention and owner exit.
- **Monthly:** `security(sync)` - Review shared-object lifecycle, user pointer validation, permissions, and resource limits.
- **Quarterly:** `maint(sync)` - Audit atomic memory orderings (`Acquire`/`Release`/`SeqCst`) and weak-ordering architecture assumptions.
- **Quarterly:** `test(sync)` - Run multi-vCPU SMP stress tests on QEMU.
- **Each Release Candidate:** `release(sync)` - Certify primitive/API support matrix against runtime evidence.
- **After Each Incident:** `fix(sync)` - Implement minimal reproducer, root-cause analysis, and regression test.
