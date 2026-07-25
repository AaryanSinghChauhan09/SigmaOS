#![no_std]
#![no_main]

/// OOP-based Physical + Virtual Memory Manager for SigmaOS
/// Based on Roadmap Item: Physical + Virtual Memory Manager with Formal Verification

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;
pub type PageNumber = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PageState { Free = 0, Allocated = 1, Reserved = 2, Locked = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MemoryError { Success = 0, OutOfMemory = 1, InvalidAddress = 2, PermissionDenied = 3 }

pub trait PhysicalMemoryManager {
    fn allocate_page(&mut self) -> Result<PhysicalAddress, MemoryError>;
    fn free_page(&mut self, addr: PhysicalAddress) -> Result<(), MemoryError>;
    fn get_page_state(&self, addr: PhysicalAddress) -> PageState;
}

#[repr(C)]
pub struct SimplePMM {
    pub page_bitmap: [AtomicUsize; 1024],
    pub total_pages: AtomicUsize,
    pub free_pages: AtomicUsize,
}

impl SimplePMM {
    pub fn new(total_pages: usize) -> Self {
        let mut page_bitmap = [AtomicUsize::new(0); 1024];
        SimplePMM {
            page_bitmap,
            total_pages: AtomicUsize::new(total_pages),
            free_pages: AtomicUsize::new(total_pages),
        }
    }
}

impl PhysicalMemoryManager for SimplePMM {
    fn allocate_page(&mut self) -> Result<PhysicalAddress, MemoryError> {
        if self.free_pages.load(Ordering::SeqCst) == 0 {
            return Err(MemoryError::OutOfMemory);
        }
        for i in 0..1024 {
            let bitmap_value = self.page_bitmap[i].load(Ordering::SeqCst);
            if bitmap_value != usize::MAX {
                for bit in 0..64 {
                    if (bitmap_value & (1 << bit)) == 0 {
                        let page_num = i * 64 + bit;
                        self.page_bitmap[i].fetch_or(1 << bit, Ordering::SeqCst);
                        self.free_pages.fetch_sub(1, Ordering::SeqCst);
                        return Ok(page_num * 4096);
                    }
                }
            }
        }
        Err(MemoryError::OutOfMemory)
    }
    fn free_page(&mut self, addr: PhysicalAddress) -> Result<(), MemoryError> {
        let page_num = addr / 4096;
        let bitmap_idx = page_num / 64;
        let bit = page_num % 64;
        if bitmap_idx >= 1024 {
            return Err(MemoryError::InvalidAddress);
        }
        self.page_bitmap[bitmap_idx].fetch_and(!(1 << bit), Ordering::SeqCst);
        self.free_pages.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn get_page_state(&self, addr: PhysicalAddress) -> PageState {
        let page_num = addr / 4096;
        let bitmap_idx = page_num / 64;
        let bit = page_num % 64;
        if bitmap_idx >= 1024 {
            return PageState::Reserved;
        }
        if (self.page_bitmap[bitmap_idx].load(Ordering::SeqCst) & (1 << bit)) != 0 {
            PageState::Allocated
        } else {
            PageState::Free
        }
    }
}

pub trait VirtualMemoryManager {
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress) -> Result<(), MemoryError>;
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), MemoryError>;
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress>;
}

#[repr(C)]
pub struct PageTableEntry {
    pub present: AtomicUsize,
    pub writable: AtomicUsize,
    pub user_accessible: AtomicUsize,
    pub physical_addr: AtomicUsize,
}

impl PageTableEntry {
    pub fn new() -> Self {
        PageTableEntry {
            present: AtomicUsize::new(0),
            writable: AtomicUsize::new(0),
            user_accessible: AtomicUsize::new(0),
            physical_addr: AtomicUsize::new(0),
        }
    }
}

pub struct SimpleVMM {
    pub page_table: Vec<Option<PageTableEntry>>,
    pub pmm: SimplePMM,
}

impl SimpleVMM {
    pub fn new(pmm: SimplePMM) -> Self {
        let mut page_table = Vec::new();
        for _ in 0..1024 {
            page_table.push(None);
        }
        SimpleVMM { page_table, pmm }
    }
}

impl VirtualMemoryManager for SimpleVMM {
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress) -> Result<(), MemoryError> {
        let page_num = virt / 4096;
        if page_num >= 1024 {
            return Err(MemoryError::InvalidAddress);
        }
        let mut entry = PageTableEntry::new();
        entry.present.store(1, Ordering::SeqCst);
        entry.writable.store(1, Ordering::SeqCst);
        entry.user_accessible.store(1, Ordering::SeqCst);
        entry.physical_addr.store(phys, Ordering::SeqCst);
        self.page_table[page_num] = Some(entry);
        Ok(())
    }
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), MemoryError> {
        let page_num = virt / 4096;
        if page_num >= 1024 {
            return Err(MemoryError::InvalidAddress);
        }
        self.page_table[page_num] = None;
        Ok(())
    }
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress> {
        let page_num = virt / 4096;
        if page_num >= 1024 {
            return None;
        }
        if let Some(ref entry) = self.page_table[page_num] {
            if entry.present.load(Ordering::SeqCst) == 1 {
                return Some(entry.physical_addr.load(Ordering::SeqCst));
            }
        }
        None
    }
}

pub trait MemoryVerifier {
    fn verify_allocation(&self, addr: PhysicalAddress) -> Result<bool, MemoryError>;
    fn verify_mapping(&self, virt: VirtualAddress) -> Result<bool, MemoryError>;
}

impl MemoryVerifier for SimpleVMM {
    fn verify_allocation(&self, addr: PhysicalAddress) -> Result<bool, MemoryError> {
        Ok(self.pmm.get_page_state(addr) == PageState::Allocated)
    }
    fn verify_mapping(&self, virt: VirtualAddress) -> Result<bool, MemoryError> {
        Ok(self.get_physical(virt).is_some())
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
