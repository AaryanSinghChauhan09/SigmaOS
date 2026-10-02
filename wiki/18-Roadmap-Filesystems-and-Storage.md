# SigmaOS Future Development Roadmap: Filesystems & Storage Subsystems

This roadmap details the future evolution of storage architectures, virtual filesystem (VFS) layers, and device drivers in SigmaOS, drawing architectural inspiration from Linux, OpenZFS, Bcachefs, FreeBSD GEOM, DragonFly BSD HAMMER2, and Haiku BFS.

---

## 1. Executive Summary & Core Storage Philosophy

SigmaOS implements a unified, zero-dependency VFS layer written in Safe Rust (`#![no_std]`). To provide crash consistency, tiering performance, and robust volume management across diverse hardware storage devices (from 1980s IDE/PIO to 2026+ NVMe 2.0 and CXL 3.0 persistent memory), SigmaOS incorporates storage patterns from leading open-source filesystems.

```
+----------------------------------------------------------------------------------------------------+
|                         SIGMAOS UNIFIED VFS & STORAGE LAYER ROADMAP                                |
+----------------------------------------------------------------------------------------------------+
|  [OpenZFS ARC & Dataset Engine] |  [Bcachefs Tiered Storage]    |  [Ext4+JBD2 Journaling Engine]   |
+----------------------------------------------------------------------------------------------------+
|  [FreeBSD GEOM Storage Subsystem] |  [DragonFly HAMMER2 PFS]     |  [Haiku BFS Extended Attributes] |
+----------------------------------------------------------------------------------------------------+
|                       SIGMAOS BARE-METAL PERSISTENT VFS & STORAGE DRIVERS                          |
+----------------------------------------------------------------------------------------------------+
```

---

## 2. OpenZFS-Inspired Storage Innovations

### 2.1 Adaptive Replacement Cache (ARC) & ZFS Datasets
- **Inspiration**: OpenZFS Adaptive Replacement Cache (ARC) and L2ARC.
- **Target Architecture**:
  - Replaces traditional LRU page caches with a dual-list Most Recently Used (MRU) and Most Frequently Used (MFU) ARC eviction governor.
  - Implements dynamic ARC size auto-tuning based on physical memory pressure and zone allocation stats (`Dma32`, `Normal`, `HighMem`).
  - Provides native copy-on-write (COW) snapshot dataset management with instant rollback capabilities under 50ms.
- **Milestones**:
  - **Phase 1**: Bare-metal Rust `#![no_std]` ARC MRU/MFU page eviction queues.
  - **Phase 2**: Asynchronous L2ARC SSD caching and compression block pipeline (LZ4/ZSTD).

---

## 3. Bcachefs & Ext4 Journaling Innovations

### 3.1 Bcachefs Multi-Device Tiered Storage Engine
- **Inspiration**: Linux Bcachefs multi-device tiering and checksumming.
- **Target Architecture**:
  - Automatic data placement across storage tiers (`NVRAM` -> `NVMe SSD` -> `SATA SSD` -> `HDD`).
  - End-to-end 64-bit CRC/xxHash checksumming for data and metadata blocks, with background scrubbers for silent data corruption self-healing.

### 3.2 Ext4 + JBD2 Journaling Parity Engine
- **Inspiration**: Linux Ext4 filesystem and Journaling Block Device 2 (JBD2).
- **Target Architecture**:
  - Full metadata journaling with ordered data logging (`data=ordered`) for crash safety against unexpected power loss.
  - Extent tree allocations supporting 128MB extent blocks and delayed allocation (`delalloc`).

---

## 4. FreeBSD GEOM & Haiku BFS Attributes

### 4.1 FreeBSD GEOM Storage Transformation Subsystem
- **Inspiration**: FreeBSD GEOM modular storage framework.
- **Target Architecture**:
  - Directed acyclic graph (DAG) of storage providers and consumers (`GEOM_DISK`, `GEOM_PART`, `GEOM_MIRROR`, `GEOM_ELI` PQC encryption).
  - Enables transparent, modular block device chaining without hardcoded kernel driver layers.

### 4.2 Haiku BFS Extended Attributes Indexing Engine
- **Inspiration**: Haiku Be File System (BFS) indexed extended attributes.
- **Target Architecture**:
  - Fast, B+tree-indexed file metadata and custom extended attributes (`xattr`).
  - Powers instant, desktop-wide file query operations (`find mime:text/x-rust`) without requiring background daemon indexing overhead.

---

## 5. Storage Subsystem Parity Matrix

| Feature | Inspired By | SigmaOS Component | Target Throughput / SLA | Status |
| :--- | :--- | :--- | :--- | :--- |
| **ARC Eviction Governor** | OpenZFS / FreeBSD | `src/filesystem/zfs_arc.rs` | 98%+ Cache Hit Ratio | Active Development |
| **Tiered Storage Engine** | Bcachefs | `src/filesystem/bcachefs.rs` | Automatic Tier Migration | Planned |
| **Ext4 JBD2 Journaling** | Linux Ext4 | `src/filesystem/ext4_jbd2.rs` | Zero Metadata Corruption | Fully Implemented |
| **GEOM Provider DAG** | FreeBSD | `src/storage/geom.rs` | Sub-10ns Block Intercept | Active Development |
| **BFS Indexed Attributes**| Haiku OS | `src/filesystem/bfs.rs` | Sub-1ms Query Search | Fully Implemented |

---

## 6. Implementation & Verification Protocol

1. **Zero External Dependencies**: Direct block allocation and VFS mapping in Safe Rust `#![no_std]`.
2. **Crash Consistency Verification**: Tested via simulated unannounced power-loss injection and journal replay verification.
