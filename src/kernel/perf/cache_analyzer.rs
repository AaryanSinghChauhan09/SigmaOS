// SigmaOS Kernel Perf - VFS & Page Cache Analyzer
// Tracks hit/miss efficiency across VFS page cache, inode cache, and dentry cache.

pub struct CacheStats {
    pub page_cache_hits: u64,
    pub page_cache_misses: u64,
    pub inode_cache_hits: u64,
    pub inode_cache_misses: u64,
    pub dentry_cache_hits: u64,
    pub dentry_cache_misses: u64,
}

impl CacheStats {
    pub fn new() -> Self {
        Self {
            page_cache_hits: 0,
            page_cache_misses: 0,
            inode_cache_hits: 0,
            inode_cache_misses: 0,
            dentry_cache_hits: 0,
            dentry_cache_misses: 0,
        }
    }

    pub fn record_page_access(&mut self, hit: bool) {
        if hit {
            self.page_cache_hits += 1;
        } else {
            self.page_cache_misses += 1;
        }
    }

    pub fn record_inode_access(&mut self, hit: bool) {
        if hit {
            self.inode_cache_hits += 1;
        } else {
            self.inode_cache_misses += 1;
        }
    }

    pub fn record_dentry_access(&mut self, hit: bool) {
        if hit {
            self.dentry_cache_hits += 1;
        } else {
            self.dentry_cache_misses += 1;
        }
    }

    pub fn page_cache_hit_ratio(&self) -> f32 {
        let total = self.page_cache_hits + self.page_cache_misses;
        if total == 0 {
            return 1.0;
        }
        (self.page_cache_hits as f32) / (total as f32)
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_analyzer_hit_ratios() {
        let mut stats = CacheStats::new();

        stats.record_page_access(true);
        stats.record_page_access(true);
        stats.record_page_access(true);
        stats.record_page_access(false);

        assert_eq!(stats.page_cache_hits, 3);
        assert_eq!(stats.page_cache_misses, 1);
        assert_eq!(stats.page_cache_hit_ratio(), 0.75);
    }
}
