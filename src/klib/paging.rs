// SigmaOS Page Directory & Table Entry primitives
// Based on 100-Improvement-Ideas.md #44: demand paging and page fault handler

use core::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultError {
    PageNotPresent,
    ProtectionViolation,
    WriteViolation,
    InvalidAddress,
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
}

#[repr(C)]
#[derive(Debug)]
pub struct SimplePageTableEntry {
    pub flags: AtomicUsize,
    pub physical_addr: AtomicUsize,
    pub accessed: AtomicUsize,
    pub dirty: AtomicUsize,
}

impl SimplePageTableEntry {
    pub fn new() -> Self {
        SimplePageTableEntry {
            flags: AtomicUsize::new(0),
            physical_addr: AtomicUsize::new(0),
            accessed: AtomicUsize::new(0),
            dirty: AtomicUsize::new(0),
        }
    }
}

impl PageTableEntry for SimplePageTableEntry {
    fn is_present(&self) -> bool {
        (self.flags.load(Ordering::SeqCst) & 1) != 0
    }
    fn is_writable(&self) -> bool {
        (self.flags.load(Ordering::SeqCst) & 2) != 0
    }
    fn is_user_accessible(&self) -> bool {
        (self.flags.load(Ordering::SeqCst) & 4) != 0
    }
    fn is_cow(&self) -> bool {
        (self.flags.load(Ordering::SeqCst) & 8) != 0
    }
    fn get_physical_address(&self) -> PhysicalAddress {
        self.physical_addr.load(Ordering::SeqCst) as PhysicalAddress
    }
    fn set_present(&mut self, present: bool) {
        if present {
            self.flags.fetch_or(1, Ordering::SeqCst);
        } else {
            self.flags.fetch_and(!1, Ordering::SeqCst);
        }
    }
    fn set_writable(&mut self, writable: bool) {
        if writable {
            self.flags.fetch_or(2, Ordering::SeqCst);
        } else {
            self.flags.fetch_and(!2, Ordering::SeqCst);
        }
    }
    fn set_user_accessible(&mut self, user: bool) {
        if user {
            self.flags.fetch_or(4, Ordering::SeqCst);
        } else {
            self.flags.fetch_and(!4, Ordering::SeqCst);
        }
    }
    fn set_cow(&mut self, cow: bool) {
        if cow {
            self.flags.fetch_or(8, Ordering::SeqCst);
        } else {
            self.flags.fetch_and(!8, Ordering::SeqCst);
        }
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
    fn get_entry(&self, index: usize) -> &dyn PageTableEntry;
    fn get_entry_mut(&mut self, index: usize) -> &mut dyn PageTableEntry;
    fn set_entry(&mut self, index: usize, entry: SimplePageTableEntry);
    fn clear_entry(&mut self, index: usize);
}

pub struct SimplePageTable {
    pub entries: Vec<SimplePageTableEntry>,
    pub physical_address: PhysicalAddress,
}

impl SimplePageTable {
    pub fn new(phys_addr: PhysicalAddress) -> Self {
        let mut entries = Vec::new();
        for _ in 0..512 {
            entries.push(SimplePageTableEntry::new());
        }
        SimplePageTable {
            entries,
            physical_address: phys_addr,
        }
    }
}

impl PageTable for SimplePageTable {
    fn get_entry(&self, index: usize) -> &dyn PageTableEntry {
        &self.entries[index]
    }
    fn get_entry_mut(&mut self, index: usize) -> &mut dyn PageTableEntry {
        &mut self.entries[index]
    }
    fn set_entry(&mut self, index: usize, entry: SimplePageTableEntry) {
        self.entries[index] = entry;
    }
    fn clear_entry(&mut self, index: usize) {
        self.entries[index] = SimplePageTableEntry::new();
    }
}

pub trait VirtualMemoryManager {
    fn map_page(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool) -> Result<(), PageFaultError> {
        self.map_page_with_size(virt, phys, user, writable, 4096)
    }
    fn map_page_with_size(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool, page_size: usize) -> Result<(), PageFaultError>;
    fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError>;
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress>;
    fn mark_copy_on_write(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError>;
    fn handle_page_fault(&mut self, fault_addr: VirtualAddress, present: bool, write_attempt: bool) -> Result<(), PageFaultError>;
}

pub struct SimpleVMM {
    pub pml4_table: SimplePageTable,
    pub pdpt_tables: Vec<Option<SimplePageTable>>,
    pub pd_tables: Vec<Option<SimplePageTable>>,
    pub pt_tables: Vec<Option<SimplePageTable>>,
    pub next_table_addr: AtomicUsize,
    pub allocated_physical_pages: Vec<PhysicalAddress>,
}

impl SimpleVMM {
    pub fn new(pml4_phys: PhysicalAddress, starting_pool_phys: PhysicalAddress) -> Self {
        let mut pdpt_tables = Vec::new();
        let mut pd_tables = Vec::new();
        let mut pt_tables = Vec::new();
        for _ in 0..16 {
            pdpt_tables.push(None);
            pd_tables.push(None);
            pt_tables.push(None);
        }
        SimpleVMM {
            pml4_table: SimplePageTable::new(pml4_phys),
            pdpt_tables,
            pd_tables,
            pt_tables,
            next_table_addr: AtomicUsize::new(starting_pool_phys as usize),
            allocated_physical_pages: Vec::new(),
        }
    }

