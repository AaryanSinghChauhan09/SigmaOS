// Virtual Memory & Paging Implementation
// 4-level paging architecture (PML4 → PDPT → PD → PT)
// Enhanced with Huge Pages (2MB/1GB), advanced page protection attributes,
// VMA demand paging simulation, and Clock (Second-Chance) replacement tracking.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub const PAGE_SIZE_BYTES: usize = 4096;
pub const PAGE_TABLE_ENTRIES: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    OutOfMemory,
    InvalidAddress,
    PageNotPresent,
    PermissionDenied,
    WriteToReadOnly,
    NonExecutablePage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub is_huge: bool,               // Page is 2MB or 1GB huge page
    pub execute_disable: bool,       // NX/XD bit
    pub cache_disable: bool,         // PCD bit
    pub write_through: bool,         // PWT bit
    pub dirty: bool,                 // D bit (written)
    pub accessed: bool,              // A bit (accessed)
    pub physical_address: PhysicalAddress,
}

impl PageTableEntry {
    pub fn new(phys: PhysicalAddress) -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: true,
            is_huge: false,
            execute_disable: false,
            cache_disable: false,
            write_through: false,
            dirty: false,
            accessed: false,
            physical_address: phys,
        }
    }

    pub fn with_attributes(
        phys: PhysicalAddress,
        writable: bool,
        is_huge: bool,
        execute_disable: bool,
    ) -> Self {
        Self {
            present: true,
            writable,
            user_accessible: true,
            is_huge,
            execute_disable,
            cache_disable: false,
            write_through: false,
            dirty: false,
            accessed: false,
            physical_address: phys,
        }
    }
}

#[derive(Clone)]
pub struct PageTable {
    pub entries: Vec<Option<PageTableEntry>>,
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_entry(&mut self, idx: usize, entry: PageTableEntry) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(entry);
        Ok(())
    }

    pub fn get_entry(&self, idx: usize) -> Option<&PageTableEntry> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_entry_mut(&mut self, idx: usize) -> Option<&mut PageTableEntry> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct PageDirectory {
    pub entries: Vec<Option<PageTable>>,
    pub huge_entries: Vec<Option<PageTableEntry>>, // Holds 2MB huge page entries
}

impl PageDirectory {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
            huge_entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_table(&mut self, idx: usize, table: PageTable) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(table);
        self.huge_entries[idx] = None; // clear huge mapping if any
        Ok(())
    }

    pub fn set_huge_entry(&mut self, idx: usize, entry: PageTableEntry) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.huge_entries[idx] = Some(entry);
        self.entries[idx] = None; // clear standard mapping if any
        Ok(())
    }

    pub fn get_table(&self, idx: usize) -> Option<&PageTable> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_table_mut(&mut self, idx: usize) -> Option<&mut PageTable> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }

    pub fn get_huge_entry(&self, idx: usize) -> Option<&PageTableEntry> {
        self.huge_entries.get(idx).and_then(|e| e.as_ref())
    }
}

impl Default for PageDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct PageDirectoryPointerTable {
    pub entries: Vec<Option<PageDirectory>>,
    pub huge_entries: Vec<Option<PageTableEntry>>, // Holds 1GB huge page entries
}

impl PageDirectoryPointerTable {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
            huge_entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_directory(&mut self, idx: usize, dir: PageDirectory) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(dir);
        self.huge_entries[idx] = None; // clear huge mapping if any
        Ok(())
    }

    pub fn set_huge_entry(&mut self, idx: usize, entry: PageTableEntry) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.huge_entries[idx] = Some(entry);
        self.entries[idx] = None; // clear standard mapping if any
        Ok(())
    }

    pub fn get_directory(&self, idx: usize) -> Option<&PageDirectory> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_directory_mut(&mut self, idx: usize) -> Option<&mut PageDirectory> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }

    pub fn get_huge_entry(&self, idx: usize) -> Option<&PageTableEntry> {
        self.huge_entries.get(idx).and_then(|e| e.as_ref())
    }
}

