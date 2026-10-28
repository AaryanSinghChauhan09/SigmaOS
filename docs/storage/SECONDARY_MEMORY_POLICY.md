# SigmaOS Secondary Memory Policy

## Executive Summary

SigmaOS manages secondary memory—persistent storage, block devices, filesystems, caches, and paging-backed storage—as a safe, observable, and recoverable subsystem. The secondary memory subsystem guarantees crash consistency, predictable resource limits, and deterministic error handling across device enumeration, block I/O, cache eviction, filesystem mounting, page swapping, and failure recovery.

## Authoritative Lifecycle State Machine

Every secondary memory storage path must follow this single authoritative 11-stage lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Contracts

| Stage | Required Behavior | Invariant / Failure Guarantee |
|---|---|---|
| **Probe** | Device hardware discovered during boot or hotplug event | Non-blocking detection, safe drop on missing hardware |
| **Identify** | Sector size, geometry, queue depth, and capability flags extracted | Unidentified devices rejected with `NotFound` or `InvalidBlockSize` |
| **Block Layer** | Requests enqueued, scheduled, and completed exactly once | Bounded queue depth; exhaustion returns `BlockError::RequestExhaustion` |
| **Cache** | In-memory block and page caching with LRU eviction | Coherent read/write; dirty writebacks executed before eviction |
| **Filesystem** | On-disk metadata and directory structures parsed | Transactional metadata updates; lost updates strictly prevented |
| **Mount** | VFS mount point registered and isolated | Atomic mount table insertion; duplicate mounts return `MountError::AlreadyMounted` |
| **Read/Write** | I/O transfers executed against block cache or direct storage | Range checking enforced; out-of-bounds access returns `OutOfBounds` |
| **Flush** | Dirty cache pages synchronized to persistent medium | Durability barrier guaranteed upon completion |
| **Unmount** | Filesystem flushed, open file descriptors revoked, VFS node unlinked | Safe unmount guaranteed; double-unmount returns `MountError::DoubleUnmount` |
| **Detach** | Block device unbound and queue drained | Active transactions aborted safely; lost-completion timeout handled |
| **Error/Recovery**| SCSI sense keys mapped, journal replayed, cache invalidation | Non-corrupting error recovery; filesystem restored to last safe state |

---

## Subsystem Classes

### 1. Block Layer Class
SigmaOS employs a **Hybrid Request-Based / Bio-Based Block Layer**. Small, uniform sector operations use fixed-size request descriptors (`BoundedBlockRequest`), while high-throughput DMA and scatter-gather operations use bio-based descriptor chains.

### 2. Device Class
Supported device classes include:
- **NVMe 2.0+**: Multi-queue high-performance SSD storage with submission/completion queues (`NvmeBlockDevice`).
- **SATA / AHCI**: Legacy rotational disks and legacy SSDs.
- **VirtIO-Block**: Paravirtualized QEMU/KVM block storage (`VirtIoBlockDevice`).
- **RAM Disks / Loop Devices**: Memory-backed testing and installation targets.

### 3. Cache Class
SigmaOS implements a **Hybrid Page/Buffer Cache** managed by `BlockCacheManager`. The cache maintains an LRU eviction queue, tracks dirty page watermarks, enforces atomic writebacks, and reports OOM metrics when memory thresholds are exceeded.

### 4. Filesystem Class
SigmaOS supports a tiered filesystem architecture:
- **SigmaFS / ext4 / Btrfs**: Native transactional persistent filesystems.
- **ZFS / HAMMER2**: Advanced storage pool filesystems with snapshots and self-healing.
- **Overlay / Tmpfs**: Ephemeral memory and layered container filesystems.

---

## Operating Policies

### Mount Policy
- Mount point targets must be empty directory inodes within the VFS hierarchy.
- Mount operations execute under lock ordering: `vfs_lock -> mount_table_lock -> superblock_lock`.
- Rollback on failure: If filesystem initialization fails midway through mounting, all allocated VFS nodes and lock references are released immediately.

### Flush Policy
- Synchronous flushes (`fsync`, `sync`) drain the dirty block cache queue and issue hardware `BlockOpCode::Flush` commands.
- Asynchronous flushes execute via periodic kernel flusher threads when dirty page capacity reaches the 70% high-water mark.

### Reclaim & OOM Policy
- When system memory pressure reaches 90% (OOM watermark), `BlockCacheManager` triggers aggressive LRU eviction for clean cached blocks.
- Dirty blocks are prioritized for emergency background writebacks. Cache memory allocation fails gracefully without panics.

### Error Policy
- All storage I/O errors are strongly typed using `BlockError`, `CacheError`, `FsCoreError`, and `MountError`.
- Hardware errors map SCSI Sense Keys (`ScsiSenseData`) directly into kernel diagnostic logs without silent dropping.

### NUMA Behavior
- Block request queues and page cache allocations are bound to local NUMA nodes corresponding to the PCI root complex of the storage controller.

### Storage Hotplug Policy
- On device detachment, the block manager transitions active requests to `RequestState::Failed` with `BlockError::DeviceDetached`.
- In-flight requests timeout via `detect_lost_completions` and are reaped safely.
