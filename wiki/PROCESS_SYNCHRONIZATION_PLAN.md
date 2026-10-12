# SigmaOS Process Synchronization Master Architecture, Support Matrix, and Implementation Plan

## Executive Summary & System Inventory

The SigmaOS process synchronization subsystem provides thread-safe and process-safe execution primitives across kernel and userspace contexts. While source code modules exist for several primitives (`src/kernel/wait_queue.rs`, `src/kernel/sigma_spinlock.rs`, `src/kernel/semaphore.rs`, `src/kernel/flock.rs`, `src/kernel/pipe.rs`, `src/kernel/eventfd.rs`, `src/kernel/futex.rs`, `src/ipc/futex.rs`, and `src/kernel/lockdep.rs`), presence of source files and isolated unit tests does not constitute full runtime integration or Ring 3 process safety.

This master plan establishes the formal architecture, design invariants, 17-PR implementation sequence, maintenance cadences, component rules, PR review template, and merge gates required to evolve SigmaOS process synchronization from isolated prototypes into a production-grade, integrated system.

### Inventory and Runtime Status Matrix

| Component | Source Files | Inventory Status | Runtime Pathway & Integration Gate |
| :--- | :--- | :--- | :--- |
| **Wait Queue** | `src/kernel/wait_queue.rs` | Prototype | In-memory wait list; requires EEVDF scheduler task state (`Blocked`) integration. |
| **Spinlocks & RW Locks** | `src/kernel/sigma_spinlock.rs` | Working | Bounded ticket spinlock & read-write spinlock; requires per-CPU IRQ-save validation. |
| **Counting Semaphore** | `src/kernel/semaphore.rs` | Prototype | Internal semaphore counter; requires process-shared unnamed memory support. |
| **Fast Userspace Mutex** | `src/kernel/futex.rs`, `src/ipc/futex.rs` | Prototype | Hash-bucket wait queues & PI state; requires user page table resolution & syscall dispatch. |
| **Advisory File Lock** | `src/kernel/flock.rs` | Prototype | Byte-range/flock table; requires VFS file descriptor table integration. |
| **IPC Primitives** | `src/kernel/pipe.rs`, `src/kernel/eventfd.rs` | Prototype | Pipe buffers and event counters; requires readiness polling and epoll integration. |
| **Lockdep Diagnostics** | `src/kernel/lockdep.rs` | Prototype | Lock class registry and circular dependency graph; requires kernel lock instrumentation. |

---

## Architectural Progression & Synchronization Domains

SigmaOS structures process synchronization into four explicit, non-overlapping domains:

```
+-----------------------------------------------------------------------------------+
|                        Domain 4: IPC Cross-Channel Synchronization                 |
|                      Pipes, FIFOs, UNIX Sockets, eventfd, MQueue                  |
+-----------------------------------------------------------------------------------+
|                     Domain 3: Process-Shared Synchronization                       |
|                 Shared Memory Mutexes, Unnamed Semaphores, Futex ABIs             |
+-----------------------------------------------------------------------------------+
|                       Domain 2: Process-Local Synchronization                      |
|                  Userspace Mutexes, Condition Variables, Barriers                 |
+-----------------------------------------------------------------------------------+
|                       Domain 1: Kernel-Internal Synchronization                    |
|                 Spinlocks, IRQ Locks, Sleeping Mutexes, Wait Queues               |
+-----------------------------------------------------------------------------------+
```

### Domain Definitions

1. **Kernel-Internal (Domain 1):** Synchronizes kernel kthreads, IRQ handlers, and system call execution. Must enforce strict sleeping vs. spinning context rules.
2. **Process-Local (Domain 2):** Synchronizes user threads operating within a single virtual address space. Managed via fast-path user atomic operations backed by kernel wait/wake system calls.
3. **Process-Shared (Domain 3):** Synchronizes independent processes sharing physical memory pages (`shm` / anonymous memory). Wait keys are tied to physical page identities and page offsets rather than user virtual addresses.
4. **IPC Cross-Channel (Domain 4):** Synchronizes asynchronous or blocking read/write operations across file descriptors and IPC endpoints using unified readiness notifications.

