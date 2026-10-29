# SigmaOS Process Synchronization Master Architecture & Development Plan

## Executive Summary & Context

SigmaOS contains synchronization-related code across modules named semaphore, flock, pipe, msgqueue, shm, eventfd, futex-adjacent interfaces, and scheduler/wait-related components. However, source presence and unit tests do not prove runtime integration. Syscall dispatch, Ring 3 process spawning, and scheduler integration remain incomplete milestones.

This plan establishes a coherent architecture and development roadmap to audit, refine, and connect these primitives into an integrated process synchronization subsystem.

The target architecture follows a 5-layer progression:
1. **Kernel wait/wakeup primitives**
2. **Kernel locks and completions**
3. **Userspace synchronization APIs**
4. **Process-shared IPC synchronization**
5. **Concurrency diagnostics**

POSIX specifies mutex ownership and condition-variable wait semantics; Linux and FreeBSD document separate locking rules for sleeping and spinning contexts. These serve as useful behavior references, but the kernel’s internal primitives remain distinct from its userspace ABI.

---

## Core Design Rules

1. **Name the synchronization domain**: Each primitive must state whether it synchronizes kernel threads, threads within one process, processes sharing memory, or operations across an IPC channel.
2. **Separate lock from event**: Use mutexes for ownership-based exclusion and condition variables/completions/wait queues for waiting on state changes. Linux specifically cautions against using semaphores as both locking and waiting mechanisms when separate mutex and completion mechanisms are more appropriate.
3. **Never sleep in atomic context**: Define which primitives may block and prohibit sleeping locks in hard-interrupt, non-preemptible, or other atomic contexts. Keep IRQ-safe spinlocks short and bounded. Linux distinguishes sleeping, CPU-local, and spinning lock categories; FreeBSD similarly warns that spin locks must not yield or block.
4. **Specify ownership and failure semantics**: Define recursive/error-checking/robust behavior, owner-only unlock rules, destruction while waiters exist, and error returns. POSIX specifies these distinctions for mutex types and robust owner-death behavior.
5. **Wait on predicates, not notifications**: Condition-variable users must loop and re-check their shared predicate after wakeup; spurious wakeups are permitted. The wait operation must atomically release the mutex and block, then reacquire it before returning.
6. **Prevent lost wakeups**: Make state checks and waiter registration atomic with respect to notification, timeout, signal, and cancellation; define memory-ordering requirements for fast paths.
7. **Use explicit memory ordering**: Every atomic variable has a documented invariant and ordering rationale. Rust atomics have defined ordering semantics and data-race rules; do not use `Relaxed` when a publication or ownership relationship requires synchronization.
8. **Address priority inversion and starvation**: Define fairness policy for mutexes and read/write locks; consider priority inheritance only after scheduler priority changes and timeout behavior are integrated. Counting semaphores do not identify an owner and therefore cannot generally provide owner-based priority inheritance.
9. **Treat user-memory objects as untrusted**: Validate shared-memory mappings, object handles, ownership, and lifetimes at every syscall boundary. Do not let a user process choose kernel pointers or manipulate kernel waiter queues directly.
10. **Test races systematically**: Combine deterministic state-machine tests, Rust model checking where feasible, stress tests, and booted QEMU tests. Loom explores modeled concurrent interleavings for instrumented code; Linux uses lockdep for lock-order/IRQ-context validation and KCSAN for sampled data-race detection.

---

