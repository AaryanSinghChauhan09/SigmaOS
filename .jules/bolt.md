## 2025-03-02 - Bulk Memory Operations for `SigmaVec` and `SigmaString`
**Learning:** In standard `no_std` kernel/klib data structures, looping over slice elements using `push` incurs repetitive capacity bounds checks and reallocations. Replacing element-by-element iteration with `reserve(other.len())` followed by `core::ptr::copy_nonoverlapping` turns slice extension into an O(1) bulk SIMD/memcpy operation. Additionally, chaining `trim_start().trim_end()` allocates intermediate string buffers; calculating start/end indices in a single pass eliminates redundant heap allocations.
**Action:** When working with custom vector or string abstractions in `klib`, always prefer single-pass boundary calculations and bulk `extend_from_slice` memory copies over element-by-element loops.

## 2025-03-03 - Cached Lengths for Fixed-Size Slice Accessors
**Learning:** Fixed-size array wrappers (e.g., `[u8; 512]`) that compute slice length on the fly via `.position(|&b| b == 0)` incur an O(N) linear byte scan on every `data(&self)` call. Storing `data_len` as an explicit `u16` field during `new()` instantiation eliminates the linear scan, reducing accessor execution to an instantaneous O(1) slice index lookup.
**Action:** For fixed-length byte buffer structs representing strings or binary payloads, always cache explicit byte length fields at initialization to guarantee O(1) slice accessors.
