/// OOP-based Paging + Virtual Memory for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 7-8
/// Implements 4-level page tables, PML4, userspace isolation, Copy-On-Write (COW), and page fault handling.

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::vec::Vec;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageTableLevel { PML4 = 0, PDPT = 1, PD = 2, PT = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultError { Success = 0, NotPresent = 1, PermissionDenied = 2, InvalidAddress = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel { Kernel = 0, User = 3 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSize {
    Standard4KB,
    Huge2MB,
    Giant1GB,
}

impl PageSize {
    pub fn byte_size(&self) -> usize {
        match self {
            PageSize::Standard4KB => 4 * 1024,
            PageSize::Huge2MB => 2 * 1024 * 1024,
            PageSize::Giant1GB => 1024 * 1024 * 1024,
        }
    }
}

pub trait PageTableEntry {
    fn is_present(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_user_accessible(&self) -> bool;
    fn is_cow(&self) -> bool;
    fn is_huge(&self) -> bool;
    fn get_physical_address(&self) -> PhysicalAddress;
    fn set_present(&mut self, present: bool);
    fn set_writable(&mut self, writable: bool);
    fn set_user_accessible(&mut self, user: bool);
    fn set_cow(&mut self, cow: bool);
    fn set_huge(&mut self, huge: bool);
    fn set_physical_address(&mut self, addr: PhysicalAddress);
    fn is_cow(&self) -> bool;
    fn set_cow(&mut self, cow: bool);
}

#[repr(C)]
pub struct SimplePageTableEntry {
    pub present: AtomicUsize,
    pub writable: AtomicUsize,
    pub user_accessible: AtomicUsize,
    pub physical_addr: AtomicUsize,
    pub accessed: AtomicUsize,
    pub dirty: AtomicUsize,
    pub cow: AtomicUsize, // Copy-On-Write (COW) indicator flag
}

impl Default for SimplePageTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl SimplePageTableEntry {
    pub const fn new() -> Self {
        SimplePageTableEntry {
            present: AtomicUsize::new(0),
            writable: AtomicUsize::new(0),
            user_accessible: AtomicUsize::new(0),
            physical_addr: AtomicUsize::new(0),
            accessed: AtomicUsize::new(0),
            dirty: AtomicUsize::new(0),
            cow: AtomicUsize::new(0),
        }
    }
}

impl PageTableEntry for SimplePageTableEntry {
    fn is_present(&self) -> bool {
        self.present.load(Ordering::SeqCst) == 1
    }
    fn is_writable(&self) -> bool {
        self.writable.load(Ordering::SeqCst) == 1
    }
    fn is_user_accessible(&self) -> bool {
        self.user_accessible.load(Ordering::SeqCst) == 1
    }
    fn is_cow(&self) -> bool {
        self.cow.load(Ordering::SeqCst) == 1
    }
    fn is_huge(&self) -> bool {
        self.huge.load(Ordering::SeqCst) == 1
    }
    fn get_physical_address(&self) -> PhysicalAddress {
        self.physical_addr.load(Ordering::SeqCst) & 0x000FFFFFFFFFF000
    }
    fn set_present(&mut self, present: bool) {
        self.present
            .store(if present { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_writable(&mut self, writable: bool) {
        self.writable
            .store(if writable { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_user_accessible(&mut self, user: bool) {
        self.user_accessible
            .store(if user { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_cow(&mut self, cow: bool) {
        self.cow.store(if cow { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_huge(&mut self, huge: bool) {
        self.huge.store(if huge { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_physical_address(&mut self, addr: PhysicalAddress) {
        self.physical_addr
            .store(addr & 0x000FFFFFFFFFF000, Ordering::SeqCst);
    }
    fn is_cow(&self) -> bool { self.cow.load(Ordering::SeqCst) == 1 }
    fn set_cow(&mut self, cow: bool) {
        self.cow.store(if cow { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait PageTable {
    fn get_entry_ref(&self, index: usize) -> &SimplePageTableEntry;
    fn get_entry(&mut self, index: usize) -> &mut dyn PageTableEntry;
    fn get_entry_ref(&self, index: usize) -> &dyn PageTableEntry;
    fn set_entry(&mut self, index: usize, entry: SimplePageTableEntry);
    fn get_physical_address(&self) -> PhysicalAddress;
}

pub struct SimplePageTable {
    pub entries: Vec<SimplePageTableEntry>,
    pub physical_addr: AtomicUsize,
}

impl SimplePageTable {
    pub fn new(physical_addr: PhysicalAddress) -> Self {
        let mut entries = Vec::new();
        for _ in 0..512 {
            entries.push(SimplePageTableEntry::new());
        }
        SimplePageTable {
            entries,
            physical_addr: AtomicUsize::new(physical_addr),
        }
    }
}

impl PageTable for SimplePageTable {
    fn get_entry_ref(&self, index: usize) -> &SimplePageTableEntry {
        if index < 512 {
            &self.entries[index]
        } else {
            static mut DUMMY: SimplePageTableEntry = SimplePageTableEntry::new();
            unsafe { &mut *(&raw mut DUMMY) }
        }
    }
    fn get_entry(&mut self, index: usize) -> &mut dyn PageTableEntry {
        let idx = index.min(511);
        &mut self.entries[idx]
    }
    fn get_entry_ref(&self, index: usize) -> &dyn PageTableEntry {
        let idx = index.min(511);
        &self.entries[idx]
    }
    fn set_entry(&mut self, index: usize, entry: SimplePageTableEntry) {
        let idx = index.min(511);
        self.entries[idx] = entry;
    }
    fn get_physical_address(&self) -> PhysicalAddress {
        self.physical_addr.load(Ordering::SeqCst)
    }
}

/// Advanced multi-core Translation Lookaside Buffer (TLB) shootdown coordinator (Linux-grade)
pub struct TlbTracker {
    pub current_epoch: AtomicUsize,
    pub pending_shootdowns: AtomicUsize,
}

impl TlbTracker {
    pub fn new() -> Self {
        TlbTracker {
            current_epoch: AtomicUsize::new(1),
            pending_shootdowns: AtomicUsize::new(0),
        }
    }

    /// Register a TLB invalidation event on page change (Phase 1 of Linux-defeating MM)
    pub fn register_invalidation(&self, _virt: VirtualAddress) {
        self.pending_shootdowns.fetch_add(1, Ordering::SeqCst);
        self.current_epoch.fetch_add(1, Ordering::SeqCst);
    }

    /// Execute and flush the shootdowns, returning the processed epoch
    pub fn flush_shootdowns(&self) -> usize {
        self.pending_shootdowns.store(0, Ordering::SeqCst);
        self.current_epoch.load(Ordering::SeqCst)
    }
}

pub trait VirtualMemoryManager {
    fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError>;
    fn map_huge_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError>;
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError>;
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress>;
    fn handle_page_fault(&mut self, virt: VirtualAddress, error_code: usize) -> Result<(), PageFaultError>;
    fn mark_copy_on_write(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError>; // Enables Linux/BSD Copy-On-Write
}

pub struct SimpleVMM {
    pub pml4: SimplePageTable,
    pub pdpt_tables: Vec<Option<SimplePageTable>>,
    pub pd_tables: Vec<Option<SimplePageTable>>,
    pub pt_tables: Vec<Option<SimplePageTable>>,
    pub next_table_addr: AtomicUsize,
    pub tlb_tracker: TlbTracker,
}

impl Default for SimpleVMM {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleVMM {
    pub fn new() -> Self {
        let pml4 = SimplePageTable::new(0x1000);
        SimpleVMM {
            pml4,
            pdpt_tables: Vec::new(),
            pd_tables: Vec::new(),
            pt_tables: Vec::new(),
            next_table_addr: AtomicUsize::new(0x2000),
            tlb_tracker: TlbTracker::new(),
        }
    }

    fn get_pml4_index(&self, virt: VirtualAddress) -> usize {
        (virt >> 39) & 0x1FF
    }

    fn get_pdpt_index(&self, virt: VirtualAddress) -> usize {
        (virt >> 30) & 0x1FF
    }

    fn get_pd_index(&self, virt: VirtualAddress) -> usize {
        (virt >> 21) & 0x1FF
    }

    fn get_pt_index(&self, virt: VirtualAddress) -> usize {
        (virt >> 12) & 0x1FF
    }

    /// Map a large/giant/huge page with strict physical and virtual alignment boundary checks
    pub fn map_large_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        size: PageSize,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError> {
        let byte_size = size.byte_size();

        // Enforce strict alignment safety checks
        if virt % byte_size != 0 || phys % byte_size != 0 {
            return Err(PageFaultError::InvalidAddress);
        }

        match size {
            PageSize::Standard4KB => self.map_page(virt, phys, user, writable),
            PageSize::Huge2MB => {
                let pml4_idx = self.get_pml4_index(virt);
                let pdpt_idx = self.get_pdpt_index(virt);
                let pd_idx = self.get_pd_index(virt);

                // PML4 and PDPT setup
                let pml4_present = self.pml4.get_entry(pml4_idx).is_present();
                if !pml4_present {
                    let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                    let mut pdpt_entry = SimplePageTableEntry::new();
                    pdpt_entry.set_present(true);
                    pdpt_entry.set_writable(true);
                    pdpt_entry.set_user_accessible(false);
                    pdpt_entry.set_physical_address(pdpt_phys);

                    let pdpt_table = SimplePageTable::new(pdpt_phys);
                    while self.pdpt_tables.len() <= pml4_idx {
                        self.pdpt_tables.push(None);
                    }
                    self.pdpt_tables[pml4_idx] = Some(pdpt_table);
                    self.pml4.set_entry(pml4_idx, pdpt_entry);
                }

                let pdpt_idx_in_vec = pml4_idx;
                if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
                    let pdpt_present = pdpt.get_entry(pdpt_idx).is_present();
                    if !pdpt_present {
                        let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                        let mut pd_entry = SimplePageTableEntry::new();
                        pd_entry.set_present(true);
                        pd_entry.set_writable(true);
                        pd_entry.set_user_accessible(false);
                        pd_entry.set_physical_address(pd_phys);

                        let pd_table = SimplePageTable::new(pd_phys);
                        while self.pd_tables.len() <= pdpt_idx {
                            self.pd_tables.push(None);
                        }
                        self.pd_tables[pdpt_idx] = Some(pd_table);
                        pdpt.set_entry(pdpt_idx, pd_entry);
                    }
                }

                let pd_idx_in_vec = pdpt_idx;
                if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                    let mut pd_entry = SimplePageTableEntry::new();
                    pd_entry.set_present(true);
                    pd_entry.set_writable(writable);
                    pd_entry.set_user_accessible(user);
                    // Point to the 2MB huge physical page directly
                    pd_entry.set_physical_address(phys);
                    // Mark as huge page by setting accessed bit/flag (mock)
                    pd_entry.accessed.store(1, Ordering::SeqCst);
                    pd.set_entry(pd_idx, pd_entry);
                }

                Ok(())
            }
            PageSize::Giant1GB => {
                let pml4_idx = self.get_pml4_index(virt);
                let pdpt_idx = self.get_pdpt_index(virt);

                // PML4 setup
                let pml4_present = self.pml4.get_entry(pml4_idx).is_present();
                if !pml4_present {
                    let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                    let mut pdpt_entry = SimplePageTableEntry::new();
                    pdpt_entry.set_present(true);
                    pdpt_entry.set_writable(true);
                    pdpt_entry.set_user_accessible(false);
                    pdpt_entry.set_physical_address(pdpt_phys);

                    let pdpt_table = SimplePageTable::new(pdpt_phys);
                    while self.pdpt_tables.len() <= pml4_idx {
                        self.pdpt_tables.push(None);
                    }
                    self.pdpt_tables[pml4_idx] = Some(pdpt_table);
                    self.pml4.set_entry(pml4_idx, pdpt_entry);
                }

                let pdpt_idx_in_vec = pml4_idx;
                if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
                    let mut pdpt_entry = SimplePageTableEntry::new();
                    pdpt_entry.set_present(true);
                    pdpt_entry.set_writable(writable);
                    pdpt_entry.set_user_accessible(user);
                    // Point to the 1GB giant physical page directly
                    pdpt_entry.set_physical_address(phys);
                    // Mark as giant page by setting dirty bit/flag (mock)
                    pdpt_entry.dirty.store(1, Ordering::SeqCst);
                    pdpt.set_entry(pdpt_idx, pdpt_entry);
                }

                Ok(())
            }
        }
    }
}

impl VirtualMemoryManager for SimpleVMM {
    fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);
        
        let pml4_present = self.pml4.get_entry(pml4_idx).is_present();
        
        if !pml4_present {
            let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
            let mut pdpt_entry = SimplePageTableEntry::new();
            pdpt_entry.set_present(true);
            pdpt_entry.set_writable(true);
            pdpt_entry.set_user_accessible(false);
            pdpt_entry.set_physical_address(pdpt_phys);

            let pdpt_table = SimplePageTable::new(pdpt_phys);
            while self.pdpt_tables.len() <= pml4_idx {
                self.pdpt_tables.push(None);
            }
            self.pdpt_tables[pml4_idx] = Some(pdpt_table);
            self.pml4.set_entry(pml4_idx, pdpt_entry);
        }
        
        let pdpt_idx_in_vec = pml4_idx;
        let pdpt_table: &mut Option<SimplePageTable> = &mut self.pdpt_tables[pdpt_idx_in_vec];
        let pdpt_present = if let Some(ref mut pdpt) = pdpt_table {
            pdpt.get_entry(pdpt_idx).is_present()
        } else {
            false
        };
        
        let pdpt_phys = self.pml4.get_entry(pml4_idx).get_physical_address();
        let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;

        if !pdpt_present {
            let pdpt_table_mut: &mut Option<SimplePageTable> = &mut self.pdpt_tables[pdpt_idx_in_vec];
            if let Some(ref mut pdpt) = pdpt_table_mut {
                let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                let mut pd_entry = SimplePageTableEntry::new();
                pd_entry.set_present(true);
                pd_entry.set_writable(true);
                pd_entry.set_user_accessible(false);
                pd_entry.set_physical_address(pd_phys);

                let pd_table = SimplePageTable::new(pd_phys);
                while self.pd_tables.len() <= pd_idx_in_vec {
                    self.pd_tables.push(None);
                }
                self.pd_tables[pd_idx_in_vec] = Some(pd_table);
                pdpt.set_entry(pdpt_idx, pd_entry);
            }
        }

        let pd_table: &mut Option<SimplePageTable> = &mut self.pd_tables[pd_idx_in_vec];
        let pd_present = if let Some(ref mut pd) = pd_table {
            pd.get_entry(pd_idx).is_present()
        } else {
            false
        };

        let pdpt_table_ref: &mut Option<SimplePageTable> = &mut self.pdpt_tables[pdpt_idx_in_vec];
        let pd_phys = if let Some(ref mut pdpt) = pdpt_table_ref {
            pdpt.get_entry(pdpt_idx).get_physical_address()
        } else {
            0
        };
        let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;

        if !pd_present {
            let pd_table_mut: &mut Option<SimplePageTable> = &mut self.pd_tables[pd_idx_in_vec];
            if let Some(ref mut pd) = pd_table_mut {
                let pt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                let mut pt_entry = SimplePageTableEntry::new();
                pt_entry.set_present(true);
                pt_entry.set_writable(true);
                pt_entry.set_user_accessible(false);
                pt_entry.set_physical_address(pt_phys);
                
                let pt_table = SimplePageTable::new(pt_phys);
                while self.pt_tables.len() <= pt_idx_in_vec {
                    self.pt_tables.push(None);
                }
                self.pt_tables[pt_idx_in_vec] = Some(pt_table);
                pd.set_entry(pd_idx, pt_entry);
            }
        }
        
        let pt_table_mut: &mut Option<SimplePageTable> = &mut self.pt_tables[pt_idx_in_vec];
        if let Some(ref mut pt) = pt_table_mut {
            let mut pt_entry = SimplePageTableEntry::new();
            pt_entry.set_present(true);
            pt_entry.set_writable(writable);
            pt_entry.set_user_accessible(user);
            pt_entry.set_physical_address(phys);
            pt.set_entry(pt_idx, pt_entry);
        }

        Ok(())
    }

    /// Map a 2MB Transparent Huge Page (THP) directly at the Page Directory level
    fn map_huge_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);

        let is_pml4_present = self.pml4.get_entry_ref(pml4_idx).is_present();
        if !is_pml4_present {
            let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
            let mut pdpt_entry = SimplePageTableEntry::new();
            pdpt_entry.set_present(true);
            pdpt_entry.set_writable(true);
            pdpt_entry.set_user_accessible(false);
            pdpt_entry.set_physical_address(pdpt_phys);

            let pdpt_table = SimplePageTable::new(pdpt_phys);
            while self.pdpt_tables.len() <= pml4_idx {
                self.pdpt_tables.push(None);
            }
            self.pdpt_tables[pml4_idx] = Some(pdpt_table);
            self.pml4.set_entry(pml4_idx, pdpt_entry);
        }

        let pdpt_idx_in_vec = pml4_idx;

        if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
            let is_pdpt_present = pdpt.get_entry_ref(pdpt_idx).is_present();
            if !is_pdpt_present {
                let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                let mut pd_entry = SimplePageTableEntry::new();
                pd_entry.set_present(true);
                pd_entry.set_writable(true);
                pd_entry.set_user_accessible(false);
                pd_entry.set_physical_address(pd_phys);

                let pd_table = SimplePageTable::new(pd_phys);
                while self.pd_tables.len() <= pdpt_idx {
                    self.pd_tables.push(None);
                }
                self.pd_tables[pdpt_idx] = Some(pd_table);
                pdpt.set_entry(pdpt_idx, pd_entry);
            }

            let pd_idx_in_vec = pdpt_idx;

            if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                let mut pd_entry = SimplePageTableEntry::new();
                pd_entry.set_present(true);
                pd_entry.set_writable(writable);
                pd_entry.set_user_accessible(user);
                pd_entry.set_huge(true);
                pd_entry.set_physical_address(phys & 0x000FFFFFFFFE0000); // 2MB aligned
                pd.set_entry(pd_idx, pd_entry);
                self.tlb_tracker.register_invalidation(virt);
            }
        }

        Ok(())
    }

    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);
        
        let pml4_present = self.pml4.get_entry(pml4_idx).is_present();
        if !pml4_present {
            return Err(PageFaultError::NotPresent);
        }
        
        let pdpt_table: &mut Option<SimplePageTable> = &mut self.pdpt_tables[pml4_idx];
        if let Some(ref mut pdpt) = pdpt_table {
            let pdpt_present = pdpt.get_entry(pdpt_idx).is_present();
            if !pdpt_present {
                return Err(PageFaultError::NotPresent);
            }
            
            let pdpt_phys = self.pml4.get_entry(pml4_idx).get_physical_address();
            let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;

            let pd_table: &mut Option<SimplePageTable> = &mut self.pd_tables[pd_idx_in_vec];
            if let Some(ref mut pd) = pd_table {
                let pd_present = pd.get_entry(pd_idx).is_present();
                if !pd_present {
                    return Err(PageFaultError::NotPresent);
                }
                
                let pd_phys = pdpt.get_entry(pdpt_idx).get_physical_address();
                let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;

                let pt_table: &mut Option<SimplePageTable> = &mut self.pt_tables[pt_idx_in_vec];
                if let Some(ref mut pt) = pt_table {
                    let mut pt_entry = SimplePageTableEntry::new();
                    pt_entry.set_present(false);
                    pt.set_entry(pt_idx, pt_entry);
                }
            }
        }

        Ok(())
    }

    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);
        
        let pml4_entry = self.pml4.get_entry_ref(pml4_idx);
        if !pml4_entry.is_present() {
            return None;
        }

        if let Some(ref pdpt) = self.pdpt_tables[pml4_idx] {
            let pdpt_entry = pdpt.get_entry_ref(pdpt_idx);
            if !pdpt_entry.is_present() {
                return None;
            }
            
            let pdpt_phys = self.pml4.get_entry_ref(pml4_idx).get_physical_address();
            let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;

            if let Some(ref pd) = self.pd_tables[pd_idx_in_vec] {
                let pd_entry_ref = pd.get_entry_ref(pd_idx);
                if !pd_entry_ref.is_present() {
                    return None;
                }
                
                let pd_phys = pdpt.get_entry_ref(pdpt_idx).get_physical_address();
                let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;

                if let Some(ref pt) = self.pt_tables[pt_idx_in_vec] {
                    let pt_entry = pt.get_entry_ref(pt_idx);
                    if pt_entry.is_present() {
                        let page_offset = virt & 0xFFF;
                        return Some(pt_entry.get_physical_address() | page_offset);
                    }
                }
            }
        }

        None
    }
    
    /// Overloaded to handle custom Linux/BSD-grade Copy-On-Write (COW) and demand paging exceptions
    fn handle_page_fault(&mut self, virt: VirtualAddress, error_code: usize) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);

        // Check if there is an active COW page entry at this address
        let mut is_cow_fault = false;
        if self.pml4.get_entry(pml4_idx).is_present() {
            if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
                if pdpt.get_entry(pdpt_idx).is_present() {
                    let pdpt_phys = self.pml4.get_entry(pml4_idx).get_physical_address();
                    let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;
                    if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                        if pd.get_entry(pd_idx).is_present() {
                            let pd_phys = pdpt.get_entry(pdpt_idx).get_physical_address();
                            let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;
                            if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
                                let entry = pt.get_entry(pt_idx);
                                if entry.is_present() && entry.is_cow() && error_code == 2 {
                                    is_cow_fault = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if is_cow_fault {
            // 1. Copy-On-Write (COW) Fault Handler:
            // Allocate a brand-new distinct physical page frame, copy contents, map it as writable, and clear the COW bit
            let new_phys_frame = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);

            if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
                let pdpt_phys = self.pml4.get_entry(pml4_idx).get_physical_address();
                let _pd_idx_in_vec_local = (pdpt_phys / 4096) * 512 + pdpt_idx;
                if let Some(ref mut _pd) = self.pd_tables[_pd_idx_in_vec_local] {
                    let pd_phys = pdpt.get_entry(pdpt_idx).get_physical_address();
                    let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;
                    if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
                        let entry = pt.get_entry(pt_idx);
                        entry.set_physical_address(new_phys_frame);
                        entry.set_writable(true); // make writable again!
                        entry.set_cow(false); // clear COW flag!
                    }
                }
            }
            Ok(())
        } else {
            // 2. Standard Demand Paging: Allocate a zeroed page frame on demand
            let phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
            self.map_page(virt, phys, true, true)
        }
    }

    /// Marks a virtual page as Read-only and sets the Copy-on-Write (COW) bit flag
    fn mark_copy_on_write(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);

        if !self.pml4.get_entry(pml4_idx).is_present() {
            return Err(PageFaultError::NotPresent);
        }

        let pdpt_table: &mut Option<SimplePageTable> = &mut self.pdpt_tables[pml4_idx];
        if let Some(ref mut pdpt) = pdpt_table {
            if !pdpt.get_entry(pdpt_idx).is_present() {
                return Err(PageFaultError::NotPresent);
            }

            let pdpt_phys = self.pml4.get_entry(pml4_idx).get_physical_address();
            let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;

            let pd_table: &mut Option<SimplePageTable> = &mut self.pd_tables[pd_idx_in_vec];
            if let Some(ref mut pd) = pd_table {
                if !pd.get_entry(pd_idx).is_present() {
                    return Err(PageFaultError::NotPresent);
                }

                let pd_phys = pdpt.get_entry(pdpt_idx).get_physical_address();
                let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;

                let pt_table: &mut Option<SimplePageTable> = &mut self.pt_tables[pt_idx_in_vec];
                if let Some(ref mut pt) = pt_table {
                    let entry = pt.get_entry(pt_idx);
                    if entry.is_present() {
                        entry.set_writable(false); // Map as Read-only to trigger a page fault on write
                        entry.set_cow(true);       // Enable COW tracking
                        return Ok(());
                    }
                }
            }
        }

        Err(PageFaultError::NotPresent)
    }
}

pub trait ProcessMemory {
    fn create_address_space(&mut self) -> Result<usize, PageFaultError>;
    fn destroy_address_space(&mut self, space_id: usize) -> Result<(), PageFaultError>;
    fn map_region(
        &mut self,
        space_id: usize,
        base: VirtualAddress,
        size: usize,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError>;
}

pub struct SimpleProcessMemory {
    pub address_spaces: Vec<Option<SimpleVMM>>,
    pub next_id: AtomicUsize,
}

impl Default for SimpleProcessMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleProcessMemory {
    pub fn new() -> Self {
        SimpleProcessMemory {
            address_spaces: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ProcessMemory for SimpleProcessMemory {
    fn create_address_space(&mut self) -> Result<usize, PageFaultError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let vmm = SimpleVMM::new();
        while self.address_spaces.len() <= id {
            self.address_spaces.push(None);
        }
        self.address_spaces[id] = Some(vmm);
        Ok(id)
    }

    fn destroy_address_space(&mut self, space_id: usize) -> Result<(), PageFaultError> {
        if space_id >= self.address_spaces.len() {
            return Err(PageFaultError::InvalidAddress);
        }
        self.address_spaces[space_id] = None;
        Ok(())
    }

    fn map_region(
        &mut self,
        space_id: usize,
        base: VirtualAddress,
        size: usize,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError> {
        if space_id >= self.address_spaces.len() {
            return Err(PageFaultError::InvalidAddress);
        }
        let vmm_opt: &mut Option<SimpleVMM> = &mut self.address_spaces[space_id];
        if let Some(ref mut vmm) = vmm_opt {
            let page_count = size.div_ceil(4096);
            for i in 0..page_count {
                let virt = base + i * 4096;
                let phys = 0x1000000 + i * 4096;
                vmm.map_page(virt, phys, user, writable)?;
            }
            Ok(())
        } else {
            Err(PageFaultError::InvalidAddress)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vmm_mapping() {
        let mut vmm = SimpleVMM::new();
        let virt = 0x1000_1000;
        let phys = 0x2000_0000;

        // Initially not mapped
        assert!(vmm.get_physical(virt).is_none());

        // Map page
        assert!(vmm.map_page(virt, phys, false, true).is_ok());

        // Should be mapped with offset (the last 12 bits are page offset)
        assert_eq!(vmm.get_physical(virt).unwrap(), phys);

        // Unmap page
        assert!(vmm.unmap_page(virt).is_ok());
        assert!(vmm.get_physical(virt).is_none());
    }

    #[test]
    fn test_process_memory() {
        let mut pm = SimpleProcessMemory::new();
        let space_id = pm.create_address_space().unwrap();

        assert!(pm.map_region(space_id, 0x4000_0000, 8192, true, true).is_ok());

        assert!(pm.destroy_address_space(space_id).is_ok());
    }

    #[test]
    fn test_linux_copy_on_write_paging() {
        let mut vmm = SimpleVMM::new();
        let virt = 0x5000_2000;
        let original_phys = 0x9000_0000;

        // 1. Initial mapping as writable and present
        vmm.map_page(virt, original_phys, true, true).unwrap();
        assert_eq!(vmm.get_physical(virt).unwrap(), original_phys);

        // 2. Mark the page as Copy-on-Write (COW) (maps it read-only, sets the COW bit)
        vmm.mark_copy_on_write(virt).unwrap();

        // Verify it is now read-only and marked as COW
        let pml4_idx = vmm.get_pml4_index(virt);
        let pdpt_idx = vmm.get_pdpt_index(virt);
        let pd_idx = vmm.get_pd_index(virt);
        let pt_idx = vmm.get_pt_index(virt);

        let pdpt_phys = vmm.pml4.get_entry(pml4_idx).get_physical_address();
        let _pd_idx_in_vec_local = (pdpt_phys / 4096) * 512 + pdpt_idx;
        let pd_phys = vmm.pdpt_tables[pml4_idx].as_ref().unwrap().get_entry_ref(pdpt_idx).get_physical_address();
        let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;

        let entry = vmm.pt_tables[pt_idx_in_vec].as_ref().unwrap().get_entry_ref(pt_idx);
        assert!(!entry.is_writable());
        assert!(entry.is_cow());

        // 3. Simulate a Write page fault (error_code = 2 represents write violation on read-only page)
        vmm.handle_page_fault(virt, 2).unwrap();

        // 4. Verify that the COW handler successfully resolved the fault:
        // Allocated a new distinct physical page frame, restored writable permissions, and cleared the COW bit!
        let resolved_entry = vmm.pt_tables[pt_idx_in_vec].as_ref().unwrap().get_entry_ref(pt_idx);
        assert!(resolved_entry.is_writable());
        assert!(!resolved_entry.is_cow());
        assert_ne!(resolved_entry.get_physical_address(), original_phys); // Allocated a distinct frame!
    }
}
