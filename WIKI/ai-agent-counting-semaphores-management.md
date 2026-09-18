# AI Agent Counting General Semaphores Management in SigmaOS

## Overview

SigmaOS concurrency synchronization architecture (`src/kernel/linux_bsd_innovations.rs`, `src/process/`, `src/klib/async_runtime.rs`) provides counting/general semaphores (`CountingSemaphore`) for managing multi-unit shared resource pools, thread worker limits, and async connection concurrency.

AI agents (such as Jules, Herdr agentic subagents, parallel compiler runners, and thread pool workers) must use counting semaphores to regulate N-capacity shared resources safely.

---

## Counting Semaphore State Machine

A counting semaphore maintains an integer counter representing available resource units:

```
            Counting Semaphore Initialized (Capacity = N)
                                │
                      [ wait() / acquire() ]
                                │
                     (Counter = Counter - 1)
                                │
            ┌───────────────────┴───────────────────┐
            ▼                                       ▼
    (Counter >= 0)                           (Counter < 0)
 [ Access Granted ]                     [ Thread Queued & Slept ]
            │                                       │
            └───────────────────┬───────────────────┘
                                ▼
                      [ post() / release() ]
                                │
                     (Counter = Counter + 1)
                                │
                     [ Wake Sleeping Waiter ]
```

---

## Programmatic Counting Semaphore Usage

AI agents regulate concurrent subagent thread access to a shared resource pool (e.g. max 8 concurrent HTTP connections or 16 parallel compilation tasks) using `CountingSemaphore`:

```rust
use sigmaos::sync::CountingSemaphore;

// Initialize counting semaphore with capacity N = 8
let sem = CountingSemaphore::new(8);

// Subagent thread acquires 1 resource unit
sem.acquire()?; // Decrements count; blocks if count <= 0

{
    // CRITICAL SECTION: Execute concurrent subagent task
    execute_parallel_subagent_task();
}

// Subagent thread releases 1 resource unit
sem.release()?; // Increments count; wakes sleeping waiter thread if count <= 0
```

---

## Futex Queue Backing (`LinuxFutexEngine`)

When resource count drops to zero, additional acquiring threads sleep on the lock memory address via `LinuxFutexEngine`:

```rust
use sigmaos::kernel::LinuxFutexEngine;

let mut futex = LinuxFutexEngine::new();

// Wait on counting semaphore memory address
futex.futex_wait(sem_uaddr, current_count, 0, subagent_thread_id, timeout_ns)?;

// Wake 1 sleeping waiter thread when resource unit becomes available
futex.futex_wake(sem_uaddr, 1);
```

---

## Directives for AI Agents

1. **Specify Realistic Capacity Bounds**: Initialize counting semaphores with capacities matching physical system constraints (e.g. CPU core count or memory pool limits).
2. **Match Acquires with Releases**: Guarantee every `acquire()` is balanced with a `release()`, using RAII guards (`CountingSemaphoreGuard`) where possible.
3. **Use Timeout Bounded Waits**: Pass bounded timeout limits (`Option<u64>`) to `acquire_timeout()` to prevent subagent worker threads from blocking indefinitely under resource exhaustion.
