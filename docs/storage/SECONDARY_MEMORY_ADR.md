# ADR 0002: Secondary Memory Lifecycle & Subsystem Architecture

## Status

Accepted

## Context

SigmaOS requires a safe, observable, and recoverable secondary-memory subsystem. Previously, standalone storage models existed in the repository without an authoritative lifecycle policy connecting firmware probe through block I/O, page caching, filesystem mounting, and graceful unmount/recovery.

## Decision

We establish an authoritative 11-stage storage lifecycle state machine (`probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`) and mandate the following architecture across all secondary memory modules:

1. **Bounded Queue Allocations**: Block device queues (`BoundedBlockDeviceManager`) are bounded (64 requests max) with typed `QueueFull` errors to guarantee no unbounded kernel memory growth.
2. **Exact-Once Completion**: Block request tracking uses atomic lifecycle counters (`Completed`, `Pending`) and state transitions to prevent double-completion and lost-completion races.
3. **LRU Page Cache with Writeback**: Cache eviction (`BlockCacheManager`) enforces LRU ordering, dirty page writebacks before release, double-eviction protection, and observable OOM metrics (`CacheOomMetrics`).
4. **Failure-Safe Mount Management**: VFS mounts (`MountManager`) enforce capacity limits (64 mount points max), transactional rollback on failed mounts, and strict double-unmount prevention.
5. **Typed Error Propagation**: No storage failure may trigger an unhandled panic; errors must be typed (`BlockDeviceError`, `CacheError`, `FsCoreError`, `MountError`) and reported via structured diagnostics.

## Consequences

- All secondary memory components follow explicit lock ordering and bounded queue guarantees.
- Boot-to-shell storage pathways and tests enforce storage integrity and OOM observability.
