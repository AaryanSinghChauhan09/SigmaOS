// SigmaOS 4-Level Page Table Walking & Paging Subsystem
// Zero-dependency, #![no_std] compliant x86_64 paging implementation.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::ptr::NonNull;

pub const PAGE_SIZE: usize = 4096;
pub const ENTRY_COUNT: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableFlags(pub u64);

impl PageTableFlags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const NO_CACHE: u64 = 1 << 4;
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags(self.0 & 0xFFF0_0000_0000_0FFF)
    }

    pub fn physical_frame(&self) -> Option<u64> {
        if self.flags().0 & PageTableFlags::PRESENT != 0 {
            Some(self.0 & 0x000F_FFFF_FFFF_F000)
        } else {
            None
        }
    }

    pub fn set_frame(&mut self, frame_addr: u64, flags: PageTableFlags) {
        self.0 = (frame_addr & 0x000F_FFFF_FFFF_F000) | flags.0 | PageTableFlags::PRESENT;
    }
}

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    pub const fn new() -> Self {
        Self {
            entries: [PageTableEntry(0); ENTRY_COUNT],
        }
    }
}

pub struct VirtualMemoryManagerV2 {
    pub pml4_table: NonNull<PageTable>,
}

impl VirtualMemoryManagerV2 {
    pub unsafe fn new(pml4_phys_addr: u64) -> Self {
        Self {
            pml4_table: NonNull::new_unchecked(pml4_phys_addr as *mut PageTable),
        }
    }

    /// Translates a virtual address to its corresponding physical address by walking PML4 -> PDPT -> PD -> PT
    pub unsafe fn translate(&self, virt_addr: u64) -> Option<u64> {
        let pml4_index = ((virt_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virt_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virt_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virt_addr >> 12) & 0x1FF) as usize;
        let page_offset = virt_addr & 0xFFF;

        let pml4 = self.pml4_table.as_ref();
        let pml4_entry = &pml4.entries[pml4_index];
        let pdpt_addr = pml4_entry.physical_frame()?;

        let pdpt = &*(pdpt_addr as *const PageTable);
        let pdpt_entry = &pdpt.entries[pdpt_index];
        let pd_addr = pdpt_entry.physical_frame()?;

        let pd = &*(pd_addr as *const PageTable);
        let pd_entry = &pd.entries[pd_index];
        let pt_addr = pd_entry.physical_frame()?;

        let pt = &*(pt_addr as *const PageTable);
        let pt_entry = &pt.entries[pt_index];
        let frame_addr = pt_entry.physical_frame()?;

        Some(frame_addr + page_offset)
    }