impl Default for PageDirectoryPointerTable {
    fn default() -> Self {
        Self::new()
    }
}

/// A Virtual Memory Area (VMA) describing a range of virtual memory for demand paging
#[derive(Debug, Clone)]
pub struct VirtualMemoryArea {
    pub start_address: u64,
    pub size: u64,
    pub is_writable: bool,
    pub is_executable: bool,
}

impl VirtualMemoryArea {
    pub fn contains(&self, address: u64) -> bool {
        address >= self.start_address && address < self.start_address + self.size
    }
}

pub struct SimpleVMM {
    pub pml4_table: Vec<Option<PageDirectoryPointerTable>>,
    pub vmas: Vec<VirtualMemoryArea>, // Virtual memory regions for demand paging
    pub active_pages_for_clock: Vec<VirtualAddress>, // Swapping tracker for Clock replacement
    pub clock_hand: usize,
}

impl SimpleVMM {
    pub fn new() -> Self {
        Self {
            pml4_table: vec![None; PAGE_TABLE_ENTRIES],
            vmas: Vec::new(),
            active_pages_for_clock: Vec::new(),
            clock_hand: 0,
        }
    }

    /// Add a Virtual Memory Area for demand paging
    pub fn register_vma(&mut self, vma: VirtualMemoryArea) {
        self.vmas.push(vma);
    }

