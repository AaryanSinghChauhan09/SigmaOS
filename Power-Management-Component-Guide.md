# Power Management Component Guide

> **Related portals:** [Package Management & Maintenance](./Home.md) &bull; **Maintainer Agent:** [`POWER_MANAGEMENT_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/POWER_MANAGEMENT_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Power Management** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Power-Management`
- **Subsystem Category**: Package Management & Maintenance
- **Autonomous Agent**: [`POWER_MANAGEMENT_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/POWER_MANAGEMENT_AGENT.md)
- **Linux Inspiration**: Linux ACPI subsystem, cpufreq/intel_pstate governors, TLP power management daemon, thermald dynamic cooling.
- **BSD Inspiration**: FreeBSD acpi driver and powerd daemon, OpenBSD apm and sensor framework (sysctl hw.sensors).

### Architectural Design
Governs ACPI hardware tables, CPU P-states/C-states, dynamic frequency scaling (DVFS), battery telemetry, thermal zones, and suspend-to-RAM (S3/S0ix).

**Current Implementation Status**: ACPI RSDP/MADT parsing, basic CPU frequency throttling, and battery state querying functional. Full sleep state transitions (S3/S4) and thermald cooling need work.
**Active Enhancement Objectives**: Complete ACPI AML bytecode interpreter, S3 suspend-to-RAM and S0ix modern standby, per-core energy-aware frequency governors, and thermal throttling policies.

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
# /etc/sigmaos/power-management.toml
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
$ sigma-ctl status power-management

# Inspect real-time log stream
$ journalctl -u power-management -f --no-pager
```

---

## 6. See Also
- [Main Portal: Package Management & Maintenance](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
