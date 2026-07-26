# 🔧 Virtual Memory & Paging Implementation

## Overview

SigmaOS implements a complete **4-level paging architecture** (PML4 → PDPT → PD → PT) consistent with the x86-64 IA-32e long mode standard. All virtual address translation is handled by `src/memory/pmm_vmm.rs`.

---

## Architecture: 4-Level Paging

x86-64 virtual addresses are 48 bits wide, split across four page table levels:

```
Virtual Address [63:0]:
  [63:48] Sign extension
  [47:39] PML4 index   (9 bits → 512 entries)
  [38:30] PDPT index   (9 bits → 512 entries)
  [29:21] PD index     (9 bits → 512 entries)
  [20:12] PT index     (9 bits → 512 entries)
  [11:0]  Page offset  (12 bits → 4096 bytes)
```

### SigmaOS Page Table Hierarchy

```rust
pub struct SimpleVMM {
    pub pml4_table: Vec<Option<Box<PageDirectoryPointerTable>>>,
    pub pmm: SimplePMM,
}

pub struct PageDirectoryPointerTable {
    pub entries: Vec<Option<Box<PageDirectory>>>,
}

pub struct PageDirectory {
    pub entries: Vec<Option<Box<PageTable>>>,
}

pub struct PageTable {
    pub entries: Vec<Option<PageTableEntry>>,
}
```

---

## Physical Memory Manager (PMM)

The PMM uses a **bitmap allocator** backed by 1024 `AtomicUsize` words (supporting 65,536 4KB pages = 256 MB):

```rust
pub struct SimplePMM {
    pub page_bitmap: [AtomicUsize; 1024],
    pub total_pages: AtomicUsize,
    pub free_pages: AtomicUsize,
}
```

### Page Allocation Flow

1. Scan bitmap words for a word that is not `usize::MAX`
2. Find the first free bit within that word using bit-scan
3. Atomically set the bit using `fetch_or`
4. Decrement `free_pages` counter
5. Return `page_num * 4096` as the physical address

---

## Virtual Memory Manager (VMM)

The VMM translates virtual addresses to physical frames and manages the 4-level page table:

```rust
impl VirtualMemoryManager for SimpleVMM {
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress) -> Result<(), MemoryError> {
        let pml4_idx = (virt >> 39) & 0x1FF;
        let pdpt_idx = (virt >> 30) & 0x1FF;
        let pd_idx   = (virt >> 21) & 0x1FF;
        let pt_idx   = (virt >> 12) & 0x1FF;
        // Walk/create page table levels, install PTE
        Ok(())
    }
}
```

### Page Table Entry Flags

| Flag | Bit | Description |
|------|-----|-------------|
| Present | 0 | Page is in physical memory |
| Writable | 1 | Page can be written |
| User-Accessible | 2 | Accessible from user-mode |
| Physical Addr | [63:12] | Frame base address |

---

## Memory Verification

The `MemoryVerifier` trait provides formal verification hooks:

```rust
pub trait MemoryVerifier {
    fn verify_allocation(&self, addr: PhysicalAddress) -> Result<bool, MemoryError>;
    fn verify_mapping(&self, virt: VirtualAddress) -> Result<bool, MemoryError>;
}
```

---

## Integration with Kernel

The PMM/VMM is consumed by the kernel boot path (UEFI phase) to set up the initial address space before handing off to the Rust `_start` entry point.

---

## Tests

```
test memory::pmm_vmm::tests::test_pmm_allocation ... ok
test memory::pmm_vmm::tests::test_vmm_mapping ... ok
```
