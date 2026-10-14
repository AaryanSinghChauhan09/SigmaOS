# SigmaOS Process Synchronization Master Architecture & Development Roadmap Plan

## 1. Executive Summary & Inventory Audit

SigmaOS contains various synchronization-related files and modules across the codebase (e.g. `src/kernel/wait_queue.rs`, `src/kernel/sigma_spinlock.rs`, `src/kernel/semaphore.rs`, `src/kernel/futex.rs`, `src/ipc/futex.rs`, `src/kernel/flock.rs`, `src/kernel/pipe.rs`, `src/kernel/eventfd.rs`, and `src/kernel/lockdep.rs`). However, source file presence and standalone unit test passing do not prove runtime end-to-end process synchronization integration across the scheduler, syscall layer, and Ring 3 user execution.

This document establishes the official SigmaOS Process Synchronization Master Architecture and PR Development Roadmap Plan. It defines a coherent progression for uniting existing prototypes and building robust process-shared synchronization:

1. **Kernel Wait/Wakeup Primitives** (`WaitQueue`, scheduler task state integration)
2. **Kernel Locks & Completions** (Sleeping mutexes, bounded spinlocks, completion events, counting semaphores)
3. **Userspace Synchronization APIs** (Condition variables, POSIX mutexes, readers-writer locks, robust owner death handling)
4. **Process-Shared IPC Synchronization** (`futex` compare-and-block syscalls, shared-memory semaphores/mutexes, pipe/eventfd readiness)
5. **Concurrency Diagnostics** (`lockdep` dependency graph validation, race detection, lock order checks)

---

## 2. Core Design Rules & Principles

Every synchronization primitive and algorithm in SigmaOS must adhere strictly to the following 11 core design rules:

1. **Name the Synchronization Domain**: Every primitive must state explicitly whether it synchronizes kernel threads, threads within a single process, processes sharing memory regions, or operations across an IPC channel.
2. **Separate Lock from Event**: Use mutexes exclusively for ownership-based mutual exclusion. Use condition variables, completions, and wait queues for waiting on state changes. Semaphores must not be substituted for ownership locks.
3. **Never Sleep in Atomic Context**: Define which primitives may block and prohibit sleeping locks in hard-interrupt, non-preemptible, or atomic contexts. Keep IRQ-safe spinlocks short and strictly bounded. Spin locks must never yield or block.
4. **Specify Ownership and Failure Semantics**: Define recursive, error-checking, and robust behaviors, owner-only unlock rules, destruction rules while waiters exist, and error returns (`EPERM`, `EDEADLK`, `ETIMEDOUT`).
5. **Wait on Predicates, Not Notifications**: Condition variable users must loop and re-check their shared predicate after wakeup (`while !predicate { wait() }`); spurious wakeups are explicitly permitted. The wait operation must atomically release the mutex and block, then reacquire it before returning.
6. **Prevent Lost Wakeups**: State checks and waiter registrations must be atomic with respect to notifications, timeouts, signals, and cancellations. Fast paths must strictly enforce memory-ordering requirements.
7. **Use Explicit Memory Ordering**: Every atomic variable must have a documented invariant and ordering rationale. Relaxed ordering must not be used where publication or ownership relationships require acquire/release synchronization.
8. **Address Priority Inversion and Starvation**: Define fairness policies for mutexes and read/write locks. Priority inheritance is introduced only after scheduler priority changes and owner tracking are integrated. Counting semaphores do not identify an owner and thus do not support owner-based priority inheritance.
9. **Treat User-Memory Objects as Untrusted**: Validate shared-memory mappings, object handles, user addresses, ownership, and lifetimes at every syscall boundary. A user process must never choose kernel pointers or manipulate kernel waiter queues directly.
10. **Test Races Systematically**: Combine deterministic state-machine tests, Rust model checking (e.g. Loom models where feasible), stress tests, and booted QEMU tests. Runtime lock-order validation (`lockdep`) and race detection complement automated testing.
11. **Decouple Kernel Internal Primitives from Userspace ABI**: Maintain a clean boundary between kernel-internal synchronization mechanisms and the exposed userspace UAPI/POSIX syscall interfaces.

