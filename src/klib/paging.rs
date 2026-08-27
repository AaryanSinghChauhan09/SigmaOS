use crate::klib::vec::Vec;
/// OOP-based Paging + Virtual Memory for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 7-8
/// Implements 4-level page tables, PML4, userspace isolation, page fault handling
use core::sync::atomic::{AtomicUsize, Ordering};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSize {
    Standard4KB,
    Huge2MB,
    Giant1GB,
}

impl PageSize {
    pub fn byte_size(&self) -> usize {
        match self {
            PageSize::Standard4KB => 4096,
            PageSize::Huge2MB => 2 * 1024 * 1024,
            PageSize::Giant1GB => 1024 * 1024 * 1024,
        }
    }
}

impl Clone for SimplePageTableEntry {
    fn clone(&self) -> Self {
        Self {
            present: AtomicUsize::new(self.present.load(Ordering::SeqCst)),
            writable: AtomicUsize::new(self.writable.load(Ordering::SeqCst)),
            user_accessible: AtomicUsize::new(self.user_accessible.load(Ordering::SeqCst)),
            physical_addr: AtomicUsize::new(self.physical_addr.load(Ordering::SeqCst)),
            accessed: AtomicUsize::new(self.accessed.load(Ordering::SeqCst)),
            dirty: AtomicUsize::new(self.dirty.load(Ordering::SeqCst)),
            cow: AtomicUsize::new(self.cow.load(Ordering::SeqCst)),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageTableLevel {
    PML4 = 0,
    PDPT = 1,
    PD = 2,
    PT = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultError {
    Success = 0,
    NotPresent = 1,
    PermissionDenied = 2,
    InvalidAddress = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    Kernel = 0,
    User = 3,
}


pub trait PageTableEntry {
    fn is_present(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_user_accessible(&self) -> bool;
    fn get_physical_address(&self) -> PhysicalAddress;
    fn set_present(&mut self, present: bool);
    fn set_writable(&mut self, writable: bool);
    fn set_user_accessible(&mut self, user: bool);
    fn set_physical_address(&mut self, addr: PhysicalAddress);
    fn is_cow(&self) -> bool {
        false
    }
    fn set_cow(&mut self, _cow: bool) {}
    fn is_huge(&self) -> bool {
        false
    }
    fn is_giant(&self) -> bool {
        false
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

impl Default for SimplePageTableEntry {
    fn default() -> Self {
        Self::new()
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
    fn is_cow(&self) -> bool {
        self.cow.load(Ordering::SeqCst) == 1
    }
    fn set_cow(&mut self, cow: bool) {
        self.cow.store(if cow { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait PageTable {
    fn get_entry_ref(&self, index: usize) -> &dyn PageTableEntry;
    fn get_entry(&mut self, index: usize) -> &mut dyn PageTableEntry;
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

static DUMMY: SimplePageTableEntry = SimplePageTableEntry::new();

impl PageTable for SimplePageTable {
    fn get_entry_ref(&self, index: usize) -> &dyn PageTableEntry {
        if index < 512 {
            &self.entries[index]
        } else {
            &DUMMY
        }
    }

    fn get_entry(&mut self, index: usize) -> &mut dyn PageTableEntry {
        if index < 512 {
            &mut self.entries[index]
        } else {
            static mut MUT_DUMMY: SimplePageTableEntry = SimplePageTableEntry::new();
            unsafe { &mut MUT_DUMMY }
        }
    }

    fn set_entry(&mut self, index: usize, entry: SimplePageTableEntry) {
        if index < 512 {
            self.entries[index] = entry;
        }
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

    pub fn map_large_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        size: PageSize,
        user: bool,
        writable: bool,
    ) -> Result<(), PageFaultError> {
        let byte_size = size.byte_size();

        if virt % byte_size != 0 || phys % byte_size != 0 {
            return Err(PageFaultError::InvalidAddress);
        }

        match size {
            PageSize::Standard4KB => self.map_page(virt, phys, user, writable),
            PageSize::Huge2MB => {
                let pml4_idx = self.get_pml4_index(virt);
                let pdpt_idx = self.get_pdpt_index(virt);
                let pd_idx = self.get_pd_index(virt);

                let pml4_present = self.pml4.get_entry_ref(pml4_idx).is_present();
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

                if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
                    let pdpt_present = pdpt.get_entry_ref(pdpt_idx).is_present();
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

                if let Some(ref mut pd) = self.pd_tables[pdpt_idx] {
                    let mut pd_entry = SimplePageTableEntry::new();
                    pd_entry.set_present(true);
                    pd_entry.set_writable(writable);
                    pd_entry.set_user_accessible(user);
                    pd_entry.set_physical_address(phys);
                    pd_entry.accessed.store(1, Ordering::SeqCst);
                    pd.set_entry(pd_idx, pd_entry);
                }

                Ok(())
            }
            PageSize::Giant1GB => {
                let pml4_idx = self.get_pml4_index(virt);
                let pdpt_idx = self.get_pdpt_index(virt);

                let pml4_present = self.pml4.get_entry_ref(pml4_idx).is_present();
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

                if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
                    let mut pdpt_entry = SimplePageTableEntry::new();
                    pdpt_entry.set_present(true);
                    pdpt_entry.set_writable(writable);
                    pdpt_entry.set_user_accessible(user);
                    pdpt_entry.set_physical_address(phys);
                    pdpt_entry.dirty.store(1, Ordering::SeqCst);
                    pdpt.set_entry(pdpt_idx, pdpt_entry);
                }

                Ok(())
            }
        }
    }

    pub fn mark_copy_on_write(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);

        if let Some(ref mut pdpt) = self.pdpt_tables.get_mut(pml4_idx).and_then(|o| o.as_mut()) {
            let pdpt_phys = self.pml4.get_entry_ref(pml4_idx).get_physical_address();
            let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;
            if let Some(ref mut pd) = self
                .pd_tables
                .get_mut(pd_idx_in_vec)
                .and_then(|o| o.as_mut())
            {
                let pd_phys = pdpt.get_entry_ref(pdpt_idx).get_physical_address();
                let pt_idx_in_vec = (pd_phys / 4096) * 512 + pd_idx;
                if let Some(ref mut pt) = self
                    .pt_tables
                    .get_mut(pt_idx_in_vec)
                    .and_then(|o| o.as_mut())
                {
                    let entry = &mut pt.entries[pt_idx];
                    entry.set_writable(false);
                    entry.cow.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PageFaultError::NotPresent)
    }
}

impl Default for SimpleVMM {
    fn default() -> Self {
        Self::new()
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

        if !self.pml4.get_entry(pml4_idx).is_present() {
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

        while self.pd_tables.len() <= pdpt_idx {
            self.pd_tables.push(None);
        }

        let pdpt_phys = self.pml4.get_entry(pml4_idx).get_physical_address();
        let pd_idx_in_vec = (pdpt_phys / 4096) * 512 + pdpt_idx;

        while self.pd_tables.len() <= pd_idx_in_vec {
            self.pd_tables.push(None);
        }

        if self.pd_tables[pd_idx_in_vec].is_none() {
            let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
            let mut pd_entry = SimplePageTableEntry::new();
            pd_entry.set_present(true);
            pd_entry.set_writable(true);
            pd_entry.set_user_accessible(false);
            pd_entry.set_physical_address(pd_phys);

            let pd_table = SimplePageTable::new(pd_phys);
            self.pd_tables[pd_idx_in_vec] = Some(pd_table);

            if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
                pdpt.set_entry(pdpt_idx, pd_entry);
            }
        }

        let pt_idx_in_vec = pd_idx_in_vec * 512 + pd_idx;
        while self.pt_tables.len() <= pt_idx_in_vec {
            self.pt_tables.push(None);
        }

        if self.pt_tables[pt_idx_in_vec].is_none() {
            let pt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
            let mut pt_entry = SimplePageTableEntry::new();
            pt_entry.set_present(true);
            pt_entry.set_writable(true);
            pt_entry.set_user_accessible(false);
            pt_entry.set_physical_address(pt_phys);

            let pt_table = SimplePageTable::new(pt_phys);
            self.pt_tables[pt_idx_in_vec] = Some(pt_table);

            if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                pd.set_entry(pd_idx, pt_entry);
            }
        }

        if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
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

        if !self.pml4.get_entry(pml4_idx).is_present() {
            return Err(PageFaultError::NotPresent);
        }

        if let Some(ref mut pdpt) = self
            .pdpt_tables
            .get_mut(pml4_idx)
            .and_then(|opt| opt.as_mut())
        {
            if !pdpt.get_entry(pdpt_idx).is_present() {
                return Err(PageFaultError::NotPresent);
            }

            if let Some(ref mut pd) = self
                .pd_tables
                .get_mut(pdpt_idx)
                .and_then(|opt| opt.as_mut())
            {
                if !pd.get_entry(pd_idx).is_present() {
                    return Err(PageFaultError::NotPresent);
                }

                if let Some(ref mut pt) =
                    self.pt_tables.get_mut(pd_idx).and_then(|opt| opt.as_mut())
                {
                    let pt_entry = pt.get_entry(pt_idx);
                    pt_entry.set_present(false);
                    return Ok(());
                }
            }
        }

        Err(PageFaultError::NotPresent)
    }

    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);

        if !self.pml4.get_entry_ref(pml4_idx).is_present() {
            return None;
        }

        if let Some(ref pdpt) = self.pdpt_tables.get(pml4_idx).and_then(|opt| opt.as_ref()) {
            let pdpt_entry = pdpt.get_entry_ref(pdpt_idx);
            if !pdpt_entry.is_present() {
                return None;
            }

            if pdpt_entry.is_giant() {
                let page_offset = virt & 0x3FFFFFFF;
                return Some(pdpt_entry.get_physical_address() | page_offset);
            }

            if let Some(ref pd) = self.pd_tables.get(pdpt_idx).and_then(|opt| opt.as_ref()) {
                let pd_entry = pd.get_entry_ref(pd_idx);
                if !pd_entry.is_present() {
                    return None;
                }

                if pd_entry.is_huge() {
                    let page_offset = virt & 0x1FFFFF;
                    return Some(pd_entry.get_physical_address() | page_offset);
                }

                if let Some(ref pt) = self.pt_tables.get(pd_idx).and_then(|opt| opt.as_ref()) {
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
        let new_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
        self.map_page(virt, new_phys, true, true)
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

impl SimpleProcessMemory {
    pub fn new() -> Self {
        SimpleProcessMemory {
            address_spaces: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleProcessMemory {
    fn default() -> Self {
        Self::new()
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
        if let Some(ref mut vmm) = self.address_spaces[space_id] {
            let page_count = (size + 4095) / 4096;
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
    fn test_paging_and_cow() {
        let mut vmm = SimpleVMM::new();
        assert!(vmm.map_page(0x1000, 0x2000000, true, true).is_ok());
        assert_eq!(vmm.get_physical(0x1000).unwrap(), 0x2000000);
        assert!(vmm.mark_copy_on_write(0x1000).is_ok());
        assert!(vmm.handle_page_fault(0x1000, 2).is_ok());
        assert_ne!(vmm.get_physical(0x1000).unwrap(), 0x2000000);
    }

    #[test]
    fn test_huge_pages_mapping() {
        let mut vmm = SimpleVMM::new();
        let virt_1gb = 0x40000000;
        let phys_1gb = 0x80000000;

        assert!(vmm.get_physical(virt_1gb).is_none());
        assert!(vmm
            .map_large_page(virt_1gb, phys_1gb, PageSize::Giant1GB, false, true)
            .is_ok());
        assert_eq!(vmm.get_physical(virt_1gb).unwrap(), phys_1gb);
        assert_eq!(
            vmm.get_physical(virt_1gb + 0x2000).unwrap(),
            phys_1gb + 0x2000
        );
    }

    #[test]
    fn test_paging_alignment_safety() {
        let mut vmm = SimpleVMM::new();

        let unaligned_virt = 0x401000;
        let phys_2mb = 0x800000;

        let res = vmm.map_large_page(unaligned_virt, phys_2mb, PageSize::Huge2MB, false, true);
        assert!(matches!(res, Err(PageFaultError::InvalidAddress)));

        let virt_1gb = 0x40000000;
        let unaligned_phys = 0x80001000;

        let res2 = vmm.map_large_page(virt_1gb, unaligned_phys, PageSize::Giant1GB, false, true);
        assert!(matches!(res2, Err(PageFaultError::InvalidAddress)));
    }
}
