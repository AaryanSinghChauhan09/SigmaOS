# SigmaOS Driver Shard Roadmap

This document tracks the development status of hardware drivers within the Sovereign Lattice.

## ðŸ› ï¸ Current Driver Status | Shard | Subsystem | Hardware | Status | | :--- | :--- | :--- | :--- | | `SovereignVGA` | Display | VGA / VESA LFB | **STABLE** | | `SovereignPS2` | Input | PS/2 Keyb/Mouse | **STABLE** | | `SovereignATA` | Storage | IDE / PATA | **STABLE** | | `SovereignE1000`| Network | Intel 8254x (QEMU) | **BETA** | | `SovereignAHCI` | Storage | SATA / SSD | **BETA** | ## ðŸš€ Driver Roadmap (Priority)

### 1. High Priority (Q3 2026)

- **NVMe Storage Shard**: Support for modern high-speed lattice storage.
- **ACPI/PM Shard**: Power management, sleep/resume, and thermal monitoring.
- **VirtIO-GPU**: Accelerated 2D/3D rendering for Zenith UI.

### 2. Medium Priority (Q4 2026)

- **Intel HDA**: Native audio support.
- **USB 3.0 (xHCI)**: Support for modern peripherals and removable storage.
- **RTL8139 / VirtIO-Net**: Expanded networking coverage.

### 3. Long-term (2027+)

- **NVIDIA/AMD Modesetting**: Basic hardware acceleration for physical GPUs.
- **Bluetooth/Wi-Fi**: Wireless lattice connectivity.

## ðŸ›¡ï¸ Driver Security

All drivers must run within the **L2 System Shard** layer, meaning they are isolated from the kernel core and require explicit `SovereignHAL` bridges for hardware register access.

---

### To contribute a driver, see [Contributing](CONTRIBUTING.md)
