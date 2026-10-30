# Architectural Decision Record: SigmaOS Memory Management Lifecycle

## Context
SigmaOS requires an authoritative lifecycle definition for memory management to guarantee thread safety, zero leak tolerance, deterministic OOM behavior, and isolation between kernel and userspace processes.

## Authoritative Memory Lifecycle
SigmaOS establishes the following mandatory 11-stage memory lifecycle state machine:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### Stage Contracts & Behaviors
1. **Boot**: Discovers E820/EFI memory map; reserved regions (kernel text, ACPI tables, MMIO) are excluded prior to dynamic allocation.
2. **Physical Allocator Init**: Initializes frame bitmap and buddy structures for available physical RAM.
3. **Paging Init**: Constructs PML4/page directories, setting up initial kernel identity mappings.
4. **Virtual Memory Init**: Establishes virtual memory region managers and kernel heap boundaries.
5. **Allocation**: Allocates physical frame(s) or virtual address ranges under strict process limits (`RLIMIT_AS`).
6. **Mapping**: Associates virtual addresses with physical frames in page tables with TLB invalidation.
7. **Protection**: Enforces hardware and software permissions (NX bit, Read/Write, User/Supervisor, W^X policy).
8. **Swap**: Handles background page eviction to secondary swap storage when physical memory pressure exceeds thresholds.
9. **Reclaim**: Scans LRU page lists to reclaim unmapped or cold pages under memory pressure.
10. **Free**: Deallocates virtual address ranges and returns physical frames to the allocator exactly once.
11. **Cleanup**: Reclaims page tables, kernel structures, and accounting entries during process termination.

## Locking Discipline & Concurrency Rules
- **Lock Ordering**: `PhysicalFrameAllocatorLock -> PageTableLock -> VirtualAddressSpaceLock -> SwapManagerLock`.
- **Interrupt Safety**: Memory allocation in interrupt context is strictly prohibited unless using pre-reserved slab pools.
- **Lost-Update Prevention**: Page table modifications use atomic compare-and-swap or lock-backed critical sections followed by TLB shootdowns.
