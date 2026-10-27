# ADR-0002: Secondary Memory Architecture and Storage Lifecycle Contract

## Status
Accepted

## Context
SigmaOS requires a safe, observable, deterministic, and recoverable secondary-memory subsystem managing persistent storage, block I/O, filesystems, caches, and swap/paging storage. Previously, isolated models existed across `zfs`, `swap`, `paging`, and `tmpfs`, but lacked a unified lifecycle contract and race-prevention policies.

## Decision
We establish a single authoritative secondary-memory storage lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Core Architecture Invariants
1. **Bounded Allocation**: Every request queue, block cache, and volume metadata structure must enforce explicit capacity bounds. Request exhaustion returns a typed `BlockError::RequestExhaustion` error instead of panicking or growing silently.
2. **Exact-Once Completion**: Block requests transition through `Unallocated -> Pending -> Dispatched -> Completed/Failed -> Freed`. Re-completing an already completed or freed request returns `BlockError::DoubleCompletion`.
3. **Lost-Completion Prevention**: Pending and dispatched requests are tracked via internal sequence counters and timeout watchers to detect lost completions.
4. **Cache Eviction and Dirty Writeback**: Buffer/page cache eviction uses LRU ordering. Dirty pages must be written back to underlying block devices before eviction, preventing lost writes. Re-evicting an evicted page returns `CacheError::NotCached`.
5. **Mount Integrity & Capacity Enforcers**: Volume operations track byte and inode capacity bounds (`VolumeCapacityBounds`). Exceeding allocation bounds returns `FsCoreError::CapacityExceeded`. Double-unmounting returns `MountError::NotMounted` or `MountError::DoubleUnmount`.

## Consequences
- Secondary-memory subsystem state transitions are fully predictable, testable, and safe against memory corruption or unbounded growth.
- System observability is enhanced through explicit error types (`BlockError`, `CacheError`, `FsCoreError`, `MountError`) and watermark metrics (`CacheOomMetrics`).
