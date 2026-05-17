# Sovereign Hardware Abstraction Layer (S-HAL)

This document maps the extensive platform-specific hardware support in SigmaOS Zenith v15.2.

### Unified Driver Architecture
1. **Unified Device Mapping**: Standardized API for Wi-Fi, USB, IoT, and printers via `UnifiedDriver` structs.
2. **Platform-Agnostic Core**: Modular dispatching based on `CPULatticeArch` (x86_64, ARM64, RISCV64).
3. **Silicon-direct Register Writes**: Memory-Mapped I/O (MMIO) wrappers (`HAL_MMIO_W32`, `HAL_MMIO_R32`) for absolute zero-overhead hardware control.
4. **Hardware Timers**: High-precision ACPI and APIC timer bootstrapping for SCHED_SOVEREIGN logic.

### Graphics & Processing
5. **SovereignVulkanLayer**: Direct SPIR-V shader blob streaming to GPU without userland Vulkan wrappers.
6. **Hardware Transpiler**: On-the-fly SIMD pipeline conversions (x86 SSE -> ARM Neon).
7. **Neural Accelerators**: Native hooks for NPU dispatch (e.g., Apple Neural Engine, Intel Movidius).
8. **Asynchronous DMA**: Zero-copy packet streaming straight to network interfaces.

### Boot & Interrupt Control
9. **APIC/GIC/PLIC Initialization**: Architecture-dependent interrupt controller setups.
10. **Hardware Rollback Points**: Firmware-level snapshot integration for rapid bare-metal recovery.
11. **ACPI Power Management**: Dynamic voltage/frequency scaling (DVFS) implemented in kernel space.
