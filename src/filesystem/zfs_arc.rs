//! ZFS Adaptive Replacement Cache (ARC) Implementation
//! Inspired by ZFS ARC algorithm from OpenZFS and FreeBSD
//! Reference: FreeBSD sys/cddl/contrib/opensolaris/uts/common/fs/zfs/arc.c

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// ZFS ARC cache entry representing a cached block
#[derive(Debug, Clone)]
pub struct ArcEntry {
    pub key: u64,         // Block address
    pub data: Vec<u8>,    // Block data
    pub compressed: bool, // Whether data is compressed
    pub ref_count: u32,   // Reference count
}

/// ARC statistics counters
#[derive(Debug, Clone, Copy, Default)]
pub struct ArcStats {
    pub hits: u64,       // Cache hits
    pub misses: u64,     // Cache misses
    pub evictions: u64,  // Total evictions
    pub ghost_hits: u64, // Ghost list hits (adaptation triggers)
}

/// ARC configuration parameters
#[derive(Debug, Clone, Copy)]
pub struct ArcConfig {
    pub min_size: usize, // Minimum cache size (bytes)
    pub max_size: usize, // Maximum cache size (bytes)
    pub compression_enabled: bool,
}

impl Default for ArcConfig {
    fn default() -> Self {
        Self {
            min_size: 64 * 1024 * 1024,  // 64 MB
            max_size: 512 * 1024 * 1024, // 512 MB
            compression_enabled: true,
        }
    }
}

/// Main ZFS ARC cache controller
///
/// Implements the ARC algorithm with:
/// - T1: Recently used once (MRU)
/// - T2: Frequently used (MFU)
/// - B1: Ghost entries evicted from T1
/// - B2: Ghost entries evicted from T2
/// - P: Adaptive parameter (target size for T1)
pub struct ZfsArc {
    pub size: usize, // Current cache size in bytes
    pub c: usize,    // Target cache size
    pub p: usize,    // Adaptive parameter (target T1 size)

    pub t1: Vec<ArcEntry>, // Recently used once (MRU)
    pub t2: Vec<ArcEntry>, // Frequently used (MFU)
    pub b1: Vec<u64>,      // Ghost list for T1 (keys only)
    pub b2: Vec<u64>,      // Ghost list for T2 (keys only)

    pub stats: ArcStats,
    config: ArcConfig,
}

impl ZfsArc {
    /// Create a new ARC cache with given configuration
    pub fn new(config: ArcConfig) -> Self {
        Self {
            size: 0,
            c: config.max_size,
            p: config.max_size / 2, // Start with balanced T1/T2
            t1: Vec::new(),
            t2: Vec::new(),
            b1: Vec::new(),
            b2: Vec::new(),
            stats: ArcStats::default(),
            config,
        }
    }

    /// Look up a block in the cache
    pub fn lookup(&mut self, key: u64) -> Option<&[u8]> {
        // Search T1 (MRU)
        for i in 0..self.t1.len() {
            if self.t1[i].key == key {
                self.stats.hits += 1;
                // Move from T1 to T2 (promote to frequent)
                let entry = self.t1.remove(i);
                self.t2.push(entry);
                return Some(&self.t2[self.t2.len() - 1].data);
            }
        }

        // Search T2 (MFU)
        for i in 0..self.t2.len() {
            if self.t2[i].key == key {
                self.stats.hits += 1;
                // Move to end (LRU ordering)
                let entry = self.t2.remove(i);
                self.t2.push(entry);
                return Some(&self.t2[self.t2.len() - 1].data);
            }
        }

        self.stats.misses += 1;
        None
    }