    /// Maps a standard 4KB page
    pub fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
    ) -> Result<(), MemoryError> {
        self.map_page_with_flags(virt, phys, true, false)
    }

    /// Maps a standard 4KB page with flags
    pub fn map_page_with_flags(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
        execute_disable: bool,
    ) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        // Ensure PML4 entry exists
        if self.pml4_table[pml4_idx].is_none() {
            self.pml4_table[pml4_idx] = Some(PageDirectoryPointerTable::new());
        }

        let pml4 = self.pml4_table[pml4_idx].as_mut().unwrap();

        // Ensure PDPT entry exists
        if pml4.get_directory(pdpt_idx).is_none() {
            pml4.set_directory(pdpt_idx, PageDirectory::new())?;
        }

        let pdpt = pml4.get_directory_mut(pdpt_idx).unwrap();

        // Ensure PD entry exists
        if pdpt.get_table(pd_idx).is_none() {
            pdpt.set_table(pd_idx, PageTable::new())?;
        }

        let pd = pdpt.get_table_mut(pd_idx).unwrap();

        // Set the page table entry
        let pte = PageTableEntry::with_attributes(phys, writable, false, execute_disable);
        pd.set_entry(pt_idx, pte)?;

        // Register in active list for Clock paging tracker
        if !self.active_pages_for_clock.contains(&virt) {
            self.active_pages_for_clock.push(virt);
        }

        Ok(())
    }

    /// Maps a 2MB Huge Page (at the Page Directory level)
    pub fn map_huge_2mb(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
    ) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;

        // Ensure PML4 entry exists
        if self.pml4_table[pml4_idx].is_none() {
            self.pml4_table[pml4_idx] = Some(PageDirectoryPointerTable::new());
        }

        let pml4 = self.pml4_table[pml4_idx].as_mut().unwrap();

        // Ensure PDPT entry exists
        if pml4.get_directory(pdpt_idx).is_none() {
            pml4.set_directory(pdpt_idx, PageDirectory::new())?;
        }

        let pdpt = pml4.get_directory_mut(pdpt_idx).unwrap();

        // Set huge page entry on PD
        let pte = PageTableEntry::with_attributes(phys, writable, true, false);
        pdpt.set_huge_entry(pd_idx, pte)?;

        Ok(())
    }

    /// Maps a 1GB Huge Page (at the PDPT level)
    pub fn map_huge_1gb(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
        writable: bool,
    ) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;

        // Ensure PML4 entry exists
        if self.pml4_table[pml4_idx].is_none() {
            self.pml4_table[pml4_idx] = Some(PageDirectoryPointerTable::new());
        }

        let pml4 = self.pml4_table[pml4_idx].as_mut().unwrap();

        let pte = PageTableEntry::with_attributes(phys, writable, true, false);
        pml4.set_huge_entry(pdpt_idx, pte)?;

        Ok(())
    }

    /// Resolves virtual address to physical address, handling huge pages and demand paging VMAs
    pub fn get_physical_address(
        &self,
        virt: VirtualAddress,
    ) -> Result<PhysicalAddress, MemoryError> {
        self.get_physical_address_with_access(virt, false, false)
    }

    /// Resolves virtual address while validating and recording access permissions (Read/Write/Execute)
    pub fn get_physical_address_with_access(
        &self,
        virt: VirtualAddress,
        write_intent: bool,
        execute_intent: bool,
    ) -> Result<PhysicalAddress, MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table.get(pml4_idx)
            .and_then(|opt| opt.as_ref());

        if pml4.is_none() {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pml4 = pml4.unwrap();

        // 1GB Huge Page Check at PML4 level (points to PDPT huge entry)
        if let Some(huge_pte) = pml4.get_huge_entry(pdpt_idx) {
            self.validate_access(huge_pte, write_intent, execute_intent)?;
            let offset = virt.0 & 0x3FFF_FFFF; // 1GB offset
            return Ok(PhysicalAddress((huge_pte.physical_address.0 & !0x3FFF_FFFF) + offset));
        }

        let pdpt = pml4.get_directory(pdpt_idx);
        if pdpt.is_none() {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pdpt = pdpt.unwrap();

        // 2MB Huge Page Check at PDPT level (points to PD huge entry)
        if let Some(huge_pte) = pdpt.get_huge_entry(pd_idx) {
            self.validate_access(huge_pte, write_intent, execute_intent)?;
            let offset = virt.0 & 0x1F_FFFF; // 2MB offset
            return Ok(PhysicalAddress((huge_pte.physical_address.0 & !0x1F_FFFF) + offset));
        }

        let pd = pdpt.get_table(pd_idx);
        if pd.is_none() {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pd = pd.unwrap();
        let pte = pd.get_entry(pt_idx);
        if pte.is_none() {
            return self.attempt_demand_paging(virt, write_intent, execute_intent);
        }

        let pte = pte.unwrap();
        self.validate_access(pte, write_intent, execute_intent)?;

        // Compute 4KB physical offset
        let offset = virt.0 & 0xFFF;
        Ok(PhysicalAddress((pte.physical_address.0 & !0xFFF) + offset))
    }

    /// Support on-demand paging if address belongs to a registered VMA
    fn attempt_demand_paging(
        &self,
        virt: VirtualAddress,
        write_intent: bool,
        execute_intent: bool,
    ) -> Result<PhysicalAddress, MemoryError> {
        for vma in &self.vmas {
            if vma.contains(virt.0) {
                if write_intent && !vma.is_writable {
                    return Err(MemoryError::WriteToReadOnly);
                }
                if execute_intent && !vma.is_executable {
                    return Err(MemoryError::NonExecutablePage);
                }
                // Simulate successful on-demand mapping allocating standard physical page frame
                let mapped_phys = PhysicalAddress(virt.0); // Simple 1-to-1 map
                return Ok(mapped_phys);
            }
        }
        Err(MemoryError::PageNotPresent)
    }

    fn validate_access(
        &self,
        pte: &PageTableEntry,
        write_intent: bool,
        execute_intent: bool,
    ) -> Result<(), MemoryError> {
        if !pte.present {
            return Err(MemoryError::PageNotPresent);
        }
        if write_intent && !pte.writable {
            return Err(MemoryError::WriteToReadOnly);
        }
        if execute_intent && pte.execute_disable {
            return Err(MemoryError::NonExecutablePage);
        }
        Ok(())
    }

    /// Simulates page table pointer updates for Clock page-replacement.
    /// Iterates through mapped pages and returns the VirtualAddress evicted if limit exceeded.
    pub fn perform_clock_replacement_step(&mut self) -> Option<VirtualAddress> {
        if self.active_pages_for_clock.is_empty() {
            return None;
        }

        // Loop twice at max to guarantee finding an entry
        for _ in 0..(self.active_pages_for_clock.len() * 2) {
            let idx = self.clock_hand % self.active_pages_for_clock.len();
            let virt = self.active_pages_for_clock[idx];

            let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
            let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
            let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
            let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

            let mut traversed = false;
            if let Some(ref mut pml4) = self.pml4_table[pml4_idx] {
                if let Some(ref mut pdpt) = pml4.get_directory_mut(pdpt_idx) {
                    if let Some(ref mut pd) = pdpt.get_table_mut(pd_idx) {
                        if let Some(ref mut pte) = pd.get_entry_mut(pt_idx) {
                            traversed = true;
                            if pte.accessed {
                                // Give second chance
                                pte.accessed = false;
                                self.clock_hand += 1;
                            } else {
                                // Evict this page
                                let evicted = self.active_pages_for_clock.remove(idx);
                                self.clock_hand = idx; // Next start point
                                return Some(evicted);
                            }
                        }
                    }
                }
            }
            if !traversed {
                // If page table was not fully traversed, increment clock hand to prevent spinning
                self.clock_hand += 1;
            }
        }
        None
    }

    /// Marks a page table entry's accessed bit to simulate memory activities
    pub fn mark_accessed(&mut self, virt: VirtualAddress, accessed: bool) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table[pml4_idx].as_mut().ok_or(MemoryError::PageNotPresent)?;
        let pdpt = pml4.get_directory_mut(pdpt_idx).ok_or(MemoryError::PageNotPresent)?;
        let pd = pdpt.get_table_mut(pd_idx).ok_or(MemoryError::PageNotPresent)?;
        let pte = pd.get_entry_mut(pt_idx).ok_or(MemoryError::PageNotPresent)?;

        pte.accessed = accessed;
        Ok(())
    }

    pub fn unmap_page(&mut self, virt: VirtualAddress) -> Result<(), MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table[pml4_idx]
            .as_mut()
            .ok_or(MemoryError::PageNotPresent)?;
        let pdpt = pml4
            .get_directory_mut(pdpt_idx)
            .ok_or(MemoryError::PageNotPresent)?;
        let pd = pdpt.get_table_mut(pd_idx).ok_or(MemoryError::PageNotPresent)?;

        pd.entries[pt_idx] = None;

        if let Some(pos) = self.active_pages_for_clock.iter().position(|&x| x == virt) {
            self.active_pages_for_clock.remove(pos);
        }

        Ok(())
    }
}

