# System Calls Abi Component Guide

> **Related portals:** [Init Supervisors & System Services](./Home.md) &bull; **Maintainer Agent:** [`SYSTEM_CALLS_ABI_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/SYSTEM_CALLS_ABI_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **System Calls Abi** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `System-Calls-Abi`
- **Subsystem Category**: Init Supervisors & System Services
- **Autonomous Agent**: [`SYSTEM_CALLS_ABI_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/SYSTEM_CALLS_ABI_AGENT.md)
- **Linux Inspiration**: Linux x86_64 syscall interface (fast syscall instruction), vDSO acceleration, POSIX.1-2017 standard compliance, and Linux ABI profile.
- **BSD Inspiration**: FreeBSD Linuxulator (binary compatibility layer running unmodified Linux binaries), NetBSD COMPAT_* emulation subsystem.

### Architectural Design
Coordinates the system call dispatch interface, register calling conventions, POSIX compatibility layer, and binary emulation for foreign operating systems.

**Current Implementation Status**: Core 80+ standard POSIX syscalls routed (read, write, open, close, mmap, fork, execve, exit). Linux ABI emulation translation layer is under development.
**Active Enhancement Objectives**: Complete Linux 6.x syscall ABI emulation table (allowing unmodified Linux ELF binaries to run natively on SigmaOS), POSIX signal masking, and epoll_create/epoll_wait.

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
# /etc/sigmaos/system-calls-abi.toml
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
$ sigma-ctl status system-calls-abi

# Inspect real-time log stream
$ journalctl -u system-calls-abi -f --no-pager
```

---

## 6. See Also
- [Main Portal: Init Supervisors & System Services](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
