# Page Table Manager Policy

## Scope

This document specifies x86_64 multi-level page table tree structures, entry flags, address canonicality checks, and mapping integrity rules for SigmaOS.

## Principles

1. **Canonical Address Validation**: All virtual addresses passed to paging functions are checked against x86_64 canonical address rules (bits 47 through 63 must match bit 47). Non-canonical addresses trigger `PagingError::NonCanonicalAddress`.
2. **Bounded Capacity**: The page table manager maintains a strict upper bound on allocated page table nodes. Exceeding this limit returns `PagingError::CapacityExceeded`.
3. **Atomic Entry Updates & Lost-Update Prevention**: Page table entries (PML4E, PDPTE, PDE, PTE) update present and flag bits atomically to prevent race conditions during concurrent page faults or TLB shootdowns.
4. **Protection Semantics**: Page flags strictly distinguish User vs. Kernel privileges (`user_accessible`), Write privileges (`writable`), and Execution permissions (`no_execute`).
