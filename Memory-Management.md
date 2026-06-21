# SigmaOS Memory Architecture

## Overview

SigmaOS uses a two-tier memory management system: a **Physical Memory Manager (PMM)** for hardware frames and a **Virtual Memory Manager (VMM)** for address space mapping. Together they support a Higher-Half Kernel layout and full Ring 3 user isolation.

## Physical Memory Manager (PMM)

The PMM uses a **Bitmap Allocator** where each bit represents one 4KB physical frame.

| Property | Value |
|---|---|
| Frame Size | 4,096 bytes |
| Bitmap Location | `0x100000` (1MB mark) |
| Reserved Area | First 2MB (Kernel + VGA + Bitmap) |
| API | `pmm_alloc_frame()` / `pmm_free_frame()` |

### Initialization
At boot, the PMM:
1. Places the bitmap at the 1MB physical mark.
2. Clears all bits (all frames free).
3. Marks the first 512 frames (2MB) as reserved to protect critical kernel structures.

## Virtual Memory Manager (VMM) — 4-Level Paging

SigmaOS implements the full x86_64 **4-Level Page Table** hierarchy:

```
Virtual Address (64-bit)
  [63:48] Sign Extension
  [47:39] PML4 Index  (9 bits)
  [38:30] PDP  Index  (9 bits)
  [29:21] PD   Index  (9 bits)
  [20:12] PT   Index  (9 bits)
  [11:0]  Page Offset (12 bits)
```

### Recursive Mapping

The 511th PML4 entry points back to the PML4 itself, enabling O(1) page table access without switching `cr3`:

```
pml4[511] → pml4 (self-reference)
```

This allows modifying any page table from kernel space by accessing the well-known recursive virtual addresses.

### TLB Invalidation

Every `mapVirtualToPhysical()` call executes `invlpg` to flush the TLB entry for that virtual address:

```cpp
__asm__ volatile("invlpg (%0)" ::"r"(vaddr) : "memory");
```

## Relevant Source Files

- `include/kernel/sigma_pmm.h` — PMM API
- `kernel/core/memory/sigma_pmm.cpp` — Bitmap allocator
- `kernel/core/memory/SovereignMemoryPaging.cpp` — 4-Level VMM + recursive mapping
