# AI Agent Guidelines: Mutual Exclusion, Monitors, Peterson's Algorithm & Operations Management in SigmaOS

## Overview
This document defines operational guidelines and architectural directives for AI agents working on **Mutual Exclusion, Monitors, Peterson's Algorithm, OS Concurrency Principles, and Operations Management** in SigmaOS. It specifies software and hardware-assisted synchronization primitives, high-level monitors with condition variables, software-based mutual exclusion (Peterson's and Dekker's algorithms), classical OS concurrency solutions (Banker's deadlock avoidance, Readers-Writers locks, Priority Inheritance Protocol), and service operations management across `#![no_std]` runtime environments in SigmaOS.

---

## 1. Concurrency, Synchronization & Operations Subsystems

AI agents interacting with process synchronization, mutual exclusion, or system operations management in SigmaOS must interface with the following core subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Microkernel Spinlocks & Atomic CAS** | `src/system/state.rs`, `src/klib/ringbuf.rs` | Spinlock mutexes (`SpinMutex`) and atomic CAS multi-producer/consumer ring buffers (`AtomicUsize` locks). |
| **Fine-Grained Scheduler Spinlocks** | `src/kernel/core/sovereign_scheduler.rs` | Fine-grained spinlocks with lock contention telemetry tracking (`FreeBSD mtx` / `Linux spinlock_t` parity). |
| **Bounded Buffer Monitor Engine** | `src/kernel/linux_bsd_innovations.rs` | High-level monitor abstraction (`BoundedBufferMonitor`) encapsulating shared data, mutexes, and condition variables (`cond_var`). |
| **IPC Semaphores & Namespaces** | `src/ipc/ipc_namespace.rs` | Counting semaphores (`SemaphoreObject`), `P()` / `V()` operations, and IPC namespace registry management. |
| **Banker's Deadlock Avoidance** | `src/kernel/scheduler.rs`, `src/security/hardening.rs` | Banker's Algorithm (`BankersDeadlockAvoidanceEngine`) evaluating resource allocation matrices for safe state execution. |
| **Operations Management & Supervision** | `src/kernel/watchdog.rs`, `src/tools/sigmatools.rs` | Hardware environmental monitor (`HardwareMonitor`), devlink health monitors, and s6-style service init supervision. |

---

## 2. Architectural Rules & Synchronization Invariants

AI agents must enforce the following 4 core invariants when implementing or auditing synchronization and operations management:

```
+-------------------------------------------------------------------------+
|              SIGMAOS MUTUAL EXCLUSION & MONITORS ARCHITECTURE           |
+-------------------------------------------------------------------------+
                                     |
         +---------------------------+---------------------------+
         |                           |                           |
         v                           v                           v
  [Hardware Spinlocks & CAS]   [High-Level Monitors]      [Peterson's Software Lock]
  • AtomicBool / AtomicUsize   • Encapsulated Mutex State • flag: [AtomicBool; 2]
  • Ordering::SeqCst Fences    • Condition Variables      • turn: AtomicUsize
  • Contention Telemetry       • Producer/Consumer Wait   • SeqCst Memory Barriers
```

### 1. Peterson's Algorithm Memory Barrier Rule
- **Invariant:** When implementing software-based 2-process mutual exclusion via Peterson's algorithm (`flag[i] = true; turn = j;`), AI agents MUST issue explicit sequential consistency memory barriers (`core::sync::atomic::fence(Ordering::SeqCst)`) between flag writes and turn reads.
- **Rationale:** Modern out-of-order processors (x86_64, ARM64) reorder store-after-load operations. Memory fences guarantee that flag writes are globally visible before evaluating `turn`.

```rust
// Peterson's Algorithm Entry Protocol in SigmaOS
pub fn peterson_lock_enter(process_id: usize, flag: &[AtomicBool; 2], turn: &AtomicUsize) {
    let other = 1 - process_id;
    flag[process_id].store(true, Ordering::SeqCst);
    turn.store(other, Ordering::SeqCst);

    // Hardware memory fence enforcing global visibility
    core::sync::atomic::fence(Ordering::SeqCst);

    while flag[other].load(Ordering::SeqCst) && turn.load(Ordering::SeqCst) == other {
        core::hint::spin_loop(); // Prevent CPU pipeline stall during spin wait
    }
}
```

### 2. High-Level Monitor Encapsulation
- **Invariant:** Monitors (`BoundedBufferMonitor`) MUST encapsulate shared state variables, lock primitives, and condition queues within a single protected module structure.
- **Rule:** Direct external access to inner monitor buffer arrays without acquiring the monitor's lock is strictly prohibited.

### 3. Deadlock Avoidance via Banker's Safety Matrix
- **Invariant:** Resource allocation managers evaluating requests from multiple processes MUST invoke `BankerAlgorithm::is_safe_state(available, max, allocation, need)` prior to granting resource claims.
- **Rule:** If granting a claim leaves the system in an unsafe state, the requesting thread MUST be placed in `ProcessState::BlockedWaiting`.

### 4. Zero Ring 0 Panic Rule
- Synchronization primitives, semaphores, and monitor operations must return explicit `Result<(), &'static str>` or status codes instead of triggering unhandled kernel panics.

---

## 3. Verification & Testing Protocols

Every mutual exclusion and operations management change must be verified via standalone unit tests and integrated test execution:

```bash
# Run standalone unit test for kernel innovations and monitors
rustc --test --edition 2021 --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# Run full test suite
./run_sigma_tests.sh
```

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes touching mutual exclusion, monitors, or operations management:

- [ ] Does Peterson's algorithm implementation include `Ordering::SeqCst` memory fences?
- [ ] Are monitors fully encapsulating shared state with condition variable wait queues?
- [ ] Does resource allocation pass Banker's algorithm safe-state verification?
- [ ] Have all unit tests passed with 0 failures in `./run_sigma_tests.sh`?
