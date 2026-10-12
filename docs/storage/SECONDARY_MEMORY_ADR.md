# Secondary Memory Subsystem Architectural Decision Record (ADR)

* **Status**: Accepted
* **Date**: 2026-03-31
* **Context**: SigmaOS secondary memory (storage, block devices, filesystems, caches, swap/paging) requires a unified lifecycle contract and deterministic failure recovery model to transition from standalone models to a production OS architecture.

---

## Decision 1: Lifecycle State Machine

SigmaOS adopts an 11-stage authoritative secondary-memory state machine:
```
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Context & Justification
Without a unified lifecycle, subsystem components (block layer, page cache, VFS mounts) manage state independently, leading to unhandled errors during device hot-unplug, dangling mount references, or lost-writeback bugs on unmount.

---

## Decision 2: Bounded Block Device Queueing and Exact-Once Completion

SigmaOS implements `BoundedBlockDeviceManager` (`src/storage/block.rs`) with atomic request tracking and exact-once completion semantics.

### Key Rules
- Request queue size is bounded per device (`max_requests`).
- Request exhaustion returns `BlockError::RequestExhaustion` without panicking or blocking indefinitely.
- Request completion transitions state atomically (`Pending`/`Dispatched` -> `Completed`/`Failed`).
- Re-completing an already completed or failed request returns `BlockError::DoubleCompletion`.
- Timed-out requests are detected via `detect_lost_completions` and marked `BlockError::LostCompletion`.

---

## Decision 3: LRU Page Cache with Watermark Eviction and OOM Metrics

SigmaOS implements `BlockCacheManager` (`src/fs/cache.rs` and `src/filesystem/cache.rs`) to manage page caching for secondary memory.

### Key Rules
- Pages are evicted based on Least-Recently-Used (LRU) order.
- Dirty pages are flushed to storage prior to eviction; failed writebacks return `CacheError::WritebackFailed`.
- Double eviction attempts return `CacheError::DoubleEviction`.
- Exposes `CacheOomMetrics` tracking page watermark levels and OOM pressure.

---

## Decision 4: Safe Mount/Unmount Lifecycle and Volume Capacity Clamping

SigmaOS implements `FilesystemCoreManager` and `MountManager` (`src/filesystem/core.rs` and `src/filesystem/mount.rs`).

### Key Rules
- Filesystem capacity is strictly bounded (`max_capacity_bytes`). Allocations exceeding available space return `FsCoreError::CapacityExceeded`.
- Volume operations use transaction records (`FileTransactionRecord`) to prevent lost updates.
- Mount table insertion is atomic; unmounting an unmounted target returns `MountError::NotMounted`.

---

## Consequences

### Positive
- Zero panics or unbounded memory growth under storage overload or I/O failure.
- Observable secondary memory health via OOM metrics and diagnostic events.
- Testable lifecycle contracts using standalone unit test suites.

### Negative / Trade-offs
- Strict request and cache bounds require tuning for high-throughput workload profiles.
