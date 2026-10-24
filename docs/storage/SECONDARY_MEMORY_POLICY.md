# SigmaOS Secondary Memory Subsystem Policy

## Overview

SigmaOS secondary memory manages persistent storage, block devices, filesystems, caches, and paging-backed storage as a safe, observable, and recoverable lifecycle subsystem.

## Lifecycle Contract

SigmaOS secondary memory enforces one authoritative lifecycle across all storage backends:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

1. **Probe**: Device discovered, hardware or virtual capabilities reported.
2. **Identify**: Sector size, block counts, geometry, and hardware queue depth limits established.
3. **Block Layer**: I/O requests validated, enqueued, executed, and completed exactly once without lost operations or request leakage.
4. **Cache**: Coherent buffer and page cache eviction with bounded metadata footprint and dirty page writeback guarantees.
5. **Filesystem**: Failure-safe mount and unmount semantics with bounded capacity and transaction journaling.
6. **Flush**: Durability guarantees made explicit across volatile caches and block layers.
7. **Unmount**: Storage resources, open references, and lock state released exactly once.
8. **Recovery**: Sudden power Loss or I/O failure recovery without filesystem corruption or state leaks.

## Storage Classes and Architectural Specifications

- **Block Layer Class**: Hybrid request-based and bio-based architecture with bounded submission queues and lock-free atomic request state transitions.
- **Device Class**: First-class support for NVMe 2.0, SATA AHCI, virtio-blk, USB mass storage, and loopback memory disks.
- **Cache Class**: Hybrid unified page and buffer cache with LRU eviction watermarks, active dirty page tracking, and OOM pressure signals.
- **Filesystem Class**: Support for SigmaFS CoW, ext4/JBD2, ZFS ARC, overlayfs, tmpfs, and synthetic procfs/sysfs mounts.
- **Mount Policy**: Single authoritative mount table per namespace with double-mount and double-unmount prevention.
- **Flush Policy**: Explicit `sync` and `fsync` flush boundaries with barrier enforcement down to physical block devices.
- **Reclaim Policy**: Observable OOM eviction under memory pressure, dirty page writeback throttling, and memory pool limits.
- **Error Policy**: Typed error taxonomy (`BlockError`, `CacheError`, `FsCoreError`, `MountError`) with zero unhandled exceptions or panics.
- **NUMA & Hotplug Behavior**: NUMA node memory affinity for block request pools and clean dynamic device hot-unplug handling.
