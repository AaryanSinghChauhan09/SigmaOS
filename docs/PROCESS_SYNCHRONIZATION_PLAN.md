# SigmaOS Process Synchronization Master Architecture, Support Matrix, and PR Plan

## Overview
SigmaOS provides several synchronization-related components across the kernel and userspace, including wait queues, spinlocks, semaphores, file locks (`flock`), pipes, message queues, shared memory (`shm`), `eventfd`, Fast Userspace Mutexes (`futex`), and lock dependency tracking (`lockdep`). This document establishes a formal, integrated process synchronization architecture and PR proposal plan.

Source file presence and standalone unit test passes do not constitute full runtime integration. Full runtime synchronization requires kernel syscall dispatch, Ring 3 process execution, process spawning, and scheduler integration working seamlessly together.

The target architecture is a coherent, layered progression:
1. **Kernel wait/wakeup primitives**: Base task wait queues and scheduler sleep/wake transitions.
2. **Kernel locks and completions**: Sleeping mutexes, bounded spinlocks, IRQ-safe locks, completions, and condition variables.
3. **Userspace synchronization APIs**: Fast-path futex compare-and-block ABIs and POSIX/libc wrappers.
4. **Process-shared IPC synchronization**: Process-shared mutexes, unnamed semaphores, and shared memory mappings.
5. **Concurrency diagnostics**: Runtime lock-dependency graphs (`lockdep`) and race detector instrumentation.

POSIX specifies mutex ownership and condition-variable wait semantics; Linux and FreeBSD document separate locking rules for sleeping and spinning contexts. While these serve as behavior references, kernel internal primitives remain distinct from the userspace ABI.

---

## Current Inventory and Subsystem Boundaries

| Primitive / Module | Source File(s) | Domain Scope | Current Evidence | Status | Milestone Gate |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Wait Queue** | `src/kernel/wait_queue.rs` | Kernel Threads / Sleeping Tasks | Standalone `rustc --test` pass | Prototype | Scheduler EEVDF enqueue/dequeue task integration |
| **Spinlocks** | `src/kernel/sigma_spinlock.rs` | Kernel Atomic / SMP Critical Sections | Standalone `rustc --test` pass | Working | Per-CPU IRQ-save state validation |
| **Semaphore** | `src/kernel/semaphore.rs` | Kernel / Thread Resource Counting | Standalone `rustc --test` pass | Prototype | Process-shared unnamed semaphore support |
| **Futex** | `src/kernel/futex.rs`, `src/ipc/futex.rs` | Process / User-Shared Memory | Standalone `rustc --test` pass | Prototype | Physical page resolution & syscall dispatch |
| **File Lock (`flock`)** | `src/kernel/flock.rs` | Inter-Process File Descriptor Sync | Standalone `rustc --test` pass | Prototype | VFS file table integration |
| **IPC Primitives** | `src/kernel/pipe.rs`, `src/kernel/eventfd.rs` | Inter-Process Notification & Data Flow | Standalone `rustc --test` pass | Prototype | Non-blocking epoll/kqueue event loop binding |
| **Lockdep** | `src/kernel/lockdep.rs` | Kernel Concurrency Diagnostics | Architecture Spec | Prototype | Kernel lock instrumentation |

*Note: No module in the table above is claimed as full runtime-supported without booted task execution evidence.*

---

## 10 Core Design Rules

