//! SigmaOS Block Buffer Cache Subsystem

use std::collections::BTreeMap;

/// State of a block buffer in the buffer cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferState {
    Clean,
    Dirty,
    Locked,
    InFlight,
}

/// Header metadata for a cached block buffer.
#[derive(Debug, Clone)]
pub struct BufferHead {
    pub dev_id: u32,
    pub block_num: u64,
    pub size: usize,
    pub state: BufferState,
    pub access_count: u64,
    pub data: Vec<u8>,
}

/// Typed errors for buffer cache management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferError {
    PoolExhausted,
    BufferLocked(u32, u64),
    NotFound(u32, u64),
    InvalidBlockSize,
    IoFailure,
}

/// BufferCache governs block allocation, LRU eviction, and dirty flushing across devices.
pub struct BufferCache {
    max_buffers: usize,
    block_size: usize,
    buffers: BTreeMap<(u32, u64), BufferHead>,
    lru_order: Vec<(u32, u64)>,
    access_counter: u64,
}

impl BufferCache {
    pub fn new(max_buffers: usize, block_size: usize) -> Self {
        Self {
            max_buffers,
            block_size,
            buffers: BTreeMap::new(),
            lru_order: Vec::new(),
            access_counter: 0,
        }
    }

    /// Read a block buffer from cache, or allocate a new buffer if missing.
    pub fn get_block(&mut self, dev_id: u32, block_num: u64) -> Result<&mut BufferHead, BufferError> {
        self.access_counter += 1;
        let counter = self.access_counter;
        let key = (dev_id, block_num);

        if !self.buffers.contains_key(&key) {
            if self.buffers.len() >= self.max_buffers {
                self.evict_lru()?;
            }

            let new_buf = BufferHead {
                dev_id,
                block_num,
                size: self.block_size,
                state: BufferState::Clean,
                access_count: counter,
                data: vec![0u8; self.block_size],
            };

            self.buffers.insert(key, new_buf);
            self.lru_order.push(key);
        }

        let buf = self.buffers.get_mut(&key).unwrap();
        buf.access_count = counter;

        // Touch LRU
        if let Some(pos) = self.lru_order.iter().position(|&x| x == key) {
            let val = self.lru_order.remove(pos);
            self.lru_order.push(val);
        }

        Ok(self.buffers.get_mut(&key).unwrap())
    }

    /// Mark a buffer as dirty.
    pub fn mark_dirty(&mut self, dev_id: u32, block_num: u64) -> Result<(), BufferError> {
        let key = (dev_id, block_num);
        if let Some(buf) = self.buffers.get_mut(&key) {
            if buf.state == BufferState::Locked {
                return Err(BufferError::BufferLocked(dev_id, block_num));
            }
            buf.state = BufferState::Dirty;
            Ok(())
        } else {
            Err(BufferError::NotFound(dev_id, block_num))
        }
    }

    /// Flush all dirty buffers to storage.
    pub fn flush_dirty(&mut self) -> Result<usize, BufferError> {
        let mut flushed = 0;
        for buf in self.buffers.values_mut() {
            if buf.state == BufferState::Dirty {
                buf.state = BufferState::Clean;
                flushed += 1;
            }
        }
        Ok(flushed)
    }

    /// Evict the least recently used clean buffer.
    fn evict_lru(&mut self) -> Result<(), BufferError> {
        let mut evict_target = None;

        for &key in &self.lru_order {
            if let Some(buf) = self.buffers.get(&key) {
                if buf.state == BufferState::Clean {
                    evict_target = Some(key);
                    break;
                }
            }
        }

        if let Some(key) = evict_target {
            self.buffers.remove(&key);
            self.lru_order.retain(|&x| x != key);
            Ok(())
        } else {
            Err(BufferError::PoolExhausted)
        }
    }

    pub fn cached_count(&self) -> usize {
        self.buffers.len()
    }
}
