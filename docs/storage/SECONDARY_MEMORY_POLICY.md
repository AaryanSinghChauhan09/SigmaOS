# SigmaOS Secondary Memory Policy

## Objective

Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

## Authoritative Storage Lifecycle Contract

SigmaOS secondary memory follows one authoritative lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Contracts and Required Behaviors

| Stage | Required Behavior | Invariants / Guarantees |
|---|---|---|
| **Probe** | Device discovered, capabilities reported | Device bus scan identifies present controllers (NVMe, SATA, virtio, USB storage). Hardware IDs assigned. |
| **Identify** | Geometry, sector size, and limits known | Sector size (e.g. 512, 4096), logical block counts, queue depth, maximum transfer chunks, and alignment known. |
| **Block Layer** | Requests validated, queued, completed exactly once | Bounded request allocation. Lost-completion and double-completion prevented. Request reuse follows strict generation token policy. |
| **Cache** | Invalidation and coherence defined | Hybrid page/buffer cache. Race-free LRU eviction, dirty page tracking, writeback throttling, observable watermark metrics (Normal, HighPressure, OomCritical). |
| **Filesystem** | Mount/unmount lifecycle safe | Bounded capacity and inode limits. Atomic transaction logging, sequence numbering, lost-update prevention. |
| **Mount** | Hierarchy integration failure-safe | Race-free mount/unmount. Failure triggers clean rollback without leaking filesystem descriptors or locks. |
| **Read / Write** | Bounded, isolated data transfer | Capability-checked I/O, range validity, memory bounds, error classification. |
| **Flush** | Data durability semantics explicit | Explicit cache sync and storage barriers. Guarantees persistence of dirty buffers before return. |
| **Unmount** | Resources and references released exactly once | Double-unmount prevented. All pending writes flushed, file handles invalidated, mount state returned cleanly. |
| **Detach** | Hotplug and device removal safe | In-flight requests gracefully failed with typed `DeviceDetached` errors. Device registrations reclaimed. |
| **Error / Recovery** | Failure does not corrupt filesystem state | All errors are typed (`BlockError`, `CacheError`, `FsCoreError`, `MountError`). Failures preserve journal/transaction consistency. |

## Subsystem Architecture Classifications

### 1. Block Layer Class
SigmaOS employs a **hybrid request/bio architecture**. High-level block I/O requests (`BlockRequest`) define operation codes (`Read`, `Write`, `Flush`, `Trim`), target block addresses, sector counts, and memory buffers. The block manager translates these into hardware-optimized command queues with bounded depths.

### 2. Device Class
Supported block storage device types:
- **NVMe**: Non-Volatile Memory Express controller interface with multi-queue support.
- **SATA / AHCI**: Serial ATA interfaces for traditional hard drives and SATA SSDs.
- **virtio-blk**: Virtual I/O block device driver (`VirtIoBlockDevice`) for paravirtualized environments.
- **USB Mass Storage**: Bounded USB storage devices.
- **Loop / RamDisk**: In-memory backing stores for ephemeral and test execution.

### 3. Cache Class
SigmaOS utilizes a **hybrid page cache and buffer cache** architecture (`BlockCacheManager`). Cache pages track dirty flags, access frequencies, LRU eviction order, and lifecycle states (`Active`, `Flushing`, `Evicting`, `Evicted`). Eviction ensures dirty blocks are written back before release.

### 4. Filesystem Class
Pluggable filesystem architecture supporting:
- **ZFS**: Advanced copy-on-write storage pool and dataset management.
- **OverlayFS**: Layered filesystem composition for container and live-boot isolation.
- **Tmpfs**: Ephemeral memory-backed filesystem.
- **SigmaFS / Ext4-inspired core**: POSIX-compatible bounded filesystem core manager (`FilesystemCoreManager`).

## Policies

### Mount Policy
Mount points are tracked in a unified mount table (`MountManager`). Mount operations transition through `Unmounted -> Mounting -> Mounted`. If filesystem initialization or verification fails during mount, the mount manager performs an atomic rollback to `Unmounted`. Double mounts and double unmounts are strictly rejected with typed errors.

### Flush Policy
Dirty cache pages are scheduled for periodic writeback or synchronized on demand via flush requests (`VIRTIO_BLK_T_FLUSH` / `BlockOpCode::Flush`). Flush operations ensure that on-device volatile write caches commit data to non-volatile media before signaling completion.

### Reclaim Policy
When cache occupancy reaches the high-watermark threshold (`HighPressure`), background reclaim activates, selecting clean LRU pages for immediate eviction and queuing dirty pages for writeback. Under `OomCritical` pressure, synchronous reclaim is enforced, and non-essential caches yield pages.

### Error Policy
No storage path may panic or produce undefined behavior on failure. All I/O errors must be typed (`BlockError`, `CacheError`, `FsCoreError`, `MountError`), diagnosable via structured logs, and propagated to calling subsystems.

### Resource Limits
- Block manager request queue capacity is strictly bounded (e.g. 1024 concurrent requests per device).
- Cache metadata and page tables have fixed maximum bounds.
- Filesystem volume byte and inode allocations are strictly verified against volume capacity bounds (`VolumeCapacityBounds`).

### NUMA Behavior
On NUMA architectures, block device queues, interrupt affinities, and cache page buffers are allocated from the memory node local to the storage controller or calling CPU core, avoiding cross-node bus traffic.

### Behavior Under Storage Hotplug
Device hotplug removal initiates graceful teardown:
1. Pending queued requests that have not been dispatched are failed with `BlockError::DeviceDetached`.
2. Block cache manager marks remaining pages for the device as invalidated.
3. Active mounts receive an emergency read-only remount or unmount trigger.
4. Device IDs and controller resources are freed.
