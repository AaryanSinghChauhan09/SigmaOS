# Kernel Core Component Guide

> **Related portals:** [Core Kernel & Hardware](./Home.md) &bull; **Maintainer Agent:** [`KERNEL_CORE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/KERNEL_CORE_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [Technical Specifications & Protocols](#2-technical-specifications--protocols)
3. [Multi-Language Implementations](#3-multi-language-implementations)
4. [Configuration & Usage](#4-configuration--usage)
5. [Diagnostics & Troubleshooting](#5-diagnostics--troubleshooting)
6. [See Also](#6-see-also)

---

## 1. Overview & Architecture
The **Kernel Core** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Kernel-Core`
- **Subsystem Category**: Core Kernel & Hardware
- **Autonomous Agent**: [`KERNEL_CORE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/KERNEL_CORE_AGENT.md)
- **Linux Inspiration**: Linux monolithic kernel (arch/x86, kernel/sched), dynamic kernel modules, per-CPU data areas, jump labels, and scalable lockless synchronization (RCU).
- **BSD Inspiration**: FreeBSD kernel modularity, OpenBSD secure micro-modular design with W^X enforcement, NetBSD rump kernels, and DragonFly BSD LWKT (Light Weight Kernel Threads).

### Architectural Design
The Kernel Core component orchestrates CPU initialization, interrupt dispatching, capability security, architecture-specific traps, and task context switching in pure #![no_std] Rust.

**Current Implementation Status**: Basic x86_64 CPU bootstrap, GDT/IDT tables, and early serial console are in place. SMP AP boot, dynamic trap handlers, and advanced RCU locks are under active development.
**Active Enhancement Objectives**: Complete multi-core SMP initialization, lockless read-copy-update (RCU) synchronization primitives, kernel address space layout randomization (KASLR), and dynamic live-patching infrastructure.

---

## 2. Technical Specifications & Protocols
- **Memory Safety**: Clean abstraction layers preventing memory corruption and buffer overflow exploits.
- **Concurrency Model**: Fine-grained thread synchronization using standard mutexes, atomics, and channels.
- **Fault Isolation**: Sandboxed process model ensuring failures do not compromise other subsystems.

---

## 3. Multi-Language Implementations
- **Rust Engine**: Primary safe abstractions, state machines, and system integration.
- **Zig Accelerator**: Zero-allocation high-throughput data buffers and SIMD mathematical routines.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

### Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all restrictions against standard library usage. Standard libraries (`std::sync::Arc`, `std::sync::Mutex`, `std::collections`, `libc`, and dynamic allocators) are authorized and actively utilized as required.

---

## 4. Configuration & Usage

```toml
# /etc/sigmaos/kernel-core.toml
[subsystem]
enabled = true
log_level = "info"
performance_mode = "balanced"
sync_interval_ms = 1000
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem daemon status
$ sigma-ctl status kernel-core

# Inspect real-time log stream
$ journalctl -u kernel-core -f --no-pager
```

---

## 6. See Also
- [Main Portal: Core Kernel & Hardware](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
