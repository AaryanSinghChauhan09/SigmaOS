# ADR: Secondary Memory Subsystem Architecture

## Status
Accepted

## Context & Problem Statement
SigmaOS previously contained separate storage, block, cache, and filesystem models (`zfs`, `swap`, `paging`, `overlay`, `tmpfs`, `installer`). However, unit-tested standalone models alone do not establish a bootable, installable, or usable operating system. Secondary memory must be governed as an integrated, safe, observable, and recoverable lifecycle subsystem.

## Decision
SigmaOS adopts a unified secondary memory lifecycle contract across all block devices, caches, and filesystems:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Key Architectural Guarantees

1. **Bounded Resource Allocation**:
   - All kernel block request queues (`BoundedBlockDeviceManager`), block cache pages (`BlockCacheManager`), and filesystem volume tables (`FilesystemCoreManager`) operate under explicit allocation bounds.
   - Resource exhaustion returns typed errors (`BlockError::RequestExhaustion`, `CacheError::CacheFull`, `FsCoreError::CapacityExceeded`) rather than panicking or leaking memory.

2. **Exact-Once Request Completion**:
   - Every block I/O request transitions through `Pending -> Dispatched -> Completed/Failed`.
   - Double completion returns `BlockError::DoubleCompletion`.
   - Timed-out dispatched requests are flagged via `BlockError::LostCompletion`.

3. **LRU Caching & OOM Metrics**:
   - Block cache pages are managed using LRU eviction with automatic dirty page writeback.
   - Cache state and OOM pressure are monitored via `CacheOomMetrics` across `Normal`, `HighPressure`, and `OomCritical` watermarks.

4. **Failure-Safe Mount & Rollback**:
   - Mount/unmount transitions follow `Unmounted -> Mounting -> Mounted -> Unmounting`.
   - Mount failures execute clean rollbacks (`MountError::RollbackSuccessful`), preventing dangling mount points or corrupt scheduler state.

5. **Sequence Tracking & Lost-Update Prevention**:
   - File transactions enforce monotonic sequence numbering. Stale commits return `FsCoreError::LostUpdateDetected`.

## Consequences & Compliance

- **Correctness**: Eliminates race conditions in block request completion and mount management.
- **Observability**: Exposes structured metrics for cache hit rates, OOM pressure, volume capacity, and SCSI sense descriptors.
- **Safety**: Safe Rust preferred; bounds checking and error propagation enforced across all secondary memory modules.
