# SigmaOS AI Agent Race Condition Management Guidelines

## 1. Executive Summary & Overview

In SigmaOS, multi-core, asynchronous, and highly concurrent execution environments require AI agents to strictly guard against **race conditions**, **data races**, **Time-of-Check to Time-of-Use (TOCTOU) hazards**, and **ABA corruption**. Operating across Ring-0 kernel space and sandboxed userland sub-systems, AI agents must enforce strict atomic memory ordering, lock hierarchy discipline, and lock-free concurrency safety.

This document establishes the official guidelines and architectural standards for AI agents managing race conditions and concurrent state synchronization in SigmaOS.

---

## 2. Race Condition Hazard Categories & Mitigations

AI agents must identify and prevent four primary classes of concurrency race hazards:

| Hazard Class | Root Cause | SigmaOS Subsystem Mitigation |
| :--- | :--- | :--- |
| **Data Race** | Concurrent unsynchronized read/write memory access | Atomic primitives (`AtomicBool`, `AtomicUsize`, `AtomicPtr`) with explicit `Ordering` |
| **TOCTOU Hazard** | Filesystem or capability state check separated from action | POSIX `*at` syscalls (`openat`, `renameat2`), atomic flags (`O_EXCL`, `O_NOFOLLOW`) |
| **ABA Problem** | Memory address recycled during lock-free CAS loop | Double-word atomic CAS, ABA generation counters, epoch-based reclamation |
| **Deadlock Race** | Circular lock dependency across concurrent threads | Strict global lock hierarchy, `IrqSafeSpinlock`, NMI watchdog lockup detection |

---

## 3. Kernel & Userland Synchronization Primitives

AI agents interfacing with shared state must select the appropriate synchronization primitive:

1. **`IrqSafeSpinlock` / `KernelSpinlock`**:
   - Must be used in Ring-0 interrupt handlers and short critical sections.
   - Disables local CPU interrupts prior to acquiring the spinlock to prevent interrupt-reentrancy deadlocks.
   - **Rule**: Spinlocks must never yield, sleep, or perform blocking I/O while held.

2. **`SimpleMutex` / Futex Wait Queues**:
   - Used in userland threads and long-running kernel task contexts where sleeping is permitted.
   - Incorporates futex wait queues (`sys_futex`) to park waiting threads without burning CPU cycles.

3. **Read-Copy-Update (RCU)**:
   - Used for read-heavy kernel data structures (e.g., VFS mount tables, routing tables, eBPF maps).
   - Readers access data concurrently without locks (`rcu_read_lock()`), while writers copy data, update pointers atomically, and defer freeing memory until a grace period elapses (`synchronize_rcu()`).

---

## 4. Atomic Memory Ordering Standards

When using atomic variables (`core::sync::atomic`), AI agents must explicitly specify memory ordering semantics:

- **`Ordering::Relaxed`**: Used only for independent counters (e.g., telemetry packet counters) where cross-thread synchronization is not required.
- **`Ordering::Acquire` / `Ordering::Release`**: Mandatory for publish-subscribe patterns, lock acquire/release, and flag synchronization.
  - `Release` ensures prior writes are visible before publishing the flag.
  - `Acquire` ensures subsequent reads see state published by the matching `Release`.
- **`Ordering::SeqCst`**: Reserved for global state changes requiring sequentially consistent global memory barriers across all CPU cores.

---

## 5. TOCTOU Filesystem Race Prevention

AI agents performing file operations must prevent TOCTOU exploitation:

1. **Relative Path Deskewing (`*at` Syscalls)**:
   - Use directory file descriptors (`openat`, `unlinkat`, `fstatat`) instead of string paths to lock down target parent directories.
2. **Atomic Creation & Replacement**:
   - Use `O_CREAT | O_EXCL` when opening new files to guarantee creation fails if the file exists.
   - Use `renameat2` with `RENAME_NOREPLACE` or `RENAME_EXCHANGE` for atomic file updates.
3. **Symbolic Link Hijack Prevention**:
   - Enforce `O_NOFOLLOW` and OpenBSD `unveil` restrictions to prevent symlink traversal attacks during path resolution.

---

## 6. Deadlock Prevention & Race Hazard Verification

1. **Global Lock Hierarchy Rule**:
   - If multiple locks must be acquired (e.g. `VfsLock -> PcbLock -> NetLock`), they must always be acquired in that exact deterministic order.
2. **Spinlock Timeout Guardrails**:
   - Kernel spinlocks execute loop counter bounds (e.g., 10,000,000 iterations) before triggering an NMI watchdog warning or panic recovery.
3. **Automated Stress & Concurrency Testing**:
   - AI agents modifying shared data paths must run `tests/stress_and_fuzz_tests.rs` to validate race-free execution under high multi-core load.

---

*Approved by the SigmaOS Concurrency & Core Kernel Architecture Committee.*
