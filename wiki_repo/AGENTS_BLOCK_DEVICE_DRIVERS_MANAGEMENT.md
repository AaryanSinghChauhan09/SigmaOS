# SigmaOS AI Agent Block Device Drivers Management Specification

This document specifies mandatory block device driver rules, AHCI/NVMe controller submission queue invariants, DMA buffer alignment standards, and Driver Shard isolation protocols for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Storage Controller & Block Device Architecture
- **Kernel Block Device Abstraction (`src/kernel/block_dev.rs`)**:
  - Block devices must expose uniform block read/write sector interfaces (512-byte and 4096-byte logical block addressing).
  - Synchronous block I/O calls must support asynchronous submission queues with completion notifications.

## 2. Hardware Controller Invariants & Queue Pairs
- **AHCI SATA Controller (`src/driver/ahci_sata_controller.rs`)**:
  - Command list and FIS (Frame Information Structure) buffers must be 1024-byte aligned in physical memory.
  - Port interrupt status registers must be cleared via write-1-to-clear (W1C) semantics after handling interrupts.
- **NVMe / VirtIO-blk Storage**:
  - Doorbell register writes must follow submission queue entry pushes.
  - Scatter-gather lists (SGLs) and Physical Region Pages (PRPs) must validate page boundaries.

## 3. Driver Shards & Modular Driver Lifecycle
- **Driver Shard Management (`src/drivers/sovereign_driver_lifecycle.rs`)**:
  - Driver shards (`DriverShardManager`) must execute inside sandboxed hardware containers (`SandboxedHardwareModule`).
  - Driver hot-swapping must quiesce active block I/O requests before unloading shard instances.

## 4. AI Agent Block Device Guidelines
1. **Physical DMA Buffer Pinning**: DMA memory buffers passed to storage controllers must be pinned via Memory Descriptor Lists (MDLs) to prevent page fault race conditions.
2. **Boundary Validation**: Sector offsets and count parameters must be validated against disk capacity limits before queue submissions.
