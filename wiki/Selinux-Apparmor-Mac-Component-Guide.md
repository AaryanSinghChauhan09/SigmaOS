# Mandatory Access Control (MAC) Subsystem Component Guide

## Overview
The **Mandatory Access Control (MAC) Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `SELINUX_APPARMOR_MAC`
- **Role**: SELinux, AppArmor & MAC Security Policy Enforcement Agent
- **Inspiration**: Linux Security Modules (AppArmor, SELinux) and FreeBSD MAC framework.

## Key Architectural Capabilities
Enforces mandatory access control policies, path-based and label-based permissions, LSM hooks, and audit log generation.

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
