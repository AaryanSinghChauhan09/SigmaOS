# Memory Management Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
The SigmaOS memory management subsystem provides physical page allocation, virtual address space management, demand paging, and kernel object caching. It must be correct above all else, then secure (ASLR, SMEP, guard pages), then fast (matching Linux's buddy+slab throughput).

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` support)
- **Systems-level:** Zig (comptime, explicit allocators, C interop when needed)
- **Scripting/tooling:** Nim (fast compile, GC-free mode available)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for OS components
- **unsafe Rust:** Only with `// SAFETY:` justification comment; prefer safe alternatives

## 3. Architecture Inspiration

### Linux Kernel
- **Buddy allocator:** Free list of 2^n order pages; O(log n) alloc/free; `mm/page_alloc.c`
- **Slab/SLUB allocator:** Object caches with per-CPU magazines; `mm/slub.c`
- **Virtual Memory Areas (VMAs):** `vm_area_struct` describing contiguous virtual regions; maple tree index
- **Demand paging:** Page fault handler; copy-on-write; swap; `mm/fault.c`
- **ASLR:** Randomized stack, heap, mmap base; `arch_mmap_rnd_bits`
- **Huge pages:** Transparent Huge Pages (THP) and explicit `mmap(MAP_HUGETLB)`
- **Memory zones:** ZONE_DMA, ZONE_DMA32, ZONE_NORMAL, ZONE_HIGHMEM

### FreeBSD
- **UMA (Universal Memory Allocator):** Slab-style with per-CPU caches; `sys/vm/uma.c`
- **SuperPages:** Automatic promotion to 2MB/1GB pages when contiguous physical memory available
- **`vm_map` / `vm_object`:** Two-level abstraction; map → objects → pages
- **NUMA-aware allocation:** `vm_phys_alloc_domain()` for NUMA-local allocation

### OpenBSD
- **`malloc(3)` guard pages:** Random guard pages between allocations; `malloc.conf` tuning
- **`mquery(2)`:** ASLR-friendly mmap hint API
- **W^X enforcement:** No page is simultaneously writable and executable

### NetBSD
- **Pmap abstraction:** Machine-independent pmap API (`pmap_enter`, `pmap_remove`) with arch-specific backends
- **Uvm (Unified Virtual Memory):** Clean separation of object, pager, and physical page layers

### Arch Linux
- Recommends `vm.swappiness=10` for desktop; `zram` for compressed swap

### NixOS
- Declarative swap configuration; `zram` and `zswap` via `boot.zramSwap`
- Reproducible memory profiling with `heaptrack` and `valgrind` in devShells

## 4. SigmaOS Source Files
- `src/kernel/mm/mod.rs` — MM subsystem root; `init_memory()` entry
- `src/kernel/mm/buddy.rs` — Buddy page allocator (physical frames)
- `src/kernel/mm/slab.rs` — SLUB-style object allocator (kernel objects)
- `src/kernel/mm/vmm.rs` — Virtual memory manager; VMA tracking
- `src/kernel/mm/page_table.rs` — x86_64 4-level page table manipulation
- `src/kernel/mm/fault.rs` — Page fault handler; demand paging; CoW
- `src/kernel/mm/aslr.rs` — Address space randomization
- `src/kernel/mm/huge_page.rs` — Transparent huge page promotion
- `src/kernel/mm/numa.rs` — NUMA topology and NUMA-local allocation
- `src/kernel/mm/swap.rs` — Swap subsystem; page eviction (LRU/CLOCK)

## 5. Current Implementation Status
| Feature | Status | Notes |
|---------|--------|-------|
| Buddy allocator | 🚧 Partial | Basic implementation; no zone separation |
| SLUB object allocator | 🔲 Missing | Using buddy for all allocations |
| Virtual memory areas | 🔲 Missing | No VMA tracking |
| Page table manipulation | 🚧 Partial | Basic mapping; no huge pages |
| Demand paging / fault handler | 🔲 Missing | All pages pre-mapped |
| Copy-on-write | 🔲 Missing | No CoW support |
| ASLR | 🔲 Missing | Fixed virtual addresses |
| Transparent huge pages | 🔲 Missing | 4KB pages only |
| NUMA-aware allocation | 🔲 Missing | No NUMA topology |
| Swap subsystem | 🔲 Missing | No swap |
| Memory zones (DMA/Normal) | 🔲 Missing | Single zone |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Memory zones: Separate physical memory into ZONE_DMA (0-16MB), ZONE_DMA32 (0-4GB), ZONE_NORMAL (rest). Required for DMA-safe allocation.
2. **[PRIORITY: HIGH]** SLUB allocator: Implement per-CPU object caches on top of buddy; use for all kernel struct allocations. Eliminates buddy overhead for small objects.
3. **[PRIORITY: HIGH]** VMA tracking: `VmaTree` (interval tree keyed on virtual address) per address space; `mmap`/`munmap`/`mprotect` operations.
4. **[PRIORITY: HIGH]** Page fault handler: On #PF, look up VMA; handle anonymous, file-backed, and CoW faults; grow stack on demand.
5. **[PRIORITY: HIGH]** ASLR: Randomize `mmap_base`, stack top, and kernel image base using boot-time entropy.
6. **[PRIORITY: MEDIUM]** Copy-on-write: Mark pages read-only on fork; copy on write fault; reference counting with `Arc<PhysFrame>`.
7. **[PRIORITY: MEDIUM]** Transparent huge pages: Promote 512 contiguous 4KB pages to 2MB; demote on partial unmap.
8. **[PRIORITY: MEDIUM]** Swap: LRU-based page eviction; `zswap` compressed page cache first; block device swap second.
9. **[PRIORITY: LOW]** NUMA-aware allocation: Prefer NUMA-local nodes; `migrate_pages` for rebalancing.

## 7. Implementation Guidelines for AI Agents

### Adding New Features
1. Physical allocators (`buddy.rs`) deal with `PhysAddr`; virtual allocators (`vmm.rs`) deal with `VirtAddr` — never mix.
2. All page table operations must acquire the address space lock (`AddressSpace::lock()`).
3. Use `PhantomData<*mut ()>` to make types `!Send`/`!Sync` where needed for per-CPU caches.
4. Never call the allocator from within an interrupt handler without a pre-allocated pool.
5. Always run `cargo check --target x86_64-unknown-none` before committing.
6. Follow `Stability > Security > Performance` priority.
7. Commit format: `feat(memory): <description>`

### Fixing Bugs
1. Memory bugs are often use-after-free or double-free — use Miri (`cargo miri test`) for unsafe code.
2. Add a reproducer as `src/kernel/mm/tests/regression_<N>.rs`.
3. Commit format: `fix(memory): <description>`

### Code Quality Checklist
- [ ] `cargo check` passes with 0 errors
- [ ] No `clippy::unused_variables` warnings
- [ ] No hard-coded credentials or secrets
- [ ] No `unsafe` without `// SAFETY:` justification comment
- [ ] Error handling uses `Result<T, E>` not `unwrap()` in production code
- [ ] All public functions have doc comments

## 8. Key Algorithms & Data Structures
- **Buddy allocator:** Array of free lists indexed by order (0..=MAX_ORDER); coalesce buddies on free; O(log n) alloc/free
- **SLUB:** Per-CPU `cpu_slab` with partial list; full slabs returned to node; O(1) fast path alloc/free
- **Interval tree (VMA):** Augmented red-black tree; node stores `max_end` for O(log n) overlap queries
- **Page frame database:** `PhysFrameInfo` array indexed by PFN; stores ref count, flags, VMA backlink
- **LRU page eviction:** Two-list (active/inactive) approximation of LRU; CLOCK-Pro for scan resistance
- **Zswap:** LZ4 compression of evicted pages into a compressed pool before hitting swap device

## 9. Security Considerations
- **ASLR:** At least 28 bits of entropy for mmap base on 64-bit; 8 bits for stack.
- **W^X:** Never map a page both writable and executable; enforce in `page_table.rs`.
- **Guard pages:** Unmapped pages below stack and above heap to catch overflows.
- **Heap metadata protection:** SLUB freelist pointers XOR-encoded to prevent freelist poisoning.
- **Memory tagging (ARM):** Support MTE (Memory Tagging Extension) on ARM64 for heap safety.
- **Kernel/user VA separation:** Never map user pages in kernel VA range; KPTI for x86_64.

## 10. Performance Targets
| Metric | Linux 6.x | SigmaOS Target |
|--------|-----------|----------------|
| `mmap(MAP_ANONYMOUS)` latency | ~1µs | < 2µs |
| Minor page fault latency | ~200ns | < 300ns |
| Buddy alloc (order-0) | ~50ns | < 100ns |
| SLUB alloc (cached) | ~15ns | < 30ns |
| `memcpy` 1MB (kernel) | ~200µs | < 250µs |

## 11. Testing Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_buddy_alloc_free_order0() {
        // Allocate and free a single page; verify no leak
    }

    #[test_case]
    fn test_buddy_coalesce() {
        // Allocate two adjacent order-0 pages; free both; verify order-1 free block
    }

    #[test_case]
    fn test_vma_find_gap() {
        // Map several regions; find unmapped gap >= N bytes
    }

    #[test_case]
    fn test_page_fault_anonymous() {
        // Trigger anonymous page fault; verify page allocated and mapped
    }

    #[test_case]
    fn test_cow_fork() {
        // Fork; write in child; verify parent page unchanged
    }
}
```

## 12. Dependencies Between Components
- **Depends on:** Kernel (boot memory map from bootloader), Timekeeping (for LRU aging timer)
- **Used by:** All other components — every allocation goes through MM; Scheduler (task stacks), FS (page cache), Net (socket buffers)

## 13. Maintenance Instructions for Future AI Agents
1. When Linux releases a new kernel, check `mm/` changelog — SLUB and folio changes are frequent.
2. Compare SigmaOS buddy allocator with `mm/page_alloc.c` for missed optimizations.
3. Prioritize stability: a MM bug crashes the whole system; test every change in QEMU.
4. Always update this file when making changes to `src/kernel/mm/`.
5. Wiki page for this component: `Memory-Management-Component-Guide` on GitHub Wiki.

## 14. References & Standards
- Linux source: `mm/`, `include/linux/mm.h`, `arch/x86/mm/`
- FreeBSD source: `sys/vm/`
- NetBSD source: `sys/uvm/`
- POSIX: `mmap(2)`, `munmap(2)`, `mprotect(2)`, `madvise(2)`
- Intel SDM Vol. 3, Chapter 4 (Paging)
- SigmaOS Wiki page: `Memory-Management-Component-Guide`
