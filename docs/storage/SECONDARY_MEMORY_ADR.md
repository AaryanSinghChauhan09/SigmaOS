# SigmaOS Secondary Memory Architecture Decision Record (ADR)

## Status
Accepted

## Context
SigmaOS requires a safe, high-performance, and recoverable secondary memory subsystem capable of managing block devices, caches, filesystems, and swap memory across bare-metal and virtualized environments. Unit-tested models alone are insufficient to guarantee safe runtime execution without formal architectural invariants.

## Decision

### 1. Unified 11-Stage Storage Lifecycle State Machine
We establish one authoritative lifecycle state machine for all storage components:
`probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`.

Every device driver, block layer, cache layer, and filesystem driver must implement explicit handlers for each lifecycle state.

### 2. Hybrid Request-Based & Bio-Based Block Layer
SigmaOS adopts a hybrid block layer design (`BoundedBlockDeviceManager` in `src/storage/block.rs`).
- **Requests**: Bounded request structs with strict state tracking (`Unallocated` -> `Pending` -> `Dispatched` -> `Completed`/`Failed` -> `Freed`).
- **Safety**: Double-completion and lost-completion prevention via atomic request status state machines.
- **Exhaustion**: Request exhaustion returns typed `BlockError::RequestExhaustion` rather than panicking or blocking indefinitely.

### 3. Integrated Page & Buffer Cache Manager
SigmaOS combines page and block caching into a unified `BlockCacheManager` (`src/fs/cache.rs`).
- **Eviction**: High-watermark and low-watermark LRU eviction.
- **Writeback**: Dirty pages are atomically marked for writeback and flushed before page eviction.
- **OOM**: Exposes observable cache metrics (`CacheOomMetrics`) to kernel OOM governors.

### 4. Failure-Safe Mount & Core Filesystem Operations
`FilesystemCoreManager` (`src/filesystem/core.rs`) and `MountManager` (`src/filesystem/mount.rs`) enforce transactional updates.
- **Mount Rollback**: Partial mount failure safely cleans up allocated resources and reverts state.
- **Double-Unmount Protection**: Unmount operations check active reference counts and state markers atomically.
- **Read-Only Degradation**: Storage I/O errors degrade filesystems to read-only status (`FsHealthStatus::ReadOnly`) rather than corrupting structure.

## Consequences
- All storage code must define explicit, bounded memory structures with typed error propagation.
- Unbounded queue growth and panic-on-error in storage paths are disallowed.
- Subsystem modifications require test evidence verifying exact-once completion and lifecycle state preservation.
