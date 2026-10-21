# Cgroups Resource Governor Component Guide

> **Related portals:** [Init Supervisors & System Services](./Home.md) &bull; **Maintainer Agent:** [`CGROUPS_RESOURCE_GOVERNOR_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CGROUPS_RESOURCE_GOVERNOR_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Cgroups Resource Governor** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Cgroups-Resource-Governor`
- **Subsystem Category**: Init Supervisors & System Services
- **Autonomous Agent**: [`CGROUPS_RESOURCE_GOVERNOR_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CGROUPS_RESOURCE_GOVERNOR_AGENT.md)
- **Linux Inspiration**: Linux cgroups v2 unified hierarchy, cpu.weight/cpu.max controllers, memory.high/memory.max limits, io.weight latency protection, and psi pressure stall information.
- **BSD Inspiration**: FreeBSD rctl (resource limits) and hierarchical jail rules, DragonFly BSD per-core CPU resource partitions.

### Architectural Design
Governs unified hierarchical resource control (CPU bandwidth, memory limits, block IO weight, process limits) inspired by Linux cgroups v2 and FreeBSD rctl.

**Current Implementation Status**: Hierarchical cgroup tree structure and basic memory quota accounting implemented. PSI pressure stall info and CPU bandwidth throttling require completion.
**Active Enhancement Objectives**: Complete CPU CFS-bandwidth quota throttling, PSI (Pressure Stall Information) calculation for CPU/memory/IO, and OOM killer targeting leaf cgroups first.

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
# /etc/sigmaos/cgroups-resource-governor.toml
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
$ sigma-ctl status cgroups-resource-governor

# Inspect real-time log stream
$ journalctl -u cgroups-resource-governor -f --no-pager
```

---

## 6. See Also
- [Main Portal: Init Supervisors & System Services](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
