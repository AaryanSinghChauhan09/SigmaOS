# AI Agent Contiguous Allocation Operation Management Architecture

## Executive Overview

Contiguous Allocation Operation Management in SigmaOS governs physical and virtual contiguous memory allocations, DMA buffer coalescing, kernel virtual memory (`vmalloc`), and filesystem cluster relocation. Implemented across `src/kernel/memory/sigma_buddy.rs`, `src/drivers/unified_dma.rs`, `src/kernel/mm/vmalloc.rs`, and `src/filesystem/defragmenter.rs`, SigmaOS uses Contiguous Memory Allocator (CMA) reserved pools (`allocate_contiguous`, `release_contiguous`), real-time DMA defragmentation coalescing (`find_contiguous_free_block`), and virtual contiguous mappings for hardware device access with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents requesting, coalescing, or managing contiguous memory allocations in SigmaOS.

---

## Subsystem Architecture & Contiguous Memory Pipeline

```
                                +-----------------------------------+
                                |    DMA / Device Driver Request    |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    Contiguous Memory Allocator    |
                                |     (src/kernel/memory/buddy)     |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | CMA Physical Reservation| | Unified DMA Buffer|   | vmalloc Virtual Map   |
            | allocate_contiguous() |   | Defrag Coalescing |   | Non-Contig Phys ->    |
            | release_contiguous()  |   | find_contiguous() |   | Contiguous Virt Space |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                |  Physical ISA / GPU Buffer Access |
                                +-----------------------------------+
```

### Core Contiguous Allocation Components

1. **Contiguous Memory Allocator (CMA) (`src/kernel/memory/sigma_buddy.rs`)**:
   - `allocate_contiguous(count_pages)`: Reserves contiguous physical memory frames from the dedicated CMA pool for high-performance DMA and GPU framebuffers.
   - `release_contiguous(phys_addr, count_pages)`: Returns contiguous physical frames to the CMA pool.

2. **Unified DMA Coalescing Engine (`src/drivers/unified_dma.rs`)**:
   - `find_contiguous_free_block(pages_needed)`: Searches physical page occupancy tables for $N$ contiguous free frames.
   - Real-Time Defragmentation: Automatically shifts active fragmented DMA pages to coalesce a single large contiguous block when fragmented memory causes allocation failure.

3. **Kernel Virtual Contiguous Allocator (`src/kernel/mm/vmalloc.rs`)**:
   - Maps non-contiguous physical pages into a contiguous virtual address range for kernel subsystem buffers exceeding single-page bounds.

4. **File System Contiguous Extent Relocation (`src/filesystem/defragmenter.rs`)**:
   - `find_free_contiguous_space(size)`: Relocates fragmented file cluster chains into contiguous sector ranges for sequential read optimization.

---

## Zero-Allocation Guardrails

AI agents executing contiguous memory operations must enforce these zero-allocation constraints:
- Bitmask scanning for contiguous free pages operates in $O(N)$ register bitwise loops without heap allocation.
- In-place DMA page coalescing shifts physical page descriptors via memory moves within pre-allocated kernel tables.

---

## Related Architectural References
- `src/kernel/memory/sigma_buddy.rs` - Master Buddy Allocator and CMA driver.
- `src/drivers/unified_dma.rs` - Unified DMA buffer manager.
- `src/kernel/mm/vmalloc.rs` - Virtual contiguous memory manager.
- `src/filesystem/defragmenter.rs` - Storage contiguous cluster defragmenter.
