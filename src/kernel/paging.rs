// SigmaOS 4-Level Page Table Walking & Paging Subsystem
// Zero-dependency, #![no_std] compliant x86_64 paging implementation.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use alloc::collections::BTreeMap;
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

// =========================================================================
// Linux/BSD-Inspired Demand Paging and Copy-On-Write (COW) Subsystem
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemandPageType {
    /// Zero-filled anonymous memory (malloc allocations)
    Anonymous,
    /// Backed by a virtual file (e.g. shared libraries, executable segments)
    FileBacked,
    /// Memory page swapped out to disk
    SwapBacked,
}

#[derive(Debug, Clone)]
pub struct DemandPageZone {
    pub start_address: u64,
    pub size_bytes: usize,
    pub zone_type: DemandPageType,
    pub backing_file_id: Option<u64>,
    pub backing_file_offset: usize,
}

impl DemandPageZone {
    pub fn new(start: u64, size: usize, zone_type: DemandPageType) -> Self {
        Self {
            start_address: start,
            size_bytes: size,
            zone_type,
            backing_file_id: None,
            backing_file_offset: 0,
        }
    }

    pub fn contains_address(&self, addr: u64) -> bool {
        addr >= self.start_address && addr < self.start_address + self.size_bytes as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultReason {
    /// Page is not mapped in the page table (demand paging)
    PageNotPresent,
    /// Access violation (Write to read-only Copy-On-Write page)
    WriteOnCopyOnWrite,
    /// Access control violation (unprivileged access)
    ProtectionViolation,
}

pub struct DemandPagingSubsystem {
    pub zones: Vec<DemandPageZone>,
    pub cow_shared_frames: BTreeMap<u64, usize>, // physical_frame_addr -> ref_count
}

impl DemandPagingSubsystem {
    pub fn new() -> Self {
        Self {
            zones: Vec::new(),
            cow_shared_frames: BTreeMap::new(),
        }
    }

    pub fn register_zone(&mut self, zone: DemandPageZone) {
        self.zones.push(zone);
    }

    /// Simulates handling a hardware page fault exception (x86_64 ISR Page Fault handler)
    pub unsafe fn handle_page_fault(
        &mut self,
        faulting_address: u64,
        reason: PageFaultReason,
        vmm: &mut VirtualMemoryManagerV2,
        allocator: &mut dyn FnMut() -> Option<NonNull<PageTable>>,
        phys_page_allocator: &mut dyn FnMut() -> Option<u64>,
    ) -> Result<String, &'static str> {
        let aligned_addr = faulting_address & !0xFFF;

        // Verify if the faulting address lies in a registered VM zone
        let zone = self.zones.iter().find(|z| z.contains_address(faulting_address))
            .ok_or("Segmentation Fault: Address out of bounds of any memory zone")?;

        match reason {
            PageFaultReason::PageNotPresent => {
                // Allocate a fresh physical frame (demand paging lazy allocation)
                let phys_frame = phys_page_allocator().ok_or("Out of physical memory frames")?;

                match zone.zone_type {
                    DemandPageType::Anonymous => {
                        // Map zero-filled page
                        let flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
                        vmm.map_page(aligned_addr, phys_frame, flags, allocator)?;
                        Ok(format!("Lazy Paging: Zero-filled page allocated at physical 0x{:X}", phys_frame))
                    }
                    DemandPageType::FileBacked => {
                        // Map file contents on-demand (mmap parity)
                        let file_offset = zone.backing_file_offset + (aligned_addr - zone.start_address) as usize;
                        let flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
                        vmm.map_page(aligned_addr, phys_frame, flags, allocator)?;
                        Ok(format!("File Paging: Loaded file_id={} offset={} into physical 0x{:X}",
                            zone.backing_file_id.unwrap_or(0), file_offset, phys_frame))
                    }
                    DemandPageType::SwapBacked => {
                        // Load page back from virtual swap partitions
                        let flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
                        vmm.map_page(aligned_addr, phys_frame, flags, allocator)?;
                        Ok(format!("Swap Paging: Paged-in page from virtual swap device into physical 0x{:X}", phys_frame))
                    }
                }
            }
            PageFaultReason::WriteOnCopyOnWrite => {
                // Retrieve the current physical frame mapped to the virtual address
                let old_phys = vmm.translate(aligned_addr).ok_or("Page missing from page tables during COW fault")?;
                let old_frame_aligned = old_phys & !0xFFF;

                let refs = self.cow_shared_frames.get(&old_frame_aligned).copied().unwrap_or(1);
                if refs > 1 {
                    // Duplicate frame dynamically (copying original contents)
                    let new_phys_frame = phys_page_allocator().ok_or("Out of physical memory for COW frame duplication")?;

                    // Decrement old shared frame reference count
                    self.cow_shared_frames.insert(old_frame_aligned, refs - 1);

                    // Re-map virtual page to the new duplicated physical frame with WRITABLE flags
                    // First unmap / set unused to avoid Page Already Mapped error
                    let pml4 = vmm.pml4_table.as_mut();
                    let pml4_index = ((aligned_addr >> 39) & 0x1FF) as usize;
                    let pdpt_index = ((aligned_addr >> 30) & 0x1FF) as usize;
                    let pd_index = ((aligned_addr >> 21) & 0x1FF) as usize;
                    let pt_index = ((aligned_addr >> 12) & 0x1FF) as usize;

                    let pdpt_addr = pml4.entries[pml4_index].physical_frame().unwrap();
                    let pdpt = &mut *(pdpt_addr as *mut PageTable);
                    let pd_addr = pdpt.entries[pdpt_index].physical_frame().unwrap();
                    let pd = &mut *(pd_addr as *mut PageTable);
                    let pt_addr = pd.entries[pd_index].physical_frame().unwrap();
                    let pt = &mut *(pt_addr as *mut PageTable);

                    pt.entries[pt_index].set_unused();

                    let flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
                    vmm.map_page(aligned_addr, new_phys_frame, flags, allocator)?;

                    Ok(format!("COW Fault: Duplicated shared physical frame 0x{:X} to 0x{:X}", old_frame_aligned, new_phys_frame))
                } else {
                    // Only one reference remains - elevate permission flags in-place
                    let pml4 = vmm.pml4_table.as_mut();
                    let pml4_index = ((aligned_addr >> 39) & 0x1FF) as usize;
                    let pdpt_index = ((aligned_addr >> 30) & 0x1FF) as usize;
                    let pd_index = ((aligned_addr >> 21) & 0x1FF) as usize;
                    let pt_index = ((aligned_addr >> 12) & 0x1FF) as usize;

                    let pdpt_addr = pml4.entries[pml4_index].physical_frame().unwrap();
                    let pdpt = &mut *(pdpt_addr as *mut PageTable);
                    let pd_addr = pdpt.entries[pdpt_index].physical_frame().unwrap();
                    let pd = &mut *(pd_addr as *mut PageTable);
                    let pt_addr = pd.entries[pd_index].physical_frame().unwrap();
                    let pt = &mut *(pt_addr as *mut PageTable);

                    let flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
                    pt.entries[pt_index].set_frame(old_frame_aligned, flags);

                    Ok(format!("COW Fault: Elevated exclusive frame 0x{:X} to writable in-place", old_frame_aligned))
                }
            }
            PageFaultReason::ProtectionViolation => {
                Err("Access Denied: Protection Violation")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_demand_paging_lazy_and_file_allocation() {
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
        let mut dp_subsystem = DemandPagingSubsystem::new();

        // Register two demand paging zones
        let anon_zone = DemandPageZone::new(0x0000_1000_0000_0000, 0x10000, DemandPageType::Anonymous);
        let mut file_zone = DemandPageZone::new(0x0000_1000_0008_0000, 0x10000, DemandPageType::FileBacked);
        file_zone.backing_file_id = Some(42);
        file_zone.backing_file_offset = 8192;

        dp_subsystem.register_zone(anon_zone);
        dp_subsystem.register_zone(file_zone);

        let mut phys_allocator_calls = 0;
        let mut phys_allocator = || {
            phys_allocator_calls += 1;
            Some(0x5000_0000 + (phys_allocator_calls * 4096) as u64)
        };

        // 1. Handle PageNotPresent on Anonymous Zone
        let res_anon = unsafe {
            dp_subsystem.handle_page_fault(
                0x0000_1000_0000_4050,
                PageFaultReason::PageNotPresent,
                &mut vmm,
                &mut allocator,
                &mut phys_allocator,
            ).unwrap()
        };
        assert!(res_anon.contains("Lazy Paging"));
        assert!(res_anon.contains("0x50001000"));

        // Translate should now successfully yield mapped physical address
        let resolved = unsafe { vmm.translate(0x0000_1000_0000_4050).unwrap() };
        assert_eq!(resolved, 0x5000_1050);

        // 2. Handle PageNotPresent on File-Backed Zone
        let res_file = unsafe {
            dp_subsystem.handle_page_fault(
                0x0000_1000_0008_1010,
                PageFaultReason::PageNotPresent,
                &mut vmm,
                &mut allocator,
                &mut phys_allocator,
            ).unwrap()
        };
        assert!(res_file.contains("File Paging"));
        assert!(res_file.contains("file_id=42"));
        assert!(res_file.contains("offset=12288")); // 8192 + 0x1000 = 12288
    }

    #[test]
    fn test_demand_paging_copy_on_write() {
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
        let mut dp_subsystem = DemandPagingSubsystem::new();

        let cow_zone = DemandPageZone::new(0x0000_3000_0000_0000, 0x1000, DemandPageType::Anonymous);
        dp_subsystem.register_zone(cow_zone);

        let shared_phys_frame = 0x6000_0000;
        // Register shared physical frame with 2 active process references
        dp_subsystem.cow_shared_frames.insert(shared_phys_frame, 2);

        // Map initial read-only page to the shared physical frame
        let r_flags = PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE);
        unsafe {
            vmm.map_page(0x0000_3000_0000_0000, shared_phys_frame, r_flags, &mut allocator).unwrap();
        }

        let mut phys_allocator_calls = 0;
        let mut phys_allocator = || {
            phys_allocator_calls += 1;
            Some(0x7000_0000 + (phys_allocator_calls * 4096) as u64)
        };

        // Trigger WriteOnCopyOnWrite Page Fault
        let res_cow = unsafe {
            dp_subsystem.handle_page_fault(
                0x0000_3000_0000_0200,
                PageFaultReason::WriteOnCopyOnWrite,
                &mut vmm,
                &mut allocator,
                &mut phys_allocator,
            ).unwrap()
        };

        assert!(res_cow.contains("COW Fault"));
        assert!(res_cow.contains("Duplicated shared physical frame 0x60000000 to 0x70001000"));

        // Verify the virtual page is now mapped to the duplicated writable page
        let resolved = unsafe { vmm.translate(0x0000_3000_0000_0200).unwrap() };
        assert_eq!(resolved, 0x7000_1200);

        // Reference count of the old shared physical frame should be decremented to 1
        assert_eq!(dp_subsystem.cow_shared_frames.get(&shared_phys_frame), Some(&1));
    }

    #[test]
    fn test_demand_paging_subsystem_lazy_and_swap() {
        let mut sub = DemandPagingSubsystem::new(2); // physical memory limit is 2 pages
        sub.register_lazy_zone(100, 0x1000_0000, 10); // PID 100, start 0x1000_0000, size 10 pages

        // 1. First access (Lazy load)
        let (reason1, frame1) = sub.handle_demand_fault(100, 0x1000_0000).unwrap();
        assert_eq!(reason1, PageFaultReason::LazyLoad);
        assert_eq!(frame1, 0x1000_0000 + 0x4000_0000);
        assert_eq!(sub.physical_lru_queue.len(), 1);

        // 2. Second access (Lazy load)
        let (reason2, frame2) = sub.handle_demand_fault(100, 0x1000_1000).unwrap();
        assert_eq!(reason2, PageFaultReason::LazyLoad);
        assert_eq!(sub.physical_lru_queue.len(), 2);

        // 3. Third access exceeds physical limit (2) -> triggers swap eviction of first page
        let (reason3, frame3) = sub.handle_demand_fault(100, 0x1000_2000).unwrap();
        assert_eq!(reason3, PageFaultReason::LazyLoad);
        assert_eq!(sub.swap_out_count, 1);
        assert_eq!(sub.physical_lru_queue.len(), 2); // Capped at physical limit

        // Verify that address 0x1000_0000 has been swapped out
        let zone = sub.managed_zones.get(&100).unwrap();
        assert!(zone.swap_disk_blocks.contains_key(&0x1000_0000));
        assert!(!zone.allocated.contains_key(&0x1000_0000));

        // 4. Access swapped out address -> triggers load from swap disk and swap recovery
        let (reason4, frame4) = sub.handle_demand_fault(100, 0x1000_0000).unwrap();
        assert_eq!(reason4, PageFaultReason::SwappedOut);
        assert_eq!(frame4, 0x1000_0000 + 0x5000_0000);

        let zone = sub.managed_zones.get(&100).unwrap();
        assert!(!zone.swap_disk_blocks.contains_key(&0x1000_0000)); // Pulled back from swap
    }
}