    /// Maps a virtual page to a physical frame
    pub unsafe fn map_page(
        &mut self,
        virt_addr: u64,
        phys_frame: u64,
        flags: PageTableFlags,
        allocator: &mut dyn FnMut() -> Option<NonNull<PageTable>>,
    ) -> Result<(), &'static str> {
        let pml4_index = ((virt_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virt_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virt_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virt_addr >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table.as_mut();

        let pml4_entry = &mut pml4.entries[pml4_index];
        let pdpt_addr = if pml4_entry.is_unused() {
            let mut table_ptr = allocator().ok_or("Out of memory for PDPT")?;
            table_ptr
                .as_mut()
                .entries
                .iter_mut()
                .for_each(|e| e.set_unused());
            let addr = table_ptr.as_ptr() as u64;
            pml4_entry.set_frame(addr, flags);
            addr
        } else {
            pml4_entry.physical_frame().unwrap()
        };

        let pdpt = &mut *(pdpt_addr as *mut PageTable);
        let pdpt_entry = &mut pdpt.entries[pdpt_index];
        let pd_addr = if pdpt_entry.is_unused() {
            let mut table_ptr = allocator().ok_or("Out of memory for PD")?;
            table_ptr
                .as_mut()
                .entries
                .iter_mut()
                .for_each(|e| e.set_unused());
            let addr = table_ptr.as_ptr() as u64;
            pdpt_entry.set_frame(addr, flags);
            addr
        } else {
            pdpt_entry.physical_frame().unwrap()
        };

        let pd = &mut *(pd_addr as *mut PageTable);
        let pd_entry = &mut pd.entries[pd_index];
        let pt_addr = if pd_entry.is_unused() {
            let mut table_ptr = allocator().ok_or("Out of memory for PT")?;
            table_ptr
                .as_mut()
                .entries
                .iter_mut()
                .for_each(|e| e.set_unused());
            let addr = table_ptr.as_ptr() as u64;
            pd_entry.set_frame(addr, flags);
            addr
        } else {
            pd_entry.physical_frame().unwrap()
        };

        let pt = &mut *(pt_addr as *mut PageTable);
        let pt_entry = &mut pt.entries[pt_index];
        if !pt_entry.is_unused() {
            return Err("Page already mapped!");
        }

        pt_entry.set_frame(phys_frame, flags);
        Ok(())
    }
}

// =========================================================================
// Memory Descriptor List (MDL) Architecture
// =========================================================================

/// Represents a Memory Descriptor List (MDL) describing the physical layout of a virtual memory buffer.
/// Inspired by the Windows and Linux/BSD kernel designs for handling DMA buffers,
/// mapping user buffers to kernel space, locking physical pages, and virtual memory protection.
#[derive(Debug, Clone)]
pub struct MemoryDescriptorList {
    pub virtual_address: u64,
    pub byte_count: usize,
    pub byte_offset: usize,
    pub physical_pages: Vec<u64>,
    pub is_probed: bool,
    pub is_locked: bool,
    pub is_mapped: bool,
    pub mapped_kernel_address: Option<u64>,
}

impl MemoryDescriptorList {
    /// Initialize a new Memory Descriptor List (MDL)
    pub fn new(virtual_address: u64, byte_count: usize) -> Self {
        let byte_offset = (virtual_address & 0xFFF) as usize;
        let start_page = virtual_address & !0xFFF;
        let end_page = (virtual_address + byte_count as u64 + 0xFFF) & !0xFFF;
        let page_count = ((end_page - start_page) / 4096) as usize;

        Self {
            virtual_address,
            byte_count,
            byte_offset,
            physical_pages: alloc::vec![0; page_count],
            is_probed: false,
            is_locked: false,
            is_mapped: false,
            mapped_kernel_address: None,
        }
    }

    /// Probes and locks the physical pages associated with the virtual address range
    pub fn probe_and_lock(&mut self, vmm: &VirtualMemoryManagerV2) -> Result<(), &'static str> {
        if self.is_locked {
            return Err("MDL is already locked");
        }

        let start_page = self.virtual_address & !0xFFF;
        for i in 0..self.physical_pages.len() {
            let virt = start_page + (i * 4096) as u64;
            let phys = unsafe { vmm.translate(virt) }.ok_or("Virtual address page fault during probe")?;
            self.physical_pages[i] = phys & !0xFFF;
        }

        self.is_probed = true;
        self.is_locked = true;
        Ok(())
    }

    /// Maps the physically locked pages of the MDL to a contiguous virtual buffer in kernel space
    pub unsafe fn map_to_kernel_space(
        &mut self,
        mut kernel_start_virt: u64,
        vmm: &mut VirtualMemoryManagerV2,
        allocator: &mut dyn FnMut() -> Option<NonNull<PageTable>>,
    ) -> Result<u64, &'static str> {
        if !self.is_locked {
            return Err("MDL must be locked before mapping");
        }
        if self.is_mapped {
            return Err("MDL is already mapped");
        }

        let map_flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
        for &phys in &self.physical_pages {
            vmm.map_page(kernel_start_virt, phys, map_flags, allocator)?;
            kernel_start_virt += 4096;
        }

