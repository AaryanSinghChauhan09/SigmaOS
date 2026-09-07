# AI Agent Contiguous Allocation Operation Management Guide

## Overview
This wiki guide details Contiguous Allocation Operation Management protocols for AI coding agents operating on SigmaOS. It covers Contiguous Memory Allocator (CMA) reserved pools (`allocate_contiguous`, `release_contiguous`), DMA contiguous buffer allocation (`find_contiguous_free_block`), real-time defragmentation coalescing, `vmalloc` virtual contiguous mapping, and file system contiguous cluster relocation.

## Key Principles
1. **CMA Reservation**: Physical DMA and GPU buffers require contiguous physical frames reserved via CMA pools.
2. **DMA Coalescing**: If contiguous DMA blocks are unavailable, active pages are shifted in real-time to coalesce a single contiguous free region.
3. **Virtual Contiguity**: `vmalloc` maps non-contiguous physical frames into a contiguous virtual address range for large kernel structures.

## CMA Allocation Example (`src/kernel/memory/sigma_buddy.rs`)
```rust
let phys_addr = cma.allocate_contiguous(16)?;
// ... perform DMA transfer ...
cma.release_contiguous(phys_addr, 16)?;
```

## Related Documents
- `docs/AI_AGENT_CONTIGUOUS_ALLOCATION_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_CONTIGUOUS_ALLOCATION_OPERATION_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENT_MEMORY_OPERATION_MANAGEMENT.md`
