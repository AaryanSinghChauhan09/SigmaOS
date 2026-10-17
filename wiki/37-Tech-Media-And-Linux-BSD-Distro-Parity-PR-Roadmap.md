# 37. Tech Media & Linux/BSD Distro Kernel & Hardening Parity PR Roadmap

## Executive Summary
This document specifies the future development roadmap for SigmaOS core kernel, low-level scheduling, hardware power management, and sandboxing capabilities formatted as Pull Request (PR) specifications. Inspirations are synthesized from leading technical media publications (**Phoronix**, **9to5Linux**, **HW Busters**, **Linux.org**, **Linux.com**, **InfoWorld**, **Linux Foundation**) and Linux/BSD operating systems (**Arch Linux**, **FreeBSD**, **OpenBSD**, **CachyOS**, **DragonFly BSD**).

---

## Roadmap PR Specifications

| PR # | Title & Target Component | Inspired Sources | Key Architectural Mechanisms |
| :--- | :--- | :--- | :--- |
| **#3701** | `feat(kernel): sched_ext BPF BORE v2 Scheduler Optimization` | Phoronix, 9to5Linux, CachyOS | Linux 6.12+ `sched_ext` extensible scheduler with Burst-Oriented Response Enhancer (BORE) v2, sub-microsecond latency task prioritization, and eBPF dispatch queues. |
| **#3702** | `feat(power): ATX 3.1 PSU & PCIe 5.1 Dynamic Power/Thermal Governor` | HW Busters, TechPowerUp | Microsecond dynamic power rail transient tracking, Cybenetics efficiency profiling, and ACPI thermal trip-point fan curves. |
| **#3703** | `feat(security): Landlock V6 Network & OpenBSD Pledge/Unveil Unsharing` | Linux.org, InfoWorld, OpenBSD | Landlock V6 socket port binding restriction, zero-allocation byte slice path unveiling, and pledge promise boundary enforcement. |
| **#3704** | `feat(storage): OpenZFS 2.3 & HAMMER2 Clustered CoW Sync Engine` | DistroWatch, FreeBSD, DragonFly BSD | OpenZFS 2.3 fast clone reflink integration, HAMMER2 multi-node PFS quorum, and background deduplication. |
| **#3705** | `feat(hal): Asahi Apple Silicon M1-M4 SMC & Tensor TPU HAL` | 9to5Mac, Phoronix, Asahi Linux | SMC power domain management, Rosetta 2 vector instruction translation, and NPU/TPU scheduling hooks. |

---

## Detailed PR Specifications

### PR #3701: `sched_ext` BPF BORE v2 Scheduler Optimization
- **PR Branch:** `feature/kernel-sched-ext-bore-v2`
- **Target Upstream:** Linux Kernel 6.12+ / CachyOS Kernel
- **Primary Inspiration:** *Phoronix ("Linux 6.12 sched_ext Benchmarks")* & *9to5Linux ("CachyOS BORE Scheduler Enhancements")*
- **Key Capabilities:**
  1. Programmable BPF scheduler dispatcher written in clean `#![no_std]` Rust.
  2. BORE v2 score calculation based on task execution bursts to minimize gaming & desktop latency.
  3. Dynamic task migration across multi-core NUMA topology with sub-80ns context switches.

### PR #3702: ATX 3.1 PSU & PCIe 5.1 Dynamic Power/Thermal Governor
- **PR Branch:** `feature/hardware-atx31-power-governor`
- **Target Upstream:** ACPI 6.5 / PCIe 5.1 CEM
- **Primary Inspiration:** *HW Busters ("ATX 3.1 Transient Load Management")* & *TechPowerUp ("PCIe 5.1 Power Excursion Limits")*
- **Key Capabilities:**
  1. Telemetry monitoring for 12V-2x6 power rail excursions and transient voltage drops.
  2. Dynamic P-state scaling integrating `schedutil` governor and thermal cooling fan curves.
  3. Hardware watchdog ping heartbeats with zero-allocation panic recovery.

### PR #3703: Landlock V6 Network & OpenBSD Pledge/Unveil Sandbox
- **PR Branch:** `feature/security-landlock-v6-pledge-unveil`
- **Target Upstream:** Linux 6.15+ Landlock / OpenBSD 7.6 Pledge
- **Primary Inspiration:** *Linux.org ("Unsharing Linux Namespaces Safely")* & *OpenBSD 7.6 Release Engineering*
- **Key Capabilities:**
  1. Landlock V6 socket binding restrictions enforcing network access masks.
  2. NUL-byte and URL-encoded traversal hardened path validation (`validate_path`) for OpenBSD-style `unveil`.
  3. Strict syscall filtering via Seccomp-BPF and Capability bitmask overlap prevention.

---

## Verification & Validation Commands

To verify this roadmap slice:
```bash
./run_sigma_tests.sh
```