---

## Core Design Rules and Invariants

1. **Name the Synchronization Domain:** Every primitive must explicitly state whether it operates on kernel threads, process-local user threads, process-shared memory, or IPC channels.
2. **Separate Lock from Event:** Use mutexes exclusively for ownership-based mutual exclusion. Use condition variables, completions, or wait queues for waiting on state changes. Semaphores must not be used as generic locking mechanisms.
3. **Never Sleep in Atomic Context:** Prohibit sleeping locks (mutexes, semaphores, wait queues) in hard-interrupt, non-preemptible, or spinlock-held contexts. Keep IRQ-safe spinlocks strictly bounded and non-allocating.
4. **Specify Ownership and Failure Semantics:** Define recursive, error-checking, or robust behaviors. Enforce owner-only unlock rules. Handle task termination while locks are held, returning explicit owner-dead errors (`EOWNERDEAD`) to successors.
5. **Wait on Predicates, Not Notifications:** Condition variable waiters must loop and re-check their shared predicate upon wakeup. The wait operation must atomically release the associated mutex and block, reacquiring the mutex prior to returning.
6. **Prevent Lost Wakeups:** State checks and waiter registrations must be atomic relative to notification, timeout, signal delivery, and task cancellation.
7. **Use Explicit Memory Ordering:** Atomic variables must have explicit Rust memory orderings (`Acquire`, `Release`, `AcqRel`, or `SeqCst`) with documented synchronization invariants. `Relaxed` ordering is strictly forbidden for publication or ownership handoffs.
8. **Address Priority Inversion and Starvation:** Implement fair queueing for contended locks. Define Priority Inheritance (PI) protocol support for mutexes once the scheduler task state integration is active.
9. **Treat User-Memory Objects as Untrusted:** Validate all user pointers, shared-memory handles, object permissions, and virtual address ranges at system call boundaries. User processes must never manipulate raw kernel wait queue pointers.
10. **Test Races Systematically:** Mandate deterministic state-machine tests, Loom model checking for fast-path atomics, stress testing under CPU contention, and multi-vCPU QEMU verification.
11. **Prevent Deadlocks via Lockdep & Strict Hierarchy:** Maintain a dynamic directed acyclic graph (DAG) of lock acquisitions. Debug builds must flag out-of-order acquisitions, recursive lock errors, and sleeping-in-atomic violations.

---

## Proposed 17-PR Sequence Roadmap

```
Phase 1: Kernel Primitives & Verification (PRs 1-8)
   │
   ├─► PR 1: Primitive Inventory & Domain Specification
   ├─► PR 2: Synchronization Contracts & Lock Context Matrix
   ├─► PR 3: Concurrency Model & Invariant Harness (Loom Integration)
   ├─► PR 4: Scheduler-Integrated Wait Queue & Wakeup Primitive
   ├─► PR 5: Kernel Sleeping Mutex with Ownership & Debug Checks
   ├─► PR 6: Bounded Ticket Spinlock & IRQ-Safe Variants
   ├─► PR 7: Completion & One-Shot Event Primitive
   └─► PR 8: Counting Semaphore with Bounded Counter
   │
Phase 2: Userspace & Advanced Synchronization (PRs 9-14)
   │
   ├─► PR 9: Condition Variable with Atomic Unlock-and-Wait
   ├─► PR 10: Read-Write Lock with Starvation & Preference Policies
   ├─► PR 11: Robust Process-Local Mutex Recovery (EOWNERDEAD)
   ├─► PR 12: Futex Wait/Wake Syscall for Userspace Fast Paths
   ├─► PR 13: Process-Shared Synchronization via Shared Memory
   └─► PR 14: Interprocess Message & Event Notifications
   │
Phase 3: Diagnostics & Production Certification (PRs 15-17)
   │
   ├─► PR 15: Lock-Order Checker & Runtime Wait Diagnostics
   ├─► PR 16: Race Detector (KCSAN) & Stress Test Matrix
   └─► PR 17: Public API Documentation, Context Matrix & Compatibility Status
```

