# Secondary Memory Policy

## Storage Subsystem Objective
Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

## Storage Lifecycle State Machine

The secondary memory subsystem governs device operations according to one authoritative lifecycle contract:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Contracts

1. **Probe**: Storage device discovered, controller capabilities (NVMe, SATA, VirtIO, USB) reported.
2. **Identify**: Geometry, sector size, queue depth, and operational limits established.
3. **Block Layer**: I/O requests validated, queued, dispatched, and completed exactly once without lost updates or double completions.
4. **Cache**: Invalidation, LRU eviction, dirty page writebacks, and coherence invariants defined.
5. **Filesystem**: Safe mount/unmount lifecycle, volume capacity bounds, and layout integrity enforced.
6. **Mount**: Process/namespace mounting, capability authorization, and reference counting active.
7. **Read/Write**: Bounded data transfers with strict error propagation and memory redzone protection.
8. **Flush**: Data durability semantics explicit across block layer and cache flush queues.
9. **Unmount**: Open file reference tracking, flush validation, and resource release exactly once.
10. **Detach**: Hotplug detachment handling and dynamic teardown without kernel panics or leaks.
11. **Error/Recovery**: Failure injection safety, transaction logging, lost-update prevention, and OOM reclamation.

## Subsystem Classes

- **Block Layer Class**: Hybrid request/bio-based queuing model with bounded slots (`BoundedBlockDeviceManager`).
- **Device Class**: NVMe, SATA, VirtIO block, and USB storage abstraction (`BlockOrientedDevice`).
- **Cache Class**: Hybrid block cache and page cache with LRU eviction and dirty page writeback queues (`BlockCacheManager`).
- **Filesystem Class**: Ext4, ZFS, Overlay, Tmpfs, and native SigmaFS support (`FilesystemCoreManager`).
- **Mount Policy**: Atomic mount points with open reference tracking and failure rollback (`MountManager`).
- **Flush Policy**: Sync on unmount and periodic dirty page flush to prevent lost writes.
- **Reclaim Policy**: OOM-triggered watermark eviction (Normal -> HighPressure -> OomCritical).
- **Error Policy**: Typed block errors (`BlockError`), SCSI sense key mapping, and failure isolation.
- **Limits**: Hard memory limits on request queues, cache entries, and total volumes.
- **NUMA & Hotplug**: Explicit domain memory bindings and hotplug state safety checks.
