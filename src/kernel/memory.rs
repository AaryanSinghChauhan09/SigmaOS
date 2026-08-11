// SigmaOS Virtual Memory Manager & Page Table implementation
// Expanding to provide full 4-level x86_64 paging, CoW, and TLB management
#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::ptr::NonNull;
use crate::kernel::memory::BuddyAllocator; // existing

pub const PAGE_SIZE: usize = 4096;

/// Page Table Entry (PTE) Flags
pub const PTE_PRESENT: u64 = 1 << 0;
pub const PTE_WRITABLE: u64 = 1 << 1;
pub const PTE_USER: u64 = 1 << 2;
pub const PTE_WRITE_THROUGH: u64 = 1 << 3;
pub const PTE_CACHE_DISABLE: u64 = 1 << 4;
pub const PTE_ACCESSED: u64 = 1 << 5;
pub const PTE_DIRTY: u64 = 1 << 6;
pub const PTE_HUGE_PAGE: u64 = 1 << 7;
pub const PTE_GLOBAL: u64 = 1 << 8;
pub const PTE_COW: u64 = 1 << 9; // Custom CoW bit in available OS area
pub const PTE_NO_EXECUTE: u64 = 1 << 63;

/// Address bounds
pub const KERNEL_BASE: usize = 0xFFFF_8000_0000_0000;
pub const USER_LIMIT: usize = 0x0000_7FFF_FFFF_FFFF;

/// Represents an entry in any level of the page table
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn new() -> Self {
        PageTableEntry(0)
    }
    
    pub fn is_unused(&self) -> bool { self.0 == 0 }
    pub fn set_unused(&mut self) { self.0 = 0; }
    pub fn flags(&self) -> u64 { self.0 & 0xFFF0_0000_0000_0FFF }
    pub fn addr(&self) -> usize { (self.0 & 0x000F_FFFF_FFFF_F000) as usize }
    pub fn set_addr(&mut self, addr: usize, flags: u64) {
        assert_eq!(addr & !0x000F_FFFF_FFFF_F000, 0);
        self.0 = (addr as u64) | flags;
    }
    pub fn is_present(&self) -> bool { (self.0 & PTE_PRESENT) != 0 }
    pub fn is_writable(&self) -> bool { (self.0 & PTE_WRITABLE) != 0 }
    pub fn set_flags(&mut self, flags: u64) {
        self.0 = (self.0 & 0x000F_FFFF_FFFF_F000) | flags;
    }
    pub fn clear_flags(&mut self, flags: u64) {
        self.0 &= !flags;
    }
}

/// A 4KB page table containing 512 entries
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
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

/// Abstraction for active page table management
pub struct ActivePageTable {
    pub pml4_phys_addr: usize,
}

impl ActivePageTable {
    pub unsafe fn new(pml4_phys_addr: usize) -> Self {
        Self { pml4_phys_addr }
    }

    /// Access the PML4 table via its linear kernel mapping
    pub fn pml4(&mut self) -> &mut PageTable {
        // Assuming linear mapping of physical memory at KERNEL_BASE
        let virt = KERNEL_BASE + self.pml4_phys_addr;
        unsafe { &mut *(virt as *mut PageTable) }
    }

    /// Translates a virtual address to a physical address
    pub fn translate(&mut self, virt_addr: usize) -> Option<usize> {
        let p4_idx = (virt_addr >> 39) & 0x1FF;
        let p3_idx = (virt_addr >> 30) & 0x1FF;
        let p2_idx = (virt_addr >> 21) & 0x1FF;
        let p1_idx = (virt_addr >> 12) & 0x1FF;
        let offset = virt_addr & 0xFFF;

        let pml4 = self.pml4();
        let p4_entry = &pml4.entries[p4_idx];
        if !p4_entry.is_present() { return None; }

        let pdpt = unsafe { &*( (KERNEL_BASE + p4_entry.addr()) as *const PageTable ) };
        let p3_entry = &pdpt.entries[p3_idx];
        if !p3_entry.is_present() { return None; }
        if (p3_entry.flags() & PTE_HUGE_PAGE) != 0 {
            // 1GB huge page
            return Some(p3_entry.addr() + (virt_addr & 0x3FFF_FFFF));
        }

        let pd = unsafe { &*( (KERNEL_BASE + p3_entry.addr()) as *const PageTable ) };
        let p2_entry = &pd.entries[p2_idx];
        if !p2_entry.is_present() { return None; }
        if (p2_entry.flags() & PTE_HUGE_PAGE) != 0 {
            // 2MB huge page
            return Some(p2_entry.addr() + (virt_addr & 0x1F_FFFF));
        }

        let pt = unsafe { &*( (KERNEL_BASE + p2_entry.addr()) as *const PageTable ) };
        let p1_entry = &pt.entries[p1_idx];
        if !p1_entry.is_present() { return None; }

        Some(p1_entry.addr() + offset)
    }