1. **Name the Synchronization Domain**: Every primitive must explicitly state whether it synchronizes kernel threads, threads within a single process, processes sharing memory, or operations across an IPC channel.
2. **Separate Lock from Event**: Use mutexes for ownership-based mutual exclusion, and condition variables, completions, or wait queues for waiting on state changes. Linux specifically cautions against using semaphores as both locking and waiting mechanisms when separate mutex and completion mechanisms are more appropriate.
3. **Never Sleep in Atomic Context**: Define which primitives may block and strictly prohibit sleeping locks in hard-interrupt, non-preemptible, or atomic contexts. Keep IRQ-safe spinlocks short and bounded. Linux distinguishes sleeping, CPU-local, and spinning lock categories; FreeBSD similarly warns that spin locks must not yield or block.
4. **Specify Ownership and Failure Semantics**: Define recursive, error-checking, and robust behavior, owner-only unlock rules, destruction while waiters exist, and error return codes (`EDEADLK`, `EPERM`, `EOWNERDEAD`). POSIX specifies these distinctions for mutex types and robust owner-death behavior.
5. **Wait on Predicates, Not Notifications**: Condition-variable and wait-queue users must loop and re-check their shared predicate after wakeup; spurious wakeups are permitted. The wait operation must atomically release the associated mutex and block, then reacquire the mutex before returning.
6. **Prevent Lost Wakeups**: State checks and waiter registration must be atomic with respect to notification, timeout, signal delivery, and cancellation. Define explicit memory-ordering requirements for fast paths.
7. **Use Explicit Memory Ordering**: Every atomic variable must have a documented invariant and ordering rationale. Use Rust atomics (`Acquire`, `Release`, `AcqRel`, `SeqCst`) with defined data-race rules; do not use `Relaxed` when a publication or ownership relationship requires synchronization.
8. **Address Priority Inversion and Starvation**: Define fairness policies for mutexes and read/write locks. Consider priority inheritance only after scheduler priority changes and timeout behavior are integrated. Counting semaphores do not identify an owner and therefore cannot generally provide owner-based priority inheritance.
9. **Treat User-Memory Objects as Untrusted**: Validate shared-memory mappings, object handles, ownership, and lifetimes at every syscall boundary. Do not let a user process choose kernel pointers or manipulate kernel waiter queues directly.
10. **Test Races Systematically**: Combine deterministic state-machine tests, Rust model checking (e.g., Loom where feasible), stress tests, and booted QEMU tests. Loom explores modeled concurrent interleavings for instrumented types; Linux uses `lockdep` for lock-order/IRQ-context validation and KCSAN for sampled data-race detection.

---

