# DMA Engine & IOMMU Memory Management Subsystem Component Guide

## Overview
The **DMA Engine & IOMMU Memory Management Subsystem** provides high-reliability OS services for SigmaOS.
- **Component ID**: `DMA_IOMMU_DIRECT_MEMORY_ACCESS`
- **Role**: Scatter-Gather Direct Memory Access and IOMMU Domain Controller
- **Inspiration**: Linux dma-mapping / dma-iommu and FreeBSD vmm_iommu.

## Key Architectural Capabilities
Provides zero-copy scatter-gather DMA memory translation, ring buffers, and DMA-remapping isolation for high-throughput network and NVMe devices.

## Language Implementations & Integration
- **Rust Engine**: Integrated in `src/kernel/hardware_firmware_subsystem.rs` with multi-core synchronization and safe abstraction boundaries.
- **Zig Accelerator**: High-performance zero-allocation buffers in `src/kernel/`.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

## Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all previous restrictions against the standard library, authorizing and leveraging standard data structures (`BTreeMap`, `HashMap`, `VecDeque`, `Arc`, `Mutex`) and libc runtime functions as needed.
