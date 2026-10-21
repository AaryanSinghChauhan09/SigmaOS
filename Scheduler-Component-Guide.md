# Scheduler Component Guide

> **Related portals:** [Core Kernel & Hardware](./Home.md) &bull; **Maintainer Agent:** [`SCHEDULER_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/SCHEDULER_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Scheduler** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Scheduler`
- **Subsystem Category**: Core Kernel & Hardware
- **Autonomous Agent**: [`SCHEDULER_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/SCHEDULER_AGENT.md)
- **Linux Inspiration**: Linux CFS (Completely Fair Scheduler), EEVDF (Earliest Eligible Virtual Deadline First), SCHED_DEADLINE, SCHED_FIFO, and topology-aware core placement.
- **BSD Inspiration**: FreeBSD ULE scheduler (interactive heuristics and NUMA affinity), OpenBSD multi-queue scheduler, DragonFly BSD per-core LWKT queues.

### Architectural Design
The Scheduler manages multi-core thread scheduling, real-time priorities, preemptive multi-tasking, energy-aware placement, and CPU affinity mask enforcement.

**Current Implementation Status**: Priority-based round-robin and basic fair queuing implemented. Real-time deadline guarantees and EEVDF lag compensation require full verification.
**Active Enhancement Objectives**: Full EEVDF scheduling algorithm with positive/negative lag compensation, core-affinity topology caches, NUMA node-aware work stealing, and energy-aware DVFS frequency hooks.

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
# /etc/sigmaos/scheduler.toml
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
$ sigma-ctl status scheduler

# Inspect real-time log stream
$ journalctl -u scheduler -f --no-pager
```

---

## 6. See Also
- [Main Portal: Core Kernel & Hardware](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