    fn get_pml4_index(&self, addr: VirtualAddress) -> usize {
        ((addr >> 39) & 0x1FF) as usize
    }

    fn get_pdpt_index(&self, addr: VirtualAddress) -> usize {
        ((addr >> 30) & 0x1FF) as usize
    }

    fn get_pd_index(&self, addr: VirtualAddress) -> usize {
        ((addr >> 21) & 0x1FF) as usize
    }

    fn get_pt_index(&self, addr: VirtualAddress) -> usize {
        ((addr >> 12) & 0x1FF) as usize
    }
}

impl VirtualMemoryManager for SimpleVMM {
    fn map_page_with_size(&mut self, virt: VirtualAddress, phys: PhysicalAddress, user: bool, writable: bool, page_size: usize) -> Result<(), PageFaultError> {
        // Enforce strict address alignment verification (must be page-aligned to its size)
        if page_size != 4096 && page_size != 2097152 && page_size != 1073741824 {
            return Err(PageFaultError::InvalidAddress);
        }
        if virt % page_size as u64 != 0 || phys % page_size as u64 != 0 {
            return Err(PageFaultError::InvalidAddress);
        }
        let pml4_idx = self.get_pml4_index(virt);
        let pdpt_idx = self.get_pdpt_index(virt);
        let pd_idx = self.get_pd_index(virt);
        let pt_idx = self.get_pt_index(virt);

        // Ensure PML4 entry is present
        {
            let mut entry = SimplePageTableEntry::new();
            entry.set_present(true);
            entry.set_writable(true);
            entry.set_user_accessible(user);
            let target_addr = self.pml4_table.physical_address + 0x1000;
            entry.set_physical_address(target_addr);
            self.pml4_table.set_entry(pml4_idx, entry);
        }

        let pdpt_idx_in_vec = pml4_idx;
        while self.pdpt_tables.len() <= pdpt_idx_in_vec {
            self.pdpt_tables.push(None);
        }
        if self.pdpt_tables[pdpt_idx_in_vec].is_none() {
            let pdpt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
            let pdpt_table = SimplePageTable::new(pdpt_phys);
            self.pdpt_tables[pdpt_idx_in_vec] = Some(pdpt_table);
        }

        let pdpt_present = if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
            pdpt.get_entry(pdpt_idx).is_present()
        } else {
            false
        };