impl Default for SimpleVMM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_table_entry() {
        let phys = PhysicalAddress(0x1000);
        let entry = PageTableEntry::new(phys);

        assert!(entry.present);
        assert!(entry.writable);
        assert_eq!(entry.physical_address.0, 0x1000);
    }

    #[test]
    fn test_page_table() {
        let mut pt = PageTable::new();
        let entry = PageTableEntry::new(PhysicalAddress(0x1000));

        pt.set_entry(0, entry).unwrap();
        assert!(pt.get_entry(0).is_some());
    }

    #[test]
    fn test_page_directory() {
        let mut pd = PageDirectory::new();
        let pt = PageTable::new();

        pd.set_table(0, pt).unwrap();
        assert!(pd.get_table(0).is_some());
    }

    #[test]
    fn test_vmm_map_page() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        let phys = PhysicalAddress(0x2000);

        vmm.map_page(virt, phys).unwrap();
        let resolved = vmm.get_physical_address(virt).unwrap();

        assert_eq!(resolved.0, 0x2000);
    }

    #[test]
    fn test_vmm_unmap_page() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x1000);
        let phys = PhysicalAddress(0x2000);

        vmm.map_page(virt, phys).unwrap();
        vmm.unmap_page(virt).unwrap();

        assert!(vmm.get_physical_address(virt).is_err());
    }

    #[test]
    fn test_vmm_huge_page_2mb() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x200000); // 2MB border
        let phys = PhysicalAddress(0x800000);

        vmm.map_huge_2mb(virt, phys, true).unwrap();
        let resolved = vmm.get_physical_address(VirtualAddress(0x200000 + 0x1000)).unwrap();

        // Must translate using base + 4KB offset
        assert_eq!(resolved.0, 0x800000 + 0x1000);
    }

    #[test]
    fn test_vmm_huge_page_1gb() {
        let mut vmm = SimpleVMM::new();
        let virt = VirtualAddress(0x40000000); // 1GB border
        let phys = PhysicalAddress(0xC0000000);

        vmm.map_huge_1gb(virt, phys, true).unwrap();
        let resolved = vmm.get_physical_address(VirtualAddress(0x40000000 + 0x200000)).unwrap();

        // Must translate using base + 2MB offset
        assert_eq!(resolved.0, 0xC0000000 + 0x200000);
    }

    #[test]
    fn test_demand_paging_vma() {
        let mut vmm = SimpleVMM::new();
        let vma = VirtualMemoryArea {
            start_address: 0x5000000,
            size: 0x100000,
            is_writable: true,
            is_executable: true,
        };

        vmm.register_vma(vma);

        // Address doesn't exist in page tables but fits in VMA -> Simulated demand allocation
        let resolved = vmm.get_physical_address_with_access(VirtualAddress(0x5020000), true, false).unwrap();
        assert_eq!(resolved.0, 0x5020000);

        // Accessing outside registered VMA fails
        assert!(vmm.get_physical_address_with_access(VirtualAddress(0x7000000), false, false).is_err());
    }

    #[test]
    fn test_clock_page_replacement() {
        let mut vmm = SimpleVMM::new();
        let page1 = VirtualAddress(0x1000);
        let page2 = VirtualAddress(0x2000);

        vmm.map_page(page1, PhysicalAddress(0x10000)).unwrap();
        vmm.map_page(page2, PhysicalAddress(0x20000)).unwrap();

        // Mark Page 1 accessed, Page 2 not accessed
        vmm.mark_accessed(page1, true).unwrap();
        vmm.mark_accessed(page2, false).unwrap();

        // Clock replacement should give Page 1 a second chance, evicting Page 2
        let evicted = vmm.perform_clock_replacement_step().unwrap();
        assert_eq!(evicted, page2);
    }

    #[test]
    fn test_invalid_index() {
        let mut pt = PageTable::new();
        let entry = PageTableEntry::new(PhysicalAddress(0x1000));

        assert!(pt.set_entry(512, entry).is_err());
    }

    #[test]
    fn test_vmm_address_alignment_verification() {
        let mut vmm = SimpleVMM::new();

        // 4KB alignment checks
        assert!(vmm.map_page(VirtualAddress(0x1005), PhysicalAddress(0x2000)).is_err());
        assert!(vmm.map_page(VirtualAddress(0x1000), PhysicalAddress(0x2003)).is_err());
        assert!(vmm.map_page(VirtualAddress(0x1000), PhysicalAddress(0x2000)).is_ok());

        // 2MB alignment checks
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200100), PhysicalAddress(0x800000), true).is_err());
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200000), PhysicalAddress(0x800100), true).is_err());
        assert!(vmm.map_huge_2mb(VirtualAddress(0x200000), PhysicalAddress(0x800000), true).is_ok());

        // 1GB alignment checks
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40001000), PhysicalAddress(0xC0000000), true).is_err());
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40000000), PhysicalAddress(0xC0001000), true).is_err());
        assert!(vmm.map_huge_1gb(VirtualAddress(0x40000000), PhysicalAddress(0xC0000000), true).is_ok());
    }
}
