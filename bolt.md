# ⚡ Bolt's Journal — SigmaOS Performance & Optimization

This journal logs CRITICAL performance bottlenecks, compiler optimization analyses, and resource-efficiency enhancements implemented across SigmaOS.

***

## 2026-08-01 - Eliminating Index-Based Modulo Division in Loop Bodies

**Learning:** Using standard indexing loops with modulo division (`i % key.len()`) inside hot loops introduces two severe performance penalties:

1.  **Division Overhead:** Integer division/modulo is one of the slowest CPU instructions (typically 10-40 cycles depending on the architecture).
2.  **Bounds Checks:** Direct array indexing (`key[index]`) forces the Rust compiler to insert branch/panic instructions for out-of-bounds safety checks, preventing auto-vectorization and loop unrolling.

Using a pre-allocated vector and a single-pass iterator chain (`.iter().cycle()`) zipped with the input iterator completely eliminates modulo division and array bounds checks, enabling the compiler to optimize the loop into highly efficient SIMD/vectorized instructions.
**Action:** Always prefer `.zip(key.iter().cycle())` over index-modulo loops for symmetric/XOR encryption and decryption operations.

## 2026-08-01 - Avoiding Heap Allocations in Dependency Traversal

**Learning:** Recursively traversing dependency trees with naive `to_visit: Vec<String>` structures incurs heavy heap reallocation and copy overhead if strings are cloned at every node visit. Storing references (`&str`) or using `to_visit` stacks with capacity pre-allocation dramatically cuts allocator stress during package dependency resolution.
**Action:** Pre-allocate capacity for traversal stacks and use borrowed string references where lifetimes allow.
