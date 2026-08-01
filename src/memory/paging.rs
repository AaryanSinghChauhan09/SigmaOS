// Virtual Memory & Paging Implementation
// 4-level paging architecture (PML4 → PDPT → PD → PT)

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAddress(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub physical_address: PhysicalAddress,
}

impl PageTableEntry {
    pub fn new(phys: PhysicalAddress) -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: true,
            physical_address: phys,
        }
    }
}

#[derive(Clone)]
#[derive(Clone)]
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
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[derive(Clone)]
#[derive(Clone)]
pub struct PageDirectory {
    pub entries: Vec<Option<PageTable>>,
}

impl PageDirectory {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_table(&mut self, idx: usize, table: PageTable) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(table);
        Ok(())
    }

    pub fn get_table(&self, idx: usize) -> Option<&PageTable> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_table_mut(&mut self, idx: usize) -> Option<&mut PageTable> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }
}

impl Default for PageDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[derive(Clone)]
#[derive(Clone)]
pub struct PageDirectoryPointerTable {
    pub entries: Vec<Option<PageDirectory>>,
}

impl PageDirectoryPointerTable {
    pub fn new() -> Self {
        Self {
            entries: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn set_directory(&mut self, idx: usize, dir: PageDirectory) -> Result<(), MemoryError> {
        if idx >= PAGE_TABLE_ENTRIES {
            return Err(MemoryError::InvalidAddress);
        }
        self.entries[idx] = Some(dir);
        Ok(())
    }

    pub fn get_directory(&self, idx: usize) -> Option<&PageDirectory> {
        self.entries.get(idx).and_then(|e| e.as_ref())
    }

    pub fn get_directory_mut(&mut self, idx: usize) -> Option<&mut PageDirectory> {
        self.entries.get_mut(idx).and_then(|e| e.as_mut())
    }
}

impl Default for PageDirectoryPointerTable {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SimpleVMM {
    pub pml4_table: Vec<Option<PageDirectoryPointerTable>>,
}

impl SimpleVMM {
    pub fn new() -> Self {
        Self {
            pml4_table: vec![None; PAGE_TABLE_ENTRIES],
        }
    }

    pub fn map_page(
        &mut self,
        virt: VirtualAddress,
        phys: PhysicalAddress,
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
        let pte = PageTableEntry::new(phys);
        pd.set_entry(pt_idx, pte)?;

        Ok(())
    }

    pub fn get_physical_address(
        &self,
        virt: VirtualAddress,
    ) -> Result<PhysicalAddress, MemoryError> {
        let pml4_idx = ((virt.0 >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt.0 >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt.0 >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt.0 >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table[pml4_idx]
            .as_ref()
            .ok_or(MemoryError::PageNotPresent)?;
        let pdpt = pml4
            .get_directory(pdpt_idx)
            .ok_or(MemoryError::PageNotPresent)?;
        let pd = pdpt.get_table(pd_idx).ok_or(MemoryError::PageNotPresent)?;
        let pte = pd.get_entry(pt_idx).ok_or(MemoryError::PageNotPresent)?;

        Ok(pte.physical_address)
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
    fn test_virtual_address_indices() {
        let virt = VirtualAddress(0x123456789ABC);

        let pml4_idx = (virt.0 >> 39) & 0x1FF;
        let pdpt_idx = (virt.0 >> 30) & 0x1FF;
        let pd_idx = (virt.0 >> 21) & 0x1FF;
        let pt_idx = (virt.0 >> 12) & 0x1FF;

        assert!(pml4_idx < 512);
        assert!(pdpt_idx < 512);
        assert!(pd_idx < 512);
        assert!(pt_idx < 512);
    }

    #[test]
    fn test_invalid_index() {
        let mut pt = PageTable::new();
        let entry = PageTableEntry::new(PhysicalAddress(0x1000));

        assert!(pt.set_entry(512, entry).is_err());
    }
}