---

## 3. Target Progression & Architectural Layers

SigmaOS structures process synchronization across five integrated layers:

```
+-----------------------------------------------------------------------+
| Layer 5: Concurrency Diagnostics & Instrumentation                    |
| (Lockdep dependency graph, held-lock assertions, race detectors)       |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| Layer 4: Process-Shared IPC & Futex Synchronization                    |
| (futex_wait/wake, shared-memory semaphores, pipe/eventfd readiness)  |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| Layer 3: Userspace Synchronization APIs                               |
| (POSIX Mutexes, Condition Variables, RW Locks, Robust Owner Death)    |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| Layer 2: Kernel Locks & Completions                                   |
| (Sleeping Kernel Mutex, Bounded Spinlock, Completion, Semaphore)      |
+-----------------------------------------------------------------------+
                                   ^
                                   |
+-----------------------------------------------------------------------+
| Layer 1: Kernel Wait/Wakeup Base Primitives                            |
| (WaitQueue, Scheduler task states, timer timeouts, signal cancel)     |
+-----------------------------------------------------------------------+
```

---

## 4. 17-PR Implementation Roadmap Sequence

| PR | Title | Scope | Acceptance Criteria |
|:---|:---|:---|:---|
| **1** | `docs(sync): inventory primitives and define subsystem boundaries` | Audit scheduler wait queues, atomics, spinlocks, semaphores, flock, pipes, message queues, shared memory, signals, event notifications, syscalls, and libc wrappers. | Inventory distinguishes kernel-internal, process-local, process-shared, and modeled code; no module is described as runtime-supported without evidence. |
| **2** | `docs(sync): define synchronization contracts and lock-context rules` | Publish primitive selection guide, lock hierarchy, blocking-context matrix, fairness expectations, memory-order rules, and lifecycle/destruction contract. | Every primitive has documented owner, valid contexts, wake policy, timeout/cancellation behavior, destruction behavior, and test obligations. |
| **3** | `test(sync): add concurrency model and invariant harness` | Add host-side deterministic tests and optional Loom models for atomic fast paths, waiter registration, wakeup, timeout, cancellation, and object destruction. | Models use Loom-aware atomics/locks; bounds are explicit; tests cover lost wakeup and ABA/reuse cases; model tests are not presented as kernel runtime evidence. |
| **4** | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Implement the base wait queue used by sleeping synchronization primitives; connect task states, scheduler enqueue/dequeue, timeout, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; notification cannot be lost between predicate check and sleep; task exits remove wait-queue references safely. |
| **5** | `feat(sync): add kernel mutex with ownership and debug checks` | Implement a sleeping mutex for task context; define ownership, try-lock, recursion policy, and error behavior. | Contended lock sleeps instead of spinning; non-owner unlock fails in debug/test mode; interrupt/atomic context use is rejected; owner and waiter cleanup survive task exit. |
| **6** | `feat(sync): add bounded spinlock and IRQ-safe variants` | Add minimal spinlock/IRQ-save primitives for tiny critical sections; document per-CPU and SMP semantics. | Lock/unlock barriers are tested; debug mode detects recursive acquisition and invalid context; no allocation, blocking, or unbounded work occurs while held. |
| **7** | `feat(sync): add completion and one-shot event primitive` | Add a completion/event for “wait until operation reaches state” use cases; integrate with wait queues and timeout. | Completion before wait is retained; multiple waiters follow documented semantics; repeated complete/reinit behavior is specified; tests cover race and teardown cases. |
| **8** | `feat(sync): add counting semaphore with bounded counter` | Implement wait/post/try-wait/timed-wait with overflow, underflow, and waiter semantics. | Counter never underflows or exceeds configured maximum; waking a waiter transfers availability correctly; interrupt-context post rules are explicit; process sharing is not claimed yet. |
| **9** | `feat(sync): add condition variable with atomic unlock-and-wait` | Implement condition wait/signal/broadcast tied to a mutex; add monotonic timed wait if clock support exists. | Mutex is released atomically with waiter registration; it is reacquired before return; callers are required to re-check predicates; spurious wakeups and timeout races are tested. |
| **10** | `feat(sync): add read-write lock with starvation policy` | Add read/write lock only after mutex/wait queues work; specify reader/writer preference or fairness and recursion policy. | Tests cover concurrent readers, exclusive writer, downgrade/upgrade policy, writer starvation bounds, and cancellation/exit cleanup. |
| **11** | `feat(sync): add robust process-local mutex recovery` | Add owner-death handling for user mutexes where thread lifecycle and shared synchronization objects support it. | A successor receives an owner-dead/inconsistent indication; recovery/consistent/unrecoverable semantics are documented and tested against POSIX expectations. |
| **12** | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Add a word-based wait/wake ABI only after syscall dispatch, safe user-memory access, scheduler wait queues, and process identity are integrated. | Wait only blocks when user value matches; wake targets correct waiters; invalid pointers and key reuse are safe; ABI is documented and architecture-tested. |
| **13** | `feat(sync): integrate process-shared synchronization with shared memory` | Connect semaphore/mutex objects to shared-memory mappings and object lifetimes; define handle creation, permissions, unlink, and owner cleanup. | Independent processes synchronize through a real shared mapping; permissions and namespace boundaries are enforced; object destruction cannot free state while waiters reference it. |
| **14** | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and event notifications with the same wait/wakeup and cancellation contracts. | Read/write readiness, queue full/empty, close, peer exit, timeout, and signal interactions are tested; no busy-wait loop required for blocking operations. |
| **15** | `debug(sync): add lock-order checker and wait diagnostics` | Add debug-only lock classes, lock-order graph, owner/waiter diagnostics, held-lock assertions, and deadlock reports. | Debug mode catches inverse lock ordering, recursive misuse, sleeping lock in interrupt context, and wait-while-held violations; diagnostics are bounded. |
| **16** | `test(sync): add race detector and stress configuration` | Add debug builds or instrumentation for data races where practical; add lock torture, producer/consumer, timeout, and high-contention tests. | Tests run on single-core and SMP QEMU; race reports include task/CPU/access context; limitations and false-negative risks are stated. |
| **17** | `docs(sync): publish API, context matrix, and compatibility status` | Document kernel APIs, UAPI/libc APIs, errno/status, object lifecycle, process sharing, architecture support, and limitations. | Every API points to implementation and tests; Linux/BSD/POSIX references are differentiated; capability matrix matches runtime evidence. |

