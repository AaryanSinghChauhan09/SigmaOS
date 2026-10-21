# Container Podman Rootless Component Guide

> **Related portals:** [Gaming, Acceleration & Virtualization](./Home.md) &bull; **Maintainer Agent:** [`CONTAINER_PODMAN_ROOTLESS_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CONTAINER_PODMAN_ROOTLESS_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Container Podman Rootless** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Container-Podman-Rootless`
- **Subsystem Category**: Gaming, Acceleration & Virtualization
- **Autonomous Agent**: [`CONTAINER_PODMAN_ROOTLESS_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CONTAINER_PODMAN_ROOTLESS_AGENT.md)
- **Linux Inspiration**: Podman rootless containers, crun lightweight OCI runtime, buildah, and slirp4netns.
- **BSD Inspiration**: FreeBSD unprivileged Jails (prison_create), OpenBSD pledge-confined micro-containers.

### Architectural Design
Governs daemonless, unprivileged rootless OCI containers, user namespace UID/GID sub-mapping (subuid/subgid), slirp4netns unprivileged networking, and quadlet systemd integration.

**Current Implementation Status**: User namespace mapping and rootless container isolation primitives functional in container runtime engine.
**Active Enhancement Objectives**: Slirp4netns-compatible unprivileged virtual network bridge, and OCI image rootfs layer extraction directly into user home dirs.

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
# /etc/sigmaos/container-podman-rootless.toml
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
$ sigma-ctl status container-podman-rootless

# Inspect real-time log stream
$ journalctl -u container-podman-rootless -f --no-pager
```

---

## 6. See Also
- [Main Portal: Gaming, Acceleration & Virtualization](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
