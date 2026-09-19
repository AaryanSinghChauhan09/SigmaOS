# AI Agents Readers-Writers Problem Operation Management Guide for SigmaOS

## Overview
SigmaOS implements high-performance, starvation-free synchronization primitives to solve classical Readers-Writers problems across microkernel and userland threads (`src/runtime/threading/thread.rs`). This guide details how AI agents manage concurrent access to shared resources, select appropriate read-write lock policies, prevent reader or writer starvation, and monitor synchronization contention via eBPF probes.

---

## Classical Readers-Writers Formulations & Policies

SigmaOS supports three distinct operational modes for read-write synchronization (`RwLock`):

```
                       ┌─────────────────────────┐
                       │  Read-Write Lock (RwLock)│
                       └────────────┬────────────┘
                                    │
         ┌──────────────────────────┼──────────────────────────┐
         ▼                          ▼                          ▼
┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
│ First Problem    │      │ Second Problem   │      │ Third Problem    │
│ Readers-Preference│      │ Writers-Preference│      │ Fair FIFO Policy │
├──────────────────┤      ├──────────────────┤      ├──────────────────┤
│ - Max concurrency│      │ - Max freshness  │      │ - Zero starvation│
│ - Writers may    │      │ - Readers may    │      │ - Strict queue   │
│   starve         │      │   starve         │      │   order          │
└──────────────────┘      └──────────────────┘      └──────────────────┘
```

1. **First Readers-Writers Problem (Readers-Preference)**:
   - Multiple concurrent readers can acquire the lock as long as no writer holds or waits for it.
   - Ideal for read-heavy workloads (e.g., page table lookups, routing table reads).
   - *Risk*: Continuous reader stream can starve pending writers.

2. **Second Readers-Writers Problem (Writers-Preference)**:
   - Once a writer requests access, new readers are blocked until the writer completes execution.
   - Ensures timely state updates (e.g., security policy hot-reloading, cache invalidation).
   - *Risk*: High writer frequency can starve incoming readers.

3. **Third Readers-Writers Problem (Fair / FIFO Policy)**:
   - Readers and writers are served in strict queue order without favoring either role.
   - Completely eliminates starvation for both readers and writers.
   - Recommended for general-purpose AI agent shared memory and metadata tables.

---

## `RwLock` Primitive Implementation (`src/runtime/threading/thread.rs`)

The SigmaOS native `RwLock` uses atomic operations and thread capabilities:

```rust
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub struct RwLock {
    readers: AtomicUsize,
    writer: AtomicBool,
    write_wait_queue: *mut Thread,
    read_wait_queue: *mut Thread,
    capability: RwLockCapability,
}

impl RwLock {
    pub unsafe fn read_lock(&self) -> bool {
        if !self.capability.can_read_lock {
            return false;
        }

        // Wait while a writer holds the lock
        while self.writer.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }

        self.readers.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub unsafe fn write_lock(&self) -> bool {
        if !self.capability.can_write_lock {
            return false;
        }

        // Acquire writer flag
        while self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }

        // Wait for active readers to complete
        while self.readers.load(Ordering::Acquire) > 0 {
            core::hint::spin_loop();
        }

        true
    }

    pub unsafe fn unlock(&self) -> bool {
        if !self.capability.can_unlock {
            return false;
        }

        if self.writer.load(Ordering::SeqCst) {
            self.writer.store(false, Ordering::Release);
        } else {
            self.readers.fetch_sub(1, Ordering::AcqRel);
        }

        true
    }
}
```

---

## Starvation Prevention & Lock Contention Telemetry

AI agents monitor `RwLock` metrics in real-time to detect lock contention and automatically adjust policy parameters:

### Metrics Tracked by Telemetry
- `reader_count`: Active concurrent readers (`AtomicUsize`).
- `writer_wait_time_ns`: Time elapsed since writer lock acquisition request.
- `lock_contention_events`: Total number of spin-loop iterations before acquiring lock.

### Autonomous Policy Adaptation Rule
```
IF (writer_wait_time_ns > 5,000,000 ns AND pending_writers > 0) THEN
    Switch RwLock policy to Writers-Preference OR Fair-FIFO
ELSE IF (reader_wait_time_ns > 10,000,000 ns) THEN
    Switch RwLock policy to Readers-Preference
END IF
```

---

## Lock Upgrade and Downgrade Semantics

AI agents must follow precise protocols when transitioning between read and write locks:

1. **Lock Downgrade (Safe)**:
   - Hold write lock -> Increment `readers` atomic count -> Set `writer` flag to `false`.
   - Maintains continuous access without relinquishing protection.

2. **Lock Upgrade (Caution)**:
   - Direct inline lock upgrade from `read_lock` to `write_lock` can cause deadlock if multiple readers attempt upgrading simultaneously.
   - *Protocol*: Release read lock -> Acquire write lock with bounded exponential backoff.

---

## Navigation
* **Return to [Master Developer Guide](Home.md)**
* **Proceed to [AI Agents Configurability Management Guide](AI_AGENTS_CONFIGURABILITY_MANAGEMENT_GUIDE.md)**
* **Proceed to [AI Agents Thread Synchronization Guide](AI_AGENTS_THREAD_SYNC_MANAGEMENT_GUIDE.md)**
