#![no_std]
#![no_main]

/// OOP-based Heap Allocator for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 31
/// Implements dynamic memory allocation and deallocation

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BlockID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HeapError { Success = 0, OutOfMemory = 1, InvalidPointer = 2, CorruptedHeap = 3 }

pub trait HeapBlock {
    fn id(&self) -> BlockID;
    fn size(&self) -> usize;
    fn is_free(&self) -> bool;
    fn set_free(&mut self, free: bool);
}

#[repr(C)]
pub struct SimpleHeapBlock {
    pub id: BlockID,
    pub size: AtomicUsize,
    pub free: AtomicUsize,
}

impl SimpleHeapBlock {
    pub fn new(id: BlockID, size: usize) -> Self {
        SimpleHeapBlock {
            id,
            size: AtomicUsize::new(size),
            free: AtomicUsize::new(1),
        }
    }
}

impl HeapBlock for SimpleHeapBlock {
    fn id(&self) -> BlockID { self.id }
    fn size(&self) -> usize { self.size.load(Ordering::SeqCst) }
    fn is_free(&self) -> bool { self.free.load(Ordering::SeqCst) == 1 }
    
    fn set_free(&mut self, free: bool) {
        self.free.store(if free { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait HeapAllocator {
    fn allocate(&mut self, size: usize) -> Result<*mut u8, HeapError>;
    fn deallocate(&mut self, ptr: *mut u8) -> Result<(), HeapError>;
    fn reallocate(&mut self, ptr: *mut u8, new_size: usize) -> Result<*mut u8, HeapError>;
    fn get_stats(&self) -> (usize, usize, usize);
}

#[repr(C)]
pub struct SimpleHeapAllocator {
    pub blocks: Vec<Option<Box<dyn HeapBlock>>>,
    pub heap_start: AtomicUsize,
    pub heap_size: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleHeapAllocator {
    pub fn new(heap_start: usize, heap_size: usize) -> Self {
        SimpleHeapAllocator {
            blocks: Vec::new(),
            heap_start: AtomicUsize::new(heap_start),
            heap_size: AtomicUsize::new(heap_size),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HeapAllocator for SimpleHeapAllocator {
    fn allocate(&mut self, size: usize) -> Result<*mut u8, HeapError> {
        for block_option in &mut self.blocks {
            if let Some(ref mut block) = *block_option {
                if block.is_free() && block.size() >= size {
                    block.set_free(false);
                    let offset = block.id() * 4096;
                    let heap_start = self.heap_start.load(Ordering::SeqCst);
                    return Ok((heap_start + offset) as *mut u8);
                }
            }
        }
        
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let block = SimpleHeapBlock::new(id, size);
        self.blocks.push(Some(Box::new(block)));
        
        let offset = id * 4096;
        let heap_start = self.heap_start.load(Ordering::SeqCst);
        Ok((heap_start + offset) as *mut u8)
    }
    
    fn deallocate(&mut self, ptr: *mut u8) -> Result<(), HeapError> {
        let heap_start = self.heap_start.load(Ordering::SeqCst);
        let offset = (ptr as usize) - heap_start;
        let block_id = offset / 4096;
        
        for block_option in &mut self.blocks {
            if let Some(ref mut block) = *block_option {
                if block.id() == block_id {
                    block.set_free(true);
                    return Ok(());
                }
            }
        }
        
        Err(HeapError::InvalidPointer)
    }
    
    fn reallocate(&mut self, ptr: *mut u8, new_size: usize) -> Result<*mut u8, HeapError> {
        self.deallocate(ptr)?;
        self.allocate(new_size)
    }
    
    fn get_stats(&self) -> (usize, usize, usize) {
        let mut total = 0;
        let mut used = 0;
        let mut free = 0;
        
        for block_option in &self.blocks {
            if let Some(ref block) = *block_option {
                total += block.size();
                if block.is_free() {
                    free += block.size();
                } else {
                    used += block.size();
                }
            }
        }
        
        (total, used, free)
    }
}

pub trait HeapDefragmenter {
    fn defragment(&mut self) -> Result<(), HeapError>;
    fn coalesce(&mut self) -> Result<(), HeapError>;
}

#[repr(C)]
pub struct SimpleHeapDefragmenter {
    pub allocator: SimpleHeapAllocator,
}

impl SimpleHeapDefragmenter {
    pub fn new(allocator: SimpleHeapAllocator) -> Self {
        SimpleHeapDefragmenter { allocator }
    }
}

impl HeapDefragmenter for SimpleHeapDefragmenter {
    fn defragment(&mut self) -> Result<(), HeapError> {
        let mut compacted = Vec::new();
        
        for block_option in &mut self.allocator.blocks {
            if let Some(ref block) = *block_option {
                if !block.is_free() {
                    compacted.push(block.size());
                }
            }
        }
        
        self.allocator.blocks = Vec::new();
        for size in compacted {
            let id = self.allocator.next_id.fetch_add(1, Ordering::SeqCst);
            let mut block = SimpleHeapBlock::new(id, size);
            block.set_free(false);
            self.allocator.blocks.push(Some(Box::new(block)));
        }
        
        Ok(())
    }
    
    fn coalesce(&mut self) -> Result<(), HeapError> {
        let mut i = 0;
        while i < self.allocator.blocks.len() - 1 {
            let current_free = if let Some(ref block) = self.allocator.blocks[i] {
                block.is_free()
            } else {
                false
            };
            
            let next_free = if let Some(ref block) = self.allocator.blocks[i + 1] {
                block.is_free()
            } else {
                false
            };
            
            if current_free && next_free {
                if let Some(ref mut block) = *self.allocator.blocks[i] {
                    let current_size = block.size();
                    if let Some(ref next_block) = *self.allocator.blocks[i + 1] {
                        let new_size = current_size + next_block.size();
                    }
                }
                self.allocator.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
        
        Ok(())
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