        if !pdpt_present {
            if let Some(ref mut pdpt) = self.pdpt_tables[pdpt_idx_in_vec] {
                let mut entry = SimplePageTableEntry::new();
                entry.set_present(true);
                entry.set_writable(true);
                entry.set_user_accessible(user);
                let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
                entry.set_physical_address(pd_phys);
                pdpt.set_entry(pdpt_idx, entry);
            }
        }

        // 2MB Huge Page support: mapped directly at PD level
        if page_size == 2097152 {
            let pd_idx_in_vec = pdpt_idx;
            while self.pd_tables.len() <= pd_idx_in_vec {
                self.pd_tables.push(None);
            }
            if self.pd_tables[pd_idx_in_vec].is_none() {
                let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
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
        while self.pd_tables.len() <= pd_idx_in_vec {
            self.pd_tables.push(None);
        }
        if self.pd_tables[pd_idx_in_vec].is_none() {
            let pd_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
            let pd_table = SimplePageTable::new(pd_phys);
            self.pd_tables[pd_idx_in_vec] = Some(pd_table);
        }

        let pd_present = if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
            pd.get_entry(pd_idx).is_present()
        } else {
            false
        };

        if !pd_present {
            if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
                let mut entry = SimplePageTableEntry::new();
                entry.set_present(true);
                entry.set_writable(true);
                entry.set_user_accessible(user);
                let pt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
                entry.set_physical_address(pt_phys);
                pd.set_entry(pd_idx, entry);
            }
        }

        let pt_idx_in_vec = pd_idx;
        while self.pt_tables.len() <= pt_idx_in_vec {
            self.pt_tables.push(None);
        }
        if self.pt_tables[pt_idx_in_vec].is_none() {
            let pt_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
            let pt_table = SimplePageTable::new(pt_phys);
            self.pt_tables[pt_idx_in_vec] = Some(pt_table);
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
        let pd_idx_in_vec = self.get_pdpt_index(virt);
        if let Some(ref mut pd) = self.pd_tables[pd_idx_in_vec] {
            let pd_idx = self.get_pd_index(virt);
            if pd.get_entry(pd_idx).get_page_size() == 2097152 {
                pd.clear_entry(pd_idx);
                return Ok(());
            }
        }

        let pt_idx_in_vec = self.get_pd_index(virt);
        if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
            let pt_idx = self.get_pt_index(virt);
            pt.clear_entry(pt_idx);
            Ok(())
        } else {
            Err(PageFaultError::PageNotPresent)
        }
    }
    
    fn get_physical(&self, virt: VirtualAddress) -> Option<PhysicalAddress> {
        let pd_idx_in_vec = self.get_pdpt_index(virt);
        if let Some(ref pd) = self.pd_tables[pd_idx_in_vec] {
            let pd_idx = self.get_pd_index(virt);
            let entry = pd.get_entry(pd_idx);
            if entry.is_present() && entry.get_page_size() == 2097152 {
                return Some(entry.get_physical_address() + (virt % 2097152));
            }
        }

        let pt_idx_in_vec = self.get_pd_index(virt);
        if let Some(ref pt) = self.pt_tables[pt_idx_in_vec] {
            let pt_idx = self.get_pt_index(virt);
            let entry = pt.get_entry(pt_idx);
            if entry.is_present() {
                return Some(entry.get_physical_address() + (virt % 4096));
            }
        }
        None
    }

    fn mark_copy_on_write(&mut self, virt: VirtualAddress) -> Result<(), PageFaultError> {
        let pt_idx_in_vec = self.get_pd_index(virt);
        if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
            let pt_idx = self.get_pt_index(virt);
            let mut entry = pt.entries[pt_idx].flags.load(Ordering::SeqCst);
            entry &= !2; // Remove write bit
            entry |= 8;  // Add Copy-on-Write bit
            pt.entries[pt_idx].flags.store(entry, Ordering::SeqCst);
            Ok(())
        } else {
            Err(PageFaultError::PageNotPresent)
        }
    }

