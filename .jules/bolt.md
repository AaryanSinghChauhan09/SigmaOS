# ⚡ Bolt's Journal — SigmaOS Performance & Optimization

This journal logs CRITICAL performance bottlenecks, compiler optimization analyses, and resource-efficiency enhancements implemented across SigmaOS.

---

## 2026-08-01 - Eliminating Index-Based Modulo Division in Loop Bodies
**Learning:** Using standard indexing loops with modulo division (`i % key.len()`) inside hot loops introduces two severe performance penalties:
1. **Division Overhead:** Integer division/modulo is one of the slowest CPU instructions (typically 10-40 cycles depending on the architecture).
2. **Bounds Checks:** Direct array indexing (`key[index]`) forces the Rust compiler to insert branch/panic instructions for out-of-bounds safety checks, preventing auto-vectorization and loop unrolling.

Using a pre-allocated vector and a single-pass iterator chain (`.iter().cycle()`) zipped with the input iterator completely eliminates modulo division and array bounds checks, enabling the compiler to optimize the loop into highly efficient SIMD/vectorized instructions.
**Action:** Always prefer `.zip(key.iter().cycle())` over index-modulo loops for symmetric/XOR encryption and decryption operations.

## 2026-08-01 - Avoiding Heap Allocations in Dependency Traversal
**Learning:** Recursively traversing dependency trees with naive `to_visit: Vec<String>` structures incurs heavy heap reallocation and copy overhead if strings are cloned at every node visit. Storing references (`&str`) or using `to_visit` stacks with capacity pre-allocation dramatically cuts allocator stress during package dependency resolution.
**Action:** Pre-allocate capacity for traversal stacks and use borrowed string references where lifetimes allow.

## 2026-08-09 - Transitioning dynamic formatting out of hotpaths
**Learning:** Performing dynamic formatting like `format!("...")` inside critical execution loops blocks register reuse and triggers standard allocator locks. Replacing them with pre-allocated trace buffers saves microsecond context processing times.
**Action:** Always use static lifetime strings or write directly to static ring buffers in critical kernel tasks.

## 2026-08-10 - O(1) Short-Circuit Optimization in Buddy Allocator
**Learning:** When performing physical memory allocation search sequences under high concurrency, scanning empty priority free lists introduces O(N) traversal overhead. Short-circuiting the traversal immediately when the allocator saturation bitmask registers 0 for the requested block order guarantees O(1) failure latency and maximizes cache line locality in low-memory states.
**Action:** Enforce fast-path bitmask lookups before diving into segment search iterators.

## 2026-08-11 - Optimizing UKSM Page Deduplication Lookup Complexity
**Learning:** Checking for duplicate physical pages using a linear scan (`contains`) on a standard `Vec` inside high-frequency deduplication passes degrades to $O(N^2)$ complexity. Utilizing a `BTreeSet` reduces search and insertion overhead to $O(\log N)$ while maintaining complete `#![no_std]` compatibility.
**Action:** Always prefer `BTreeSet` or sorted collections over linear lookup vectors for high-density indexing/deduplication arrays in memory-constrained environments.

## 2026-08-14 - Zero-Allocation Pass for Kernel Slab Cache Selection
**Learning:** Allocating temporary heap vectors inside a kernel memory allocator's `allocate()` method introduces recursive allocation hazards and $O(N^2)$ sorting overhead on every allocation request. A single-pass $O(N)$ scalar search eliminates dynamic memory allocation during slab lookup and dramatically reduces kernel allocation latency.
**Action:** Never allocate dynamic memory inside core memory management search loops; use stack scalars or direct iterator passes.
