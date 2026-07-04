// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Virtual Memory Manager (Rust, no_std)
//! Implements 4-level page table walking for x86_64
//! =========================================================================

use super::pmm::pmm_alloc_shard;

type U32 = u32;
type U64 = u64;
type Usize = usize;

const PAGE_SIZE: U64 = 4096;
const PAGE_MASK: U64 = 0xFFF;
const PML4_SHIFT: U64 = 39;
const PDPT_SHIFT: U64 = 30;
const PD_SHIFT: U64 = 21;
const PT_SHIFT: U64 = 12;

// Page table entry flags
const PTE_PRESENT: U64 = 1 << 0;
const PTE_WRITABLE: U64 = 1 << 1;
const PTE_USER: U64 = 1 << 2;
const PTE_NO_EXECUTE: U64 = 1 << 63;

pub struct SovereignVmm {
    pml4_base: U64,
    initialized: bool,
}

impl SovereignVmm {
    pub const fn new() -> Self {
        SovereignVmm {
            pml4_base: 0,
            initialized: false,
        }
    }

    pub unsafe fn init(&mut self) {
        if self.initialized { return; }
        
        // Allocate a top-level page directory (PML4 for x86_64)
        let page = pmm_alloc_shard();
        if !page.is_null() {
            self.pml4_base = page as U64;
            
            // Zero the page table
            let mut i: Usize = 0;
            while i < (PAGE_SIZE as Usize) {
                *(page.add(i)) = 0;
                i += 1;
            }
        }
        
        self.initialized = true;
    }

    unsafe fn get_or_create_table(&mut self, table_addr: U64, index: Usize, flags: U64) -> U64 {
        let table_ptr = table_addr as *mut U64;
        let entry = *table_ptr.add(index);
        
        if entry & PTE_PRESENT != 0 {
            // Table exists, return physical address
            entry & !PAGE_MASK
        } else {
            // Allocate new table
            let new_page = pmm_alloc_shard();
            if new_page.is_null() {
                return 0;
            }
            
            let new_phys = new_page as U64;
            
            // Zero the new table
            let mut i: Usize = 0;
            while i < (PAGE_SIZE as Usize) {
                *(new_page.add(i)) = 0;
                i += 1;
            }
            
            // Set entry with flags
            *table_ptr.add(index) = new_phys | flags | PTE_PRESENT | PTE_WRITABLE;
            
            new_phys
        }
    }

    pub unsafe fn map_page(&mut self, virt: U64, phys: U64, flags: U32) -> i32 {
        if !self.initialized || self.pml4_base == 0 { return -1; }
        
        // Align addresses to page boundaries
        let virt_aligned = virt & !PAGE_MASK;
        let phys_aligned = phys & !PAGE_MASK;
        
        // Extract indices for each level
        let pml4_index = ((virt_aligned >> PML4_SHIFT) & 0x1FF) as Usize;
        let pdpt_index = ((virt_aligned >> PDPT_SHIFT) & 0x1FF) as Usize;
        let pd_index = ((virt_aligned >> PD_SHIFT) & 0x1FF) as Usize;
        let pt_index = ((virt_aligned >> PT_SHIFT) & 0x1FF) as Usize;
        
        // Convert flags to PTE format
        let pte_flags: U64 = if (flags & 1) != 0 { PTE_WRITABLE } else { 0 } |
                              if (flags & 2) != 0 { PTE_USER } else { 0 };
        
        // Walk PML4 -> PDPT
        let pdpt_phys = self.get_or_create_table(self.pml4_base, pml4_index, 0);
        if pdpt_phys == 0 { return -1; }
        
        // Walk PDPT -> PD
        let pd_phys = self.get_or_create_table(pdpt_phys, pdpt_index, 0);
        if pd_phys == 0 { return -1; }
        
        // Walk PD -> PT
        let pt_phys = self.get_or_create_table(pd_phys, pd_index, 0);
        if pt_phys == 0 { return -1; }
        
        // Set final PTE
        let pt_ptr = pt_phys as *mut U64;
        let existing = *pt_ptr.add(pt_index);
        
        if existing & PTE_PRESENT != 0 {
            // Page already mapped
            return -2;
        }
        
        *pt_ptr.add(pt_index) = phys_aligned | pte_flags | PTE_PRESENT;
        
        // Invalidate TLB entry
        core::arch::asm!("invlpg ({})", in(reg) virt_aligned);
        
        0 // Success
    }

