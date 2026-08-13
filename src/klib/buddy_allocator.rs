
/// OOP-based Buddy Allocator for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 3-4
/// Implements 2^n page frames with free list per order, split/coalesce
use core::sync::atomic::{AtomicUsize, Ordering};

pub type BlockID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AllocError {
    Success = 0,
    OutOfMemory = 1,
    InvalidBlock = 2,
    Fragmentation = 3,
}

pub trait BuddyAllocator {
    fn allocate(&mut self, order: usize) -> Result<BlockID, AllocError>;
    fn free(&mut self, block_id: BlockID, order: usize) -> Result<(), AllocError>;
    fn get_free_count(&self, order: usize) -> usize;
    /// Linux-inspired lazy reclamation: free a page cache item or unused clean page if OOM
    fn reclaim_pages(&mut self, target_order: usize) -> Result<(), AllocError>;
}

#[repr(C)]
pub struct Block {
    pub order: AtomicUsize,
    pub free: AtomicUsize,
    pub left: AtomicUsize,
    pub right: AtomicUsize,
    pub is_cache: AtomicUsize, // 1 if occupied by reclaimable page cache/buffers, 0 otherwise
}

impl Block {
    pub fn new(order: usize) -> Self {
        Block {
            order: AtomicUsize::new(order),
            free: AtomicUsize::new(1),
            left: AtomicUsize::new(0),
            right: AtomicUsize::new(0),
            is_cache: AtomicUsize::new(0),
        }
    }
}

pub struct SimpleBuddyAllocator {
    pub max_order: AtomicUsize,
    pub free_lists: [Vec<BlockID>; 12],
    pub blocks: Vec<Option<Block>>,
    pub next_id: AtomicUsize,
}

impl SimpleBuddyAllocator {
    pub fn new(max_order: usize, _total_frames: usize) -> Self {
        let mut free_lists: [Vec<BlockID>; 12] = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let mut blocks = Vec::new();
        let next_id = AtomicUsize::new(0);
        
        let initial_order = max_order;
        let initial_block_id = next_id.fetch_add(1, Ordering::SeqCst);
        let initial_block = Block::new(initial_order);
        blocks.push(Some(initial_block));
        free_lists[initial_order].push(initial_block_id);

        SimpleBuddyAllocator {
            max_order: AtomicUsize::new(max_order),
            free_lists,
            blocks,
            next_id,
        }
    }
}

impl BuddyAllocator for SimpleBuddyAllocator {
    fn reclaim_pages(&mut self, target_order: usize) -> Result<(), AllocError> {
        // Search for blocks allocated as is_cache, free them to satisfy target_order allocation
        let mut found_reclaimable = None;
        for (id, block_opt) in self.blocks.iter().enumerate() {
            if let Some(block) = block_opt {
                if block.free.load(Ordering::SeqCst) == 0 && block.is_cache.load(Ordering::SeqCst) == 1 {
                    let order = block.order.load(Ordering::SeqCst);
                    if order >= target_order {
                        found_reclaimable = Some((id, order));
                        break;
                    }
                }
            }
        }

        if let Some((id, order)) = found_reclaimable {
            // Free the cache page back to the allocator
            self.free(id, order)?;
            Ok(())
        } else {
            Err(AllocError::OutOfMemory)
        }
    }

    fn allocate(&mut self, order: usize) -> Result<BlockID, AllocError> {
        if order > self.max_order.load(Ordering::SeqCst) {
            return Err(AllocError::OutOfMemory);
        }

        let mut retry_count = 0;
        loop {
            for current_order in order..=self.max_order.load(Ordering::SeqCst) {
                if !self.free_lists[current_order].is_empty() {
                    let block_id = self.free_lists[current_order].remove(0);
                    
                    if current_order > order {
                        let new_order = current_order - 1;
                        let left_id = self.next_id.fetch_add(1, Ordering::SeqCst);
                        let right_id = self.next_id.fetch_add(1, Ordering::SeqCst);

                        let left_block = Block::new(new_order);
                        let right_block = Block::new(new_order);

                        left_block.free.store(0, Ordering::SeqCst);

                        if let Some(ref mut parent) = self.blocks[block_id] {
                            parent.left.store(left_id, Ordering::SeqCst);
                            parent.right.store(right_id, Ordering::SeqCst);
                            parent.free.store(0, Ordering::SeqCst);
                        }

                        while left_id >= self.blocks.len() {
                            self.blocks.push(None);
                        }
                        while right_id >= self.blocks.len() {
                            self.blocks.push(None);
                        }

                        self.blocks[left_id] = Some(left_block);
                        self.blocks[right_id] = Some(right_block);

                        self.free_lists[new_order].push(right_id);

                        return Ok(left_id);
                    }

                    if let Some(ref mut block) = self.blocks[block_id] {
                        block.free.store(0, Ordering::SeqCst);
                    }

                    return Ok(block_id);
                }
            }

            // If we are out of memory, try to reclaim cache pages (like Linux kswapd/lazy reclaim)
            if retry_count == 0 {
                if self.reclaim_pages(order).is_ok() {
                    retry_count += 1;
                    continue;
                }
            }
            break;
        }

        Err(AllocError::OutOfMemory)
    }

