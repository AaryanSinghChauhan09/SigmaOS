#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, AtomicU32, Ordering};

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_2M: usize = 2 * 1024 * 1024;
pub const PAGE_SIZE_1G: usize = 1024 * 1024 * 1024;
pub const PFN_SHIFT: usize = 12;
pub const PAGE_OFFSET: usize = 0xFFFF800000000000;

pub struct PhysicalAddress(pub u64);
pub struct VirtualAddress(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFaultError {
    NotPresent,
    PermissionDenied,
    InvalidAddress,
    AlreadyMapped,
}

pub struct Page {
    pub flags: AtomicUsize,
    pub count: AtomicUsize,
    pub mapping: Option<usize>,
    pub index: u64,
    pub private: Option<usize>,
    pub zone: Option<*const Zone>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            flags: AtomicUsize::new(0),
            count: AtomicUsize::new(1),
            mapping: None,
            index: 0,
            private: None,
            zone: None,
        }
    }

    pub fn inc_ref(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn dec_ref(&self) -> bool {
        self.count.fetch_sub(1, Ordering::SeqCst) == 1
    }

    pub fn get_count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

pub enum PageFlag {
    Locked = 1 << 0,
    Error = 1 << 1,
    Referenced = 1 << 2,
    Uptodate = 1 << 3,
    Dirty = 1 << 4,
    Mappedtods = 1 << 5,
    Anonymous = 1 << 6,
    Swapped = 1 << 7,
    Reclaimed = 1 << 8,
    Kernel = 1 << 9,
}

pub struct Zone {
    pub zone_start_pfn: u64,
    pub spanned_pages: u64,
    pub present_pages: u64,
    pub free_area: [FreeArea; 11],
    pub watermark: [usize; 3],
    pub nr_reclaimed: u64,
    pub nr_slabs: u64,
    pub cached_objects: u64,
    pub total_objects: u64,
}

pub struct FreeArea {
    pub free: usize,
    pub order: u32,
}

impl Zone {
    pub fn new(start_pfn: u64, size_pages: u64) -> Self {
        Zone {
            zone_start_pfn: start_pfn,
            spanned_pages: size_pages,
            present_pages: size_pages,
            free_area: [FreeArea { free: 0, order: 0 }; 11],
            watermark: [0; 3],
            nr_reclaimed: 0,
            nr_slabs: 0,
            cached_objects: 0,
            total_objects: 0,
        }
    }
}

pub struct ZonedPageAllocator {
    zones: Vec<Zone>,
    total_pages: usize,
    free_pages: usize,
}

impl ZonedPageAllocator {
    pub fn new() -> Self {
        ZonedPageAllocator {
            zones: Vec::new(),
            total_pages: 0,
            free_pages: 0,
        }
    }

    pub fn add_zone(&mut self, zone: Zone) {
        self.free_pages += zone.present_pages as usize;
        self.total_pages += zone.present_pages as usize;
        self.zones.push(zone);
    }

    pub fn allocate(&mut self, order: u32) -> Option<PhysicalAddress> {
        for zone in &mut self.zones {
            if zone.free_area[order as usize].free > 0 {
                zone.free_area[order as usize].free -= 1;
                self.free_pages -= 1 << order;
                let pfn = zone.zone_start_pfn;
                return Some(PhysicalAddress(pfn << PFN_SHIFT));
            }
        }
        None
    }

    pub fn free(&mut self, addr: PhysicalAddress, order: u32) {
        let pfn = addr.0 >> PFN_SHIFT;
        for zone in &mut self.zones {
            if pfn >= zone.zone_start_pfn && pfn < zone.zone_start_pfn + zone.spanned_pages {
                zone.free_area[order as usize].free += 1;
                zone.present_pages += 1 << order;
                self.free_pages += 1 << order;
                break;
            }
        }
    }

    pub fn total_free(&self) -> usize {
        self.free_pages
    }

    pub fn total_allocated(&self) -> usize {
        self.total_pages - self.free_pages
    }
}

pub struct VmArea {
    pub vm_start: u64,
    pub vm_end: u64,
    pub vm_flags: u32,
    pub vm_page_prot: u32,
    pub vm_pgoff: u64,
    pub vm_file: Option<usize>,
    pub vm_private_data: Option<usize>,
}

impl VmArea {
    pub fn new(start: u64, end: u64, flags: u32) -> Self {
        VmArea {
            vm_start: start,
            vm_end: end,
            vm_flags: flags,
            vm_page_prot: 0,
            vm_pgoff: 0,
            vm_file: None,
            vm_private_data: None,
        }
    }
}

pub struct VmSpace {
    pub pgd: usize,
    pub vmas: Vec<VmArea>,
    pub total_vm: u64,
    pub locked_vm: u64,
    pub pinned_vm: u64,
}

impl VmSpace {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set_addr(&mut self, addr: u64, flags: PageFlags) {
        // Clear everything but flags, and mask the address to align with 4KB
        self.0 = (addr & 0x0000_00FF_FFFF_F000) | flags.0;
    }

    pub fn get_addr(&self) -> u64 {
        self.0 & 0x0000_00FF_FFFF_F000
    }

    pub fn flags(&self) -> PageFlags {
        PageFlags(self.0 & 0xFFF0_0000_0000_0FFF)
    }

    pub fn is_present(&self) -> bool {
        (self.0 & PageFlags::PRESENT) != 0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

/// A standard Page Table (containing 512 entries on x86_64)
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); 512],
        }
    }
}

/// Virtual Memory Manager (VMM) handling paging
pub struct VirtualMemoryManager {
    pub root_directory: NonNull<PageTable>,
}

impl VirtualMemoryManager {
    pub fn new(root_directory: NonNull<PageTable>) -> Self {
        Self { root_directory }
    }

    /// Translates a virtual address into a physical address
    pub fn translate(&self, virtual_addr: u64) -> Option<u64> {
        // Mock translation logic for SigmaOS OOP structure
        // In a real x86_64 system, we would walk PML4 -> PDPT -> PD -> PT
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_ref() };

        let entry = &root.entries[pt_index as usize];
        if entry.is_present() {
            Some(entry.get_addr() + (virtual_addr & 0xFFF))
        } else {
            None
        }
    }

    /// Maps a virtual page to a physical frame
    pub fn map_page(
        &mut self,
        virtual_addr: u64,
        physical_addr: u64,
        flags: PageFlags,
    ) -> Result<(), &'static str> {
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_mut() };

        let entry = &mut root.entries[pt_index as usize];
        if entry.is_present() {
            return Err("Page already mapped!");
        }

        entry.set_addr(physical_addr, flags);
        Ok(())
    }

    /// Unmaps a virtual page
    pub fn unmap_page(&mut self, virtual_addr: u64) -> Result<(), &'static str> {
        let pt_index = (virtual_addr >> 12) & 0x1FF;
        let root = unsafe { self.root_directory.as_mut() };

        let entry = &mut root.entries[pt_index as usize];
        if !entry.is_present() {
            return Err("Page is not mapped!");
        }

        entry.clear();
        Ok(())
    }
}