## Proposed PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(sync): inventory primitives and define subsystem boundaries` | Audit scheduler wait queues, atomics, spinlocks, semaphores, flock, pipes, message queues, shared memory, signals, event notifications, syscalls, and libc wrappers. Record source, runtime path, current evidence, owner, and dependencies. | Inventory distinguishes kernel-internal, process-local, process-shared, and modeled code; no module is described as runtime-supported without evidence. |
| 2 | `docs(sync): define synchronization contracts and lock-context rules` | Publish primitive selection guide, lock hierarchy, blocking-context matrix, fairness expectations, memory-order rules, and lifecycle/destruction contract. | Every primitive has documented owner, valid contexts, wake policy, timeout/cancellation behavior, destruction behavior, and test obligations. |
| 3 | `test(sync): add concurrency model and invariant harness` | Add host-side deterministic tests and optional Loom models for atomic fast paths, waiter registration, wakeup, timeout, cancellation, and object destruction. | Models use Loom-aware atomics/locks; bounds are explicit; tests cover lost wakeup and ABA/reuse cases; model tests are not presented as kernel runtime evidence. |
| 4 | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Implement the base wait queue used by sleeping synchronization primitives; connect task states, scheduler enqueue/dequeue, timeout, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; notification cannot be lost between predicate check and sleep; task exits remove wait-queue references safely. |
| 5 | `feat(sync): add kernel mutex with ownership and debug checks` | Implement a sleeping mutex for task context; define ownership, try-lock, recursion policy, and error behavior. | Contended lock sleeps instead of spinning; non-owner unlock fails in debug/test mode; interrupt/atomic context use is rejected; owner and waiter cleanup survive task exit. |
| 6 | `feat(sync): add bounded spinlock and IRQ-safe variants` | Add minimal spinlock/IRQ-save primitives for tiny critical sections; document per-CPU and SMP semantics. | Lock/unlock barriers are tested; debug mode detects recursive acquisition and invalid context; no allocation, blocking, or unbounded work occurs while held. |
| 7 | `feat(sync): add completion and one-shot event primitive` | Add a completion/event for “wait until operation reaches state” use cases; integrate with wait queues and timeout. | Completion before wait is retained; multiple waiters follow documented semantics; repeated complete/reinit behavior is specified; tests cover race and teardown cases. |
| 8 | `feat(sync): add counting semaphore with bounded counter` | Implement wait/post/try-wait/timed-wait with overflow, underflow, and waiter semantics. | Counter never underflows or exceeds configured maximum; waking a waiter transfers availability correctly; interrupt-context post rules are explicit; process sharing is not claimed yet. |
| 9 | `feat(sync): add condition variable with atomic unlock-and-wait` | Implement condition wait/signal/broadcast tied to a mutex; add monotonic timed wait if clock support exists. | Mutex is released atomically with waiter registration; it is reacquired before return; callers are required to re-check predicates; spurious wakeups and timeout races are tested. |
| 10 | `feat(sync): add read-write lock with starvation policy` | Add read/write lock only after mutex/wait queues work; specify reader/writer preference or fairness and recursion policy. | Tests cover concurrent readers, exclusive writer, downgrade/upgrade policy, writer starvation bounds, and cancellation/exit cleanup. |
| 11 | `feat(sync): add robust process-local mutex recovery` | Add owner-death handling for user mutexes where thread lifecycle and shared synchronization objects support it. | A successor receives an owner-dead/inconsistent indication; recovery/consistent/unrecoverable semantics are documented and tested against POSIX expectations. |
| 12 | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Add a word-based wait/wake ABI only after syscall dispatch, safe user-memory access, scheduler wait queues, and process identity are integrated. Use compare-and-block semantics to avoid the check-to-sleep race. | Wait only blocks when the user value still matches; wake targets the correct waiters; invalid pointers and key reuse are safe; ABI is documented and architecture-tested. Linux’s futex family provides a reference for userspace-built mutexes, semaphores, and condition variables. |
| 13 | `feat(sync): integrate process-shared synchronization with shared memory` | Connect semaphore/mutex objects to shared-memory mappings and object lifetimes; define handle creation, permissions, unlink, and owner cleanup. | Independent processes synchronize through a real shared mapping; permissions and namespace boundaries are enforced; object destruction cannot free state while waiters reference it. POSIX allows process-shared unnamed semaphores where accessible to multiple processes. |
| 14 | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and event notifications with the same wait/wakeup and cancellation contracts. | Read/write readiness, queue full/empty, close, peer exit, timeout, and signal interactions are tested; no busy-wait loop is required for blocking operations. |
| 15 | `debug(sync): add lock-order checker and wait diagnostics` | Add debug-only lock classes, lock-order graph, owner/waiter diagnostics, held-lock assertions, and deadlock reports. | Debug mode catches inverse lock ordering, recursive misuse, sleeping lock in interrupt context, and wait-while-held violations; diagnostics are bounded and do not expose sensitive data. Linux lockdep tracks lock classes, dependencies, and IRQ usage to flag potential deadlocks. |
| 16 | `test(sync): add race detector and stress configuration` | Add debug builds or instrumentation for data races where practical; add lock torture, producer/consumer, timeout, and high-contention tests. | Tests run on single-core and SMP QEMU when available; race reports include task/CPU/access context; limitations and false-negative risks are stated. KCSAN is a sampling race detector, not a proof of race freedom. |
| 17 | `docs(sync): publish API, context matrix, and compatibility status` | Document kernel APIs, UAPI/libc APIs, errno/status, object lifecycle, process sharing, architecture support, and limitations. | Every API points to implementation and tests; Linux/BSD/POSIX references are clearly differentiated; capability matrix matches runtime evidence. |

**Priority order**: Complete PRs 1–8 first, but gate implementation of blocking mutex/semaphore behavior on the scheduler’s real wait/wakeup path. Then add condition variables, read/write locks, and only later process-shared futex-like behavior. SigmaOS’s published milestones—kernel syscall dispatch, Ring 3 execution, process spawning, and scheduler integration—are prerequisites for meaningful process-shared synchronization.

---

## Periodic Maintenance PR Cadence

