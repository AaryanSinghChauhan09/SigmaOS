# Secondary Memory Policy Specification

## Objective

Maintain SigmaOS secondary memory as a safe, observable, recoverable storage subsystem: enumerate devices, manage block I/O, mount filesystems, cache data, page to disk, reclaim space, and recover from failures without corruption or undefined behavior.

## Authoritative 11-Stage Storage Lifecycle State Machine

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Lifecycle Stage Requirements

1. **Probe**: Device is discovered via PCI bus, virtio enumeration, or hotplug controller; capabilities are reported.
2. **Identify**: Geometry (logical/physical sector size, total sectors) and hardware limits are identified and frozen.
3. **Block Layer**: I/O requests are validated against bounds, queued, and completed exactly once.
4. **Cache**: Buffer cache and page cache invalidation and coherence semantics are strictly defined.
5. **Filesystem**: Mount and unmount lifecycles follow failure-safe transitions.
6. **Mount**: Filesystem volume is mounted onto the VFS tree in a thread-safe, race-free manner.
7. **Read/Write**: Data block transfers respect bounds, transaction logs, and permission engines.
8. **Flush**: Data durability guarantees (FUA / barrier / sync) are explicit and bounded.
9. **Unmount**: Dirty pages are flushed, open file descriptors are invalidated, and resources are released exactly once.
10. **Detach**: Device hardware is safely decoupled without pending requests or lost completions.
11. **Error / Recovery**: Hardware failures or unexpected disconnects trigger typed error recovery without corrupting filesystem structures.

## System Classes & Architectural Choices

- **Block Layer Class**: Hybrid request-based and bio-based architecture with bounded queue depths (e.g. 64 elements) to prevent kernel heap exhaustion.
- **Device Class**: Supports NVMe 2.0 (ZNS), VirtIO Block (`virtio-blk`), SATA/AHCI, and USB Mass Storage (BOT/UAS).
- **Cache Class**: Hybrid LRU page cache and block buffer cache governed by dirty page writeback watermarks and OOM reclaim.
- **Filesystem Class**: SigmaFS / Ext4 / ZFS / OverlayFS / TmpFS with POSIX-compatible file semantics and transactional journal logging.
- **Mount Policy**: Race-free mount table updates; prevents double-unmount, lost updates, or dangling mounts.
- **Flush Policy**: Synchronous and asynchronous writeback options with explicit barrier guarantees prior to unmount or system shutdown.
- **Reclaim Policy**: LRU-based buffer cache eviction triggered when high memory watermark is reached; dirty pages are committed before eviction.
- **Error Policy**: All storage I/O errors are strongly typed (`BlockDeviceError`, `CacheError`, `FsCoreError`, `MountError`), diagnosable, and propagated without kernel panics.
- **Resource Limits**: Hard caps on block request depth, buffer cache page count, mount entry count, and active inodes.
- **NUMA Behavior**: Local node allocation for block request rings and buffer cache allocations when NUMA topology is active.
- **Hotplug Handling**: Safe dynamic device attachment and detachment with graceful queue drain and VFS unmount safety checks.
