# SigmaOS Process Synchronization Master Architecture & PR Proposal Plan

## Executive Overview
SigmaOS contains synchronization-related code across modules such as `semaphore`, `flock`, `pipe`, `msgqueue`, `shm`, `eventfd`, `futex`, `wait_queue`, and `sigma_spinlock`. However, source presence and unit tests alone do not establish a complete, runtime-integrated process synchronization subsystem. Subsystem milestones—including syscall dispatch, process spawning, Ring 3 execution, and scheduler wait/wakeup integration—are prerequisites for full process-shared synchronization.

This plan establishes a formal audit, design baseline, and implementation roadmap to connect and validate these synchronization primitives into a coherent 5-stage architecture.

---

## Architectural Progression
The target architecture follows a strict 5-layer progression:

1. **Kernel wait/wakeup primitives**: Scheduler-integrated base wait queue (`WaitQueue`, `WaitQueueEntry`), task state transitions (`TaskState::Blocked`), timeout timers, and wakeup dispatch.
2. **Kernel locks and completions**: Context-aware sleeping locks (`Mutex`), bounded spinlocks (`SigmaSpinlock`, `SigmaRwSpinlock`), completions (`Completion`), and counting semaphores (`Semaphore`).
3. **Userspace synchronization APIs**: POSIX/C-library sync primitives (mutexes, condition variables, semaphores, read-write locks) backed by kernel syscalls.
4. **Process-shared IPC synchronization**: Fast Userspace Mutexes (`futex`), shared memory mapping sync (`shm`), advisory file locks (`flock`), anonymous/named pipes (`pipe`), event notifications (`eventfd`), and POSIX message queues (`msgqueue`).
5. **Concurrency diagnostics**: Runtime lock dependency graph checker (`lockdep`), lock context validation, held-lock assertions, and data-race detection instrumentation.

---

## Core Design Rules

1. **Name the synchronization domain**: Each primitive must explicitly specify whether it synchronizes kernel threads, threads within a single process, processes sharing memory, or operations across an IPC channel.
2. **Separate lock from event**: Use mutexes strictly for ownership-based mutual exclusion. Use condition variables, completions, or wait queues for waiting on state changes. Do not substitute semaphores for ownership locks.
3. **Never sleep in atomic context**: Distinguish sleeping locks (mutexes, semaphores) from spinning locks (spinlocks, IRQ-save spinlocks). Sleeping locks are forbidden in hard-interrupt, non-preemptible, or atomic contexts. Keep IRQ-safe spinlocks short and bounded.
4. **Specify ownership and failure semantics**: Define recursive, error-checking, and robust behavior, owner-only unlock rules, destruction rules while waiters exist, and error returns (e.g., `EDEADLK`, `EPERM`, `ETIMEDOUT`, `EOWNERDEAD`).
5. **Wait on predicates, not notifications**: Callers of condition variables and wait queues must loop and re-check shared predicates after wakeup. Spurious wakeups are permitted. Wait operations must atomically release the mutex/predicate lock and block, reacquiring the lock before returning.
6. **Prevent lost wakeups**: Ensure state checks and waiter registration are atomic with respect to notification, timeout, signal delivery, and cancellation. Enforce strict memory-ordering requirements on fast paths.
7. **Use explicit memory ordering**: Every atomic variable must have a written invariant and ordering rationale. Do not use `Relaxed` ordering when publication, ownership transfer, or synchronization is required.
8. **Address priority inversion and starvation**: Define fairness policies for mutexes and read/write locks. Priority inheritance (`PI`) will be integrated after task priority mutation and owner-waiter tracking are integrated. Counting semaphores do not track owners and cannot support owner-based priority inheritance.
9. **Treat user-memory objects as untrusted**: Validate shared-memory mappings, object handles, user pointers, permissions, and lifetimes at every syscall boundary. Never permit user processes to manipulate kernel waiter queues or supply raw kernel pointers.
10. **Test races systematically**: Combine deterministic state-machine unit tests, host-side model checking (e.g., Loom where applicable), stress tests, and booted QEMU multi-vCPU tests.

---

