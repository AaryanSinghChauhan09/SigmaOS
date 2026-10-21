# Mangohud Gaming Component Guide

> **Related portals:** [Gaming, Acceleration & Virtualization](./Home.md) &bull; **Maintainer Agent:** [`MANGOHUD_GAMING_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/MANGOHUD_GAMING_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Mangohud Gaming** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Mangohud-Gaming`
- **Subsystem Category**: Gaming, Acceleration & Virtualization
- **Autonomous Agent**: [`MANGOHUD_GAMING_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/MANGOHUD_GAMING_AGENT.md)
- **Linux Inspiration**: MangoHud Vulkan overlay layer, Feral Interactive GameMode CPU governor switcher, and DXVK HUD.
- **BSD Inspiration**: FreeBSD Linuxulator gaming configurations, OpenBSD hw.sensors telemetry.

### Architectural Design
Provides hardware-accelerated Vulkan and OpenGL in-game performance overlays displaying real-time FPS, frame timing graphs, GPU/CPU temperatures, VRAM usage, and GameMode governor controls inspired by Omarchy Linux.

**Current Implementation Status**: Config generator, FPS limiter logic, and hardware sensor telemetry tracking implemented in OmarchyMangoHudGamingEngine.
**Active Enhancement Objectives**: Direct Vulkan overlay layer hook (`VK_LAYER_MANGOHUD`), frame-pacing variance analysis, and automated GPU clock locking.

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
# /etc/sigmaos/mangohud-gaming.toml
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
$ sigma-ctl status mangohud-gaming

# Inspect real-time log stream
$ journalctl -u mangohud-gaming -f --no-pager
```

---

## 6. See Also
- [Main Portal: Gaming, Acceleration & Virtualization](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
