// SigmaOS 4-Level Page Table Walking & Paging Subsystem
// Zero-dependency, std-based x86_64 paging implementation.
// Linux/BSD-inspired demand paging, swap, and memory pressure handling.

use core::ptr::NonNull;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::vec::Vec;
use std::collections::BTreeMap;
use std::string::String;

pub const PAGE_SIZE: usize = 4096;
pub const ENTRY_COUNT: usize = 512;
pub const SWAP_SLOT_SIZE: usize = 4096;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemandPageType {
    AnonymousZero,
    FileBacked,
    SharedMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultReason {
    PageNotPresent,
    WriteProtectionViolation,
    UserAccessViolation,
}

#[derive(Debug, Clone)]
pub struct DemandPageZone {
    pub start_vaddr: u64,
    pub page_count: usize,
    pub zone_type: DemandPageType,
    pub read_only: bool,
}

// =========================================================================
// Swap Subsystem (Linux/BSD-inspired)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapEntryType {
    Unused,
    InUse,
    Bad,
}

#[derive(Debug, Clone)]
pub struct SwapEntry {
    pub slot_id: u64,
    pub entry_type: SwapEntryType,
    pub backing_file: Option<String>,
    pub backing_offset: u64,
}

pub struct SwapManager {
    pub swap_slots: BTreeMap<u64, SwapEntry>,
    pub next_slot_id: AtomicU64,
    pub total_swap_bytes: AtomicUsize,
    pub used_swap_bytes: AtomicUsize,
    pub swap_in_progress: AtomicUsize,
}

impl SwapManager {
    pub fn new(total_swap_bytes: usize) -> Self {
        SwapManager {
            swap_slots: std::collections::BTreeMap::new(),
            next_slot_id: AtomicU64::new(0),
            total_swap_bytes: AtomicUsize::new(total_swap_bytes),
            used_swap_bytes: AtomicUsize::new(0),
            swap_in_progress: AtomicUsize::new(0),
        }
    }

    pub fn allocate_swap_slot(&mut self) -> Result<u64, &'static str> {
        let slot_id = self.next_slot_id.fetch_add(1, Ordering::SeqCst);
        
        if self.used_swap_bytes.load(Ordering::Relaxed) + SWAP_SLOT_SIZE > self.total_swap_bytes.load(Ordering::Relaxed) {
            return Err("Swap space exhausted");
        }

        let entry = SwapEntry {
            slot_id,
            entry_type: SwapEntryType::InUse,
            backing_file: None,
            backing_offset: 0,
        };

        self.swap_slots.insert(slot_id, entry);
        self.used_swap_bytes.fetch_add(SWAP_SLOT_SIZE, Ordering::SeqCst);
        
        Ok(slot_id)
    }

    pub fn free_swap_slot(&mut self, slot_id: u64) -> Result<(), &'static str> {
        if let Some(entry) = self.swap_slots.remove(&slot_id) {
            if entry.entry_type == SwapEntryType::InUse {
                self.used_swap_bytes.fetch_sub(SWAP_SLOT_SIZE, Ordering::SeqCst);
            }
            Ok(())
        } else {
            Err("Swap slot not found")
        }
    }

    pub fn swap_in(&mut self, slot_id: u64, dest_vaddr: u64) -> Result<(), &'static str> {
        self.swap_in_progress.fetch_add(1, Ordering::SeqCst);
        
        if let Some(entry) = self.swap_slots.get(&slot_id) {
            if entry.entry_type != SwapEntryType::InUse {
                self.swap_in_progress.fetch_sub(1, Ordering::SeqCst);
                return Err("Swap slot not in use");
            }
            
            // Perform actual swap-in operation
            // This would read from backing storage and write to dest_vaddr
            
            self.swap_in_progress.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            self.swap_in_progress.fetch_sub(1, Ordering::SeqCst);
            Err("Swap slot not found")
        }
    }

    pub fn swap_out(&mut self, src_vaddr: u64) -> Result<u64, &'static str> {
        self.swap_in_progress.fetch_add(1, Ordering::SeqCst);
        
        let slot_id = self.allocate_swap_slot()?;
        
        // Perform actual swap-out operation
        // This would read from src_vaddr and write to backing storage
        
        if let Some(entry) = self.swap_slots.get_mut(&slot_id) {
            entry.backing_offset = 0; // Set actual offset
        }
        
        self.swap_in_progress.fetch_sub(1, Ordering::SeqCst);
        Ok(slot_id)
    }

    pub fn get_swap_usage(&self) -> (usize, usize) {
        let used = self.used_swap_bytes.load(Ordering::Relaxed);
        let total = self.total_swap_bytes.load(Ordering::Relaxed);
        (used, total)
    }
}

pub struct DemandPagingSubsystem {
    pub total_memory_bytes: usize,
    pub mapped_zones: Vec<DemandPageZone>,
    pub allocated_fault_pages: usize,
    pub swap_manager: SwapManager,
    pub memory_pressure: AtomicUsize,
}

