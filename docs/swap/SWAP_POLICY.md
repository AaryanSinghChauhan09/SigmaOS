# SigmaOS Swap Subsystem Policy

## Overview

The swap manager (`src/swap/core.rs`, `src/swap/enqueue.rs`) provides secondary block-backed and zram/zswap memory swapping for cold anonymous pages in SigmaOS.

---

## Core Guarantees & Invariants

1. **Race-Free Swap-Out**: Evicting pages to swap slots is thread-safe and atomic. Duplicate swap-out requests for an active page are prevented.
2. **Race-Free Swap-In**: Page fault handling reading back swapped pages is race-free; concurrent swap-in requests for the same swap slot synchronize on the slot descriptor state.
3. **Bounded Swap Metadata**: Total swap slot tracking bitmaps, slot priorities, and block mappings use bounded metadata structures allocated at swap device registration time.
4. **Lost-Swap Prevention**: Active swap slots retain validity flags until page table entries are fully updated and flushed, ensuring swapped data is never lost or orphaned.
5. **Double-Swap Prevention**: Attempting to free an unallocated swap slot or double-allocate a swap index returns an explicit error (`SwapError::InvalidSlot` / `SwapError::AlreadyAllocated`).
6. **Observable OOM Enforcement**: Complete exhaustion of physical frames and swap slot capacity triggers deterministic OOM score evaluation and returns `SwapError::OutOfMemory`, preventing unbounded blocks or kernel panics.

---

## Implementation Specifications

- **Source Implementation**: `src/swap/core.rs`, `src/swap/enqueue.rs`
- **Unit & Priority Queue Tests**: `tests/swap/core.rs`

---

## Enforcement & Verification

- `cargo check --lib`
- `rustc --test --edition=2021 tests/swap/core.rs -o build/test_swap && ./build/test_swap`
