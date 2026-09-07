# AI Agent Concurrent Process Management Specification for SigmaOS

This document provides specifications for AI agents developing, managing, and synchronizing concurrent processes, threads, and lock primitives across **SigmaOS**.

---

## 1. Concurrency Models in SigmaOS

SigmaOS combines multi-paradigm concurrency mechanisms across user space, kernel space, and compatibility shims:

1. **Capability-Based Threading** (`src/runtime/threading/thread.rs`):
   - Capability-gated thread creation, state management, priority assignment, and stack allocation.
   - Non-std context switching using register frames (`ThreadContext`).

2. **Fast Userspace Futex Engine** (`src/kernel/linux_bsd_innovations.rs` & `src/kernel/unix_primitives.rs`):
   - `LinuxFutexEngine` provides atomic wait/wake/requeue semantics for high-performance userland synchronization.

3. **SMP Multi-Queue Process Schedulers**:
   - `sched_ext` dynamic eBPF schedulers (`ScxBpfland`, `ScxLavd`, `ScxCachyBore`).
   - FreeBSD ULE dual-queue scheduler (`src/kernel/scheduler.rs`).
   - CachyOS BORE burst scheduler (`src/kernel/bore.rs`).

---

## 2. Synchronization Primitives Reference

| Synchronization Primitive | Implementation File | Key Features & Use Cases |
| :--- | :--- | :--- |
| `Mutex` | `src/runtime/threading/thread.rs` | Capability-checked atomic bool lock with thread ownership tracking |
| `Semaphore` | `src/runtime/threading/thread.rs` | Counting semaphore with capability checks and atomic wait/signal |
| `RwLock` | `src/runtime/threading/thread.rs` | Reader-writer lock supporting multiple readers or single writer |
| `LinuxFutexEngine` | `src/kernel/linux_bsd_innovations.rs` | Linux `sys_futex` parity with bucketed wait queues and atomic checks |
| `FineGrainedSpinlock` | `src/kernel/core/sovereign_scheduler.rs` | Contention-tracking spinlock for kernel critical sections |
| `TicketSpinlock` | `src/kernel/classic_os.rs` | Ticket spinlock with exponential backoff for high CPU core counts |
| `RwSemaphore` | `src/kernel/linux_parity.rs` | Kernel reader-writer semaphore for VFS and subsystem locks |
| `FastMutex` / `GuardedMutex` | `src/kernel/wdk_core.rs` | WDK driver synchronization with IRQL/APC level disabling |

---

## 3. Agent Integration Guidelines

When writing or modifying code that executes concurrently in SigmaOS:

1. **Always Verify Capabilities**:
   Ensure thread creation and synchronization checks respect `ThreadCapability`, `MutexCapability`, `SemaphoreCapability`, or `RwLockCapability`.

2. **Avoid Unbounded Spinning**:
   In spinlock loops, always include `core::hint::spin_loop()` or yield to the scheduler to prevent hardware CPU execution pipeline stalls.

3. **Avoid Race Conditions in Memory**:
   For userspace locks utilizing futexes (`LinuxFutexEngine`), ensure initial value check (`*uaddr == val`) is executed atomically before enqueuing waiter threads.

---

## 4. Verification

Execute tests for concurrent process and lock primitives:

```bash
# Run unit tests
./run_sigma_tests.sh

# Run system integration suite
pytest tests/test_integration_system.py
```
