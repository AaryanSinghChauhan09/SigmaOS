# SigmaOS Process Synchronization Architecture, Design Rules, and Roadmap Plan

## 1. Subsystem Overview & Architectural Boundaries

SigmaOS contains various synchronization-related source modules across the kernel and IPC layers, including `wait_queue`, `sigma_spinlock`, `semaphore`, `futex`, `flock`, `pipe`, `eventfd`, and `lockdep`. However, source file presence alone does not establish a complete, runtime-integrated process synchronization subsystem. This document defines the canonical master architecture, 5-stage progression model, core design rules, 17-PR roadmap, periodic maintenance cadences, component-specific rules, PR review templates, and completion gates to guide full runtime integration.

### The 5-Stage Architectural Progression Model
1. **Kernel wait/wakeup primitives**: Base wait queues and task state transitions (`TASK_INTERRUPTIBLE`, `TASK_UNINTERRUPTIBLE`) integrated with the scheduler.
2. **Kernel locks and completions**: Sleeping mutexes, bounded spinlocks, IRQ-safe spinlocks, completions, and counting semaphores for kernel thread coordination.
3. **Userspace synchronization APIs**: Condition variables, read/write locks, and futex-style compare-and-block syscalls (`sys_futex`) for process-local fast paths.
4. **Process-shared IPC synchronization**: Unnamed semaphores and mutexes in shared memory (`shm`), file advisory locking (`flock`), pipes, and `eventfd` counter notifications across process boundaries.
5. **Concurrency diagnostics**: Runtime lock-dependency tracking graph (`lockdep`), static/dynamic race detection hooks, and lock-torture stress harnesses.

---

## 2. Core Design Rules

1. **Name the synchronization domain**: Each primitive must state whether it synchronizes kernel threads, threads within one process, processes sharing memory, or operations across an IPC channel.
2. **Separate lock from event**: Use mutexes for ownership-based mutual exclusion and condition variables/completions/wait queues for waiting on state changes. Do not use semaphores as generic locks when mutexes or completions are appropriate.
3. **Never sleep in atomic context**: Define which primitives may block and strictly prohibit sleeping locks in hard-interrupt, non-preemptible, or atomic contexts. Keep IRQ-safe spinlocks short, non-allocating, and strictly bounded.
4. **Specify ownership and failure semantics**: Define recursive/error-checking/robust behavior, owner-only unlock rules, destruction while waiters exist, and explicit error returns (`EDEADLK`, `EPERM`, `ETIMEDOUT`).
5. **Wait on predicates, not notifications**: Condition-variable users must loop and re-check their shared predicate after wakeup; spurious wakeups are permitted. Wait operations must atomically release the mutex and block, then reacquire the mutex before returning.
6. **Prevent lost wakeups**: State checks and waiter registration must be atomic with respect to notifications, timeouts, signals, and cancellations. Define explicit memory-ordering invariants on fast paths.
7. **Use explicit memory ordering**: Every atomic variable must have a documented invariant and ordering rationale. Do not use `Relaxed` when publication, release-acquire, or ownership relationships require synchronization.
8. **Address priority inversion and starvation**: Define fairness policies for mutexes and read/write locks. Priority inheritance (PI) will be introduced only after scheduler task priority structures and lock owner tracking are fully integrated.
9. **Treat user-memory objects as untrusted**: Validate shared-memory mappings, object handles, user pointers, permissions, and lifetimes at every syscall boundary. Do not allow user processes to manipulate kernel waiter queues or supply arbitrary kernel pointers.
10. **Test races systematically**: Combine deterministic state-machine unit tests, Rust model checking (e.g. Loom) where feasible, stress testing under contention, and booted QEMU integration tests.

---