### Implementation Matrix & Acceptance Criteria

| PR | Title | Scope | Acceptance Criteria |
| :--- | :--- | :--- | :--- |
| **PR 1** | `docs(sync): inventory primitives and define subsystem boundaries` | Audit scheduler wait queues, spinlocks, semaphores, flock, pipes, eventfd, futexes, and lockdep. | Inventory clearly distinguishes kernel-internal, process-local, process-shared, and modeled code; no primitive is marked supported without runtime proof. |
| **PR 2** | `docs(sync): define synchronization contracts and lock-context rules` | Publish primitive selection guide, lock ordering hierarchy, blocking context matrix, and memory-ordering rules. | Every primitive documents ownership rules, valid execution contexts, wake policy, timeout/cancellation semantics, and destruction contracts. |
| **PR 3** | `test(sync): add concurrency model and invariant harness` | Host-side deterministic tests and Loom models for fast-path atomics, waiter registration, wakeup, and destruction. | Models use Loom-aware atomics/locks; bounds are explicit; lost wakeup and ABA reuse scenarios are covered under model checking. |
| **PR 4** | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Implement base wait queue connected to EEVDF scheduler task states (`Blocked`/`Ready`), timeouts, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; predicate check to sleep transition is race-free; task exit cleans wait references safely. |
| **PR 5** | `feat(sync): add kernel mutex with ownership and debug checks` | Sleeping mutex for task context; ownership tracking, try-lock, recursion policy, and owner-death detection. | Contended lock blocks task context; non-owner unlock fails; usage in interrupt/atomic context is rejected; owner and waiter cleanup survive task exit. |
| **PR 6** | `feat(sync): add bounded spinlock and IRQ-safe variants` | Ticket spinlock and `irqsave`/`irqrestore` primitives for short CPU-local and SMP critical sections. | Lock/unlock memory barriers are verified; debug mode detects recursive acquisition and invalid context; no heap allocation or blocking while held. |
| **PR 7** | `feat(sync): add completion and one-shot event primitive` | Completion/event primitive for "wait until operation completes" pattern; integrated with wait queues. | Completion prior to wait is retained; multiple waiters wake correctly according to broadcast policy; repeated complete/reinit behavior is defined and tested. |
| **PR 8** | `feat(sync): add counting semaphore with bounded counter` | Wait, post, try-wait, and timed-wait operations with max-count boundaries and waiter queues. | Counter never underflows or exceeds maximum; waiter wake transfers token availability; post-from-interrupt rules are enforced; process sharing is not claimed yet. |
| **PR 9** | `feat(sync): add condition variable with atomic unlock-and-wait` | Condition wait/signal/broadcast tied to a mutex; monotonic timed-wait support. | Mutex is released atomically with waiter queue registration; reacquired before return; callers loop on predicates; spurious wakeups and races tested. |
| **PR 10** | `feat(sync): add read-write lock with starvation policy` | Reader/writer lock with configurable reader-preference or writer-fairness rules. | Unit tests cover concurrent readers, exclusive writer, downgrade/upgrade policies, writer starvation bounds, and task exit cleanup. |
| **PR 11** | `feat(sync): add robust process-local mutex recovery` | Owner-death detection for userspace mutexes (`EOWNERDEAD` / `ENOTRECOVERABLE`). | Successor thread receives owner-dead notification; recovery and state consistency semantics match POSIX specification. |
| **PR 12** | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | Word-based compare-and-block wait/wake system call interface for Ring 3 fast-path lock implementations. | Syscall blocks only if current value matches expected value; wake targets correct waiters; invalid user memory pointers are handled gracefully. |
| **PR 13** | `feat(sync): integrate process-shared synchronization with shared memory` | Tie futex keys and unnamed semaphores to physical page offset and shared memory mapping identity. | Independent processes synchronize across shared memory mappings; namespace boundaries and permissions enforced; object teardown handles active waiters safely. |
| **PR 14** | `feat(sync): add interprocess message and event synchronization` | Integrate message queues, pipes, and eventfd counters with wait queues and readiness polling. | Readiness states, queue saturation, peer process exit, timeout, and signal interruptions tested without busy-wait loops. |
| **PR 15** | `debug(sync): add lock-order checker and wait diagnostics` | Dynamic lockdep graph, acquisition tracking, held-lock assertions, and deadlock reporting. | Debug mode catches out-of-order acquisitions, recursive misuse, sleeping-in-atomic errors, and wait-while-held violations. |
| **PR 16** | `test(sync): add race detector and stress configuration` | Data-race instrumentation and high-contention lock torture stress tests across single-core and SMP QEMU. | Contention tests run under multi-vCPU QEMU; reports log CPU/task context and memory access traces; limitations documented. |
| **PR 17** | `docs(sync): publish API, context matrix, and compatibility status` | Complete documentation of kernel APIs, UAPI system calls, error codes, and compatibility status matrix. | Every public API references implementation and test suites; POSIX, Linux, and FreeBSD compatibility nuances explicitly documented. |

