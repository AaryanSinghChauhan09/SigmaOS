#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
/// SigmaOS Page Cache — absorbs Linux mm/filemap.c and mm/page-writeback.c
/// Caches file data in memory pages, tracks dirty pages, writeback pressure

#[cfg(not(test))]
use crate::klib::BTreeMap;

#[cfg(test)]
use std::collections::BTreeMap;

use std::vec::Vec;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageStatus {
    Clean,
    Dirty,
    Writeback,
    Evicted,
}

/// Debian-inspired: Cache page priority tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PagePriority {
    Low = 0,
    Standard = 1,
    High = 2,
    Required = 3, // Sticky - protected from standard evictions
}

/// Clear Linux-inspired: Aggressive sequential read-ahead engine for high-throughput buffering
#[derive(Debug, Clone)]
pub struct ClearLinuxReadAheadEngine {
    last_accessed_page: Option<u64>,
    sequential_streak: usize,
    pub prefetch_window: usize,
}

impl ClearLinuxReadAheadEngine {
    pub fn new() -> Self {
        Self {
            last_accessed_page: None,
            sequential_streak: 0,
            prefetch_window: 2,
        }
    }

    /// Updates sequential access tracking and returns desired prefetch count
    pub fn on_access(&mut self, page_idx: u64) -> usize {
        if let Some(last) = self.last_accessed_page {
            if page_idx == last + 1 {
                self.sequential_streak += 1;
                // Scale prefetch aggressively for long sequential streaks
                if self.sequential_streak > 5 {
                    self.prefetch_window = 8;
                } else if self.sequential_streak > 2 {
                    self.prefetch_window = 4;
                }
            } else {
                self.sequential_streak = 0;
                self.prefetch_window = 2;
            }
        }
        self.last_accessed_page = Some(page_idx);
        self.prefetch_window
    }
}

/// NixOS-inspired: Immutable hash-addressed page content deduplicator
#[derive(Debug, Clone)]
pub struct NixOSPageDeduplicator {
    // Content hash -> list of (inode_id, page_idx)
    hash_index: BTreeMap<u64, Vec<(u64, u64)>>,
}

impl NixOSPageDeduplicator {
    pub fn new() -> Self {
        Self {
            hash_index: BTreeMap::new(),
        }
    }

    /// Computes a fast FNV-1a hash of the page data
    pub fn hash_page(data: &[u8; PAGE_SIZE]) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for &b in data {
            hash ^= b as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }

    /// Registers a page in the index, returning true if a duplicate exists
    pub fn register_page(&mut self, inode_id: u64, page_idx: u64, data: &[u8; PAGE_SIZE]) -> bool {
        let h = Self::hash_page(data);
        let entries = self.hash_index.entry(h).or_insert_with(Vec::new);
        let exists = !entries.is_empty();
        if !entries.contains(&(inode_id, page_idx)) {
            entries.push((inode_id, page_idx));
        }
        exists
    }

    pub fn unregister_page(&mut self, inode_id: u64, page_idx: u64) {
        for entries in self.hash_index.values_mut() {
            entries.retain(|&k| k != (inode_id, page_idx));
        }
    }

    pub fn deduplicated_pages_count(&self) -> usize {
        let mut total = 0;
        for entries in self.hash_index.values() {
            if entries.len() > 1 {
                total += entries.len() - 1;
            }
        }
        total
    }
}

/// SteamOS-inspired: Dynamic background write-buffering throttle to safeguard interactive latency
#[derive(Debug, Clone)]
pub struct SteamOSWritebackThrottle {
    pub high_watermark: usize,
    pub low_watermark: usize,
}

impl SteamOSWritebackThrottle {
    pub fn new(high_watermark: usize, low_watermark: usize) -> Self {
        Self {
            high_watermark,
            low_watermark,
        }
    }

    /// Returns true if writeback is actively requested due to dirty page accumulation
    pub fn should_throttle(&self, dirty_pages_count: usize) -> bool {
        dirty_pages_count >= self.high_watermark
    }
}

/// A single page in the cache
#[derive(Debug)]
pub struct CachedPage {
    pub inode_id: u64,
    pub page_idx: u64, // File offset in pages
    pub status: PageStatus,
    pub data: [u8; PAGE_SIZE],
    pub access_count: u32,
    pub pin_count: u32, // PTE references — cannot evict if > 0
    pub priority: PagePriority, // Debian-inspired sticky priority pinning
}

impl CachedPage {
    pub fn new(inode_id: u64, page_idx: u64) -> Self {
        CachedPage {
            inode_id,
            page_idx,
            status: PageStatus::Clean,
            data: [0u8; PAGE_SIZE],
            access_count: 0,
            pin_count: 0,
            priority: PagePriority::Standard,
        }
    }

    pub fn mark_dirty(&mut self) {
        if self.status == PageStatus::Clean {
            self.status = PageStatus::Dirty;
        }
    }
    pub fn mark_clean(&mut self) {
        self.status = PageStatus::Clean;
    }
    pub fn pin(&mut self) {
        self.pin_count += 1;
    }
    pub fn unpin(&mut self) {
        if self.pin_count > 0 {
            self.pin_count -= 1;
        }
    }
    pub fn can_evict(&self) -> bool {
        self.pin_count == 0 && self.status != PageStatus::Writeback && self.priority != PagePriority::Required
    }
}

