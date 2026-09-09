#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── ARC Buffer Header ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ArcBufferHeader {
    pub block_id: u64,
    pub size_bytes: u32,
    pub access_count: u32,
    pub data: Vec<u8>,
}

impl ArcBufferHeader {
    pub fn new(block_id: u64, data: &[u8]) -> Self {
        ArcBufferHeader {
            block_id,
            size_bytes: data.len() as u32,
            access_count: 1,
            data: data.to_vec(),
        }
    }
}

// ─── Sovereign ZFS Adaptive Replacement Cache (ARC) ───────────────────────────

pub struct SovereignZfsArc {
    pub max_cache_blocks: usize,
    pub p_target_mru: usize,     // Adaptive target size for T1 (MRU)
    pub t1_mru: Vec<ArcBufferHeader>, // Most Recently Used cache
    pub t2_mfu: Vec<ArcBufferHeader>, // Most Frequently Used cache
    pub b1_ghost: Vec<u64>,           // Eviction ghost list for T1
    pub b2_ghost: Vec<u64>,           // Eviction ghost list for T2
    pub hits: u64,
    pub misses: u64,
}

impl SovereignZfsArc {
    pub fn new(max_cache_blocks: usize) -> Self {
        SovereignZfsArc {
            max_cache_blocks,
            p_target_mru: max_cache_blocks / 2,
            t1_mru: Vec::new(),
            t2_mfu: Vec::new(),
            b1_ghost: Vec::new(),
            b2_ghost: Vec::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Access or fetch a block from the Adaptive Replacement Cache
    pub fn access_block(&mut self, block_id: u64, fallback_data: &[u8]) -> Vec<u8> {
        // 1. Check T1 (MRU)
        if let Some(pos) = self.t1_mru.iter().position(|b| b.block_id == block_id) {
            self.hits = self.hits.saturating_add(1);
            let mut buf = self.t1_mru.remove(pos);
            buf.access_count = buf.access_count.saturating_add(1);
            let data = buf.data.clone();
            // Move to T2 (MFU) on repeated access
            self.t2_mfu.insert(0, buf);
            return data;
        }

        // 2. Check T2 (MFU)
        if let Some(pos) = self.t2_mfu.iter().position(|b| b.block_id == block_id) {
            self.hits = self.hits.saturating_add(1);
            let mut buf = self.t2_mfu.remove(pos);
            buf.access_count = buf.access_count.saturating_add(1);
            let data = buf.data.clone();
            self.t2_mfu.insert(0, buf);
            return data;
        }

        // Cache Miss
        self.misses = self.misses.saturating_add(1);

        // Check if block was in ghost list B1
        if let Some(pos) = self.b1_ghost.iter().position(|&id| id == block_id) {
            self.b1_ghost.remove(pos);
            // Increase target MRU size p
            self.p_target_mru = (self.p_target_mru + 1).min(self.max_cache_blocks);
        } else if let Some(pos) = self.b2_ghost.iter().position(|&id| id == block_id) {
            self.b2_ghost.remove(pos);
            // Decrease target MRU size p (bias towards frequency)
            self.p_target_mru = self.p_target_mru.saturating_sub(1);
        }

        // Evict if at capacity
        self.replace_evict();

        // Insert new block into T1 (MRU)
        let new_buf = ArcBufferHeader::new(block_id, fallback_data);
        let data = new_buf.data.clone();
        self.t1_mru.insert(0, new_buf);
        data
    }

    /// Adaptive replacement eviction logic
    fn replace_evict(&mut self) {
        let total_active = self.t1_mru.len() + self.t2_mfu.len();
        if total_active < self.max_cache_blocks {
            return;
        }

        if !self.t1_mru.is_empty() && (self.t1_mru.len() > self.p_target_mru || (self.t2_mfu.is_empty())) {
            // Evict from T1 into B1
            if let Some(evicted) = self.t1_mru.pop() {
                if self.b1_ghost.len() >= self.max_cache_blocks {
                    self.b1_ghost.pop();
                }
                self.b1_ghost.insert(0, evicted.block_id);
            }
        } else if !self.t2_mfu.is_empty() {
            // Evict from T2 into B2
            if let Some(evicted) = self.t2_mfu.pop() {
                if self.b2_ghost.len() >= self.max_cache_blocks {
                    self.b2_ghost.pop();
                }
                self.b2_ghost.insert(0, evicted.block_id);
            }
        }
    }

    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zfs_arc_basic_hit_miss() {
        let mut arc = SovereignZfsArc::new(4);
        let d1 = arc.access_block(100, b"data100");
        assert_eq!(d1, b"data100");
        assert_eq!(arc.misses, 1);
        assert_eq!(arc.hits, 0);

        // Immediate second access = cache hit
        let d2 = arc.access_block(100, b"fallback");
        assert_eq!(d2, b"data100");
        assert_eq!(arc.hits, 1);
        assert_eq!(arc.t2_mfu.len(), 1); // Promoted to MFU
    }

    #[test]
    fn test_zfs_arc_ghost_adaptation_b1() {
        let mut arc = SovereignZfsArc::new(2);
        arc.access_block(1, b"one");
        arc.access_block(2, b"two");
        // Capacity reached: access 3 causes eviction of 1 into B1 ghost
        arc.access_block(3, b"three");
        assert!(arc.b1_ghost.contains(&1));

        let initial_p = arc.p_target_mru;
        // Accessing block 1 from B1 ghost increases target p
        arc.access_block(1, b"one_again");
        assert!(arc.p_target_mru >= initial_p);
    }

    #[test]
    fn test_zfs_arc_mfu_promotion() {
        let mut arc = SovereignZfsArc::new(4);
        arc.access_block(10, b"content");
        assert_eq!(arc.t1_mru.len(), 1);
        assert_eq!(arc.t2_mfu.len(), 0);

        arc.access_block(10, b"content");
        assert_eq!(arc.t1_mru.len(), 0);
        assert_eq!(arc.t2_mfu.len(), 1);
    }

    #[test]
    fn test_zfs_arc_hit_ratio() {
        let mut arc = SovereignZfsArc::new(4);
        arc.access_block(1, b"a"); // miss
        arc.access_block(1, b"a"); // hit
        assert_eq!(arc.hit_ratio(), 0.5);
    }

    #[test]
    fn test_zfs_arc_capacity_invariant() {
        let mut arc = SovereignZfsArc::new(3);
        for i in 1..=10 {
            arc.access_block(i, b"bulk");
        }
        let total_cached = arc.t1_mru.len() + arc.t2_mfu.len();
        assert!(total_cached <= 3);
    }

    #[test]
    fn test_zfs_arc_empty_ratio() {
        let arc = SovereignZfsArc::new(4);
        assert_eq!(arc.hit_ratio(), 0.0);
    }
}