## 17-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
| :-: | :--- | :--- | :--- |
| **1** | `docs(sync): inventory primitives and define subsystem boundaries` | Audit scheduler wait queues, atomics, spinlocks, semaphores, flock, pipes, message queues, shared memory, signals, event notifications, syscalls, and libc wrappers. | Inventory distinguishes kernel-internal, process-local, process-shared, and modeled code; no module is described as runtime-supported without evidence. |
| **2** | `docs(sync): define synchronization contracts and lock-context rules` | Publish primitive selection guide, lock hierarchy, blocking-context matrix, fairness expectations, memory-order rules, and lifecycle/destruction contract. | Every primitive has documented owner, valid contexts, wake policy, timeout/cancellation behavior, destruction behavior, and test obligations. |
| **3** | `test(sync): add concurrency model and invariant harness` | Add host-side deterministic tests and optional Loom models for atomic fast paths, waiter registration, wakeup, timeout, cancellation, and object destruction. | Models use Loom-aware atomics/locks; bounds are explicit; tests cover lost wakeup and ABA/reuse cases; model tests are not presented as kernel runtime evidence. |
| **4** | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Implement the base wait queue used by sleeping synchronization primitives; connect task states, scheduler enqueue/dequeue, timeout, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; notification cannot be lost between predicate check and sleep; task exits remove wait-queue references safely. |
| **5** | `feat(sync): add kernel mutex with ownership and debug checks` | Implement a sleeping mutex for task context; define ownership, try-lock, recursion policy, and error behavior. | Contended lock sleeps instead of spinning; non-owner unlock fails in debug/test mode; interrupt/atomic context use is rejected; owner and waiter cleanup survive task exit. |
| **6** | `feat(sync): add bounded spinlock and IRQ-safe variants` | Add minimal spinlock/IRQ-save primitives for tiny critical sections; document per-CPU and SMP semantics. | Lock/unlock barriers are tested; debug mode detects recursive acquisition and invalid context; no allocation, blocking, or unbounded work occurs while held. |
| **7** | `feat(sync): add completion and one-shot event primitive` | Add a completion/event for "wait until operation reaches state" use cases; integrate with wait queues and timeout. | Completion before wait is retained; multiple waiters follow documented semantics; repeated complete/reinit behavior is specified; tests cover race and teardown cases. |
| **8** | `feat(sync): add counting semaphore with bounded counter` | Implement wait/post/try-wait/timed-wait with overflow, underflow, and waiter semantics. | Counter never underflows or exceeds configured maximum; waking a waiter transfers availability correctly; interrupt-context post rules are explicit; process sharing is not claimed yet. |
| **9** | `feat(sync): add condition variable with atomic unlock-and-wait` | Implement condition wait/signal/broadcast tied to a mutex; add monotonic timed wait if clock support exists. | Mutex is released atomically with waiter registration; it is reacquired before return; callers are required to re-check predicates; spurious wakeups and timeout races are tested. |
| **10** | `feat(sync): add read-write lock with starvation policy` | Add read/write lock only after mutex/wait queues work; specify reader/writer preference or fairness and recursion policy. | Tests cover concurrent readers, exclusive writer, downgrade/upgrade policy, writer starvation bounds, and cancellation/exit cleanup. |
| **11** | `feat(sync): add robust process-local mutex recovery` | Add owner-death handling for user mutexes where thread lifecycle and shared synchronization objects support it. | A successor receives an owner-dead/inconsistent indication; recovery/consistent/unrecoverable semantics are documented and tested against POSIX expectations. |
| **12** | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Add a word-based wait/wake ABI only after syscall dispatch, safe user-memory access, scheduler wait queues, and process identity are integrated. Use compare-and-block semantics to avoid the check-to-sleep race. | Wait only blocks when the user value still matches; wake targets the correct waiters; invalid pointers and key reuse are safe; ABI is documented and architecture-tested. |
| **13** | `feat(sync): integrate process-shared synchronization with shared memory` | Connect semaphore/mutex objects to shared-memory mappings and object lifetimes; define handle creation, permissions, unlink, and owner cleanup. | Independent processes synchronize through a real shared mapping; permissions and namespace boundaries are enforced; object destruction cannot free state while waiters reference it. |
| **14** | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and event notifications with the same wait/wakeup and cancellation contracts. | Read/write readiness, queue full/empty, close, peer exit, timeout, and signal interactions are tested; no busy-wait loop is required for blocking operations. |
| **15** | `debug(sync): add lock-order checker and wait diagnostics` | Add debug-only lock classes, lock-order graph, owner/waiter diagnostics, held-lock assertions, and deadlock reports. | Debug mode catches inverse lock ordering, recursive misuse, sleeping lock in interrupt context, and wait-while-held violations; diagnostics are bounded and do not expose sensitive data. |
| **16** | `test(sync): add race detector and stress configuration` | Add debug builds or instrumentation for data races where practical; add lock torture, producer/consumer, timeout, and high-contention tests. | Tests run on single-core and SMP QEMU when available; race reports include task/CPU/access context; limitations and false-negative risks are stated. |
| **17** | `docs(sync): publish API, context matrix, and compatibility status` | Document kernel APIs, UAPI/libc APIs, errno/status, object lifecycle, process sharing, architecture support, and limitations. | Every API points to implementation and tests; Linux/BSD/POSIX references are clearly differentiated; capability matrix matches runtime evidence. |

*Recommended Priority Focus: Complete PRs 1–8 first, gating implementation of blocking mutex/semaphore behavior on the scheduler's real wait/wakeup path. Then add condition variables, read/write locks, and later process-shared futex-like behavior.*

---

## Periodic Maintenance PR Cadence

