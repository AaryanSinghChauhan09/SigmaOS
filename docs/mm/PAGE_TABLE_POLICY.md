# Paging and Page-Table Manager Policy (`PAGE_TABLE_POLICY.md`)

## 1. Overview & Objectives
The Paging and Page Table Manager (`PagingManager` in `src/mm/paging.rs` and `MultiLevelPageTableTree` in `src/mm/page_table.rs`) governs x86_64 4-level page table navigation, mapping, unmapping, CR3 reload, and TLB invalidation.

Key Requirements:
- **Canonical Address Invariants**: Addresses must pass x86_64 sign-extension checks (`is_canonical`).
- **Race-Safe Mappings**: Page table entries are modified under lock with atomic TLB flush notification tracking (`tlb_flush_required`).
- **Bounded Page Capacity**: Maximum mapped pages clamped via capacity configuration (`max_capacity`).
- **CR3 Context Switching**: `reload_cr3()` validates physical root CR3 and invalidates TLB caches.
- **Double-Free & Lost-Update Prevention**: Re-mapping an already mapped page yields `PagingError::AlreadyMapped`, and unmapping an unmapped page yields `PagingError::NotMapped`.

---

## 2. Page Table Invariants

1. **Canonical Enforcement**: Bits 47 through 63 of virtual addresses must be identical sign extensions. Non-canonical addresses return `PagingError::NonCanonicalAddress`.
2. **Double-Mapping Prevention**: Mapping a virtual page that is already present returns `PagingError::AlreadyMapped(virt_addr)`.
3. **Double-Unmapping Prevention**: Unmapping a page that is not present returns `PagingError::NotMapped(virt_addr)`.
4. **Capacity Limits**: Total mapped page count cannot exceed `max_capacity`. Exceeding limit returns `PagingError::CapacityExceeded`.
5. **TLB Coherency**: Every mapping update or unmap triggers `tlb_flush_required.store(true, Ordering::SeqCst)`.

---

## 3. Implementation Verification
Verified by unit tests in `tests/mm/paging.rs`:
- `test_paging_canonical_address_validation`
- `test_paging_map_unmap_basic`
- `test_paging_capacity_exceeded`
- `test_paging_non_canonical_error`
- `test_paging_cr3_reload`