| Cadence | Proposed Recurring PR/Task | Required Evidence |
|---|---|---|
| Every PR | `ci(sync): run primitive-specific tests for changed code` | Formatting/build, invariant tests, Loom model where applicable, and QEMU test if the runtime path changed; skipped checks explained. |
| Weekly | `maint(sync): triage deadlocks, stalls, and race reports` | Reproducer, affected primitive/context, task trace, owner, severity, and fix or tracked issue. |
| Weekly | `test(sync): review wakeup and timeout regressions` | Lost-wakeup, missed-timeout, duplicate-wake, cancellation, and task-exit results on fixed workloads. |
| Monthly | `audit(sync): review lock ordering and context use` | Changed lock graph, interrupt/preemption context, held-lock rules, lock duration, and sleeping-in-atomic risks reviewed. |
| Monthly | `test(sync): run mutex/semaphore/condition stress matrix` | Contention, owner exit, timeout, signal delivery, destruction, and fairness tests with repeatable configuration. |
| Monthly | `security(sync): review shared-object lifecycle and permissions` | User pointers, shared-memory identity, namespace isolation, object reuse/ABA, resource limits, and unauthorized wake/control paths checked. |
| Quarterly | `maint(sync): review atomic orderings and architecture assumptions` | Audit acquire/release/compare-exchange choices, cache coherence assumptions, alignment, and supported architecture behavior. |
| Quarterly | `test(sync): run SMP and weak-ordering validation` | Multi-vCPU stress/model tests; record architecture and limitations; do not infer weak-memory safety from x86-only results. |
| Each release candidate | `release(sync): certify synchronization support claims` | Primitive/API matrix, runtime tests, ABI docs, known race/deadlock issues, and architecture/configuration evidence. |
| After each incident | `fix(sync): prevent recurrence of <race/deadlock>` | Minimal reproducer, root cause, regression test, affected versions, data-integrity/security impact, and backport decision. |

These should be recurring review tasks or candidate PRs, not automatic implementation changes. Linux’s lockdep and KCSAN show how runtime lock-order validation and race detection can complement tests; neither should be represented as a formal proof of correctness.

---

## Component-Specific Rules

- **Scheduler/wait queues**: Register the waiter before releasing the predicate lock; make sleep and wake transitions atomic from the caller’s perspective; remove waiters on timeout, signal, and exit.
- **Mutexes**: Enforce owner semantics; define relock and non-owner unlock behavior; keep critical sections short; do not substitute semaphores for ownership locks. POSIX specifies distinct normal, error-checking, recursive, and robust mutex behaviors.
- **Condition variables**: Always pair with a mutex and predicate; wait in a loop; permit spurious wakeups; atomically release-and-block, then reacquire before returning. Use monotonic clocks for elapsed-time deadlines where supported.
- **Semaphores**: Define process/thread sharing, maximum count, overflow, post-from-interrupt constraints, fairness, and destruction rules. Do not promise process-shared semantics until shared memory and object lifecycle are integrated.
- **Spinlocks and IRQ locks**: Use only for tiny bounded critical sections; prohibit sleeping, blocking allocation, or scheduler calls while held; specify local IRQ/preemption behavior. FreeBSD documents that spin locks must not yield; Linux separates spin, CPU-local, and sleeping lock classes.
- **Read/write locks**: Document writer starvation policy and upgrade/downgrade behavior; use only where measured read-heavy workloads justify them.
- **Atomics**: Use the weakest ordering that satisfies a written happens-before argument; explain publication, ownership, and reclamation invariants; avoid “stronger is always safer” and unexplained `Relaxed`.
- **Shared memory/futex**: Tie wait keys to stable shared object identity and offset, not a raw user pointer alone; handle unmap, process exit, remapping, and object destruction without waking a reused object incorrectly.
- **IPC queues and eventfd**: Reuse common wait/wakeup semantics; define close/EOF, queue saturation, broadcast, timeout, and cancellation behavior consistently.
- **Scheduler priority**: Decide whether mutex waiters inherit priority; if not implemented, document inversion risk. Do not add priority inheritance until the scheduler and lock owner tracking can support it safely.
- **Debugging**: Keep a debug-only lock dependency graph and ownership assertions; record lock class, context, and acquisition site without leaking user data.

---

## PR Review Template and Merge Gates

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

### Review Rules & Merge Gates
- **Maintainer approvals**: Require one maintainer approval for isolated tests/docs and two qualified reviews for atomic algorithms, scheduler wait queues, IRQ-safe locks, process-shared objects, or syscall/UAPI changes.
- **Branch protection**: Protect the default branch with required PR reviews and passing checks.

---

## Completion Gates

- **Prototype**: Data structures or lock models pass unit tests, but are not connected to real tasks.
- **Integrated**: Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
- **Supported**: Repeatable tests cover lifecycle, ownership, interrupt context, process sharing where claimed, starvation/priority behavior, architecture-specific memory ordering, and documented error semantics.

Do not call the process synchronization subsystem complete because the repository contains semaphore, flock, or IPC filenames. The decisive gate is a booted userspace/process test using the real scheduler and syscall path, backed by deterministic race models and ongoing lock/race diagnostics.
