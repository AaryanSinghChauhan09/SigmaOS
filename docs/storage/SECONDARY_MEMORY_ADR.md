# Secondary Memory Architectural Decision Record (ADR)

## Status
Accepted

## Context
SigmaOS requires a safe, observable, and recoverable secondary memory subsystem. Historically, storage components existed as isolated models without a unified lifecycle contract or strict error handling. Without an explicit lifecycle contract, race conditions, double-completions, lost writebacks, and illegal unmount states can corrupt persistent data.

## Decision
We establish a unified secondary memory subsystem with the following architectural commitments:

1. **Authoritative 11-Stage Storage Lifecycle**:
   `probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`

2. **Bounded Request Queues and Exact-Once Completion**:
   All block request allocations are bounded by `max_requests`. Request completion transitions state atomically (`Pending -> Dispatched -> Completed/Failed`). Attempting to complete a request multiple times yields `BlockError::DoubleCompletion`. Lost requests are detected via submission timestamps and timeout scans.

3. **Page Cache Watermarks and Lost-Writeback Prevention**:
   The buffer/page cache (`BlockCacheManager`) enforces eviction limits (`max_pages`). Dirty pages are synchronized prior to eviction to guarantee data durability. Watermarks (`Normal`, `HighPressure`, `OomCritical`) provide full observability into kernel storage memory usage.

4. **Mount Lifecycle with Reference Tracking**:
   Filesystem mounts (`MountManager`) track open references (`open_ref_count`). Active mounts cannot be unmounted (`MountError::ResourceBusy`). Unmounting an unmounted volume returns `MountError::DoubleUnmount`. Failed initialization triggers automatic transaction rollback.

5. **Volume Capacity and Lost-Update Protection**:
   `FilesystemCoreManager` limits byte and inode capacity per volume, returning `FsCoreError::CapacityExceeded` when full. Transactions verify sequence numbers upon commit to prevent lost updates (`FsCoreError::LostUpdateDetected`).

## Consequences
- **Positive**: Eliminates storage leaks, lost updates, double completions, and unmount races. Provides diagnostic visibility into storage OOM and watermark states.
- **Negative**: Requires explicit error handling across storage call sites and strict compliance with memory bounds.