### Sequential Priority Gate Rule

PRs 1 through 8 form the mandatory Phase 1 foundation. Implementation of blocking mutexes and semaphores MUST be gated on the EEVDF scheduler's real wait/wakeup task state pathway (PR 4). Condition variables, read/write locks, and process-shared futex syscalls MUST NOT be claimed as supported until Phase 1 is fully integrated and certified under QEMU.

---

## Periodic Maintenance PR Cadence

To prevent regressions and preserve architectural integrity, the process synchronization subsystem adheres to the following maintenance cadences:

| Cadence | Task Description | Required Artifact / Evidence |
| :--- | :--- | :--- |
| **Every PR** | `ci(sync): run primitive-specific tests` | Passing build, invariant unit tests, Loom models, and QEMU test log. |
| **Weekly** | `maint(sync): triage deadlocks and stalls` | Reproducer, task trace, lockdep report, and tracked issue resolution. |
| **Weekly** | `test(sync): review wakeup & timeout regressions` | Lost-wakeup, timeout drift, and signal delivery performance report. |
| **Monthly** | `audit(sync): review lock ordering & context use` | Updated lock dependency graph; audit of sleeping locks in atomic contexts. |
| **Monthly** | `test(sync): run contention stress matrix` | Multi-threaded lock contention, owner death, and fairness metrics report. |
| **Monthly Security** | `security(sync): review shared-object lifecycle` | Audit of user pointer validation, physical page keys, and resource limits. |
| **Quarterly** | `maint(sync): review atomic orderings` | Memory ordering audit verifying acquire/release rationale across architectures. |
| **Quarterly** | `test(sync): SMP and weak-ordering validation` | Multi-vCPU QEMU stress tests validating memory barrier correctness. |
| **Release Candidate** | `release(sync): certify synchronization claims` | Certified capability matrix, complete test logs, and documented limitations. |
| **After Incident** | `fix(sync): prevent recurrence of race/deadlock` | Minimal reproducer, root-cause analysis, regression test, and patch. |

---

## Component-Specific Rules

