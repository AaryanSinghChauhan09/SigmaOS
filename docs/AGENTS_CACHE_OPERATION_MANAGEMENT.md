# SigmaOS AI Agent Cache Operation Management Specification

This document specifies mandatory explicit CPU cache line flushing (`clflushopt`/`clwb`), TLB entry invalidation/shootdown protocols, DMA buffer cache coherency standards, and JIT instruction cache synchronization rules for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Explicit CPU Cache Line Flushing & Persistent Memory Writeback
- **Cache Flushing Intrinsics**:
  - Modifications to persistent memory or non-volatile RAM structures must issue explicit `clwb` (Cache Line Write Back) or `clflushopt` instructions followed by `sfence` memory barriers.
  - Ensures modified data is written back to physical memory controllers before acknowledging transaction completion.

## 2. Translation Lookaside Buffer (TLB) Invalidation & Shootdowns
- **TLB Cache Operations (`src/memory/tlb_associative.rs`)**:
  - Virtual address unmapping or page table protection mask changes must issue `invlpg` (x86_64) or `tlbi` (ARM64) TLB page invalidations.
  - Multi-core SMP page table updates must issue inter-processor interrupt (IPI) TLB shootdowns before freeing physical frames.

## 3. DMA Buffer Cache Coherency & JIT Instruction Cache Synchronization
- **DMA Buffer Synchronization**:
  - Non-cache-coherent DMA transfers must invalidate or flush CPU data cache ranges before and after device DMA engine operations.
- **JIT & Dynamic Binary Translation Cache Invalidation**:
  - Dynamic code generation (Rosetta / eBPF JIT) must invalidate the CPU instruction cache (`isb` / `sys_icache_invalidate`) before executing generated machine instructions.

## 4. AI Agent Cache Operation Directives
1. **Always Flush Before DMA**: Invalidate CPU cache lines over DMA buffers prior to device transfer initiation.
2. **Synchronize Multi-Core TLBs**: Ensure IPI shootdowns complete across all CPU cores prior to returning physical frames to the allocator.
