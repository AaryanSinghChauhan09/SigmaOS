# Live Iso Installer Calamares Component Guide

> **Related portals:** [Package Management & Maintenance](./Home.md) &bull; **Maintainer Agent:** [`LIVE_ISO_INSTALLER_CALAMARES_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/LIVE_ISO_INSTALLER_CALAMARES_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Live Iso Installer Calamares** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Live-Iso-Installer-Calamares`
- **Subsystem Category**: Package Management & Maintenance
- **Autonomous Agent**: [`LIVE_ISO_INSTALLER_CALAMARES_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/LIVE_ISO_INSTALLER_CALAMARES_AGENT.md)
- **Linux Inspiration**: Calamares universal installer framework, Archinstall automated CLI installer, and Ubuntu Subiquity.
- **BSD Inspiration**: FreeBSD bsdinstall interactive terminal installer, OpenBSD install.sub automated disk partitioning.

### Architectural Design
Governs graphical live media boot, automated hardware qualification, disk partitioning (GPT/ESP), rootfs decompression (zstd), user setup, and EFI bootloader installation inspired by Calamares in Omarchy Linux.

**Current Implementation Status**: Partitioning layout calculator and raw rootfs streaming engine implemented in Nim and Zig installer modules.
**Active Enhancement Objectives**: Interactive QML/Rust graphical wizard with automatic timezone geolocation and live install progress bars.

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
# /etc/sigmaos/live-iso-installer-calamares.toml
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
$ sigma-ctl status live-iso-installer-calamares

# Inspect real-time log stream
$ journalctl -u live-iso-installer-calamares -f --no-pager
```

---

## 6. See Also
- [Main Portal: Package Management & Maintenance](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
