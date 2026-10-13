# SigmaOS Secondary Memory Policy Specification

## 1. Overview

Secondary memory in SigmaOS represents persistent storage devices, block I/O processing engines, page and buffer caching mechanisms, filesystems, and swap-backed paging stores. Managing secondary memory as an authoritative lifecycle subsystem guarantees safe, observable, and recoverable storage operations across all hardware platforms and virtio virtualized environments.

## 2. Architecture & Subsystem Classes

### 2.1 Block Layer Class
- **Type**: Hybrid Request-Based & Bio-Based Engine (`BoundedBlockDeviceManager`).
- **Request Allocator**: Bounded request queue with typed `BlockError::RequestExhaustion`.
- **Completion Model**: Exact-once completion tracking with `BlockError::DoubleCompletion` and timeout-based lost-completion detection (`BlockError::LostCompletion`).

### 2.2 Device Classes
- **NVMe 2.0**: High-performance multi-queue submission/completion queue devices (`NvmeBlockDevice`).
- **SATA/SSD**: Flash Translation Layer (FTL) devices with wear-leveling simulation (`SsdBlockDevice`).
- **VirtIO Block**: Paravirtualized block storage devices with ring queue buffer descriptors (`VirtIoBlockDevice`).
- **RAM Disk / Loop Device**: In-memory and file-backed block devices.

### 2.3 Cache Class
- **Type**: Hybrid Page Cache & Buffer Cache (`BlockCacheManager`).
- **Eviction Strategy**: Least-Recently-Used (LRU) page eviction with automatic dirty page writeback.
- **Watermarks**: High and low watermark triggers for background writeback and OOM pressure mitigation.

### 2.4 Filesystem Class
- **Core Engine**: `FilesystemCoreManager` and `MountManager`.
- **Supported Formats**: SigmaFS (native), Ext4, Btrfs, ZFS, OverlayFS, TmpFS, DevTmpFS, ProcFS, SysFS.

## 3. Operational Policies

### 3.1 Mount & Unmount Policy
- **Concurrency**: Lock-free or lock-ordered race-free mount entry additions (`MountManager`).
- **Double-Unmount Protection**: Explicit state checking (`MountState::Mounted` -> `MountState::Unmounting` -> `MountState::Unmounted`) preventing double-unmount corruption.
- **Rollback Safety**: Transactional rollback on initialization or mounting failures.

### 3.2 Flush & Durability Policy
- **Explicit Semantics**: Flush commands (`BlockOpCode::Flush`) force write-ahead journal logs and cache dirty buffers to physical non-volatile storage.
- **Barrier Commands**: Support for I/O barriers ensuring sequential order across power-loss boundaries.

### 3.3 Space Reclaim & OOM Policy
- **Quota Bounds**: Strict upper bounds on volume capacity (`max_capacity_bytes`) and active inodes.
- **OOM Metrics**: Observable metrics (`CacheOomMetrics`) exposing page usage, watermarks, and reclamation counts.
- **Automatic Trimming**: Periodic execution of `BlockOpCode::DiscardTrim` for flash storage block zeroing.

### 3.4 Error Handling & Propagation
- All secondary storage failures MUST return typed errors (`BlockError`, `CacheError`, `FsCoreError`, `MountError`).
- Panic on I/O error is strictly prohibited in kernel storage execution paths.

### 3.5 NUMA & Hotplug Behavior
- **NUMA Local Allocation**: Storage request queues and buffer descriptors prioritize NUMA node-local memory pools.
- **Hotplug Invalidation**: Detaching a device triggers `BlockError::DeviceDetached`, marks active requests as failed, invalidates cache pages for that device ID, and safely unmounts associated volumes.
