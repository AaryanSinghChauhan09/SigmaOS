# Spinlock Management Guidelines for AI Agents (`docs/ai_agents_spinlocks_management.md`)

This document provides AI agents with directives, architectural standards, Rust types, and safety rules for managing **Spinlocks** across the SigmaOS kernel and driver subsystems.

---

## 1. Overview of Spinlocks in SigmaOS

In a `#![no_std]` operating system kernel like SigmaOS, traditional blocking mutexes (which sleep the calling thread) cannot be used in low-level context, interrupt handlers, or core scheduler code. SigmaOS utilizes **Spinlocks** for fine-grained, thread-safe synchronization across kernel CPUs and interrupt vectors.

Key characteristics:
* **Busy-Waiting:** Acquiring CPUs spin in a tight loop using `core::hint::spin_loop()` until the lock memory location is freed.
* **IRQ Safety:** Spinlocks held in interrupt handlers or context switches must ensure interrupts are disabled or masked to prevent single-core deadlocks.
* **Contention Tracking:** Spinlock implementations record lock frequency and spin cycles for dynamic scheduler and lock profiling.

---

## 2. Core Spinlock Implementations & Rust Types

### 2.1 `FineGrainedSpinlock` (`src/kernel/core/sovereign_scheduler.rs`)
Inspired by FreeBSD `mtx` and Linux `spinlock_t`, `FineGrainedSpinlock` provides atomic CAS locking with fine-grained lock contention metrics:

```rust
pub struct FineGrainedSpinlock {
    locked: AtomicUsize,
    pub lock_count: AtomicUsize,
    pub spin_cycles: AtomicUsize,
}

impl FineGrainedSpinlock {
    pub const fn new() -> Self;
    pub fn lock(&self);
    pub fn unlock(&self);
}
```

* **Usage Pattern:** Used by `SovereignAdaptiveScheduler` for task queue synchronization and contention monitoring.

### 2.2 `SpinMutex` (`src/system/state.rs`)
Wraps system configuration and static global state structures:

```rust
// SAFETY: SpinMutex ensures exclusive access via a spinlock before returning mutable pointers.
pub static SYSTEM_STATE: SpinMutex<GlobalSystemConfig> = SpinMutex::new(...);
```

* **Usage Pattern:** Used for global kernel configuration access where sleeping is prohibited.

### 2.3 Atomic RingBuffer Spinlock (`src/klib/ringbuf.rs`)
Simple lock-free / low-overhead spinlock integrated into Multi-Producer Multi-Consumer (MPMC) lock-free IPC channels:

```rust
pub struct AtomicRingBuffer<T> {
    lock: AtomicUsize, // 0 = free, 1 = locked
    // ...
}
```

### 2.4 WDK Driver Spinlock (`src/kernel/wdk_core.rs`)
Windows Kernel WDK compatibility spinlock abstraction that enforces IRQL execution bounds:

```rust
// Ensures spinlocks are not acquired above DISPATCH_LEVEL
pub fn acquire_spinlock_wdk(irql: KernelIrql) -> Result<(), &'static str> {
    if irql > KernelIrql::DispatchLevel {
        return Err("SpinLock: Cannot acquire spinlock above DISPATCH_LEVEL");
    }
    // ...
}
```

---

## 3. Spinlock Safety & Directives for AI Agents

When implementing or modifying kernel code involving spinlocks, AI agents MUST follow these mandatory rules:

1. **Never Sleep While Holding a Spinlock:**
   Do NOT invoke context switches, thread sleeps, asynchronous I/O, or blocking memory allocations while holding a spinlock.

2. **Always Use `core::hint::spin_loop()`:**
   Spin loops MUST include `core::hint::spin_loop()` inside busy-wait loops to optimize CPU execution pipeline efficiency and reduce power consumption on x86 (`PAUSE`), ARM (`YIELD`), and RISC-V architectures.

   ```rust
   while self.locked.compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
       core::hint::spin_loop();
   }
   ```

3. **Strict Lock Ordering (Deadlock Prevention):**
   If acquiring multiple spinlocks, always acquire them in a global, deterministic order (e.g., `SchedulerLock -> MemoryManagerLock -> DeviceLock`) to prevent AB-BA deadlocks across SMP cores.

4. **Disable Interrupts in IRQ Handlers:**
   When acquiring a spinlock that is also accessed inside an Interrupt Service Routine (ISR), disable interrupts on the local CPU (`irq_save()`) prior to acquiring the lock, and restore them (`irq_restore()`) upon releasing the lock.

5. **Keep Critical Sections Minimal:**
   Perform only mandatory memory or state mutations inside spinlock-protected regions. Copy small data out of critical sections before performing complex logic.

---

## 4. Spinlock Contention & Metrics Inspection

AI agents can evaluate spinlock contention and lock efficiency using embedded kernel statistics:

```rust
let spinlock = FineGrainedSpinlock::new();
spinlock.lock();
// ... critical section ...
spinlock.unlock();

let total_locks = spinlock.lock_count.load(Ordering::Relaxed);
let total_spins = spinlock.spin_cycles.load(Ordering::Relaxed);
let avg_spins_per_lock = if total_locks > 0 { total_spins / total_locks } else { 0 };
```

---

## 5. Verification & Testing Procedure for Spinlocks

When modifying spinlock structures or locking logic:

1. **Run Unit Tests:**
   Execute kernel scheduler spinlock unit tests:
   ```bash
   rustc --test --edition 2021 src/kernel/core/sovereign_scheduler.rs -o build/test_spinlock
   ./build/test_spinlock
   ```

2. **Run Full Kernel Test Suite:**
   Validate overall OS stability under thread contention:
   ```bash
   ./run_sigma_tests.sh
   ```

---
*Maintained by the SigmaOS Core Kernel Team.*
