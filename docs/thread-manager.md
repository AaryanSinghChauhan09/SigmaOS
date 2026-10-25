# SigmaOS Thread Manager Architecture & Subsystem Contract

The **SigmaOS Thread Manager** provides the canonical thread lifecycle abstraction and concurrency control model for the kernel.
It unifies saved CPU context, stack limits with guard zones, scheduling parameters, wait channels, signal masks, file descriptor table references, capability contexts, and join/detach mechanics into a single, cohesive `Thread` object (`src/kernel/thread.rs`).

---

## Phase 1: Define the Contract

### Thread States

SigmaOS thread execution is governed by seven explicit lifecycle states:

| State | Description |
|---|---|
| `New` | Thread object instantiated, ID allocated, but not yet enqueued into a CPU run queue. |
| `Ready` | Thread is runnable and waiting in a run queue for CPU slice allocation. |
| `Running` | Thread is actively executing on a CPU core. |
| `Blocked` | Thread is sleeping on a wait channel or waiting for an I/O event / timer. |
| `Suspended` | Thread execution is paused by a signal or control operation (`suspend()`). |
| `Zombie` | Thread execution has terminated (`exit()`), awaiting join/reaping by owner or parent. |
| `Terminated` | Thread resources are fully reclaimed; object marked for removal from thread table. |

#### Legal State Transition Matrix

```
[New] ------> [Ready] <=======> [Running]
  |             |                   |
  v             v                   v
[Terminated] [Suspended] <=====> [Blocked]
  ^             |                   |
  |             v                   v
  +---------- [Zombie] <------------+
```

Allowed transitions:
- `New` -> `Ready`, `Terminated`
- `Ready` -> `Running`, `Suspended`, `Zombie`
- `Running` -> `Ready`, `Blocked`, `Suspended`, `Zombie`
- `Blocked` -> `Ready`, `Suspended`, `Zombie`
- `Suspended` -> `Ready`, `Zombie`
- `Zombie` -> `Terminated`

All invalid transitions (e.g. `New` -> `Running`, `Blocked` -> `Running`, or self-transitions) are rejected at runtime with an `Err(&str)`.

---

### Thread Ownership Categories

1. **Kernel Thread (`ThreadOwnership::Kernel`)**: Pure background thread executing in kernel address space without a user page table or user stack.
2. **User Thread (`ThreadOwnership::User`)**: Thread executing user code, bound to a process, with distinct kernel and user stack metadata.
3. **Process Primary Thread (`ThreadOwnership::Process`)**: Lead thread of a process group; process termination triggers cascading cleanup.
4. **Detached Thread (`ThreadOwnership::Detached`)**: Independent thread automatically reclaimed on exit (`Zombie` -> `Terminated`) without requiring an explicit `join()`.

---

### Guarantees & Safety Mechanisms

- **Preemption Model**: Support for voluntary yielding (`sched_yield`), timer-driven quantum expiry, and signal interruption.
- **Priority Ranges**: Bounded static priority between `0` (highest priority / real-time) and `139` (lowest priority / idle). Default normal priority is `120`.
- **CPU Affinity**: 64-bit affinity bitmask (`CpuAffinity`) controlling eligible CPU core execution sets.
- **Stack Limits & Guard Zones**: `StackMeta` defines base address, stack size, and guard zones to detect and prevent kernel stack overflows.
- **Lost-Wakeup Prevention**: Wait-channel wakeups (`wake_wait_channel`) and timeout operations (`timeout_wait_channel`) mutate thread state atomically under `ThreadManager` lock discipline.
- **Lock-Order Tracker**: Dynamic acquisition tracking (`locks_held`) to detect lock order inversions and prevent deadlocks.

---

### Non-Goals

- **No Unsupported POSIX Claims**: POSIX `pthread` compliance is not claimed until full syscall dispatcher integration and libc test suites pass.
- **No Direct Hardware Memory Context Switch in Phase 1**: Low-level register context switching on physical hardware is gated by QEMU boot validation.

---

## Phase 2: Core Thread Object

Implemented in `src/kernel/thread.rs`:

```rust
pub struct Thread {
    pub tid: ThreadId,
    pub pid: Pid,
    pub name: String,
    pub ownership: ThreadOwnership,
    pub state: ThreadState,
    pub cpu_context: CpuContext,
    pub kernel_stack: StackMeta,
    pub user_stack: Option<StackMeta>,
    pub priority: u8,
    pub nice: i8,
    pub sched_class: SchedulingClass,
    pub affinity: CpuAffinity,
    pub wait_channel: Option<WaitChannelId>,
    pub signal_mask: u64,
    pub pending_signals: u64,
    pub fd_table_ref: Option<u64>,
    pub rlimits_ref: Option<u64>,
    pub capability_context: u64,
    pub exit_status: Option<i32>,
    pub is_detached: bool,
    pub locks_held: Vec<u64>,
}
```

---

## Phase 3: Scheduler Integration Roadmap

1. **Run Queue**: Per-CPU priority queues (`Ready` state) with lockless or fine-grained spinlock enqueue/dequeue hooks.
2. **Context Switch**: Architecture-specific assembly stubs (`CpuContext`) saving/restoring register state and switching stack pointers.
3. **Preemption**: Timer interrupt handler preempts running thread when slice expires or higher-priority thread becomes ready.
4. **Wait Channels**: `WaitChannelId` indexing for blocking queues (mutexes, semaphores, condition variables, I/O wait queues).
5. **Reaping & Exit**: Automated thread cleanup (`reap_terminated`), orphan reparenting, and process exit coordination.
6. **Observability**: Direct inspection of thread states, CPU time, context switches, and lock ordering via kernel diagnostics.

---

## Phase 4: POSIX-Facing Behavior Roadmap

- Thread lifecycle: `pthread_create`, `pthread_exit`, `pthread_join`, `pthread_detach`.
- Timers & Sleep: `sched_yield`, `nanosleep`, timerfd integration.
- Synchronization Primitives: `pthread_mutex`, `pthread_cond`, `pthread_rwlock`, semaphores.
- Signal Delivery: Direct signal mask filtering (`signal_mask`), pending signals (`pending_signals`), and targeted thread delivery.
- Thread-Local Storage (TLS): CPU architecture register setup (`FS`/`GS` base on x86_64).

---

## Suggested First 10 PRs

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

## Verification & Acceptance Criteria

Every thread manager change must pass the following verification block:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```
