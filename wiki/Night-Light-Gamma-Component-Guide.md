# Night Light Gamma Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`NIGHT_LIGHT_GAMMA_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/NIGHT_LIGHT_GAMMA_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Night Light Gamma** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Night-Light-Gamma`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`NIGHT_LIGHT_GAMMA_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/NIGHT_LIGHT_GAMMA_AGENT.md)
- **Linux Inspiration**: hyprsunset, wlsunset, and redshift gamma controllers.
- **BSD Inspiration**: OpenBSD wsdisplay color calibration, FreeBSD xgamma.

### Architectural Design
Governs smooth color temperature transitions (1000K to 10000K), solar twilight calculation, display hardware gamma LUT manipulation, and blue-light reduction inspired by Omarchy Linux Hyprsunset.

**Current Implementation Status**: Color temperature stepping and hyprsunset command generation implemented in HyprsunsetNightLightEngine and Nim sigma_nightlight.nim.
**Active Enhancement Objectives**: Direct DRM/KMS hardware gamma LUT programming via libdrm, smooth multi-second ramp interpolation, and per-monitor color curves.

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
# /etc/sigmaos/night-light-gamma.toml
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
$ sigma-ctl status night-light-gamma

# Inspect real-time log stream
$ journalctl -u night-light-gamma -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
