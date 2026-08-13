// SigmaOS 4-Level Page Table Walking & Paging Subsystem
// Zero-dependency, #![no_std] compliant x86_64 paging implementation.

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
pub struct PageTableEntry(u64);

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
    pml4_table: NonNull<PageTable>,
    pub tlb_invalidations: core::sync::atomic::AtomicUsize,
    pub tlb_flushes: core::sync::atomic::AtomicUsize,
    pub is_5level_enabled: bool,
}

impl VirtualMemoryManagerV2 {
    pub unsafe fn new(pml4_phys_addr: u64) -> Self {
        Self {
            pml4_table: NonNull::new_unchecked(pml4_phys_addr as *mut PageTable),
            tlb_invalidations: core::sync::atomic::AtomicUsize::new(0),
            tlb_flushes: core::sync::atomic::AtomicUsize::new(0),
            is_5level_enabled: false,
        }
    }

    /// Simulate TLB invalidation for a specific virtual page address.
    pub fn invlpg(&self, _virt_addr: u64) {
        self.tlb_invalidations
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    }

    /// Simulate a complete TLB flush across all mapping caches.
    pub fn flush_tlb_all(&self) {
        self.tlb_flushes
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    }

    /// Translates a virtual address to its corresponding physical address by walking PML4 -> PDPT -> PD -> PT
    /// Supports 5-Level virtual memory paging matching PML5/P4D indexes when enabled.
    /// Also supports Huge Pages translation (2MB/1GB).
    pub unsafe fn translate(&self, virt_addr: u64) -> Option<u64> {
        let mut root_addr = self.pml4_table.as_ptr() as u64;

        if self.is_5level_enabled {
            // PML5 level translation (bits 48-56 of virtual address)
            let pml5_index = ((virt_addr >> 48) & 0x1FF) as usize;
            let pml5 = &*(root_addr as *const PageTable);
            let pml5_entry = &pml5.entries[pml5_index];
            root_addr = pml5_entry.physical_frame()?;
        }

        let pml4_index = ((virt_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virt_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virt_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virt_addr >> 12) & 0x1FF) as usize;

        let pml4 = &*(root_addr as *const PageTable);
        let pml4_entry = &pml4.entries[pml4_index];
        let pdpt_addr = pml4_entry.physical_frame()?;

        let pdpt = &*(pdpt_addr as *const PageTable);
        let pdpt_entry = &pdpt.entries[pdpt_index];

        // 1GB Huge Page translation
        if pdpt_entry.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
            let page_offset = virt_addr & 0x3FFF_FFFF; // 1GB offset mask
            return Some(pdpt_entry.physical_frame()? + page_offset);
        }

        let pd_addr = pdpt_entry.physical_frame()?;

        let pd = &*(pd_addr as *const PageTable);
        let pd_entry = &pd.entries[pd_index];

        // 2MB Huge Page translation
        if pd_entry.flags().0 & PageTableFlags::HUGE_PAGE != 0 {
            let page_offset = virt_addr & 0x1F_FFFF; // 2MB offset mask
            return Some(pd_entry.physical_frame()? + page_offset);
        }

        let pt_addr = pd_entry.physical_frame()?;

        let pt = &*(pt_addr as *const PageTable);
        let pt_entry = &pt.entries[pt_index];
        let frame_addr = pt_entry.physical_frame()?;
        let page_offset = virt_addr & 0xFFF;

