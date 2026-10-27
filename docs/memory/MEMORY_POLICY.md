# SigmaOS Main Memory Management Policy

## 1. Overview & Core Directive

SigmaOS manages main memory as an authoritative lifecycle subsystem. Memory management encompasses physical allocation, paging, virtual address space mapping, swapping, reclaim, OOM handling, and observability. All memory operations must proceed without leaks, races, corruption, or undefined behavior.

## 2. Authoritative Lifecycle Contract

The SigmaOS main memory subsystem strictly enforces the following 11-stage lifecycle:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### Stage Requirements & Behaviors

| Stage | Required Behavior |
|---|---|
| Boot | Discovers physical memory map from firmware (ACPI/e820/EFI) and excludes reserved hardware/firmware regions. |
| Physical Allocation Init | Initializes bounded buddy/bitmap frame allocators; tracks frame metadata race-free. |
| Paging Init | Constructs initial 4-level x86_64 page tables with kernel address space identity/higher-half mappings. |
| Virtual Memory Init | Sets up Virtual Memory Areas (VMAs) and user/kernel isolated address space boundaries. |
| Allocation | Allocates physical frame or virtual region with bounded memory tracking and typed error handling on exhaustion. |
| Mapping | Safe page-table insertion with TLB invalidation and race-free page attribute modification. |
| Protection | Enforces W^X (Write XOR Execute) and User/Supervisor permission bits on every page table entry. |
| Swap | Swaps cold/dirty pages to swap devices safely without lost writes or race conditions under memory pressure. |
| Reclaim | LRU/MGLRU page scanning and proactive slab/page-cache eviction before OOM invocation. |
| Free | Returns physical frames and virtual mappings back to pools exactly once without double-free errors. |
| Cleanup | Reaps address space contexts, unwinds page table hierarchies, and clears residual tracking structures. |

## 3. Subsystem Policies

### Physical Allocator Class
- **Buddy Allocator**: Primary physical frame allocator (`src/memory/buddy_allocator.rs`, `src/memory/sigma_buddy.rs`) for power-of-two page order allocations.
- **Slab Allocator**: Kernel object cache allocator (`src/memory/slab_allocator.rs`) operating on buddy-allocated physical pages for fixed-size object caching.

### Frame Allocator Policy
- Frame allocation must be race-free and thread-safe.
- Reserved regions (e.g., lower 1MB, ACPI tables, kernel text/data) are protected from allocation.
- Frame exhaustion returns `PhysicalMemoryError::Exhausted` rather than panicking.
- Double-free protection via explicit frame state bitmasks and reference counts.

### Paging Policy
- 4-level page table architecture (`PML4 -> PDPT -> PD -> PT`).
- Explicit CR3 register manipulation with ASID/PCID support for fast context switches.
- Strict canonical address enforcement (`0x0000_0000_0000_0000` to `0x000F_FFFF_FFFF_FFFF` for user, `0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF` for kernel).

### Virtual Memory Policy
- Address spaces isolated between user and kernel processes.
- Memory protection enforced via page flags (`PRESENT`, `WRITABLE`, `USER_ACCESSIBLE`, `NO_EXECUTE`).
- W^X policy: Memory regions cannot be simultaneously writable and executable.

### Swap & Reclaim Policy
- Bounded swap slot bitmap and priority queues (`src/swap/core.rs`, `src/swap/enqueue.rs`).
- Background reclaim daemon (`kswapd`) triggered when free memory drops below low watermark.
- Page aging via Multi-Generational LRU (MGLRU).

### OOM Behavior
- OOM killer (`src/memory/oom_killer.rs`) selects task based on OOM score (`memory_usage + badness_adj`).
- Designed state: Returns typed allocation errors or safely terminates chosen process without kernel panic.

### NUMA & Hotplug
- NUMA awareness via ACPI SLIT/SRAT tables (`src/memory/acpi_slit.rs`).
- Memory hotplug handling safely adjusts node frame boundaries dynamically.
