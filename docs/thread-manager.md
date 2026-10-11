# SigmaOS Thread Manager Architecture & Subsystem Contract

## Purpose

The SigmaOS Thread Manager provides the canonical thread lifecycle abstraction and core primitives for executing concurrent tasks within kernel space and user processes. It defines explicit thread states, state transitions, ownership semantics, stack bounds, wait-channel safety, and cancellation behavior.

---

## Thread Lifecycle & States

Every thread in SigmaOS strictly progresses through a defined state machine:

| Thread State | Description | Legal Transitions |
|---|---|---|
| `New` | Newly instantiated thread object, context initialized, memory allocated | -> `Ready` |
| `Ready` | Thread is runnable and queued in a CPU run queue awaiting scheduling | -> `Running` |
| `Running` | Thread is currently executing on a CPU core | -> `Ready`, `Blocked`, `Suspended`, `Zombie`, `Terminated` |
| `Blocked` | Thread is sleeping, awaiting a wait-channel event, timer, or I/O | -> `Ready` |
| `Suspended` | Thread is explicitly paused by process/thread control signals | -> `Ready` |
| `Zombie` | Thread has exited; termination status stored awaiting `join()` reap | -> `Terminated` |
| `Terminated` | Stack and metadata fully reclaimed; ID recycled | Terminal state |

---

## Ownership Semantics

Threads in SigmaOS fall into one of four explicit ownership categories:

1. **Kernel Threads (`kthread`):**
   - Executed entirely in supervisor mode (`Ring 0`).
   - Managed directly by the kernel thread manager.
   - Do not possess a user-space page table or user stack.
2. **User Threads (`uthread`):**
   - Associated with a user process memory space (`CR3` mapping).
   - Has dual stacks: kernel trap stack and user stack.
3. **Process Threads:**
   - Grouped under a parent Process Control Block (PCB).
   - Share file-descriptor tables, memory mappings, and signal dispositiions.
4. **Detached Threads:**
   - Disowned upon creation or via explicit `detach()` call.
   - Reclaim resources automatically upon transition to `Zombie` without requiring `join()`.

---

## Subsystem Guarantees & Non-Goals

### Guarantees

- **Preemption Model:** Preemptive scheduling driven by LAPIC timer interrupts with voluntary yield capability (`sys_sched_yield`).
- **Priority Ranges:** 0 to 139 (0–99 Real-Time FIFO/RR, 100–139 Normal/BORE/EEVDF with nice values -20 to +19).
- **Affinity Semantics:** Core pinning bitmask (`cpu_set_t`) enforced by run-queue pick routines.
- **Stack Limits:** Enforced kernel stack boundaries with memory guard pages to catch stack overflows before memory corruption occurs.
- **Cancellation Behavior:** Cooperative cancellation flags checked at cancellation points (syscall entry, blocking wait channels, explicit testpoints).

### Non-Goals

- **POSIX Compliance:** POSIX `pthreads` compliance is **not claimed** for kernel thread manager structures until libc-facing syscall wrappers and signal routing behavior pass end-to-end userland execution tests.

---

## Public API & Struct Blueprint

```rust
pub struct CanonicalThread {
    pub tid: u64,
    pub pid: u64,
    pub state: ThreadState,
    pub priority: u8,
    pub nice: i8,
    pub cpu_affinity: u64,
    pub kernel_stack_top: usize,
    pub user_stack_top: Option<usize>,
    pub wait_channel: Option<u64>,
    pub signal_mask: u64,
    pub join_handle: Option<ThreadJoinState>,
}
```

---

## Testing & Acceptance Criteria

- `cargo check --lib` passes clean.
- `cargo test --lib` passes clean.
- `./run_sigma_tests.sh` passes clean.
- Unit tests verify every state transition in `ThreadState`.
- Concurrency/race tests verify lost-wakeup prevention on wait channels during concurrent timeout and signal handling.
