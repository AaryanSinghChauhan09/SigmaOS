# Architecture Decision Record: Secondary Memory Subsystem

## Status
Approved

## Context
SigmaOS is a safe, reliable, and observable operating system written in Rust. Secondary memory—comprising persistent disk storage, block devices, block caches, filesystems, and page-backed swap storage—was previously represented by individual models without a unified lifecycle subsystem contract.

Without a single authoritative storage lifecycle state machine and strict operational boundaries, storage components risk double-completion races, lost cache updates, unbounded request allocations, double-unmount bugs, and unhandled I/O failures.

## Decision
We establish secondary memory as an authoritative lifecycle subsystem in SigmaOS, governed by strict resource bounds, explicit error typing, and a unified 11-stage state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Key Architectural Choices

1. **Bounded Block Device Management (`BoundedBlockDeviceManager`)**:
   - Fixed request queues eliminate unbounded heap allocations in interrupt or I/O dispatch contexts.
   - Request allocation and reuse follow explicit state machine transitions (`Unallocated`, `Pending`, `Dispatched`, `Completed`, `Failed`, `Freed`).
   - Request exhaustion yields a typed `BlockError::RequestExhaustion` rather than panicking or blocking indefinitely.
   - Request completion enforces exact-once guarantees, returning `BlockError::DoubleCompletion` on duplicate completions.

2. **Race-Free Block Cache Management (`BlockCacheManager`)**:
   - Cache pages track exact lifecycle states (`Active`, `Flushing`, `Evicting`, `Evicted`).
   - Eviction uses LRU scheduling with mandatory dirty page writeback.
   - Attempting to evict an already evicted page returns `CacheError::DoubleEviction` or `CacheError::NotCached`.
   - Real-time watermark metrics (`WatermarkLevel::Normal`, `HighPressure`, `OomCritical`) expose cache pressure to kernel memory reclaimers.

3. **Safe Filesystem Core & Mount Management (`FilesystemCoreManager`, `MountManager`)**:
   - Filesystem volumes enforce explicit byte and inode capacity bounds (`VolumeCapacityBounds`), returning `FsCoreError::CapacityExceeded` when full.
   - Transaction sequence tracking (`FileTransactionRecord`) prevents lost updates (`FsCoreError::LostUpdateDetected`).
   - Mount states (`Unmounted`, `Mounting`, `Mounted`, `Unmounting`) prevent mount races and double-unmount operations (`MountError::DoubleUnmount`).

4. **100% Evidence-Gated Lifecycle Validation**:
   - Every secondary memory claim must be backed by unit tests, integration tests, or boot-to-shell smoke tests. Model-only assertions without evidence cannot mark storage features complete.

## Consequences

### Positive
- Zero storage leaks under stress tests.
- Observable OOM and cache memory pressure behavior.
- Complete protection against double-completion, lost-writeback, and double-unmount errors.
- Strict POSIX-compatible filesystem and mount semantics.

### Negative
- Require strict request queue and cache page bounds configuration during system setup.
