# AI Agent Development Instructions for Cache Memory & Page Cache Subsystems (`src/kernel/mm/cpu_cache.rs` & `src/kernel/mm/page_cache.rs`)

This document details guidelines for CPU cache coherence (L1/L2/L3 caches), instruction/data cache flushing (`CLFLUSH`/`INVD`/`WBINVD`), VFS page cache management, and dirty page writeback flushing in SigmaOS.

## Subsystem Architecture & Directives

1. **CPU Cache Management & Coherence (`cpu_cache.rs`)**
   - Manages CPU cache topology, cache line sizes (typically 64 bytes), L1 instruction/data caches, unified L2/L3 caches, and Non-Temporal SIMD memory store flushes (`movntdq`).
   - When modifying DMA memory regions or executable code pages (JIT / dynamic kernel modules), invalidate/flush affected cache lines using explicit `clflush` instructions or TLB invalidations (`invlpg`).

2. **VFS Page Cache & Dirty Page Writeback (`page_cache.rs`)**
   - Implements VFS page caching to accelerate file system I/O reads/writes.
   - Cache eviction policies: LRU (Least Recently Used) or 2Q (Two-Queue) page replacement algorithms.
   - Background flusher threads write dirty pages back to disk (`sync` / `fsync` parity) when dirty page thresholds exceed `20%` of available RAM.

3. **Concurrency & Lock Safety**
   - Page cache lookup operations must use read-copy-update (RCU) or lock-free radix tree lookups to minimize read lock contention.
   - Hold page locks only during page writeback or buffer state transitions.

4. **Verification**
   - Verify cache management logic using `cargo check --lib`.
