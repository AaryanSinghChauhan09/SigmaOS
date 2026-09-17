# SigmaOS AI Agent Bitmap Operations Management Directive (`AGENTS_BITMAP_OPERATIONS.md`)

This document defines technical directives, synchronization rules, and operational guidelines for AI agents managing bitmap operations in SigmaOS.

---

## 1. Core Principles for Bitmap Management

Bitmaps in SigmaOS are used across critical kernel and memory subsystems (physical page frame allocators, PID allocation, interrupt vector tracking, and VM dirty page logging). AI agents modifying bitmap routines must observe the following directives:

1. **Lock-Free Atomic Bit Manipulation (`AtomicBitmap`):**
   - Resource allocation and bit flipping must use atomic bitwise operations (`AtomicU64` or `AtomicUsize`) with explicit memory ordering (`Ordering::SeqCst` or `Ordering::AcqRel`).
   - Bit allocation methods (`set_bit`, `clear_bit`, `test_bit`, `alloc_one`) must avoid data races without resorting to blocking mutex locks in atomic contexts.

2. **Bounds Safety & Word Alignment:**
   - Always validate bit offsets against total capacity before calculating array/word indices (`bit_offset / 64` or `bit_offset / 8`).
   - Handle array edge cases and word boundaries safely to prevent buffer overruns or unaligned memory access.

3. **Zero-Dependency `#![no_std]` Implementation:**
   - Use native `AtomicBitmap` or `klib::bitmap` abstractions rather than external bitset crates.
   - For static system allocations (PIDs, IRQs, Page Frames), prefer const-generic fixed arrays over dynamic allocations.

4. **Resource Accounting & Population Count:**
   - Use hardware-accelerated population counting (`count_ones()`, `count_zeros()`, `trailing_zeros()`) for fast free-slot searching (`find_first_zero`).
   - Maintain strict synchronization between total allocated bit count and system resource tracking statistics.

---

## 2. Pre-Commit Bitmap Verification Checklist

AI agents making changes to bitmap structures must verify:
- [ ] Bit manipulation methods safely handle out-of-bounds bit offsets.
- [ ] Concurrent bit modifications on `AtomicBitmap` pass atomic test suite.
- [ ] Memory alignment and word-boundary bitwise operations operate correctly across 32-bit and 64-bit architectures.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