## 3. Proposed PR Sequence (17-PR Roadmap)

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(sync): inventory primitives and define subsystem boundaries` | Audit scheduler wait queues, atomics, spinlocks, semaphores, flock, pipes, message queues, shared memory, signals, event notifications, syscalls, and libc wrappers. | Distinguishes kernel-internal, process-local, process-shared, and modeled code; no module claimed as runtime-supported without evidence. |
| **2** | `docs(sync): define synchronization contracts and lock-context rules` | Publish primitive selection guide, lock hierarchy, blocking-context matrix, fairness expectations, memory-order rules, and lifecycle/destruction contracts. | Every primitive has documented owner, valid contexts, wake policy, timeout/cancellation behavior, destruction behavior, and test obligations. |
| **3** | `test(sync): add concurrency model and invariant harness` | Add host-side deterministic tests and optional Loom models for atomic fast paths, waiter registration, wakeup, timeout, cancellation, and object destruction. | Models use Loom-aware atomics/locks; bounds are explicit; tests cover lost wakeup and ABA/reuse cases; model tests are not presented as kernel runtime evidence. |
| **4** | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Implement base wait queue used by sleeping synchronization primitives; connect task states, scheduler enqueue/dequeue, timeout, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; notification cannot be lost between predicate check and sleep; task exits remove wait-queue references safely. |
| **5** | `feat(sync): add kernel mutex with ownership and debug checks` | Implement sleeping mutex for task context; define ownership, try-lock, recursion policy, and error behavior. | Contended lock sleeps instead of spinning; non-owner unlock fails in debug mode; interrupt/atomic context use rejected; owner and waiter cleanup survive task exit. |
| **6** | `feat(sync): add bounded spinlock and IRQ-safe variants` | Add minimal spinlock/IRQ-save primitives for tiny critical sections; document per-CPU and SMP semantics. | Lock/unlock barriers tested; debug mode detects recursive acquisition and invalid context; no allocation, blocking, or unbounded work occurs while held. |
| **7** | `feat(sync): add completion and one-shot event primitive` | Add completion/event for "wait until operation reaches state" use cases; integrate with wait queues and timeout. | Completion before wait is retained; multiple waiters follow documented semantics; repeated complete/reinit behavior specified; tests cover race and teardown cases. |
| **8** | `feat(sync): add counting semaphore with bounded counter` | Implement wait/post/try-wait/timed-wait with overflow, underflow, and waiter semantics. | Counter never underflows or exceeds maximum; waking a waiter transfers availability correctly; interrupt-context post rules explicit; process sharing not claimed yet. |
| **9** | `feat(sync): add condition variable with atomic unlock-and-wait` | Implement condition wait/signal/broadcast tied to a mutex; add monotonic timed wait if clock support exists. | Mutex released atomically with waiter registration; reacquired before return; callers required to re-check predicates; spurious wakeups and timeout races tested. |
| **10** | `feat(sync): add read-write lock with starvation policy` | Add read/write lock only after mutex/wait queues work; specify reader/writer preference or fairness and recursion policy. | Tests cover concurrent readers, exclusive writer, downgrade/upgrade policy, writer starvation bounds, and cancellation/exit cleanup. |
| **11** | `feat(sync): add robust process-local mutex recovery` | Add owner-death handling for user mutexes where thread lifecycle and shared synchronization objects support it. | A successor receives owner-dead/inconsistent indication; recovery/consistent/unrecoverable semantics documented and tested against POSIX expectations. |
| **12** | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Add word-based wait/wake ABI after syscall dispatch, safe user-memory access, scheduler wait queues, and process identity are integrated. Use compare-and-block semantics to avoid check-to-sleep race. | Wait only blocks when user value still matches; wake targets correct waiters; invalid pointers and key reuse are safe; ABI documented and architecture-tested. |
| **13** | `feat(sync): integrate process-shared synchronization with shared memory` | Connect semaphore/mutex objects to shared-memory mappings and object lifetimes; define handle creation, permissions, unlink, and owner cleanup. | Independent processes synchronize through real shared mapping; permissions and namespace boundaries enforced; object destruction cannot free state while waiters reference it. |
| **14** | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and event notifications with same wait/wakeup and cancellation contracts. | Read/write readiness, queue full/empty, close, peer exit, timeout, and signal interactions tested; no busy-wait loop required for blocking operations. |
| **15** | `debug(sync): add lock-order checker and wait diagnostics` | Add debug-only lock classes, lock-order graph, owner/waiter diagnostics, held-lock assertions, and deadlock reports. | Debug mode catches inverse lock ordering, recursive misuse, sleeping lock in interrupt context, and wait-while-held violations; diagnostics bounded and privacy-preserving. |
| **16** | `test(sync): add race detector and stress configuration` | Add debug builds or instrumentation for data races where practical; add lock torture, producer/consumer, timeout, and high-contention tests. | Tests run on single-core and SMP QEMU; race reports include task/CPU/access context; limitations and false-negative risks stated. |
| **17** | `docs(sync): publish API, context matrix, and compatibility status` | Document kernel APIs, UAPI/libc APIs, errno/status, object lifecycle, process sharing, architecture support, and limitations. | Every API points to implementation and tests; Linux/BSD/POSIX references clearly differentiated; capability matrix matches runtime evidence. |

---

## 4. Priority Order & Execution Strategy

- **First Milestone**: Complete PRs 1–8 first. Implementation of blocking mutex/semaphore behavior is strictly gated on the scheduler's real wait/wakeup path.
- **Second Milestone**: Add condition variables (PR 9) and read/write locks (PR 10).
- **Third Milestone**: Robust mutex recovery (PR 11), process-shared futex syscalls (PR 12), shared-memory synchronization (PR 13), and interprocess event synchronization (PR 14).
- **Prerequisite Gate**: Kernel syscall dispatch, Ring 3 execution, process spawning, and scheduler task state transitions are prerequisites for process-shared runtime synchronization claims.

---

## 5. Periodic Maintenance Cadence

| Cadence | Task Name | Required Evidence & Actions |
|---|---|---|
| **Every PR** | `ci(sync)` | Run primitive-specific unit tests, invariant checks, formatting/build checks, and Loom models where applicable. |
| **Weekly** | `maint(sync)` | Triage deadlocks, stalls, and race reports. Document reproducers, task traces, owner, and issue tracking. |
| **Weekly** | `test(sync)` | Review wakeup and timeout regressions across lost-wakeup, missed-timeout, and task-exit benchmarks. |
| **Monthly** | `audit(sync)` | Review lock ordering and context usage. Inspect changed lock graphs and sleeping-in-atomic risks. |
| **Monthly** | `test(sync)` | Run mutex/semaphore/condition stress matrix under heavy contention, owner exit, and signal delivery. |
| **Monthly** | `security(sync)` | Review shared-object lifecycle and permissions. Validate user pointers, handle mapping, and namespace isolation. |
| **Quarterly** | `maint(sync)` | Review atomic orderings and architecture assumptions. Audit acquire/release/compare-exchange memory barriers. |
| **Quarterly** | `test(sync)` | Run SMP and weak-ordering validation tests across multi-vCPU configurations. |
| **Release Candidate** | `release(sync)` | Certify synchronization claims. Verify primitive capability matrix matches runtime test evidence. |
| **After Incident** | `fix(sync)` | Prevent recurrence of races/deadlocks. Produce minimal reproducer, root cause analysis, and regression tests. |

---

## 6. Component-Specific Rules

- **Scheduler / Wait Queues**: Register the waiter before releasing the predicate lock; make sleep and wake transitions atomic from caller perspective; remove waiters on timeout, signal, and exit.
- **Mutexes**: Enforce owner semantics; define relock (`EDEADLK`) and non-owner unlock (`EPERM`) behavior; keep critical sections short; never substitute semaphores for ownership locks.
- **Condition Variables**: Always pair with a mutex and predicate; wait in a loop; permit spurious wakeups; atomically release-and-block, then reacquire before returning.
- **Semaphores**: Define process/thread sharing, maximum counter, overflow/underflow checks, post-from-interrupt constraints, and destruction rules.
- **Spinlocks and IRQ Locks**: Use only for tiny bounded critical sections; prohibit sleeping, memory allocation, or scheduler calls while held; specify local IRQ/preemption save/restore state.
- **Read/Write Locks**: Document writer starvation policy and upgrade/downgrade behavior; use only where measured read-heavy workloads justify them.
- **Atomics**: Use explicit memory ordering (`Acquire`, `Release`, `AcqRel`, `SeqCst`) with a written happens-before rationale; avoid unexplained `Relaxed`.
- **Shared Memory / Futex**: Tie wait keys to stable physical page mappings and offset, not raw virtual user pointers; handle unmapping, process exit, and object teardown cleanly.
- **IPC Queues & eventfd**: Reuse common wait/wakeup semantics; define close/EOF, queue saturation, broadcast, timeout, and cancellation behavior consistently.
- **Scheduler Priority**: Document priority inversion risks if priority inheritance is not active; introduce PI only after task owner tracking is supported in the scheduler.
- **Debugging**: Maintain a debug lock-dependency graph (`lockdep`); record lock class, context, and acquisition callsite without exposing sensitive user memory.

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

**Merge Gates**: Requires 1 maintainer approval for docs/isolated tests, and 2 qualified reviews for atomic algorithms, wait queues, IRQ locks, process-shared objects, or syscall/UAPI changes.

---

## 8. Completion Gates

1. **Prototype**: Data structures and lock models pass unit tests, but are not connected to real booted tasks.
2. **Integrated**: Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
3. **Supported**: Repeatable stress tests cover lifecycle, ownership, interrupt context, process sharing where claimed, starvation/priority behavior, weak memory ordering, and documented error semantics.
