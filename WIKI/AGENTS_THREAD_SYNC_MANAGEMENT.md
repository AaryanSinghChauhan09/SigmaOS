# SigmaOS Thread Synchronization, Futex & Atomic Concurrency Guide for AI Agents

This guide provides technical specifications, fast userspace mutex (futex) syscall operations, atomic memory orderings (`Ordering::Acquire`, `Ordering::Release`, `Ordering::Relaxed`), and lock-free thread synchronization rules for AI agents working in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Thread Synchronization Architecture

SigmaOS implements thread synchronization without external crate dependencies, relying on core atomic primitives and kernel futex syscalls (`src/kernel/syscall/table.rs`, `src/scheduler/ebpf_scheduler.rs`):

* **Fast Userspace Mutex (Futex Syscall):**
  Provides lightweight userspace synchronization primitives (`Futex = 202`) with kernel wait and wake queues to avoid unnecessary context switches.
* **Atomic Flags & Orderings (`AtomicBool`, `AtomicU64`, `AtomicUsize`):**
  Used across lock-free structures (such as eBPF scheduler programs, metrics collection counters, and ring buffer head/tail pointers).

---

## 2. Memory Ordering & Lock-Free Synchronization Rules

When implementing or modifying concurrent thread synchronization:

1. **Explicit Memory Orderings:**
   Do NOT use `Ordering::Relaxed` for pointer published state or cross-thread flag synchronization. Always use `Ordering::Release` when publishing shared state updates and `Ordering::Acquire` when reading published state.
2. **Futex Wait/Wake Semantics:**
   Userspace lock implementations MUST check atomic lock variables before invoking the futex wait system call to maximize fast-path non-blocking execution.
3. **Spinlock & Backoff Policy:**
   In `#![no_std]` spinlock routines, thread loops MUST execute CPU pause hints (`core::hint::spin_loop()`) to reduce bus contention during contention.

---

## 3. Checklist for AI Agents Managing Thread Synchronization

1. **Verify Memory Orderings:** Check that atomic stores and loads use appropriate Acquire/Release semantics.
2. **Test Concurrency Pipelines:**
   Run scheduler and concurrency unit tests:
   ```bash
   cargo test --lib -- scheduler::tests
   ./run_sigma_tests.sh
   ```
