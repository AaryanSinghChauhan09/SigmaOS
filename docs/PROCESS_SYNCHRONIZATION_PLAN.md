# SigmaOS Process Synchronization Architecture, Support Matrix, and PR Proposal Plan

## Executive Overview and Architectural Vision

SigmaOS process synchronization provides thread-safe and process-safe concurrency primitives across kernel execution, process-local user threads, process-shared IPC channels, and user fast-path futex operations.

The process synchronization subsystem follows a strict 5-layer progression architecture:
1. **Kernel wait/wakeup primitives**: Base wait queues (`WaitQueue`, `WaitQueueEntry`) for task state blocking, scheduling, and wakeups.
2. **Kernel locks and completions**: Ticket spinlocks (`SigmaSpinlock`), read-write spinlocks (`SigmaRwSpinlock`), sleeping mutexes, and completions.
3. **Userspace synchronization APIs**: POSIX-compatible user mutexes, condition variables, and semaphores (`Semaphore`, `SemaphoreManager`).
4. **Process-shared IPC synchronization**: Fast userspace mutexes (`FutexQueue`, `FutexManager`), advisory file locks (`FileLockManager`), pipes (`Pipe`), and event file descriptors (`EventFd`).
5. **Concurrency diagnostics**: Runtime lock-dependency graph checker (`lockdep`) and deadlock analysis tools.

---

## Core Design Rules

1. **Name the synchronization domain**: Each primitive explicitly declares its synchronization scope (kernel-internal, thread-local within process, process-shared memory, or IPC channel).
2. **Separate lock from event**: Mutexes handle ownership-based exclusion; condition variables, wait queues, and completions handle waiting on state changes. Semaphores are not used as mutual exclusion locks.
3. **Never sleep in atomic context**: Sleeping locks or wait queue blocks are prohibited in hard-IRQ, non-preemptible, or spinlock-held contexts.
4. **Specify ownership and failure semantics**: Mutexes require owner-only unlocks. Unlocking by a non-owner, recursive re-locking without explicit recursive configuration, or destroying primitives with active waiters triggers explicit error returns or debug assertions.
5. **Wait on predicates, not notifications**: Callers must re-evaluate state predicates in a loop upon waking to handle spurious wakeups. `cond_wait` atomically releases the associated mutex, blocks, and re-acquires the mutex before returning.
6. **Prevent lost wakeups**: Predicate state checks and waiter registration are atomic relative to concurrent notifications, timeouts, signals, or thread cancellations.
7. **Use explicit memory ordering**: Atomic fast paths explicitly document acquire/release and compare-exchange memory ordering invariants. `Ordering::Relaxed` is prohibited where synchronization relationships exist.
8. **Address priority inversion and starvation**: Mutexes and read-write locks employ defined fairness policies. Priority inheritance is integrated once scheduler priority modification and owner tracking are bound. Counting semaphores do not track individual owners and cannot provide owner-based priority inheritance.
9. **Treat user-memory objects as untrusted**: User-provided handles, shared memory pointers, and futex keys are validated at the syscall boundary. Direct manipulation of kernel waiter structures by user code is prohibited.
10. **Test races systematically**: Deterministic state-machine unit tests, model-checking where appropriate, and multi-threaded stress tests validate concurrent interleavings.

---

