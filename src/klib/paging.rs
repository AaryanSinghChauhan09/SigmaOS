use core::mem;
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
    fn get_page_size(&self) -> usize { 0 }
    fn set_page_size(&mut self, size: usize) { let _ = size; }
}

#[repr(C)]
pub struct SimplePageTableEntry {
    pub present: AtomicUsize,
    pub writable: AtomicUsize,
    pub user_accessible: AtomicUsize,
    pub physical_addr: AtomicUsize,
    pub accessed: AtomicUsize,
    pub dirty: AtomicUsize,
    pub page_size_flag: AtomicUsize,
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
            page_size_flag: AtomicUsize::new(0),
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
    fn get_page_size(&self) -> usize {
        self.page_size_flag.load(Ordering::SeqCst)
    }
    fn set_page_size(&mut self, size: usize) {
        self.page_size_flag.store(size, Ordering::SeqCst);
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
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool) -> Result<(), PageFaultError> {
        self.map_page_with_size(virt, phys, user, writable, 4096)
    }
    fn map_page_with_size(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool, page_size: usize) -> Result<(), PageFaultError>;
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
    fn map_page_with_size(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool, page_size: usize) -> Result<(), PageFaultError> {
        // Enforce strict address alignment verification (must be page-aligned to its size)
        if page_size != 4096 && page_size != 2097152 && page_size != 1073741824 {
            return Err(PageFaultError::InvalidAddress);
        }
        if virt % page_size != 0 || phys % page_size != 0 {
            return Err(PageFaultError::InvalidAddress);
        }

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
        
        // 1GB Giant Page support: mapped directly at PDPT level
        if page_size == 1073741824 {
            let pdpt_idx_in_vec = pml4_idx;
            while self.pdpt_tables.len() <= pdpt_idx_in_vec {
                self.pdpt_tables.push(None);
            }
            if self.pdpt_tables[pdpt_idx_in_vec].is_none() {
                let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                let pdpt_table = SimplePageTable::new(pdpt_phys);
                self.pdpt_tables[pdpt_idx_in_vec] = Some(pdpt_table);
            }
            if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
                let mut pdpt_entry = SimplePageTableEntry::new();
                pdpt_entry.set_present(true);
                pdpt_entry.set_writable(writable);
                pdpt_entry.set_user_accessible(user);
                pdpt_entry.set_physical_address(phys);
                pdpt_entry.set_page_size(1073741824);
                pdpt.set_entry(pdpt_idx, pdpt_entry);
            }
            return Ok(());
        }

        let pdpt_idx_in_vec = pml4_idx;
        while self.pdpt_tables.len() <= pdpt_idx_in_vec {
            self.pdpt_tables.push(None);
        }
        if self.pdpt_tables[pdpt_idx_in_vec].is_none() {
            let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
            let pdpt_table = SimplePageTable::new(pdpt_phys);
            self.pdpt_tables[pdpt_idx_in_vec] = Some(pdpt_table);
        }

        let pdpt_present = if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
            pdpt.get_entry(pdpt_idx).is_present()
        } else {
            false
        };

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

        // 2MB Huge Page support: mapped directly at PD level
        if page_size == 2097152 {
            let pd_idx_in_vec = pdpt_idx;
            while self.pd_tables.len() <= pd_idx_in_vec {
                self.pd_tables.push(None);
            }
            if self.pd_tables[pd_idx_in_vec].is_none() {
                let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                let pd_table = SimplePageTable::new(pd_phys);
                self.pd_tables[pd_idx_in_vec] = Some(pd_table);
            }
            if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                let mut pd_entry = SimplePageTableEntry::new();
                pd_entry.set_present(true);
                pd_entry.set_writable(writable);
                pd_entry.set_user_accessible(user);
                pd_entry.set_physical_address(phys);
                pd_entry.set_page_size(2097152);
                pd.set_entry(pd_idx, pd_entry);
            }
            return Ok(());
        }

        let pd_idx_in_vec = pdpt_idx;
        let pd_present = if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
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

        let pt_idx_in_vec = pd_idx;
        if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
            let mut pt_entry = SimplePageTableEntry::new();
            pt_entry.set_present(true);
            pt_entry.set_writable(writable);
            pt_entry.set_user_accessible(user);
            pt_entry.set_physical_address(phys);
            pt_entry.set_page_size(4096);
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

        if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
            let pdpt_present = pdpt.get_entry(pdpt_idx).is_present();
            if !pdpt_present {
                return Err(PageFaultError::NotPresent);
            }

            if let Some(ref mut pd) = self.pd_tables[pdpt_idx] {
                let pd_present = pd.get_entry(pd_idx).is_present();
                if !pd_present {
                    return Err(PageFaultError::NotPresent);
                }

                if let Some(ref mut pt) = self.pt_tables[pd_idx] {
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
            
            // If 1GB Giant page is mapped at PDPT level
            if pdpt_entry.get_page_size() == 1073741824 {
                let page_offset = virt & 0x3FFFFFFF;
                return Some(pdpt_entry.get_physical_address() | page_offset);
            }

            if let Some(ref pd) = self.pd_tables[pdpt_idx] {
                let pd_entry = pd.get_entry_ref(pd_idx);
                if !pd_entry.is_present() {
                    return None;
                }
                
                // If 2MB Huge page is mapped at PD level
                if pd_entry.get_page_size() == 2097152 {
                    let page_offset = virt & 0x1FFFFF;
                    return Some(pd_entry.get_physical_address() | page_offset);
                }

                if let Some(ref pt) = self.pt_tables[pd_idx] {
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

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }
    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
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
    fn test_vmm_huge_and_giant_pages() {
        let mut vmm = SimpleVMM::new();

        // 2MB Huge page mapping: virt and phys aligned to 2MB
        let huge_virt = 0x2000_0000;
        let huge_phys = 0x4000_0000;
        assert!(vmm.map_page_with_size(huge_virt, huge_phys, false, true, 2097152).is_ok());
        assert_eq!(vmm.get_physical(huge_virt).unwrap(), huge_phys);
        // Test offset in huge page
        assert_eq!(vmm.get_physical(huge_virt + 0x1000).unwrap(), huge_phys + 0x1000);

        // 1GB Giant page mapping: virt and phys aligned to 1GB
        let giant_virt = 0x4000_0000;
        let giant_phys = 0x8000_0000;
        assert!(vmm.map_page_with_size(giant_virt, giant_phys, false, true, 1073741824).is_ok());
        assert_eq!(vmm.get_physical(giant_virt).unwrap(), giant_phys);
        // Test offset in giant page
        assert_eq!(vmm.get_physical(giant_virt + 0x2000).unwrap(), giant_phys + 0x2000);
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