*Recommended Initial Focus*: Complete PRs 1–8 first, gating implementation of blocking mutex/semaphore behavior on the scheduler's real wait/wakeup path.

---

## 5. Periodic Maintenance Cadence

| Cadence | Task | Required Evidence |
|:---|:---|:---|
| **Every PR** | Run primitive-specific tests for changed code | Build/formatting checks, invariant unit tests, Loom models where applicable, and QEMU test if runtime path changed. |
| **Weekly** | Triage deadlocks, stalls, and race reports | Reproducer, affected primitive/context, task trace, owner, severity, and fix or tracked issue. |
| **Weekly** | Review wakeup and timeout regressions | Lost-wakeup, missed-timeout, duplicate-wake, cancellation, and task-exit results on fixed workloads. |
| **Monthly** | Review lock ordering and context use | Lock graph changes, interrupt/preemption context, held-lock rules, lock duration, and sleeping-in-atomic risks reviewed. |
| **Monthly** | Run mutex/semaphore/condition stress matrix | Contention, owner exit, timeout, signal delivery, destruction, and fairness tests with repeatable configuration. |
| **Monthly** | Review shared-object lifecycle and permissions | User pointers, shared-memory identity, namespace isolation, object reuse/ABA, resource limits, and unauthorized wake paths checked. |
| **Quarterly** | Review atomic orderings and architecture assumptions | Audit acquire/release/compare-exchange choices, cache coherence assumptions, alignment, and supported architecture behavior. |
| **Quarterly** | Run SMP and weak-ordering validation | Multi-vCPU stress/model tests; record architecture and limitations; do not infer weak-memory safety from x86-only results. |
| **Each Release Candidate** | Certify synchronization support claims | Primitive/API matrix, runtime tests, ABI docs, known race/deadlock issues, and architecture/configuration evidence. |
| **After Each Incident** | Prevent recurrence of race or deadlock | Minimal reproducer, root cause, regression test, affected versions, data-integrity impact, and backport decision. |

