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
}

#[repr(C)]
pub struct Block {
    pub order: AtomicUsize,
    pub free: AtomicUsize,
    pub left: AtomicUsize,
    pub right: AtomicUsize,
}

impl Block {
    pub fn new(order: usize) -> Self {
        Block {
            order: AtomicUsize::new(order),
            free: AtomicUsize::new(1),
            left: AtomicUsize::new(0),
            right: AtomicUsize::new(0),
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

        let initial_order = max_order.min(11);
        let initial_block_id = next_id.fetch_add(1, Ordering::SeqCst);
        let initial_block = Block::new(initial_order);
        blocks.push(Some(initial_block));
        free_lists[initial_order].push(initial_block_id);

        SimpleBuddyAllocator {
            max_order: AtomicUsize::new(initial_order),
            free_lists,
            blocks,
            next_id,
        }
    }
}

impl BuddyAllocator for SimpleBuddyAllocator {
    fn allocate(&mut self, order: usize) -> Result<BlockID, AllocError> {
        if order > self.max_order.load(Ordering::SeqCst) {
            return Err(AllocError::OutOfMemory);
        }

        for current_order in order..=self.max_order.load(Ordering::SeqCst) {
            if !self.free_lists[current_order].is_empty() {
                let block_id = self.free_lists[current_order].remove(0);

                if current_order > order {
                    let new_order = current_order - 1;
                    let left_id = self.next_id.fetch_add(1, Ordering::SeqCst);
                    let right_id = self.next_id.fetch_add(1, Ordering::SeqCst);

                    let left_block = Block::new(new_order);
                    let right_block = Block::new(new_order);

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

use crate::klib::vec::Vec;

#[cfg(test_disabled)]
mod tests {
    use super::*;

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
    fn test_fragmentation() {
        let allocator = SimpleBuddyAllocator::new(5, 32);
        let ratio = allocator.get_fragmentation_ratio();
        assert!((0.0..=1.0).contains(&ratio));
    }
}