## 17-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
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
| **12** | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Add a word-based wait/wake ABI only after syscall dispatch, safe user-memory access, scheduler wait queues, and process identity are integrated. | Wait only blocks when user value matches key; wake targets correct waiters; invalid user pointers and key reuse are safe; ABI documented and architecture-tested. |
| **13** | `feat(sync): integrate process-shared synchronization with shared memory` | Connect semaphore/mutex objects to shared-memory mappings and object lifetimes; define handle creation, permissions, unlink, and owner cleanup. | Independent processes synchronize through real shared mapping; permissions and namespace boundaries enforced; object destruction cannot free state while waiters reference it. |
| **14** | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and event notifications with the same wait/wakeup and cancellation contracts. | Read/write readiness, queue full/empty, close, peer exit, timeout, and signal interactions are tested; no busy-wait loop required for blocking operations. |
| **15** | `debug(sync): add lock-order checker and wait diagnostics` | Add debug-only lock classes, lock-order graph, owner/waiter diagnostics, held-lock assertions, and deadlock reports. | Debug mode catches inverse lock ordering, recursive misuse, sleeping lock in interrupt context, and wait-while-held violations; diagnostics bounded. |
| **16** | `test(sync): add race detector and stress configuration` | Add debug builds or instrumentation for data races where practical; add lock torture, producer/consumer, timeout, and high-contention tests. | Tests run on single-core and SMP QEMU when available; race reports include task/CPU/access context; limitations stated. |
| **17** | `docs(sync): publish API, context matrix, and compatibility status` | Document kernel APIs, UAPI/libc APIs, errno/status, object lifecycle, process sharing, architecture support, and limitations. | Every API points to implementation and tests; Linux/BSD/POSIX references differentiated; capability matrix matches runtime evidence. |

### Execution Strategy
Focus priority on **PRs 1–8** first. Gate blocking mutex and semaphore behavior on the scheduler's real wait/wakeup path. Follow with condition variables (PR 9) and read/write locks (PR 10), then futexes (PR 12) and process-shared synchronization (PRs 13–14).

---

## Periodic Maintenance Cadence
- **Every PR**: Run primitive-specific tests (`cargo test`, `rustc --test`). Verify formatting and memory ordering arguments.
- **Weekly**: Triage deadlocks, stalls, and race reports. Review wakeup and timeout regressions on fixed workloads.
- **Monthly**: Audit lock ordering, preemption/interrupt context rules, sleeping-in-atomic risks, and high-contention stress matrices. Review shared-object permissions and lifecycle safety.
- **Quarterly**: Review atomic memory orderings, architecture assumptions (x86_64 vs weak ordering), and run multi-vCPU SMP stress tests.
- **Release Candidate**: Certify synchronization support claims against runtime test evidence.
- **After Each Incident**: Publish fix PR with minimal reproducer, root-cause analysis, and regression test.

---

## Component-Specific Rules
- **Scheduler / Wait Queues**: Register waiter before releasing predicate lock; make sleep/wake transitions atomic from caller perspective; remove waiters on timeout, signal delivery, or task exit.
- **Mutexes**: Enforce owner semantics; define non-owner unlock and relock errors; keep critical sections short; prohibit mutexes in atomic context.
- **Condition Variables**: Always pair with a mutex and predicate loop; permit spurious wakeups; atomically release mutex on wait and reacquire before returning.
- **Semaphores**: Define process vs thread sharing, maximum counter, overflow handling, post-from-interrupt constraints, and destruction rules.
- **Spinlocks and IRQ Locks**: Use strictly for tiny bounded critical sections; prohibit sleeping, heap allocation, or scheduler calls while held.
- **Read/Write Locks**: Document writer starvation policy and upgrade/downgrade behavior; use only for measured read-heavy workloads.
- **Atomics**: Use explicit memory ordering with written happens-before rationale; avoid unexplained `Relaxed`.
- **Shared Memory / Futex**: Key futexes to stable physical/object identity, handle unmap, process exit, and key reuse safely.
- **IPC Queues & eventfd**: Reuse common wait/wakeup semantics; define close/EOF, queue saturation, and cancellation behavior consistently.
- **Debugging & lockdep**: Track lock dependency graph, lock classes, and acquisition site assertions in debug mode.

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

### Merge Gates & Approvals
- Isolated tests and documentation PRs require **1 maintainer approval**.
- Atomic algorithms, scheduler wait queues, IRQ-safe locks, process-shared objects, or UAPI/syscall changes require **2 qualified reviews**.

---

## Completion Gates
1. **Prototype**: Data structures or lock models pass unit tests, but are not connected to booted tasks.
2. **Integrated**: Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
3. **Supported**: Repeatable tests cover lifecycle, ownership, interrupt context, process sharing, starvation/priority behavior, weak memory ordering, and documented error semantics.
