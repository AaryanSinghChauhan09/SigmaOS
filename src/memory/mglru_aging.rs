// SPDX-License-Identifier: MIT
// Multi-Generational LRU (MGLRU) Page Aging Subsystem for SigmaOS (`src/memory/mglru_aging.rs`)
// Inspired by Linux MGLRU (mm/vmscan.c, CONFIG_LRU_GEN) and FreeBSD Page Daemon (sys/vm/vm_pageout.c)
// Implements multi-generational page aging queues (Max 4 Generations), Page Table Access Bit scanning,
// Generation promotion/demotion thresholds, and memory pressure eviction queues.

use std::collections::{BTreeMap, VecDeque};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

pub const MAX_MGLRU_GENERATIONS: usize = 4;
pub const PAGE_ACCESSED_BIT: u8 = 1 << 0;
pub const PAGE_DIRTY_BIT: u8 = 1 << 1;

/// MGLRU Page Descriptor
#[derive(Debug, Clone)]
pub struct MglruPageDescriptor {
    pub pfn: u64,
    pub generation: usize, // 0 = Oldest/Coldest (Eviction target), 3 = Youngest/Hottest
    pub access_flags: u8,
    pub ref_count: u32,
    pub last_access_timestamp: u64,
}

/// MGLRU Multi-Generational Memory Page Aging Engine
pub struct MglruPageAgingEngine {
    pub generations: [VecDeque<u64>; MAX_MGLRU_GENERATIONS], // Generation ID -> Queue of PFNs
    pub page_map: BTreeMap<u64, MglruPageDescriptor>,        // PFN -> Descriptor
    pub current_max_generation: usize,
    pub total_pages_evicted: u64,
    pub total_aging_scans: u64,
}

impl MglruPageAgingEngine {
    pub fn new() -> Self {
        const EMPTY_QUEUE: VecDeque<u64> = VecDeque::new();
        Self {
            generations: [EMPTY_QUEUE; MAX_MGLRU_GENERATIONS],
            page_map: BTreeMap::new(),
            current_max_generation: MAX_MGLRU_GENERATIONS - 1,
            total_pages_evicted: 0,
            total_aging_scans: 0,
        }
    }

    /// Track a new physical page frame into the youngest generation (Gen 3)
    pub fn track_page(&mut self, pfn: u64, is_dirty: bool) {
        let flags = PAGE_ACCESSED_BIT | if is_dirty { PAGE_DIRTY_BIT } else { 0 };
        let youngest_gen = MAX_MGLRU_GENERATIONS - 1;

        let desc = MglruPageDescriptor {
            pfn,
            generation: youngest_gen,
            access_flags: flags,
            ref_count: 1,
            last_access_timestamp: 100, // Initial timestamp
        };

        self.page_map.insert(pfn, desc);
        self.generations[youngest_gen].push_back(pfn);
    }

    /// Record a page access event (Page Table Reference Bit set)
    pub fn record_page_access(&mut self, pfn: u64, timestamp: u64) {
        if let Some(desc) = self.page_map.get_mut(&pfn) {
            desc.access_flags |= PAGE_ACCESSED_BIT;
            desc.last_access_timestamp = timestamp;
            desc.ref_count += 1;

            // Promote page to youngest generation if accessed
            let old_gen = desc.generation;
            let youngest_gen = MAX_MGLRU_GENERATIONS - 1;
            if old_gen < youngest_gen {
                if let Some(pos) = self.generations[old_gen].iter().position(|&p| p == pfn) {
                    self.generations[old_gen].remove(pos);
                }
                desc.generation = youngest_gen;
                self.generations[youngest_gen].push_back(pfn);
            }
        }
    }

    /// Execute a Linux-style Page Table Aging Scan (Sweeps access bits & demotes inactive pages)
    pub fn run_aging_scan(&mut self) {
        self.total_aging_scans += 1;

        // Snapshot existing PFNs per generation before demotions to ensure single-step aging per scan
        let mut generation_snapshots: [Vec<u64>; MAX_MGLRU_GENERATIONS] = Default::default();
        for gen in 1..MAX_MGLRU_GENERATIONS {
            generation_snapshots[gen] = self.generations[gen].iter().copied().collect();
        }

        for gen in 1..MAX_MGLRU_GENERATIONS {
            for pfn in generation_snapshots[gen].clone() {
                if let Some(desc) = self.page_map.get_mut(&pfn) {
                    if (desc.access_flags & PAGE_ACCESSED_BIT) == 0 {
                        // Demote by 1 generation
                        if let Some(pos) = self.generations[gen].iter().position(|&p| p == pfn) {
                            self.generations[gen].remove(pos);
                        }
                        let target_gen = gen - 1;
                        desc.generation = target_gen;
                        self.generations[target_gen].push_back(pfn);
                    } else {
                        // Clear accessed bit for next aging cycle
                        desc.access_flags &= !PAGE_ACCESSED_BIT;
                    }
                }
            }
        }
    }

    /// Reclaim/Evict oldest cold pages from Generation 0 under memory pressure
    pub fn reclaim_cold_pages(&mut self, target_page_count: usize) -> Vec<u64> {
        let mut evicted = Vec::new();

        while evicted.len() < target_page_count {
            // Pick from Generation 0 (oldest cold pages)
            if let Some(pfn) = self.generations[0].pop_front() {
                self.page_map.remove(&pfn);
                evicted.push(pfn);
                self.total_pages_evicted += 1;
            } else {
                // If Gen 0 is empty, trigger an aging scan to populate Gen 0 from Gen 1..3
                self.run_aging_scan();
                if self.generations[0].is_empty() {
                    break; // No more pages available to reclaim
                }
            }
        }

        evicted
    }
}

impl Default for MglruPageAgingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mglru_page_tracking_and_access_promotion() {
        let mut engine = MglruPageAgingEngine::new();
        engine.track_page(0x1000, false);
        engine.track_page(0x2000, false);

        assert_eq!(engine.generations[3].len(), 2);

        // Run aging scan without access -> Scan 1 clears accessed bit, Scans 2..4 demote Gen 3 -> 2 -> 1 -> 0
        engine.run_aging_scan();
        engine.run_aging_scan();
        engine.run_aging_scan();
        engine.run_aging_scan();

        assert_eq!(engine.generations[0].len(), 2);

        // Access page 0x1000 -> Promotes back to Gen 3
        engine.record_page_access(0x1000, 200);
        assert_eq!(engine.generations[3].len(), 1);
        assert_eq!(engine.generations[0].len(), 1);
    }

    #[test]
    fn test_mglru_cold_page_reclamation() {
        let mut engine = MglruPageAgingEngine::new();
        engine.track_page(0x5000, false);
        engine.track_page(0x6000, false);

        // Force demotion to Gen 0
        engine.run_aging_scan();
        engine.run_aging_scan();
        engine.run_aging_scan();
        engine.run_aging_scan();

        let reclaimed = engine.reclaim_cold_pages(2);
        assert_eq!(reclaimed.len(), 2);
        assert_eq!(engine.total_pages_evicted, 2);
        assert!(engine.page_map.is_empty());
    }
}