    /// Helper to get next level table, allocating if not present
    fn get_next_level(&mut self, entry: &mut PageTableEntry, alloc: &mut BuddyAllocator, flags: u64) -> Result<&mut PageTable, &'static str> {
        if !entry.is_present() {
            let phys = alloc.allocate_pages(1).ok_or("OOM in page table allocation")?.as_ptr() as usize;
            let virt = KERNEL_BASE + phys;
            let table = unsafe { &mut *(virt as *mut PageTable) };
            table.clear();
            entry.set_addr(phys, PTE_PRESENT | PTE_WRITABLE | PTE_USER | flags); // Intermediate levels often have generic permissive flags
            Ok(table)
        } else {
            let phys = entry.addr();
            let virt = KERNEL_BASE + phys;
            Ok(unsafe { &mut *(virt as *mut PageTable) })
        }
    }

    /// Map a page
    pub fn map_page(&mut self, virt_addr: usize, phys_addr: usize, flags: u64, alloc: &mut BuddyAllocator) -> Result<(), &'static str> {
        assert_eq!(virt_addr % PAGE_SIZE, 0);
        assert_eq!(phys_addr % PAGE_SIZE, 0);

        let p4_idx = (virt_addr >> 39) & 0x1FF;
        let p3_idx = (virt_addr >> 30) & 0x1FF;
        let p2_idx = (virt_addr >> 21) & 0x1FF;
        let p1_idx = (virt_addr >> 12) & 0x1FF;

        let pml4 = self.pml4();
        let p4_entry = &mut pml4.entries[p4_idx];
        let pdpt = self.get_next_level(p4_entry, alloc, flags)?;
        
        let p3_entry = &mut pdpt.entries[p3_idx];
        let pd = self.get_next_level(p3_entry, alloc, flags)?;
        
        let p2_entry = &mut pd.entries[p2_idx];
        let pt = self.get_next_level(p2_entry, alloc, flags)?;
        
        let p1_entry = &mut pt.entries[p1_idx];
        if p1_entry.is_present() {
            return Err("Page already mapped");
        }
        
        p1_entry.set_addr(phys_addr, flags | PTE_PRESENT);
        Ok(())
    }

    /// Unmap a page
    pub fn unmap_page(&mut self, virt_addr: usize) -> Result<(), &'static str> {
        assert_eq!(virt_addr % PAGE_SIZE, 0);
        let p4_idx = (virt_addr >> 39) & 0x1FF;
        let p3_idx = (virt_addr >> 30) & 0x1FF;
        let p2_idx = (virt_addr >> 21) & 0x1FF;
        let p1_idx = (virt_addr >> 12) & 0x1FF;

        let pml4 = self.pml4();
        let p4_entry = &mut pml4.entries[p4_idx];
        if !p4_entry.is_present() { return Err("Page not mapped (P4)"); }
        let pdpt = unsafe { &mut *( (KERNEL_BASE + p4_entry.addr()) as *mut PageTable ) };
        
        let p3_entry = &mut pdpt.entries[p3_idx];
        if !p3_entry.is_present() { return Err("Page not mapped (P3)"); }
        let pd = unsafe { &mut *( (KERNEL_BASE + p3_entry.addr()) as *mut PageTable ) };

        let p2_entry = &mut pd.entries[p2_idx];
        if !p2_entry.is_present() { return Err("Page not mapped (P2)"); }
        let pt = unsafe { &mut *( (KERNEL_BASE + p2_entry.addr()) as *mut PageTable ) };

        let p1_entry = &mut pt.entries[p1_idx];
        if !p1_entry.is_present() { return Err("Page not mapped (P1)"); }

        p1_entry.set_unused();
        Self::invlpg(virt_addr);
        Ok(())
    }

    /// Protect a page (change flags)
    pub fn protect_page(&mut self, virt_addr: usize, new_flags: u64) -> Result<(), &'static str> {
        assert_eq!(virt_addr % PAGE_SIZE, 0);
        let p1_idx = (virt_addr >> 12) & 0x1FF;
        let pt = self.walk_to_pt(virt_addr)?;
        let p1_entry = &mut pt.entries[p1_idx];
        
        if !p1_entry.is_present() { return Err("Page not mapped"); }
        let phys = p1_entry.addr();
        p1_entry.set_addr(phys, new_flags | PTE_PRESENT);
        Self::invlpg(virt_addr);
        Ok(())
    }

    fn walk_to_pt(&mut self, virt_addr: usize) -> Result<&mut PageTable, &'static str> {
        let p4_idx = (virt_addr >> 39) & 0x1FF;
        let p3_idx = (virt_addr >> 30) & 0x1FF;
        let p2_idx = (virt_addr >> 21) & 0x1FF;

        let pml4 = self.pml4();
        let p4_entry = &mut pml4.entries[p4_idx];
        if !p4_entry.is_present() { return Err("P4 missing"); }
        let pdpt = unsafe { &mut *( (KERNEL_BASE + p4_entry.addr()) as *mut PageTable ) };
        
        let p3_entry = &mut pdpt.entries[p3_idx];
        if !p3_entry.is_present() { return Err("P3 missing"); }
        let pd = unsafe { &mut *( (KERNEL_BASE + p3_entry.addr()) as *mut PageTable ) };

        let p2_entry = &mut pd.entries[p2_idx];
        if !p2_entry.is_present() { return Err("P2 missing"); }
        let pt = unsafe { &mut *( (KERNEL_BASE + p2_entry.addr()) as *mut PageTable ) };
        Ok(pt)
    }

    /// TLB Shootdown implementation wrapper
    pub fn invlpg(addr: usize) {
        unsafe {
            core::arch::asm!("invlpg [{}]", in(reg) addr, options(nostack, preserves_flags));
        }
    }
}