| Cadence | Recurring Task / Focus | Required Evidence |
| :--- | :--- | :--- |
| **Every PR** | `ci(sync): run primitive-specific tests for changed code` | Formatting/build, invariant tests, Loom model where applicable, and QEMU test if runtime path changed. |
| **Weekly** | `maint(sync): triage deadlocks, stalls, and race reports` | Reproducer, affected primitive/context, task trace, owner, severity, and fix or tracked issue. |
| **Weekly** | `test(sync): review wakeup and timeout regressions` | Lost-wakeup, missed-timeout, duplicate-wake, cancellation, and task-exit results on fixed workloads. |
| **Monthly** | `audit(sync): review lock ordering and context use` | Changed lock graph, interrupt/preemption context, held-lock rules, lock duration, and sleeping-in-atomic risks reviewed. |
| **Monthly** | `test(sync): run mutex/semaphore/condition stress matrix` | Contention, owner exit, timeout, signal delivery, destruction, and fairness tests with repeatable configuration. |
| **Monthly** | `security(sync): review shared-object lifecycle and permissions` | User pointers, shared-memory identity, namespace isolation, object reuse/ABA, resource limits, and unauthorized wake/control paths checked. |
| **Quarterly** | `maint(sync): review atomic orderings and architecture assumptions` | Audit acquire/release/compare-exchange choices, cache coherence assumptions, alignment, and supported architecture behavior. |
| **Quarterly** | `test(sync): run SMP and weak-ordering validation` | Multi-vCPU stress/model tests; record architecture and limitations; do not infer weak-memory safety from x86-only results. |
| **Release Candidate** | `release(sync): certify synchronization support claims` | Primitive/API matrix, runtime tests, ABI docs, known race/deadlock issues, and architecture/configuration evidence. |
| **Post-Incident** | `fix(sync): prevent recurrence of <race/deadlock>` | Minimal reproducer, root cause, regression test, affected versions, data-integrity/security impact, and backport decision. |

---

## Component-Specific Rules

- **Scheduler / Wait Queues**: Register the waiter before releasing the predicate lock; make sleep and wake transitions atomic from the caller's perspective; remove waiters on timeout, signal, and task exit.
- **Mutexes**: Enforce owner semantics; define relock and non-owner unlock behavior; keep critical sections short; do not substitute semaphores for ownership locks.
- **Condition Variables**: Always pair with a mutex and predicate; wait in a loop; permit spurious wakeups; atomically release-and-block, then reacquire before returning. Use monotonic clocks for elapsed-time deadlines where supported.
- **Semaphores**: Define process/thread sharing, maximum count, overflow, post-from-interrupt constraints, fairness, and destruction rules. Do not promise process-shared semantics until shared memory and object lifecycle are integrated.
- **Spinlocks and IRQ Locks**: Use only for tiny bounded critical sections; prohibit sleeping, blocking allocation, or scheduler calls while held; specify local IRQ/preemption behavior.
- **Read/Write Locks**: Document writer starvation policy and upgrade/downgrade behavior; use only where measured read-heavy workloads justify them.
- **Atomics**: Use the weakest ordering that satisfies a written happens-before argument; explain publication, ownership, and reclamation invariants; avoid "stronger is always safer" and unexplained `Relaxed`.
- **Shared Memory / Futex**: Tie wait keys to stable shared object identity and offset, not a raw user pointer alone; handle unmap, process exit, remapping, and object destruction without waking a reused object incorrectly.
- **IPC Queues and Eventfd**: Reuse common wait/wakeup semantics; define close/EOF, queue saturation, broadcast, timeout, and cancellation behavior consistently.
- **Scheduler Priority**: Decide whether mutex waiters inherit priority; if not implemented, document inversion risk. Do not add priority inheritance until the scheduler and lock owner tracking can support it safely.
- **Debugging**: Keep a debug-only lock dependency graph and ownership assertions; record lock class, context, and acquisition site without leaking user data.

---

## PR Review Template & Completion Gates

### PR Review Template
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

### Subsystem Completion Gates
1. **Prototype**: Data structures or lock models pass unit tests, but are not connected to real booted tasks.
2. **Integrated**: Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
3. **Supported**: Repeatable tests cover lifecycle, ownership, interrupt context, process sharing where claimed, starvation/priority behavior, architecture-specific memory ordering, and documented error semantics.

*The decisive gate for process synchronization completeness is a booted userspace/process test using the real scheduler and syscall path, backed by deterministic race models and ongoing lock/race diagnostics.*
