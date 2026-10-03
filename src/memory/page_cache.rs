//! # Page Cache Subsystem
//!
//! Linux-inspired page cache for caching filesystem data in memory.
//! Implements LRU eviction and dirty page writeback.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Page size (4KB standard)
pub const PAGE_SIZE: usize = 4096;

/// Maximum pages in cache (configurable limit)
pub const MAX_CACHE_PAGES: usize = 262144; // 1GB with 4KB pages

/// Page flags (inspired by Linux page flags)
#[derive(Debug, Clone, Copy)]
pub struct PageFlags {
    /// Page is dirty (needs writeback)
    pub dirty: bool,
    /// Page is locked (I/O in progress)
    pub locked: bool,
    /// Page is uptodate (valid data)
    pub uptodate: bool,
    /// Page is referenced (for LRU)
    pub referenced: bool,
    /// Page is under writeback
    pub writeback: bool,
}

impl PageFlags {
    pub const CLEAN: Self = Self {
        dirty: false,
        locked: false,
        uptodate: true,
        referenced: false,
        writeback: false,
    };

    pub const DIRTY: Self = Self {
        dirty: true,
        locked: false,
        uptodate: true,
        referenced: true,
        writeback: false,
    };
}

/// Cached page entry
pub struct CachedPage {
    /// Page data
    pub data: Vec<u8>,
    /// Page flags
    pub flags: PageFlags,
    /// Reference count
    pub refcount: AtomicUsize,
    /// Last access time
    pub last_access: AtomicU64,
    /// Inode number (file identifier)
    pub inode: u64,
    /// Page offset within file
    pub offset: u64,
}

impl CachedPage {
    pub fn new(inode: u64, offset: u64) -> Self {
        Self {
            data: alloc::vec![0u8; PAGE_SIZE],
            flags: PageFlags::CLEAN,
            refcount: AtomicUsize::new(0),
            last_access: AtomicU64::new(0),
            inode,
            offset,
        }
    }

    pub fn with_data(inode: u64, offset: u64, data: Vec<u8>) -> Self {
        Self {
            data,
            flags: PageFlags::CLEAN,
            refcount: AtomicUsize::new(0),
            last_access: AtomicU64::new(0),
            inode,
            offset,
        }
    }

    /// Increment reference count
    pub fn get(&self) {
        self.refcount.fetch_add(1, Ordering::Acquire);
    }

    /// Decrement reference count
    pub fn put(&self) -> usize {
        self.refcount.fetch_sub(1, Ordering::Release)
    }

    /// Get reference count
    pub fn refcount(&self) -> usize {
        self.refcount.load(Ordering::Acquire)
    }

    /// Mark page as dirty
    pub fn mark_dirty(&mut self) {
        self.flags.dirty = true;
        self.flags.uptodate = true;
    }

    /// Mark page as clean
    pub fn mark_clean(&mut self) {
        self.flags.dirty = false;
    }

    /// Update last access time
    pub fn touch(&mut self, time: u64) {
        self.last_access.store(time, Ordering::Release);
        self.flags.referenced = true;
    }
}

/// Page cache key (inode + offset)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PageKey {
    pub inode: u64,
    pub offset: u64,
}

impl PageKey {
    pub const fn new(inode: u64, offset: u64) -> Self {
        Self { inode, offset }
    }
}

/// LRU list entry
struct LruEntry {
    key: PageKey,
    last_access: u64,
}

/// Page cache (inspired by Linux address_space)
pub struct PageCache {
    /// Cached pages (indexed by PageKey)
    pages: BTreeMap<PageKey, CachedPage>,
    /// LRU list for eviction
    lru: Vec<LruEntry>,
    /// Maximum cache size (pages)
    max_pages: usize,
    /// Current cache size (pages)
    current_pages: AtomicUsize,
    /// Total cache hits
    hits: AtomicU64,
    /// Total cache misses
    misses: AtomicU64,
    /// Total evictions
    evictions: AtomicU64,
    /// Total dirty pages
    dirty_pages: AtomicUsize,
    /// Current time counter
    current_time: AtomicU64,
}

impl PageCache {
    pub fn new(max_pages: usize) -> Self {
        Self {
            pages: BTreeMap::new(),
            lru: Vec::new(),
            max_pages,
            current_pages: AtomicUsize::new(0),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            evictions: AtomicU64::new(0),
            dirty_pages: AtomicUsize::new(0),
            current_time: AtomicU64::new(0),
        }
    }

    /// Get current time
    fn now(&self) -> u64 {
        self.current_time.fetch_add(1, Ordering::Relaxed)
    }

