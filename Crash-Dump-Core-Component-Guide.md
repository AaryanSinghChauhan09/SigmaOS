# Crash Dump Core Component Guide

> **Related portals:** [Storage, Volumes & Filesystems](./Home.md) &bull; **Maintainer Agent:** [`CRASH_DUMP_CORE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CRASH_DUMP_CORE_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Crash Dump Core** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Crash-Dump-Core`
- **Subsystem Category**: Storage, Volumes & Filesystems
- **Autonomous Agent**: [`CRASH_DUMP_CORE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CRASH_DUMP_CORE_AGENT.md)
- **Linux Inspiration**: Linux kdump/kexec secondary kernel crash collection, systemd-coredump, and crash utility.
- **BSD Inspiration**: FreeBSD savecore and minidump crash architecture, OpenBSD dmesg crash buffers.

### Architectural Design
Captures kernel panic state, registers, stack traces, and physical memory pages into crash dump images for post-mortem automated triage.

**Current Implementation Status**: Early kernel panic handler, register frame dumper, and serial stack trace printer implemented. Persistent disk crash dump capture in progress.
**Active Enhancement Objectives**: Compressed minidump physical memory writing to dedicated swap/raw partitions, symbol demangling, and automated backtrace JSON generator.

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
# /etc/sigmaos/crash-dump-core.toml
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
$ sigma-ctl status crash-dump-core

# Inspect real-time log stream
$ journalctl -u crash-dump-core -f --no-pager
```

---

## 6. See Also
- [Main Portal: Storage, Volumes & Filesystems](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
