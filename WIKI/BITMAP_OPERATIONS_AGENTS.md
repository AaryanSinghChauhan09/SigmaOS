# AI Agent Development Instructions for Bitmap Operations & Physical Memory Management (`src/klib/bitmap.rs` & `src/memory/bitmap_pmm.rs`)

This document provides directives for bitwise manipulations, physical frame allocation bitmaps, fast bit-search primitives, and atomic bit operations in SigmaOS.

## Subsystem Architecture & Directives

1. **Zero-Allocation Bitmap Primitive (`src/klib/bitmap.rs`)**
   - Implements high-performance `SigmaBitmap` tracking contiguous bits across `u64` word arrays.
   - Use intrinsics like `trailing_zeros()` (`cttz`), `leading_zeros()`, and `count_ones()` (`popcount`) for $O(1)$ contiguous free bit searches.

2. **Physical Memory Manager (PMM) Bitmap (`src/memory/bitmap_pmm.rs`)**
   - Tracks 4 KiB physical page frames across system DRAM.
   - Bit value `0` denotes an available physical frame; bit value `1` denotes an allocated or reserved page frame.
   - Contiguous physical frame allocations (for DMA buffers or huge pages) must perform multi-word bit searches using aligned 64-bit mask comparisons (`u64::MAX`).

3. **Atomic Bit Operations & Concurrency**
   - Multi-threaded or SMP bitmap state updates must use atomic bitwise operations (`AtomicU64::fetch_or`, `AtomicU64::fetch_and`) to prevent bit corruption without acquiring global locks.

4. **Verification**
   - Verify bitmap operations using `cargo check --lib` prior to submitting changes.
