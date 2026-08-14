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
    pub const HUGE_PAGE: u64 = 1 << 7; // Page Size (PS) flag for huge pages
    pub const COW: u64 = 1 << 9; // Copy-On-Write flag
    pub const SWAPPED_OUT: u64 = 1 << 10; // Page is evacuated to swap disk space
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

// ==========================================
// Virtual Memory Demand Paging & Swapping
// ==========================================

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultReason {
    LazyLoad,
    SwappedOut,
    CopyOnWrite,
    ProtectionViolation,
}

#[derive(Debug, Clone)]
pub struct DemandPageZone {
    pub start_address: u64,
    pub size_pages: usize,
    pub allocated: HashMap<u64, u64>, // VirtAddr -> PhysAddr
    pub swap_disk_blocks: HashMap<u64, Vec<u8>>, // VirtAddr -> Evacuated swap bytes
}

pub struct DemandPagingSubsystem {
    pub managed_zones: HashMap<u64, DemandPageZone>, // PID -> Page zone
    pub physical_lru_queue: Vec<(u64, u64)>,        // (PID, VirtAddr) for LRU tracking
    pub max_physical_frames: usize,
    pub swap_out_count: usize,
}

impl DemandPagingSubsystem {
    pub fn new(max_physical_frames: usize) -> Self {
        Self {
            managed_zones: HashMap::new(),
            physical_lru_queue: Vec::new(),
            max_physical_frames,
            swap_out_count: 0,
        }
    }

    /// Registers a lazy demand-loadable zone for a process
    pub fn register_lazy_zone(&mut self, pid: u64, start_address: u64, size_pages: usize) {
        self.managed_zones.insert(pid, DemandPageZone {
            start_address,
            size_pages,
            allocated: HashMap::new(),
            swap_disk_blocks: HashMap::new(),
        });
    }

    /// Performs LRU eviction of physical frames when memory limit is reached (Tails & Linux Swap inspired)
    pub fn evacuate_to_swap_lru(&mut self) -> Result<(u64, u64), &'static str> {
        if self.physical_lru_queue.is_empty() {
            return Err("No active frames in LRU queue to swap out");
        }

        // Evict the least-recently used page (oldest)
        let (pid, virt_addr) = self.physical_lru_queue.remove(0);
        let zone = self.managed_zones.get_mut(&pid).ok_or("Zone missing for pid")?;

        if let Some(phys_addr) = zone.allocated.remove(&virt_addr) {
            // Write to simulated swap disk block
            let mut mock_swapped_bytes = vec![0u8; 4096];
            mock_swapped_bytes[0] = 0xAA; // mock payload marker
            zone.swap_disk_blocks.insert(virt_addr, mock_swapped_bytes);
            self.swap_out_count += 1;
            Ok((pid, virt_addr))
        } else {
            Err("Page metadata not found during swap-out")
        }
    }

    /// Handles and resolves demand page faults natively
    pub fn handle_demand_fault(
        &mut self,
        pid: u64,
        fault_addr: u64,
    ) -> Result<(PageFaultReason, u64), &'static str> {
        let aligned_addr = fault_addr & !0xFFF;

        // Check if zone exists and address boundaries
        {
            let zone = self.managed_zones.get(&pid).ok_or("No mapped zone for PID")?;
            let page_offset = (aligned_addr - zone.start_address) / 4096;
            if page_offset as usize >= zone.size_pages {
                return Err("Fault address outside registered process memory boundaries");
            }
        }

        // Case 1: Swapped Out Page
        let is_swapped = {
            let zone = self.managed_zones.get(&pid).ok_or("No mapped zone for PID")?;
            zone.swap_disk_blocks.contains_key(&aligned_addr)
        };

        if is_swapped {
            let zone = self.managed_zones.get_mut(&pid).ok_or("No mapped zone for PID")?;
            zone.swap_disk_blocks.remove(&aligned_addr);
            let reallocated_phys = aligned_addr + 0x5000_0000;
            zone.allocated.insert(aligned_addr, reallocated_phys);
            self.physical_lru_queue.push((pid, aligned_addr));
            return Ok((PageFaultReason::SwappedOut, reallocated_phys));
        }

        // Case 2: Lazy Load Page (First access)
        let is_allocated = {
            let zone = self.managed_zones.get(&pid).ok_or("No mapped zone for PID")?;
            zone.allocated.contains_key(&aligned_addr)
        };

        if !is_allocated {
            // Check if we need to swap out a page first to respect physical limit
            if self.physical_lru_queue.len() >= self.max_physical_frames {
                let _ = self.evacuate_to_swap_lru()?;
            }

            let zone = self.managed_zones.get_mut(&pid).ok_or("No mapped zone for PID")?;
            let new_phys = aligned_addr + 0x4000_0000; // Lazy dynamic mapping
            zone.allocated.insert(aligned_addr, new_phys);
            self.physical_lru_queue.push((pid, aligned_addr));
            return Ok((PageFaultReason::LazyLoad, new_phys));
        }

        Err("Page fault cannot be handled by DemandPagingSubsystem")
    }
}

impl Default for DemandPagingSubsystem {
    fn default() -> Self {
        Self::new(1024)
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