    fn free(&mut self, block_id: BlockID, order: usize) -> Result<(), AllocError> {
        if block_id >= self.blocks.len() {
            return Err(AllocError::InvalidBlock);
        }

        let mut current_id = block_id;
        let mut current_order = order;

        loop {
            if let Some(ref mut block) = self.blocks[current_id] {
                block.free.store(1, Ordering::SeqCst);
                block.order.store(current_order, Ordering::SeqCst);
            }

            let buddy_id = current_id ^ (1 << current_order);

            if buddy_id >= self.blocks.len() {
                self.free_lists[current_order].push(current_id);
                return Ok(());
            }

            let buddy_free = if let Some(ref buddy) = self.blocks[buddy_id] {
                buddy.free.load(Ordering::SeqCst) == 1
                    && buddy.order.load(Ordering::SeqCst) == current_order
            } else {
                false
            };

            if !buddy_free || current_order >= self.max_order.load(Ordering::SeqCst) {
                self.free_lists[current_order].push(current_id);
                return Ok(());
            }

            let parent_id = if current_id < buddy_id {
                current_id
            } else {
                buddy_id
            };

            if let Some(ref mut buddy) = self.blocks[buddy_id] {
                buddy.free.store(0, Ordering::SeqCst);
            }
            if let Some(ref mut block) = self.blocks[current_id] {
                block.free.store(0, Ordering::SeqCst);
            }

            self.free_lists[current_order].retain(|&id| id != buddy_id && id != current_id);

            current_id = parent_id;
            current_order += 1;
        }
    }

    fn get_free_count(&self, order: usize) -> usize {
        if order >= 12 {
            return 0;
        }
        self.free_lists[order].len()
    }
}

pub trait MemoryPool {
    fn get_total_frames(&self) -> usize;
    fn get_used_frames(&self) -> usize;
    fn get_fragmentation_ratio(&self) -> f64;
}

impl MemoryPool for SimpleBuddyAllocator {
    fn get_total_frames(&self) -> usize {
        self.blocks.len()
    }

    fn get_used_frames(&self) -> usize {
        let mut used = 0;
        for block_option in &self.blocks {
            if let Some(ref block) = *block_option {
                if block.free.load(Ordering::SeqCst) == 0 {
                    used += 1;
                }
            }
        }
        used
    }

    fn get_fragmentation_ratio(&self) -> f64 {
        let total = self.get_total_frames();
        if total == 0 {
            return 0.0;
        }
        let used = self.get_used_frames();
        let free = total - used;
        if free == 0 {
            return 0.0;
        }

        let mut free_blocks = 0;
        for i in 0..12 {
            free_blocks += self.free_lists[i].len();
        }

        if free_blocks == 0 {
            return 0.0;
        }
        (free_blocks as f64) / (free as f64)
    }
}

pub use crate::klib::Vec;

/// Allocation record tracked by LeakTracker (inspired by Valgrind and LeakSanitizer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocRecord {
    pub ptr: *mut u8,
    pub size: usize,
    pub tag: [u8; 32], // tag/location metadata
}

/// Valgrind and LeakSanitizer-inspired memory leak detector interface
pub trait MemoryLeakTracker {
    fn on_alloc(&mut self, ptr: *mut u8, size: usize, tag: &[u8]);
    fn on_free(&mut self, ptr: *mut u8);
    fn report_leaks(&self) -> (usize, usize); // Returns (leak_count, leaked_bytes)
}

/// LeakTracker implementation
pub struct LeakTracker {
    pub allocations: Vec<AllocRecord>,
}

impl LeakTracker {
    pub fn new() -> Self {
        LeakTracker {
            allocations: Vec::new(),
        }
    }
}

impl MemoryLeakTracker for LeakTracker {
    fn on_alloc(&mut self, ptr: *mut u8, size: usize, tag: &[u8]) {
        let mut tag_arr = [0u8; 32];
        let tag_len = tag.len().min(31);
        tag_arr[..tag_len].copy_from_slice(&tag[..tag_len]);

        self.allocations.push(AllocRecord {
            ptr,
            size,
            tag: tag_arr,
        });
    }

