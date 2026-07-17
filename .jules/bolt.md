# ⚡ Bolt's Performance Optimization Journal

This journal chronicles performance-obsessed engineering insights, breakthroughs, and critical learnings encountered during the optimization of the SigmaOS microkernel.

---

## 2026-03-02 - Buddy Allocator Merging Double-Moves
**Learning:** Consuming the `MemoryBlock` struct during an unsuccessful buddy allocator merge attempt causes dynamic Rust compiler ownership move violations because the block doesn't implement `Copy`.
**Action:** Return the unmerged `MemoryBlock` back in the `Err` variant of a `Result<MemoryBlock, MemoryBlock>` so the deallocation queue can cleanly push it to the appropriate free list without violating ownership invariants.

## 2026-03-01 - Round-Robin Scheduling Cycle Boundaries
**Learning:** A simple modulo-based tick count can cycle the current index back to the initial index when the ticks match a multiple of the process count times the time slice.
**Action:** When testing time slice rotation, tick the scheduler for a fraction of the full cycle (e.g., 15 ticks instead of 20 for 2 processes) to verify index shifts without wrapping around to 0.

## 2026-02-28 - Host-Based `_start` Symbol Clashes
**Learning:** Defining `pub extern "C" fn _start()` in `#![no_std]` binary targets results in duplicate symbol linker errors when running `cargo test` on developer hosts linking standard glibc libraries.
**Action:** Use conditional attributes `#![cfg_attr(not(any(target_os = "linux", target_os = "windows", target_os = "macos", test)), no_std)]` and conditionally rename the entry point to `main` on developer host platforms.