1. **Scheduler / Wait Queues:** The waiter task MUST be registered in the wait queue before releasing the predicate lock. Sleep and wake transitions MUST be atomic from the caller's perspective. Waiters MUST be safely removed on timeout, signal interruption, or task exit.
2. **Mutexes:** Strict owner tracking MUST be enforced. Re-locking by the owner MUST fail or behave according to explicit recursion policy. Non-owner unlock attempts MUST panic or return errors in debug mode. Critical sections MUST be kept minimal.
3. **Condition Variables:** MUST always be paired with an explicit mutex and predicate loop. Spurious wakeups MUST be handled gracefully by callers. The wait call MUST atomically release the mutex and block, reacquiring the mutex prior to return.
4. **Semaphores:** Define maximum count, overflow, underflow, and post-from-interrupt rules. Counting semaphores DO NOT track owner identity and MUST NOT be used for priority inheritance.
5. **Spinlocks & IRQ Locks:** Used strictly for tiny, non-allocating, bounded critical sections. Yielding, sleeping, or invoking scheduler/allocator code while holding a spinlock is strictly prohibited.
6. **Read/Write Locks:** Writer starvation MUST be bounded. Upgrade/downgrade rules MUST be explicitly documented. Read locks MUST NOT be held across sleeping calls.
7. **Atomics:** Use the weakest memory ordering satisfying a written happens-before proof. Document publication, ownership handoff, and reclamation invariants.
8. **Shared Memory / Futex:** Futex keys MUST be derived from stable physical page identities and page offsets, NEVER raw user virtual addresses alone. Unmapping, remapping, or process exit MUST NOT corrupt kernel wait buckets.
9. **IPC Queues and eventfd:** Reuse standard wait queue infrastructure. Readiness polling (epoll/kqueue) MUST integrate with wait queue wakeup notifications.
10. **Scheduler Priority:** Priority inheritance (PI) state MUST be maintained if mutex waiters have higher priority than the lock owner. If PI is unfulfilled, priority inversion risks MUST be documented.
11. **Diagnostics (Lockdep):** Maintain dynamic lock dependency graphs and ownership assertions. Diagnostic traces MUST NOT leak user memory contents or sensitive kernel pointers.

---

## PR Review Template and Merge Gates

All pull requests modifying the synchronization subsystem MUST complete the following review template:

```markdown
## Summary
What synchronization behavior or correctness gap does this PR address?

## Primitive and Domain
Primitive: Mutex | Spinlock | Semaphore | Condition | Completion | Wait Queue | IPC/Futex
Domain: Kernel-Only | Process-Local | Process-Shared | IPC-Cross-Channel

## Contract
Ownership rules, blocking context rules, memory ordering, fairness policy, timeout/cancellation,
destruction rules, error behavior, and supported CPU architectures.

## Concurrency Hazards
Lost wakeup, out-of-order locking, priority inversion, reentrancy, object reuse,
task/process exit, and interrupt context interaction.

## Runtime Evidence
- [ ] Unit Tests / Model (`rustc --test`)
- [ ] Loom Concurrency Model (`cargo test --test loom`)
- [ ] QEMU Single vCPU Execution
- [ ] QEMU Multi vCPU SMP Execution
- [ ] Physical Hardware Validation

## Tests
Normal path, contention stress, timeout handling, owner exit, invalid context rejection,
teardown cleanup, and lockdep diagnostic checks.

## Status and Documentation
Status: Proposed | Prototype | Integrated | Supported
Links: API Docs, Feature Status, Test Evidence Logs.

## Checklist
- [ ] Lock/context rules documented and enforced
- [ ] Wait registration cannot race with wakeup
- [ ] Memory orderings have a written rationale
- [ ] Failure/timeout/exit paths release waiters and references
- [ ] Tests cover race and lifecycle cases
- [ ] ABI/layout changes are reviewed and documented
- [ ] Required CI passes; review discussions resolved
```

### Review Gate Requirements

- **Isolated Documentation / Tests:** Requires at least ONE maintainer approval.
- **Atomic Algorithms, Scheduler Wait Queues, IRQ Locks, Process-Shared Objects, or Syscalls:** Requires at least TWO qualified subsystem maintainer approvals and 100% passing automated checks.

---

## Completion Gates

| Level | Definition and Required Evidence |
| :--- | :--- |
| **Prototype** | Data structures, fast paths, or lock models pass isolated unit tests on host (`rustc --test`); not yet integrated into booted kernel scheduler or syscall dispatch paths. |
| **Integrated** | Real booted SigmaOS tasks can contend, block, wake, time out, and exit cleanly using the primitive without lost wakeups, deadlock hangs, or resource leaks under QEMU. |
| **Supported** | Repeatable stress tests cover full lifecycle, ownership, interrupt contexts, process sharing, starvation/fairness rules, multi-vCPU memory ordering, and complete UAPI compatibility. |

*Crucial Note:* The presence of synchronization source files or isolated unit tests DOES NOT qualify the subsystem as Supported. The decisive integration gate is a booted Ring 3 process test exercising the real scheduler, virtual memory, and syscall path backed by dynamic lockdep validation and multi-vCPU stress testing.