    fn handle_page_fault(&mut self, fault_addr: VirtualAddress, present: bool, write_attempt: bool) -> Result<(), PageFaultError> {
        if !present {
            // Allocate physical page frame dynamically (simulate demand paging)
            let new_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
            self.allocated_physical_pages.push(new_phys);
            self.map_page(fault_addr & !0xFFF, new_phys, true, true)?;
            return Ok(());
        }

        if write_attempt {
            let pt_idx_in_vec = self.get_pd_index(fault_addr);
            if let Some(ref mut pt) = self.pt_tables[pt_idx_in_vec] {
                let pt_idx = self.get_pt_index(fault_addr);
                let entry = pt.get_entry(pt_idx);
                if entry.is_cow() {
                    // Resolve Copy-on-Write page fault
                    let orig_phys = entry.get_physical_address();
                    let new_phys = self.next_table_addr.fetch_add(0x1000, Ordering::SeqCst) as u64;
                    self.allocated_physical_pages.push(new_phys);

                    // Simulate zero-copy page cloning
                    println!("COW: Cloning page frame at {:X} to {:X}", orig_phys, new_phys);

                    let mut new_entry = SimplePageTableEntry::new();
                    new_entry.set_present(true);
                    new_entry.set_writable(true);
                    new_entry.set_user_accessible(true);
                    new_entry.set_physical_address(new_phys);
                    pt.set_entry(pt_idx, new_entry);
                    return Ok(());
                } else {
                    return Err(PageFaultError::WriteViolation);
                }
            }
        }

        Err(PageFaultError::ProtectionViolation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_bit_manipulation() {
        let mut entry = SimplePageTableEntry::new();
        assert!(!entry.is_present());
        entry.set_present(true);
        assert!(entry.is_present());

        assert!(!entry.is_writable());
        entry.set_writable(true);
        assert!(entry.is_writable());

        assert!(!entry.is_cow());
        entry.set_cow(true);
        assert!(entry.is_cow());

        entry.set_physical_address(0x7FFFFFFF000);
        assert_eq!(entry.get_physical_address(), 0x7FFFFFFF000);
    }

    #[test]
    fn test_vmm_mapping_flow() {
        let mut vmm = SimpleVMM::new(0x1000, 0x10000);
        assert!(vmm.get_physical(0x1000).is_none());

        vmm.map_page(0x1000, 0x50000, true, true).unwrap();
        assert_eq!(vmm.get_physical(0x1000), Some(0x50000));
        assert_eq!(vmm.get_physical(0x100F), Some(0x5000F));

        vmm.unmap_page(0x1000).unwrap();
        assert!(vmm.get_physical(0x1000).is_none());
    }

    #[test]
    fn test_vmm_huge_page_mapping_flow() {
        let mut vmm = SimpleVMM::new(0x1000, 0x10000);
        assert!(vmm.get_physical(0x200000).is_none());

        // Map 2MB Huge Page
        vmm.map_page_with_size(0x200000, 0x400000, true, true, 2097152).unwrap();
        assert_eq!(vmm.get_physical(0x200000), Some(0x400000));
        assert_eq!(vmm.get_physical(0x2000FF), Some(0x4000FF));

        vmm.unmap_page(0x200000).unwrap();
        assert!(vmm.get_physical(0x200000).is_none());
    }

    #[test]
    fn test_vmm_demand_paging_and_cow() {
        let mut vmm = SimpleVMM::new(0x1000, 0x10000);
        // Page fault on not-present address -> demand allocates a new page frame
        vmm.handle_page_fault(0x3000, false, false).unwrap();
        assert!(vmm.get_physical(0x3000).is_some());

        // Mark Copy-on-Write
        vmm.mark_copy_on_write(0x3000).unwrap();
        let phys_before = vmm.get_physical(0x3000).unwrap();

        // Write attempt on Copy-on-Write page triggers copy/cloning allocation
        vmm.handle_page_fault(0x3000, true, true).unwrap();
        let phys_after = vmm.get_physical(0x3000).unwrap();
        assert_ne!(phys_before, phys_after);
    }
}
