# SigmaOS AI Agent Cache Memory & Block Size Management Guidelines

## 1. Overview
SigmaOS implements hardware and software cache block size management frameworks operated by AI system agents (such as `CacheBlockSizeManager`, `ZfsArcCacheEngine`, `PageCacheBlockGovernor`, and `CacheCoherenceManager`). These guidelines define L1/L2/L3 CPU cache line alignment (64-byte lines), cache flushing (`clflush` / `clflushopt`), VFS page cache block sizes (4KB / 64KB), ZFS Adaptive Replacement Cache (ARC) block sizes, and package cache pruning (`paccache`).

## 2. Core Cache Block Size Management Principles

### 2.1 CPU Hardware Cache Line Alignment (64-Byte Lines)
- **Cache Line Padding**: High-frequency data structures, spinlocks, and ring buffer head/tail pointers (`HeapRingBuffer`, `SovereignPipe`) are padded to 64-byte alignment boundaries (`#[repr(align(64))]`) to eliminate false sharing across SMP CPU cores.
- **Cache Invalidation & Flushing**: Hardware DMA buffers execute `clflush` / `clflushopt` instructions to flush dirty cache lines prior to peripheral DMA read/write operations.

### 2.2 VFS Page Cache & ZFS ARC Block Sizes
- **VFS Page Cache Block Sizes**: Filesystem page caches utilize $4\text{KB}$ default block allocations, supporting $64\text{KB}$ jumbo page cache blocks for high-throughput NVMe / ZFS storage streams.
- **ZFS ARC Dynamic Block Sizing**: `ZfsArcCacheEngine` manages variable ZFS block sizes ($512\text{B}$ to $128\text{KB}$) based on file size and I/O access patterns, dynamically scaling ARC size under low-memory pressure.

### 2.3 Automated Package Cache Pruning (`paccache`)
- **Package Cache Pruning**: `paccache_prune` (`src/package/paccache.rs`) scans local package caches (`/var/cache/sigpkg/pkg/`), retaining the $N$ most recent package versions (default $N=2$) and removing obsolete `.deb`, `.rpm`, `.pkg.tar.zst`, and `.apk` archives.

---
*Maintained by the SigmaOS Cache & File System Steering Committee.*
