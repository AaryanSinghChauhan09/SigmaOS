# USB Stack, xHCI/EHCI & HID Subsystem Component Guide

## Overview
The **USB Stack, xHCI/EHCI & HID Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `USB_SUBSYSTEM_HID_MASS_STORAGE`
- **Role**: USB Controller and Class Driver Governor
- **Inspiration**: Linux USB subsystem and FreeBSD usb2 stack.

## Key Architectural Capabilities
Drives Universal Serial Bus topologies across xHCI/EHCI controllers, managing HID input peripherals, USB mass storage SCSI command translation, and UVC video.

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
