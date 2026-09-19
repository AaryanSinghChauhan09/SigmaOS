# AI Agent Development Instructions for Cluster Bitmap Operations & Storage Allocation (`src/fs/` & `src/orchestration/`)

This document provides guidelines for storage cluster block allocation bitmaps, distributed node cluster status bitmaps, zero-allocation cluster bitmap queries, and atomic cluster allocation locks in SigmaOS.

## Subsystem Architecture & Directives

1. **Filesystem Cluster Bitmap Allocation (`src/fs/sigmafs.rs` & `src/fs/vfs.rs`)**
   - Filesystem storage clusters (contiguous groups of 4 KiB file blocks) track allocation states using bitmap structures (`SigmaBitmap` / `ClusterBitmap`).
   - Bit `0` indicates a free storage cluster; bit `1` indicates an allocated storage cluster.
   - Contiguous multi-cluster allocations (for large sequential files) must use $O(1)$ SIMD/bitwise trailing zero count primitives (`cttz` / `popcount`) to locate continuous free cluster runs.

2. **Distributed Node Cluster State Bitmaps (`src/orchestration/sigmakube.rs`)**
   - SigmaKube cluster orchestrators track node health and deployment states using bitmask vectors (`ClusterNodeStateBitmap`).
   - Cluster status updates (`Online`, `Draining`, `Offline`, `Cordoned`) must update node state bitmasks atomically.

3. **Concurrency & Synchronization**
   - Concurrent storage cluster allocations must acquire cluster block allocation locks (`ClusterBitmapLock`) or use lock-free atomic bit operations (`AtomicU64::fetch_or`/`fetch_and`) to prevent double-allocation race conditions across threads.

4. **Verification**
   - Run `cargo check --lib` to ensure no warnings or broken imports are introduced.