// VMA structures

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmAccess {
    Read,
    ReadWrite,
    Execute,
    None,
}

impl VmAccess {
    pub fn to_pte_flags(&self) -> u64 {
        let mut flags = PTE_USER;
        match self {
            VmAccess::Read => flags |= PTE_NO_EXECUTE,
            VmAccess::ReadWrite => flags |= PTE_WRITABLE | PTE_NO_EXECUTE,
            VmAccess::Execute => {},
            VmAccess::None => flags = 0,
        }
        flags
    }
}

#[derive(Debug, Clone)]
pub struct VmArea {
    pub start: usize,
    pub end: usize,
    pub access: VmAccess,
    pub is_cow: bool,
    pub mapped_file: Option<u64>,
    pub name: alloc::string::String,
}

pub struct VirtualMemoryManager {
    pub areas: BTreeMap<usize, VmArea>,
    pub pt: ActivePageTable,
    pub page_ref_counts: BTreeMap<usize, usize>, // Physical page reference counts for CoW
}

impl VirtualMemoryManager {
    pub fn new(pml4_phys: usize) -> Self {
        Self {
            areas: BTreeMap::new(),
            pt: unsafe { ActivePageTable::new(pml4_phys) },
            page_ref_counts: BTreeMap::new(),
        }
    }

    pub fn inc_ref(&mut self, phys: usize) {
        let count = self.page_ref_counts.entry(phys).or_insert(0);
        *count += 1;
    }

