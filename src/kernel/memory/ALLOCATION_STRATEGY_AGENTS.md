# AI Agent Development Instructions for Memory Allocation Strategies & Best-Fit Allocation (`src/kernel/memory/`)

This document outlines guidelines for memory allocation algorithms, buddy allocator coalescing, slab object cache fitting, and Best-Fit / Worst-Fit / First-Fit memory allocation strategies in SigmaOS.

## Subsystem Architecture & Directives

1. **Buddy Allocator & Best-Fit Block Selection (`sigma_buddy.rs` & `resource_allocator.rs`)**
   - The buddy allocator calculates block orders ($2^k$ pages). When searching freelists, select the smallest free block order that satisfies requested allocation sizes (Best-Fit strategy) to minimize internal memory fragmentation.
   - When splitting higher-order blocks into buddies, track migration types (`Unmovable`, `Reclaimable`, `Movable`, `Cma`, `HighAtomic`) to prevent anti-fragmentation cross-contamination.

2. **Slab & Zone Object Allocator (`slab_allocator.rs` & `src/slab.rs`)**
   - Object caches manage fixed-size slab pools (`32B`, `64B`, `128B`, `256B`, `512B`, `1024B`, `2048B`).
   - Route requested memory sizes to the Best-Fit slab size class. For instance, a 100-byte allocation request must route to the `128B` slab cache rather than a larger `256B` pool.

3. **NUMA-Aware Memory Fitting (`numa_allocator.rs` & `numa_aware.rs`)**
   - Best-Fit NUMA allocation prioritizes local CPU node memory banks first. Fall back to remote NUMA nodes with the lowest interconnect latency penalty when local node memory is exhausted.

4. **Locking & Verification**
   - Memory allocation strategies must execute in $O(1)$ or bounded $O(\log N)$ time and hold spinlocks for minimal duration.
   - Verify code changes with `cargo check --lib` before committing.
