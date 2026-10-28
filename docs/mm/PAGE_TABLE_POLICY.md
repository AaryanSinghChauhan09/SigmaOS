# Page Table & Paging Policy

## 1. Page Table Management Standards

SigmaOS paging and page table management (`src/mm/paging.rs`, `src/mm/page_table.rs`) enforce the following structural invariants:

1. **Explicit Invariants**: 4-level x86_64 page tables (`PML4 -> PDPT -> PD -> PT`). Page directory and page table entries must be aligned to 4096-byte boundaries.
2. **Race-Free Mappings**: Modifying page table entries requires hold of the address space page table lock. TLB invalidations (invlpg / TLB shootdown) are issued immediately after mapping modifications.
3. **Bounded Page Table Capacity**: Page table memory growth is bounded by process Virtual Memory Area (VMA) maximum allocations and physical frame capacity limits.
4. **Lost-Update Prevention**: Updates to page table attributes (e.g. dirty, accessed, present bits) use atomic compare-and-swap operations to prevent concurrent overwrites during page reclaim or swap-out.
5. **Double-Free Protection**: Page tables verify frame reference counts before releasing underlying frames, preventing double-free corruption during address space destruction.
6. **Safe Address-Space Switching**: Context switches reload CR3 or switch PCID/ASID tags safely without exposing stale or intermediate mapping states to executing threads.
