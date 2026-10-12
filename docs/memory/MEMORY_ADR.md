# ADR 0002: SigmaOS Memory Subsystem Architecture

## Status
Accepted

## Context
SigmaOS requires a reliable, secure, and observable main memory management subsystem. Previous model implementations demonstrated unit-test concepts but lacked standard subsystem policies and explicit lifecycle constraints.

## Decisions

1. **Unified Memory Lifecycle State Machine**
   All physical frame allocation, page mapping, virtual memory region creation, swap queueing, and frame freeing must strictly conform to the 11-stage memory lifecycle:
   `boot -> physical allocator init -> paging init -> virtual memory init -> allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup`.

2. **Typed Error Handling Over Panics**
   Physical frame exhaustion, page table capacity limits, non-canonical virtual addresses, and swap slot depletion must return typed Rust `Result<T, Error>` enums (`FrameAllocationError`, `PagingError`, `MmError`, `SwapError`) instead of triggering kernel panics.

3. **W^X Protection Policy Enforcement**
   Virtual Memory Areas (VMAs) enforce Write XOR Execute (W^X) security invariants. Any mapping attempt requesting simultaneously `write = true` and `execute = true` will be rejected at the VMM layer.

4. **Bounded Swap Metadata & Priority Queueing**
   Swap slot allocations are tracked using bounded bit vectors and Mutex-guarded descriptors. Swap operations are dispatched via priority queues (`SwapPriority::High`, `Normal`, `Low`) to guarantee real-time kernel pages take precedence over background user anonymous pages.

## Consequences
- Guarantees memory safety and race-free allocation in multi-core kernel context.
- Prevents memory leaks and double-free vulnerabilities.
- Enables clear observability via structured memory lifecycle metrics and status checks.
