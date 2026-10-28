# SigmaOS Secondary Memory Subsystem Policy

## 1. Overview & Objectives

SigmaOS manages secondary memory—persistent storage, block devices, filesystems, caches, and paging-backed storage—as a safe, observable, recoverable lifecycle subsystem.

The secondary memory subsystem objective is to:
> **Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.**

---

## 2. Authoritative Storage Lifecycle Contract

SigmaOS enforces a single authoritative lifecycle contract across all storage paths:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| **Probe** | Device discovered, hardware capabilities reported |
| **Identify** | Geometry, sector size, queue limits, and capabilities known |
| **Block I/O** | Requests validated, queued, completed exactly once |
| **Cache** | Invalidation, dirty page writeback, and coherence defined |
| **Filesystem** | Mount/unmount lifecycle safe and transactional |
| **Flush** | Data durability semantics explicit across block and cache layers |
| **Unmount** | Resources, references, and buffers released exactly once |
| **Recovery** | Failure does not corrupt filesystem state or leak state |

---

## 3. Subsystem Architecture Classes

### 3.1 Block Layer Class
- **Hybrid Request-Based Architecture**: Supports direct bio request dispatch and multi-queue ring submission (`BoundedBlockDeviceManager`).
- **Bounded Request Queues**: Kernel request allocations are strictly bounded to prevent unbounded kernel memory growth. Request exhaustion returns a typed error (`BlockError::RequestExhaustion`).
- **Exact-Once Completion**: Double completion returns `BlockError::DoubleCompletion`; lost completion triggers timeout detection and failure status (`BlockError::LostCompletion`).

### 3.2 Device Class
- **NVMe 2.0**: High-performance submission/completion multi-queue device model (`NvmeBlockDevice`).
- **VirtIO Block**: Virtqueue-backed async I/O driver (`VirtIoBlockDevice`).
- **SATA / SSD**: Flash Translation Level (FTL) wear-leveling simulation (`SsdBlockDevice`).
- **RAM Disk / Loop Device**: In-memory and file-backed block devices.

### 3.3 Cache Class
- **Page & Buffer Cache**: Managed by `BlockCacheManager` in `src/fs/cache.rs`.
- **LRU Eviction**: Eviction is race-free and deterministic based on timestamp/access metrics.
- **OOM Watermark Metrics**: `CacheOomMetrics` measures active, dirty, evicted, and reclaimed pages across `Normal`, `HighPressure`, and `OomCritical` levels.

### 3.4 Filesystem Class
- **Managed Core**: Managed by `FilesystemCoreManager` in `src/filesystem/core.rs`.
- **Supported FS Types**: Ext4, Btrfs, ZFS, Overlay, Tmpfs, and SigmaFS.
- **Capacity Bounds**: Volume total/used/free byte and inode limits strictly enforced (`VolumeCapacityBounds`).

---

## 4. Subsystem Operational Policies

### 4.1 Mount Policy
- Managed by `MountManager` in `src/filesystem/mount.rs`.
- **State Machine**: `Unmounted -> Mounting -> Mounted -> Unmounting`.
- **Rollback Guarantee**: Initialization failure during mount automatically rolls back dirty state (`MountError::RollbackSuccessful`).
- **Double-Unmount Protection**: Attempting to unmount an already unmounted path returns `MountError::DoubleUnmount`.

### 4.2 Flush Policy
- All dirty cache pages must be written back to underlying block devices before device unmount or detachment.
- `BlockOpCode::Flush` forces explicit hardware controller flush operations.

### 4.3 Reclaim Policy
- Under memory pressure (`HighPressure` / `OomCritical`), clean LRU cache pages are reclaimed first.
- Dirty pages are enqueued for writeback before reclamation.

### 4.4 Error Policy
- All storage I/O and lifecycle operations return strongly-typed Rust `Result<T, E>` enums: `BlockError`, `CacheError`, `FsCoreError`, `MountError`.
- Unbounded loops or hidden panics on storage failure are strictly prohibited.

### 4.5 Limits & Bounds
- **Max Requests**: Bounded per block manager instance.
- **Max Cache Pages**: Bounded cache entry limits.
- **Max Volumes & Mount Entries**: Bounded volume tables.

### 4.6 NUMA & Hotplug Behavior
- Cache pages and I/O buffers bind to local NUMA memory nodes where available.
- Storage hotplug triggers immediate request drain, cache invalidation, clean unmount, and device removal.
