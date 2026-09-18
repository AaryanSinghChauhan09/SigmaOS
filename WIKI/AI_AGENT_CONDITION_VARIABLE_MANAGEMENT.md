# AI Agent Guidelines for SigmaOS Condition Variables & Synchronization Primitives Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Condition Variables, Futex-backed Condvars, Mutexes, and Capability-Guarded Thread Synchronization**.

---

## 1. System Architecture & Synchronization Overview

SigmaOS implements condition variables and thread synchronization primitives across two primary subsystems:

1. **Kernel Futex & Wait Queue Primitive Tier (`src/kernel/unix_primitives.rs`)**
   - **`FutexTable` & `FutexWaiter`:** Kernel-level fast userspace synchronization table supporting `FUTEX_WAIT`, `FUTEX_WAKE`, and `FUTEX_REQUEUE` semantics. Address-keyed condition variable parking and notification.
   - **`WaitQueue` & `WaitEntry`:** In-kernel sleep queues for thread parking and thundering-herd resistant signaling.

2. **Capability-Guarded Runtime Primitives (`src/runtime/threading/thread.rs`)**
   - **`Mutex` & `MutexCapability`:** Capability-based mutual exclusion lock (`can_lock`, `can_unlock`).
   - **`Semaphore` & `SemaphoreCapability`:** Counting semaphore (`can_wait`, `can_signal`).
   - **`RwLock` & `RwLockCapability`:** Readers-writer lock supporting concurrent reader locks (`can_read_lock`, `can_write_lock`, `can_unlock`).

---

## 2. Condition Variable Mechanics & Code Patterns

Condition variables in SigmaOS rely on futex address parking combined with capability-guarded mutexes.

### 1. Futex-backed Condition Variable Pattern
To implement a condition variable `wait` and `notify`/`notify_all` pattern:
- **`cond_wait`:** Releases the associated `Mutex`, parks caller `pid` on `uaddr` via `futex.wait(pid, uaddr, current_seq, expected_seq, bitmask)`, and re-acquires the mutex upon unparking.
- **`cond_notify_one`:** Calls `futex.wake(uaddr, 1, bitmask)` to unpark 1 waiting thread.
- **`cond_notify_all`:** Calls `futex.wake(uaddr, usize::MAX, bitmask)` to unpark all waiting threads.

```rust
use sigma::kernel::unix_primitives::{FutexTable, FutexError};

let mut futex = FutexTable::new();
let uaddr = 0x7fff_0000_1000u64;

// Condvar wait: park current thread on uaddr if condition sequence matches
futex.wait(current_pid, uaddr, seq, expected_seq, u32::MAX)?;

// Condvar notify_one: wake 1 waiter on uaddr
futex.wake(uaddr, 1, u32::MAX);

// Condvar notify_all: wake all waiters on uaddr
futex.wake(uaddr, usize::MAX, u32::MAX);
```

### 2. Capability-Guarded Mutex & RwLock Operations
Runtime synchronization primitives enforce capability rights before allowing state mutations:

```rust
use sigma::runtime::threading::thread::{Mutex, MutexCapability, RwLock, RwLockCapability};

// Mutex with full capability
let mut_cap = MutexCapability::full();
let mutex = unsafe { Mutex::new(mut_cap) };

unsafe {
    if mutex.lock() {
        // Critical section
        mutex.unlock();
    }
}

// RwLock with reader/writer capability
let rw_cap = RwLockCapability::full();
let rwlock = unsafe { RwLock::new(rw_cap) };

unsafe {
    if rwlock.read_lock() {
        // Concurrent read section
        rwlock.unlock();
    }
}
```

---

## 3. Testing & Verification Protocol for AI Agents

When modifying condition variables, futexes, or capability-guarded synchronization primitives, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for kernel primitives and runtime threading:

```bash
rustc --test --edition=2021 src/kernel/unix_primitives.rs -o build/test_primitives && ./build/test_primitives
rustc --test --edition=2021 src/runtime/threading/thread.rs -o build/test_threading && ./build/test_threading
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core process/thread synchronization subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Safety Rules

- **Spurious Wakeup Handling:** Always wrap `futex.wait` or condition variable checks in a `while` loop that re-evaluates the condition predicate.
- **Capability Authorization:** Always verify `capability.can_lock` / `can_unlock` before attempting state mutations on `Mutex`, `Semaphore`, or `RwLock`.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
