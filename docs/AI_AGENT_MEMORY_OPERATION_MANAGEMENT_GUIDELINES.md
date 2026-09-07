# SigmaOS AI Agent Memory Operation Management Guidelines

## 1. Overview
SigmaOS provides low-level physical and virtual memory management subsystems operated autonomously by AI memory agents (such as `MemoryOperationManager`, `PmmVmmPagingGovernor`, `SlabObjectCacheManager`, and `SimdMemoryVectorEngine`). These guidelines define physical page frame allocation, slab/object cache allocations, SIMD vectorized memory routines (`klib::isa`), zero-copy DMA ring buffers, Memory Descriptor List (MDL) pinning, and UVM page loans for AI agents in SigmaOS.

## 2. Core Memory Operation Management Principles

### 2.1 Physical & Virtual Memory Management (PMM / VMM)
- **PMM Bitmap & Page Frame Allocation**: AI agents allocate physical page frames ($4\text{KB}$, $2\text{MB}$ huge pages, $1\text{GB}$ giant pages) via bitmap page frame allocators (`src/memory/pmm_vmm.rs`).
- **PML4 Self-Referential Paging**: Page tables are mapped via PML4 self-referential page mapping (`PML4_SELF_REF_INDEX = 510`). AI agents handling page faults handle Copy-on-Write (COW) faults atomically.

### 2.2 Slab & Object Cache Allocations
- **Size-Class Slab Allocators**: Small, frequent object allocations use slab object caches (`SlabObjectCacheAllocator` in `src/kernel/memory.rs`).
- **Recycle Bin Node Chaining**: Freed slab objects are linked into intrusive free-list chains for $O(1)$ zero-fragmentation re-allocation.

### 2.3 Vectorized SIMD Memory Routines (`klib::isa`)
- **Hardware-Vectorized Memcpy / Memset**: Large memory copies or fills execute hardware-optimized AVX-512, AVX2, SSE2, or ARM NEON SIMD routines routed by `klib::isa`.
- **Zero-Allocation Guarantee**: Utility functions in `klib` operate with zero heap allocations, ensuring no unexpected allocation panics occur during low-memory execution.

### 2.4 Zero-Copy DMA Buffers & MDL Pinning
- **Memory Descriptor List (MDL) Pinning**: High-speed PCI Express device transfers pin physical memory pages using MDL descriptors, preventing page migration during active DMA transactions.
- **DmaRingBuffer Allocation**: Hardware drivers allocate contiguous ring buffers (`DmaRingBufferAllocator`) with 64-byte cache line alignment.

### 2.5 Universal Virtual Memory (UVM) Page Loans
- **UVM Anonymous Mapping & Loans**: AI processes exchange zero-copy memory pages via UVM page loan grants (`src/klib/uvm.rs`), transferring page ownership without copying byte payloads across address spaces.

---
*Maintained by the SigmaOS Virtual Memory & Kernel Steering Committee.*
