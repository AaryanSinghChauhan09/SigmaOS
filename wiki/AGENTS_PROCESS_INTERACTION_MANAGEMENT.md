# SigmaOS AI Agent Process Interaction Management Architecture & Governance Specification

**Document Version:** 1.0.0
**Target Systems:** SigmaOS Microkernel Core, Process Manager (`src/kernel/process.rs`), IPC Subsystem (`src/klib/ringbuf.rs`, zero-copy IPC channels), Signal Handler, Task Scheduler (`src/scheduler/scheduler.rs`), Security Capability Ring (`src/security/capability.rs`)
**Scope:** Operating System Process Inter-Process Communication (IPC), Process Synchronization, Signal Handlers, Capability-Gated Process Interactions, Shared Memory Management, and Autonomous AI Agent Governance Rules for Process Lifecycle Interactions.

---

## 1. Executive Summary & Core Directives

In SigmaOS, process interaction management governs how isolated user-space processes, microkernel driver shards, system services, and containerized runtimes communicate, synchronize, exchange signals, and share memory. AI agents operating on or within SigmaOS must strictly adhere to zero-dependency, capability-gated, lock-free process interaction principles inspired by OpenBSD `pledge`/`unveil`, FreeBSD Capsicum capability rights, and Linux `io_uring` zero-copy IPC.

### Core Directives for Autonomous AI Agents
1. **Zero-Implicit IPC:** Processes have zero default capability to send messages, signals, or inspect other processes unless explicitly granted capability rights via `CapabilityGate` (`src/security/capability.rs`).
2. **Lock-Free Fast-Path Interaction:** High-frequency inter-process communication must use lock-free ring buffers (`src/klib/ringbuf.rs`, `src/klib/ring_buffer.rs`) or shared memory rings rather than blocking microkernel syscall traps.
3. **Capability-Gated Signal Delivery:** Signal delivery (`SIGKILL`, `SIGTERM`, `SIGUSR1`, `SIGCHLD`) is mediated by capability checks. Unprivileged processes cannot signal processes outside their capability domain or namespace.
4. **Deterministic Lifecycle Synchronization:** Process wait queues, parent-child relationships, and exit status collections must avoid zombie/orphan process leaks through deterministic wait-group cleanup in `src/kernel/process.rs`.
5. **No-Alloc IPC Transfers:** IPC payload serialization must utilize stack-allocated or static zero-copy buffers (`UutilsCoreutilsZeroCopyBuffer`) avoiding dynamic memory fragmentation in high-throughput paths.

---

## 2. Process Interaction Subsystems & Architecture

```
+-------------------------------------------------------------------------------+
|                         SigmaOS User Space / Process Ring                      |
|                                                                               |
|   +-----------------------+                    +--------------------------+   |
|   |  Process A (Sender)   |                    |   Process B (Receiver)   |   |
|   +-----------+-----------+                    +------------+-------------+   |
|               |                                             ^                 |
+---------------|---------------------------------------------|-----------------+
                | Capability Verification                     | Signal / Msg
                v                                             |
+-------------------------------------------------------------+-----------------+
|                       SigmaOS Kernel / Security Ring                          |
|                                                                               |
|   +-----------------------------------------------------------------------+   |
|   |          Capability Gate Verification (`Capability::ProcessExec`)     |   |
|   +-----------------------------------+-----------------------------------+   |
|                                       |                                       |
|               +-----------------------+-----------------------+               |
|               v                                               v               |
|   +-----------------------+                       +-----------------------+   |
|   | Lock-Free IPC Ring    |                       | Signal & Event Queue  |   |
|   | (Shared Memory/Ring)  |                       | (Async Notification)  |   |
|   +-----------------------+                       +-----------------------+   |
+-------------------------------------------------------------------------------+
```

---

## 3. IPC & Message Passing Governance

### 3.1 Lock-Free Ring Buffer IPC Channels
* All microkernel service calls and daemon-to-daemon communications must be routed through lock-free atomic ring buffers (`RingBuf<T>` or `HeapRingBuffer<T>`).
* Lock contention during IPC is strictly prohibited in real-time execution modes (`BORE` and `EEVDF` kernel schedulers).

### 3.2 Shared Memory Extents & Page Loan Interactions
* Shared memory pages exchanged between processes must be managed via zero-copy page loans (`uvm_page_loans` in `src/klib/uvm.rs`).
* Shared memory mapping requires `Permission::MemDirectAccess` capability verification before physical frame mapping into the target virtual memory map (`pmap`).

### 3.3 Zero-Copy Signal & Notification Queues
* Signal dispatching must use atomic bitmasks for standard signals (`1..32`) and lock-free queues for payload-carrying real-time signals (`SIGRTMIN..SIGRTMAX`).

---

## 4. Capability-Gated Process Isolation Rules

1. **Process Inspection:** Reading process state via `/proc` or system telemetry requires `Permission::ProcessControl` capability.
2. **Process Tracing & Debugging:** `ptrace` equivalent debugging attached to a running process requires explicit administrative capability and OpenBSD `pledge("stdioproc")` permission.
3. **Parent-Child IPC Scoping:** Child processes inherit restricted subsets of parent capabilities, enforcing strict least-privilege propagation.

---

## 5. Verification & Testing Requirements

1. **IPC Stress Testing:** Execute `tests/test_stress_fuzz_bench.py` and zero-copy IPC channel unit tests to verify throughput and lock freedom.
2. **Capability Violation Auditing:** Ensure that unauthorized signal or message attempts return `PermissionDenied` errors and log audit records into `ChainedAuditTrailLedger`.
3. **Full System Verification:** Run `./run_sigma_tests.sh` to ensure all 224+ Rust unit tests and Python integration tests pass cleanly.

---

*End of Specification.*
