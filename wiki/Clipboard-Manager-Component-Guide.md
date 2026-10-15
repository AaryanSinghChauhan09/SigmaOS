# Clipboard Manager Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`CLIPBOARD_MANAGER_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CLIPBOARD_MANAGER_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Clipboard Manager** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Clipboard-Manager`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`CLIPBOARD_MANAGER_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CLIPBOARD_MANAGER_AGENT.md)
- **Linux Inspiration**: cliphist Wayland clipboard manager, wl-clipboard (wl-copy / wl-paste), and GPaste daemon.
- **BSD Inspiration**: OpenBSD xclip/xsel selection forwarding, FreeBSD clipboard managers.

### Architectural Design
Governs Wayland primary selection, clipboard copy/paste history, multi-mime-type payload streaming (text, PNG, HTML), and secure password masking inspired by Omarchy Linux Cliphist.

**Current Implementation Status**: Ring buffer history, deduplication, and query search implemented in CliphistClipboardEngine and Zig sigma_cliphist_fast_ring.zig.
**Active Enhancement Objectives**: Direct wl-data-control-v1 protocol client, encrypted clipboard persistence database, and automatic sensitive password purge.

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
# /etc/sigmaos/clipboard-manager.toml
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
$ sigma-ctl status clipboard-manager

# Inspect real-time log stream
$ journalctl -u clipboard-manager -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
