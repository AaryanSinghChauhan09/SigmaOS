# SigmaOS AI Agent Main Memory Management Specification

This document specifies mandatory physical memory zoning rules, page watermark evaluation standards, asynchronous page reclamation protocols (`kswapd`), and main heap memory invariants for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Physical Memory Zoning Architecture
- **Memory Zones (`src/memory/zone.rs`)**:
  - `ZONE_DMA`: Ancient 16MiB ISA DMA physical address space (0x0000_0000 - 0x0100_0000).
  - `ZONE_DMA32`: 32-bit PCI/PCIe DMA address space (below 4GiB).
  - `ZONE_NORMAL`: Directly mapped 64-bit physical RAM.
  - `ZONE_HIGHMEM`: 32-bit architecture high-memory regions requiring dynamic kmap page translations.

## 2. Page Watermark Evaluation & Asynchronous Reclamation (`kswapd`)
- **Watermark Thresholds (`src/memory/kswapd.rs`, `src/memory/manager.rs`)**:
  - `Watermark::High`: Free pages abundant; direct allocations proceed without wakeup.
  - `Watermark::Low`: Asynchronous `kswapd` daemon wakes up to scan LRU lists and reclaim inactive file/anonymous pages.
  - `Watermark::Min`: Direct page reclamation enforced on allocating threads; OOM-killer invoked if free memory drops below `min`.

## 3. Kernel Heap & Zone Allocator
- **Kernel Heap Management (`src/memory/heap.rs`)**:
  - Main kernel heap allocations must check for slab cache availability before falling back to page-level buddy allocation.
  - Heap expansion must preserve 4KiB page boundary alignment and update guard page protection masks.

## 4. AI Agent Main Memory Guidelines
1. **Never Bypass Watermarks**: AI agents modifying allocator routines must respect `min`/`low`/`high` watermark checks before returning physical frames.
2. **LRU Page Active/Inactive Balance**: Ensure page migrations between active and inactive LRU queues reflect access bit counts.
