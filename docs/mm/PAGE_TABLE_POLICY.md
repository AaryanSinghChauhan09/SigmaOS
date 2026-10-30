# Page Table & Paging Policy

## Architectural Invariants
1. **Canonical Address Validation**: Addresses in x86_64 must be in canonical form (`0x0000_0000_0000_0000..=0x0000_7FFF_FFFF_FFFF` for user or `0xFFFF_8000_0000_0000..=0xFFFF_FFFF_FFFF_FFFF` for kernel). Non-canonical addresses trigger an explicit `PagingError::NonCanonicalAddress`.
2. **Race-Free Mapping Operations**: Modifications to page tables (map, unmap, protect) are protected by page-table locks or lock-free atomic CAS primitives with synchronized TLB shootdowns across CPUs.
3. **Bounded Page Table Capacity**: Page table nodes are allocated from bounded frame reserves to prevent unconstrained page table growth under malicious nested mapping attempts.
4. **Lost-Update Prevention**: Concurrent page fault resolution or page unmapping operations hold the target page table entry lock to prevent updates from being overwritten.
5. **Double-Free Prevention**: Unmapping a page entry verifies the present bit (`P=1`); unmapping an already absent entry is a no-op or returns `PagingError::NotMapped`.
6. **Safe Address Space Switch**: Context switching updates `CR3` only when switching to a different address space, flushing non-global TLB entries safely.
