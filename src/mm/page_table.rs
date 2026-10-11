// SigmaOS Page Table Manager
// Explicit multi-level x86_64 page table representations (PML4, PDPT, PD, PT), entry flags, and invariant checks.

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;

pub const ENTRIES_PER_TABLE: usize = 512;
pub const PAGE_SIZE: u64 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PageTableFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub write_through: bool,
    pub cache_disable: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub huge_page: bool,
    pub global: bool,
    pub no_execute: bool,
}

impl PageTableFlags {
    pub fn to_u64(&self) -> u64 {
        let mut bits = 0u64;
        if self.present { bits |= 1 << 0; }
        if self.writable { bits |= 1 << 1; }
        if self.user_accessible { bits |= 1 << 2; }
        if self.write_through { bits |= 1 << 3; }
        if self.cache_disable { bits |= 1 << 4; }
        if self.accessed { bits |= 1 << 5; }
        if self.dirty { bits |= 1 << 6; }
        if self.huge_page { bits |= 1 << 7; }
        if self.global { bits |= 1 << 8; }
        if self.no_execute { bits |= 1 << 63; }
        bits
    }

    pub fn from_u64(bits: u64) -> Self {
        Self {
            present: (bits & (1 << 0)) != 0,
            writable: (bits & (1 << 1)) != 0,
            user_accessible: (bits & (1 << 2)) != 0,
            write_through: (bits & (1 << 3)) != 0,
            cache_disable: (bits & (1 << 4)) != 0,
            accessed: (bits & (1 << 5)) != 0,
            dirty: (bits & (1 << 6)) != 0,
            huge_page: (bits & (1 << 7)) != 0,
            global: (bits & (1 << 8)) != 0,
            no_execute: (bits & (1 << 63)) != 0,
        }
    }
}

/// Page Table Entry with atomic bitwise updates for lost-update prevention
#[derive(Debug)]
pub struct PageTableEntry {
    raw: AtomicU64,
}

impl PageTableEntry {
    pub fn new() -> Self {
        Self {
            raw: AtomicU64::new(0),
        }
    }

    pub fn set(&self, phys_frame: u64, flags: PageTableFlags) {
        let val = (phys_frame & 0x000F_FFFF_FFFF_F000) | flags.to_u64();
        self.raw.store(val, Ordering::SeqCst);
    }

    pub fn clear(&self) -> u64 {
        self.raw.swap(0, Ordering::SeqCst)
    }

    pub fn is_present(&self) -> bool {
        (self.raw.load(Ordering::SeqCst) & (1 << 0)) != 0
    }

    pub fn phys_frame(&self) -> u64 {
        self.raw.load(Ordering::SeqCst) & 0x000F_FFFF_FFFF_F000
    }

    pub fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_u64(self.raw.load(Ordering::SeqCst))
    }
}

impl Default for PageTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-level 4-level x86_64 Page Table Tree structure
#[derive(Debug)]
pub struct MultiLevelPageTableTree {
    pub cr3_phys: u64,
    pub mapped_pages: AtomicUsize,
    pub max_capacity_pages: usize,
    /// Level indices key: (PML4_idx, PDPT_idx, PD_idx, PT_idx) -> PageTableEntry
    pub table_entries: Mutex<HashMap<(usize, usize, usize, usize), PageTableEntry>>,
}

impl MultiLevelPageTableTree {
    pub fn new(cr3_phys: u64, max_capacity_pages: usize) -> Self {
        Self {
            cr3_phys,
            mapped_pages: AtomicUsize::new(0),
            max_capacity_pages,
            table_entries: Mutex::new(HashMap::new()),
        }
    }

    /// Decompose virtual address into x86_64 4-level page table indices
    pub fn virt_to_indices(virt_addr: u64) -> (usize, usize, usize, usize) {
        let pml4 = ((virt_addr >> 39) & 0x1FF) as usize;
        let pdpt = ((virt_addr >> 30) & 0x1FF) as usize;
        let pd   = ((virt_addr >> 21) & 0x1FF) as usize;
        let pt   = ((virt_addr >> 12) & 0x1FF) as usize;
        (pml4, pdpt, pd, pt)
    }

    /// Check address canonicality
    pub fn is_canonical(virt_addr: u64) -> bool {
        let sign_bits = virt_addr >> 47;
        sign_bits == 0 || sign_bits == 0x1FFFF
    }
}
