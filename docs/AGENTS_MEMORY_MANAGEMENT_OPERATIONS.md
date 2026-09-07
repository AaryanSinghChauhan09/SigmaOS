# AI Agent Guidelines: Memory Management Operations in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Memory Management Operations**, physical memory frame allocation (`BitmapFrameAllocator`), virtual address space layout, 4-level paging (`PML4` → `PDPT` → `PD` → `PT`), two-tier kernel heap allocators (Slab & Buddy System), SMEP/SMAP hardware execution protection, and zram/zswap compressed swap operations in SigmaOS.

SigmaOS memory management (`src/memory/`, `src/kernel/memory.rs`, `src/klib/paging.rs`) adheres to a strict `#![no_std]` zero-dependency design, providing sub-nanosecond physical allocation lookups, lock-free slab object caches, and hardware-enforced supervisor isolation.

---

## 1. Memory Management Subsystems & Layout

AI agents interacting with memory management operations must interface with the following core architectural modules:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Physical Memory Manager (`BitmapFrameAllocator`)** | `src/klib/paging.rs`, `src/memory/bitmap_pmm.rs` | 4KB physical page frame allocator tracking available RAM frames via bitmask tables (`0` = free, `1` = allocated). |
| **Virtual Memory Manager (`SimpleVMM`)** | `src/memory/paging.rs`, `src/memory/pmm_vmm.rs` | 4-level x86_64 paging architecture managing PML4, PDPT, PD, and PT entries with 2MB/1GB Huge Page support. |
| **Slab & Buddy Allocators** | `src/memory/buddy_allocator.rs`, `src/memory/sigma_buddy.rs` | Two-tier kernel heap: Slab allocator for fixed-size objects (<4KB) and Buddy allocator for contiguous page orders (4KB to 4MB). |
| **SMEP & SMAP Protection Enforcer** | `src/security/kernel_hardening.rs`, `src/memory/segmentation_paging.rs` | Hardware supervisor mode execution (`SMEP`) and access (`SMAP`) prevention enforcer (`STAC`/`CLAC` AC flag handling). |
| **Page Swap & Reclamation (`kswapd`)** | `src/memory/kswapd.rs`, `src/memory/vmm_completion.rs` | Second-Chance Clock page replacement and zram compressed in-memory swap cache manager. |

---

## 2. Virtual Address Space Layout (x86_64 Canonical)

```
Kernel Virtual Address Space (Upper Half, 48-bit Canonical):
  0xFFFF800000000000 - 0xFFFF87FFFFFFFFFF  Physical Memory Direct Map (8TB)
  0xFFFF888000000000 - 0xFFFF88FFFFFFFFFF  vmalloc / ioremap Space
  0xFFFFFE0000000000 - 0xFFFFFEFFFFFFFFFF  Kernel Heap (Slab & Buddy)
  0xFFFFFFFF80000000 - 0xFFFFFFFFFFFFFFFF  Kernel Text & Data Segments

User Virtual Address Space (Lower Half, 48-bit Canonical):
  0x0000000000001000 - 0x00007FFFFFFFFFFF  User Space Range
  0x0000700000000000 - 0x00007FFFFFFFFFFF  User Stack (grows downward)
  0x0000600000000000 - 0x00006FFFFFFFFFFF  mmap Region
  0x0000000000400000 - 0x00005FFFFFFFFFFF  User Heap (brk, grows upward)
```

---

## 3. Allocation Protocols & Hardened Protection Rules

When making allocation or paging changes, AI agents must enforce the following security protocols:

### 1. SMEP and SMAP Compliance
- **SMEP (Supervisor Mode Execution Prevention):** Ring 0 Kernel MUST NEVER execute code residing in user-space pages.
- **SMAP (Supervisor Mode Access Prevention):** Ring 0 Kernel MUST NEVER read or write user-space memory without explicit `STAC` (Set AC flag) and `CLAC` (Clear AC flag) guards (`copy_from_user` / `copy_to_user`).

```rust
// Standard SMAP protection guard in SigmaOS kernel
pub fn safe_user_copy(dst: &mut [u8], src_user_ptr: *const u8) -> Result<(), MemoryError> {
    stac(); // Temporarily allow supervisor user-data access
    let res = unsafe { copy_user_bytes(dst, src_user_ptr) };
    clac(); // Immediately re-enable SMAP protection
    res
}
```

### 2. Physical Frame Allocation
- Always verify frame alignment (`phys_addr % 4096 == 0`).
- For multi-page contiguous DMA allocations, use the Buddy allocator order calculation (`order = log2(page_count)`).

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes to memory management or page table operations:

- [ ] Are physical address and virtual address alignments verified before mapping page tables?
- [ ] Are kernel memory allocations protected with non-executable (NX/XD) bits when writable?
- [ ] Are user-space memory copies wrapped in SMAP (`stac`/`clac`) safety blocks?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
