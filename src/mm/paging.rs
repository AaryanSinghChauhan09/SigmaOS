// SigmaOS Safe Paging Manager
// High-level safe paging operations with bounded capacity, lost-update prevention, and TLB shootdown.

use super::page_table::{MultiLevelPageTableTree, PageTableEntry, PageTableFlags};
use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PagingError {
    NonCanonicalAddress(u64),
    CapacityExceeded,
    AlreadyMapped(u64),
    NotMapped(u64),
    DoubleFree(u64),
    InvalidFlags,
}

impl core::fmt::Display for PagingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonCanonicalAddress(addr) => write!(f, "PagingError: address {:#x} is non-canonical", addr),
            Self::CapacityExceeded => write!(f, "PagingError: page table capacity limit reached"),
            Self::AlreadyMapped(addr) => write!(f, "PagingError: virtual address {:#x} is already mapped", addr),
            Self::NotMapped(addr) => write!(f, "PagingError: virtual address {:#x} is not mapped", addr),
            Self::DoubleFree(addr) => write!(f, "PagingError: double unmap attempt for address {:#x}", addr),
            Self::InvalidFlags => write!(f, "PagingError: invalid page table entry flags"),
        }
    }
}

pub type PagingResult<T> = Result<T, PagingError>;

#[derive(Debug)]
pub struct PagingManager {
    tree: MultiLevelPageTableTree,
    active_cr3: Mutex<u64>,
    tlb_flush_pending: AtomicBool,
}

impl PagingManager {
    pub fn new(cr3_phys: u64, max_capacity_pages: usize) -> Self {
        Self {
            tree: MultiLevelPageTableTree::new(cr3_phys, max_capacity_pages),
            active_cr3: Mutex::new(cr3_phys),
            tlb_flush_pending: AtomicBool::new(false),
        }
    }

    /// Map a virtual page to a physical frame with atomic lost-update prevention
    pub fn map_page(&self, virt_addr: u64, phys_frame: u64, flags: PageTableFlags) -> PagingResult<()> {
        if !MultiLevelPageTableTree::is_canonical(virt_addr) {
            return Err(PagingError::NonCanonicalAddress(virt_addr));
        }

        if self.tree.mapped_pages.load(Ordering::SeqCst) >= self.tree.max_capacity_pages {
            return Err(PagingError::CapacityExceeded);
        }

        let key = MultiLevelPageTableTree::virt_to_indices(virt_addr);
        let mut map = self.tree.table_entries.lock().unwrap();

        if let Some(entry) = map.get(&key) {
            if entry.is_present() {
                return Err(PagingError::AlreadyMapped(virt_addr));
            }
        }

        let entry = PageTableEntry::new();
        entry.set(phys_frame, flags);
        map.insert(key, entry);

        self.tree.mapped_pages.fetch_add(1, Ordering::SeqCst);
        self.tlb_flush_pending.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// Unmap a virtual page with double-free prevention
    pub fn unmap_page(&self, virt_addr: u64) -> PagingResult<u64> {
        if !MultiLevelPageTableTree::is_canonical(virt_addr) {
            return Err(PagingError::NonCanonicalAddress(virt_addr));
        }

        let key = MultiLevelPageTableTree::virt_to_indices(virt_addr);
        let mut map = self.tree.table_entries.lock().unwrap();

        let entry = map.remove(&key).ok_or(PagingError::NotMapped(virt_addr))?;
        if !entry.is_present() {
            return Err(PagingError::NotMapped(virt_addr));
        }

        let old_raw = entry.clear();
        let old_phys = old_raw & 0x000F_FFFF_FFFF_F000;

        self.tree.mapped_pages.fetch_sub(1, Ordering::SeqCst);
        self.tlb_flush_pending.store(true, Ordering::SeqCst);

        Ok(old_phys)
    }

    /// Check if TLB shootdown or flush is required
    pub fn check_and_clear_tlb_flush(&self) -> bool {
        self.tlb_flush_pending.swap(false, Ordering::SeqCst)
    }

    /// Reload CR3 context register abstraction
    pub fn reload_cr3(&self, new_cr3: u64) {
        let mut active = self.active_cr3.lock().unwrap();
        *active = new_cr3;
        self.tlb_flush_pending.store(true, Ordering::SeqCst);
    }

    pub fn active_cr3(&self) -> u64 {
        *self.active_cr3.lock().unwrap()
    }

    pub fn mapped_pages(&self) -> usize {
        self.tree.mapped_pages.load(Ordering::SeqCst)
    }
}
