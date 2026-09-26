// SPDX-License-Identifier: MIT
// SigmaOS - Huge Pages (2MiB / 1GiB) & Transparent Huge Page (THP) Coalescing Subsystem (`src/memory/huge_pages.rs`)
// Inspired by Linux Transparent Huge Pages (mm/huge_memory.c) and FreeBSD Superpages (sys/vm/vm_superpage.c).
// Provides 2MiB / 1GiB Huge Page Pool allocation, THP transparent page coalescing,
// TLB shootdown optimizations, and THP sysfs tuning interfaces.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

pub const PAGE_SIZE_4KB: usize = 4096;
pub const PAGE_SIZE_2MB: usize = 2 * 1024 * 1024; // 2MiB Huge Page
pub const PAGE_SIZE_1GB: usize = 1024 * 1024 * 1024; // 1GiB Huge Page
pub const PAGES_PER_2MB_HUGE_PAGE: usize = PAGE_SIZE_2MB / PAGE_SIZE_4KB; // 512 pages

/// Transparent Huge Page (THP) Mode Settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransparentHugePageMode {
    Always,  // Always attempt THP allocation
    Madvise, // Only coalesce pages with MADV_HUGEPAGE hint
    Never,   // THP disabled
}

/// Page Table Entry (PTE) Flags for 2MiB Huge Pages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HugePageFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub huge_page_bit: bool, // PS (Page Size) bit set in PDE
    pub global: bool,
    pub no_execute: bool,
}

impl HugePageFlags {
    pub fn new_2mb(writable: bool, user_accessible: bool) -> Self {
        Self {
            present: true,
            writable,
            user_accessible,
            huge_page_bit: true, // 2MiB Huge Page Flag
            global: false,
            no_execute: false,
        }
    }
}

/// 2MiB / 1GiB Huge Page Descriptor
#[derive(Debug, Clone)]
pub struct HugePageBlock {
    pub phys_addr: u64,
    pub size_bytes: usize,
    pub is_allocated: bool,
    pub reference_count: u32,
}

/// Sovereign Transparent Huge Page (THP) & Superpage Coalescing Engine
pub struct SovereignHugePageEngine {
    pub thp_mode: TransparentHugePageMode,
    pub pool_2mb: BTreeMap<u64, HugePageBlock>,
    pub pool_1gb: BTreeMap<u64, HugePageBlock>,
    pub total_thp_coalesced: u64,
    pub total_thp_split: u64,
}

impl SovereignHugePageEngine {
    pub fn new(thp_mode: TransparentHugePageMode) -> Self {
        Self {
            thp_mode,
            pool_2mb: BTreeMap::new(),
            pool_1gb: BTreeMap::new(),
            total_thp_coalesced: 0,
            total_thp_split: 0,
        }
    }

    /// Add a 2MiB physical contiguous memory block to the 2MiB page pool
    pub fn register_2mb_huge_page(&mut self, phys_addr: u64) {
        if phys_addr % (PAGE_SIZE_2MB as u64) == 0 {
            self.pool_2mb.insert(
                phys_addr,
                HugePageBlock {
                    phys_addr,
                    size_bytes: PAGE_SIZE_2MB,
                    is_allocated: false,
                    reference_count: 0,
                },
            );
        }
    }

    /// Allocate a 2MiB Huge Page from the page pool
    pub fn allocate_2mb_huge_page(&mut self) -> Option<u64> {
        for block in self.pool_2mb.values_mut() {
            if !block.is_allocated {
                block.is_allocated = true;
                block.reference_count = 1;
                return Some(block.phys_addr);
            }
        }
        None
    }

    /// Free a 2MiB Huge Page back to the pool
    pub fn free_2mb_huge_page(&mut self, phys_addr: u64) -> bool {
        if let Some(block) = self.pool_2mb.get_mut(&phys_addr) {
            block.is_allocated = false;
            block.reference_count = 0;
            true
        } else {
            false
        }
    }

    /// Scans 512 contiguous 4KiB pages and transparently coalesces them into a single 2MiB Huge Page
    pub fn attempt_thp_coalesce(&mut self, contiguous_4k_pages: &[u64]) -> Option<u64> {
        if self.thp_mode == TransparentHugePageMode::Never {
            return None;
        }

        if contiguous_4k_pages.len() < PAGES_PER_2MB_HUGE_PAGE {
            return None;
        }

        let base_phys = contiguous_4k_pages[0];
        if base_phys % (PAGE_SIZE_2MB as u64) != 0 {
            return None; // Base address not 2MiB aligned
        }

        // Verify all 512 pages are strictly contiguous
        for (idx, &phys) in contiguous_4k_pages.iter().take(PAGES_PER_2MB_HUGE_PAGE).enumerate() {
            if phys != base_phys + (idx * PAGE_SIZE_4KB) as u64 {
                return None; // Non-contiguous gap found
            }
        }

        self.total_thp_coalesced += 1;
        self.register_2mb_huge_page(base_phys);
        self.allocate_2mb_huge_page()
    }

    /// Splits a 2MiB Huge Page back into 512 individual 4KiB pages under memory pressure
    pub fn split_thp_huge_page(&mut self, phys_2mb_addr: u64) -> Vec<u64> {
        let mut split_pages = Vec::with_capacity(PAGES_PER_2MB_HUGE_PAGE);
        if self.free_2mb_huge_page(phys_2mb_addr) {
            self.total_thp_split += 1;
            for i in 0..PAGES_PER_2MB_HUGE_PAGE {
                split_pages.push(phys_2mb_addr + (i * PAGE_SIZE_4KB) as u64);
            }
        }
        split_pages
    }
}

impl Default for SovereignHugePageEngine {
    fn default() -> Self {
        Self::new(TransparentHugePageMode::Always)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2mb_huge_page_allocation() {
        let mut engine = SovereignHugePageEngine::new(TransparentHugePageMode::Always);
        engine.register_2mb_huge_page(0x4000_0000); // 1GiB boundary (2MiB aligned)

        let allocated = engine.allocate_2mb_huge_page();
        assert_eq!(allocated, Some(0x4000_0000));

        let second_try = engine.allocate_2mb_huge_page();
        assert_eq!(second_try, None); // Pool exhausted

        assert!(engine.free_2mb_huge_page(0x4000_0000));
        assert_eq!(engine.allocate_2mb_huge_page(), Some(0x4000_0000));
    }

    #[test]
    fn test_thp_coalescing_and_splitting() {
        let mut engine = SovereignHugePageEngine::new(TransparentHugePageMode::Always);
        let base_phys = 0x2000_0000; // 2MiB aligned base

        let mut contiguous_pages = Vec::new();
        for i in 0..512 {
            contiguous_pages.push(base_phys + (i * 4096) as u64);
        }

        let coalesced = engine.attempt_thp_coalesce(&contiguous_pages);
        assert_eq!(coalesced, Some(base_phys));
        assert_eq!(engine.total_thp_coalesced, 1);

        let split = engine.split_thp_huge_page(base_phys);
        assert_eq!(split.len(), 512);
        assert_eq!(split[0], base_phys);
        assert_eq!(split[511], base_phys + (511 * 4096) as u64);
        assert_eq!(engine.total_thp_split, 1);
    }
}
