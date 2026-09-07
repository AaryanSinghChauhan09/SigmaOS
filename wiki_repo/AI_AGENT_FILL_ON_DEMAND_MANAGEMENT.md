# AI Agent Fill-On-Demand Techniques & Operation Management Architecture in SigmaOS

This document specifies fill-on-demand virtual memory techniques, Copy-on-Write (CoW) page fault handling, Kernel Samepage Merging (KSM), and zram compressed memory swap-in/swap-out guidelines for AI agents working on memory management in SigmaOS (`src/memory/paging.rs`).

---

## ⚡ 1. Fill-On-Demand & Virtual Memory Area Subsystem

SigmaOS implements lazy fill-on-demand memory allocation inspired by Linux VMAs and BSD VM spaces:

```
+---------------------------------------------------------------------------------+
| Address Lookup (`get_physical_address_with_access`)                            |
| Resolves VirtualAddress through PML4 -> PDPT -> PD -> PT page tables.           |
+---------------------------------------------------------------------------------+
                                       |
                   [Page Table Entry Not Present Fault]
                                       v
+---------------------------------------------------------------------------------+
| Fill-On-Demand Resolution (`attempt_demand_paging`)                             |
| 1. Checks `zram_pool` compressed pages -> Decompresses & restores page frame.   |
| 2. Checks `vmas` (`VirtualMemoryArea`) -> Allocates physical frame on-demand.   |
+---------------------------------------------------------------------------------+
                                       |
                          [Write Intent on Read-Only KSM]
                                       v
+---------------------------------------------------------------------------------+
| Copy-On-Write (CoW) Page Fault Handler                                           |
| Clones shared page to unique writable physical frame (`unique_phys`).           |
+---------------------------------------------------------------------------------+
```

---

## ⚙️ 2. Core On-Demand Concepts & Mechanics

1. **Virtual Memory Area Registration (`VirtualMemoryArea`)**
   - Regions of virtual address space registered via `register_vma`.
   - On page faults within a registered VMA, physical pages are frame-allocated on-demand (`attempt_demand_paging`).
2. **Copy-on-Write (CoW) Page Splitting**
   - Write access to KSM-merged read-only pages triggers CoW splitting in `get_physical_address_with_access`.
   - Allocates a unique physical frame, clears `is_ksm_shared`, and sets `writable = true`.
3. **zram Compressed Memory Swap**
   - Page eviction via Clock algorithm (`perform_clock_replacement_step`) compresses page frame contents into `zram_pool`.
   - Subsequent memory access triggers zram decompression restore on-demand.

---

## 🛡️ 3. Rules & Directives for AI Agents

1. **VMA Range Boundaries**
   - Verify `vma.contains(virt.0)` before attempting fill-on-demand allocation.
2. **Copy-On-Write Protection**
   - Always validate `vma.is_writable` and `vma.is_executable` during page fault resolution to prevent unauthorized write or execution faults (`MemoryError::WriteToReadOnly` / `MemoryError::NonExecutablePage`).
3. **KSM Registry Cleanups**
   - Remove virtual address entries from `ksm_registry` when CoW splitting occurs to prevent invalid reference tracking.

---

## ⚙️ 4. Verification Commands for Memory Agents

- **Paging Subsystem Unit Tests:**
  `cargo test --lib -- memory::paging::tests`
- **Full SigmaOS Test Pipeline:**
  `./run_sigma_tests.sh`
