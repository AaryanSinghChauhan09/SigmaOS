// SPDX-License-Identifier: MIT
// SigmaOS - Huge Pages (2MiB / 1GiB) & Transparent Huge Page (THP) Coalescing Subsystem (`src/memory/huge_pages.rs`)
// Inspired by Linux Transparent Huge Pages (mm/huge_memory.c) and FreeBSD Superpages (sys/vm/vm_superpage.c).
// Provides 2MiB / 1GiB Huge Page Pool allocation, THP transparent page coalescing,
// TLB shootdown optimizations, and THP sysfs tuning interfaces.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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

/// THP Defrag Mode Settings (matching Linux `/sys/kernel/mm/transparent_hugepage/defrag`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThpDefragMode {
    Always,       // Direct reclaim and compaction on allocation fault
    Defer,        // Wake khugepaged daemon to compact asynchronously
    DeferMadvise, // Direct compaction only for MADV_HUGEPAGE, defer others
    Madvise,      // Direct compaction only for MADV_HUGEPAGE
    Never,        // Disable THP defragmentation and compaction
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
        for (idx, &phys) in contiguous_4k_pages
            .iter()
            .take(PAGES_PER_2MB_HUGE_PAGE)
            .enumerate()
        {
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

// =========================================================================
// THP KHUGEPAGED DEFRAG DAEMON & RUNTIME PAGE PROMOTION ENGINE
// =========================================================================

/// THP Page Promotion Policy Descriptor
#[derive(Debug, Clone)]
pub struct ThpPagePromotionPolicy {
    pub pages_to_scan: usize,          // Pages scanned per khugepaged iteration (default: 4096)
    pub scan_sleep_millisecs: u64,     // Sleep between scan passes (default: 10000ms)
    pub alloc_sleep_millisecs: u64,    // Sleep on allocation failure (default: 60000ms)
    pub max_ptes_none: usize,          // Max unmapped PTEs allowed during collapse (default: 511)
    pub max_ptes_swap: usize,          // Max swapped PTEs allowed during collapse (default: 64)
    pub max_ptes_shared: usize,        // Max shared PTEs allowed during collapse (default: 256)
    pub access_frequency_threshold: u32, // Access count required for 4KiB -> 2MiB promotion
}

impl Default for ThpPagePromotionPolicy {
    fn default() -> Self {
        Self {
            pages_to_scan: 4096,
            scan_sleep_millisecs: 10000,
            alloc_sleep_millisecs: 60000,
            max_ptes_none: 511,
            max_ptes_swap: 64,
            max_ptes_shared: 256,
            access_frequency_threshold: 10,
        }
    }
}

/// THP Memory Compactor for Zone Defragmentation
#[derive(Debug, Clone)]
pub struct ThpMemoryCompactor {
    pub defrag_mode: ThpDefragMode,
    pub total_compactions_run: u64,
    pub total_compacted_2mb_blocks: u64,
}

impl ThpMemoryCompactor {
    pub fn new(mode: ThpDefragMode) -> Self {
        Self {
            defrag_mode: mode,
            total_compactions_run: 0,
            total_compacted_2mb_blocks: 0,
        }
    }

    /// Compacts fragmented 4KiB page runs into aligned 2MiB physical blocks
    pub fn compact_zone_for_thp(
        &mut self,
        fragmented_phys_pages: &mut [u64],
    ) -> Result<Vec<u64>, &'static str> {
        if self.defrag_mode == ThpDefragMode::Never {
            return Err("THP Defrag is disabled");
        }

        self.total_compactions_run += 1;
        let mut aligned_2mb_blocks = Vec::new();

        // Sort physical page addresses to locate contiguous 2MiB runs
        fragmented_phys_pages.sort_unstable();

        let mut current_run = Vec::new();
        for &phys in fragmented_phys_pages.iter() {
            if current_run.is_empty() {
                if phys % (PAGE_SIZE_2MB as u64) == 0 {
                    current_run.push(phys);
                }
            } else {
                let last = *current_run.last().unwrap();
                if phys == last + (PAGE_SIZE_4KB as u64) {
                    current_run.push(phys);
                    if current_run.len() == PAGES_PER_2MB_HUGE_PAGE {
                        let base_2mb = current_run[0];
                        aligned_2mb_blocks.push(base_2mb);
                        self.total_compacted_2mb_blocks += 1;
                        current_run.clear();
                    }
                } else {
                    current_run.clear();
                    if phys % (PAGE_SIZE_2MB as u64) == 0 {
                        current_run.push(phys);
                    }
                }
            }
        }

        Ok(aligned_2mb_blocks)
    }
}

