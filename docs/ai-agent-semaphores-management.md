# AI Agent Binary Semaphores & Concurrency Synchronization in SigmaOS

## Overview

SigmaOS synchronization primitives (`src/kernel/linux_bsd_innovations.rs`, `src/process/blocked_state.rs`, `src/klib/async_runtime.rs`) implement lock-free atomic operations, fast userspace mutexes (`LinuxFutexEngine`), and binary semaphores for thread-safe resource access.

AI agents (such as Jules, Herdr agentic subagents, and concurrent task workers) must use binary semaphores (`BinarySemaphore`) to coordinate shared state access, avoid race conditions, and guarantee deadlock freedom.

---

## Binary Semaphore State Machine

A binary semaphore acts as a 1-bit synchronization gate:

```
            Semaphore Initialized (Value = 1)
                           │
                 [ wait() / P() / acquire() ]
                           │
                           ▼
            Semaphore Locked (Value = 0)
                           │
             [ post() / V() / release() ]
                           │
                           ▼
            Semaphore Unlocked (Value = 1)
```

---

## Programmatic Binary Semaphore Usage

AI agents coordinate critical section access across worker subagent threads using binary semaphores:

```rust
use sigmaos::sync::BinarySemaphore;

let sem = BinarySemaphore::new(1); // Initially unlocked (value = 1)

// Acquire binary semaphore before accessing shared workspace buffer
sem.acquire()?; // Decrements value to 0; blocks if value was 0

{
    // CRITICAL SECTION: Safe shared resource manipulation
    shared_workspace_buffer.push_data(data_payload);
}

// Release binary semaphore when done
sem.release()?; // Increments value to 1; wakes sleeping waiter thread
```

---

## Futex Backing (`futex_wait` / `futex_wake`)

Under heavy thread contention, binary semaphores in SigmaOS delegate thread sleeping and waking to the Linux-parity fast userspace mutex engine (`LinuxFutexEngine`):

```rust
use sigmaos::kernel::LinuxFutexEngine;

let mut futex = LinuxFutexEngine::new();

// Wait on lock memory address if value matches expected
futex.futex_wait(lock_uaddr, current_val, expected_val, agent_thread_id, timeout_ns)?;

// Wake sleeping subagent thread upon lock release
futex.futex_wake(lock_uaddr, 1);
```

---

## Deadlock Prevention Rules for AI Agents

1. **Strict Lock Hierarchy Ordering**: Always acquire binary semaphores in global alphabetical / ID order (e.g. `sem_a` before `sem_b`).
2. **RAII Lock Guard Scope**: Use RAII wrapper guards (`BinarySemaphoreGuard`) to ensure semaphores automatically unlock upon scope exit or panic.
3. **Timeout Bounded Waits**: Avoid infinite wait loops; always supply a maximum timeout value (`Option<u64>`) when acquiring semaphores in background subagent tasks.
