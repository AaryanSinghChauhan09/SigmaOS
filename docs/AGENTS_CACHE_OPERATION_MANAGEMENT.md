# Sovereign AI Agent Cache Operation Management Specification

This document specifies mandatory protocols for explicit CPU cache line flushing (`clflush`, `clflushopt`, `clwb`), Translation Lookaside Buffer (TLB) invalidation and inter-processor interrupt (IPI) shootdowns, Page Cache Radix-Tree lookups, SLUB object cache recycling, CPU cache line alignment (`#[repr(align(64))]`), and JIT instruction cache synchronization for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) operating within the SigmaOS kernel and userland subsystems.

---

## 1. Explicit CPU Cache Line Flushing & Persistent Memory Writeback

1. **Low-Level Cache Line Flushing Primitives**:
   - **`clflush`**: Flushes the specified cache line from all cache levels across all cores.
   - **`clflushopt`**: Optimized unordered cache line flush instruction allowing concurrent execution across non-overlapping addresses. Must be followed by `sfence` when memory order serialization is required.
   - **`clwb`**: Cache Line Write Back write-optimized primitive. Writes back modified cache line data to main memory without evicting the line from the L1/L2/L3 cache hierarchies.
2. **Persistent Memory Transaction Serialization**:
   - Updates to non-volatile RAM (NVRAM), NVDIMM persistent memory log structures, or transactional journals (`src/filesystem/ext4.rs`, `src/filesystem/cow_snapshot.rs`) must issue `clwb` / `clflushopt` instructions followed by `sfence` memory barriers before committing transaction headers.

---

## 2. Translation Lookaside Buffer (TLB) Invalidation & SMP Shootdowns

1. **Page-Table Invalidation Primitives (`src/memory/tlb_associative.rs`)**:
   - Virtual page unmapping or protection bit updates (`PTE_PRESENT`, `PTE_USER`, `PTE_RW`) must immediately execute CPU-specific page invalidation instructions:
     - **x86_64**: `invlpg [addr]`
     - **ARM64**: `tlbi vaae1is, [addr]`
     - **RISC-V**: `sfence.vma [addr]`
2. **SMP Inter-Processor Interrupt (IPI) Shootdown Protocol**:
   - In multi-core symmetric multiprocessing (SMP) environments, unmapping physical frames assigned to active process page tables requires broadcasting an IPI TLB shootdown request to all CPUs executing threads in the target page directory.
   - The originating CPU must wait for all target cores to acknowledge TLB invalidation before returning the physical page frame to the buddy frame allocator (`src/memory/pmm_vmm.rs`, `src/memory/buddy_allocator.rs`).

---

## 3. Page Cache & SLUB Object Cache Operations

1. **Page Cache Radix-Tree Lookups (`src/klib/adt.rs`, `docs/filesystem.md`)**:
   - VFS file page cache lookups use lock-free $O(\log N)$ Radix-Tree structures with Read-Copy-Update (RCU) protection.
   - Hot page cache allocations must maintain cache-line locality and avoid invalidating adjacent tree node cache lines.
2. **SLUB Object Cache Recycling (`src/klib/slab.rs`, `src/klib/custom_allocator.rs`)**:
   - Kernel slab allocators recycle fixed-size kernel objects (dentries, inodes, socket descriptors) using per-CPU lock-free object pools.
   - Unused slab pages must be reclaimed and returned to the physical memory allocator when slab utilization drops below 25%.

---

## 4. CPU Cache Line Alignment & False Sharing Prevention

1. **64-Byte CPU Cache Line Alignment (`#[repr(align(64))]`)**:
   - All high-frequency concurrent structures (spinlocks, atomic counters, lock-free ring buffer read/write indices in `src/klib/ringbuf.rs`) must enforce 64-byte alignment to prevent false sharing cache ping-ponging across CPU cores.
2. **Per-CPU Scratchpads**:
   - Frequently modified kernel metrics and scheduler task queues must utilize per-CPU variables to isolate cache lines to individual CPU cores.

---

## 5. DMA Buffer Coherency & JIT Instruction Cache Synchronization

1. **DMA Buffer Cache Coherency**:
   - For non-cache-coherent bus hardware (ISA, legacy PCI, embedded SPI/I2C), CPU data cache ranges covering DMA buffer regions must be invalidated before DMA receive reads and flushed after DMA write writes (`src/driver/ahci_sata_controller.rs`).
2. **JIT Dynamic Binary Translation Cache Invalidation**:
   - Dynamic binary translation engines (Rosetta translation layer, eBPF JIT compiler in `src/unimplemented_features.rs`) generating machine code into memory must flush data caches and invalidate the instruction cache (`isb` on ARM64, `fence.i` on RISC-V) prior to branch jumps.

---

## 6. AI Agent Cache Operation Directives Summary

1. **Always Flush Before DMA**: Explicitly flush/invalidate CPU data cache lines over DMA buffers before initiating controller DMA bus operations.
2. **Synchronize Multi-Core TLBs**: Execute IPI shootdowns across all active cores prior to freeing unmapped physical page frames.
3. **Enforce 64-Byte Alignment**: Annotate concurrent atomic pointers and queue indices with `#[repr(align(64))]`.
4. **Invalidate Instruction Caches on Code JIT**: Always issue CPU instruction cache invalidation barriers after generating or patching executable memory.
