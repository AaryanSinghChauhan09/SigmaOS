# SigmaOS Secondary Memory Subsystem Policy

## Objective

Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

## Authoritative Secondary-Memory Lifecycle Contract

SigmaOS defines one authoritative lifecycle for secondary memory:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

Every stage in the lifecycle enforces strict behavioral guarantees and typed errors:

| Stage | Required Behavior | Invariant & Enforcement |
|---|---|---|
| **Probe** | Device discovered, capabilities reported | Hardware bus scanning (PCI, virtio, USB, NVMe) reports vendor ID, class, and features. |
| **Identify** | Geometry, sector size, and limits known | LBA block count, sector size (512B/4KiB), queue depth, and alignment reported. |
| **Block Layer** | Requests validated, queued, completed exactly once | Bounded queue depth, race-free request allocation, lost-completion timeouts, double-completion prevention. |
| **Cache** | Invalidation and coherence defined | LRU page/buffer cache eviction, bounded metadata, dirty page writeback, OOM watermark metrics. |
| **Filesystem** | Mount/unmount lifecycle safe | Filesystem capacity bounds, superblock verification, POSIX directory/file model consistency. |
| **Mount** | Active mount registered, root inode isolated | Bounded mount tree, double-mount/double-unmount prevention, race-free namespace attachment. |
| **Read/Write** | Data transfer executing with W^X and lock bounds | Exact byte bounds checking, sector alignment, lock ordering enforcement, zero loss under concurrent access. |
| **Flush** | Data durability semantics explicit | Explicit `sync`, `fsync`, barrier ops, dirty buffer flushes before state transition or detach. |
| **Unmount** | Resources and references released exactly once | Reference counting drain, dirty writeback completion, unmount rollback on pending I/O, no dangling refs. |
| **Detach** | Device hotplug removal clean | Immediate queue drain/abort, return `BlockError::DeviceDetached` for in-flight requests, memory safety preserved. |
| **Error / Recovery** | Failure does not corrupt filesystem state | Logically transaction-journaled or CoW-protected operations, automatic degraded-state recovery, zero silent corruption. |

---

## Subsystem Architectural Design

### 1. Block Layer Class
- **Architecture**: Hybrid Request/Bio-Based Engine.
- **Request Engine**: `BoundedBlockDeviceManager` in `src/storage/block.rs` manages fixed-capacity request slots, preventing unbounded heap growth in kernel context.
- **Completion Model**: Exact-once completion tracking (`Pending` -> `Dispatched` -> `Completed` / `Failed`). Lost completions are detected via timestamped timeouts.

### 2. Device Class Support
- **NVMe 2.0**: High-performance multi-queue submission/completion queue device (`NvmeBlockDevice`).
- **VirtIO Block**: VirtIO 1.1 spec implementation (`VirtIoBlockDevice` in `src/storage/virtio.rs`) with virtqueue split-ring descriptors.
- **SATA / SSD**: Flash Translation Layer (FTL) wear-leveling simulation (`SsdBlockDevice`).
- **USB Storage / Loop / RAM Disks**: Standard block-oriented abstractions.

### 3. Cache Class Architecture
- **Hybrid Page & Buffer Cache**: `BlockCacheManager` in `src/fs/cache.rs` provides 4KiB page-aligned cache entries with LRU eviction and dirty-flag writebacks.
- **Watermark & OOM**: Explicit high/low watermarks trigger background dirty page flushes and page reclamation before kernel OOM conditions.

### 4. Filesystem Classes
- **ZFS / Sovereign SigmaFS**: Advanced Copy-on-Write (CoW) transaction-based filesystems.
- **Ext4 / FAT32 / Btrfs / SquashFS**: Standard POSIX-compatible block filesystems.
- **Tmpfs / Devtmpfs / Procfs / Sysfs**: Synthetic in-memory filesystems.
- **OverlayFS**: Tiered layered filesystems for container and live environment support.

---

## Operations & Policies

### Mount Policy
- Active mounts are tracked in `MountManager` (`src/filesystem/mount.rs`).
- Unmounting requires zero active open file handles unless forced (`MNT_FORCE`).
- Attempts to double-unmount or unmount non-existent mount points return typed `MountError::NotMounted` or `MountError::AlreadyUnmounted`.

### Flush & Durability Policy
- `flush()` barriers are required prior to unmounting or detaching block devices.
- File writes can specify synchronous flags (`O_SYNC`, `O_DSYNC`) or issue `fsync()` to trigger immediate dirty buffer flush to block storage.

### Reclaim & OOM Policy
- Kernel memory pressure triggers secondary-memory cache eviction via `BlockCacheManager::reclaim_memory()`.
- Unused clean cache pages are freed instantly; dirty pages are scheduled for async writeback before liberation.

### Hotplug & Detach Policy
- Sudden detachment triggers device status flag `DeviceDetached`.
- All pending and dispatched operations fail gracefully returning `BlockError::DeviceDetached` without kernel panic.

### NUMA & Memory Binding
- Where NUMA topology is present, block request queues and buffer caches bind allocations to local NUMA memory domains to minimize cross-socket interconnect latency.
