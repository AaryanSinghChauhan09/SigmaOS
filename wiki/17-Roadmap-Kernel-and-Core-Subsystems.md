# SigmaOS Future Development Roadmap: Kernel & Core Subsystems

This roadmap details the architectural targets, milestone goals, and component designs for the future evolution of the SigmaOS microkernel and core executive subsystems, taking direct architectural inspiration from the most advanced features of modern Linux and BSD operating systems.

---

## 1. Executive Summary & Core Philosophy

SigmaOS features a zero-dependency, bare-metal microkernel written in Safe Rust (`#![no_std]`), Zig, and Nim. To surpass existing operating systems in performance, security, and predictability, the core kernel subsystem incorporates high-efficiency algorithms from Linux, FreeBSD, OpenBSD, DragonFly BSD, and illumos.

```
+----------------------------------------------------------------------------------------------------+
|                             SIGMAOS CORE KERNEL ARCHITECTURE ROADMAP                               |
+----------------------------------------------------------------------------------------------------+
|  [Linux EEVDF & sched_ext]   |   [FreeBSD Capsicum & Jails]   |   [OpenBSD KARL & Pledge/Unveil]   |
+----------------------------------------------------------------------------------------------------+
|  [DragonFly HAMMER2 PFS]     |   [illumos DTrace & SDT]       |   [NuttX RT-Lane Deadline Scheduler] |
+----------------------------------------------------------------------------------------------------+
|                         SIGMAOS BARE-METAL RUST #![no_std] EXECUTIVE KERNEL                        |
+----------------------------------------------------------------------------------------------------+
```

---

## 2. Linux-Inspired Kernel Innovations

### 2.1 EEVDF (Earliest Eligible Virtual Deadline First) Scheduler
- **Inspiration**: Linux 6.6+ CFS replacement (Peter Zijlstra's EEVDF scheduler).
- **Target Architecture**:
  - Replaces traditional virtual runtime (`vruntime`) with virtual deadline (`vdeadline`) computation.
  - Guarantees latency bounds for interactive and real-time tasks without sacrificing throughput for background compute jobs.
  - Implements dynamic lag balancing across multi-socket NUMA nodes with sub-50ns task migration overhead.
- **Milestones**:
  - **Phase 1**: Bare-metal Rust `#![no_std]` EEVDF virtual time and eligibility tree data structures.
  - **Phase 2**: Multi-core SMP runqueue lock-free CAS lag synchronization.
  - **Phase 3**: `sched_ext` (eBPF extensible scheduler) interface for dynamic user-defined scheduling policies.

### 2.2 Dynamic Kernel LSM & eBPF Engine
- **Inspiration**: Linux BPF LSM (Linux Security Modules) and Landlock.
- **Target Architecture**:
  - Native zero-dependency eBPF JIT compiler for x86_64, AArch64, and RISC-V 64.
  - Granular, programmable LSM hooks attached to system call entry points, VFS path operations, socket allocation, and process lifecycle events.

---

## 3. FreeBSD-Inspired Subsystem Innovations

### 3.1 Capsicum Capability-Based Security
- **Inspiration**: FreeBSD Capsicum framework.
- **Target Architecture**:
  - File descriptors function as explicit capability tokens (`CapRights`).
  - System call execution in Capability Mode prevents access to global namespaces (VFS, IPC, network PID spaces) unless explicitly granted by capability rights.
- **Milestones**:
  - **Phase 1**: Enforce `cap_enter()` capability mode per process descriptor.
  - **Phase 2**: Fine-grained capability rights verification on all VFS and socket file descriptors.

### 3.2 Lightweight Virtualization: Sovereign Jails
- **Inspiration**: FreeBSD Jails & RACCT/RCTL.
- **Target Architecture**:
  - Zero-overhead kernel container isolation dividing network interface structures (`VNET`), IPC semaphores, process tables, and mount points.
  - Resource limits enforcement (CPU percentage, memory limits, I/O bandwidth, thread count) using lock-free atomic counters.

---

## 4. OpenBSD-Inspired Hardening & Isolation

### 4.1 KARL (Kernel Address Randomized Link)
- **Inspiration**: OpenBSD KARL kernel re-linking on boot.
- **Target Architecture**:
  - Re-orders kernel module functions, executive structures, and syscall entry gates randomly during early boot.
  - Prevents Return-Oriented Programming (ROP) and Jump-Oriented Programming (JOP) exploits by ensuring unique function offsets on every boot cycle.

### 4.2 Pledge & Unveil Path Isolation
- **Inspiration**: OpenBSD `pledge(2)` and `unveil(2)`.
- **Target Architecture**:
  - `pledge`: Restricts system call capabilities to specified promise sets (e.g. `stdio`, `rpath`, `wpath`, `inet`, `dns`).
  - `unveil`: Restricts process VFS visibility exclusively to explicitly permitted path subsets with granular read (`r`), write (`w`), execute (`x`), and create (`c`) permissions.

---

## 5. Subsystem Convergence & Compatibility Matrix

| Feature | Inspired By | SigmaOS Component | Target Latency / Goal | Status |
| :--- | :--- | :--- | :--- | :--- |
| **EEVDF Scheduler** | Linux Kernel 6.6+ | `src/kernel/scheduler.rs` | Sub-50ns Task Switch | Active Development |
| **Capsicum Rights** | FreeBSD | `src/security/capsicum.rs` | Zero Global Syscall Bypass | Fully Implemented |
| **KARL Randomization** | OpenBSD | `src/kernel/karl.rs` | 100% Unique Function Offsets | Active Development |
| **Pledge / Unveil** | OpenBSD | `src/security/pledge.rs` | Sub-10ns Perms Audit | Fully Implemented |
| **Sovereign Jails** | FreeBSD / illumos | `src/kernel/jails.rs` | Zero-Overhead Containers | Active Development |

---

## 6. Implementation & Verification Protocol

All kernel additions must strictly adhere to the following rules:
1. **Zero External Dependencies**: Implemented natively in Safe Rust `#![no_std]` without external crates.
2. **Deterministic Verification**: Verified through `./run_sigma_tests.sh` and kernel unit tests.
3. **Lock-Free Concurrency**: Atomic CAS structures for SMP concurrency without spinlock contention.
