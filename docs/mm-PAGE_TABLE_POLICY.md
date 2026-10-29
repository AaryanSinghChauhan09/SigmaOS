# Page-Table Manager Policy (PAGE_TABLE_POLICY.md)

## Requirements & Standards

### 1. Page-Table Invariants
- 4-level x86_64 / ARM64 page translation structures (PML4 -> PDPT -> PD -> PT).
- Kernel space mappings occupy higher canonical half addresses and are present in every process page directory with `PTE_SUPERVISOR` privileges.
- User space mappings occupy lower canonical half addresses with `PTE_USER` flags.

### 2. Race-Free Mapping & Lost-Update Prevention
- Modifications to page tables require holding the inner `AddressSpace` lock.
- Updates to existing mapping entries invalidate TLB caches locally and issue inter-processor interrupt (IPI) TLB shootdowns to remote cores.
- Atomicity is enforced for page table walk and entry updates.

### 3. Page-Table Capacity & Bounded Structures
- Page table page allocations are tracked against process virtual memory quotas.
- Total kernel virtual memory allocated for page table structures is strictly bounded.

### 4. Double-Free & Orphan Mapping Prevention
- Unmapping a page checks whether the underlying physical frame reference count reaches zero before returning the frame to the physical allocator.
- Destroying an address space recursively frees child page tables and releases referenced frames without leaks or double-frees.

### 5. Safe Address-Space Switching
- Address-space context switches set CR3 / TTBR0 atomically.
- PCID (Process Context ID) or ASID (Address Space ID) tagging is used where hardware support exists to prevent unnecessary TLB flushes on context switch.
