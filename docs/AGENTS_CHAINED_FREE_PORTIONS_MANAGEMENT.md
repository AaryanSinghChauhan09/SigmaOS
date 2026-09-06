# AGENTS_CHAINED_FREE_PORTIONS_MANAGEMENT.md — AI Agent Chained Free List Memory Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, memory layout invariants, lock-free atomic routines, and verification protocols for managing, developing, and extending **Chained Free Portions, Intrusive Free Lists, Slab Freelists, and Boundary-Tag Memory Coalescing** in **SigmaOS**.

---

## 1. SigmaOS Chained Free Portions Architecture Overview

In low-level kernel memory management, freed memory blocks (portions) are intrusively chained together in linked lists or recycle bins without allocating external metadata overhead.

### Core Chained Free List Subsystems
* **Intrusive Slab Freelist Chaining (`src/klib/slab.rs`, `src/memory/slab.rs`)**:
  - `SlabCacheInner`: Lock-free $O(1)$ object allocation and deallocation using atomic `CompareAndSwap` (CAS) on the `free_head` index chain embedded directly inside unallocated object slots.
* **Recycle Bin & Heap Free Block Chains (`src/klib/custom_allocator.rs`)**:
  - Ring-buffer and intrusive linked free-list recycle bins reusing freed heap portions (`free(ptr, size)`) for subsequent allocations.
* **FreeBSD UMA & Zone Freelist Page Chains (`src/memory/zone.rs`)**:
  - Intrusive singly-linked page chains (`uma_zfree` / `uma_zalloc`) maintaining $O(1)$ CPU-local free object caches.
* **Buddy Allocator Order-N Free Lists (`src/klib/buddy_allocator.rs`, `src/memory/sigma_buddy.rs`)**:
  - Order-indexed free block arrays (`free_lists[order]`) tracking available power-of-two page frame blocks, performing coalescing on adjacent buddy blocks.

---

## 2. Guidelines for Chained Free Portions Management

When modifying or implementing intrusive free-list chaining algorithms:

### 1. Intrusive Node Layout & Alignment Rules
* **Minimum Slot Size**: Any freed memory portion must be at least large enough to store an intrusive pointer or `usize` index (e.g., 8 bytes on x86_64 / AArch64 / RISC-V 64).
* **Pointer Alignment**: Intrusive free-list next pointers embedded in freed slots must be aligned to 8-byte boundaries to prevent unaligned memory faults during atomic CAS operations.

### 2. Double-Free Prevention & ABA Mitigation
* **Double-Free Guarding**: In `SlabCacheInner::free` and `uma_zfree`, verify that the pointer being returned to the free chain does not already exist in the free list (preventing cyclic free list corruption).
* **ABA Generation Tagging**: Use atomic tagged pointers or versioned generation counters when pushing/popping nodes from lock-free atomic `free_head` chains to prevent the ABA problem in multithreaded allocation.

### 3. Boundary-Tag Coalescing & CoW Free Extents
* **Boundary-Tag Merging**: When returning contiguous free portions to heap or physical memory managers (`bitmap_pmm`), check adjacent left/right boundary tags to coalesce small free portions into larger contiguous free spans.

---

## 3. Verification & Testing Protocols

1. **Slab & Buddy Allocator Unit Tests**: Run core memory allocator unit tests:
   ```bash
   cargo test --lib klib::slab klib::buddy_allocator
   ```
2. **Core Memory & Fuzzing Inspection Tests**: Run the full test suite runner:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Chained Free Changes

Before submitting intrusive free-list or chained free portion modifications:
- [ ] Confirmed minimum slot size ($ \ge 8$ bytes) for embedded free-list pointers.
- [ ] Confirmed double-free detection to prevent cyclic free list corruption.
- [ ] Verified atomic CAS tagged pointers for lock-free `free_head` updates.
- [ ] Verified boundary-tag coalescing for contiguous free portions.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded chained free list learnings using `initiate_memory_recording`.
