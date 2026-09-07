# SigmaOS AI Agent Blocks Management Guidelines

## 1. Overview
SigmaOS manages storage blocks, filesystem allocation blocks, and hardware I/O queues via specialized AI storage agents (such as `BlockStorageGovernor`, `NvmeQueueManager`, `BtrfsBlockGroupTree`, and `ZfsVdevBlockAllocator`). These guidelines define NVMe / AHCI storage block driver I/O submission queues, Btrfs block group trees, ZFS vdev block allocation, and block-level encryption in SigmaOS.

## 2. Core Blocks Management Principles

### 2.1 NVMe & AHCI Block Driver I/O Queues
- **NVMe Submission & Completion Queues**: High-performance storage operations submit 512-byte / 4096-byte LBA block commands to NVMe hardware submission queues (`ModernNvmeDriver` in `src/drivers/modern_nvme.rs`) with MSI-X interrupt vector completion.
- **AHCI SATA Command Slots**: Legacy SATA storage interfaces submit block commands via 32 AHCI command slots (`AhciStorageDriver`).

### 2.2 Btrfs & ZFS Storage Block Trees
- **Btrfs Block Groups**: Storage blocks are grouped into Data, Metadata, and System block groups (`src/filesystem/modern_fs.rs`), allocated dynamically based on storage workload demands.
- **ZFS vdev Block Allocation**: ZFS storage pools allocate blocks across mirror, raidz1, raidz2, and raidz3 vdevs using log-structured allocation trees and zero-copy SLAB block allocators.

### 2.3 Transparent LUKS2 Block Encryption
- **AES-XTS Block Encryption**: Block-level storage partitions are encrypted transparently using LUKS2 AES-XTS 256/512-bit cipher engines (`Luks2CryptVolume`).

---
*Maintained by the SigmaOS Storage & Hardware Block Steering Committee.*
