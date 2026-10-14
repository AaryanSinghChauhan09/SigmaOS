# SigmaOS Memory Subsystem Architectural Decision Record (ADR)

`docs/memory/MEMORY_ADR.md`

## Title
Authoritative 11-Stage Memory Management Lifecycle State Machine

## Context
Kernel memory subsystems often suffer from subtle bugs—such as use-after-free, double-free, lost TLB shootdowns, lost reclaim wakeups, and accidental OOM panics—when physical allocation, page table manipulation, virtual memory management, swapping, and reclaim operate as uncoordinated components. To ensure safety, predictability, and observability across SigmaOS, an explicit memory lifecycle state machine is required.

## Decision
SigmaOS establishes a single, authoritative 11-stage memory lifecycle state machine:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### Stage Definitions and Contracts

1. **Boot:** Discovers system physical memory map from BIOS/UEFI/E820/ACPI; identifies usable, reserved, ACPI, and NVS regions.
2. **Physical Allocator Init:** Initializes early bitmap PMM, buddy trees, and slab caches; excludes reserved regions from free lists.
3. **Paging Init:** Constructs root PML4 page table structures, maps physical memory identity region, and enables x86_64 CR3 paging.
4. **Virtual Memory Init:** Sets up kernel virtual address space (`0xFFFF_8000_0000_0000`+) and user VMA manager (`0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`).
5. **Allocation:** Requests physical frame(s) or slab memory; returns typed error (`PhysicalMemoryError::Exhausted`) on failure.
6. **Mapping:** Inserts virtual-to-physical address translation entries into 4-level page tables with TLB shootdown synchronization across cores.
7. **Protection:** Applies page-level access flags (User/Supervisor, Read/Write, No-Execute) enforcing strict W^X security policies.
8. **Swap:** Under memory pressure, serializes anonymous pages to swap slots with priority queueing (`SwapPriority`) and double-swap prevention.
9. **Reclaim:** Scans MGLRU pages via `kswapd`, flushes clean/dirty pages, and notifies sleeping allocators with lost-wakeup protection.
10. **Free:** Clears virtual mappings, updates page-table refcounts, and returns physical frames to buddy/slab allocators exactly once with `DoubleFree` checking.
11. **Cleanup:** Unmaps page table structures, destroys VMAs upon process exit, and reclaims kernel tracking metadata.

## Consequences
- **Safety:** Prevents double-free and use-after-free conditions by enforcing strict transition rules at each stage.
- **Observability:** Structured events and metrics are emitted at every lifecycle state transition.
- **Determinism:** Out-Of-Memory conditions are handled gracefully without panicking the kernel.
