#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]


#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── KSM Page Descriptor ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KsmPage {
    pub pid: u32,
    pub virt_addr: u64,
    pub content_hash: u64,
    pub is_cow: bool,
    pub shared_count: u32,
}

impl KsmPage {
    pub fn new(pid: u32, virt_addr: u64, data: &[u8]) -> Self {
        // Pure-Rust 64-bit FNV-1a hash calculation for zero-dependency page identity
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        KsmPage {
            pid,
            virt_addr,
            content_hash: hash,
            is_cow: false,
            shared_count: 1,
        }
    }
}

// ─── Sovereign KSM Memory Deduplicator ────────────────────────────────────────

pub struct SovereignKsmEngine {
    pub stable_tree: Vec<KsmPage>,   // Already merged CoW pages
    pub unstable_tree: Vec<KsmPage>, // Candidate pages awaiting duplicate matches
    pub page_size: usize,
    pub scan_sleep_ticks: u32,
    pub pages_scanned: u64,
    pub pages_merged: u64,
    pub bytes_saved: u64,
}

impl SovereignKsmEngine {
    pub fn new(page_size: usize) -> Self {
        SovereignKsmEngine {
            stable_tree: Vec::new(),
            unstable_tree: Vec::new(),
            page_size,
            scan_sleep_ticks: 20,
            pages_scanned: 0,
            pages_merged: 0,
            bytes_saved: 0,
        }
    }

    /// Register a candidate memory page (`madvise(addr, len, MADV_MERGEABLE)`)
    pub fn mergeable_page(&mut self, pid: u32, virt_addr: u64, data: &[u8]) -> bool {
        self.pages_scanned = self.pages_scanned.saturating_add(1);
        let candidate = KsmPage::new(pid, virt_addr, data);

        // 1. Check Stable Tree for existing merged identical page
        if let Some(stable_page) = self.stable_tree.iter_mut().find(|p| p.content_hash == candidate.content_hash) {
            stable_page.shared_count = stable_page.shared_count.saturating_add(1);
            self.pages_merged = self.pages_merged.saturating_add(1);
            self.bytes_saved = self.bytes_saved.saturating_add(self.page_size as u64);
            return true;
        }

        // 2. Check Unstable Tree for duplicate candidate
        if let Some(pos) = self.unstable_tree.iter().position(|p| p.content_hash == candidate.content_hash) {
            let mut matched = self.unstable_tree.remove(pos);
            matched.is_cow = true;
            matched.shared_count = 2; // Matched candidate + new candidate
            self.stable_tree.push(matched);

            self.pages_merged = self.pages_merged.saturating_add(1);
            self.bytes_saved = self.bytes_saved.saturating_add(self.page_size as u64);
            return true;
        }

        // 3. No match yet -> Insert into unstable tree for future deduplication
        self.unstable_tree.push(candidate);
        false
    }

    /// Break CoW sharing when a process writes to a shared page
    pub fn break_cow(&mut self, content_hash: u64) -> bool {
        if let Some(pos) = self.stable_tree.iter().position(|p| p.content_hash == content_hash) {
            let page = &mut self.stable_tree[pos];
            if page.shared_count > 2 {
                page.shared_count -= 1;
            } else {
                // Only 1 left sharing, demote from stable tree
                self.stable_tree.remove(pos);
            }
            self.bytes_saved = self.bytes_saved.saturating_sub(self.page_size as u64);
            true
        } else {
            false
        }
    }

    pub fn pages_shared(&self) -> usize {
        self.stable_tree.len()
    }

    pub fn pages_sharing(&self) -> u32 {
        self.stable_tree.iter().map(|p| p.shared_count.saturating_sub(1)).sum()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ksm_initial_state() {
        let ksm = SovereignKsmEngine::new(4096);
        assert_eq!(ksm.pages_scanned, 0);
        assert_eq!(ksm.pages_merged, 0);
        assert_eq!(ksm.bytes_saved, 0);
    }

    #[test]
    fn test_ksm_unstable_candidate_insertion() {
        let mut ksm = SovereignKsmEngine::new(4096);
        // First unique page goes to unstable tree
        let merged = ksm.mergeable_page(100, 0x1000, b"unique data 1");
        assert!(!merged);
        assert_eq!(ksm.unstable_tree.len(), 1);
        assert_eq!(ksm.stable_tree.len(), 0);
    }

    #[test]
    fn test_ksm_duplicate_promoted_to_stable() {
        let mut ksm = SovereignKsmEngine::new(4096);
        ksm.mergeable_page(100, 0x1000, b"identical content");
        // Second identical page merges and creates stable CoW entry
        let merged = ksm.mergeable_page(200, 0x2000, b"identical content");
        assert!(merged);
        assert_eq!(ksm.unstable_tree.len(), 0);
        assert_eq!(ksm.stable_tree.len(), 1);
        assert_eq!(ksm.pages_shared(), 1);
        assert_eq!(ksm.pages_sharing(), 1);
        assert_eq!(ksm.bytes_saved, 4096);
    }

    #[test]
    fn test_ksm_multi_sharing_deduplication() {
        let mut ksm = SovereignKsmEngine::new(4096);
        ksm.mergeable_page(100, 0x1000, b"zero page");
        ksm.mergeable_page(200, 0x2000, b"zero page");
        ksm.mergeable_page(300, 0x3000, b"zero page"); // 3rd identical page

        assert_eq!(ksm.stable_tree[0].shared_count, 3);
        assert_eq!(ksm.pages_sharing(), 2);
        assert_eq!(ksm.bytes_saved, 8192); // 2 extra pages saved
    }

    #[test]
    fn test_ksm_break_cow_on_write() {
        let mut ksm = SovereignKsmEngine::new(4096);
        ksm.mergeable_page(100, 0x1000, b"shared data");
        ksm.mergeable_page(200, 0x2000, b"shared data");
        let hash = ksm.stable_tree[0].content_hash;

        assert!(ksm.break_cow(hash));
        assert_eq!(ksm.bytes_saved, 0);
    }

    #[test]
    fn test_ksm_break_cow_nonexistent() {
        let mut ksm = SovereignKsmEngine::new(4096);
        assert!(!ksm.break_cow(0xDEADBEEF));
    }
}
