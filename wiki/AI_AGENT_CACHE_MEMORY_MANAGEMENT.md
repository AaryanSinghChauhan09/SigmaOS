# AI Agent Guidelines for SigmaOS Cache Memory & Package Block Cache Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Blocks of Cache Memory Management**, covering both kernel page caching and package block/proxy caching.

---

## 1. System Architecture & Cache Subsystem Overview

SigmaOS implements two primary caching tiers across kernel memory management and userland package distribution:

1. **Kernel Page & File Block Cache (`src/kernel/mm/page_cache.rs`)**
   - Absorbs Linux `mm/filemap.c` and `mm/page-writeback.c`.
   - Manages physical memory page caching (`PageCache`, `CachedPage`), dirty page tracking, page pinning (`pin_count`), and sticky priority tiers (`PagePriority`).
   - Features distro-inspired cache engines:
     - **Clear Linux Prefetching Engine (`ClearLinuxReadAheadEngine`):** Sequential access tracking and dynamic read-ahead prefetch window scaling.
     - **NixOS Hash Deduplicator (`NixOSPageDeduplicator`):** Fast FNV-1a content-hash page index for immutable page content deduplication.
     - **SteamOS Writeback Throttle (`SteamOSWritebackThrottle`):** Background writeback throttling with high/low watermarks to preserve interactive UI responsiveness.

2. **Package Block & Offline Proxy Cache (`src/package/cache.rs`)**
   - Implements offline-first package caching and registry proxying (`SimplePackageCache`, `SimpleCachedPackage`).
   - Manages local block cache storage, O(1) constant-time metadata lookups, configurable eviction policies (`EvictionPolicy` - LRU, LFU, FIFO via `SimpleCacheEviction`), registry proxying (`SimpleRegistryProxy`), and offline synchronization (`SimpleOfflineMode`).

---

## 2. Core Structures & Code Patterns

AI agents modifying cache management algorithms must adhere to these core data structures and methods:

### Kernel Page Cache (`PageCache`)
- **Page Statuses (`PageStatus`):** `Clean`, `Dirty`, `Writeback`, `Evicted`.
- **Page Priority Tiers (`PagePriority`):** `Low`, `Standard`, `High`, `Required` (sticky priority tier protecting pages from standard eviction).
- **Page Lifecycle:**
  - `lookup(inode_id, page_idx)`: Retrieves a mutable reference to `CachedPage` and triggers sequential read-ahead window evaluation.
  - `write_page(inode_id, page_idx, offset, data)`: Writes slice data, marks page status dirty, registers hash with `NixOSPageDeduplicator`, and triggers writeback flush if `SteamOSWritebackThrottle` watermark is exceeded.
  - `writeback_all()`: Flushes all dirty pages to disk, transitioning status to `Clean` and tracking `writeback_ops`.

```rust
use sigma::kernel::mm::page_cache::{PageCache, PagePriority};

let mut cache = PageCache::new(1024); // 1024 x 4KB pages
cache.write_page(1, 0, 0, b"SigmaOS Cached Block Data");

// Elevate priority to protect page from eviction
if let Some(page) = cache.lookup(1, 0) {
    page.priority = PagePriority::Required;
}

// Flush dirty pages to storage
cache.writeback_all();
```

### Package Block Cache (`SimplePackageCache`)
- **Cached Package Representation (`SimpleCachedPackage`):** Fixed 4096-byte memory block container with atomic size and timestamp tracking.
- **Cache Eviction (`SimpleCacheEviction`):** Evicts package blocks according to selected policy (`LRU`, `LFU`, `FIFO`).
- **SIMD Optimized Operations:** Bulk slice copying (`copy_from_slice`) for SIMD-accelerated memory block caching in `SimpleRegistryProxy`.

```rust
use sigma::package::cache::{SimplePackageCache, SimpleRegistryProxy, RegistryProxy};

let cache = SimplePackageCache::new(50); // 50MB max cache
let mut proxy = SimpleRegistryProxy::new(cache);

// Cache package binary block
proxy.cache_response(b"kernel-zen", b"PACKAGE_PAYLOAD_BYTES")?;
```

---

## 3. Testing & Verification Protocol for AI Agents

When modifying page cache engines or package block caches, AI agents must run the following validation commands in order:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for page cache and package block cache:

```bash
rustc --test --edition=2021 src/kernel/mm/page_cache.rs -o build/test_page_cache && ./build/test_page_cache
rustc --test --edition=2021 src/package/cache.rs -o build/test_pkg_cache && ./build/test_pkg_cache
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core memory management subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Performance Directives

- **Non-Blocking Evictions:** Ensure `can_evict()` respects pinned references (`pin_count == 0`), non-writeback state, and sticky `PagePriority::Required` tiers.
- **Bulk Memory Copying:** Use SIMD-optimized `copy_from_slice` instead of byte-by-byte loops for block cache memory transfers.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
