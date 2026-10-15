# Media Codec Hardware Component Guide

> **Related portals:** [Multimedia & Audio Subsystem](./Home.md) &bull; **Maintainer Agent:** [`MEDIA_CODEC_HARDWARE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/MEDIA_CODEC_HARDWARE_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Media Codec Hardware** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Media-Codec-Hardware`
- **Subsystem Category**: Multimedia & Audio Subsystem
- **Autonomous Agent**: [`MEDIA_CODEC_HARDWARE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/MEDIA_CODEC_HARDWARE_AGENT.md)
- **Linux Inspiration**: Linux VA-API (libva), VDPAU, FFmpeg hardware acceleration, and Mesa Gallium video decoders.
- **BSD Inspiration**: FreeBSD libva ports and Intel iHD media drivers.

### Architectural Design
Governs hardware-accelerated video decoding and encoding (H.264, HEVC, VP9, AV1) via VA-API and VDPAU across Intel, AMD, and NVIDIA hardware.

**Current Implementation Status**: VA-API driver probing, format negotiation, and basic H.264 NAL unit parsing implemented in media subsystem. AV1 decoding in progress.
**Active Enhancement Objectives**: Direct DMA-BUF surface import to Wayland compositor for zero-copy 4K 60FPS video playback, and hardware AV1 encoding.

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
# /etc/sigmaos/media-codec-hardware.toml
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
$ sigma-ctl status media-codec-hardware

# Inspect real-time log stream
$ journalctl -u media-codec-hardware -f --no-pager
```

---

## 6. See Also
- [Main Portal: Multimedia & Audio Subsystem](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
