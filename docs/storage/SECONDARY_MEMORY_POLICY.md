# SigmaOS Secondary Memory Policy

## Objective

Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem:
enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space,
and recover from failures without corruption or undefined behavior.

## Authoritative Storage Lifecycle Contract

The SigmaOS secondary memory subsystem follows one authoritative, strictly ordered lifecycle:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements and Behaviors

| Stage | Required Behavior |
|---|---|
| **Probe** | Device discovered, bus enumeration executed, capabilities reported. |
| **Identify** | Geometry, sector size, command limits, and media characteristics established. |
| **Block I/O** | Requests validated, queued in bounded rings, completed exactly once. |
| **Cache** | Page/buffer caching active; eviction, invalidation, and coherence strictly defined. |
| **Filesystem** | Mount/unmount lifecycle safe; directory and inode mappings bounded. |
| **Mount** | Filesystem attached to VFS hierarchy without races or stale reference leaks. |
| **Read / Write** | Data transferred through validated boundaries with POSIX semantics. |
| **Flush** | Data durability and barrier/FUA semantics explicit across volatile tiers. |
| **Unmount** | Resources, locks, and buffer references released exactly once. |
| **Detach** | Device safely removed from driver hierarchy and descriptor tables. |
| **Recovery** | Hardware or software failure detected and isolated without corrupting filesystem state. |

## Subsystem Architecture Classifications

### 1. Block Layer Class
- Hybrid bio-request architecture:
  - Upper layers submit asynchronous `BlockRequest` descriptors.
  - Lower block drivers process requests via bounded FIFO queues with atomic tracking.
  - Request queue depth is strictly bounded to prevent kernel exhaustion.
  - Exactly-once completion is enforced; double-completion returns typed `BlockError::DoubleCompletion`.

### 2. Device Class
- **VirtIO Block (`virtio-blk`)**: Paravirtualized block device for virtualized and emulation environments (QEMU/KVM).
- **NVMe (`nvme`)**: Direct PCI memory-mapped submission/completion queues for high-performance non-volatile storage.
- **SATA / AHCI**: Command queueing (NCQ) block interface for standard SATA hard drives and SSDs.
- **USB Storage**: SCSI command pass-through over USB mass storage protocol.
- **RamDisk / Loop**: In-memory ephemeral block devices for staging, boot initramfs, and testing.

### 3. Cache Class
- Hybrid page cache and buffer cache architecture:
  - Block-level caching managed by `BlockCacheManager`.
  - LRU (Least Recently Used) replacement policy with bounded page limits.
  - Synchronized dirty page tracking with deterministic writeback flushing.
  - Observable watermark tiers (`Normal`, `PressureWarn`, `OomCritical`) for proactive memory management.

### 4. Filesystem Class
- **Native SigmaFS**: Capability-aware, self-healing copy-on-write filesystem.
- **OpenZFS / ZFS**: Storage pool (zpool) integrated CoW filesystem with ARC caching and dataset snapshots.
- **ext4 / ext4_mount**: Standard journaling block filesystem compatibility layer.
- **tmpfs / devfs / sysfs / procfs**: Virtual in-memory pseudofilesystems for OS state and kernel interfaces.
- **OverlayFS**: Layered read-only lower and read-write upper branches for live/immutable booting.

## Subsystem Policies

### Mount Policy
- Mount registration is atomic and race-free via `MountManager`.
- Duplicate mounts on the same target mount point are rejected (`MountError::AlreadyMounted`).
- Mount operations validate device readiness and filesystem superblock integrity before exposure.
- Unmount transitions through an atomic state machine, releasing all active references and locks.
- Double-unmount attempts return `MountError::NotMounted`.

### Flush and Durability Policy
- Flush commands issue write barriers to underlying physical media.
- Dirty cache pages are flushed periodically or synchronously upon explicit `sync()` / `fsync()` syscalls.
- Failure during writeback isolates the affected block, marks the filesystem degraded, and prevents lost writebacks.

### Reclaim Policy
- Eviction from block cache occurs when capacity reaches configured thresholds.
- Active or locked pages are protected from eviction.
- Clean pages are evicted immediately; dirty pages trigger synchronous or asynchronous writeback before release.
- Cache metadata is statically bounded to prevent metadata memory bloat.

### Error Handling Policy
- Every storage error is strongly typed (no silent drops or undefined conversions).
- Errors propagate to userspace via standard POSIX error codes (`EIO`, `ENOMEM`, `EBUSY`, `ENOSPC`).
- I/O timeouts and lost completions trigger device reset and diagnostic logging.

### Limits and Bounds
- Request queues have hard limits (default: 1024 concurrent requests per device).
- Cache managers enforce strict page capacity limits.
- Volume allocations are bounded by registered volume size; exceeding capacity returns `FsCoreError::CapacityExceeded`.

### NUMA Behavior
- Storage queues and cache pages are bound to the local NUMA memory node where the device controller resides.
- Cross-node request dispatching employs queue affinity to minimize inter-socket memory bus traffic.

### Behavior Under Storage Hotplug
- Device detachment initiates graceful drain of active requests.
- New submissions to detached devices are rejected immediately with `BlockError::NotFound`.
- Filesystems mounted on detached media transition to read-only or error-recovery state to prevent corrupt writes.