    /// Insert a new block into the cache
    pub fn insert(&mut self, key: u64, data: Vec<u8>) {
        let entry_size = data.len();

        // An entry larger than the entire cache cannot be retained. Skip it
        // instead of evicting every resident block and then looping forever.
        if entry_size > self.c {
            return;
        }

        // Replacing an existing key promotes it to T2 and updates accounting.
        let was_cached = if let Some(index) = self.t1.iter().position(|entry| entry.key == key) {
            let old_entry = self.t1.remove(index);
            self.size = self.size.saturating_sub(old_entry.data.len());
            true
        } else if let Some(index) = self.t2.iter().position(|entry| entry.key == key) {
            let old_entry = self.t2.remove(index);
            self.size = self.size.saturating_sub(old_entry.data.len());
            true
        } else {
            false
        };

        // Check ghost lists for adaptation
        if !was_cached && Self::remove_from_ghost(&mut self.b1, key) {
            self.stats.ghost_hits += 1;
            self.adapt_p(true); // Hit in B1, increase P (favor MRU)
        } else if !was_cached && Self::remove_from_ghost(&mut self.b2, key) {
            self.stats.ghost_hits += 1;
            self.adapt_p(false); // Hit in B2, decrease P (favor MFU)
        }

        // Ensure space by evicting if necessary
        while self.size > self.c - entry_size {
            let size_before_eviction = self.size;
            self.evict_to_target();
            if self.size == size_before_eviction {
                return;
            }
        }

        // Insert into T1 (newly accessed)
        let entry = ArcEntry {
            key,
            data,
            compressed: false,
            ref_count: 1,
        };
        self.size += entry_size;
        if was_cached {
            self.t2.push(entry);
        } else {
            self.t1.push(entry);
        }
    }

    /// Evict entries to reach target cache size
    pub fn evict_to_target(&mut self) {
        if self.t1.is_empty() && self.t2.is_empty() {
            return;
        }

        // P is a byte target for T1. Compare byte usage rather than entry
        // counts because cache blocks can have different sizes.
        let t1_size = self.t1.iter().fold(0usize, |total, entry| {
            total.saturating_add(entry.data.len())
        });
        let evict_from_t1 = !self.t1.is_empty() && (self.t2.is_empty() || t1_size > self.p);

        if evict_from_t1 && !self.t1.is_empty() {
            let entry = self.t1.remove(0);
            self.size = self.size.saturating_sub(entry.data.len());
            self.stats.evictions += 1;
            // Add to ghost list B1
            if self.b1.len() < 1000 {
                // Limit ghost list size
                self.b1.push(entry.key);
            }
        } else if !self.t2.is_empty() {
            let entry = self.t2.remove(0);
            self.size = self.size.saturating_sub(entry.data.len());
            self.stats.evictions += 1;
            // Add to ghost list B2
            if self.b2.len() < 1000 {
                self.b2.push(entry.key);
            }
        }
    }

    /// Adapt the P parameter based on ghost list hits
    pub fn adapt_p(&mut self, ghost_hit_mru: bool) {
        let delta = (self.c / 100).max(1); // 1% of cache size, minimum 1

        if ghost_hit_mru {
            // Hit in B1: increase P (favor MRU/T1)
            self.p = self.p.saturating_add(delta).min(self.c);
        } else {
            // Hit in B2: decrease P (favor MFU/T2)
            self.p = self.p.saturating_sub(delta);
        }
    }

    /// Handle memory pressure by freeing specified bytes
    pub fn memory_pressure(&mut self, bytes_to_free: usize) {
        let target_size = self.size.saturating_sub(bytes_to_free);
        while self.size > target_size && (!self.t1.is_empty() || !self.t2.is_empty()) {
            self.evict_to_target();
        }
    }

