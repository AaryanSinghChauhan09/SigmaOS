# ZRAM Compressed In-Memory Swap Subsystem Component Guide

> **Related portals:** [Storage, Volumes & Filesystems](./Home.md) &bull; **Maintainer Agent:** [`SWAP_ZRAM_COMPRESSION_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/SWAP_ZRAM_COMPRESSION_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **ZRAM Compressed In-Memory Swap Subsystem** provides high-reliability, sovereign system infrastructure for SigmaOS:
- **Component ID**: `SWAP_ZRAM_COMPRESSION`
- **Subsystem Category**: Storage, Volumes & Filesystems
- **Autonomous Agent**: `SWAP_ZRAM_COMPRESSION_AGENT`
- **Upstream Parity Reference**: Linux zram.ko, zswap writeback cache, Fedora zram-generator, and systemd-zram.

Creates virtual compressed swap block devices in physical RAM using fast algorithms (LZ4, Zstandard), dramatically expanding effective memory capacity and eliminating SSD swap wear.

---

## 2. Technical Specifications & Protocols
- **Memory Safety**: Memory-safe kernel and userland execution boundaries preventing memory corruption and buffer overflow exploits.
- **Concurrency Model**: Lock-free or fine-grained synchronization primitives using standard mutexes and atomic operations.
- **Error Handling**: Explicit, zero-cost error propagation eliminating silent failures and unhandled edge cases.

---

## 3. Multi-Language Implementations
- **Rust Engine**: Integrated in `src/kernel/sovereign_parity_apex_v100.rs` with safe abstraction boundaries.
- **Zig Accelerator**: Fast deterministic execution buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

### Standard Library Authorization
Per updated SigmaOS architectural governance, this subsystem drops all previous restrictions against standard library usage. Standard libraries (`std::collections::BTreeMap`, `std::sync::Arc`, `std::sync::Mutex`, `VecDeque`, and `libc`) are authorized and actively utilized.

---

## 4. Configuration & Usage

```toml
# /etc/systemd/zram-generator.conf
[zram0]
zram-size = min(ram / 2, 8192)
compression-algorithm = zstd
swap-priority = 100
fs-type = swap
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem status and diagnostics
$ sigma-ctl status swap_zram_compression

# Inspect system logs for events
$ journalctl -u swap_zram_compression --no-pager
```

---

## 6. See Also
- [Main Portal: Storage, Volumes & Filesystems](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
