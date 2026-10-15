# Quickshell Widgets Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`QUICKSHELL_WIDGETS_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/QUICKSHELL_WIDGETS_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Quickshell Widgets** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Quickshell-Widgets`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`QUICKSHELL_WIDGETS_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/QUICKSHELL_WIDGETS_AGENT.md)
- **Linux Inspiration**: Quickshell QML Wayland widget framework, Eww (Elkowars Wacky Widgets), and Conky desktop system monitor.
- **BSD Inspiration**: FreeBSD xmobar lightweight status bar, OpenBSD wscons visual telemetry.

### Architectural Design
Provides reactive, GPU-accelerated desktop widgets (on-screen display sliders for volume and brightness, media player controls, lockscreen clock, system telemetry HUDs) inspired by Omarchy Linux Quickshell.

**Current Implementation Status**: Widget registry, geometry coordinates, and visibility controls implemented in OmarchyQuickshellWidgetsEngine. OSD slider templates defined.
**Active Enhancement Objectives**: Declarative UI markup parser for rapid user widget creation, MPRIS2 media controller protocol bridge, and smooth spring physics animations.

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
# /etc/sigmaos/quickshell-widgets.toml
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
$ sigma-ctl status quickshell-widgets

# Inspect real-time log stream
$ journalctl -u quickshell-widgets -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
