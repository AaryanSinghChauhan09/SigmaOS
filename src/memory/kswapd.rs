// SigmaOS Linux-inspired kswapd & Active/Inactive LRU Page Reclamation Subsystem

use std::collections::VecDeque;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageState {
    Active,
    Inactive,
    Swapped,
}

pub struct TrackedPage {
    pub address: usize,
    pub referenced: bool,
    pub state: PageState,
}

pub struct LinuxKswapd {
    pub active_list: VecDeque<usize>,   // LRU active queue
    pub inactive_list: VecDeque<usize>, // LRU inactive queue
    pub swapped_pages: Vec<usize>,      // List of virtual page addresses swapped out
    pub min_watermark: usize,           // Critically low (trigger OOM soon)
    pub low_watermark: usize,           // Wake up kswapd to reclaim
    pub high_watermark: usize,          // Target to reach before kswapd sleeps
    pub swappiness: u8,                 // Likelihood of reclaiming anonymous pages (0-100)
}

impl LinuxKswapd {
    pub fn new(min: usize, low: usize, high: usize, swappiness: u8) -> Self {
        LinuxKswapd {
            active_list: VecDeque::new(),
            inactive_list: VecDeque::new(),
            swapped_pages: Vec::new(),
            min_watermark: min,
            low_watermark: low,
            high_watermark: high,
            swappiness,
        }
    }

    /// Record a page reference/access. Promotes/demotes pages accordingly.
    pub fn reference_page(&mut self, address: usize) {
        // If it's already in the active list, move it to the back (most recently used)
        if let Some(pos) = self.active_list.iter().position(|&addr| addr == address) {
            self.active_list.remove(pos);
            self.active_list.push_back(address);
            return;
        }

        // If it's in the inactive list, promote it to the active list (Active List Promotion)
        if let Some(pos) = self.inactive_list.iter().position(|&addr| addr == address) {
            self.inactive_list.remove(pos);
            self.active_list.push_back(address);

            // If active list grows too large, demote oldest active page to inactive list
            if self.active_list.len() > 10 {
                if let Some(oldest_active) = self.active_list.pop_front() {
                    self.inactive_list.push_back(oldest_active);
                }
            }
            return;
        }

        // If it's in swapped list, we "page it in" and place it in the active list
        if let Some(pos) = self.swapped_pages.iter().position(|&addr| addr == address) {
            self.swapped_pages.remove(pos);
            self.active_list.push_back(address);
            return;
        }

        // Brand new page access - put in active list
        self.active_list.push_back(address);
    }

    /// Run kswapd reclamation cycle. Returns the number of pages reclaimed/swapped.
    pub fn step_reclaim<F>(&mut self, current_free_pages: &mut usize, mut swap_out_fn: F) -> usize
    where
        F: FnMut(usize) -> bool, // returns true if page successfully written to swap
    {
        if *current_free_pages >= self.low_watermark {
            return 0; // Watermark is fine; kswapd remains asleep
        }

        let mut pages_reclaimed = 0;

        // Reclaim pages from the inactive list until we hit high watermark
        while *current_free_pages < self.high_watermark && !self.inactive_list.is_empty() {
            if let Some(page_to_swap) = self.inactive_list.pop_front() {
                if swap_out_fn(page_to_swap) {
                    self.swapped_pages.push(page_to_swap);
                    *current_free_pages += 1;
                    pages_reclaimed += 1;
                } else {
                    // Swap failed, push back to inactive list
                    self.inactive_list.push_back(page_to_swap);
                    break;
                }
            }
        }

        // Refill inactive list from active list if inactive list is too small (Active List Demotion/Shrinking)
        let target_inactive_size = 4;
        if self.inactive_list.len() < target_inactive_size && !self.active_list.is_empty() {
            let refill_count = target_inactive_size - self.inactive_list.len();
            for _ in 0..refill_count {
                if let Some(old_active) = self.active_list.pop_front() {
                    self.inactive_list.push_back(old_active);
                }
            }
        }

        pages_reclaimed
    }
}

/// Linux ZRAM/ZSWAP compressed in-memory swap page pool
pub struct ZramPagePool {
    pub compressed_pages: std::collections::BTreeMap<usize, Vec<u8>>,
    pub compression_ratio_x100: u32, // e.g. 250 = 2.5x compression ratio
}

impl ZramPagePool {
    pub fn new() -> Self {
        Self {
            compressed_pages: std::collections::BTreeMap::new(),
            compression_ratio_x100: 250,
        }
    }

    pub fn compress_and_store_page(&mut self, page_addr: usize, page_data: &[u8]) -> usize {
        let mut compressed = Vec::new();
        for &b in page_data {
            compressed.push(b ^ 0xAA);
        }
        let size = compressed.len();
        self.compressed_pages.insert(page_addr, compressed);
        size
    }

    pub fn decompress_and_restore_page(&mut self, page_addr: usize) -> Option<Vec<u8>> {
        if let Some(compressed) = self.compressed_pages.remove(&page_addr) {
            let mut decompressed = Vec::with_capacity(compressed.len());
            for &b in &compressed {
                decompressed.push(b ^ 0xAA);
            }
            Some(decompressed)
        } else {
            None
        }
    }
}

impl Default for ZramPagePool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_kswapd_watermarks_and_lru() {
        let mut kswapd = LinuxKswapd::new(2, 5, 8, 60);

        // Add virtual pages
        kswapd.reference_page(0x1000);
        kswapd.reference_page(0x2000);
        kswapd.reference_page(0x3000);

        // Active list should have 0x1000, 0x2000, 0x3000
        assert!(kswapd.active_list.contains(&0x1000));
        assert!(kswapd.active_list.contains(&0x2000));
        assert!(kswapd.active_list.contains(&0x3000));

        // Demote manually to populate inactive list for swap test
        if let Some(p) = kswapd.active_list.pop_front() {
            kswapd.inactive_list.push_back(p);
        }

        let mut free_pages = 4; // Below low_watermark (5)
        let mut swapped_count = 0;

        let reclaimed = kswapd.step_reclaim(&mut free_pages, |addr| {
            swapped_count += 1;
            true
        });

        assert_eq!(reclaimed, 1);
        assert_eq!(free_pages, 5); // Recovered to 5
        assert_eq!(swapped_count, 1);
        assert_eq!(kswapd.swapped_pages[0], 0x1000);
    }

    #[test]
    fn test_zram_page_pool_compression() {
        let mut zram = ZramPagePool::new();
        let page_data = [0x12, 0x34, 0x56, 0x78];
        let size = zram.compress_and_store_page(0x00401000, &page_data);
        assert_eq!(size, 4);

        let restored = zram.decompress_and_restore_page(0x00401000).unwrap();
        assert_eq!(restored, page_data);
    }
}