    pub fn dec_ref(&mut self, phys: usize, alloc: &mut BuddyAllocator) {
        if let Some(count) = self.page_ref_counts.get_mut(&phys) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.page_ref_counts.remove(&phys);
                let ptr = NonNull::new((KERNEL_BASE + phys) as *mut u8).unwrap();
                alloc.free_pages(ptr, 1);
            }
        }
    }

    pub fn mmap(&mut self, addr: usize, length: usize, access: VmAccess, is_cow: bool, mapped_file: Option<u64>, name: &str) -> Result<usize, &'static str> {
        let aligned_addr = addr & !(PAGE_SIZE - 1);
        let aligned_length = (length + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        
        let area = VmArea {
            start: aligned_addr,
            end: aligned_addr + aligned_length,
            access,
            is_cow,
            mapped_file,
            name: alloc::string::String::from(name),
        };
        self.areas.insert(aligned_addr, area);
        Ok(aligned_addr)
    }

    pub fn munmap(&mut self, addr: usize, length: usize, alloc: &mut BuddyAllocator) -> Result<(), &'static str> {
        let aligned_addr = addr & !(PAGE_SIZE - 1);
        let aligned_length = (length + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        
        if let Some(area) = self.areas.remove(&aligned_addr) {
            for vaddr in (area.start..area.end).step_by(PAGE_SIZE) {
                if let Some(phys) = self.pt.translate(vaddr) {
                    self.dec_ref(phys, alloc);
                    let _ = self.pt.unmap_page(vaddr);
                }
            }
            Ok(())
        } else {
            Err("No mapping found")
        }
    }

    pub fn handle_page_fault(&mut self, fault_addr: usize, err_code: u64, alloc: &mut BuddyAllocator) -> Result<(), &'static str> {
        let is_write = (err_code & 2) != 0;
        let is_user = (err_code & 4) != 0;
        let is_present = (err_code & 1) != 0;

        let fault_page = fault_addr & !(PAGE_SIZE - 1);

        // Find area
        let mut target_area = None;
        for (_, area) in self.areas.iter() {
            if fault_addr >= area.start && fault_addr < area.end {
                target_area = Some(area.clone());
                break;
            }
        }

        let area = target_area.ok_or("Segmentation fault: Unmapped region")?;

        if is_write && area.access == VmAccess::Read {
            return Err("Segmentation fault: Write to read-only region");
        }

        if is_present && is_write && area.is_cow {
            // Copy on write fault
            let pt = self.pt.walk_to_pt(fault_page)?;
            let p1_idx = (fault_page >> 12) & 0x1FF;
            let p1_entry = &pt.entries[p1_idx];
            
            if (p1_entry.flags() & PTE_COW) != 0 {
                let old_phys = p1_entry.addr();
                let ref_count = *self.page_ref_counts.get(&old_phys).unwrap_or(&1);
                
                if ref_count == 1 {
                    // We are the only owner, just make it writable
                    self.pt.protect_page(fault_page, area.access.to_pte_flags())?;
                } else {
                    // Copy page
                    let new_phys = alloc.allocate_pages(1).ok_or("OOM during CoW")?.as_ptr() as usize - KERNEL_BASE;
                    self.inc_ref(new_phys);
                    
                    // Copy data
                    unsafe {
                        let src = (KERNEL_BASE + old_phys) as *const u8;
                        let dst = (KERNEL_BASE + new_phys) as *mut u8;
                        core::ptr::copy_nonoverlapping(src, dst, PAGE_SIZE);
                    }
                    
                    self.dec_ref(old_phys, alloc);
                    let _ = self.pt.unmap_page(fault_page);
                    self.pt.map_page(fault_page, new_phys, area.access.to_pte_flags(), alloc)?;
                }
                return Ok(());
            }
        } else if !is_present {
            // Normal demand paging
            let new_phys = alloc.allocate_pages(1).ok_or("OOM during demand paging")?.as_ptr() as usize - KERNEL_BASE;
            self.inc_ref(new_phys);
            
            // Zero the page
            unsafe {
                core::ptr::write_bytes((KERNEL_BASE + new_phys) as *mut u8, 0, PAGE_SIZE);
            }

            self.pt.map_page(fault_page, new_phys, area.access.to_pte_flags(), alloc)?;
            return Ok(());
        }

        Err("Unhandled page fault condition")
    }
}