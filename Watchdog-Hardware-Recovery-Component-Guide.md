# Hardware Watchdog Recovery Governor Component Guide

> **Related portals:** [Core Kernel & Hardware](./Home.md) &bull; **Maintainer Agent:** [`WATCHDOG_HARDWARE_RECOVERY_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/WATCHDOG_HARDWARE_RECOVERY_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Hardware Watchdog Recovery Governor** provides high-reliability, sovereign system infrastructure for SigmaOS:
- **Component ID**: `WATCHDOG_HARDWARE_RECOVERY`
- **Subsystem Category**: Core Kernel & Hardware
- **Autonomous Agent**: `WATCHDOG_HARDWARE_RECOVERY_AGENT`
- **Upstream Parity Reference**: Linux watchdog driver framework, systemd RuntimeWatchdogSec, and FreeBSD watchdogd.

Arms hardware watchdog timers (Intel TCO, AMD SP5100, IPMI, /dev/watchdog) and maintains active heartbeat pings, triggering automatic safe reboots in the event of hard kernel deadlocks.

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
# /etc/systemd/system.conf watchdog tuning
RuntimeWatchdogSec=30s
RebootWatchdogSec=10min
KExecWatchdogSec=0
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem status and diagnostics
$ sigma-ctl status watchdog_hardware_recovery

# Inspect system logs for events
$ journalctl -u watchdog_hardware_recovery --no-pager
```

---

## 6. See Also
- [Main Portal: Core Kernel & Hardware](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