    // Helper: remove key from ghost list
    fn remove_from_ghost(ghost_list: &mut Vec<u64>, key: u64) -> bool {
        if let Some(pos) = ghost_list.iter().position(|&k| k == key) {
            ghost_list.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get current statistics
    pub fn get_stats(&self) -> ArcStats {
        self.stats
    }

    /// Return the cache configuration used to initialize this controller.
    pub fn config(&self) -> ArcConfig {
        self.config
    }

    /// Get cache hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let total = self.stats.hits + self.stats.misses;
        if total == 0 {
            0.0
        } else {
            self.stats.hits as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_arc_basic_insert_lookup() {
        let config = ArcConfig {
            min_size: 1024,
            max_size: 4096,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);

        // Insert block
        let data = vec![1u8, 2, 3, 4];
        arc.insert(100, data.clone());

        // Lookup should succeed
        let result = arc.lookup(100);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), &[1u8, 2, 3, 4]);
        assert_eq!(arc.stats.hits, 1);

        // After first hit, should be in T2 (MFU)
        assert_eq!(arc.t2.len(), 1);
    }

    #[test]
    fn test_arc_eviction_mru_mfu_balance() {
        let config = ArcConfig {
            min_size: 512,
            max_size: 1024,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);

        // Fill cache beyond capacity
        for i in 0..10 {
            arc.insert(i, vec![0u8; 200]);
        }

        // Cache size should be under max
        assert!(arc.size <= config.max_size);
        assert!(arc.stats.evictions > 0);

        // Ghost lists should have some entries
        assert!(!arc.b1.is_empty() || !arc.b2.is_empty());
    }

    #[test]
    fn test_arc_ghost_list_adapt() {
        let config = ArcConfig {
            min_size: 512,
            max_size: 2048,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);
        let initial_p = arc.p;

        // Insert and evict to populate ghost list
        arc.insert(1, vec![0u8; 300]);
        arc.insert(2, vec![0u8; 300]);
        arc.insert(3, vec![0u8; 300]);
        arc.insert(4, vec![0u8; 300]);
        arc.insert(5, vec![0u8; 300]);
        arc.insert(6, vec![0u8; 300]);
        arc.insert(7, vec![0u8; 300]);

        // Re-insert evicted entry (should trigger adaptation)
        if !arc.b1.is_empty() {
            let ghost_key = arc.b1[0];
            arc.insert(ghost_key, vec![0u8; 300]);
            // P should have changed
            assert_ne!(arc.p, initial_p);
        }
    }

    #[test]
    fn test_arc_memory_pressure() {
        let config = ArcConfig {
            min_size: 512,
            max_size: 2048,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);

        // Fill cache
        for i in 0..5 {
            arc.insert(i, vec![0u8; 300]);
        }

        let size_before = arc.size;
        assert!(size_before > 0);

        // Apply memory pressure
        arc.memory_pressure(800);

        // Size should have decreased
        assert!(arc.size < size_before);
    }

    #[test]
    fn oversized_entry_does_not_evict_resident_blocks() {
        let config = ArcConfig {
            min_size: 0,
            max_size: 8,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);
        arc.insert(1, vec![1; 4]);
        arc.insert(2, vec![2; 9]);

        assert_eq!(arc.size, 4);
        assert!(arc.lookup(1).is_some());
        assert!(arc.lookup(2).is_none());
    }

    #[test]
    fn replacing_key_updates_accounting_and_promotes_entry() {
        let config = ArcConfig {
            min_size: 0,
            max_size: 8,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);
        arc.insert(1, vec![1; 4]);
        arc.insert(1, vec![2; 8]);

        assert_eq!(arc.size, 8);
        assert!(arc.t1.is_empty());
        assert_eq!(arc.t2.len(), 1);
        assert_eq!(arc.lookup(1), Some(&[2; 8][..]));
        assert_eq!(arc.size, 8);
    }

    #[test]
    fn zero_capacity_cache_rejects_nonempty_entries_without_looping() {
        let config = ArcConfig {
            min_size: 0,
            max_size: 0,
            compression_enabled: false,
        };
        let mut arc = ZfsArc::new(config);
        arc.insert(1, vec![1]);

        assert_eq!(arc.size, 0);
        assert!(arc.t1.is_empty());
        assert!(arc.t2.is_empty());
    }
}
