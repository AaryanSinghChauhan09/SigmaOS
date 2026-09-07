# AI Agent Guidelines: Clock Page Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Clock Page Replacement Management**, Second-Chance page eviction tracking, virtual memory reclamation, access bit reset, and zram/zswap compressed swap integration in SigmaOS.

SigmaOS utilizes a Clock (Second-Chance) replacement algorithm (`SimpleVMM` in `src/memory/paging.rs`) to maintain high physical page frame utilization while preventing thrashing and unnecessary disk I/O.

---

## 1. Clock Page Replacement Subsystems

AI agents interacting with page replacement in SigmaOS must interface with the following virtual memory subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Virtual Memory Manager (`SimpleVMM`)** | `src/memory/paging.rs` | 4-level paging VMM tracking active pages (`active_pages_for_clock`) and clock pointer traversal (`clock_hand`). |
| **Page Table Entry (`PageTableEntry`)** | `src/memory/paging.rs` | 4KB/2MB/1GB page table entries containing `accessed` (A) and `dirty` (D) hardware bit flags. |
| **zram Compressed Swap (`ZramPage`)** | `src/memory/paging.rs` | In-memory compressed swap pool receiving evicted pages during clock replacement. |
| **Demand Paging VMA (`VirtualMemoryArea`)** | `src/memory/paging.rs` | Region descriptors enabling lazy allocation and zram swap-in fault recovery. |

---

## 2. Clock Page Replacement Workflow

The Clock replacement algorithm operates as a circular queue traversing mapped virtual pages:

```
                  +-------------------------------+
                  |  1. Advance Clock Hand        |
                  |  (clock_hand % len)           |
                  +-------------------------------+
                                  |
                                  v
                  +-------------------------------+
                  |  2. Inspect Accessed Bit (A)   |
                  +-------------------------------+
                     /                         \
         A = true   /                           \  A = false
                   v                             v
  +-------------------------------+   +-------------------------------+
  | Reset Accessed Bit (A = false)|   | Evict Page & Compress to zram |
  | Give "Second Chance"          |   | Unmap Page Table Entry        |
  +-------------------------------+   +-------------------------------+
                   |                             |
                   +-----------> Loop <----------+
```

### Key Rules for Clock Traversal
1. **Accessed Bit Check:** If `pte.accessed` is `true`, reset `pte.accessed = false` ("second chance") and increment `clock_hand`.
2. **Page Eviction:** If `pte.accessed` is `false`, evict the page from `active_pages_for_clock`, compress its contents into `zram_pool`, and clear the page table entry (`pd.entries[pt_idx] = None`).
3. **Loop Bound:** Traverse at most `2 * active_pages_for_clock.len()` steps to guarantee eviction or exit when all pages have had their accessed bits cleared.

```rust
// Standard Clock replacement execution in SigmaOS SimpleVMM
pub fn perform_clock_replacement_step(&mut self) -> Option<VirtualAddress> {
    if self.active_pages_for_clock.is_empty() {
        return None;
    }
    // Traverses circular active page list, inspecting and clearing accessed bits
    // ...
}
```

---

## 3. zram Compressed Swap Integration

When a page is evicted by the Clock algorithm:
- The raw page frame data is compressed into a `ZramPage` block and pushed to `zram_pool`.
- If the evicted page is subsequently accessed by a process, `get_physical_address_with_access()` intercepts the page fault, decompresses the page from `zram_pool`, and re-maps it into the 4-level page table.

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes to virtual memory or page replacement logic:

- [ ] Does page mapping automatically append new virtual pages to `active_pages_for_clock`?
- [ ] Does page unmapping (`unmap_page`) clean up references in `active_pages_for_clock`?
- [ ] Is the accessed bit properly reset during second-chance traversal?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