/// Background khugepaged THP Defrag & Runtime Page Promotion Daemon
pub struct ThpKhugepagedDefragDaemon {
    pub policy: ThpPagePromotionPolicy,
    pub compactor: ThpMemoryCompactor,
    pub huge_page_engine: SovereignHugePageEngine,
    pub pages_collapsed: u64,
    pub alloc_success: u64,
    pub alloc_fail: u64,
    pub scan_count: u64,
}

impl ThpKhugepagedDefragDaemon {
    pub fn new(
        thp_mode: TransparentHugePageMode,
        defrag_mode: ThpDefragMode,
    ) -> Self {
        Self {
            policy: ThpPagePromotionPolicy::default(),
            compactor: ThpMemoryCompactor::new(defrag_mode),
            huge_page_engine: SovereignHugePageEngine::new(thp_mode),
            pages_collapsed: 0,
            alloc_success: 0,
            alloc_fail: 0,
            scan_count: 0,
        }
    }

    /// Evaluates runtime page access counters and promotes 4KiB page ranges to 2MiB THPs
    pub fn evaluate_and_promote_page_range(
        &mut self,
        base_phys_2mb: u64,
        access_counts: &[u32; PAGES_PER_2MB_HUGE_PAGE],
        has_madv_hugepage: bool,
    ) -> Option<u64> {
        self.scan_count += 1;

        if self.huge_page_engine.thp_mode == TransparentHugePageMode::Never {
            self.alloc_fail += 1;
            return None;
        }

        if self.huge_page_engine.thp_mode == TransparentHugePageMode::Madvise && !has_madv_hugepage {
            self.alloc_fail += 1;
            return None;
        }

        let avg_access: u32 = access_counts.iter().sum::<u32>() / (PAGES_PER_2MB_HUGE_PAGE as u32);
        if !has_madv_hugepage && avg_access < self.policy.access_frequency_threshold {
            self.alloc_fail += 1;
            return None;
        }

        let mut contiguous_4k_pages = Vec::with_capacity(PAGES_PER_2MB_HUGE_PAGE);
        for i in 0..PAGES_PER_2MB_HUGE_PAGE {
            contiguous_4k_pages.push(base_phys_2mb + (i * PAGE_SIZE_4KB) as u64);
        }

        if let Some(promoted_2mb) = self.huge_page_engine.attempt_thp_coalesce(&contiguous_4k_pages) {
            self.pages_collapsed += PAGES_PER_2MB_HUGE_PAGE as u64;
            self.alloc_success += 1;
            Some(promoted_2mb)
        } else {
            self.alloc_fail += 1;
            None
        }
    }

    /// Triggers memory compaction on defragmentation requests and promotes newly formed 2MiB blocks
    pub fn trigger_khugepaged_scan_and_defrag(
        &mut self,
        fragmented_pages: &mut [u64],
    ) -> usize {
        self.scan_count += 1;

        if let Ok(compacted_blocks) = self.compactor.compact_zone_for_thp(fragmented_pages) {
            let mut promoted_count = 0;
            for block_base in compacted_blocks {
                let dummy_access = [20u32; PAGES_PER_2MB_HUGE_PAGE];
                if self.evaluate_and_promote_page_range(block_base, &dummy_access, true).is_some() {
                    promoted_count += 1;
                }
            }
            promoted_count
        } else {
            0
        }
    }
}

impl Default for ThpKhugepagedDefragDaemon {
    fn default() -> Self {
        Self::new(TransparentHugePageMode::Always, ThpDefragMode::Always)
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

    #[test]
    fn test_thp_khugepaged_daemon_and_compaction() {
        let mut daemon = ThpKhugepagedDefragDaemon::new(
            TransparentHugePageMode::Always,
            ThpDefragMode::Always,
        );

        let base_phys = 0x4000_0000u64; // 2MiB aligned
        let access_counts = [15u32; PAGES_PER_2MB_HUGE_PAGE];

        let promoted = daemon.evaluate_and_promote_page_range(base_phys, &access_counts, false);
        assert_eq!(promoted, Some(base_phys));
        assert_eq!(daemon.alloc_success, 1);
        assert_eq!(daemon.pages_collapsed, 512);

        let mut fragmented_run = Vec::new();
        let run_base = 0x6000_0000u64; // 2MiB aligned
        for i in 0..512 {
            fragmented_run.push(run_base + (i * 4096) as u64);
        }

        let promoted_count = daemon.trigger_khugepaged_scan_and_defrag(&mut fragmented_run);
        assert_eq!(promoted_count, 1);
        assert_eq!(daemon.compactor.total_compactions_run, 1);
        assert_eq!(daemon.compactor.total_compacted_2mb_blocks, 1);
    }
}
