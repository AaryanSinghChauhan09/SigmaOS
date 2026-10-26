# SigmaOS Secondary Memory Subsystem Policy

## 1. Overview and Objective

SigmaOS manages secondary memory—persistent storage, block devices, filesystems, caches, and paging-backed storage—as an integrated, safe, observable, and recoverable lifecycle subsystem. The primary objective is to maintain secondary memory as a storage subsystem that enumerates devices, manages block I/O, mounts filesystems, caches data, pages to disk, reclaims space, and recovers from failures without corruption or undefined behavior.

## 2. Authoritative Storage Lifecycle

Every persistent storage device and path in SigmaOS adheres strictly to a single authoritative lifecycle:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Lifecycle Stage Guarantees

| Stage | Required Behavior |
|---|---|
| **Probe** | Hardware or virtio controller discovered, PCI/device capabilities reported. |
| **Identify** | Storage geometry, sector/block size, capacity, and operational limits determined. |
| **Block I/O** | Requests validated against bounds, queued in lock-free ring, completed exactly once. |
| **Cache** | Invalidation, writeback dirtying, LRU eviction, and cache coherence explicitly defined. |
| **Filesystem** | Superblock read, transaction recovery validated, mount/unmount lifecycle safe. |
| **Mount** | VFS mount entry attached, namespace isolation enforced, custom mount flags applied. |
| **Read/Write** | Bounded buffer reads/writes, transaction sequence numbering, lost-update prevention. |
| **Flush** | Durability semantics explicit, sync flush barriers enforced down to block device. |
| **Unmount** | Resources, buffer locks, and references released exactly once with rollback safety. |
| **Detach** | Safe device unplug or driver detachment, active queues drained or aborted. |
| **Recovery** | Failure during any stage triggers journal replay/rollback without corruption. |

## 3. Subsystem Architecture Classes

### 3.1 Block Layer Class
SigmaOS employs a **hybrid bio/request-based block layer**:
- **Bio-based submission**: Low-latency, direct block I/O requests for virtualized devices (VirtIO-blk) and fast NVMe ring buffers.
- **Request-based scheduling**: Queue-based request aggregation, elevator sorting, and deadline scheduling for high-throughput persistent disks.

### 3.2 Device Classes
The subsystem explicitly supports four primary block device classes:
1. **NVMe**: Non-Volatile Memory Express controller with multi-queue support and ZNS (Zone Namespaces).
2. **SATA/AHCI**: Standard Serial ATA controllers with NCQ (Native Command Queuing).
3. **VirtIO Block**: VirtIO-blk para-virtualized devices with split/packed ring descriptors.
4. **USB / Mass Storage**: Bulk-Only Transport (BOT) and USB Attached SCSI (UAS) storage devices.

### 3.3 Cache Class
- **Hybrid Page & Buffer Cache**: Unifies block device buffer caches with virtual memory page caches.
- **LRU Eviction & Watermarks**: Multi-tiered watermark tracking (`Normal`, `HighPressure`, `OomCritical`) with background dirty-page writeback workers.

### 3.4 Filesystem Class
- Supports ZFS (OpenZFS-inspired CoW pool management), OverlayFS (container layers), TmpFS (RAM-backed transient filesystem), and SigmaFS (custom POSIX-compliant CoW journaling filesystem).

## 4. Operational Policies

### 4.1 Mount Policy
- Mount operations must be atomic and race-free.
- Target mount points must exist and be free of concurrent unmount operations.
- Failure during mount initialization triggers a complete rollback to `Unmounted` state (`MountError::RollbackSuccessful`).

### 4.2 Flush Policy
- Synchronous flush requests (`VIRTIO_BLK_T_FLUSH`, `fsync`, `fdatasync`) issue immediate barrier commands to the underlying block driver.
- Dirty cache pages are flushed before unmount or unmap operations proceed.

### 4.3 Reclaim and OOM Policy
- When memory pressure hits `HighPressure`, background cache reclamation evicts clean pages.
- Under `OomCritical`, dirty pages are synchronously written back, and unmapped cache pages are forcibly reclaimed.
- OOM metrics (`CacheOomMetrics`) are tracked continuously and exposed via diagnostics.

### 4.4 Typed Error Policy
All storage operations must return strongly-typed errors. Generic untyped failures or accidental panics are prohibited:
- `BlockError`: `RequestExhaustion`, `LostCompletion`, `DoubleCompletion`, `QueueFull`, `DeviceDetached`.
- `CacheError`: `NotCached`, `CacheFull`, `DoubleEviction`, `WritebackFailed`, `InvalidPage`.
- `FsCoreError`: `CapacityExceeded`, `LostUpdateDetected`, `FsCorrupted`.
- `MountError`: `AlreadyMounted`, `DoubleUnmount`, `ResourceBusy`, `RollbackSuccessful`.

### 4.5 Queue and Allocation Limits
- Kernel request queues are bounded (e.g. maximum 256 pending requests per device queue).
- Cache metadata structures allocation is bounded by configurable physical memory percentages (default max 25% of RAM).

### 4.6 NUMA Behavior
- Block request queues and DMA buffer pools are allocated on the local NUMA node corresponding to the PCIe host bridge interrupt domain.

### 4.7 Storage Hotplug Behavior
- Upon device detachment, active pending requests transition immediately to `BlockError::DeviceDetached`.
- Mount points attached to detached storage transition to read-only state, preventing further dirty allocations.
