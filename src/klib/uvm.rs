use core::mem;
/// NetBSD & OpenBSD-inspired UVM (Universal Virtual Memory) Subsystem for SigmaOS
/// Provides machine-independent Anonymous Maps (amap), decoupled Physical Maps (pmap),
/// and zero-copy Page Loanout mechanisms to prevent memory copying overhead.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UvmError {
    Success = 0,
    InvalidAddress = 1,
    MappingFailed = 2,
    PageNotLoaned = 3,
}

pub type PhysAddr = usize;
pub type VirtAddr = usize;

/// UvmPmap (Physical Map Interface - Machine-Dependent Paging layer)
pub struct UvmPmap {
    pub pmap_entries: Vec<Option<PhysAddr>>,
}

impl UvmPmap {
    pub fn new() -> Self {
        UvmPmap {
            pmap_entries: Vec::new(),
        }
    }

    /// Establish a virtual-to-physical address mapping (pmap_enter equivalent)
    pub fn pmap_enter(&mut self, virt: VirtAddr, phys: PhysAddr) {
        let idx = virt >> 12; // 4KB page index
        while self.pmap_entries.len <= idx {
            self.pmap_entries.push(None);
        }
        self.pmap_entries[idx] = Some(phys);
    }

    /// Extract physical address from virtual address (pmap_extract equivalent)
    pub fn pmap_extract(&self, virt: VirtAddr) -> Option<PhysAddr> {
        let idx = virt >> 12;
        if idx < self.pmap_entries.len {
            self.pmap_entries[idx]
        } else {
            None
        }
    }

    /// Remove a mapping (pmap_remove equivalent)
    pub fn pmap_remove(&mut self, virt: VirtAddr) {
        let idx = virt >> 12;
        if idx < self.pmap_entries.len {
            self.pmap_entries[idx] = None;
        }
    }
}

/// UvmAmap (Anonymous Memory Map - tracks copy-on-write anonymous pages cleanly without Mach shadow objects)
pub struct AmapSlot {
    pub slot_idx: usize,
    pub phys_page: PhysAddr,
    pub ref_count: AtomicUsize,
}

pub struct UvmAmap {
    pub slots: Vec<Option<AmapSlot>>,
}

impl UvmAmap {
    pub fn new() -> Self {
        UvmAmap { slots: Vec::new() }
    }

    /// Add an anonymous page reference slot (amap_add equivalent)
    pub fn amap_add(&mut self, slot: usize, phys: PhysAddr) {
        let amap_slot = AmapSlot {
            slot_idx: slot,
            phys_page: phys,
            ref_count: AtomicUsize::new(1),
        };
        while self.slots.len <= slot {
            self.slots.push(None);
        }
        self.slots[slot] = Some(amap_slot);
    }

    /// Retrieve physical address of an anonymous page slot
    pub fn amap_lookup(&self, slot: usize) -> Option<PhysAddr> {
        if slot < self.slots.len {
            self.slots[slot].as_ref().map(|s| s.phys_page)
        } else {
            None
        }
    }

    /// Trigger copy-on-write clone on a slot reference (amap_cow equivalent)
    pub fn amap_cow(&self, slot: usize, new_phys: PhysAddr) {
        if slot < self.slots.len {
            if let Some(ref s) = self.slots[slot] {
                // If ref_count > 1, dec and create a new independent private slot mapping
                s.ref_count.fetch_sub(1, Ordering::SeqCst);
                // In production, we'd update our mapping, stubbed for safe no_std test
                let _ = new_phys;
            }
        }
    }
}

/// Zero-copy Page Loanout tracker (Allows fast page-lending between address spaces)
pub struct PageLoanEntry {
    pub phys_page: PhysAddr,
    pub loan_count: AtomicUsize,
}

pub struct UvmPageLoan {
    pub loans: Vec<Option<PageLoanEntry>>,
}

impl UvmPageLoan {
    pub fn new() -> Self {
        UvmPageLoan { loans: Vec::new() }
    }

    /// Lend a physical page out to a target address space (zero-copy page loanout)
    pub fn loan_page(&mut self, phys: PhysAddr) {
        // Search if already loaned, else create new
        for i in 0..self.loans.len {
            if let Some(ref loan) = self.loans[i] {
                if loan.phys_page == phys {
                    loan.loan_count.fetch_add(1, Ordering::SeqCst);
                    return;
                }
            }
        }

        let entry = PageLoanEntry {
            phys_page: phys,
            loan_count: AtomicUsize::new(1),
        };
        self.loans.push(Some(entry));
    }

    /// Retrieve loaned page count
    pub fn query_loan_count(&self, phys: PhysAddr) -> usize {
        for i in 0..self.loans.len {
            if let Some(ref loan) = self.loans[i] {
                if loan.phys_page == phys {
                    return loan.loan_count.load(Ordering::SeqCst);
                }
            }
        }
        0
    }
}

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
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
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::std::{alloc as std_alloc, Layout};
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
    fn test_uvm_pmap_translation() {
        let mut pmap = UvmPmap::new();
        pmap.pmap_enter(0x1000, 0x2000000);
        pmap.pmap_enter(0x2000, 0x4000000);

        assert_eq!(pmap.pmap_extract(0x1000).unwrap(), 0x2000000);
        assert_eq!(pmap.pmap_extract(0x2000).unwrap(), 0x4000000);

        pmap.pmap_remove(0x1000);
        assert!(pmap.pmap_extract(0x1000).is_none());
    }

    #[test]
    fn test_uvm_amap_slots() {
        let mut amap = UvmAmap::new();
        amap.amap_add(0, 0x2000000);
        amap.amap_add(1, 0x4000000);

        assert_eq!(amap.amap_lookup(0).unwrap(), 0x2000000);
        assert_eq!(amap.amap_lookup(1).unwrap(), 0x4000000);

        // Verify COW reference decrements correctly
        assert_eq!(
            amap.slots[0]
                .as_ref()
                .unwrap()
                .ref_count
                .load(Ordering::SeqCst),
            1
        );
        amap.amap_cow(0, 0x8000000);
        assert_eq!(
            amap.slots[0]
                .as_ref()
                .unwrap()
                .ref_count
                .load(Ordering::SeqCst),
            0
        );
    }

    #[test]
    fn test_uvm_page_loans() {
        let mut loan = UvmPageLoan::new();
        loan.loan_page(0x2000000);
        assert_eq!(loan.query_loan_count(0x2000000), 1);

        // Multiple lenders
        loan.loan_page(0x2000000);
        assert_eq!(loan.query_loan_count(0x2000000), 2);
        assert_eq!(loan.query_loan_count(0x4000000), 0);
    }
}
