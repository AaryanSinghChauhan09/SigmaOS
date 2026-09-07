# SigmaOS AI Agent Memory Management Specification

This specification details memory management guidelines, buddy/slab allocator invariants, and zero-leak memory rules for AI autonomous engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Physical & Virtual Memory Management (PMM / VMM)
- **Bitmap PMM Allocator (`src/memory/pmm_vmm.rs`)**:
  - Manages physical frame tracking using bitmap bitsets.
  - All physical allocations must check frame bounds and alignment before returning physical base addresses.
- **Virtual Memory Manager (VMM)**:
  - Maps virtual address spaces with page tables across x86_64 (PML4), ARM64 (TTBR0), and RISC-V (satp).
  - Enforces Copy-On-Write (CoW) page flags and hardened guard pages on stack and heap allocations.

## 2. Kernel & Userland Allocators
- **Buddy Allocator (`src/memory/buddy_allocator.rs`, `src/klib/buddy_allocator.rs`)**:
  - Power-of-two memory block splitting and coalescing.
  - Order calculation must enforce lower and upper bounds (Order 0 through Order 10).
- **Custom Slab & Zone Allocator (`src/klib/custom_allocator.rs`, `src/klib/slab.rs`)**:
  - Caches fixed-size objects (inodes, IPC message buffers, task descriptors) to avoid fragmentation.
  - Recycles deallocated blocks via lock-free recycle bins with ASLR guard pages.

## 3. AI Agent Memory Rules & Best Practices
1. **Never Call External C Allocators**:
   - AI agents must not introduce `libc::malloc` or external allocator dependencies in `#![no_std]` core crates.
2. **Always Verify Deallocations**:
   - Every allocation path introduced by an AI agent must have a matching deallocation/drop implementation.
3. **Validate Memory Boundaries**:
   - DMA ring buffers and MMIO page faults must validate faulting addresses against `0` (NULL pointer) and physical bounds before handling faults (`src/hal/multi_arch.rs`).
4. **Automated Testing & Regression Verification**:
   - Any modifications to memory management modules must pass both standalone unit tests (`rustc --test`) and `./run_sigma_tests.sh`.
