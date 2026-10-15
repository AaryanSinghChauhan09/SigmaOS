# Process Namespaces & Jail Isolation Subsystem Component Guide

## Overview
The **Process Namespaces & Jail Isolation Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `NAMESPACES_ISOLATION_JAILS`
- **Role**: Container and Sandbox Boundary Governor
- **Inspiration**: Linux Namespaces, FreeBSD Jails, and OpenBSD pledge/unveil.

## Key Architectural Capabilities
Provides lightweight container virtualization via PID, mount, network, IPC, and UTS isolation domains combined with FreeBSD Jails and OpenBSD pledge paradigms.

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
