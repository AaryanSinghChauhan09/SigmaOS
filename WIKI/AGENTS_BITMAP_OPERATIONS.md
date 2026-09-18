# AI Agent Bitmap Operations Management Architecture (`docs/AGENTS_BITMAP_OPERATIONS.md`)

This guide details the technical architecture, memory layout, and AI agent operational protocols for bitmap structures in SigmaOS.

---

## 1. Subsystem Architecture & Usage

Bitmaps serve as primary bitset allocators and status tracking structures in SigmaOS:

### A. Atomic Bitmap Allocator (`AtomicBitmap`)
- Defined in `src/klib/bitmap.rs` and `src/kernel/atomic_extended.rs`.
- Employs atomic word arrays (`AtomicU64`) to track allocation states for physical page frames, PIDs (Process IDs), and IRQ (Interrupt Request) vectors without mutex contention.
- Implements `alloc_one()`, `set()`, `clear()`, `test()`, `count_ones()`, and `count_zeros()` with atomic memory orderings.

### B. Physical Memory Manager Bitmaps
- Physical memory managers parse boot memory maps and track frame availability using bitmap arrays. Free pages are represented by `0` and allocated pages by `1`.
- Sequential page allocations utilize contiguous zero-bit scans (`find_next_zero_area`).

### C. Hypervisor Dirty Page Logging
- Virtual machine guest memory tracking uses dirty bitmaps to log modified guest physical pages during live migration and checkpointing.

---

## 2. AI Agent Operational Guidelines

1. **Race-Condition Audit:** Ensure bit set/clear loops evaluate returns correctly (e.g., `set()` returning `false` if the bit was already set).
2. **Architecture Portability:** Verify bit shifts (`1 << (offset % 64)`) operate on 64-bit integer types to prevent truncation on 32-bit platforms.
3. **Automated Testing:** Run `./run_sigma_tests.sh` to validate atomic bitmap unit tests.
