# SigmaOS AI Agent Cache Operation Management Guidelines

## 1. Executive Summary & Overview

Cache subsystems in SigmaOS operate across multiple architectural tiers: CPU L1/L2/L3 hardware caches, kernel VFS page and dentry caches, ZFS Adaptive Replacement Cache (ARC), package content-addressed store (CAS) caches, and eBPF LSM security policy caches. Efficient cache management by AI agents is critical to maintaining high I/O throughput, low memory pressure, and zero stale-state hazards.

This document establishes the official guidelines and architectural standards for AI agents managing cache operations, cache eviction policies, hardware cache coherence, and automated cache pruning in SigmaOS.

---

## 2. Taxonomy of Cache Subsystems in SigmaOS

AI agents manage five core cache layers across kernel and userland space:

| Cache Subsystem | Module Path | Cache Topology & Eviction Strategy |
| :--- | :--- | :--- |
| **Package CAS Store Cache** | `src/package/cache.rs`, `src/package/paccache.rs` | Content-Addressed Store (`/var/lib/sigpkg/store`), LRU pruning via `paccache` |
| **ZFS Adaptive Replacement Cache** | `src/filesystem/` | ZFS ARC cache balancing MRU (Most Recently Used) and MFU (Most Frequently Used) lists |
| **Kernel VFS Page & Inode Cache** | `src/kernel/vfs/vfs.rs` | Page cache and dentry/inode lookup cache with `drop_caches` sysctl trigger |
| **Nix Flakes Binary Cache** | `src/sigpkg/sovereign_package_innovations.rs` | Binary substituter cache (`NixFlakesCacheEngine`) for hermetic build outputs |
| **Hardware CPU Cache Line** | `src/klib/isa.rs` | CPU L1/L2/L3 cache flushing (`clflush`, `clflushopt`, `wbinvd`) and non-temporal I/O |

---

## 3. Cache Operation Protocols for AI Agents

### 3.1 Package Store & CAS Cache Pruning (`paccache`)

1. **Content-Addressed Store Garbage Collection**:
   - Package tarballs and uncompressed layer blobs stored in `/var/lib/sigpkg/store` are indexed by SHA256 hashes.
   - AI agents run automated cache garbage collection (`paccache_prune`) when store disk usage exceeds configured thresholds (e.g. 80% partition capacity).
2. **Retention Policy**:
   - By default, retain the latest 2 installed package versions and remove unreferenced orphan CAS blobs (`gc_orphan_blobs`).

---

### 3.2 ZFS Adaptive Replacement Cache (ARC) Management

1. **ARC Memory Pressure Adaptation**:
   - `ZfsArcCacheEngine` dynamically scales ARC memory size (`c_min` to `c_max`) based on global page frame pressure.
   - When kernel memory pressure rises (`sys_memory_pressure > 85%`), AI agents signal ZFS ARC to evict MRU/MFU unmapped pages before invoking the OOM killer.

---

### 3.3 Kernel Page Cache & VFS Dentry Dropping

1. **System Drop Caches**:
   - AI agents invoke `/proc/sys/vm/drop_caches` equivalent calls to reclaim reclaimable page cache (`1`), dentries/inodes (`2`), or both (`3`).
2. **Zero-Copy Page Cache Pinning**:
   - Direct I/O and DMA transfers pin page cache memory frames using MDL (Memory Descriptor List) descriptors to prevent unexpected cache eviction during active transfers.

---

### 3.4 Hardware CPU Cache Coherence & Invalidation

For Ring-0 driver developers and hardware agents:

1. **DMA Buffer Flushing**:
   - Non-cache-coherent DMA hardware transfers require explicit cache line flushes (`clflush` / `clflushopt`) or write-back invalidations (`wbinvd`) prior to handing buffer control to peripheral PCI/NVMe devices.
2. **Non-Temporal Stores**:
   - High-throughput streaming writes (e.g. framebuffers, NVMe queues) use non-temporal instructions (`movntdq`) to bypass CPU L1/L2 caches and prevent cache pollution.

---

## 4. Verification & Cache Testing Protocol

AI agents modifying cache management routines must pass verification:

1. **Package Cache Suite**: Execute `./run_sigma_tests.sh` to run `test_package_caching_engine`.
2. **System Stress & Fuzzing Matrix**: Run `tests/stress_and_fuzz_tests.rs` to verify zero memory leaks and cache consistency under continuous file I/O and package transaction fuzzing.

---

*Approved by the SigmaOS Storage, Memory & File System Steering Committee.*