impl DemandPagingSubsystem {
    pub fn new(total_memory_bytes: usize, swap_bytes: usize) -> Self {
        Self {
            total_memory_bytes,
            mapped_zones: std::vec::Vec::new(),
            allocated_fault_pages: 0,
            swap_manager: SwapManager::new(swap_bytes),
            memory_pressure: AtomicUsize::new(0),
        }
    }

    pub fn map_demand_zone(&mut self, zone: DemandPageZone) {
        self.mapped_zones.push(zone);
    }

    pub fn handle_demand_fault(
        &mut self,
        vaddr: u64,
        reason: PageFaultReason,
    ) -> Result<(), &'static str> {
        let matching_zone = self.mapped_zones.iter().find(|z| {
            vaddr >= z.start_vaddr && vaddr < z.start_vaddr + (z.page_count as u64 * 4096)
        });

        if matching_zone.is_some() && reason == PageFaultReason::PageNotPresent {
            // Check memory pressure and potentially swap out
            let pressure = self.memory_pressure.load(Ordering::Relaxed);
            if pressure > 80 {
                // High memory pressure, consider swapping out
                self.try_swap_out()?;
            }

            self.allocated_fault_pages += 1;
            Ok(())
        } else {
            Err("Page fault outside mapped demand zone or invalid reason")
        }
    }

    pub fn try_swap_out(&mut self) -> Result<(), &'static str> {
        // Select a page to swap out (LRU or other algorithm)
        // For now, just return success
        Ok(())
    }

    pub fn update_memory_pressure(&self, used_bytes: usize) {
        let pressure = (used_bytes * 100) / self.total_memory_bytes;
        self.memory_pressure.store(pressure, Ordering::SeqCst);
    }

    pub fn get_active_mapped_pages_count(&self) -> usize {
        self.allocated_fault_pages
    }

    pub fn get_swap_usage(&self) -> (usize, usize) {
        self.swap_manager.get_swap_usage()
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
            physical_pages: std::vec![0; page_count],
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
            let phys =
                unsafe { vmm.translate(virt) }.ok_or("Virtual address page fault during probe")?;
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

        let mapped_address =
            kernel_start_virt - (self.physical_pages.len() * 4096) as u64 + self.byte_offset as u64;
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

        assert!(unsafe {
            vmm.map_page(virt_user_addr, phys_frame1, flags, &mut user_allocator)
                .is_ok()
        });
        assert!(unsafe {
            vmm.map_page(
                virt_user_addr + 4096,
                phys_frame2,
                flags,
                &mut user_allocator,
            )
            .is_ok()
        });

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
        let mapped_address = unsafe {
            mdl.map_to_kernel_space(kernel_start_virt, &mut vmm, &mut kern_allocator)
                .unwrap()
        };
        assert!(mdl.is_mapped);
        assert_eq!(mapped_address, kernel_start_virt + 256);

        // Cleanup
        mdl.unmap();
        assert!(!mdl.is_mapped);
        mdl.unlock();
        assert!(!mdl.is_locked);
    }

    #[test]
    fn test_swap_manager_allocation_and_free() {
        let mut swap_mgr = SwapManager::new(8192); // 2 swap slots

        // Allocate first slot
        let slot1 = swap_mgr.allocate_swap_slot().unwrap();
        assert_eq!(slot1, 0);
        let (used, total) = swap_mgr.get_swap_usage();
        assert_eq!(used, 4096);
        assert_eq!(total, 8192);

        // Allocate second slot
        let slot2 = swap_mgr.allocate_swap_slot().unwrap();
        assert_eq!(slot2, 1);
        let (used, total) = swap_mgr.get_swap_usage();
        assert_eq!(used, 8192);
        assert_eq!(total, 8192);

        // Attempt to allocate when full
        assert!(swap_mgr.allocate_swap_slot().is_err());

        // Free first slot
        swap_mgr.free_swap_slot(slot1).unwrap();
        let (used, total) = swap_mgr.get_swap_usage();
        assert_eq!(used, 4096);
        assert_eq!(total, 8192);

        // Free second slot
        swap_mgr.free_swap_slot(slot2).unwrap();
        let (used, total) = swap_mgr.get_swap_usage();
        assert_eq!(used, 0);
        assert_eq!(total, 8192);
    }

    #[test]
    fn test_demand_paging_with_swap() {
        let mut demand_paging = DemandPagingSubsystem::new(65536, 8192); // 16 pages RAM, 2 pages swap

        let zone = DemandPageZone {
            start_vaddr: 0x1000_0000,
            page_count: 4,
            zone_type: DemandPageType::AnonymousZero,
            read_only: false,
        };

        demand_paging.map_demand_zone(zone);

        // Update memory pressure to 85%
        demand_paging.update_memory_pressure(55705); // 85% of 65536

        // Handle page fault with high memory pressure
        let result = demand_paging.handle_demand_fault(0x1000_0000, PageFaultReason::PageNotPresent);
        assert!(result.is_ok());
        assert_eq!(demand_paging.get_active_mapped_pages_count(), 1);

        // Check swap usage
        let (_used, total) = demand_paging.get_swap_usage();
        assert_eq!(total, 8192);
    }
}
