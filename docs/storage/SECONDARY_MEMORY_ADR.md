# ADR 0002: Safe, Observable, and Recoverable Secondary Memory Subsystem

* **Status**: Accepted
* **Date**: 2026-10-13
* **Context**: SigmaOS managed storage and secondary memory through fragmented file systems and storage driver models. To ensure production readiness, reliability, and security across real hardware and virtualized runtimes, secondary memory must be governed as an authoritative lifecycle subsystem.

---

## Context and Problem Statement

Kernel secondary memory management involves interacting with physical block devices (NVMe, VirtIO, SATA, USB Mass Storage), caching block structures in physical memory, mounting file systems, executing durability flushes, and managing error recovery. Previous implementation models lacked a unified storage lifecycle contract, leaving potential edge cases such as lost request completions, double cache evictions, lost updates during file writes, or unsafe double unmounts.

SigmaOS requires an explicit, observable, and deterministic architectural contract for secondary memory that guarantees zero data corruption, bounded resource usage, and typed error handling.

---

## Decision Drivers

1. **Safety & Zero Corruption**: Prevent request reuse races, lost updates, double evictions, and double unmounts.
2. **Deterministic Lifecycle**: Enforce an 11-stage storage lifecycle from initial hardware probe to detach and recovery.
3. **Bounded Kernel Resource Allocations**: Block request queues, page caches, and mount entries must operate within hard capacity limits with observable OOM metrics.
4. **POSIX & Standard OS Conformance**: Provide standard filesystem semantics while remaining compatible with modular Linux/BSD-inspired storage layers.
5. **Observability**: Maintain structured telemetry for block queues, cache watermarks, mount states, and recovery events.

---

## Technical Architecture & Design

### 1. Storage Subsystem State Machine

All secondary memory components must enforce the following lifecycle flow:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### 2. Subsystem Components

#### Block Device Manager (`src/storage/block.rs`, `src/storage/virtio.rs`)
- Maintains `BoundedBlockDeviceManager` with ring queue size limits (e.g. 256 requests).
- Enforces atomic request state tracking: `Free` -> `Enqueued` -> `InFlight` -> `Completed`.
- Prevents lost completions via completion wait queues and timeout detectors.
- Prevents double completions by validating request transition boundaries atomically.
- Returns `BlockError::QueueFull` when capacity limits are hit.

#### Block Cache Manager (`src/fs/cache.rs`)
- Implements `BlockCacheManager` backed by LRU page queues.
- Limits maximum cache memory pages to prevent unbounded physical memory consumption.
- Tracks page state (`Clean`, `Dirty`, `Evicting`) to ensure safe writeback prior to eviction.
- Reports `CacheOomMetrics` (total pages, dirty pages, evicted count, cache hit ratio) for subsystem observability.

#### Filesystem Core & Mount Manager (`src/filesystem/core.rs`, `src/filesystem/mount.rs`)
- Manages volume capacity boundaries, inode metadata, and transactional records (`FileTransactionRecord`).
- Implements atomic mount table updates with rollback guarantees upon mounting failure.
- Prevents double unmounting through atomic state checks (`Mounted` -> `Unmounting` -> `Unmounted`).
- Guarantees lost-update prevention by synchronizing dirty buffers during write transactions and unmount sequences.

---

## Consequences

### Positive Consequences
- Guarantees memory and concurrency safety across all storage paths.
- Provides strict error typing (`BlockError`, `CacheError`, `FsCoreError`, `MountError`) for diagnostic visibility.
- Prevents memory exhaustion caused by unbounded I/O request queues or unconstrained page caches.
- Enables predictable recovery and self-healing under unexpected system halt or storage device disconnect.

### Negative Consequences
- Slightly increased memory overhead for tracking block request states and transaction metadata.
- I/O callers must explicitly handle queue full and cache pressure errors when limits are reached.

---

## Compliance & Enforcement

Compliance with ADR 0002 is enforced through:
1. `STORAGE_RULES.toml` component rules and verification checks.
2. Unit and integration test suites:
   - `tests/storage/block.rs`
   - `tests/filesystem/cache.rs`
   - `tests/filesystem/core.rs`
   - Root integration tests (`tests/storage_block_tests.rs`, `tests/filesystem_cache_tests.rs`, `tests/filesystem_core_tests.rs`).
3. Continuous integration boot-to-shell smoke tests.
