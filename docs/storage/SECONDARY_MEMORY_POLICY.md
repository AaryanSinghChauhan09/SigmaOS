# Secondary Memory Subsystem Policy

## Authoritative Secondary Memory Lifecycle Contract

SigmaOS manages secondary memory—persistent storage, block devices, filesystems, caches, and paging-backed storage—as a single authoritative lifecycle subsystem:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| **Probe** | Device discovered, capabilities reported |
| **Identify** | Geometry, sector size, and limits known |
| **Block Layer** | Requests validated, queued, completed exactly once |
| **Cache** | Invalidation and coherence defined |
| **Filesystem** | Mount/unmount lifecycle safe |
| **Flush** | Data durability semantics explicit |
| **Unmount** | Resources and references released exactly once |
| **Recovery** | Failure does not corrupt filesystem state |

---

## Secondary Memory Classes & Architectural Definitions

### 1. Block Layer Class
- **Architecture**: Hybrid Request-Based and Bio-Based Block Layer.
- **Queueing Discipline**: Bounded elevator queue with priority levels and strict request limits.
- **Completion Guarantee**: Exact-once completion tracking via atomic request ID state transitions (`Pending -> Dispatched -> Completed/Failed`).
- **Timeout & Lost-Completion Handling**: Subsystem periodically scans for dispatched requests exceeding submission timeout (`timeout_ms`), transitioning lost requests to `Failed` state to prevent kernel hangs.

### 2. Device Class
- **Supported Storage Classes**:
  - **NVMe 2.0**: High-throughput multi-queue block devices (`NvmeBlockDevice`).
  - **virtio-blk**: Virtualized block devices for QEMU hypervisors (`VirtIoBlockDevice`).
  - **SATA / SSD**: Solid-state flash storage with FTL wear-leveling simulation (`SsdBlockDevice`).
  - **RamDisk & Loop Devices**: In-memory and file-backed virtual block devices.

### 3. Cache Class
- **Architecture**: Hybrid Page Cache and Buffer Cache Manager (`BlockCacheManager`).
- **Page Eviction**: Least Recently Used (LRU) eviction policy.
- **Watermark Enforcement**:
  - `Normal`: Below 75% cache capacity.
  - `HighPressure`: 75% to 89% cache capacity.
  - `OomCritical`: 90%+ cache capacity, triggering immediate background page eviction and writebacks.
- **Lost-Writeback Prevention**: Dirty pages marked for eviction are synchronized to disk prior to buffer deallocation.

### 4. Filesystem Class
- **Supported Filesystems**: ZFS (OpenZFS-inspired ARC cache and datasets), OverlayFS, tmpfs, ext4, and custom SigmaFS.
- **Capacity Bounds**: Volume capacity (`VolumeCapacityBounds`) explicitly limits total bytes, free bytes, and total inode allocations.
- **Lost-Update Prevention**: Write operations use atomic sequence numbers and transaction IDs (`FileTransactionRecord`). Mismatched sequence commits return `LostUpdateDetected`.

### 5. Mount Policy
- **Lifecycle Control**: Enforced through `MountManager` with explicit state transitions (`Unmounted -> Mounting -> Mounted -> Unmounting`).
- **Active Reference Protection**: Mount points with open file descriptors or active processes return `ResourceBusy` on unmount attempts.
- **Double-Unmount Protection**: Unmounting an already unmounted or missing mount point returns `DoubleUnmount` or `NotMounted`.
- **Atomic Rollback**: If filesystem initialization fails during mount, the state rolls back to `Unmounted` cleanly without leaking mount table entries.

### 6. Flush & Reclaim Policy
- **Data Durability**: Explicit flush operations (`flush()`) flush dirty buffer caches and issue hardware barriers.
- **Reclaim Policy**: OOM metrics track total pages, dirty pages, eviction count, and reclaimed bytes, allowing the system to shed clean cache pages under memory pressure.

### 7. Error & Recovery Policy
- **Typed Errors**: Storage operations return typed errors (`BlockError`, `CacheError`, `FsCoreError`, `MountError`).
- **Recovery Guarantees**: Storage failures (e.g., power interruption or unexpected hotplug detach) preserve on-disk consistency through transaction rollback and writeback journaling.

### 8. NUMA Behavior & Limits
- **NUMA Local Allocation**: Buffer cache pages and block request structures are bound to NUMA memory domains corresponding to the local PCIe root complex.
- **Allocation Limits**: Fixed maximum request slots (`max_requests`) and maximum cache page entries (`max_pages`) prevent unbounded memory allocation in kernel space.

### 9. Hotplug Behavior
- **Device Detachment**: Unexpected device removal transitions active requests to `DeviceDetached` state and unregisters the device cleanly from the block device manager.
