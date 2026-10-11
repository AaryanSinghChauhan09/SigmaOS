// SigmaOS Anti-Fragmentation Memory Compaction Engine
// Scans buddy allocator free lists and compacts adjacent blocks to satisfy high-order allocations.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

pub const MAX_ORDER: usize = 11;
pub const PAGE_SIZE: usize = 4096;

/// Page migration status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionStatus {
    Success,
    Partial,
    NoMemory,
    Skipped,
}

/// Free block in physical memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreeBlock {
    pub pfn: u64,
    pub order: usize,
}

/// Memory Compactor for Buddy Allocator Free Lists
#[derive(Debug, Clone)]
pub struct PageCompactor {
    pub free_lists: [Vec<u64>; MAX_ORDER],
    pub total_pages: usize,
    pub migrated_pages: usize,
}

impl PageCompactor {
    pub fn new(total_pages: usize) -> Self {
        const EMPTY_LIST: Vec<u64> = Vec::new();
        Self {
            free_lists: [
                EMPTY_LIST, EMPTY_LIST, EMPTY_LIST, EMPTY_LIST,
                EMPTY_LIST, EMPTY_LIST, EMPTY_LIST, EMPTY_LIST,
                EMPTY_LIST, EMPTY_LIST, EMPTY_LIST,
            ],
            total_pages,
            migrated_pages: 0,
        }
    }

    /// Add a free block PFN at specific order
    pub fn add_free_block(&mut self, pfn: u64, order: usize) {
        if order < MAX_ORDER {
            self.free_lists[order].push(pfn);
            self.free_lists[order].sort_unstable();
        }
    }

    /// Calculate external fragmentation index (0 to 100)
    pub fn external_fragmentation_index(&self, target_order: usize) -> u32 {
        let mut total_free = 0usize;
        let mut target_free = 0usize;

        for (ord, list) in self.free_lists.iter().enumerate() {
            let blocks = list.len();
            let pages = blocks * (1 << ord);
            total_free += pages;
            if ord >= target_order {
                target_free += pages;
            }
        }

        if total_free == 0 {
            return 0;
        }

        let usable_ratio = (target_free * 100) / total_free;
        100 - (usable_ratio as u32).min(100)
    }

    /// Perform a compaction pass to assemble at least one block of `target_order`
    pub fn compact_for_order(&mut self, target_order: usize) -> CompactionStatus {
        if target_order >= MAX_ORDER {
            return CompactionStatus::Skipped;
        }

        // If target order already has free blocks, no compaction needed
        if !self.free_lists[target_order].is_empty() {
            return CompactionStatus::Success;
        }

        // Try merging lower orders iteratively up to target_order
        for ord in 0..target_order {
            let mut i = 0;
            while i + 1 < self.free_lists[ord].len() {
                let pfn1 = self.free_lists[ord][i];
                let pfn2 = self.free_lists[ord][i + 1];
                let buddy_span = 1 << ord;

                // Check if pfn1 and pfn2 are mutual buddies
                if pfn1 + buddy_span == pfn2 && (pfn1 % (2 * buddy_span) == 0) {
                    // Merge into next higher order
                    self.free_lists[ord].remove(i + 1);
                    self.free_lists[ord].remove(i);
                    self.free_lists[ord + 1].push(pfn1);
                    self.free_lists[ord + 1].sort_unstable();
                    self.migrated_pages += 2 * (1 << ord);
                } else {
                    i += 1;
                }
            }
        }

        if !self.free_lists[target_order].is_empty() {
            CompactionStatus::Success
        } else if self.migrated_pages > 0 {
            CompactionStatus::Partial
        } else {
            CompactionStatus::NoMemory
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_compaction_merging() {
        let mut compactor = PageCompactor::new(1024);
        // Add adjacent order-0 buddies
        compactor.add_free_block(0, 0);
        compactor.add_free_block(1, 0);
        compactor.add_free_block(2, 0);
        compactor.add_free_block(3, 0);

        assert_eq!(compactor.compact_for_order(2), CompactionStatus::Success);
        assert!(!compactor.free_lists[2].is_empty());
        assert_eq!(compactor.free_lists[2][0], 0);
    }
}
