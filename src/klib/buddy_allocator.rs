#![no_std]
#![no_main]

/// OOP-based Buddy Allocator for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Week 3-4
/// Implements 2^n page frames with free list per order, split/coalesce

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BlockID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AllocError { Success = 0, OutOfMemory = 1, InvalidBlock = 2, Fragmentation = 3 }

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
    pub fn new(max_order: usize, total_frames: usize) -> Self {
        let mut free_lists: [Vec<BlockID>; 12] = [
            Vec::new(), Vec::new(), Vec::new(), Vec::new(),
            Vec::new(), Vec::new(), Vec::new(), Vec::new(),
            Vec::new(), Vec::new(), Vec::new(), Vec::new(),
        ];
        let mut blocks = Vec::new();
        let mut next_id = AtomicUsize::new(1);
        
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
    fn allocate(&mut self, order: usize) -> Result<BlockID, AllocError> {
        if order > self.max_order.load(Ordering::SeqCst) {
            return Err(AllocError::OutOfMemory);
        }
        
        for current_order in order..=self.max_order.load(Ordering::SeqCst) {
            if !self.free_lists[current_order].is_empty() {
                let block_id = self.free_lists[current_order].remove(0);
                
                while current_order > order {
                    let new_order = current_order - 1;
                    let left_id = self.next_id.fetch_add(1, Ordering::SeqCst);
                    let right_id = self.next_id.fetch_add(1, Ordering::SeqCst);
                    
                    let mut left_block = Block::new(new_order);
                    let mut right_block = Block::new(new_order);
                    
                    if let Some(ref mut parent) = self.blocks[block_id] {
                        parent.left.store(left_id, Ordering::SeqCst);
                        parent.right.store(right_id, Ordering::SeqCst);
                        parent.free.store(0, Ordering::SeqCst);
                    }
                    
                    while left_id >= self.blocks.len() { self.blocks.push(None); }
                    while right_id >= self.blocks.len() { self.blocks.push(None); }
                    
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
                buddy.free.load(Ordering::SeqCst) == 1 && buddy.order.load(Ordering::SeqCst) == current_order
            } else {
                false
            };
            
            if !buddy_free || current_order >= self.max_order.load(Ordering::SeqCst) {
                self.free_lists[current_order].push(current_id);
                return Ok(());
            }
            
            let parent_id = if current_id < buddy_id { current_id } else { buddy_id };
            
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
        if order >= 12 { return 0; }
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
        if total == 0 { return 0.0; }
        let used = self.get_used_frames();
        let free = total - used;
        if free == 0 { return 0.0; }
        
        let mut free_blocks = 0;
        for i in 0..12 {
            free_blocks += self.free_lists[i].len();
        }
        
        if free_blocks == 0 { return 0.0; }
        (free_blocks as f64) / (free as f64)
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    fn retain<F>(&mut self, mut f: F) where F: FnMut(&T) -> bool {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(self.data.add(i), self.data.add(write_idx), 1);
                    }
                    write_idx += 1;
                }
            }
        }
        self.len = write_idx;
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