    /// Look up page in cache
    pub fn lookup(&mut self, inode: u64, offset: u64) -> Option<&mut CachedPage> {
        let key = PageKey::new(inode, offset);
        let now = self.now();
        if let Some(page) = self.pages.get_mut(&key) {
            page.touch(now);
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(page)
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Add page to cache
    pub fn insert(&mut self, mut page: CachedPage) -> Result<(), PageCacheError> {
        let key = PageKey::new(page.inode, page.offset);

        // Check if we need to evict
        if self.current_pages.load(Ordering::Relaxed) >= self.max_pages {
            self.evict_one()?;
        }

        let now = self.now();
        // Insert page
        page.touch(now);
        if page.flags.dirty {
            self.dirty_pages.fetch_add(1, Ordering::Relaxed);
        }

        self.pages.insert(key, page);
        self.current_pages.fetch_add(1, Ordering::Relaxed);
        self.lru.push(LruEntry {
            key,
            last_access: now,
        });

        Ok(())
    }

    /// Evict least recently used page
    fn evict_one(&mut self) -> Result<(), PageCacheError> {
        // Find LRU page (oldest non-referenced, non-locked)
        let mut evict_idx = None;
        for (i, entry) in self.lru.iter().enumerate() {
            if let Some(page) = self.pages.get(&entry.key) {
                if page.refcount() == 0 && !page.flags.locked {
                    evict_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = evict_idx {
            let entry = self.lru.remove(idx);
            if let Some(mut page) = self.pages.remove(&entry.key) {
                // Writeback dirty page before eviction
                if page.flags.dirty {
                    Self::writeback_page(&mut page)?;
                    self.dirty_pages.fetch_sub(1, Ordering::Relaxed);
                }
                self.current_pages.fetch_sub(1, Ordering::Relaxed);
                self.evictions.fetch_add(1, Ordering::Relaxed);
                return Ok(());
            }
        }

        Err(PageCacheError::NoEvictablePages)
    }

    /// Write dirty page to storage
    fn writeback_page(page: &mut CachedPage) -> Result<(), PageCacheError> {
        if !page.flags.dirty {
            return Ok(());
        }

        // In real implementation, would write to disk
        // For now, just mark as clean
        page.mark_clean();
        Ok(())
    }

    /// Flush all dirty pages
    pub fn flush(&mut self) -> Result<usize, PageCacheError> {
        let mut flushed = 0;
        let keys: Vec<PageKey> = self.pages.keys().copied().collect();

        for key in keys {
            if let Some(page) = self.pages.get_mut(&key) {
                if page.flags.dirty {
                    Self::writeback_page(page)?;
                    flushed += 1;
                }
            }
        }

        self.dirty_pages.store(0, Ordering::Relaxed);
        Ok(flushed)
    }

    /// Invalidate page (remove from cache)
    pub fn invalidate(&mut self, inode: u64, offset: u64) -> bool {
        let key = PageKey::new(inode, offset);
        if let Some(page) = self.pages.remove(&key) {
            if page.flags.dirty {
                self.dirty_pages.fetch_sub(1, Ordering::Relaxed);
            }
            self.current_pages.fetch_sub(1, Ordering::Relaxed);
            // Remove from LRU
            self.lru.retain(|e| e.key != key);
            true
        } else {
            false
        }
    }

    /// Invalidate all pages for inode
    pub fn invalidate_inode(&mut self, inode: u64) -> usize {
        let keys: Vec<PageKey> = self
            .pages
            .keys()
            .filter(|k| k.inode == inode)
            .copied()
            .collect();

        let mut invalidated = 0;
        for key in keys {
            if self.invalidate(key.inode, key.offset) {
                invalidated += 1;
            }
        }

        invalidated
    }

    /// Get cache statistics
    pub fn stats(&self) -> PageCacheStats {
        let total_requests = self.hits.load(Ordering::Relaxed) + self.misses.load(Ordering::Relaxed);
        let hit_rate = if total_requests > 0 {
            (self.hits.load(Ordering::Relaxed) as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        PageCacheStats {
            current_pages: self.current_pages.load(Ordering::Relaxed),
            max_pages: self.max_pages,
            dirty_pages: self.dirty_pages.load(Ordering::Relaxed),
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            evictions: self.evictions.load(Ordering::Relaxed),
            hit_rate,
        }
    }
}

/// Page cache statistics
#[derive(Debug, Clone, Copy)]
pub struct PageCacheStats {
    pub current_pages: usize,
    pub max_pages: usize,
    pub dirty_pages: usize,
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub hit_rate: f64,
}

/// Page cache errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageCacheError {
    /// No evictable pages available
    NoEvictablePages,
    /// Page is locked
    PageLocked,
    /// I/O error during writeback
    IoError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = CachedPage::new(1, 0);
        assert_eq!(page.inode, 1);
        assert_eq!(page.offset, 0);
        assert_eq!(page.data.len(), PAGE_SIZE);
        assert!(!page.flags.dirty);
    }

    #[test]
    fn test_page_reference_count() {
        let page = CachedPage::new(1, 0);
        assert_eq!(page.refcount(), 0);
        page.get();
        assert_eq!(page.refcount(), 1);
        page.put();
        assert_eq!(page.refcount(), 0);
    }

    #[test]
    fn test_page_cache_lookup() {
        let mut cache = PageCache::new(100);
        let page = CachedPage::new(1, 0);
        cache.insert(page).unwrap();

        assert!(cache.lookup(1, 0).is_some());
        assert!(cache.lookup(1, 4096).is_none());
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 1);
    }

    #[test]
    fn test_page_cache_eviction() {
        let mut cache = PageCache::new(2);
        cache.insert(CachedPage::new(1, 0)).unwrap();
        cache.insert(CachedPage::new(1, 4096)).unwrap();
        cache.insert(CachedPage::new(1, 8192)).unwrap();

        assert_eq!(cache.stats().current_pages, 2);
        assert_eq!(cache.stats().evictions, 1);
    }

    #[test]
    fn test_page_dirty_tracking() {
        let mut cache = PageCache::new(100);
        let mut page = CachedPage::new(1, 0);
        page.mark_dirty();
        cache.insert(page).unwrap();

        assert_eq!(cache.stats().dirty_pages, 1);
        cache.flush().unwrap();
        assert_eq!(cache.stats().dirty_pages, 0);
    }

    #[test]
    fn test_page_invalidate() {
        let mut cache = PageCache::new(100);
        cache.insert(CachedPage::new(1, 0)).unwrap();
        cache.insert(CachedPage::new(1, 4096)).unwrap();

        let invalidated = cache.invalidate_inode(1);
        assert_eq!(invalidated, 2);
        assert_eq!(cache.stats().current_pages, 0);
    }
}