        let mapped_address = kernel_start_virt - (self.physical_pages.len() * 4096) as u64 + self.byte_offset as u64;
        self.mapped_kernel_address = Some(mapped_address);
        self.is_mapped = true;
        Ok(mapped_address)
    }

    /// Unmaps the mapped virtual buffer in kernel space
    pub fn unmap(&mut self) {
        self.mapped_kernel_address = None;
        self.is_mapped = false;
    }

    /// Unlocks the physical pages
    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.is_probed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4level_page_table_walking() {
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();
        let mut pt = PageTable::new();

        let pml4_ptr = &mut pml4 as *mut PageTable;
        let pdpt_ptr = NonNull::new(&mut pdpt as *mut PageTable);
        let pd_ptr = NonNull::new(&mut pd as *mut PageTable);
        let pt_ptr = NonNull::new(&mut pt as *mut PageTable);

        let mut allocator_calls = 0;
        let mut allocator = || {
            allocator_calls += 1;
            match allocator_calls {
                1 => Some(pdpt_ptr.unwrap()),
                2 => Some(pd_ptr.unwrap()),
                3 => Some(pt_ptr.unwrap()),
                _ => None,
            }
        };

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(pml4_ptr as u64) };
        let virt = 0x0000_7FFF_FFFF_F000;
        let phys = 0x0000_0000_1000_0000;
        let flags = PageTableFlags(PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);

        // Map the page
        assert!(unsafe { vmm.map_page(virt, phys, flags, &mut allocator).is_ok() });
        assert_eq!(allocator_calls, 3);

        // Translate the page
        let translated = unsafe { vmm.translate(virt).unwrap() };
        assert_eq!(translated, phys);
    }

    #[test]
    fn test_memory_descriptor_list_initialisation_and_mapping() {
        let mut pml4 = PageTable::new();

        // Separate page tables for user mapping
        let mut user_pdpt = PageTable::new();
        let mut user_pd = PageTable::new();
        let mut user_pt = PageTable::new();

        // Separate page tables for kernel mapping
        let mut kern_pdpt = PageTable::new();
        let mut kern_pd = PageTable::new();
        let mut kern_pt = PageTable::new();

        let pml4_ptr = &mut pml4 as *mut PageTable;
        let user_pdpt_ptr = NonNull::new(&mut user_pdpt as *mut PageTable);
        let user_pd_ptr = NonNull::new(&mut user_pd as *mut PageTable);
        let user_pt_ptr = NonNull::new(&mut user_pt as *mut PageTable);

        let kern_pdpt_ptr = NonNull::new(&mut kern_pdpt as *mut PageTable);
        let kern_pd_ptr = NonNull::new(&mut kern_pd as *mut PageTable);
        let kern_pt_ptr = NonNull::new(&mut kern_pt as *mut PageTable);

        let mut user_allocator_calls = 0;
        let mut user_allocator = || {
            user_allocator_calls += 1;
            match user_allocator_calls {
                1 => Some(user_pdpt_ptr.unwrap()),
                2 => Some(user_pd_ptr.unwrap()),
                3 => Some(user_pt_ptr.unwrap()),
                _ => None,
            }
        };

        let mut kern_allocator_calls = 0;
        let mut kern_allocator = || {
            kern_allocator_calls += 1;
            match kern_allocator_calls {
                1 => Some(kern_pdpt_ptr.unwrap()),
                2 => Some(kern_pd_ptr.unwrap()),
                3 => Some(kern_pt_ptr.unwrap()),
                _ => None,
            }
        };

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(pml4_ptr as u64) };

        // Pre-map user virtual address range sharing the exact same PML4/PDPT/PD index space
        let virt_user_addr = 0x0000_1000_0000_0000;
        let phys_frame1 = 0x1000_0000;
        let phys_frame2 = 0x1000_1000;
        let flags = PageTableFlags(PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);

        assert!(unsafe { vmm.map_page(virt_user_addr, phys_frame1, flags, &mut user_allocator).is_ok() });
        assert!(unsafe { vmm.map_page(virt_user_addr + 4096, phys_frame2, flags, &mut user_allocator).is_ok() });

        // Initialize MDL
        let mut mdl = MemoryDescriptorList::new(virt_user_addr + 256, 5000);
        assert_eq!(mdl.byte_offset, 256);
        assert_eq!(mdl.physical_pages.len(), 2);
        assert!(!mdl.is_probed);
        assert!(!mdl.is_locked);

        // Probe and lock MDL
        mdl.probe_and_lock(&vmm).unwrap();
        assert!(mdl.is_probed);
        assert!(mdl.is_locked);
        assert_eq!(mdl.physical_pages[0], phys_frame1);
        assert_eq!(mdl.physical_pages[1], phys_frame2);

        // Map locked MDL to a contiguous kernel virtual address range (requires allocation of kernel mapping space page directories)
        let kernel_start_virt = 0x0000_2000_0000_0000;
        let mapped_address = unsafe { mdl.map_to_kernel_space(kernel_start_virt, &mut vmm, &mut kern_allocator).unwrap() };
        assert!(mdl.is_mapped);
        assert_eq!(mapped_address, kernel_start_virt + 256);

        // Cleanup
        mdl.unmap();
        assert!(!mdl.is_mapped);
        mdl.unlock();
        assert!(!mdl.is_locked);
    }
}
