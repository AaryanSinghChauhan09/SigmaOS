# Architecture Decision Record: Secondary Memory Subsystem

## ADR-0004: Safe, Observable, and Recoverable Secondary Memory Subsystem

- **Status**: Accepted
- **Date**: 2026-10-26
- **Context**: SigmaOS secondary memory comprises persistent storage, block devices, filesystems, block caches, swap, and paging-backed storage. Previously, unit-tested models operated as isolated prototypes without a unified, safe, and recoverable lifecycle contract.

## Decision

SigmaOS adopts a unified architecture for secondary memory based on one authoritative 11-stage lifecycle contract:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Key Architectural Standards

1. **Strict Lifecycle State Machine**:
   - Every block device, cache page, and filesystem mount transitions through explicit states.
   - Illegal state transitions (e.g. double completion, double eviction, double unmount) are prevented by atomic state invariants and return strongly-typed error codes.

2. **Bounded Resource Allocation & Zero Leaks**:
   - Device queues, request slots, cache pages, and transaction buffers are statically or dynamically bounded.
   - Out-of-memory (OOM) conditions trigger structured page reclamation rather than kernel panics.

3. **Concurrency and Locking Discipline**:
   - Lock hierarchy for storage operations is strictly strictly ordered:
     `VFS Mount Lock -> Filesystem Transaction Lock -> Cache Page Lock -> Block Device Lock`
   - All locks are non-nesting where possible or follow strict ascending lock ordering to prevent deadlocks.

4. **Failure Recovery and Durability**:
   - Transaction sequence numbers detect lost updates in filesystem metadata.
   - Mount failure during setup executes an automatic, safe rollback to `Unmounted`.
   - Device hotplug or abrupt detachment cleanly invalidates pending requests with `BlockError::DeviceDetached`.

## Consequences

- **Positive**:
  - Eliminates corrupt filesystem states and memory leaks during storage failures.
  - Ensures 100% observable storage metrics and typed errors.
  - Enforces strict compliance with `STORAGE_RULES.toml`.

- **Negative**:
  - Requires explicit state checks and atomic transitions across all storage code paths.
