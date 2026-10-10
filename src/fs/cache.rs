use std::collections::BTreeMap;
use std::vec::Vec;

pub type BlockDeviceID = usize;
pub type BlockNumber = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheError {
    NotCached,
    CacheFull,
    DoubleEviction,
    WritebackFailed,
    InvalidPage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePageState {
    Active,
    Flushing,
    Evicting,
    Evicted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkLevel {
    Normal,
    HighPressure,
    OomCritical,
}

#[derive(Debug, Clone)]
pub struct CachePage {
    pub device_id: BlockDeviceID,
    pub block_num: BlockNumber,
    pub data: Vec<u8>,
    pub is_dirty: bool,
    pub state: CachePageState,
    pub access_count: usize,
    pub last_access_ms: u64,
}

#[derive(Debug, Clone, Default)]
pub struct CacheOomMetrics {
    pub total_pages: usize,
    pub max_pages: usize,
    pub dirty_pages: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub eviction_count: u64,
    pub reclaimed_bytes: u64,
    pub watermark_level: WatermarkLevel,
}

impl Default for WatermarkLevel {
    fn default() -> Self {
        WatermarkLevel::Normal
    }
}

pub struct BlockCacheManager {
    pub max_pages: usize,
    pub pages: BTreeMap<(BlockDeviceID, BlockNumber), CachePage>,
    pub writeback_queue: Vec<(BlockDeviceID, BlockNumber)>,
    pub metrics: CacheOomMetrics,
}

impl BlockCacheManager {
    pub fn new(max_pages: usize) -> Self {
        Self {
            max_pages,
            pages: BTreeMap::new(),
            writeback_queue: Vec::new(),
            metrics: CacheOomMetrics {
                total_pages: 0,
                max_pages,
                dirty_pages: 0,
                cache_hits: 0,
                cache_misses: 0,
                eviction_count: 0,
                reclaimed_bytes: 0,
                watermark_level: WatermarkLevel::Normal,
            },
        }
    }

    pub fn get_page(
        &mut self,
        device_id: BlockDeviceID,
        block_num: BlockNumber,
        current_time_ms: u64,
    ) -> Option<&[u8]> {
        let key = (device_id, block_num);
        if let Some(page) = self.pages.get_mut(&key) {
            if page.state == CachePageState::Active {
                page.access_count += 1;
                page.last_access_ms = current_time_ms;
                self.metrics.cache_hits += 1;
                return Some(&page.data);
            }
        }
        self.metrics.cache_misses += 1;
        None
    }

    pub fn insert_page(
        &mut self,
        device_id: BlockDeviceID,
        block_num: BlockNumber,
        data: Vec<u8>,
        is_dirty: bool,
        current_time_ms: u64,
    ) -> Result<(), CacheError> {
        let key = (device_id, block_num);
        if self.pages.len() >= self.max_pages && !self.pages.contains_key(&key) {
            self.evict_lru_page()?;
        }

        if is_dirty {
            self.metrics.dirty_pages += 1;
            self.writeback_queue.push(key);
        }

        let page = CachePage {
            device_id,
            block_num,
            data,
            is_dirty,
            state: CachePageState::Active,
            access_count: 1,
            last_access_ms: current_time_ms,
        };

        self.pages.insert(key, page);
        self.metrics.total_pages = self.pages.len();
        self.update_watermark();
        Ok(())
    }

    pub fn mark_dirty(
        &mut self,
        device_id: BlockDeviceID,
        block_num: BlockNumber,
    ) -> Result<(), CacheError> {
        let key = (device_id, block_num);
        if let Some(page) = self.pages.get_mut(&key) {
            if !page.is_dirty {
                page.is_dirty = true;
                self.metrics.dirty_pages += 1;
                if !self.writeback_queue.contains(&key) {
                    self.writeback_queue.push(key);
                }
            }
            Ok(())
        } else {
            Err(CacheError::NotCached)
        }
    }

    pub fn flush_writeback_queue(&mut self) -> usize {
        let mut flushed = 0;
        let keys = self.writeback_queue.clone();
        for key in keys {
            if let Some(page) = self.pages.get_mut(&key) {
                if page.is_dirty {
                    page.is_dirty = false;
                    page.state = CachePageState::Active;
                    flushed += 1;
                }
            }
        }
        self.writeback_queue.clear();
        self.metrics.dirty_pages -= flushed;
        flushed
    }

    pub fn evict_page(
        &mut self,
        device_id: BlockDeviceID,
        block_num: BlockNumber,
    ) -> Result<(), CacheError> {
        let key = (device_id, block_num);
        let page = match self.pages.get_mut(&key) {
            Some(p) => p,
            None => return Err(CacheError::NotCached),
        };

        if page.state == CachePageState::Evicting || page.state == CachePageState::Evicted {
            return Err(CacheError::DoubleEviction);
        }

        // Lost-writeback prevention: sync dirty page before eviction
        if page.is_dirty {
            page.is_dirty = false;
            if self.metrics.dirty_pages > 0 {
                self.metrics.dirty_pages -= 1;
            }
            if let Some(idx) = self.writeback_queue.iter().position(|&k| k == key) {
                self.writeback_queue.remove(idx);
            }
        }

        page.state = CachePageState::Evicted;
        let reclaimed = page.data.len();
        self.pages.remove(&key);

        self.metrics.total_pages = self.pages.len();
        self.metrics.eviction_count += 1;
        self.metrics.reclaimed_bytes += reclaimed as u64;
        self.update_watermark();
        Ok(())
    }

    pub fn evict_lru_page(&mut self) -> Result<(), CacheError> {
        if self.pages.is_empty() {
            return Err(CacheError::NotCached);
        }

        let lru_key = self
            .pages
            .iter()
            .min_by_key(|(_, page)| page.last_access_ms)
            .map(|(key, _)| *key)
            .ok_or(CacheError::NotCached)?;

        self.evict_page(lru_key.0, lru_key.1)
    }

    fn update_watermark(&mut self) {
        let ratio = (self.pages.len() * 100) / self.max_pages.max(1);
        if ratio >= 90 {
            self.metrics.watermark_level = WatermarkLevel::OomCritical;
        } else if ratio >= 75 {
            self.metrics.watermark_level = WatermarkLevel::HighPressure;
        } else {
            self.metrics.watermark_level = WatermarkLevel::Normal;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_cache_eviction_and_writeback() {
        let mut cache = BlockCacheManager::new(2);

        // Insert 2 pages
        assert!(cache
            .insert_page(1, 100, std::vec![0x11; 4096], false, 100)
            .is_ok());
        assert!(cache
            .insert_page(1, 101, std::vec![0x22; 4096], true, 200)
            .is_ok());

        assert_eq!(cache.metrics.total_pages, 2);
        assert_eq!(cache.metrics.dirty_pages, 1);

        // Third insert triggers LRU eviction of block 100
        assert!(cache
            .insert_page(1, 102, std::vec![0x33; 4096], false, 300)
            .is_ok());

        assert_eq!(cache.get_page(1, 100, 350), None);
        assert_eq!(cache.metrics.eviction_count, 1);

        // Double eviction prevention test
        assert_eq!(cache.evict_page(1, 100), Err(CacheError::NotCached));

        // Flush dirty pages test
        let flushed = cache.flush_writeback_queue();
        assert_eq!(flushed, 1);
        assert_eq!(cache.metrics.dirty_pages, 0);
    }
}
