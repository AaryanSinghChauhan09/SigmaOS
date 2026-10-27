# SigmaOS Thread Manager Architecture & Specification

## Executive Summary

The SigmaOS Thread Manager provides the canonical `Thread` abstraction, lifecycle state machine, stack metadata guards, CPU context management, wait-channel wakeups, and lock-order tracking across kernel and user execution threads.

This document formally specifies the contracts, state machine, ownership models, safety guarantees, and PR sequence for the thread subsystem in accordance with SigmaOS architecture guidelines.

---

## Phase 1: Subsystem Contract & Guarantees

### 1. Thread Lifecycle States

The thread state machine strictly enforces legal transitions between 7 explicit states:

```text
               +-----+
               | New |
               +--+--+
                  |
                  v
              +-------+ <----------------+
              | Ready |                  |
              +---+---+                  |
                  |                      |
                  v                      |
             +---------+                 |
             | Running +-----------------+
             +----+----+
                  |
        +---------+---------+
        |                   |
        v                   v
   +---------+        +-----------+
   | Blocked |        | Suspended |
   +----+----+        +-----+-----+
        |                   |
        +---------+---------+
                  |
                  v
              +--------+
              | Zombie |
              +---+----+
                  |
                  v
            +------------+
            | Terminated |
            +------------+
```

State descriptions and transition rules:

- **`New`**: Thread object allocated and initialized; not yet enqueued. Transition to `Ready` on enqueue, or `Terminated` if creation fails.
- **`Ready`**: Thread is runnable and queued in a CPU run queue awaiting scheduling. Transition to `Running` when selected by scheduler, `Suspended` on pause, or `Zombie` on kill.
- **`Running`**: Thread actively executes on a CPU core. Transition to `Ready` on yield/preemption, `Blocked` on wait channel sleep, `Suspended` on pause signal, or `Zombie` on exit.
- **`Blocked`**: Thread is sleeping on a kernel synchronization object or wait channel (`WaitChannelId`). Transition to `Ready` on channel wakeup, timeout, or signal interruption; `Suspended` or `Zombie` on forced signal termination.
- **`Suspended`**: Thread execution is suspended by a signal or control operation (`SIGSTOP`, debugger). Transition to `Ready` on resume signal (`SIGCONT`), or `Zombie` on termination.
- **`Zombie`**: Thread has exited; kernel resources remain for return code retrieval (`exit_status`). Transition to `Terminated` upon `join()` or parent process reaping.
- **`Terminated`**: Stacks, file descriptor tables, and kernel tracking objects are completely reclaimed. Terminal state.

Any transition not explicitly allowed by `ThreadState::is_valid_transition` returns an error (`"invalid thread state transition"`).

---

### 2. Thread Ownership

- **`Kernel`**: Pure kernel background thread executing within kernel address space (no user process bindings).
- **`User`**: User-space thread bound to an owning process PID.
- **`Process`**: Primary thread of a process group.
- **`Detached`**: Thread configured for automatic resource reclamation upon exit without requiring a `join()` caller.

---

### 3. Core Guarantees & Safety Invariants

1. **Preemption Model**: Threads are preemptible during normal kernel and user mode execution. Spinlock hold regions disable preemption to guarantee interrupt safety.
2. **Priority Bounds**: Static priorities map to `[0..139]` (`120` default). Nice values map to `[-20..19]`. Priority values outside range are clamped.
3. **Affinity Semantics**: Multi-core CPU scheduling affinity is specified via a 64-bit mask (`CpuAffinity`).
4. **Stack Guard Safety**: Stacks (`StackMeta`) define base addresses, sizes, and guard zones. Pointers falling into the guard zone trigger immediate fault handling.
5. **Wait-Channel Wakeup Safety**: Wait channels (`WaitChannelId`) prevent lost wakeups through atomic state checking during wakeup registration, timeouts, and signal interruptions.
6. **Lock-Order Tracking**: Threads record held lock addresses (`locks_held`) to track lock acquisition ordering and prevent deadlock hazards.

---

## Phase 2: Canonical `Thread` Object Structure

The canonical implementation resides in `src/kernel/thread.rs`:

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

1. **Run Queue**: Per-CPU ready queues supporting priority levels, load balancing hooks, and scheduling classes (`Normal`, `Fifo`, `RoundRobin`, `Batch`, `Idle`, `Deadline`).
2. **Context Switching**: Register save/restore (`CpuContext`), stack pointer switching, and safe `Running` <-> `Ready` transitions.
3. **Preemption Control**: Timer interrupt driven preemption, voluntary yielding, and bounded scheduling latency.
4. **Blocking & Wakeup**: Safe sleep on wait channels (`block_on`), wakeups (`wake_wait_channel`), timeouts (`timeout_wait_channel`), and signal interruptions (`deliver_signal`).
5. **Exit & Reaping**: Thread exit, `join()`, `detach()`, zombie cleanup (`reap_terminated`), and stack reclamation.
6. **Observability**: Process/thread status queries, CPU utilization tracking, and context switch metrics.

---

## Phase 4: POSIX API Mapping

Minimum libc / POSIX thread API target coverage:

- `pthread_create` -> `ThreadManager::create_thread`
- `pthread_exit` -> `Thread::exit`
- `pthread_join` -> `Thread::join`
- `pthread_detach` -> `Thread::detach`
- `pthread_yield` / `sched_yield` -> Scheduler yield
- `pthread_sigmask` -> `Thread::set_signal_mask`

---

## 10-PR Sequence Roadmap

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
