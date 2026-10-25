# ADR-0012: Secondary Memory Architecture & Lifecycle Subsystem

## Status
Accepted

## Context
SigmaOS previously maintained fragmented storage models (`zfs`, `swap`, `paging`, `tmpfs`, `overlay`) without an overarching lifecycle contract or standardized secondary memory policy. To ensure storage safety, data durability, observable caching, and recoverable filesystems, SigmaOS must formalize secondary memory as a unified lifecycle subsystem.

## Decision
1. Establish a strict, 11-stage secondary memory lifecycle state machine:
   `probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`
2. Implement bounded request queues in `BoundedBlockDeviceManager` (`src/storage/block.rs`) to prevent unbounded memory growth during high I/O concurrency.
3. Enforce exact-once request completion and lost-completion timeout detection in the block device layer.
4. Integrate a hybrid LRU page cache (`BlockCacheManager` in `src/fs/cache.rs`) with OOM watermarks and double-eviction protection.
5. Provide bounded mount management (`MountManager` in `src/filesystem/mount.rs`) enforcing double-unmount protection and clean reference draining.

## Consequences
- **Positive**: Eliminates storage resource leaks, double-completion races, lost-writeback bugs, and unmount panics.
- **Positive**: Provides predictable memory bounds under storage heavy load and OOM conditions.
- **Negative**: Requires explicit error checks across all storage operations; raw unchecked buffer writes are strictly prohibited.
