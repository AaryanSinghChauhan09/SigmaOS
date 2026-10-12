# ADR 003: Subsystem Memory Management Lifecycle and Safety Contracts

## Status
Accepted

## Context
SigmaOS manages main memory as a fundamental operating system subsystem. Previously, memory modules existed as disjoint models without unified state machine definitions, bounded guarantees, or strict lifecycle contracts. To ensure kernel safety, deterministic execution, race freedom, and no accidental panics during memory exhaustion, SigmaOS requires an authoritative Architecture Decision Record (ADR) governing memory management.

## Decision

### 1. Authoritative 11-Stage Lifecycle State Machine
All memory operations follow the linear state machine:
`boot -> physical allocator init -> paging init -> virtual memory init -> allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup`

1. **Boot**: Memory discovery via e820; reserved physical regions excluded prior to allocator initialization.
2. **Physical Allocator Init**: Hybrid Buddy + Slab structures populated with validated physical frame metadata.
3. **Paging Init**: 4-level x86_64 page tables allocated and root CR3 loaded.
4. **Virtual Memory Init**: User/kernel VMAs constructed with strict boundary isolation (`0x00000000_00010000` to `0xFFFF8000_0000_0000`).
5. **Allocation**: Atomic frame and slab retrieval returning typed errors (`FrameAllocationError::OutOfMemory`).
6. **Mapping**: Page table entries updated with atomic TLB invalidation and lost-update prevention locks.
7. **Protection**: Permission checks (Read, Write, Execute, User) and W^X (Write XOR Execute) policy enforcement.
8. **Swap**: Slot allocation in `SwapManager` with priority queueing (`SwapQueueManager`).
9. **Reclaim**: Background page compaction (`kswapd` / LRU) with lost-wakeup notification (`reclaim_wakeup_notified`).
10. **Free**: Deallocation releasing frame descriptors and slot indices exactly once (`DoubleFree` detection).
11. **Cleanup**: Address-space destruction and frame recycling upon thread or process exit.

### 2. Synchronization and Lock Ordering
To prevent deadlocks and race conditions:
1. **Virtual Memory Lock (`VirtualMemoryManager`)** -> **Paging Lock (`PagingManager`)** -> **Physical Allocator Lock (`PhysicalMemoryManager`)** -> **Swap Lock (`SwapManager`)**.
2. Nested lock acquisition in reverse order is strictly prohibited.
3. Frame state transitions use seq_cst atomic ordering (`ref_count`, `state`).

### 3. OOM as a Designed State
- Out-Of-Memory (OOM) condition must NEVER cause an unhandled kernel panic.
- Frame exhaustion returns `FrameAllocationError::OutOfMemory`.
- Swap exhaustion sets `oom_triggered` atomic flag and invokes heuristic victim selection.

## Consequences
- Guarantees race-free frame allocation, double-free protection, and no memory corruption.
- Establishes clear test contracts for memory stress and limit enforcement.
