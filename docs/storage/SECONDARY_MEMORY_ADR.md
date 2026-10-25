# Architecture Decision Record (ADR): Secondary Memory Subsystem Governance

- **Status**: Accepted
- **Date**: 2026-03-31
- **Subsystem**: Secondary Memory / Storage Architecture

---

## Context

SigmaOS requires a safe, deterministic, and observable storage architecture capable of handling persistent devices, block device drivers, filesystem mounts, page caching, and error recovery. Storage subsystem bugs, lost request completions, double unmounts, and unhandled memory exhaustion lead to filesystem corruption or kernel panics.

## Decision

SigmaOS adopts a unified secondary-memory storage contract enforcing an authoritative 11-stage lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Key Architectural Standards

1. **Bounded Request Queues**:
   - `BoundedBlockDeviceManager` enforces strict capacity upper bounds (`max_requests`). Submissions exceeding queue capacity fail cleanly with `BlockError::RequestExhaustion`.

2. **Exactly-Once Completion**:
   - Every block request must transition from `Pending` -> `Dispatched` -> `Completed` or `Failed`. Re-completing a request returns `BlockError::DoubleCompletion`. Reaping a non-dispatched or active request returns `BlockError::DeviceBusy`.

3. **Page Cache Watermarking & LRU Eviction**:
   - `BlockCacheManager` tracks cache memory utilization across four watermark tiers (`Low`, `Medium`, `High`, `OomCritical`). Eviction follows strict LRU ordering with dirty page writeback before eviction. Double eviction returns `CacheError::NotCached`.

4. **Failure-Safe Mount Lifecycle**:
   - `MountManager` manages mount points as reference-counted structures (`open_ref_count`). Active references block unmounting with `MountError::ResourceBusy`. Failed mount initializations trigger clean rollbacks via `mount_with_rollback`.

5. **Lost Update Prevention**:
   - Filesystem updates use sequence-numbered transactions (`begin_transaction`, `commit_transaction`). Out-of-order or stale transaction commits fail with `FsCoreError::LostUpdateDetected`.

---

## Consequences

- **Safety**: Eliminates double-free, double-completion, and lost-update race conditions in storage drivers and filesystems.
- **Observability**: Exposes structured metrics for watermark levels, request queue lengths, and I/O error counts.
- **Resource Constraints**: Guarantees zero unbounded kernel memory allocations during storage stress.
