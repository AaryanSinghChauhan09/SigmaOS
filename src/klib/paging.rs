#![no_std]
#![no_main]

/// OOP-based Paging + Virtual Memory for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 7-8
/// Implements 4-level page tables, PML4, userspace isolation, page fault handling

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageTableLevel { PML4 = 0, PDPT = 1, PD = 2, PT = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageFaultError { Success = 0, NotPresent = 1, PermissionDenied = 2, InvalidAddress = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PrivilegeLevel { Kernel = 0, User = 3 }

pub trait PageTableEntry {
    fn is_present(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_user_accessible(&self) -> bool;
    fn get_physical_address(&self) -> PhysicalAddress;
    fn set_present(&mut self, present: bool);
    fn set_writable(&mut self, writable: bool);
    fn set_user_accessible(&mut self, user: bool);
    fn set_physical_address(&mut self, addr: PhysicalAddress);
}

#[repr(C)]
pub struct SimplePageTableEntry {
    pub present: AtomicUsize,
    pub writable: AtomicUsize,
    pub user_accessible: AtomicUsize,
    pub physical_addr: AtomicUsize,
    pub accessed: AtomicUsize,
    pub dirty: AtomicUsize,
}

impl SimplePageTableEntry {
    pub fn new() -> Self {
        SimplePageTableEntry {
            present: AtomicUsize::new(0),
            writable: AtomicUsize::new(0),
            user_accessible: AtomicUsize::new(0),
            physical_addr: AtomicUsize::new(0),
            accessed: AtomicUsize::new(0),
            dirty: AtomicUsize::new(0),
        }
    }
}

impl PageTableEntry for SimplePageTableEntry {
    fn is_present(&self) -> bool { self.present.load(Ordering::SeqCst) == 1 }
    fn is_writable(&self) -> bool { self.writable.load(Ordering::SeqCst) == 1 }
    fn is_user_accessible(&self) -> bool { self.user_accessible.load(Ordering::SeqCst) == 1 }
    fn get_physical_address(&self) -> PhysicalAddress {
        self.physical_addr.load(Ordering::SeqCst) & 0x000FFFFFFFFFF000
    }
    fn set_present(&mut self, present: bool) {
        self.present.store(if present { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_writable(&mut self, writable: bool) {
        self.writable.store(if writable { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_user_accessible(&mut self, user: bool) {
        self.user_accessible.store(if user { 1 } else { 0 }, Ordering::SeqCst);
    }
    fn set_physical_address(&mut self, addr: PhysicalAddress) {
        self.physical_addr.store(addr & 0x000FFFFFFFFFF000, Ordering::SeqCst);
    }
}

pub trait PageTable {
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

impl PageTable for SimplePageTable {
    fn get_entry(&mut self, index: usize) -> &mut dyn PageTableEntry {
        if index < 512 {
            &mut self.entries[index]
        } else {
            static mut DUMMY: SimplePageTableEntry = SimplePageTableEntry::new();
            unsafe { &mut DUMMY }
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
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool) -> Result<(), PageFaultError>;
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError>;
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress>;
    fn handle_page_fault(&mut self, virt: VirtualAddress, error_code: usize) -> Result<(), PageFaultError>;
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
}

impl VirtualMemoryManager for SimpleVMM {
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);
        
        let pml4_entry = self.pml4.get_entry(pml4_idx);
        
        if !pml4_entry.is_present() {
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
        
        let pdpt_phys = pml4_entry.get_physical_address();
        let pdpt_idx_in_vec = pml4_idx;
        
        if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
            let pdpt_entry = pdpt.get_entry(pdpt_idx);
            
            if !pdpt_entry.is_present() {
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
            
            let pd_phys = pdpt_entry.get_physical_address();
            let pd_idx_in_vec = pdpt_idx;
            
            if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                let pd_entry = pd.get_entry(pd_idx);
                
                if !pd_entry.is_present() {
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
                
                let pt_phys = pd_entry.get_physical_address();
                let pt_idx_in_vec = pd_idx;
                
                if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
                    let mut pt_entry = SimplePageTableEntry::new();
                    pt_entry.set_present(true);
                    pt_entry.set_writable(writable);
                    pt_entry.set_user_accessible(user);
                    pt_entry.set_physical_address(phys);
                    pt.set_entry(pt_idx, pt_entry);
                }
            }
        }
        
        Ok(())
    }
    
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError> {
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);
        
        let pml4_entry = self.pml4.get_entry(pml4_idx);
        if !pml4_entry.is_present() {
            return Err(PageFaultError::NotPresent);
        }
        
        if let Some(ref mut pdpt) = self.pdpt_tables[pml4_idx] {
            let pdpt_entry = pdpt.get_entry(pdpt_idx);
            if !pdpt_entry.is_present() {
                return Err(PageFaultError::NotPresent);
            }
            
            if let Some(ref mut pd) = self.pd_tables[pdpt_idx] {
                let pd_entry = pd.get_entry(pd_idx);
                if !pd_entry.is_present() {
                    return Err(PageFaultError::NotPresent);
                }
                
                if let Some(ref mut pt) = self.pt_tables[pd_idx] {
                    let pt_entry = pt.get_entry(pt_idx);
                    pt_entry.set_present(false);
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
        
        let pml4_entry = self.pml4.get_entry(pml4_idx);
        if !pml4_entry.is_present() {
            return None;
        }
        
        if let Some(ref pdpt) = self.pdpt_tables[pml4_idx] {
            let pdpt_entry = pdpt.get_entry(pdpt_idx);
            if !pdpt_entry.is_present() {
                return None;
            }
            
            if let Some(ref pd) = self.pd_tables[pdpt_idx] {
                let pd_entry = pd.get_entry(pd_idx);
                if !pd_entry.is_present() {
                    return None;
                }
                
                if let Some(ref pt) = self.pt_tables[pd_idx] {
                    let pt_entry = pt.get_entry(pt_idx);
                    if pt_entry.is_present() {
                        let page_offset = virt & 0xFFF;
                        return Some(pt_entry.get_physical_address() | page_offset);
                    }
                }
            }
        }
        
        None
    }
    
    fn handle_page_fault(&mut self, virt: VirtualAddress, _error_code: usize) -> Result<(), PageFaultError> {
        let phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst);
        self.map_page(virt, phys, true, true)
    }
}

pub trait ProcessMemory {
    fn create_address_space(&mut self) -> Result<usize, PageFaultError>;
    fn destroy_address_space(&mut self, space_id: usize) -> Result<(), PageFaultError>;
    fn map_region(&mut self, space_id: usize, base: VirtualAddress, size: usize, user: bool, writable: bool) -> Result<(), PageFaultError>;
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
    
    fn map_region(&mut self, space_id: usize, base: VirtualAddress, size: usize, user: bool, writable: bool) -> Result<(), PageFaultError> {
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
