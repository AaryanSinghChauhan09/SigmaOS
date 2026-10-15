# CPU Frequency Scaling & Performance Governor Subsystem Component Guide

## Overview
The **CPU Frequency Scaling & Performance Governor Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `CPU_FREQUENCY_SCALING_GOVERNOR`
- **Role**: Dynamic P-State, Boost Frequency and Energy-Performance Governor
- **Inspiration**: Linux cpufreq / intel_pstate / amd_pstate and FreeBSD powerd.

## Key Architectural Capabilities
Adjusts CPU core frequencies dynamically across schedutil, performance, and powersave policies with hardware P-state coordination (Intel HWP / AMD CPPC).

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
