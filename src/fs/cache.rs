// SPDX-License-Identifier: MIT
//! Block Cache Manager Implementation for SigmaOS Filesystems.

use std::collections::BTreeMap;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheError {
    Success,
    CacheFull,
    BlockNotFound,
    DirtyWritebackFailed,
    AlreadyEvicted,
}

#[derive(Debug, Clone)]
pub struct CacheBlock {
    pub block_id: u64,
    pub data: Vec<u8>,
    pub is_dirty: bool,
    pub last_accessed: u64,
}

pub struct BlockCacheManager {
    pub max_capacity_blocks: usize,
    pub cache_map: BTreeMap<u64, CacheBlock>,
    pub access_counter: u64,
    pub eviction_count: usize,
    pub dirty_writebacks: usize,
}

impl BlockCacheManager {
    pub fn new(max_capacity_blocks: usize) -> Self {
        Self {
            max_capacity_blocks,
            cache_map: BTreeMap::new(),
            access_counter: 0,
            eviction_count: 0,
            dirty_writebacks: 0,
        }
    }

    pub fn read_block(&mut self, block_id: u64) -> Option<Vec<u8>> {
        self.access_counter += 1;
        let counter = self.access_counter;
        if let Some(block) = self.cache_map.get_mut(&block_id) {
            block.last_accessed = counter;
            Some(block.data.clone())
        } else {
            None
        }
    }

    pub fn write_block(&mut self, block_id: u64, data: Vec<u8>) -> Result<(), CacheError> {
        self.access_counter += 1;
        let counter = self.access_counter;

        if self.cache_map.contains_key(&block_id) {
            let block = self.cache_map.get_mut(&block_id).unwrap();
            block.data = data;
            block.is_dirty = true;
            block.last_accessed = counter;
            Ok(())
        } else {
            if self.cache_map.len() >= self.max_capacity_blocks {
                self.evict_lru()?;
            }
            self.cache_map.insert(
                block_id,
                CacheBlock {
                    block_id,
                    data,
                    is_dirty: true,
                    last_accessed: counter,
                },
            );
            Ok(())
        }
    }

    pub fn flush_dirty_blocks(&mut self) -> usize {
        let mut flushed = 0;
        for block in self.cache_map.values_mut() {
            if block.is_dirty {
                block.is_dirty = false;
                flushed += 1;
            }
        }
        self.dirty_writebacks += flushed;
        flushed
    }

    pub fn evict_lru(&mut self) -> Result<u64, CacheError> {
        if self.cache_map.is_empty() {
            return Err(CacheError::AlreadyEvicted);
        }

        let lru_id = *self
            .cache_map
            .values()
            .min_by_key(|b| b.last_accessed)
            .map(|b| &b.block_id)
            .unwrap();

        let block = self.cache_map.get(&lru_id).unwrap();
        if block.is_dirty {
            self.dirty_writebacks += 1;
        }

        self.cache_map.remove(&lru_id);
        self.eviction_count += 1;
        Ok(lru_id)
    }
}