---

## 6. Component-Specific Rules

- **Scheduler / Wait Queues**: Register the waiter before releasing the predicate lock; make sleep and wake transitions atomic from the caller's perspective; remove waiters on timeout, signal, and exit.
- **Mutexes**: Enforce owner semantics; define relock and non-owner unlock behavior; keep critical sections short; do not substitute semaphores for ownership locks. POSIX specifies distinct normal, error-checking, recursive, and robust mutex behaviors.
- **Condition Variables**: Always pair with a mutex and predicate; wait in a loop; permit spurious wakeups; atomically release-and-block, then reacquire before returning. Use monotonic clocks for elapsed-time deadlines where supported.
- **Semaphores**: Define process/thread sharing, maximum count, overflow, post-from-interrupt constraints, fairness, and destruction rules. Do not promise process-shared semantics until shared memory and object lifecycle are integrated.
- **Spinlocks and IRQ Locks**: Use only for tiny bounded critical sections; prohibit sleeping, blocking allocation, or scheduler calls while held; specify local IRQ/preemption behavior. Spin locks must never yield or block.
- **Read/Write Locks**: Document writer starvation policy and upgrade/downgrade behavior; use only where measured read-heavy workloads justify them.
- **Atomics**: Use the weakest ordering that satisfies a written happens-before argument; explain publication, ownership, and reclamation invariants; avoid "stronger is always safer" and unexplained Relaxed.
- **Shared Memory & Futex**: Tie wait keys to stable shared object identity and offset, not a raw user pointer alone; handle unmap, process exit, remapping, and object destruction without waking a reused object incorrectly.
- **IPC Queues & Eventfd**: Reuse common wait/wakeup semantics; define close/EOF, queue saturation, broadcast, timeout, and cancellation behavior consistently.
- **Scheduler Priority**: Decide whether mutex waiters inherit priority; if not implemented, document inversion risk. Do not add priority inheritance until the scheduler and lock owner tracking can support it safely.
- **Debugging**: Keep a debug-only lock dependency graph (`lockdep`) and ownership assertions; record lock class, context, and acquisition site without leaking user data.

---

## 7. PR Review Template & Merge Gates

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

### Review Merge Gates
- **1 Maintainer Review**: Required for isolated documentation and standalone unit test additions.
- **2 Qualified Reviews**: Required for atomic algorithms, scheduler wait queue integrations, IRQ-safe locks, process-shared objects, or syscall/UAPI changes.

---

## 8. Subsystem Completion Gates & Milestone Definitions

- **Prototype**: Data structures or lock models pass unit tests, but are not connected to real running tasks.
- **Integrated**: Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
- **Supported**: Repeatable tests cover lifecycle, ownership, interrupt context, process sharing where claimed, starvation/priority behavior, architecture-specific memory ordering, and documented error semantics.

*Crucial Rule*: Do not call the process synchronization subsystem complete simply because the repository contains semaphore, flock, or IPC filenames. The decisive gate is a booted userspace/process test using the real scheduler and syscall path, backed by deterministic race models and ongoing lock/race diagnostics.
