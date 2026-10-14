# Page Table & Paging Policy Specification

`docs/mm/PAGE_TABLE_POLICY.md`

## Overview
The Page Table Manager (`src/mm/paging.rs` and `src/mm/page_table.rs`) controls 4-level x86_64 hardware page table indexing (PML4 -> PDPT -> Page Directory -> Page Table) and virtual-to-physical address translation in SigmaOS.

## Invariants & Rules

1. **Explicit Invariants:**
   - Physical page table frames are allocated through `PhysicalMemoryManager`.
   - Virtual addresses must be in canonical form (sign-extended bit 47). Non-canonical addresses trigger `PagingError::NonCanonicalAddress`.
2. **Race-Safe Mapping:**
   - Page table modifications take a fine-grained address-space spinlock.
   - Core page table updates emit TLB shootdowns across SMP cores via Inter-Processor Interrupts (IPIs) when modifying active address spaces.
3. **Bounded Capacity:**
   - Page table allocations are bounded by address space size limits and system frame availability.
   - Exceeding page table capacity returns `PagingError::CapacityExceeded`.
4. **Lost-Update Prevention:**
   - Page table entry (PTE) modifications use atomic read-modify-write operations or spinlock protection.
   - `tlb_flush_required` flags track pending TLB invalidations to prevent stale translations.
5. **Double-Free & Unmap Protection:**
   - Unmapping an unmapped virtual address returns `PagingError::NotMapped` and does not decrease physical page reference counts twice.
6. **Address Space Switch Safety:**
   - CR3 updates reload the root PML4 table address safely, ensuring PCID (Process Context ID) invalidation rules are observed when supported.
