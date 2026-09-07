# SigmaOS AI Agent Peterson's Algorithm Operation Management Guide

This guide defines implementation standards, atomic memory fence rules, and multi-core synchronization protocols for AI agents implementing Peterson's Mutual Exclusion Algorithm in `#![no_std]` Rust kernel space across SigmaOS.

---

## 1. Overview & Purpose of Peterson's Algorithm in SigmaOS

Peterson's algorithm is a classical concurrent programming algorithm for mutual exclusion that allows two threads or two CPU execution contexts to share a single-use resource without conflicts, using only shared memory for communication.

In modern multi-core SMP microkernels (x86_64, ARM64, RISC-V), Peterson's algorithm requires explicit atomic operations and sequentially consistent memory fences (`Ordering::SeqCst` / `fence(Ordering::SeqCst)`) to prevent compiler and hardware out-of-order execution reordering.

---

## 2. Lock Architecture (`PetersonLock`)

```rust
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering, fence};

pub struct PetersonLock {
    flag: [AtomicBool; 2],
    turn: AtomicUsize,
}

impl PetersonLock {
    pub const fn new() -> Self {
        Self {
            flag: [AtomicBool::new(false), AtomicBool::new(false)],
            turn: AtomicUsize::new(0),
        }
    }

    /// Acquires critical section for process/core ID `process_id` (0 or 1)
    pub fn lock(&self, process_id: usize) {
        let other = 1 - process_id;

        // 1. Indicate intent to enter critical section
        self.flag[process_id].store(true, Ordering::SeqCst);

        // 2. Yield turn to the other process
        self.turn.store(other, Ordering::SeqCst);

        // 3. Full memory fence prevents store/load reordering
        fence(Ordering::SeqCst);

        // 4. Spin wait while other process wants to enter and it's their turn
        while self.flag[other].load(Ordering::SeqCst) && self.turn.load(Ordering::SeqCst) == other {
            core::hint::spin_loop();
        }
    }

    /// Releases critical section for process/core ID `process_id` (0 or 1)
    pub fn unlock(&self, process_id: usize) {
        self.flag[process_id].store(false, Ordering::SeqCst);
        fence(Ordering::SeqCst);
    }
}
```

---

## 3. Essential Memory Ordering & CPU Fence Requirements

1. **`Ordering::SeqCst` Enforcement:** In software-based mutual exclusion without atomic CPU instructions (like `CMPXCHG` or `LL/SC`), `SeqCst` ordering MUST be enforced across all `load` and `store` operations on `flag` and `turn`.
2. **Explicit Memory Fence:** Insert `core::sync::atomic::fence(Ordering::SeqCst)` after setting `flag` and `turn` to block CPU store-load reordering (where a store to `turn` is delayed after a load of `flag[other]`).
3. **Spin-Wait Hint:** Always call `core::hint::spin_loop()` inside the polling loop to optimize CPU pipeline power consumption and hyperthreading pipeline efficiency.

---

## 4. N-Process Generalizations (Filter & Lamport's Bakery Algorithms)

For synchronization involving $N > 2$ CPU cores or threads:
* **Filter Algorithm:** Extends Peterson's algorithm to $N$ processes using $N-1$ levels of flags and turns.
* **Lamport's Bakery Algorithm:** Uses atomic ticket numbering (`AtomicUsize`) to guarantee FIFO starvation-free mutual exclusion for arbitrary process counts.
