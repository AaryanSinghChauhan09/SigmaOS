# Upstream Study: Linux Unified Driver Model

1. **Upstream Problem**: Device enumeration, bus registration, PCI/USB probing, and power management callbacks.
2. **Data Structure & Control Flow**: `bus_type`, `device_driver`, `device`, PCI vendor/device ID matching tables.
3. **Security & Failure Assumptions**: Driver fault containment; Ring 3 microkernel separation where applicable.
4. **Direct Adoption**: Bus/Driver registration pattern and PCI device ID match tables.
5. **Rust/SigmaOS Adaptation**: `DriverShardContainer` with eBPF/Capsicum isolation for userland driver shards.
6. **Exit Test Criteria**: PCI device probe correctly identifies VirtIO-blk and VirtIO-net devices in QEMU.
7. **Licensing & Attribution**: GPL-2.0 (Clean-room specification in SigmaOS).
