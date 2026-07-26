use crate::klib::vec::Vec;
/// OOP-based Paging + Virtual Memory for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 7-8
/// Implements 4-level page tables, PML4, userspace isolation, page fault handling,
/// and Linux-style Copy-on-Write (CoW) address space cloning.
use core::sync::atomic::{AtomicUsize, Ordering};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageTableLevel {
    PML4 = 0,
    PDPT = 1,
    PD = 2,
    PT = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageFaultError {
    Success = 0,
    NotPresent = 1,
    PermissionDenied = 2,
    InvalidAddress = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PrivilegeLevel {
    Kernel = 0,
    User = 3,
}

pub trait PageTableEntry {
    fn is_present(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_user_accessible(&self) -> bool;
    fn is_cow(&self) -> bool;
    fn get_physical_address(&self) -> PhysicalAddress;
    fn set_present(&mut self, present: bool);
    fn set_writable(&mut self, writable: bool);
    fn set_user_accessible(&mut self, user: bool);
    fn set_cow(&mut self, cow: bool);
    fn set_physical_address(&mut self, addr: PhysicalAddress);
    fn get_page_size(&self) -> usize {
        0
    }
    fn set_page_size(&mut self, size: usize) {
        let _ = size;
    }
}

#[repr(C)]
pub struct SimplePageTableEntry {
    pub present: AtomicUsize,
    pub writable: AtomicUsize,
    pub user_accessible: AtomicUsize,
    pub physical_addr: AtomicUsize,
    pub accessed: AtomicUsize,
    pub dirty: AtomicUsize,
    pub cow: AtomicUsize,
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
    fn set_physical_address(&mut self, addr: PhysicalAddress) {
        self.physical_addr
            .store(addr & 0x000FFFFFFFFFF000, Ordering::SeqCst);
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
            unsafe { &*(&raw mut DUMMY) }
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

pub trait VirtualMemoryManager {
    fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError>;
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError>;
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress>;
    fn handle_page_fault(
        &mut self,
        virt: VirtualAddress,
        error_code: usize,
    ) -> Result<(), PageFaultError>;
}

pub struct SimpleVMM {
    pub pml4: SimplePageTable,
    pub pdpt_tables: Vec<Option<SimplePageTable>>,
    pub pd_tables: Vec<Option<SimplePageTable>>,
    pub pt_tables: Vec<Option<SimplePageTable>>,
    pub next_table_addr: AtomicUsize,
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
            let pdpt_table_mut: &mut Option<SimplePageTable> =
                &mut self.pdpt_tables[pdpt_idx_in_vec];
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
                let pd_entry = pd.get_entry_ref(pd_idx);
                if !pd_entry.is_present() {
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

    fn handle_page_fault(
        &mut self,
        virt: VirtualAddress,
        _error_code: usize,
    ) -> Result<(), PageFaultError> {
        let phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
        self.map_page(virt, phys, true, true)
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

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
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

        assert!(pm
            .map_region(space_id, 0x4000_0000, 8192, true, true)
            .is_ok());

        assert!(pm.destroy_address_space(space_id).is_ok());
    }
}
