# Secondary Memory Subsystem Policy

## 1. Executive Summary

SigmaOS manages secondary memory—persistent storage, block devices, filesystems, caches, and paging-backed storage—as a safe, observable, and recoverable lifecycle subsystem. This document establishes the governing policies, architectural rules, resource limits, and error handling contracts for all secondary memory subsystems in SigmaOS.

## 2. Storage Lifecycle Contract

Every storage device and filesystem in SigmaOS adheres strictly to an authoritative 11-stage lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| **Probe** | Hardware or virtual device discovered; bus capabilities and driver bindings reported. |
| **Identify** | Geometry, sector/block size, capabilities, and capacity limits queried and verified. |
| **Block I/O** | Request queues allocated; requests validated, queued, and completed exactly once. |
| **Cache** | Page/buffer cache mappings established; eviction and coherence state machines initialized. |
| **Filesystem** | Superblock/metadata probed; filesystem structural integrity validated. |
| **Mount** | Mount point bound; mount options enforced; superblock registered in mount namespace. |
| **Read/Write** | Data I/O executed with capability checks, boundary clamping, and transaction tracking. |
| **Flush** | Sync operations push dirty cache lines to persistent media; writeback durability guaranteed. |
| **Unmount** | Open file handles checked; dirty buffers flushed; mount references released cleanly. |
| **Detach** | Device removed or unbinding handled safely without stale pointer access or memory leak. |
| **Recovery** | I/O errors or unexpected disconnects handled via transaction rollback or journal replay. |

## 3. Subsystem Architecture Classes

### 3.1 Block Layer Class
- **Architecture**: Hybrid request-based and bio-based block device manager (`BoundedBlockDeviceManager`).
- **Queue Model**: Fixed-capacity request queues per block device (`MAX_REQUESTS` default: 64/128).
- **Request Lifecycle**: `Unallocated -> Pending -> Dispatched -> Completed/Failed -> Freed`.
- **Invariants**:
  - Double-completion and lost-completion are strictly prohibited (`BlockError::DoubleCompletion`, `BlockError::LostCompletion`).
  - Request queue exhaustion returns a typed error (`BlockError::RequestExhaustion`).

### 3.2 Device Classes
- **Supported Storage Classes**: NVMe 2.0, SATA (AHCI), VirtIO Block (`VirtIoBlockDevice`), USB Mass Storage, RAM Disk, Loop Devices.
- **Addressing**: Logical Block Addressing (LBA) with configurable block sizes (512B to 4096B).

### 3.3 Cache Class
- **Cache Architecture**: Hybrid Page Cache & Buffer Cache (`BlockCacheManager`).
- **Eviction Strategy**: Least Recently Used (LRU) with dirty page writeback.
- **Coherence**: Atomic page state transitions (`Active`, `Flushing`, `Evicting`, `Evicted`).
- **Pressure Management**: Three-tier watermark monitoring (`Normal`, `HighPressure`, `OomCritical`).

### 3.4 Filesystem Class
- **Supported Filesystems**: SigmaFS / Core FS (`FilesystemCoreManager`), ext4, ZFS (ARC inspired), tmpfs, overlayfs, devtmpfs, procfs, sysfs.
- **POSIX Conformance**: POSIX-compatible file descriptors, directory structures, locking, and permission evaluations.

## 4. Operational Policies

### 4.1 Mount Policy
- Mount point targets must be valid directory paths in the root hierarchy.
- Re-mounting an already mounted path returns `MountError::AlreadyMounted`.
- Unmounting requires no active file handle holds unless forced. Unmounting an unmounted target returns `MountError::DoubleUnmount`.
- Mount options (`CustomMountFlags`) support `read_only`, `no_exec`, `no_suid`, `sync`, and `bind`.

### 4.2 Flush & Durability Policy
- Synchronous writes (`sync = true`) execute immediate flush to persistent block media.
- Asynchronous cache writebacks occur automatically when dirty page watermarks exceed `HighPressure` threshold.
- Transaction records (`FileTransactionRecord`) ensure transaction logging and lost-update detection (`FsCoreError::LostUpdateDetected`).

### 4.3 Reclaim & OOM Policy
- Page cache eviction reclaims clean or flushed dirty pages under memory pressure.
- `CacheOomMetrics` provides real-time observability of cache hits, misses, evictions, dirty page counts, and reclaimed bytes.
- When memory pressure reaches `OomCritical`, the kernel halts new non-essential block allocations and triggers emergency cache flushing.

### 4.4 Hotplug & Detach Policy
- Device removal marks the block device status as `DeviceDetached`.
- Pending I/O requests on detached devices fail immediately with `BlockError::DeviceDetached`.
- Active mounts on detached devices transition to `FsHealthStatus::Degraded` or `FsHealthStatus::Corrupted` and reject new write operations.

### 4.5 NUMA & Affinity Behavior
- Block request queues and cache pages allocate memory preferentially from the local NUMA domain matching the PCI host bridge / storage controller affinity.

## 5. Security and Safety Rules
- All cross-user storage operations require capability verification (`CAP_SYS_ADMIN` for mount/unmount/format).
- Boundary-clamped memory access prevents buffer overruns during block DMA transfers.
- Safe Rust is preferred across all storage layers. Any `unsafe` block must document invariants, safety rationale, and locking disciplines.
