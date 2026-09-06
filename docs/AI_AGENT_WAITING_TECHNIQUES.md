# AI Agent Guidelines for SigmaOS Waiting Techniques & Synchronization Primitives

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Waiting Techniques, Synchronization Mechanisms, and Process Parking Primitives**.

---

## 1. System Architecture & Waiting Primitives Overview

SigmaOS absorbs foundational Linux and FreeBSD kernel waiting techniques into safe, capability-aware Rust implementations in `src/kernel/unix_primitives.rs` and core kernel scheduling modules:

| Waiting Primitive | Heritage / Inspiration | Primary Source File | Core Data Structures | Use Case |
| :--- | :--- | :--- | :--- | :--- |
| **Wait Queues** | Linux `kernel/sched/wait.c` | `src/kernel/unix_primitives.rs` | `WaitQueue`, `WaitEntry` | Sleep/wakeup queues for I/O events, process state changes |
| **Futex Table** | Linux `kernel/futex.c` | `src/kernel/unix_primitives.rs` | `FutexTable`, `FutexWaiter`, `FutexOp` | Fast userspace locking (mutex, condvar, semaphore park/unpark) |
| **Turnstiles** | FreeBSD `sys/kern/subr_turnstile.c` | `src/kernel/unix_primitives.rs` | `Turnstile`, `TurnstileWaiter` | Contended kernel mutexes with priority inheritance |
| **Sequence Locks** | Linux `include/linux/seqlock.h` | `src/kernel/unix_primitives.rs` | `SeqLock` | Lock-free reader non-blocking waits for frequent reads/rare writes |
| **Callout Wheel** | FreeBSD `sys/kern/kern_timeout.c` | `src/kernel/unix_primitives.rs` | `CalloutWheel`, `Callout`, `CalloutState` | One-shot and periodic timer callouts & softclock ticks |
| **RCU Epochs** | Linux RCU / FreeBSD SMR | `src/kernel/unix_primitives.rs` | `RcuEpoch` | Quiescent-state epoch reclamation without reader locks |
| **Kqueue Multiplexer** | FreeBSD `sys/kern/kern_event.c` | `src/kernel/unix_primitives.rs` | `Kqueue`, `Kevent`, `KqFilter` | Scalable event notification & async I/O waiting |

---

## 2. Waiting Primitive Mechanics & Code Patterns

AI agents modifying task blocking, wait queues, or synchronization mechanisms must adhere to the following code patterns:

### 1. Wait Queues (`WaitQueue`)
Used when a thread must park until a condition or I/O event is satisfied:
- **Exclusive vs Non-Exclusive Waiters:** `add_waiter(pid, exclusive)` allows waking either single exclusive threads (preventing thundering herd) or all non-exclusive waiters.
- **Wakeup:** `wake_up(nr)` unparks up to `nr` threads and increments the queue generation counter (`generation()`).

```rust
use sigma::kernel::unix_primitives::WaitQueue;

let mut wq = WaitQueue::new();
wq.add_waiter(pid, false); // Add non-exclusive waiter
let woken_pids = wq.wake_up(1); // Wake up 1 waiter
```

### 2. Fast Userspace Futexes (`FutexTable`)
Implements `FUTEX_WAIT` and `FUTEX_WAKE` semantics keyed by userspace memory addresses (`uaddr`):
- **`wait`:** Checks if `*uaddr == expected`. Parks caller `pid` if values match; returns `FutexError::WouldBlock` otherwise.
- **`wake`:** Unparks up to `nr` waiters whose bitset overlaps with the wake bitmask.

```rust
use sigma::kernel::unix_primitives::{FutexTable, FutexError};

let mut futex = FutexTable::new();
// FUTEX_WAIT: park if value is 0
futex.wait(pid, uaddr, current_val, 0, u32::MAX)?;
// FUTEX_WAKE: wake 1 waiter on uaddr
let woken_count = futex.wake(uaddr, 1, u32::MAX);
```

### 3. Priority-Inheritance Turnstiles (`Turnstile`)
Prevents priority inversion on contended kernel mutexes by dynamically inheriting the highest priority among waiting threads (`owner_eff_prio`):

```rust
use sigma::kernel::unix_primitives::Turnstile;

let mut turnstile = Turnstile::new();
// Thread 1 (prio 50) acquires lock
turnstile.lock(1, 50).unwrap();
// Thread 2 (prio 10 - higher) contends; turnstile propagates priority to Thread 1
turnstile.lock(2, 10).unwrap_err();
assert_eq!(turnstile.effective_priority(), Some(10));
```

### 4. Sequence Locks (`SeqLock`)
Allows writers to update data while readers read without blocking, retrying if a write occurred during the read window:

```rust
use sigma::kernel::unix_primitives::SeqLock;

let lock = SeqLock::new();
// Reader loop
loop {
    let seq = lock.read_begin();
    let val = read_shared_data();
    if !lock.read_retry(seq) {
        break val;
    }
}
```

---

## 3. Testing & Verification Protocol for AI Agents

When modifying waiting queues, futexes, callouts, or turnstiles, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suite for kernel primitives:

```bash
rustc --test --edition=2021 src/kernel/unix_primitives.rs -o build/test_primitives && ./build/test_primitives
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core kernel scheduling subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Conventions

- **Atomic Ordering:** Ensure atomic synchronization variables use explicit memory ordering (`Acquire`/`Release`/`AcqRel`/`SeqCst`).
- **No Allocation In Loops:** Waiter queue removals (`retain`, `remove`) and softclock ticks must be $O(N)$ bounded and host-safe.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
