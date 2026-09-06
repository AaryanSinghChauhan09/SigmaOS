# SigmaOS AI Agent Readers/Writers Problems Operation Management Guide

This guide defines concurrency control protocols, lock-free reading mechanisms, and synchronization patterns for AI agents solving Readers/Writers problems across kernel, VFS, memory management, and network subsystems in SigmaOS.

---

## 1. Core Principles of Readers/Writers Synchronization

In `#![no_std]` kernel space, multiple readers may access shared data concurrently, but writers require exclusive access. AI agents MUST observe the following guidelines:

1. **No Standard Library Locks:** Do NOT import `std::sync::RwLock` or `std::sync::Mutex`. Use native `#![no_std]` lock primitives (e.g. `AtomicU32`, `TicketSpinlock`, or RCU pointers).
2. **Writer Starvation Prevention:** Default to **Writer-Preference** or **Fair FIFO Read-Write Locks** in high-write subsystems (e.g., page table updates, routing table modifications) to prevent continuous reader streams from starving write operations.
3. **Lock-Free Read Pathways (RCU):** For read-heavy, low-write kernel structures (e.g., VFS path resolution, process tree lookups, eBPF maps), use Read-Copy-Update (RCU) or hazard pointers to achieve $O(1)$ zero-lock contention reads.
4. **Bounded Spin-Wait Backoff:** Spin-wait loops in read-write locks MUST execute `core::hint::spin_loop()` or exponential pause backoff to prevent bus locking on multi-socket SMP systems.

---

## 2. Readers/Writers Lock Variants in SigmaOS

### A. Atomic Reader-Count Spinlock (`AtomicRwLock`)
Uses a single `AtomicU32` state variable:
* `0`: Unlocked.
* `1..0x7FFF_FFFF`: Active reader count ($N$ concurrent readers).
* `0x8000_0000`: Writer lock held exclusively.

```rust
pub struct AtomicRwLock<T> {
    state: AtomicU32,
    data: UnsafeCell<T>,
}

impl<T> AtomicRwLock<T> {
    pub const WRITER_BIT: u32 = 0x8000_0000;

    pub fn read(&self) -> ReadGuard<T> {
        loop {
            let current = self.state.load(Ordering::Relaxed);
            if current & Self::WRITER_BIT != 0 {
                core::hint::spin_loop();
                continue;
            }
            if self.state.compare_exchange_weak(
                current,
                current + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                return ReadGuard { lock: self };
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        loop {
            if self.state.compare_exchange_weak(
                0,
                Self::WRITER_BIT,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                return WriteGuard { lock: self };
            }
            core::hint::spin_loop();
        }
    }
}
```

---

## 3. Read-Copy-Update (RCU) Lock-Free Pattern

When readers outnumber writers by 100:1 or more, use Read-Copy-Update (RCU):

1. **Read Path:** Readers dereference `AtomicPtr<T>` under `rcu_read_lock()` without acquiring locks or mutating state cache lines.
2. **Write Path:** Writers allocate a new object copy, modify the copy, swap the `AtomicPtr<T>` pointer atomically via `Release` ordering, and defer deallocation until an RCU grace period (`synchronize_rcu()`) completes.

---

## 4. Deadlock Avoidance & Interrupt Safety

* **Lock Ordering Hierarchy:** Always acquire Readers/Writers locks in strict hierarchical order (e.g., `VFS_Inode_Lock` $\to$ `Directory_Entry_Lock` $\to$ `Page_Cache_Lock`).
* **Interrupt-Safe Locks (`read_irqsave` / `write_irqsave`):** If a reader/writer lock is accessed inside an IRQ handler, AI agents MUST disable local CPU interrupts prior to acquiring the lock to prevent self-deadlock.
