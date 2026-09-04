## 2025-03-02 - Bulk Memory Operations for `SigmaVec` and `SigmaString`
**Learning:** In standard `no_std` kernel/klib data structures, looping over slice elements using `push` incurs repetitive capacity bounds checks and reallocations. Replacing element-by-element iteration with `reserve(other.len())` followed by `core::ptr::copy_nonoverlapping` turns slice extension into an O(1) bulk SIMD/memcpy operation. Additionally, chaining `trim_start().trim_end()` allocates intermediate string buffers; calculating start/end indices in a single pass eliminates redundant heap allocations.
**Action:** When working with custom vector or string abstractions in `klib`, always prefer single-pass boundary calculations and bulk `extend_from_slice` memory copies over element-by-element loops.

## 2026-09-02 - Bulk `copy_from_slice` in Package Cache Buffer Allocation
**Learning:** In package registry proxy caching, copying payload buffers byte-by-byte in `for i in 0..data_len` loops forces per-index bounds checking and prevents the compiler from emitting vectorized `memcpy` intrinsics. Replacing manual byte-level array assignment with `cached.data[..data_len].copy_from_slice(&data[..data_len])` leverages optimized bulk CPU/SIMD memory transfer routines.
**Action:** When populating static or dynamic byte arrays in caching layers, always use `copy_from_slice` over manual element loops.
