# Architectural Decision Record: Secondary Memory Lifecycle Subsystem

## Status
Accepted

## Context
SigmaOS previously featured isolated models and prototype drivers across ZFS, swap, paging, virtio, and userland filesystems. However, without an authoritative lifecycle state machine, bounded queue allocations, and typed failure recovery contracts, the storage layer risked memory leaks, request exhaustion panics, lost completions, lost writebacks, and filesystem corruption upon sudden device detachment or I/O failure.

A secondary memory subsystem must govern persistent block devices, storage controllers, caches, mount trees, and filesystems under strict bounds and reproducible recovery guarantees.

## Decision

1. **One Authoritative Lifecycle State Machine**:
   Adopt an explicit 11-stage storage lifecycle:
   `probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`.
   Every transition has strict preconditions, atomic operations, and deterministic post-conditions.

2. **Bounded Resource Allocation**:
   All request queues, cache metadata pools, and volume capacity quotas must be strictly bounded at allocation time. Unbounded growth is forbidden. When resource capacity is reached, functions must return typed errors (`RequestExhaustion`, `CapacityExceeded`, `CacheFull`) rather than allocating unboundedly or panicking.

3. **Exactly-Once Request Completion**:
   Block requests transition monotonically through `Unallocated -> Pending -> InFlight -> Completed -> Reaped`. Double-completion and lost-completion are detected and rejected. Request IDs cannot be reused until reaped.

4. **Failure-Safe Mount and Lost-Update Prevention**:
   Filesystem mounts must implement rollback on failure. Volume modifications and file transactions track monotonically increasing sequence numbers and transaction records (`FileTransactionRecord`) to detect and abort conflicting concurrent updates (`LostUpdateDetected`).

5. **Observable OOM and Watermark Semantics**:
   The cache manager models memory pressure through explicit watermarks (`Normal`, `HighPressure`, `OomCritical`), exposing eviction metrics and writeback statistics for system telemetry and governor daemons.

## Consequences

- **Positive**: Eliminates arbitrary memory leaks in storage request paths. Prevents data corruption from race conditions during mount/unmount and cache eviction.
- **Positive**: Provides clear observability into storage queue depth, cache hit rates, and error conditions.
- **Negative**: Adds explicit bounds checking and status validation overhead across storage subsystem calls.
- **Neutral**: Requires driver implementations and test suites to adhere strictly to the lifecycle state machine and PR evidence standards.
