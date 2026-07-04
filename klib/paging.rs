// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: paging - Virtual memory management (x86_64 4-level paging)
//! Hand-rolled zero-dependency implementation, no_std, no pre-defined libraries/functions
//! =========================================================================

#![no_std]

/// Page table entry
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    const PRESENT: u64 = 1 << 0;
    const WRITABLE: u64 = 1 << 1;
    const USER: u64 = 1 << 2;

    pub const fn new() -> Self {
        Self { entry: 0 }
    }

    pub fn set_address(&mut self, addr: u64) {
        self.entry = addr & 0x000f_ffff_ffff_f000;
    }

    pub fn get_address(&self) -> u64 {
        self.entry & 0x000f_ffff_ffff_f000
    }

    pub fn set_present(&mut self, present: bool) {
        if present {
            self.entry |= Self::PRESENT;
        } else {
            self.entry &= !Self::PRESENT;
        }
    }

    pub fn set_writable(&mut self, writable: bool) {
        if writable {
            self.entry |= Self::WRITABLE;
        } else {
            self.entry &= !Self::WRITABLE;
        }
    }

    pub fn set_user(&mut self, user: bool) {
        if user {
            self.entry |= Self::USER;
        } else {
            self.entry &= !Self::USER;
        }
    }
}

/// Page table (512 entries)
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); 512],
        }
    }

    pub fn get_entry(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }
}

/// Paging manager
pub struct PagingManager {
    pml4: *mut PageTable,
}

impl PagingManager {
    pub const fn new(pml4: *mut PageTable) -> Self {
        Self { pml4 }
    }

    pub fn map_page(&mut self, virtual_addr: u64, physical_addr: u64) {
        let pml4_idx = (virtual_addr >> 39) & 0x1FF;
        let pdp_idx = (virtual_addr >> 30) & 0x1FF;
        let pd_idx = (virtual_addr >> 21) & 0x1FF;
        let pt_idx = (virtual_addr >> 12) & 0x1FF;

        // For simplicity, we'll assume tables are pre-allocated
        // TODO: allocate tables on demand in a real implementation
        let pml4 = unsafe { &mut *self.pml4 };
        let pdpte = pml4.get_entry(pml4_idx as usize);
        // Assume pdp is already present
        let pdp_addr = pdpte.get_address();
        let pdp = unsafe { &mut *(pdp_addr as *mut PageTable) };
        let pde = pdp.get_entry(pdp_idx as usize);
        let pd_addr = pde.get_address();
        let pd = unsafe { &mut *(pd_addr as *mut PageTable) };
        let pte = pd.get_entry(pd_idx as usize);
        let pt_addr = pte.get_address();
        let pt = unsafe { &mut *(pt_addr as *mut PageTable) };
        let page_entry = pt.get_entry(pt_idx as usize);
        page_entry.set_address(physical_addr);
        page_entry.set_present(true);
        page_entry.set_writable(true);
    }
}
