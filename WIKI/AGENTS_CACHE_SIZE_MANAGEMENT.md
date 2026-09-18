# SigmaOS AI Agent Cache Size Management Specification

This document specifies mandatory cache sizing bounds, slab object cache quota rules, package proxy cache retention limits, and CPU L1/L2/L3 cache alignment standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Slab Object Cache Size Limits
- **Kernel & Klib Slab Caches (`src/klib/slab.rs`, `src/memory/resource_allocator.rs`)**:
  - Slab object caches must specify maximum capacity quotas and page limits per slab object type (inodes, vnodes, file descriptors, IPC buffers).
  - Slab caches must release empty pages back to the buddy allocator when slab utilization drops below 25%.

## 2. Package Proxy & Local Package Cache Pruning
- **Registry Proxy Cache (`src/package/cache.rs`)**:
  - Package binary payloads must be stored using bulk memory transfers (`copy_from_slice`) into bounded registry proxy caches.
  - Package cache pruners (`paccache` / Arch `PacmanCacheCleaner` parity) must maintain a configurable candidate count limit (default: keep candidate count = 3 versions) and prune stale tarballs automatically.

## 3. Custom Allocator Recycle Bins & ASLR Guards
- **Custom Allocator Bins (`src/klib/custom_allocator.rs`)**:
  - Lock-free recycle bins must enforce maximum capacity bounds (e.g., maximum 64 recycled chunks) to prevent unbounded RSS memory growth.

## 4. Hardware CPU Cache Line Alignment
- **64-Byte Cache Line Alignment**:
  - Performance-critical data structures (mutexes, spinlocks, ring buffer head/tail pointers) must align to 64-byte cache line boundaries (`#[repr(align(64))]`) to eliminate false sharing.

## 5. AI Agent Cache Directives
1. **Never Allow Unbounded Caches**: Every hash map cache, registry proxy, or recycle bin must feature hard capacity caps or LRU eviction algorithms.
2. **Bulk Memory Transfers**: Memory cache stores must perform bulk slice copies rather than element-by-element iteration loops.
