# SigmaOS Thread Manager Architecture & Subsystem Contract

## Purpose

The SigmaOS Thread Manager provides the canonical `Thread` abstraction and lifecycle manager for SigmaOS (`src/kernel/thread.rs`). It defines explicit thread states, state transitions, ownership semantics, stack bounds with guard zones, wait-channel safety, signals, capability context, join/detach mechanics, and lock-order tracking.

---

## Thread Lifecycle & States

Every thread in SigmaOS strictly progresses through a defined state machine (`ThreadState`):

| Thread State | Description | Legal Transitions |
|---|---|---|
| `New` | Newly instantiated thread object, context initialized, memory allocated | -> `Ready`, `Terminated` |
| `Ready` | Thread is runnable and queued in a run queue awaiting scheduling | -> `Running`, `Suspended`, `Zombie` |
| `Running` | Thread is currently executing on a CPU core | -> `Ready`, `Blocked`, `Suspended`, `Zombie` |
| `Blocked` | Thread is sleeping, awaiting a wait-channel event, timer, or I/O | -> `Ready`, `Suspended`, `Zombie` |
| `Suspended` | Thread is explicitly paused by process/thread control signals | -> `Ready`, `Zombie` |
| `Zombie` | Thread has exited; termination status stored awaiting `join()` reap | -> `Terminated` |
| `Terminated` | Stack and metadata fully reclaimed; ID recycled | Terminal state |

Transition rules are enforced via `ThreadState::is_valid_transition()`. Direct invalid jumps are rejected with an error.

---

## Ownership Semantics

Threads in SigmaOS fall into one of four explicit ownership categories (`ThreadOwnership`):

1. **Kernel Threads (`Kernel`):**
   - Executed entirely in supervisor mode (`Ring 0`).
   - Managed directly by the kernel thread manager.
   - Do not possess a user-space page table or user stack (`user_stack: None`).
2. **User Threads (`User`):**
   - Associated with a user process memory space (`CR3` mapping).
   - Has dual stacks: kernel trap stack and user stack (`StackMeta`).
3. **Process Threads (`Process`):**
   - Primary thread of a process group.
   - Shares file-descriptor tables, memory mappings, and signal dispositions under parent PCB.
4. **Detached Threads (`Detached`):**
   - Disowned upon creation or via explicit `detach()` call.
   - Reclaims resources automatically upon transition to `Zombie` (`Terminated`) without requiring `join()`.

---

## Canonical Data Structures

### `Thread` Structure (`src/kernel/thread.rs`)

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
    pub priority: u8,            // 0..139 (default 120)
    pub nice: i8,                 // -20..19 (default 0)
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
    pub locks_held: Vec<u64>,     // Lock Order Tracker
}
```

### `ThreadManager` Control Plane

```rust
pub struct ThreadManager {
    next_tid: AtomicU32,
    threads: Vec<Thread>,
}
```

Methods provided:
- `create_thread(pid, name, ownership, kernel_stack) -> ThreadId`
- `find(tid) -> Option<&Thread>` / `find_mut(tid) -> Option<&mut Thread>`
- `enqueue_ready(tid) -> Result<(), &'static str>`
- `wake_wait_channel(wchan) -> usize` (Lost-wakeup safe wait-channel wake)
- `reap_terminated()` (Reclaims terminated thread resources)
- `active_count() -> usize`

---

## Subsystem Guarantees & Non-Goals

### Guarantees

- **Preemption Model:** Preemptive scheduling driven by LAPIC timer interrupts with voluntary yield capability (`sys_sched_yield`).
- **Priority Ranges:** 0 to 139 (0–99 Real-Time FIFO/RR, 100–139 Normal/BORE/EEVDF with nice values -20 to +19).
- **Affinity Semantics:** Core pinning bitmask (`CpuAffinity`) enforced by run-queue pick routines.
- **Stack Guard Bounds:** `StackMeta` checks (`contains`, `is_in_guard_zone`) detect kernel stack overflows before memory corruption occurs.
- **Lock Order Tracking:** `acquire_lock` and `release_lock` track held lock addresses on each thread to diagnose lock-inversion deadlocks.
- **Wait-Channel Wakeup Safety:** `wake_wait_channel` safely inspects state and wakes all threads waiting on a matching channel identifier.

### Non-Goals

- **POSIX Compliance:** Full POSIX `pthreads` compliance is **not claimed** for kernel thread manager structures until libc-facing syscall wrappers and signal routing behavior pass end-to-end userland execution tests.

---

## Verification & Standalone Testing

The thread manager includes unit tests covering state machine transitions, lock-order tracking, stack guard bounds, wait-channel wakeups, join/detach mechanics, and thread reaping.

To execute the standalone test suite:

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```
