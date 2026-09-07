# AI Agent Cache Operation Management Specification for SigmaOS

This document specifies operational standards for AI agents managing package store caches, ZFS ARC, VFS page/dentry caches, and CPU cache coherence in **SigmaOS**.

---

## 1. Cache Operation Management Protocol

AI agents managing cache subsystems must adhere to the following rules:

1. **CAS Package Cache Pruning**:
   - Execute `paccache_prune` to prune old package layers when disk usage exceeds 80%. Retain the 2 most recent package versions by default.

2. **ZFS ARC Pressure Response**:
   - Signal `ZfsArcCacheEngine` to shrink ARC cache limits when global physical memory pressure exceeds 85%.

3. **CPU Cache Line Invalidation**:
   - Use `clflush` / `clflushopt` on non-coherent DMA buffers prior to peripheral device activation.

4. **Hermetic Binary Caching**:
   - Validate SHA256 CAS hash checksums prior to populating or consuming entries from `NixFlakesCacheEngine`.

---

## 2. Verification Protocol

- Run `./run_sigma_tests.sh` to execute the package caching engine test suite and system integration matrix.

---

*Maintained by the SigmaOS Storage & Memory Steering Committee.*
