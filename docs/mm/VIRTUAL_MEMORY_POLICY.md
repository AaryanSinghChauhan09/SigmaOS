# Virtual Memory Management Policy

## 1. Virtual Memory Area (VMA) Specifications

SigmaOS virtual memory management (`src/mm/virtual.rs`, `src/mm/virtual_memory.rs`) governs virtual address space mappings according to these requirements:

1. **User/Kernel Mapping Isolation**: User VMAs are strictly confined to lower-half canonical addresses (`< 0x8000_0000_0000`). Kernel VMAs reside exclusively in higher-half canonical space (`>= 0xFFFF_8000_0000_0000`).
2. **Mapping Modification Semantics**: Adding, resizing, or removing VMAs validates range overlap, page alignment, and canonical bounds before modifying page tables.
3. **Protection Semantics & W^X**: Enforces `READ`, `WRITE`, `EXECUTE`, and `USER` permission flags. No VMA may simultaneously possess `WRITE` and `EXECUTE` permissions.
4. **Swap Integration**: VMAs flagged as swappable can have anonymous pages paged out to swap storage under memory pressure, marking page table entries as swapped.
5. **Reclaim Semantics**: Inactive memory pages mapped by VMAs are reclaimed by `kswapd` using MGLRU page scanning order.
6. **OOM & Exhaustion Behavior**: Exceeding process VMA limits or global frame capacity returns `VirtualMemoryError::Exhausted` or triggers the OOM killer on the highest-badness process.
