# SigmaOS Secondary Memory Subsystem Policy

## Objective

Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

## Authoritative Secondary Memory Lifecycle State Machine

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Lifecycle Stage Requirements

| Stage | Required Behavior | Invariant / Enforcement |
|---|---|---|
| **Probe** | Hardware device discovered, capability flags and interfaces reported. | No uncontrolled MMIO or DMA access prior to driver probe initialization. |
| **Identify** | Block device geometry, sector size (512B/4096B), and capacity limits known. | Geometry parameters verified before exposing block device handle. |
| **Block Layer** | Block I/O requests validated, enqueued in bounded ring queues, completed exactly once. | Typed error returned on request pool exhaustion; lost/double completions strictly prevented. |
| **Cache** | Block cache pages allocated, LRU cache eviction, writeback invalidation and cache coherence defined. | Cache page limit strictly bounded; double eviction prohibited. |
| **Filesystem** | Superblock/inode state initialized; capacity clamped; lost-update prevention enforced. | Transactional logging or copy-on-write semantics for metadata updates. |
| **Mount** | Mount point registered in namespace; flags evaluated; rollback guaranteed on mount failure. | Atomic mount state transition (`Unmounted` -> `Mounted`). |
| **Read/Write** | POSIX-compatible block and stream read/write operations with bounds check. | Capability token verification on file descriptors; offset overflow protection. |
| **Flush** | Dirty cache pages written back to physical storage; durable flush semantics explicit. | All dirty pages in cache range synced before transaction completion. |
| **Unmount** | Open handles closed; pending flush executed; filesystem references released exactly once. | Atomic transition (`Mounted` -> `Unmounted`); double-unmount prohibited. |
| **Detach** | Device driver safely detached; hot-unplug handled cleanly without kernel panics. | In-flight I/O requests drained or aborted with typed I/O error (`BlockError::DeviceDetached`). |
| **Recovery** | Failure, crash, or unexpected power loss recovered via journal/log replay without corruption. | FSCK or snapshot rollback restores system to a clean state without orphan allocations. |

## Subsystem Architectural Classes

### 1. Block Layer Class
SigmaOS utilizes a **hybrid request-based and bio-based block layer**:
- High-level POSIX and Virtual File System (VFS) operations create bio-like block ranges (`BioRequest`).
- Low-level driver interface maps bios into bounded ring queue requests (`BlockRequest`) handled asynchronously by block controllers (e.g., VirtIO, NVMe).

### 2. Device Class Support
- **VirtIO Block (`virtio-blk`)**: Ring queue descriptor chain I/O for virtualized hypervisor execution.
- **NVMe / PCIe Storage**: Multi-queue high-throughput flash controllers with zone namespaces (ZNS).
- **SATA / AHCI**: Legacy SATA controller driver support with NCQ request queues.
- **USB Mass Storage (BOT/UAS)**: USB Bulk-Only Transport and USB Attached SCSI protocol adapters.

### 3. Cache Class
SigmaOS implements a **hybrid buffer cache and page cache engine**:
- **Buffer Cache**: Low-level sector/block cache backing metadata and raw block devices (`src/fs/cache.rs`).
- **Page Cache**: Page-aligned (4KiB) virtual memory mapping engine for filesystem file data caching.
- LRU (Least Recently Used) replacement policy with dirty writeback threshold queues.

### 4. Filesystem Class
- **SigmaFS / Complete Filesystems**: Default native POSIX filesystems with journaling and Ext4 compatibility (`src/filesystem/core.rs`, `src/filesystem/ext4.rs`).
- **ZFS / ZFS ARC**: Advanced copy-on-write storage pool filesystem with hybrid adaptive replacement cache (`src/filesystem/zfs.rs`).
- **OverlayFS / TmpFS**: In-memory and tiered union filesystem engines (`src/filesystem/tmpfs.rs`).

## System Policies & Limits

### Mount Policy
- Mount operations must check capability tokens (`CAP_SYS_ADMIN` or equivalent storage capability).
- Mount points must be valid directories within the current mount namespace.
- Circular mount dependencies and duplicate target mounts are rejected with typed `MountError`.
- Rollback mechanism restores pristine mount table state if filesystem initialization fails.

### Flush Policy
- Periodic background dirty-page flush timer (`kswapd` / `flushtriggers`).
- Explicit sync calls (`fsync`, `sync`, `fdatasync`) block until physical device completes I/O flush commands.
- Power-loss emergency flush flushes all modified cache blocks before system halt.

### Reclaim & OOM Policy
- Storage cache allocation enforces strict memory watermarks (`CacheOomMetrics`).
- High memory pressure triggers aggressive LRU cache eviction of clean pages.
- Dirty pages are scheduled for rapid background writeback before page reclamation.
- Persistent OOM conditions throttle non-critical storage request queues rather than panicking kernel.

### Error & Hotplug Policy
- Unbounded retries are forbidden; I/O operations enforce maximum timeout and retry thresholds.
- Storage device removal (hot-unplug) invalidates device handles and returns `BlockError::DeviceDetached` to pending waiters.
- FSCK self-healing mechanisms repair minor metadata corruption automatically upon mount.
