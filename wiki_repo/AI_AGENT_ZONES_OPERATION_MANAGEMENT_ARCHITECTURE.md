# AI Agent Zones Operation Management Architecture

## Executive Overview

Zones Operation Management in SigmaOS governs Illumos/Solaris-inspired container zones isolation, FreeBSD VM zone page queue transitions, and power management thermal zones. Implemented across `src/kernel/linux_bsd_innovations.rs`, `src/kernel/memory/sigma_buddy.rs`, and `src/kernel/power/mod.rs`, SigmaOS manages lightweight OS-level container isolation (`SovereignZonesManager`, `SovereignZone`, `create_zone`), fair-share CPU percentage allocation (`calculate_cpu_percentage`), virtual network interface (VNIC) IP assignment (`configure_vnic`), VM memory zone allocations (`BsdVmZoneAllocator`), and hardware thermal zone throttling (`ThermalZone`) with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents instantiating, configuring, or managing zones in SigmaOS.

---

## Subsystem Architecture & Zones Pipeline

```
                                +-----------------------------------+
                                |    Application / System Request   |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    Sovereign Zones Manager        |
                                | (src/kernel/linux_bsd_innovations)|
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | Container Zones       |   | FreeBSD VM Zones  |   | Power Thermal Zones   |
            | cpu_shares & VNICs    |   | BsdVmZoneAllocator|   | ThermalZone CPU Temp  |
            | calculate_cpu_pct()   |   | Page Queue Shift  |   | Throttling & Trip     |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |  Isolated Exec & Thermal Control  |
                                +-----------------------------------+
```

### Core Zones Components

1. **Solaris Container Zones Manager (`src/kernel/linux_bsd_innovations.rs`)**:
   - `SovereignZonesManager`: Tracks active isolated execution environments (`SovereignZone`).
   - `create_zone(name, cpu_shares, max_memory_bytes)`: Spawns isolated container zone with proportional CPU share weight and hard RAM limits.
   - `calculate_cpu_percentage(zone_name)`: Computes proportional CPU time allocation ($CPU_{pct} = \frac{shares_{zone}}{\sum shares} \times 100$).
   - `configure_vnic(zone_name, ip_addr)`: Binds virtual network interface addresses to isolated zones.

2. **FreeBSD VM Zone Page Allocator (`src/kernel/memory/sigma_buddy.rs`)**:
   - `BsdVmZoneAllocator`: Manages VM memory zone page transitions between `Free`, `Active`, `Inactive`, and `Wired` queues.

3. **Thermal Power Zones (`src/kernel/power/mod.rs`)**:
   - `ThermalZone`: Monitors CPU and board temperature trip points, automatically scaling frequency governors when operating temperatures exceed limits.

---

## Zero-Allocation Guardrails

AI agents executing zones operations must observe these constraints:
- CPU percentage share math uses fixed-point float or integer calculations.
- VM page queue transitions adjust atomic atomic page counters without heap allocations.

---

## Related Architectural References
- `src/kernel/linux_bsd_innovations.rs` - Solaris Zones container manager.
- `src/kernel/memory/sigma_buddy.rs` - FreeBSD VM zone allocator.
- `src/kernel/power/mod.rs` - Thermal power zones.
- `docs/AI_AGENT_KERNEL_MANAGEMENT_ARCHITECTURE.md` - Master kernel subsystem architecture.