    fn on_free(&mut self, ptr: *mut u8) {
        // Mark the record as freed by setting ptr to null
        for i in 0..self.allocations.len() {
            if self.allocations[i].ptr == ptr {
                self.allocations[i].ptr = core::ptr::null_mut();
                self.allocations[i].size = 0;
                break;
            }
        }
    }

    fn report_leaks(&self) -> (usize, usize) {
        let mut leak_count = 0;
        let mut leaked_bytes = 0;
        for i in 0..self.allocations.len() {
            let record = &self.allocations[i];
            if !record.ptr.is_null() {
                leak_count += 1;
                leaked_bytes += record.size;
            }
        }
        (leak_count, leaked_bytes)
    }
}

/// Allocation record tracked by LeakTracker (inspired by Valgrind and LeakSanitizer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocRecord {
    pub ptr: *mut u8,
    pub size: usize,
    pub tag: [u8; 32], // tag/location metadata
}

/// Valgrind and LeakSanitizer-inspired memory leak detector interface
pub trait MemoryLeakTracker {
    fn on_alloc(&mut self, ptr: *mut u8, size: usize, tag: &[u8]);
    fn on_free(&mut self, ptr: *mut u8);
    fn report_leaks(&self) -> (usize, usize); // Returns (leak_count, leaked_bytes)
}

/// LeakTracker implementation
pub struct LeakTracker {
    pub allocations: Vec<AllocRecord>,
}

impl LeakTracker {
    pub fn new() -> Self {
        LeakTracker {
            allocations: Vec::new(),
        }
    }
}

impl MemoryLeakTracker for LeakTracker {
    fn on_alloc(&mut self, ptr: *mut u8, size: usize, tag: &[u8]) {
        let mut tag_arr = [0u8; 32];
        let tag_len = tag.len().min(31);
        tag_arr[..tag_len].copy_from_slice(&tag[..tag_len]);

        self.allocations.push(AllocRecord {
            ptr,
            size,
            tag: tag_arr,
        });
    }

    fn on_free(&mut self, ptr: *mut u8) {
        // Mark the record as freed by setting ptr to null
        for i in 0..self.allocations.len() {
            if self.allocations[i].ptr == ptr {
                self.allocations[i].ptr = core::ptr::null_mut();
                self.allocations[i].size = 0;
                break;
            }
        }
    }

    fn report_leaks(&self) -> (usize, usize) {
        let mut leak_count = 0;
        let mut leaked_bytes = 0;
        for i in 0..self.allocations.len() {
            let record = &self.allocations[i];
            if !record.ptr.is_null() {
                leak_count += 1;
                leaked_bytes += record.size;
            }
        }
        (leak_count, leaked_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leak_tracker_diagnostics() {
        let mut tracker = LeakTracker::new();
        let p1 = 0x5000 as *mut u8;
        let p2 = 0x6000 as *mut u8;

        tracker.on_alloc(p1, 256, b"p1_buffer");
        tracker.on_alloc(p2, 512, b"p2_buffer");

        tracker.on_free(p1);

        let (leak_count, leaked_bytes) = tracker.report_leaks();
        assert_eq!(leak_count, 1);
        assert_eq!(leaked_bytes, 512);
    }

    #[test]
    fn test_buddy_allocator() {
        let mut allocator = SimpleBuddyAllocator::new(10, 1024);

        let block_1 = allocator.allocate(3).unwrap();
        assert!(block_1 > 0);

        let block_2 = allocator.allocate(3).unwrap();
        assert!(block_2 > 0);
        assert_ne!(block_1, block_2);

        assert!(allocator.free(block_1, 3).is_ok());
        assert!(allocator.free(block_2, 3).is_ok());
    }

    #[test]
    fn test_lazy_reclaim() {
        let mut allocator = SimpleBuddyAllocator::new(3, 8);

        // Allocate all blocks
        let b1 = allocator.allocate(2).unwrap();
        let _b2 = allocator.allocate(2).unwrap();

        // Mark b1 as being used by page cache
        if let Some(ref mut block) = allocator.blocks[b1] {
            block.is_cache.store(1, Ordering::SeqCst);
        }

        // Next allocation of order 2 should fail due to OOM, but lazy reclaim should free b1 and succeed!
        let b3 = allocator.allocate(2).unwrap();
        assert_eq!(b3, b1);
    }

    #[test]
    fn test_fragmentation() {
        let allocator = SimpleBuddyAllocator::new(5, 32);
        let ratio = allocator.get_fragmentation_ratio();
        assert!((0.0..=1.0).contains(&ratio));
    }
}
