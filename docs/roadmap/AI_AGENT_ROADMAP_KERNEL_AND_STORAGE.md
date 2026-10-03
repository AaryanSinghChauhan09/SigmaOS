# SigmaOS AI Agent Roadmap: Kernel Subsystems & Advanced Storage Layer

## Overview & Architecture Parity Goals
This document specifies the operational roadmap for AI engineering agents working on the SigmaOS `#![no_std]` Safe Rust kernel and storage subsystems. The target is full architectural parity and functional superiority over modern Linux (6.12+ LTS) and BSD kernels (FreeBSD 14.1, OpenBSD 7.6, NetBSD 10.0, DragonFly BSD 6.4).

---

## 1. Process Scheduling & Concurrency Subsystem

### Linux & BSD Inspiration Sources
- **Linux 6.12+ EEVDF & BORE (Burst-Oriented Response Enhancer):** Fair-latency deadline scheduling for interactive desktop and latency-critical AI workloads.
- **Linux `sched_ext` (Extensible Scheduler Class):** Dynamic BPF-based user/kernel space scheduler hot-swapping.
- **FreeBSD ULE Scheduler:** Multi-core affinity, topology-aware SMT/NUMA queue balancing.
- **OpenBSD SCHED_BSD:** Simple lockless runqueues with fine-grained CPU pledge constraints.

### AI Agent Execution Directives
1. **BORE & EEVDF Rust Implementation:**
   - Enhance `src/kernel/process.rs` and `src/performance/smart_optimizer.rs` with virtual runtime tracking, lag calculation, and burst score decay algorithms.
   - Enforce O(log N) intrusive red-black tree task selection without heap allocations in `#![no_std]`.
2. **`sched_ext` Parity Engine:**
   - Support safe Rust dynamic scheduler plugin loading via capability-checked function pointer tables (`KernelSchedulerClass`).
3. **Lockless RCU & Concurrency Primitives:**
   - Verify non-blocking Read-Copy-Update (`src/kernel/rcu.rs`) deferred reclamation invariants.

---

## 2. Virtual Memory & Physical Page Allocator

### Linux & BSD Inspiration Sources
- **Linux Transparent Huge Pages (THP) & Auto-NUMA:** Dynamic 2MB/1GB page promotion/demotion.
- **FreeBSD VM Subsystem & Superpages:** Dynamic page reservation and reservation-queue coalescing.
- **NetBSD UVM (Universal Virtual Memory):** Page fault clustering, anonymous memory swap-backed objects.

### AI Agent Execution Directives
1. **Buddy Allocator Optimizations:**
   - Maintain O(1) bitwise trailing zero calculation (`trailing_zeros()`) in `src/kernel/memory.rs` for buddy order determination.
   - Validate checkpoint/restore recovery mechanisms (`create_checkpoint`, `restore_checkpoint`) during kernel memory panic recovery.
2. **Copy-on-Write (CoW) Demand Paging:**
   - Implement type-safe physical page reference counting (`page_ref_counts`) and fault handlers in `VirtualMemoryManager`.
3. **Hardware Memory Protection (Intel MPK / PKEY):**
   - Wire `SovereignIntelMpkEngine` (`src/security/memory_protection.rs`) to process page tables to enforce user-space memory domain segmentation.

---

## 3. High-Performance Storage & Filesystem Layer

### Linux & BSD Inspiration Sources
- **Bcachefs (Linux 6.7+):** Structural copy-on-write, checksumming, multi-device tiering, encryption, and native compression.
- **ZFS (FreeBSD OpenZFS):** ARC page cache, RAID-Z block allocation, ZIL logging, and dataset snapshots.
- **HAMMER2 (DragonFly BSD):** Fine-grained snapshot isolation, master/slave clustering, zero-copy deduplication.
- **OpenBSD FFS / NetBSD LFS:** Fast File System journaling, W^X metadata protection, and immutable append-only logs.

### AI Agent Execution Directives
1. **SigmaFS Engine Hardening:**
   - Update `src/filesystem/sigma_fs.rs` with B-tree extent indexing, ChaCha20-Poly1305 block-level encryption, and Zstd compression.
2. **Unified Page Cache:**
   - Integrate `PageCacheManager` (`src/memory/page_cache.rs`) with `VirtualMemoryManager` for zero-copy file-backed page mapping (`mmap`).
3. **Crash-Consistent Journaling & Atomic Rollback:**
   - Implement transactional journal commit logs supporting instant rollback recovery times under 1 second.
