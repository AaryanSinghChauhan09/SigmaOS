# Udev & Devd Device Node Hotplug Manager Component Guide

> **Related portals:** [Core Kernel & Hardware](./Home.md) &bull; **Maintainer Agent:** [`UDEV_DEVD_HOTPLUG_DEVICE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/UDEV_DEVD_HOTPLUG_DEVICE_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Udev & Devd Device Node Hotplug Manager** provides high-reliability, sovereign system infrastructure for SigmaOS:
- **Component ID**: `UDEV_DEVD_HOTPLUG_DEVICE`
- **Subsystem Category**: Core Kernel & Hardware
- **Autonomous Agent**: `UDEV_DEVD_HOTPLUG_DEVICE_AGENT`
- **Upstream Parity Reference**: systemd-udevd, eudev, Linux netlink kobject uevent, and FreeBSD devd.

Monitors kernel uevents over netlink, dynamically populates and maintains /dev, generates persistent disk symlinks (/dev/disk/by-id, /dev/disk/by-uuid), and executes user-defined hardware rules.

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
# /etc/udev/rules.d/70-sigma-hardware.rules
ACTION=="add", SUBSYSTEM=="block", KERNEL=="nvme*", ATTR{queue/scheduler}="none"
ACTION=="add", SUBSYSTEM=="input", ATTRS{name}=="*Keyboard*", TAG+="power-switch"
ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="adbusers"
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem status and diagnostics
$ sigma-ctl status udev_devd_hotplug_device

# Inspect system logs for events
$ journalctl -u udev_devd_hotplug_device --no-pager
```

---

## 6. See Also
- [Main Portal: Core Kernel & Hardware](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
