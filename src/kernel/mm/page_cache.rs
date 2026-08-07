use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
/// SigmaOS Page Cache — absorbs Linux mm/filemap.c and mm/page-writeback.c
/// Caches file data in memory pages, tracks dirty pages, writeback pressure
use crate::klib::HashMap;
extern crate alloc;
use alloc::vec::Vec;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageStatus {
    Clean,
    Dirty,
    Writeback,
    Evicted,
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
        self.pin_count == 0 && self.status != PageStatus::Writeback
    }
}

/// Page cache — global in-memory file cache
pub struct PageCache {
    pages: HashMap<(u64, u64), CachedPage>, // (inode_id, page_idx) → page
    capacity: usize,
    hits: AtomicUsize,
    misses: AtomicUsize,
    dirty_count: AtomicUsize,
    evictions: AtomicUsize,
    writeback_ops: AtomicUsize,
}

impl PageCache {
    pub fn new(capacity_pages: usize) -> Self {
        PageCache {
            pages: HashMap::new(),
            capacity: capacity_pages,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            dirty_count: AtomicUsize::new(0),
            evictions: AtomicUsize::new(0),
            writeback_ops: AtomicUsize::new(0),
        }
    }

    /// Look up a page in the cache
    pub fn lookup(&mut self, inode_id: u64, page_idx: u64) -> Option<&mut CachedPage> {
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
}
