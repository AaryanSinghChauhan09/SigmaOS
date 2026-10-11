// SigmaOS Kernel Samepage Merging (KSM) Daemon
// Identifies identical anonymous memory pages and merges them into shared read-only CoW frames.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const PAGE_SIZE: usize = 4096;

/// Stable merged tree node
#[derive(Debug, Clone)]
pub struct KsmMergedPage {
    pub pfn: u64,
    pub hash: u64,
    pub shared_count: usize,
    pub pids: Vec<u32>,
}

/// KSM Background Scanner Daemon
#[derive(Debug, Clone, Default)]
pub struct KsmDaemon {
    pub stable_tree: BTreeMap<u64, KsmMergedPage>, // Hash -> Merged page
    pub scan_pointer: u64,
    pub pages_scanned: u64,
    pub pages_shared: u64,
    pub pages_sharing: u64,
    pub pages_unshared: u64,
}

impl KsmDaemon {
    pub fn new() -> Self {
        Self {
            stable_tree: BTreeMap::new(),
            scan_pointer: 0,
            pages_scanned: 0,
            pages_shared: 0,
            pages_sharing: 0,
            pages_unshared: 0,
        }
    }

    /// Fast 64-bit FNV-1a hash of page content for fast deduplication lookup
    pub fn hash_page(data: &[u8; PAGE_SIZE]) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    /// Submit a candidate page for potential KSM merging
    pub fn consider_page(&mut self, pfn: u64, data: &[u8; PAGE_SIZE], pid: u32) -> Option<u64> {
        self.pages_scanned += 1;
        let hash = Self::hash_page(data);

        if let Some(merged) = self.stable_tree.get_mut(&hash) {
            // Found duplicate page! Merge candidate pfn into existing shared pfn
            merged.shared_count += 1;
            merged.pids.push(pid);
            self.pages_sharing += 1;
            Some(merged.pfn)
        } else {
            // New unique page in stable tree
            let merged = KsmMergedPage {
                pfn,
                hash,
                shared_count: 1,
                pids: alloc::vec![pid],
            };
            self.stable_tree.insert(hash, merged);
            self.pages_shared += 1;
            None
        }
    }

    /// Break sharing when a write occurs on a CoW page
    pub fn break_cow(&mut self, pfn: u64, pid: u32) {
        for merged in self.stable_tree.values_mut() {
            if merged.pfn == pfn {
                if let Some(pos) = merged.pids.iter().position(|&p| p == pid) {
                    merged.pids.remove(pos);
                    merged.shared_count = merged.shared_count.saturating_sub(1);
                    self.pages_sharing = self.pages_sharing.saturating_sub(1);
                    self.pages_unshared += 1;
                }
                break;
            }
        }
    }

    /// Calculate total bytes saved by KSM deduplication
    pub fn memory_savings_bytes(&self) -> usize {
        (self.pages_sharing as usize) * PAGE_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ksm_page_merging() {
        let mut ksm = KsmDaemon::new();
        let page_zeros = [0u8; PAGE_SIZE];

        let target_pfn = ksm.consider_page(100, &page_zeros, 1);
        assert_eq!(target_pfn, None); // First instance creates entry

        let second_pfn = ksm.consider_page(200, &page_zeros, 2);
        assert_eq!(second_pfn, Some(100)); // Second instance merges into PFN 100

        assert_eq!(ksm.memory_savings_bytes(), 4096);
    }
}
