# SigmaOS AI Agent Spinlock Management Guide

This guide defines spinlock synchronization rules, atomic memory ordering standards, interrupt-safe lock patterns, and deadlock prevention protocols for AI coding agents developing on the `#![no_std]` SigmaOS microkernel.

---

## 1. `#![no_std]` Spinlock Synchronization Primitives

SigmaOS avoids standard library `std::sync::Mutex` in core kernel and `klib` modules. Coding agents must utilize microkernel spinlocks operating directly on atomic booleans or integer flags (`core::sync::atomic::AtomicBool` / `AtomicU32`):

```rust
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock {
    locked: AtomicBool,
}

impl Spinlock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
```

---

## 2. Atomic Memory Ordering Standards

To ensure memory barrier safety across multi-core SMP architectures (x86_64, AArch64, RISC-V 64, LoongArch64):

* **Lock Acquisition (`Acquire` / `SeqCst`):** Always use `Ordering::Acquire` or `Ordering::SeqCst` when acquiring spinlocks or reading shared state flags to prevent compiler/CPU memory instruction reordering.
* **Lock Release (`Release` / `SeqCst`):** Always use `Ordering::Release` or `Ordering::SeqCst` when releasing spinlocks or committing shared memory updates.
* **Non-Critical Counter Updates (`Relaxed`):** `Ordering::Relaxed` is permitted ONLY for isolated stats counters or diagnostic metrics where strict memory barriers are unnecessary.

---

## 3. Interrupt-Safe IRQ-Save Spinlocks

When acquiring spinlocks inside device driver IRQ handlers or scheduling interrupt contexts, agents MUST disable local CPU interrupts before locking to prevent self-deadlock:

```rust
pub struct IrqSaveSpinlockGuard<'a> {
    lock: &'a Spinlock,
    flags: usize,
}

impl Spinlock {
    pub fn lock_irqsave(&self) -> usize {
        let flags = disable_local_interrupts();
        self.lock();
        flags
    }

    pub fn unlock_irqrestore(&self, flags: usize) {
        self.unlock();
        restore_local_interrupts(flags);
    }
}
```

---

## 4. Deadlock Prevention Guidelines

1. **Strict Lock Ordering:** Always acquire nested spinlocks in a fixed, global order (e.g. `PMM Lock -> VMM Lock -> Process Table Lock`).
2. **No Memory Allocation While Holding Locks:** NEVER call heap allocators or long-running I/O routines while holding a spinlock to prevent lock contention spikes and OOM deadlocks.
3. **Bounded Spin Loops:** Implement spin-loop timeout limits or fallback yield hints (`core::hint::spin_loop()`) to prevent CPU core lock-up under heavy contention.
4. **Lock-Free Fallbacks:** For high-throughput queues, prefer lock-free atomic ring buffers (`AtomicUsize` head/tail pointers) over spinlock-guarded queues.
