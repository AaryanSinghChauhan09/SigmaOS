# SigmaOS AI Agent Concurrency Mechanism Operation Management Guidelines

## 1. Executive Summary & Overview

SigmaOS incorporates a multi-tier concurrency and synchronization architecture spanning Ring-0 kernel space and sandboxed Ring-3 userland processes. To maintain zero-deadlock guarantees, high throughput, and microsecond-level scheduling latency, AI agents managing or modifying concurrent paths must master the complete suite of SigmaOS concurrency mechanisms.

This document establishes the official guidelines and architectural standards for AI agents managing concurrency mechanisms, lock primitives, lock-free ring buffers, and thread synchronization in SigmaOS.

---

## 2. Taxonomy of Concurrency Mechanisms in SigmaOS

AI agents select concurrency primitives based on execution context, critical section duration, and interrupt safety:

| Concurrency Primitive | Kernel / Userland Module | Typical Usage Scenario | Interrupt Safety & Blocking Rules |
| :--- | :--- | :--- | :--- |
| **`IrqSafeSpinlock`** | `src/kernel/spinlock.rs` | Short Ring-0 critical sections, interrupt handlers | Disables local IRQs. **Must never sleep or yield**. |
| **`KernelSpinlock`** | `src/kernel/spinlock.rs` | Multi-core CPU core state synchronization | Non-IRQ interrupt context. **Must never block**. |
| **`SimpleMutex`** | `src/kernel/` | Long-running kernel tasks, VFS path operations | Futex-backed. Thread blocks/sleeps if contented. |
| **`RwLock` (Read-Write Lock)** | Kernel & Userland IPC | Multi-reader, single-writer data structures | Allows parallel readers; writer acquires exclusive lock. |
| **`Condvar` (Condition Var)** | Kernel & Userland IPC | Thread notification on boolean predicate | Paired with `SimpleMutex`; futex wait queue parked. |
| **Atomic CAS (`AtomicUsize`)** | `core::sync::atomic` | Lock-free counters, flags, ring buffer pointers | Non-blocking. Uses explicit memory `Ordering`. |
| **Read-Copy-Update (RCU)** | `src/kernel/` | Read-heavy VFS mount & routing tables | Zero-lock reads; deferred writer memory reclamation. |

---

## 3. Operational Protocols for Concurrency Management

### 3.1 Spinlock & Interrupt Safety Guidelines

1. **Short Critical Sections**:
   - Spinlocks must protect minimal instruction sequences (e.g. updating a pointer, incrementing a queue head).
2. **No Sleeping Under Spinlocks**:
   - Holding a spinlock while invoking functions that allocate memory, block on I/O, or yield the CPU is an immediate kernel panic error.
3. **IRQ-Safe Spinlock Protection**:
   - Any spinlock shared between a normal thread and an Interrupt Service Routine (ISR) must use `IrqSafeSpinlock` to disable local CPU interrupts prior to lock acquisition.

---

### 3.2 Mutexes, Priority Inheritance, & Futex Wait Queues

1. **Priority Inheritance Protocol (PIP)**:
   - When a high-priority task (e.g. EEVDF real-time audio thread) blocks on a mutex held by a low-priority task, the kernel temporarily elevates the low-priority task's priority to prevent **priority inversion**.
2. **Futex Wait Queues (`sys_futex`)**:
   - Userland mutex contentions execute a fast-path atomic CAS. If contented, the thread transitions to `FUTEX_WAIT` in kernel space without busy-spinning.

---

### 3.3 Lock-Free Concurrency & ABA Problem Mitigation

1. **Compare-and-Swap (CAS) Loops**:
   - Lock-free operations use `compare_exchange_weak` or `compare_exchange` loops with explicit memory barriers (`Ordering::Acquire` / `Ordering::Release`).
2. **ABA Problem Prevention**:
   - Lock-free queues and freelists append double-word generation tags (`generation_count`) or use epoch-based memory reclamation to prevent recycling pointer hazards.

---

### 3.4 Deadlock Prevention Discipline

AI agents must enforce the three rules of deadlock prevention:

1. **Strict Lock Ordering**: Acquire multiple locks in deterministic global lexicographical or hierarchy order (`Lock A -> Lock B -> Lock C`).
2. **Lock Timeouts**: Acquire locks with bounded iteration spin counts or `try_lock()` timeouts.
3. **NMI Watchdog Deadlock Detection**: Anti-lockup NMI watchdog timers monitor core spinlock iteration counters and trigger stack traces if a deadlock is detected.

---

## 4. Verification & Stress Testing Protocol

AI agents modifying concurrency mechanisms or lock-guarded code paths must execute verification:

1. **Native Test Runner**: Run `./run_sigma_tests.sh` to confirm thread synchronization correctness.
2. **Stress & Fuzzing Matrix**: Run `tests/stress_and_fuzz_tests.rs` to validate multi-core concurrent execution under high thread contention.

---

*Approved by the SigmaOS Concurrency & Core Kernel Architecture Committee.*
