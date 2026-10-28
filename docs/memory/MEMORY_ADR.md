# Memory Architecture Decision Record (ADR)

## ADR-001: SigmaOS Main Memory Subsystem Architecture

### Status
Accepted

### Context
SigmaOS requires a safe, deterministic, observable, and high-performance memory management subsystem. Previous designs relied on fragmented memory experiments. A unified, production-grade memory subsystem contract is required to support multi-tasking, page-table isolation, virtual memory areas, and swap management.

### Decision
1. **Authoritative Memory Lifecycle**: Implement and enforce an 11-stage linear memory lifecycle state machine:
   `boot -> physical allocator init -> paging init -> virtual memory init -> allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup`.
2. **Physical Allocator Architecture**: Combine a Buddy Frame Allocator for page-granularity physical frame allocation with a Slab Cache Allocator for kernel object instances.
3. **Paging & Address Space Isolation**: Enforce 4-level x86_64 paging with PCID/ASID context tags, strict canonical address validation, and explicit User/Supervisor space separation.
4. **Virtual Memory Area (VMA) Governance**: Enforce Write-XOR-Execute (W^X) permissions on all VMAs and track allocations via bounded data structures.
5. **Swap & OOM Handling**: Treat OOM as a designed error state managed by `kswapd` page reclaim and bounded swap slots.

### Consequences
- **Safety**: Double-free errors, lost updates, and frame exhaustion races are prevented by explicit state locks and typed errors.
- **Observability**: Allocation, mapping, protection, and swap events expose structured telemetry for kernel diagnostics.
- **Reliability**: Eliminates unbounded recursive allocation panics across all physical and virtual memory operations.
