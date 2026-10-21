# Wofi Application Menu Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`WOFI_APPLICATION_MENU_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/WOFI_APPLICATION_MENU_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Wofi Application Menu** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Wofi-Application-Menu`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`WOFI_APPLICATION_MENU_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/WOFI_APPLICATION_MENU_AGENT.md)
- **Linux Inspiration**: wofi GTK/Wayland menu, rofi-wayland fork, and fuzzel minimal launcher.
- **BSD Inspiration**: OpenBSD dmenu simplicity, FreeBSD wofi ports.

### Architectural Design
Provides a highly customizable, CSS3-styleable application launcher, window switcher, and dmenu script runner optimized for Wayland compositors inspired by Omarchy Linux Wofi.

**Current Implementation Status**: Application menu categorization and dmenu stdin/stdout pipe mode functional in desktop toolkit. CSS parser active.
**Active Enhancement Objectives**: Fuzzy matching score normalization across mixed desktop entries and terminal shell history, and sub-pixel scrolling.

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
# /etc/sigmaos/wofi-application-menu.toml
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
$ sigma-ctl status wofi-application-menu

# Inspect real-time log stream
$ journalctl -u wofi-application-menu -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