/// Page cache — global in-memory file cache
pub struct PageCache {
    pages: BTreeMap<(u64, u64), CachedPage>, // (inode_id, page_idx) → page
    capacity: usize,
    hits: AtomicUsize,
    misses: AtomicUsize,
    dirty_count: AtomicUsize,
    evictions: AtomicUsize,
    writeback_ops: AtomicUsize,
    pub read_ahead: ClearLinuxReadAheadEngine,
    pub deduplicator: NixOSPageDeduplicator,
    pub throttle: SteamOSWritebackThrottle,
}

impl PageCache {
    pub fn new(capacity_pages: usize) -> Self {
        PageCache {
            pages: BTreeMap::new(),
            capacity: capacity_pages,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            dirty_count: AtomicUsize::new(0),
            evictions: AtomicUsize::new(0),
            writeback_ops: AtomicUsize::new(0),
            read_ahead: ClearLinuxReadAheadEngine::new(),
            deduplicator: NixOSPageDeduplicator::new(),
            throttle: SteamOSWritebackThrottle::new(8, 2), // High watermark 8 dirty pages, low 2
        }
    }

    /// Look up a page in the cache
    pub fn lookup(&mut self, inode_id: u64, page_idx: u64) -> Option<&mut CachedPage> {
        // Trigger Clear Linux prefetching heuristic on lookup
        self.read_ahead.on_access(page_idx);

        if self.pages.contains_key(&(inode_id, page_idx)) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            let p = self.pages.get_mut(&(inode_id, page_idx)).unwrap();
            p.access_count += 1;
            Some(p)
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Insert a new page, evicting LRU if over capacity
    pub fn insert(&mut self, inode_id: u64, page_idx: u64) -> &mut CachedPage {
        if self.pages.len() >= self.capacity {
            self.evict_one();
        }
        let key = (inode_id, page_idx);
        self.pages.insert(key, CachedPage::new(inode_id, page_idx));
        self.pages.get_mut(&key).unwrap()
    }

    /// Write data into a cached page, marking it dirty
    pub fn write_page(&mut self, inode_id: u64, page_idx: u64, offset: usize, data: &[u8]) {
        let page = if self.pages.contains_key(&(inode_id, page_idx)) {
            self.pages.get_mut(&(inode_id, page_idx)).unwrap()
        } else {
            if self.pages.len() >= self.capacity {
                self.evict_one();
            }
            self.pages
                .insert((inode_id, page_idx), CachedPage::new(inode_id, page_idx));
            self.pages.get_mut(&(inode_id, page_idx)).unwrap()
        };
        let end = (offset + data.len()).min(PAGE_SIZE);
        let len = end - offset;
        page.data[offset..end].copy_from_slice(&data[..len]);
        let was_clean = page.status == PageStatus::Clean;
        page.mark_dirty();
        if was_clean {
            self.dirty_count.fetch_add(1, Ordering::Relaxed);
        }

        // NixOS-inspired hash-addressed deduplication registration
        self.deduplicator.register_page(inode_id, page_idx, &page.data);

        // SteamOS-inspired dynamic writeback throttle check
        if self.throttle.should_throttle(self.dirty_pages()) {
            self.writeback_all();
        }
    }

    /// Read from cached page
    pub fn read_page(
        &mut self,
        inode_id: u64,
        page_idx: u64,
        offset: usize,
        buf: &mut [u8],
    ) -> bool {
        if let Some(page) = self.lookup(inode_id, page_idx) {
            let end = (offset + buf.len()).min(PAGE_SIZE);
            let len = end - offset;
            buf[..len].copy_from_slice(&page.data[offset..end]);
            true
        } else {
            false
        }
    }

    /// Writeback all dirty pages (simulate flush to storage)
    pub fn writeback_all(&mut self) -> usize {
        let mut count = 0;
        for page in self.pages.values_mut() {
            if page.status == PageStatus::Dirty {
                page.status = PageStatus::Writeback;
                page.mark_clean();
                count += 1;
                self.writeback_ops.fetch_add(1, Ordering::Relaxed);
                self.dirty_count.fetch_sub(1, Ordering::Relaxed);
            }
        }
        count
    }

    /// Evict the least-recently-used clean page
    fn evict_one(&mut self) {
        let victim = self
            .pages
            .iter()
            .filter(|(_, p)| p.can_evict() && p.status == PageStatus::Clean)
            .min_by_key(|(_, p)| p.access_count)
            .map(|(k, _)| *k);

        if let Some(key) = victim {
            self.pages.remove(&key);
            self.evictions.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
    pub fn dirty_pages(&self) -> usize {
        self.dirty_count.load(Ordering::Relaxed)
    }
    pub fn hits(&self) -> usize {
        self.hits.load(Ordering::Relaxed)
    }
    pub fn misses(&self) -> usize {
        self.misses.load(Ordering::Relaxed)
    }
    pub fn hit_rate(&self) -> f64 {
        let h = self.hits() as f64;
        let total = (self.hits() + self.misses()) as f64;
        if total == 0.0 {
            0.0
        } else {
            h / total
        }
    }
    pub fn evictions(&self) -> usize {
        self.evictions.load(Ordering::Relaxed)
    }
    pub fn writeback_ops(&self) -> usize {
        self.writeback_ops.load(Ordering::Relaxed)
    }
}

impl Default for PageCache {
    fn default() -> Self {
        Self::new(1024)
    } // 4MB default (1024 × 4K pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_cache_write_read() {
        let mut cache = PageCache::new(16);
        cache.write_page(1, 0, 0, b"SigmaOS page cache!");
        let mut buf = [0u8; 19];
        let hit = cache.read_page(1, 0, 0, &mut buf);
        assert!(hit);
        assert_eq!(&buf, b"SigmaOS page cache!");
        assert_eq!(cache.dirty_pages(), 1);
    }

    #[test]
    fn test_page_cache_miss() {
        let mut cache = PageCache::new(16);
        let mut buf = [0u8; 10];
        let hit = cache.read_page(99, 0, 0, &mut buf);
        assert!(!hit);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_writeback() {
        let mut cache = PageCache::new(16);
        cache.write_page(1, 0, 0, b"dirty data");
        cache.write_page(1, 1, 0, b"more dirty");
        assert_eq!(cache.dirty_pages(), 2);
        let written = cache.writeback_all();
        assert_eq!(written, 2);
        assert_eq!(cache.dirty_pages(), 0);
    }

    #[test]
    fn test_capacity_eviction() {
        let mut cache = PageCache::new(4);
        for i in 0..4u64 {
            cache.write_page(1, i, 0, b"data");
            // Mark each clean via writeback so it can be evicted
            cache.writeback_all();
        }
        // Now insert a 5th page — should evict one
        cache.write_page(1, 99, 0, b"new page");
        assert!(cache.page_count() <= 4);
        assert!(cache.evictions() > 0);
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = PageCache::new(16);
        cache.insert(2, 0);
        cache.lookup(2, 0); // hit
        cache.lookup(2, 1); // miss
        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 1);
        assert!((cache.hit_rate() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_clear_linux_read_ahead() {
        let mut engine = ClearLinuxReadAheadEngine::new();
        // Default prefetch size on first access is 2
        assert_eq!(engine.prefetch_window, 2);

        // Access sequentially: page 0, then 1, 2, 3, 4, 5...
        engine.on_access(0);
        assert_eq!(engine.on_access(1), 2);
        assert_eq!(engine.on_access(2), 2);

        // High sequential streak should expand the window (Clear Linux prefetch heuristics)
        engine.on_access(3); // streak = 3
        assert_eq!(engine.prefetch_window, 4);

        engine.on_access(4);
        engine.on_access(5);
        engine.on_access(6); // streak > 5
        assert_eq!(engine.prefetch_window, 8);

        // Mismatched access breaks sequential streak
        assert_eq!(engine.on_access(100), 2);
    }

    #[test]
    fn test_debian_priority_pinning() {
        let mut cache = PageCache::new(2);

        // Insert two pages, marking page 1 as PagePriority::Required (mission-critical)
        cache.write_page(1, 0, 0, b"normal data");
        cache.writeback_all();

        cache.write_page(1, 1, 0, b"mission critical");
        if let Some(page) = cache.lookup(1, 1) {
            page.priority = PagePriority::Required;
        }
        cache.writeback_all();

        // Since normal data has lower priority, inserting new data should evict page 0 (normal data) rather than page 1 (sticky critical)
        cache.write_page(1, 2, 0, b"newer data");
        cache.writeback_all();

        // Page 1 must still exist in cache due to Required priority sticky pinning
        assert!(cache.lookup(1, 1).is_some());
    }

    #[test]
    fn test_nixos_page_deduplication() {
        let mut dedup = NixOSPageDeduplicator::new();
        let page_data1 = [0xAAu8; PAGE_SIZE];
        let page_data2 = [0xAAu8; PAGE_SIZE]; // Identical page data
        let page_data3 = [0x55u8; PAGE_SIZE]; // Distinct page data

        // Register first page
        let dup1 = dedup.register_page(1, 100, &page_data1);
        assert!(!dup1);

        // Register second page with identical content - NixOS deduplication should spot it!
        let dup2 = dedup.register_page(1, 101, &page_data2);
        assert!(dup2);
        assert_eq!(dedup.deduplicated_pages_count(), 1);

        // Register distinct third page
        let dup3 = dedup.register_page(1, 102, &page_data3);
        assert!(!dup3);
        assert_eq!(dedup.deduplicated_pages_count(), 1);
    }

    #[test]
    fn test_steamos_writeback_throttle() {
        let throttle = SteamOSWritebackThrottle::new(4, 1);
        assert!(!throttle.should_throttle(2));
        assert!(throttle.should_throttle(4));
        assert!(throttle.should_throttle(5));
    }
}
