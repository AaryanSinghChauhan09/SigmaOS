# PCI Express Bus Enumeration & AER Subsystem Component Guide

## Overview
The **PCI Express Bus Enumeration & AER Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `PCI_EXPRESS_BUS_ENUMERATION`
- **Role**: PCIe Root Complex, MSI/MSI-X and BAR Allocation Agent
- **Inspiration**: Linux pci-driver core and OpenBSD pci/pcie subsystem.

## Key Architectural Capabilities
Recursively enumerates PCIe root complexes, bridges, allocates Base Address Registers (BARs), routes MSI/MSI-X interrupts, and monitors Advanced Error Reporting (AER).

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
