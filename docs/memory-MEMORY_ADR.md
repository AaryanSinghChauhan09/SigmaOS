# ADR 0002: Authoritative Memory Subsystem Lifecycle and Architecture

## Status

Accepted

## Context

SigmaOS requires a formal, predictable memory management architecture to transition from standalone unit-tested memory models to an integrated, production-grade operating system kernel. Unbounded allocations, race conditions in page mapping, unhandled frame exhaustion, and uncoordinated swapping lead to panics, security vulnerabilities, or kernel deadlocks under memory pressure.

## Decision

We establish one authoritative 11-stage memory management lifecycle:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

Key architectural principles:
1. **Hybrid Physical Allocation**: Use a Bitmap PMM for boot-time allocation, a Buddy Allocator for page frame allocation (orders 0..10), and a Slab Allocator for object caching.
2. **Strict User/Kernel Address Space Isolation**: User spaces occupy the lower canonical address half; kernel spaces occupy the higher canonical half. W^X (Write XOR Execute) is strictly enforced.
3. **Bounded Memory & OOM Governance**: Unbounded allocations are prohibited in kernel space. OOM is treated as a designed, observable system state evaluated via process badness scoring.
4. **Race-Free Concurrency & Lock Hierarchy**: Lock ordering must follow `PhysicalAllocatorLock -> PageTableLock -> SwapLock` to prevent deadlocks.
5. **Evidence-Based Status Gate**: No memory component may be marked complete without passing unit tests, integration tests, and boot-to-shell validation.

## Consequences

- All memory modifications must uphold the 11-stage lifecycle contract.
- Page table operations must issue TLB shootdowns and hold valid address space locks.
- Kernel code must handle frame allocation failures gracefully via typed `Result<T, MemoryError>` returns.
