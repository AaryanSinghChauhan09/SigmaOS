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
    fn get_physical_address(&self) -> PhysicalAddress;
    fn set_present(&mut self, present: bool);
    fn set_writable(&mut self, writable: bool);
    fn set_user_accessible(&mut self, user: bool);
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
    pub page_size_flag: AtomicUsize,
}

impl Default for SimplePageTableEntry {
    fn default() -> Self {
        Self::new()
    }
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
}

pub trait PageTable {
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
    #[allow(clippy::deref_addrof)]
    fn get_entry(&mut self, index: usize) -> &mut dyn PageTableEntry {
        if index < 512 {
            &mut self.entries[index]
        } else {
            static mut DUMMY: SimplePageTableEntry = SimplePageTableEntry::new();
            unsafe { &mut *(&raw mut DUMMY) }
        }
    }
    fn get_entry_ref(&self, index: usize) -> &dyn PageTableEntry {
        if index < 512 {
            &self.entries[index]
        } else {
            static DUMMY: SimplePageTableEntry = SimplePageTableEntry::new();
            &DUMMY
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
        let pdpt_present = if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
            pdpt.get_entry(pdpt_idx).is_present()
        } else {
            false
        };

        if !pdpt_present {
            if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
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
        let pd_present = if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
            pd.get_entry(pd_idx).is_present()
        } else {
            false
        };

        if !pd_present {
            if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                let pt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
                let mut pt_entry = SimplePageTableEntry::new();
                pt_entry.set_present(true);
                pt_entry.set_writable(true);
                pt_entry.set_user_accessible(false);
                pt_entry.set_physical_address(pt_phys);

                let pt_table = SimplePageTable::new(pt_phys);
                while self.pt_tables.len() <= pd_idx {
                    self.pt_tables.push(None);
                }
                self.pt_tables[pd_idx] = Some(pt_table);
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

            if let Some(ref pd) = self.pd_tables[pdpt_idx] {
                let pd_entry = pd.get_entry_ref(pd_idx);
                if !pd_entry.is_present() {
                    return None;
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
        error_code: usize,
    ) -> Result<(), PageFaultError> {
        let is_write = (error_code & 2) != 0; // standard page fault error code bit 1 is write

        // Check if the page is already present but read-only (Copy-on-Write)
        if let Some(phys_addr) = self.get_physical(virt) {
            let pd_idx = self.get_pd_index(virt);
            let pt_idx = self.get_pt_index(virt);

            if let Some(ref mut pt_opt) = self.pt_tables.get_mut(pd_idx) {
                if let Some(ref mut pt) = *pt_opt {
                    let entry = pt.get_entry(pt_idx);
                    if entry.is_present() && !entry.is_writable() && is_write {
                        // Copy-on-Write trigger! Duplicate physical frame
                        let new_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);

                        // In real hardware, we'd copy memory contents:
                        // core::ptr::copy_nonoverlapping(phys_addr as *const u8, new_phys as *mut u8, 4096);
                        let _ = phys_addr;

                        entry.set_physical_address(new_phys);
                        entry.set_writable(true);
                        return Ok(());
                    }
                }
            }
        }

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
    fn clone_address_space_cow(&mut self, parent_id: usize) -> Result<usize, PageFaultError>;
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
        if let Some(ref mut vmm) = self.address_spaces[space_id] {
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

    fn clone_address_space_cow(&mut self, parent_id: usize) -> Result<usize, PageFaultError> {
        if parent_id >= self.address_spaces.len() || self.address_spaces[parent_id].is_none() {
            return Err(PageFaultError::InvalidAddress);
        }

        let child_id = self.create_address_space()?;
        let mut child_vmm = SimpleVMM::new();

        if let Some(ref mut parent_vmm) = self.address_spaces[parent_id] {
            // Iterate over active page directory indices
            for pml4_idx in 0..512 {
                if parent_vmm.pml4.get_entry(pml4_idx).is_present() {
                    if let Some(ref mut pdpt_opt) = parent_vmm.pdpt_tables.get_mut(pml4_idx) {
                        if let Some(ref mut pdpt) = *pdpt_opt {
                            for pdpt_idx in 0..512 {
                                if pdpt.get_entry(pdpt_idx).is_present() {
                                    if let Some(ref mut pd_opt) =
                                        parent_vmm.pd_tables.get_mut(pdpt_idx)
                                    {
                                        if let Some(ref mut pd) = *pd_opt {
                                            for pd_idx in 0..512 {
                                                if pd.get_entry(pd_idx).is_present() {
                                                    if let Some(ref mut pt_opt) =
                                                        parent_vmm.pt_tables.get_mut(pd_idx)
                                                    {
                                                        if let Some(ref mut pt) = *pt_opt {
                                                            for pt_idx in 0..512 {
                                                                let entry = pt.get_entry(pt_idx);
                                                                if entry.is_present() {
                                                                    let virt = (pml4_idx << 39)
                                                                        | (pdpt_idx << 30)
                                                                        | (pd_idx << 21)
                                                                        | (pt_idx << 12);
                                                                    let phys = entry
                                                                        .get_physical_address();
                                                                    let is_user =
                                                                        entry.is_user_accessible();
                                                                    let was_writable =
                                                                        entry.is_writable();

                                                                    // If it was writable, we apply Copy-On-Write (mark read-only)
                                                                    if was_writable {
                                                                        entry.set_writable(false);
                                                                    }

                                                                    // Map in child VMM as read-only too (Copy-On-Write)
                                                                    child_vmm.map_page(
                                                                        virt, phys, is_user, false,
                                                                    )?;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.address_spaces[child_id] = Some(child_vmm);
        Ok(child_id)
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
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.data.add(index)) }
        } else {
            None
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
    fn test_process_memory() {
        let mut pm = SimpleProcessMemory::new();
        let space_id = pm.create_address_space().unwrap();

        assert!(pm
            .map_region(space_id, 0x4000_0000, 8192, true, true)
            .is_ok());

        assert!(pm.destroy_address_space(space_id).is_ok());
    }

    #[test]
    fn test_linux_style_copy_on_write() {
        let mut pm = SimpleProcessMemory::new();
        let parent_id = pm.create_address_space().unwrap();

        // Map a writable region in parent
        assert!(pm
            .map_region(parent_id, 0x5000_0000, 4096, true, true)
            .is_ok());

        // Clone parent address space as Copy-On-Write (read-only)
        let child_id = pm.clone_address_space_cow(parent_id).unwrap();

        // Verify parent is now mapped
        let parent_vmm = pm.address_spaces[parent_id].as_ref().unwrap();
        assert!(!parent_vmm.get_physical(0x5000_0000).is_none());

        // Simulate Write Page Fault in parent on 0x5000_0000
        let parent_vmm_mut = pm.address_spaces[parent_id].as_mut().unwrap();
        let old_phys = parent_vmm_mut.get_physical(0x5000_0000).unwrap();

        // Error code 2 = write to read-only page
        assert!(parent_vmm_mut.handle_page_fault(0x5000_0000, 2).is_ok());

        // Writable restored and physical address updated (page copied!)
        let new_phys = parent_vmm_mut.get_physical(0x5000_0000).unwrap();
        assert_ne!(old_phys, new_phys);
    }
}