## 17-PR Sequence Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(sync): inventory primitives and define subsystem boundaries` | Audit wait queues, spinlocks, semaphores, flock, pipes, eventfd, futex, and lockdep. | Inventory distinguishes kernel-internal, process-local, process-shared, and modeled code; no module claimed runtime-supported without evidence. |
| 2 | `docs(sync): define synchronization contracts and lock-context rules` | Publish selection guide, lock hierarchy, blocking-context matrix, memory orderings, and lifecycle contracts. | Primitives document owner, valid context, wake policy, timeout/cancellation, and destruction behavior. |
| 3 | `test(sync): add concurrency model and invariant harness` | Host-side deterministic tests and atomic state models for fast paths, waiter registration, wakeups, and timeouts. | Models cover lost wakeups, ABA/reuse cases, and atomic fast paths. |
| 4 | `feat(sync): add scheduler-integrated wait queue and wakeup primitive` | Base `WaitQueue` primitive connected to task states, scheduler enqueue/dequeue, timeouts, and cancellation. | QEMU tests prove block/wake and timeout behavior with real tasks; lost-wakeup free predicate check; task exit cleans references. |
| 5 | `feat(sync): add kernel mutex with ownership and debug checks` | Sleeping kernel mutex for task context with ownership tracking, try-lock, and non-owner unlock checks. | Contended lock sleeps; non-owner unlock fails in debug mode; rejected in atomic context; clean teardown on task exit. |
| 6 | `feat(sync): add bounded spinlock and IRQ-safe variants` | Minimal `SigmaSpinlock` ticket locks and IRQ-save variants for short SMP critical sections. | Lock/unlock memory barriers tested; debug mode detects recursive lock and invalid context; zero allocations/blocking while held. |
| 7 | `feat(sync): add completion and one-shot event primitive` | Completion mechanism for operation state signaling integrated with wait queues and timeouts. | Pre-signal state retained; multi-waiter signaling verified; clear re-initialization and teardown semantics. |
| 8 | `feat(sync): add counting semaphore with bounded counter` | Bounded counting `Semaphore` supporting wait, post, try-wait, and timed-wait. | Counter bounded by maximum limit; wakeup transfers availability correctly; post-from-IRQ rules explicit. |
| 9 | `feat(sync): add condition variable with atomic unlock-and-wait` | Condition variable tied to mutex with atomic unlock-and-wait and monotonic clock timed waits. | Mutex released atomically on wait registration; re-acquired before return; callers re-check predicate; spurious wakeup safe. |
| 10 | `feat(sync): add read-write lock with starvation policy` | Reader/writer lock (`SigmaRwSpinlock`) with fair writer preference and recursion policy. | Concurrent readers and exclusive writer tested; writer starvation bounded; exit/cancellation cleanup verified. |
| 11 | `feat(sync): add robust process-local mutex recovery` | POSIX robust mutex owner-death handling and state recovery mechanisms. | Successor notified of owner-dead state; unrecoverable/consistent transition semantics documented and tested. |
| 12 | `feat(sync): add futex-style wait/wake syscall for userspace fast paths` | User-space word-based compare-and-block `FutexQueue` and `FutexManager` wait/wake syscall dispatch. | Wait blocks only if user memory matches expectation; wake targets correct physical page key; invalid pointers rejected cleanly. |
| 13 | `feat(sync): integrate process-shared synchronization with shared memory` | Process-shared semaphores and mutexes mapped into shared memory with handle lifecycle management. | Processes synchronize through shared physical page mapping; permissions enforced; destruction prevented while waiters active. |
| 14 | `feat(sync): add interprocess message and event synchronization` | Connect `Pipe` and `EventFd` primitives to unified wait/wakeup, readiness polling, and cancellation contracts. | Read/write readiness, buffer full/empty, close, peer exit, and signal handling tested without busy-wait loops. |
| 15 | `debug(sync): add lock-order checker and wait diagnostics` | Lock-dependency graph (`lockdep`) tracking lock classes, acquisition contexts, and deadlock detection. | Catches lock-order inversion, recursive misuse, and sleeping in atomic context; diagnostic reports bounded. |
| 16 | `test(sync): add race detector and stress configuration` | Multi-threaded lock stress tests, producer-consumer benchmarks, and contention profiling. | Stress tests pass on single-core and SMP QEMU; race reports capture task/CPU context. |
| 17 | `docs(sync): publish API, context matrix, and compatibility status` | Publish comprehensive API specification, UAPI headers, POSIX/Linux/BSD comparison, and capability matrix. | API documented with test cross-references; compatibility status accurately matches runtime evidence. |

---

## Periodic Maintenance Cadence

- **Every PR**: `ci(sync)` - Run primitive unit tests, verify formatting/builds, and validate invariant assertions.
- **Weekly**: `maint(sync)` - Triage deadlocks, stalls, and lock contention reports; review wakeup/timeout test results.
- **Monthly**: `audit(sync)` - Audit lock ordering hierarchy, atomic memory orderings, and sleeping-in-atomic context risks.
- **Quarterly**: `maint(sync)` - Perform architecture memory-ordering audits across x86_64 and AArch64; validate weak-ordering assumptions under SMP.
- **Release Candidate**: `release(sync)` - Certify synchronization support claims, test coverage matrix, and ABI compatibility.
- **After Incident**: `fix(sync)` - Construct minimal regression reproducers, perform root-cause analysis, and deploy fixes.

---

## Component-Specific Rules

- **Scheduler/Wait Queues**: Register waiter before releasing predicate lock; atomic sleep-and-wake transitions from caller perspective; remove waiters on timeout or exit.
- **Mutexes**: Strict owner-only unlock rules; non-owner unlocks or double-locks fail with explicit error; critical sections kept minimal.
- **Condition Variables**: Always paired with a mutex; caller re-checks predicate in a loop; atomic release-and-block with re-acquisition on wake.
- **Semaphores**: Explicit count limits; bounded counter overflow/underflow protection; process-shared semantics require shared memory mapping.
- **Spinlocks and IRQ Locks**: Short critical sections; no sleeping, heap allocations, or blocking while held; explicit per-CPU IRQ state handling.
- **Read/Write Locks**: Starvation protection for pending writers; explicit upgrade/downgrade semantics.
- **Atomics**: Explicit memory ordering (`Acquire`, `Release`, `AcqRel`, `SeqCst`) with written rationale for every atomic variable.
- **Shared Memory / Futex**: Futex keys tied to physical page identity and offset; safe against unmapping, remapping, or process exit.
- **IPC Queues and EventFd**: Reuses core `WaitQueue` mechanics; clear non-blocking `EAGAIN` and readiness signaling via poll/epoll.
- **Scheduler Priority**: Document priority inversion risks; integrate priority inheritance only after owner tracking and scheduler priority hooks are verified.
- **Debugging**: Enable runtime lock-dependency (`lockdep`) graph checking in debug builds to catch potential deadlocks prior to runtime exposure.

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

### Maintainer Approval Rules
- **Isolated tests/docs**: Requires 1 maintainer approval.
- **Atomic algorithms, scheduler wait queues, IRQ-safe locks, process-shared objects, or syscall/UAPI changes**: Requires 2 qualified maintainer reviews.

---

## Subsystem Completion Gates

- **Prototype**: Data structures or lock models pass unit tests, but are not connected to real tasks.
- **Integrated**: Booted SigmaOS tasks can contend, block, wake, time out, and exit using the primitive without lost wakeups or resource leaks.
- **Supported**: Repeatable tests cover lifecycle, ownership, interrupt context, process sharing where claimed, starvation/priority behavior, architecture-specific memory ordering, and documented error semantics.
