# Waybar Status Bar Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`WAYBAR_STATUS_BAR_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/WAYBAR_STATUS_BAR_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Waybar Status Bar** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Waybar-Status-Bar`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`WAYBAR_STATUS_BAR_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/WAYBAR_STATUS_BAR_AGENT.md)
- **Linux Inspiration**: Waybar GTK/Wayland modular panel, Polybar X11 status bar, and i3status lightweight telemetry generator.
- **BSD Inspiration**: FreeBSD sysctl hardware metrics, OpenBSD systat status monitors.

### Architectural Design
Provides highly customizable modular status bar panels with real-time CPU, RAM, battery, audio volume, network throughput, active window title, and tray indicators inspired by Omarchy Linux.

**Current Implementation Status**: Modular bar layout engine implemented in OmarchyWaybarStatusBarEngine supporting left/center/right module positioning. Real-time metric feeds integrated with kernel rings.
**Active Enhancement Objectives**: Custom CSS gradient styling parser, interactive tooltip popups with calendar and audio sink selectors, and battery discharge rate predictor.

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
# /etc/sigmaos/waybar-status-bar.toml
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
$ sigma-ctl status waybar-status-bar

# Inspect real-time log stream
$ journalctl -u waybar-status-bar -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
