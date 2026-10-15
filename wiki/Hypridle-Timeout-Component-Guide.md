# Hypridle Timeout Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`HYPRIDLE_TIMEOUT_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/HYPRIDLE_TIMEOUT_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Hypridle Timeout** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Hypridle-Timeout`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`HYPRIDLE_TIMEOUT_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/HYPRIDLE_TIMEOUT_AGENT.md)
- **Linux Inspiration**: hypridle Wayland idle listener daemon, swayidle, and systemd-logind idle hints.
- **BSD Inspiration**: OpenBSD apm idle sleep triggers, FreeBSD devd ACPI power management events.

### Architectural Design
Governs multi-stage system idle event detection, display brightness dimming, Wayland lock invocation, DPMS display power-down, and automatic system suspension inspired by Omarchy Linux Hypridle.

**Current Implementation Status**: Multi-stage idle timeout evaluation implemented in HypridleTimeoutGovernor. Brightnessctl dimming and DPMS off command templates active.
**Active Enhancement Objectives**: Wayland ext-idle-notify-v1 protocol server integration, media playback active inhibitor detection, and smooth gamma fade-to-black.

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
# /etc/sigmaos/hypridle-timeout.toml
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
$ sigma-ctl status hypridle-timeout

# Inspect real-time log stream
$ journalctl -u hypridle-timeout -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
