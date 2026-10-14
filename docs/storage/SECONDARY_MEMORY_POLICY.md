# SigmaOS Secondary Memory Policy Specification

## 1. Overview & Objective

The objective of the SigmaOS secondary memory subsystem is:

> **Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.**

This policy governs persistent storage, block devices, filesystems, block/page caches, and paging-backed storage across SigmaOS kernel and userland subsystems.

---

## 2. Authoritative Storage Lifecycle Contract

All storage paths in SigmaOS MUST adhere to the authoritative 11-stage lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Matrix & Behavioral Requirements

| Stage | Stage Name | Required Behavior & Standards |
|---|---|---|
| 1 | **Probe** | Device discovered, bus capabilities (PCIe/USB/VirtIO) reported without blocking system execution. |
| 2 | **Identify** | Geometry, sector size (512B/4096B), total LBA blocks, and queue capacity limits identified and validated. |
| 3 | **Block Layer** | Block I/O requests validated, bounded queue allocation enforced, scheduled, and completed exactly once. |
| 4 | **Cache** | Page cache / buffer cache lookups, cache line invalidation, dirty page marking, and coherence rules defined. |
| 5 | **Filesystem** | Superblock/inode initialization, geometry check, and safe filesystem lifecycle binding. |
| 6 | **Mount** | Mount point association with race-free state transitions, capability token verification, and isolation. |
| 7 | **Read/Write** | Bounded I/O data transfers with range checks, permission checks, and atomic transaction journaling. |
| 8 | **Flush** | Data durability semantics made explicit (synchronous flush, FUA, barrier commands) before transaction commit. |
| 9 | **Unmount** | Resources, buffer references, and dirty writebacks released cleanly with zero reference leaks. |
| 10 | **Detach** | Device reference zeroed, driver context torn down, and device node safely detached from VFS. |
| 11 | **Error / Recovery** | I/O errors typed and propagated (`BlockError`), dirty state salvaged or rolled back, filesystem state preserved without corruption. |

---

## 3. Subsystem Architecture & Classification

### 3.1 Block Layer Class
- **Architecture**: Hybrid Request-Based / Bio-Based Block Layer (`BoundedBlockDeviceManager`).
- **Queueing**: Bounded ring queues per device (`BoundedBlockRequest`).
- **Concurrency**: Lock-free atomic state transitions (`Unallocated`, `Pending`, `Dispatched`, `Completed`, `Failed`, `Freed`).

### 3.2 Device Classes
- **VirtIO Block**: VirtIO 1.0/1.1 spec compliant DMA queueing with ring descriptor validation (`src/storage/virtio.rs`).
- **NVMe**: Zone Namespaces (ZNS), NVMe 2.0 command submission / completion queues with MSI-X interrupt vectors.
- **SATA / AHCI**: Command List structures, FIS processing, and NCQ (Native Command Queuing) up to 32 tags.
- **USB Mass Storage**: Bulk-Only Transport (BOT) and USB Attached SCSI Protocol (UASP) endpoint drivers.

### 3.3 Cache Class
- **Architecture**: Hybrid Page Cache & Buffer Cache (`BlockCacheManager`).
- **Policy**: LRU (Least Recently Used) eviction with dirty-page writeback watermarks (`watermark_high`, `watermark_low`).
- **Coherence**: Atomic page status locks preventing lost writebacks and double evictions under OOM pressure.

### 3.4 Filesystem Class
- **SigmaFS / Core FS**: POSIX-compatible filesystem engine supporting journaling, transactions, and extended attributes.
- **ZFS / Btrfs**: Copy-on-Write (CoW) multi-device pool filesystems with checksum verification and snapshot manager.
- **Overlay & Tmpfs**: In-memory ephemeral filesystems with explicit backing-store page boundaries.

---

## 4. Operational Policies

### 4.1 Mount Policy
- Mount point bindings must be atomic and race-free (`MountManager`).
- Double-mount and double-unmount attempts return explicit error codes (`MountError::AlreadyMounted`, `MountError::NotMounted`).
- Unmount performs mandatory dirty buffer flushes; failure during unmount triggers safe rollback to operational state.

### 4.2 Flush Policy
- Synchronous `flush()` commands issue barrier operations down to device hardware (VirtIO `VIRTIO_BLK_T_FLUSH`, NVMe `Flush`).
- Transaction log commits require hardware write acknowledgement before block update clearance.

### 4.3 Reclaim Policy
- Memory pressure triggers kernel cache reclaim.
- Reclaim candidate pages must be cleanly written back if dirty before eviction.
- Under severe OOM, storage cache metrics (`total_pages`, `dirty_pages`, `reclaimed_pages`) are reported to the OOM governor.

### 4.4 Error Policy
- All storage I/O operations must return typed error variants (`BlockError`, `CacheError`, `FsCoreError`, `MountError`).
- Unhandled panic on I/O failure is strictly forbidden in kernel storage paths.
- Corruption events trigger read-only fallback mode (`FsHealthStatus::ReadOnly`) to preserve on-disk state.

### 4.5 Limits & Bounded Execution
- Maximum pending request queue size: 1024 requests per device.
- Maximum block cache entries: 65,536 pages (256 MB) default limit.
- Maximum active mounts: 256 system-wide mount entries.

### 4.6 NUMA & Hotplug Behavior
- Request allocations originate from the memory node matching the PCIe device host bridge domain.
- Device detach events safely invalidate pending I/O queues and return `BlockError::DeviceDetached` for in-flight requests.
