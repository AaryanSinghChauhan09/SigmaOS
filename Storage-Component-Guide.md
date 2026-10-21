# Storage Component Guide

> **Related portals:** [Storage, Volumes & Filesystems](./Home.md) &bull; **Maintainer Agent:** [`STORAGE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/STORAGE_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Storage** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Storage`
- **Subsystem Category**: Storage, Volumes & Filesystems
- **Autonomous Agent**: [`STORAGE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/STORAGE_AGENT.md)
- **Linux Inspiration**: Linux blk-mq multi-queue block layer, NVMe over Fabrics, dm-crypt disk encryption, and mdraid software RAID.
- **BSD Inspiration**: FreeBSD CAM (Common Access Method), GEOM framework, OpenBSD bioctl and softraid encrypted volumes.

### Architectural Design
Governs NVMe, AHCI/SATA, VirtIO-Block, SCSI drivers, block IO queuing, request merging, multi-queue dispatching, and software RAID arrays.

**Current Implementation Status**: VirtIO-Block and AHCI SATA controller drivers implemented. Multi-queue NVMe driver, request merging, and TRIM/discard support are ongoing.
**Active Enhancement Objectives**: Full NVMe 1.4+ driver with multi-queue submission/completion rings, automated TRIM/Discard wear-leveling, software RAID-1/RAID-5 engine, and hardware crypto acceleration.

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
# /etc/sigmaos/storage.toml
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
$ sigma-ctl status storage

# Inspect real-time log stream
$ journalctl -u storage -f --no-pager
```

---

## 6. See Also
- [Main Portal: Storage, Volumes & Filesystems](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