    pub unsafe fn unmap_page(&mut self, virt: U64) -> i32 {
        if !self.initialized || self.pml4_base == 0 { return -1; }
        
        let virt_aligned = virt & !PAGE_MASK;
        
        let pml4_index = ((virt_aligned >> PML4_SHIFT) & 0x1FF) as Usize;
        let pdpt_index = ((virt_aligned >> PDPT_SHIFT) & 0x1FF) as Usize;
        let pd_index = ((virt_aligned >> PD_SHIFT) & 0x1FF) as Usize;
        let pt_index = ((virt_aligned >> PT_SHIFT) & 0x1FF) as Usize;
        
        // Walk PML4
        let pml4_ptr = self.pml4_base as *mut U64;
        let pdpt_entry = *pml4_ptr.add(pml4_index);
        if pdpt_entry & PTE_PRESENT == 0 { return -1; }
        let pdpt_phys = pdpt_entry & !PAGE_MASK;
        
        // Walk PDPT
        let pdpt_ptr = pdpt_phys as *mut U64;
        let pd_entry = *pdpt_ptr.add(pdpt_index);
        if pd_entry & PTE_PRESENT == 0 { return -1; }
        let pd_phys = pd_entry & !PAGE_MASK;
        
        // Walk PD
        let pd_ptr = pd_phys as *mut U64;
        let pt_entry = *pd_ptr.add(pd_index);
        if pt_entry & PTE_PRESENT == 0 { return -1; }
        let pt_phys = pt_entry & !PAGE_MASK;
        
        // Clear PTE
        let pt_ptr = pt_phys as *mut U64;
        *pt_ptr.add(pt_index) = 0;
        
        // Invalidate TLB
        core::arch::asm!("invlpg ({})", in(reg) virt_aligned);
        
        0 // Success
    }
    
    pub unsafe fn get_physical_address(&self, virt: U64) -> U64 {
        if !self.initialized || self.pml4_base == 0 { return 0; }
        
        let virt_aligned = virt & !PAGE_MASK;
        let offset = virt & PAGE_MASK;
        
        let pml4_index = ((virt_aligned >> PML4_SHIFT) & 0x1FF) as Usize;
        let pdpt_index = ((virt_aligned >> PDPT_SHIFT) & 0x1FF) as Usize;
        let pd_index = ((virt_aligned >> PD_SHIFT) & 0x1FF) as Usize;
        let pt_index = ((virt_aligned >> PT_SHIFT) & 0x1FF) as Usize;
        
        let pml4_ptr = self.pml4_base as *const U64;
        let pdpt_entry = *pml4_ptr.add(pml4_index);
        if pdpt_entry & PTE_PRESENT == 0 { return 0; }
        let pdpt_phys = pdpt_entry & !PAGE_MASK;
        
        let pdpt_ptr = pdpt_phys as *const U64;
        let pd_entry = *pdpt_ptr.add(pdpt_index);
        if pd_entry & PTE_PRESENT == 0 { return 0; }
        let pd_phys = pd_entry & !PAGE_MASK;
        
        let pd_ptr = pd_phys as *const U64;
        let pt_entry = *pd_ptr.add(pd_index);
        if pt_entry & PTE_PRESENT == 0 { return 0; }
        let pt_phys = pt_entry & !PAGE_MASK;
        
        let pt_ptr = pt_phys as *const U64;
        let pte = *pt_ptr.add(pt_index);
        if pte & PTE_PRESENT == 0 { return 0; }
        
        (pte & !PAGE_MASK) | offset
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_VMM: SovereignVmm = SovereignVmm::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn vmm_init_shard() {
    G_VMM.init();
}

#[no_mangle]
pub unsafe extern "C" fn vmm_map_page(virt: U64, phys: U64, flags: U32) -> i32 {
    G_VMM.map_page(virt, phys, flags)
}

#[no_mangle]
pub unsafe extern "C" fn vmm_unmap_page(virt: U64) -> i32 {
    G_VMM.unmap_page(virt)
}
