# Thermal Cooling Component Guide

> **Related portals:** [Core Kernel & Hardware](./Home.md) &bull; **Maintainer Agent:** [`THERMAL_COOLING_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/THERMAL_COOLING_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Thermal Cooling** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Thermal-Cooling`
- **Subsystem Category**: Core Kernel & Hardware
- **Autonomous Agent**: [`THERMAL_COOLING_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/THERMAL_COOLING_AGENT.md)
- **Linux Inspiration**: Linux thermal subsystem, thermald (Intel Thermal Daemon), and lm-sensors fan control.
- **BSD Inspiration**: OpenBSD sysctl hw.sensors thermal monitoring, FreeBSD acpi_thermal driver.

### Architectural Design
Governs CPU/GPU thermal zone monitoring, fan curve PWM modulation, thermal trip points (passive, active, critical), and proactive throttling.

**Current Implementation Status**: ACPI thermal zone discovery and CPU temperature polling functional. Dynamic fan curve PWM regulation is under active development.
**Active Enhancement Objectives**: Custom user fan curve profiles with hysteresis smoothing, emergency thermal shutdown trip wire, and passive CPU clock down-stepping.

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
# /etc/sigmaos/thermal-cooling.toml
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
$ sigma-ctl status thermal-cooling

# Inspect real-time log stream
$ journalctl -u thermal-cooling -f --no-pager
```

---

## 6. See Also
- [Main Portal: Core Kernel & Hardware](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
