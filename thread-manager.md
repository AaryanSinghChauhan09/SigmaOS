# SigmaOS Thread Manager Specification & Subsystem Contract

## Executive Overview

The SigmaOS Thread Manager (`src/kernel/thread.rs`) defines the canonical thread lifecycle, context state, scheduling properties, wait-channel wakeup safety, signal masks, and stack bounds enforcement across kernel and user execution modes.

This document serves as the formal specification for the Thread Manager subsystem contract under Phase 1 and Phase 2 of the SigmaOS Thread Manager Development Roadmap.

---

## Subsystem Contract & Guarantees

### 1. Thread Lifecycle States

Every thread in SigmaOS is in exactly one of the following explicit states:

- **`New`**: Newly created thread object; metadata initialized, not yet inserted into scheduler run queue.
- **`Ready`**: Runnable state; present in or eligible for insertion into a CPU run queue.
- **`Running`**: Currently executing on a CPU core.
- **`Blocked`**: Sleeping on a wait channel (`WaitChannelId`), timer, or I/O primitive.
- **`Suspended`**: Execution paused via signal (`SIGSTOP`) or administrative control.
- **`Zombie`**: Execution terminated, exit status saved, waiting for parent process or joiner to reap.
- **`Terminated`**: Resources fully reclaimed and state destroyed; ready for memory deallocation.

#### Legal State Transition Matrix

| From State | Allowed Target States | Trigger Event |
|---|---|---|
| `New` | `Ready`, `Terminated` | Scheduler enqueue or creation abort |
| `Ready` | `Running`, `Suspended`, `Zombie` | Context switch, signal suspend, early exit |
| `Running` | `Ready`, `Blocked`, `Suspended`, `Zombie` | Timer preemption / yield, wait block, signal, exit |
| `Blocked` | `Ready`, `Suspended`, `Zombie` | Wait channel wake / timeout, signal interruption, fatal signal |
| `Suspended` | `Ready`, `Zombie` | `SIGCONT` resume, process termination |
| `Zombie` | `Terminated` | `join()`, parent wait, or automatic reap for detached threads |
| `Terminated` | *(None)* | Terminal state |

Direct jumps outside this transition table (e.g. `New` -> `Running`, `Blocked` -> `Running`) are strictly prohibited and return an error (`Err("invalid thread state transition")`).

---

### 2. Ownership Categories

- **`Kernel`**: Pure kernel-space thread without user-mode address space or page tables.
- **`User`**: User-space thread bound to a process address space.
- **`Process`**: Primary thread of a process group lifecycle.
- **`Detached`**: Independent thread whose resources are automatically reclaimed upon exit without requiring explicit `join()`.

---

### 3. Core Technical Invariants & Guarantees

1. **Preemption & CPU Context**: Saved `CpuContext` preserves all x86_64 general-purpose registers (`rax`..`r15`), instruction pointer (`rip`), stack pointer (`rsp`), base pointer (`rbp`), flags (`rflags`), and segment selectors (`cs`, `ss`).
2. **Stack Bounds & Guard Zones**: `StackMeta` maintains explicit base address, stack size, and guard size. Stack pointer validation functions `contains(ptr)` and `is_in_guard_zone(ptr)` detect stack overflow or guard violation prior to execution.
3. **Priority & Affinity**: Priority is bounded between `0` (highest priority) and `139` (lowest priority). `CpuAffinity` bitmask governs multi-core scheduling placement up to 64 CPUs.
4. **Wait-Channel Wakeup Safety**: Blocking on a wait channel sets `state = Blocked` and records `wait_channel = Some(wchan)`. Wakeup operations acquire state guards to prevent lost wakeups between concurrent timeouts, signal delivery, and wait channel wake calls.
5. **Lock-Order Tracker**: `locks_held` tracks active lock memory addresses acquired by the thread to enforce strict lock ordering and detect lock inversion warnings.

---

## Current Status vs. Modeled Capabilities

| Feature | Implementation Status | Evidence / Location |
|---|---|---|
| Canonical `Thread` object & metadata | **Unit-Tested** | `src/kernel/thread.rs` |
| `ThreadState` legal transition guard | **Unit-Tested** | `ThreadState::is_valid_transition` |
| `ThreadManager` wait-channel & signals | **Unit-Tested** | `ThreadManager::wake_wait_channel`, `deliver_signal` |
| Stack guard zone checking | **Unit-Tested** | `StackMeta::is_in_guard_zone` |
| Lock order tracking | **Unit-Tested** | `Thread::acquire_lock`, `release_lock` |
| Preemptive CPU Run-Queue Dispatch | *Modeled / Pending Phase 3* | Target for scheduler integration PRs |
| Hardware Interrupt Context Switching | *Modeled / Pending Phase 3* | Target for x86_64 IDT/APIC integration |

---

## Verification Procedures

To run the standalone Thread Manager unit test suite:

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

To run the complete SigmaOS test suite:

```bash
./run_sigma_tests.sh
```
