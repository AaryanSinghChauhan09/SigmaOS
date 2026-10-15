# ACPI Power Management & AML Parser Subsystem Component Guide

## Overview
The **ACPI Power Management & AML Parser Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `ACPI_POWER_RESOURCE_PARSER`
- **Role**: ACPI DSDT/SSDT Table Parser and System Sleep Coordinator
- **Inspiration**: Linux ACPICA subsystem and NetBSD acpica integration.

## Key Architectural Capabilities
Parses ACPI AML bytecode tables, regulates system sleep states (S0 through S5), handles thermal zones and passive fan cooling trips, and reads smart battery telemetry.

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
