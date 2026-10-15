# Device Mapper, LVM & Cryptsetup Subsystem Component Guide

## Overview
The **Device Mapper, LVM & Cryptsetup Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `DEVICE_MAPPER_LVM_CRYPTSETUP`
- **Role**: Virtual Block Layer, Logical Volume Management & LUKS2 Encryption Controller
- **Inspiration**: Linux dm-crypt, dm-verity, LVM2, and FreeBSD GEOM/geli.

## Key Architectural Capabilities
Manages virtual block devices, linear/striped/mirrored volume mappings, LUKS2 encrypted storage with AES-XTS-256, and dm-verity integrity validation.

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
