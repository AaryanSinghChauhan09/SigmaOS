# AI Agent Concurrent Process Management Architecture in SigmaOS

This document details the concurrent process management, multithreading primitives, synchronization primitives, lock-free data structures, and cross-subsystem scheduling mechanisms across **SigmaOS**.

---

## 1. Threading Primitives & Capability Access Control

In `src/runtime/threading/thread.rs`, SigmaOS implements custom capability-backed threading without relying on standard library thread assumptions:

1. **`ThreadContext`**:
   - Represents the CPU hardware register context (`rax`-`r15`, `rip`, `rflags`, `cs`-`gs`, `mxcsr`, `fcw`, `fsw`).
   - Default `rflags` set to `0x202` (Interrupt Enable active), `mxcsr` set to `0x1F80` (SSE exceptions masked), `fcw` set to `0x037F`.

2. **`Thread` Control Block**:
   - Stores thread state (`Uninitialized`, `Ready`, `Running`, `Blocked`, `Terminated`) managed via atomic operations (`AtomicUsize`).
   - Capability flags (`ThreadCapability`): `can_create`, `can_terminate`, `can_suspend`, `can_resume`, `can_set_priority`.

3. **`ThreadManager`**:
   - Fixed array manager (`[Option<NonNull<Thread>>; 256]`) providing safe lookup and capability enforcement for creation, priority management, and thread termination.

---

## 2. Kernel Synchronization Primitives

SigmaOS supports multiple synchronization primitives tailored for microkernel performance, fast userspace locking, and subsystem compatibility:

### 2.1 Fast Userspace Locks (`sys_futex`)
- **`LinuxFutexEngine`** (`src/kernel/linux_bsd_innovations.rs` & `src/kernel/unix_primitives.rs`):
  - Bucketed `HashMap<u64, Vec<FutexWaiter>>` for zero-allocation waiter enqueuing.
  - Supports `FUTEX_WAIT`, `FUTEX_WAKE`, `FUTEX_REQUEUE`, and `FUTEX_WAKE_OP`.
  - Atomically validates memory before sleeping to prevent missed wakeups (`EAGAIN` returned on memory state change).

### 2.2 Capability-Based Synchronization (`src/runtime/threading/thread.rs`)
- **`Mutex`**: Uses `AtomicBool` locked status and `AtomicUsize` owner ID guarded by `MutexCapability`. Spin loops with `core::hint::spin_loop()`.
- **`Semaphore`**: Atomic `count` and `max_count` guarded by `SemaphoreCapability` (`can_wait`, `can_signal`).
- **`RwLock`**: Reader-writer lock tracking reader counts (`AtomicUsize`) and writer active status (`AtomicBool`) guarded by `RwLockCapability`.

### 2.3 Kernel Fine-Grained & Ticket Spinlocks
- **`FineGrainedSpinlock`** (`src/kernel/core/sovereign_scheduler.rs`):
  - Tracks lock contention stats (acquisitions, contention count, spin loops) for FreeBSD `mtx` and Linux `spinlock_t` parity.
- **`TicketSpinlock`** (`src/kernel/classic_os.rs`):
  - Fair ticket allocation spinlock utilizing exponential backoff during high lock contention.

### 2.4 Reader-Writer Semaphores
- **`RwSemaphore`** (`src/kernel/linux_parity.rs`):
  - Linux `rw_semaphore` parity supporting shared read acquisition and exclusive write acquisition.

### 2.5 Windows Driver Kit (WDK) Executive Locks (`src/kernel/wdk_core.rs`)
- **`MutexObject`**: Kernel dispatch object supporting thread ownership validation.
- **`FastMutex`**: Acquires lock while raising IRQL to `APC_LEVEL` to prevent thread suspension.
- **`GuardedMutex`**: Enters a guarded region disabling kernel Asynchronous Procedure Calls (APCs).

---

## 3. Concurrent Process Scheduling & Execution

Concurrent task execution in SigmaOS is governed by hybrid multi-model schedulers:

1. **`sched_ext` Extensible BPF Schedulers** (`src/distro/sovereign_nextgen_distro_leap.rs`):
   - Offloads task dispatch decisions to eBPF programs (`ScxBpfland`, `ScxLavd`, `ScxCachyBore`, `ScxCentral`).
2. **CachyOS BORE (Burst-Oriented Response Enhancer)** (`src/kernel/bore.rs`):
   - Dynamically balances interactive tasks vs batch processing based on burst time tracking.
3. **FreeBSD ULE Scheduler** (`src/kernel/scheduler.rs`):
   - Maintains dual interactive and batch queues across multi-core SMP nodes.
4. **POSIX Real-Time Preemption** (`NuttxRealtimeTaskGovernor`):
   - Preemption-threshold scheduling to bound real-time priority inversion.

---

## 4. Subsystem Cross-Concurrency & Verification

All 46 SigmaOS subsystems dispatch concurrent operations using the unified `SovereignUniversalDistroBridge`:

```rust
use crate::distro::linux_bsd_inspirations::{SovereignUniversalDistroBridge, UniversalSubsystemMode};

let bridge = SovereignUniversalDistroBridge::new(UniversalSubsystemMode::LinuxArch);
let result = bridge.dispatch_cross_subsystem_operation("process", "spawn_concurrent_worker");
assert!(result);
```

### Testing Concurrency & Threading

Run kernel unit tests for threading and primitives:

```bash
# Run standalone tests
./run_sigma_tests.sh

# Run python integration test suite
pytest tests/test_integration_system.py
```
