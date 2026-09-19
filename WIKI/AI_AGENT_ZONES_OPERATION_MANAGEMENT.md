# AI Agent Zones Operation Management Guide

## Overview
This wiki guide details Zones Operation Management protocols for AI coding agents operating on SigmaOS. It covers Illumos/Solaris-inspired container zones isolation (`SovereignZonesManager`, `SovereignZone`, `create_zone`), fair-share CPU percentage calculation (`calculate_cpu_percentage`), virtual network interface (VNIC) IP assignment (`configure_vnic`), FreeBSD VM zone page queue transitions (`BsdVmZoneAllocator`), and thermal zone throttling (`ThermalZone`).

## Key Principles
1. **Solaris Container Zones**: Isolated execution environments with proportional CPU share weight and memory limits.
2. **VNIC Networking**: Dedicated virtual network interface IP addresses assigned per zone.
3. **FreeBSD VM Zones**: Memory page transitions across Free, Active, Inactive, and Wired page queues.

## Container Zone Example (`src/kernel/linux_bsd_innovations.rs`)
```rust
let mut manager = SovereignZonesManager::new();
manager.create_zone("web_zone", 150, 2048 * 1024 * 1024)?;
manager.configure_vnic("web_zone", "10.0.0.10")?;
```

## Related Documents
- `docs/AI_AGENT_ZONES_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_ZONES_OPERATION_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENT_MEMORY_OPERATION_MANAGEMENT.md`
