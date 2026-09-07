# Cache Levels & Memory Hierarchy Guidelines for AI Agents (`docs/ai_agents_cache_levels.md`)

This document provides AI agents with directives, architectural standards, Rust structures, and safety rules for managing **Cache Levels** across CPU hardware simulation, kernel virtual memory, virtual filesystem, and package management in SigmaOS.

---

## 1. Multi-Level Cache Overview in SigmaOS

SigmaOS manages caching across four distinct architectural tiers:

| Cache Tier | Scope | Implementation File | Key Features & Protocols |
| :--- | :--- | :--- | :--- |
| **CPU Hardware Cache** | L1I, L1D, L2, L3 | `src/kernel/mm/cpu_cache.rs` | 8/16-way set associativity, MESI coherence, PLRU/LRU eviction, write-through/write-back |
| **Kernel Page Cache** | VFS / Disk I/O | `src/kernel/mm/page_cache.rs` | Dirty page flushing, read-ahead buffer pages, page status flags |
| **Kernel Object Slab Cache** | Kernel Memory Allocator | `src/kernel/mm/slab_allocator.rs` & `src/klib/slab.rs` | Object caches (e.g. `dentry`, `inode`, `task_struct`), slab reuse, cache pressure reclamation |
| **Package Cache (`paccache`)** | Userland / SigmaPkg | `src/package/paccache.rs` | Distro package tarball retention, version trimming (`paccache -r -k 2`) |

---

## 2. Hardware CPU Cache Hierarchy (`src/kernel/mm/cpu_cache.rs`)

The core kernel models physical CPU cache hierarchies to optimize memory access patterns and simulate multi-core cache coherence:

### 2.1 Cache Levels & Properties
* **L1 Instruction Cache (L1I):** 32KB per core, 8-way set associative, pseudo-LRU (PLRU), read-only.
* **L1 Data Cache (L1D):** 32KB per core, 8-way set associative, PLRU, 4-cycle access latency.
* **L2 Unified Cache (L2U):** 512KB per core, 8-way set associative, true LRU, 12-cycle access latency.
* **L3 Shared Cache (L3U):** 16MB shared across cores, 16-way set associative, true LRU, 40-cycle access latency.
* **DRAM Backing:** Main memory, 200-cycle access latency.

### 2.2 Cache Coherence Protocol (MESI)
State transitions across SMP CPU cores follow standard MESI protocol flags:
* **Modified (M):** Cache line is dirty and present only in local L1/L2.
* **Exclusive (E):** Cache line is clean and present only in local L1/L2.
* **Shared (S):** Cache line is clean and present in multiple CPU caches.
* **Invalid (I):** Cache line contains stale or unallocated memory.

---

## 3. Kernel Virtual Memory & Page Cache (`src/kernel/mm/page_cache.rs`)

The VFS page cache caches block storage sectors in physical RAM pages (`CachedPage`) to accelerate file read/write operations:

```rust
pub struct PageCache {
    pages: HashMap<u64, CachedPage>, // page_index -> CachedPage
    capacity: usize,
}
```

Key operations:
* **Page Lookup & Allocation:** On read miss, allocate a physical RAM page, fetch block storage bytes into memory, and insert into `PageCache`.
* **Dirty Page Synchronization:** Modifying file memory marks `CachedPage` as dirty (`PageStatus::Dirty`). The kernel background flusher or `sync()` writes dirty pages back to disk.

---

## 4. Kernel Object Slab Cache (`src/klib/slab.rs` & `src/kernel/mm/slab_allocator.rs`)

To eliminate physical memory fragmentation for frequently created and destroyed kernel structures (such as VFS `dentry` or `inode` objects), SigmaOS uses slab caches:

```rust
pub struct SlabCache<T> {
    object_size: usize,
    free_list: Vec<Box<T>>,
}
```

* **Allocation:** Reuses pre-allocated memory slots from `free_list` before invoking the physical page allocator.
* **Reclamation:** Under kernel memory pressure, slab caches free empty slabs back to the physical buddy allocator.

---

## 5. VFS Directory Cache & Cache Pressure Tuning

VFS path resolution performance relies on the directory dentry cache (`dcache`). AI agents can tune VFS cache retention using `vfs_cache_pressure`:

* **`vfs_cache_pressure = 20`:** Retain directory and inode caches aggressively in RAM (favors file-intensive workloads, e.g. web servers, game loading).
* **`vfs_cache_pressure = 100`:** Default balanced cache reclamation pressure.
* **`vfs_cache_pressure = 150`:** Reclaim VFS directory caches aggressively to free memory for application heap space.

---

## 6. Package Cache Management (`paccache`) (`src/package/paccache.rs`)

`PaccacheEngine` manages downloaded package tarball storage under `/var/cache/sigpkg/pkg/`:

```rust
pub struct PaccacheEngine {
    pub config: PaccacheConfig,
    pub cache_entries: Vec<PackageCacheEntry>,
}
```

* **Trimming Old Packages:** Retains the `k` most recent versions of installed packages (default `keep_versions = 2`) and purges older uninstalled package tarballs to conserve disk space.

---

## 7. Directives & Guidelines for AI Agents

1. **Flush Page Caches Before Unmounting:**
   Always flush dirty pages in `PageCache` before unmounting filesystems or shutting down storage devices to avoid data corruption.
2. **Account for L1/L2 Latency in Perf Benchmarks:**
   When analyzing scheduler or kernel IPC latency, ensure cache line bouncing and L3 misses are minimized by aligning structs to 64-byte boundaries (`#[repr(align(64))]`).
3. **Respect `vfs_cache_pressure` Limits:**
   Do NOT manually purge dentry caches unless the system experiences low-memory warnings or userland memory allocations fail.

---

## 8. Verification & Testing Procedure

When modifying cache hierarchy or page cache code:

1. **Run CPU Cache & Page Cache Unit Tests:**
   ```bash
   cargo test --lib kernel::mm::cpu_cache
   cargo test --lib kernel::mm::page_cache
   cargo test --lib klib::slab
   ```

2. **Run Full Kernel Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```

---
*Maintained by the SigmaOS Core Kernel Memory Team.*
