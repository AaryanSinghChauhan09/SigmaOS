# Architecture Decision Record: Secondary Memory Subsystem

## Status
Accepted

## Context
SigmaOS requires a unified secondary memory architecture that connects block devices, page caches, filesystems, and swap mechanisms into a safe, deterministic, and observable runtime pipeline. Previous model-only components lacked end-to-end lifecycle guarantees and failure recovery boundaries.

## Decision
1. Establish an authoritative 11-stage secondary memory lifecycle: `probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`.
2. Implement bounded request allocation in `src/storage/block.rs` with typed error return (`BlockError::RequestExhaustion`) to eliminate kernel memory exhaustion under heavy load.
3. Enforce exactly-once completion semantics and atomic state tracking (`Unallocated`, `Pending`, `Dispatched`, `Completed`, `Failed`, `Freed`) for block I/O requests.
4. Implement hybrid page/buffer cache management in `src/fs/cache.rs` with active dirty page writebacks, race-free eviction, and observable OOM metrics.
5. Implement bounded volume capacity tracking and double-unmount protection in `src/filesystem/core.rs` and `src/filesystem/mount.rs`.

## Consequences
- Bounded kernel memory footprint across storage layers under extreme pressure.
- Fully typed storage error propagation down to userspace syscall boundaries.
- Deterministic mount state machine preventing orphaned mounts or corrupt filesystem references.
