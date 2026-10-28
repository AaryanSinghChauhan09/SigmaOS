# SigmaOS Secondary Memory Subsystem Policy

## 1. Subsystem Objective & Scope

SigmaOS secondary memory manages persistent storage, block devices, filesystems, block caches, and swap/paging storage as an integrated, safe, observable, and recoverable lifecycle subsystem.

The single authoritative storage lifecycle state machine is defined as:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

---

## 2. Architectural Classifications

### 2.1 Block Layer Class
SigmaOS employs a **hybrid request-based/bio-based block layer**:
- High-level block requests (read, write, flush, discard) are constructed into bounded request objects (`BlockRequest`).
- Low-level drivers (NVMe, VirtIO, AHCI) translate requests into scatter-gather DMA descriptions (bio buffers).
- All request allocation queues are statically or dynamically bounded to avoid kernel memory exhaustion.

### 2.2 Device Class
Supported storage device classes include:
- **NVMe**: PCIe Express High-Performance Non-Volatile Memory submission/completion queue pairs.
- **VirtIO Block (`virtio-blk`)**: Paravirtualized storage queue controller for virtualized environments.
- **SATA / AHCI**: Advanced Host Controller Interface for legacy and modern Serial ATA drives.
- **USB Mass Storage / UASP**: USB Attached SCSI Protocol storage controllers.
- **RAM Disk / Loop Devices**: Memory-backed block devices for temporary and isolated staging.

### 2.3 Cache Class
SigmaOS implements a **hybrid page cache and block buffer cache**:
- **Page Cache**: Caches 4KiB page-sized units mapped directly to virtual memory address spaces.
- **Buffer Cache**: Caches metadata blocks (superblocks, inodes, allocation bitmaps) with LRU eviction and dirty writeback queues.

### 2.4 Filesystem Class
SigmaOS supports modular filesystems:
- **SigmaFS / Ext4**: POSIX-compliant journaled block filesystem with extent mapping.
- **ZFS**: Advanced copy-on-write storage pool with RAID-Z, ARC (Adaptive Replacement Cache), and checksum integrity.
- **Tmpfs**: Memory-backed volatile RAM filesystem with hard OOM quota enforcement.
- **OverlayFS**: Lower/upper branch filesystem unioning for immutable base images with mutable overlays.

---

## 3. Storage Policies

### 3.1 Mount Policy
- Mount point targets must be absolute paths.
- Re-mounting an active mount point without unmounting return typed `AlreadyMounted` errors.
- Mount operations validate target filesystem superblocks before updating the global mount table.
- Cyclic or recursive bind mounts are rejected during path resolution.

### 3.2 Flush Policy
- Synchronous write requests (`O_SYNC`, `fsync()`) block until the storage controller acknowledges persistent flash/media commit via hardware flush operations (`VIRTIO_BLK_T_FLUSH` or NVMe Flush command).
- Periodic background dirty page writebacks flush dirty cache pages according to age and watermark pressure.

### 3.3 Reclaim Policy
- When memory pressure approaches `HighPressure` or `OomCritical` watermarks, cache eviction reclaims clean cached pages first.
- Dirty pages are enqueued for urgent asynchronous writeback before reclamation.
- Non-reclaimable pages under active I/O block eviction until I/O completion.

### 3.4 Error Policy
- All storage I/O errors are typed (`BlockError`, `CacheError`, `FsCoreError`, `MountError`).
- Hardware submission/completion timeouts trigger device reset or degraded status logging without kernel panics.
- Unrecoverable read/write failures mark affected volume sectors or health status as `Degraded` or `ReadOnly`.

---

## 4. Resource Limits & NUMA Behavior

- Kernel request queues enforce strict max depth limits (`MAX_REQUESTS`).
- Buffer and page cache allocations enforce global and per-volume maximum memory limits.
- NUMA awareness binds storage completion ring buffers and DMA page allocations to the local NUMA node of the target PCI storage device controller.

---

## 5. Storage Hotplug & Recovery Behavior

- Hot-unplug events immediately mark device handles as `DeviceDetached`.
- In-flight requests on detached devices fail with typed `DeviceDetached` errors and are cleaned up without reference leaks.
- Mount points associated with detached devices transition to read-only or error-handling states, allowing clean unmount operations.
