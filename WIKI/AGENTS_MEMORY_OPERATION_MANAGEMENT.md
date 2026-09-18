# AGENTS_MEMORY_OPERATION_MANAGEMENT.md — AI Agent Memory Operation Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, memory safety invariants, page mapping rules, SIMD vector routing, and verification protocols for managing, developing, and extending **Memory Operations, Page Frame Allocation, Virtual Memory Mapping, DMA Buffers, and Memory Descriptor Lists (MDLs)** in **SigmaOS**.

---

## 1. SigmaOS Memory Operation Architecture Overview

Memory operations in SigmaOS span physical page frame allocation, PML4/PDPT/PD/PT virtual memory page tables, SIMD-accelerated memory copies (`memcpy`/`memset`), zero-copy DMA ring buffers, and Userland Virtual Memory (UVM) page loaning.

### Core Memory Modules
* **Physical Memory Manager (PMM) & Bitmap Allocator (`src/memory/pmm_vmm.rs`, `src/klib/bitmap.rs`)**:
  - `BitmapPmm`: Tracking physical frame availability in 4096-byte page frames (`PAGE_SIZE_4096`).
  - Boot memory map parsing (`BootMemoryMap`) and high-half kernel mapping (`PHYSICAL_MEMORY_OFFSET`).
* **Virtual Memory Manager (VMM) & Paging (`src/kernel/vmm_paging.rs`, `src/klib/paging.rs`)**:
  - PML4 self-referential page table index mapping (`PML4_SELF_REF_INDEX = 510`).
  - Page fault handling, Copy-on-Write (CoW) page duplication, and 2MB/1GB huge page mapping.
* **Buddy & Resource Allocators (`src/memory/sigma_buddy.rs`, `src/memory/resource_allocator.rs`)**:
  - Binary buddy allocator (`SigmaBuddyAllocator`) handling Order-N contiguous page allocations.
  - `MemoryDescriptorList` (MDL) pinning/unpinning physical pages for zero-copy DMA hardware transfers.
  - `DmaRingBufferAllocator` for PCIe NIC, NVMe, and audio hardware ring buffers.
* **SIMD Vectorized Memory Operations (`src/klib/isa.rs`, `src/klib/sigmalib.rs`)**:
  - Vectorized `memcpy` and `memset` operations automatically routed to AVX-512, AVX2, or NEON vector instructions based on hardware ISA level.
* **Userland Virtual Memory (UVM) (`src/klib/uvm.rs`)**:
  - UVM anonymous map (`amap`) slot allocation, physical page loaning (`uvm_page_loan`), and page table mapping (`pmap`).

---

## 2. Guidelines for Memory Operations Management

When modifying or implementing memory management routines:

### 1. Page Frame Alignment & Boundaries
* **Page Alignment**: Physical and virtual base addresses passed to page allocators or table mappers must be aligned to 4KB (`0x1000`), 2MB (`0x200000`), or 1GB (`0x40000000`) boundaries.
* **Huge Page Flags**: Mark 2MB and 1GB huge page table entries with the `PAGE_ENTRY_HUGE` bit (`0x80` on x86_64 Page Directory / PDPT entries).

### 2. Zero-Copy DMA & MDL Page Pinning Rules
* **Physical Pinning**: Always pin physical pages (`mdl.pin_pages()`) before passing addresses to hardware DMA controllers (NVMe, E1000, USB xHCI) to prevent PMM page reallocation during active DMA transfers.
* **Unpinning Guard**: Unpin MDL pages (`mdl.unpin_pages()`) immediately when DMA transfer completions or ring buffer teardowns occur.

### 3. SIMD Memory Copy Alignment
* **Vector Alignment**: SIMD `memcpy` fast-paths (AVX2/AVX-512) require 32-byte or 64-byte aligned pointers for optimal memory bus throughput; fall back to unaligned scalar loops for head/tail byte offsets.

---

## 3. Verification & Testing Protocols

1. **Memory & Paging Unit Tests**: Run core memory and paging unit tests:
   ```bash
   cargo test --lib memory klib::paging klib::isa
   ```
2. **Core System & Stress Test Runner**: Run the full test suite runner:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Memory Operation Changes

Before submitting memory operation or paging modifications:
- [ ] Confirmed 4KB/2MB/1GB boundary page alignment for physical/virtual addresses.
- [ ] Confirmed physical page pinning (MDL) for hardware DMA buffers.
- [ ] Verified SIMD `memcpy` alignment invariants and scalar fallback for tail bytes.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded memory operation learnings using `initiate_memory_recording`.
