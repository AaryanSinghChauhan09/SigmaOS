# SigmaOS Memory Subsystem Policy (MEMORY_POLICY.md)

## Objective

Maintain SigmaOS memory management as a safe, observable, testable subsystem: allocate, map, protect, swap, limit, reclaim, and release memory without leaks, races, corruption, or undefined behavior.

## Authoritative Memory Lifecycle

SigmaOS defines one authoritative memory lifecycle across 11 explicit stages:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| Boot | Memory map discovered from firmware (e820/UEFI/DTB); reserved regions excluded before allocation starts. |
| Physical Allocator Init | Frame allocator initialized with physical memory bounds, bitmap/buddy tracking, and redzone/sanitizer checks. |
| Paging Init | Kernel root page structures (PML4/Page Directory) setup with W^X protection on physical kernel boundaries. |
| Virtual Memory Init | User/kernel address space boundaries established, non-overlapping kernel higher-half mappings fixed. |
| Allocation | Frames and virtual regions allocated with bounded capacity and typed failure errors (`OutOfMemory`, `InvalidRegion`). |
| Mapping | Virtual-to-physical address mappings committed to page tables with explicit locking and race-free TLB invalidated shootdowns. |
| Protection | Access control flags (User/Supervisor, Read/Write, Execute-Disable) applied to every active page translation. |
| Swap | Anonymous pages paged out to swap slots under memory pressure; metadata tracking is bounded and race-safe. |
| Reclaim | Proactive LRU/MGLRU page scanning and cache eviction triggered under memory pressure thresholds. |
| Free | Virtual ranges unmapped and physical frames returned exactly once without double-free or lost wakeups. |
| Cleanup | Process address spaces, page tables, and swap entries destroyed cleanly upon process termination. |

---

## Subsystem Policies

### 1. Physical Allocator Class
SigmaOS uses a **hybrid allocator architecture**:
- **Buddy Allocator** for coarse power-of-two physical frame blocks (4 KiB to 4 MiB).
- **Slab Allocator** for kernel object caches (`ProcessControlBlock`, `PageTable`, `VNode`).
- **Bitmap PMM** for boot-time allocation and early physical region accounting.

### 2. Frame Allocator Policy
- **Concurrency**: Spinlock or lock-free atomic bitsets enforce race-free physical frame allocation.
- **Double-Free Protection**: Bitmaps and order tracking prevent returning an unallocated frame twice.
- **Exhaustion Handling**: Returns `Result<PhysFrame, MemoryError::OutOfMemory>` instead of panicking.
- **Reserved Protection**: MMIO, ACPI, kernel text/data, and firmware regions are marked reserved during boot discovery and can never be allocated for general memory.

### 3. Paging Policy
- 4-level x86_64 / ARM64 page translation.
- **Invariants**: Lower half (0x0000_0000_0000_0000 .. 0x0000_7FFF_FFFF_FFFF) is strict userland; Upper half is kernel-only.
- **Lost-Update Prevention**: Page table modifications require write locks on the corresponding address space and explicit TLB shootdown broadcasts.

### 4. Virtual Memory Policy
- W^X (Write XOR Execute) enforced across all mappings.
- Kernel pages are marked Supervisor-only (`PTE_USER` bit clear).
- User memory allocations are bounded by process resource limits (`rlimit_as`, `rlimit_rss`).

### 5. Swap Policy
- Swap space is divided into fixed-size swap slots matching page frame size (4 KiB).
- Swap metadata uses bounded slot bitsets.
- Swap-in and Swap-out operations are atomic and race-free.

### 6. Reclaim Policy
- Memory pressure levels: `Normal` (>20% free), `Low` (5-20% free), `Critical` (<5% free).
- `Low` pressure triggers page cache eviction and zswap compression.
- `Critical` pressure triggers synchronous reclaim and swap-out of anonymous process pages.

### 7. OOM Policy
- Out-Of-Memory (OOM) is a designed, observable system state.
- Badness scoring selects non-critical user processes for termination based on RSS usage, process lifetime, and OOM score adjustment.
- System processes, init (PID 1), and kernel threads are protected from OOM termination.

### 8. Memory Limits and Bounds
- Kernel structure queues, page caches, and swap tables enforce strict upper allocation bounds.
- System calls requesting virtual memory (`mmap`, `brk`) return typed errors when hitting process or global system limits.

### 9. NUMA Behavior
- Systems with multiple NUMA nodes assign frame allocators per-node (ACPI SLIT/SRAT parsing).
- Default allocation policy uses local-node preference with cross-node fallback.

### 10. Memory Hotplug
- Memory hot-add and hot-remove operations update node zone descriptors and frame bitmaps atomically under global memory write lock.