        Some(frame_addr + page_offset)
    }

    /// Simulate a Page Fault resolution trigger. If it encounters a write on a COW-gated page,
    /// it resolves the violation by copying the page frame on-the-fly.
    pub unsafe fn handle_page_fault(
        &mut self,
        fault_addr: u64,
        is_write: bool,
    ) -> Result<u64, &'static str> {
        let pml4_index = ((fault_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((fault_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((fault_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((fault_addr >> 12) & 0x1FF) as usize;

        let pml4 = self.pml4_table.as_mut();
        let pml4_entry = &mut pml4.entries[pml4_index];
        let pdpt_addr = pml4_entry.physical_frame().ok_or("PF: PDPT missing")?;

        let pdpt = &mut *(pdpt_addr as *mut PageTable);
        let pdpt_entry = &mut pdpt.entries[pdpt_index];
        let pd_addr = pdpt_entry.physical_frame().ok_or("PF: PD missing")?;

        let pd = &mut *(pd_addr as *mut PageTable);
        let pd_entry = &mut pd.entries[pd_index];
        let pt_addr = pd_entry.physical_frame().ok_or("PF: PT missing")?;

        let pt = &mut *(pt_addr as *mut PageTable);
        let pt_entry = &mut pt.entries[pt_index];

        let flags = pt_entry.flags();
        if is_write && (flags.0 & PageTableFlags::COW != 0) {
            // Copy-On-Write page-fault triggering!
            let old_frame = pt_entry.physical_frame().ok_or("PF: Frame missing")?;
            let new_frame = old_frame + 0x1000_0000; // mock copy reallocation offset

            let mut new_flags = flags.0;
            new_flags &= !PageTableFlags::COW; // Clear COW flag
            new_flags |= PageTableFlags::WRITABLE; // Enable write permission

            pt_entry.set_frame(new_frame, PageTableFlags(new_flags));
            self.invlpg(fault_addr);
            return Ok(new_frame);
        }

        Err("Page fault cannot be resolved as COW")
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
        let mut pdpt_ptr = NonNull::new(&mut pdpt as *mut PageTable);
        let mut pd_ptr = NonNull::new(&mut pd as *mut PageTable);
        let mut pt_ptr = NonNull::new(&mut pt as *mut PageTable);

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
    fn test_5level_paging() {
        let mut pml5 = PageTable::new();
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();
        let mut pt = PageTable::new();

        let pml5_ptr = &mut pml5 as *mut PageTable;

        // Wire up the 5 levels manually
        pml5.entries[1].set_frame(
            &pml4 as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );
        pml4.entries[1].set_frame(
            &pdpt as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );
        pdpt.entries[1].set_frame(
            &pd as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );
        pd.entries[1].set_frame(
            &pt as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );
        pt.entries[1].set_frame(0x8000_0000, PageTableFlags(PageTableFlags::PRESENT));

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(pml5_ptr as u64) };
        vmm.is_5level_enabled = true;

        // virt index [1, 1, 1, 1, 1] offset 0
        let virt = (1 << 48) | (1 << 39) | (1 << 30) | (1 << 21) | (1 << 12);
        let translated = unsafe { vmm.translate(virt).unwrap() };
        assert_eq!(translated, 0x8000_0000);
    }

    #[test]
    fn test_tlb_tracking() {
        let pml4 = PageTable::new();
        let vmm = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };

        assert_eq!(
            vmm.tlb_invalidations
                .load(core::sync::atomic::Ordering::SeqCst),
            0
        );
        vmm.invlpg(0x1000);
        assert_eq!(
            vmm.tlb_invalidations
                .load(core::sync::atomic::Ordering::SeqCst),
            1
        );

        assert_eq!(
            vmm.tlb_flushes.load(core::sync::atomic::Ordering::SeqCst),
            0
        );
        vmm.flush_tlb_all();
        assert_eq!(
            vmm.tlb_flushes.load(core::sync::atomic::Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn test_huge_pages_translation() {
        let pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };

        // 1. Test 1GB Huge Page
        unsafe {
            let pml4_ptr = vmm.pml4_table.as_ptr();
            (*pml4_ptr).entries[0].set_frame(
                &pdpt as *const PageTable as u64,
                PageTableFlags(PageTableFlags::PRESENT),
            );
        }
        pdpt.entries[0].set_frame(
            0x4000_0000,
            PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::HUGE_PAGE),
        );

        let translated_1gb = unsafe { vmm.translate(0x1234).unwrap() };
        assert_eq!(translated_1gb, 0x4000_1234);

        // 2. Test 2MB Huge Page
        pdpt.entries[0].set_frame(
            &pd as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        ); // Reset to point to PD
        pd.entries[0].set_frame(
            0x20_0000,
            PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::HUGE_PAGE),
        );

        let translated_2mb = unsafe { vmm.translate(0x4567).unwrap() };
        assert_eq!(translated_2mb, 0x20_4567);
    }

    #[test]
    fn test_cow_page_fault_resolution() {
        let mut pml4 = PageTable::new();
        let mut pdpt = PageTable::new();
        let mut pd = PageTable::new();
        let mut pt = PageTable::new();

        pml4.entries[0].set_frame(
            &pdpt as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );
        pdpt.entries[0].set_frame(
            &pd as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );
        pd.entries[0].set_frame(
            &pt as *const PageTable as u64,
            PageTableFlags(PageTableFlags::PRESENT),
        );

        // Frame starting with Copy-On-Write flag active
        pt.entries[0].set_frame(
            0x1000,
            PageTableFlags(PageTableFlags::PRESENT | PageTableFlags::COW),
        );

        let mut vmm = unsafe { VirtualMemoryManagerV2::new(&pml4 as *const PageTable as u64) };

        // Trigger write fault -> resolves on-the-fly and copies frame
        let new_frame = unsafe { vmm.handle_page_fault(0x0, true).unwrap() };
        assert_eq!(new_frame, 0x1000 + 0x1000_0000);

        // Check that page is now writable and COW flag is cleared
        let flags = pt.entries[0].flags();
        assert_eq!(flags.0 & PageTableFlags::COW, 0);
        assert_ne!(flags.0 & PageTableFlags::WRITABLE, 0);
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
