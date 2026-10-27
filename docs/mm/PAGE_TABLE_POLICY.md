# SigmaOS Page Table & Paging Policy

## Overview

The paging subsystem (`src/mm/paging.rs`, `src/mm/page_table.rs`) handles 4-level x86_64 page table translation, page entry mapping/unmapping, canonical address space checks, and address space switching for SigmaOS.

---

## Core Guarantees & Invariants

1. **Page Table Invariants**: All virtual addresses must satisfy x86_64 48-bit canonical address specifications (bits 47-63 sign-extended). Non-canonical virtual addresses return `PagingError::NonCanonicalAddress`.
2. **Race-Free Mapping**: Mutating page directory entries or page table entries is synchronized or thread-safe, ensuring concurrent map/unmap operations do not introduce corrupted translation structures.
3. **Bounded Page-Table Capacity**: Total mapped virtual pages are tracked and bounded per process address space. Attempts to map beyond the maximum page table capacity return `PagingError::CapacityExceeded`.
4. **Lost-Update Prevention**: Updating existing page table mappings (e.g., flag updates or remapping) flushes TLB entries via explicit TLB shootdown hooks and atomically updates page table structures to prevent lost updates.
5. **Double-Free Prevention**: Unmapping an unmapped virtual page or freeing page table structures twice returns an error (`PagingError::NotMapped` / `PagingError::AlreadyFreed`) rather than corrupting memory pools.
6. **Safe Address Space Switching**: CR3 root page directory switches ensure context updates preserve kernel higher-half mappings and prevent execution of invalid user address spaces.

---

## Implementation Specifications

- **Source Files**: `src/mm/paging.rs`, `src/mm/page_table.rs`
- **Unit & Concurrency Tests**: `tests/mm/paging.rs`
- **Paging Architecture**: 4-Level Paging (PML4 -> PDPT -> PD -> PT)

---

## Enforcement & Verification

- `cargo check --lib`
- `rustc --test --edition=2021 tests/mm/paging.rs -o build/test_paging && ./build/test_paging`
