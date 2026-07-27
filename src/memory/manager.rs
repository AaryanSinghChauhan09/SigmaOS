#![no_std]
#![no_main]

/// OOP-based Memory Manager for SigmaOS
/// Implements memory management using OOP principles with traits and structs
/// No dependency on external memory management frameworks
/// Based on Roadmap Item 4: Memory manager

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Block ID
pub type BlockID = usize;

/// Block state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BlockState {
    Free = 0,
    Allocated = 1,
    Reserved = 2,
}

/// Memory block trait (OOP interface)
pub trait MemoryBlock {
    /// Get block ID
    fn id(&self) -> BlockID;
    /// Get block size
    fn size(&self) -> usize;
    /// Get block state
    fn state(&self) -> BlockState;
    /// Get block address
    fn address(&self) -> usize;
    /// Set block state
    fn set_state(&mut self, state: BlockState);
    /// Get block info
    fn info(&self) -> BlockInfo;
}

/// Block info
#[repr(C)]
pub struct BlockInfo {
    pub id: BlockID,
    pub size: usize,
    pub state: BlockState,
    pub address: usize,
    pub capability: BlockCapability,
}

impl BlockInfo {
    pub fn new(id: BlockID) -> Self {
        BlockInfo {
            id,
            size: 0,
            state: BlockState::Free,
            address: 0,
            capability: BlockCapability::new(),
        }
    }
}

/// Block capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockCapability {
    pub can_allocate: bool,
    pub can_free: bool,
}

impl BlockCapability {
    pub fn new() -> Self {
        BlockCapability {
            can_allocate: false,
            can_free: false,
        }
    }

    pub fn full() -> Self {
        BlockCapability {
            can_allocate: true,
            can_free: true,
        }
    }
}

/// Simple memory block (OOP: Concrete block class)
#[repr(C)]
pub struct SimpleMemoryBlock {
    pub id: BlockID,
    pub size: usize,
    pub state: AtomicUsize, // BlockState as usize
    pub address: usize,
    pub capability: BlockCapability,
}

impl SimpleMemoryBlock {
    pub fn new(id: BlockID, size: usize, address: usize, capability: BlockCapability) -> Self {
        SimpleMemoryBlock {
            id,
            size,
            state: AtomicUsize::new(BlockState::Free as usize),
            address,
            capability,
        }
    }

    pub fn get_state(&self) -> BlockState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state_atomic(&self, state: BlockState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl MemoryBlock for SimpleMemoryBlock {
    fn id(&self) -> BlockID {
        self.id
    }

    fn size(&self) -> usize {
        self.size
    }

    fn state(&self) -> BlockState {
        self.get_state()
    }

    fn address(&self) -> usize {
        self.address
    }

    fn set_state(&mut self, state: BlockState) {
        self.set_state_atomic(state);
    }

    fn info(&self) -> BlockInfo {
        BlockInfo {
            id: self.id,
            size: self.size,
            state: self.get_state(),
            address: self.address,
            capability: self.capability,
        }
    }
}

/// Memory manager trait (OOP interface)
pub trait MemoryManager {
    /// Allocate memory
    fn allocate(&mut self, size: usize) -> Result<BlockID, MemoryError>;
    /// Free memory
    fn free(&mut self, id: BlockID) -> Result<(), MemoryError>;
    /// Get block
    fn get_block(&self, id: BlockID) -> Option<&dyn MemoryBlock>;
    /// List blocks by state
    fn list_blocks(&self, state: BlockState) -> Vec<BlockID>;
    /// Get manager statistics
    fn stats(&self) -> MemoryStats;
}

/// Memory error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MemoryError {
    Success = 0,
    OutOfMemory = 1,
    InvalidBlock = 2,
    PermissionDenied = 3,
}

/// Memory statistics
#[repr(C)]
pub struct MemoryStats {
    pub total_blocks: usize,
    pub free_blocks: usize,
    pub allocated_blocks: usize,
    pub total_memory: usize,
    pub used_memory: usize,
}

impl MemoryStats {
    pub fn new() -> Self {
        MemoryStats {
            total_blocks: 0,
            free_blocks: 0,
            allocated_blocks: 0,
            total_memory: 0,
            used_memory: 0,
        }
    }
}

/// Simple memory manager (OOP: Concrete manager class)
pub struct SimpleMemoryManager {
    blocks: Vec<Option<Box<dyn MemoryBlock>>>,
    next_id: AtomicUsize,
    stats: MemoryStats,
    capability: ManagerCapability,
    heap_start: usize,
    heap_size: usize,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_allocate: bool,
    pub can_free: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_allocate: false,
            can_free: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_allocate: true,
            can_free: true,
        }
    }
}

impl SimpleMemoryManager {
    pub fn new(heap_start: usize, heap_size: usize, capability: ManagerCapability) -> Self {
        SimpleMemoryManager {
            blocks: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: MemoryStats::new(),
            capability,
            heap_start,
            heap_size,
        }
    }

    fn find_free_block(&self, size: usize) -> Option<BlockID> {
        for block_option in &self.blocks {
            if let Some(ref block) = *block_option {
                if block.state() == BlockState::Free && block.size() >= size {
                    return Some(block.id());
                }
            }
        }
        None
    }
}

impl MemoryManager for SimpleMemoryManager {
    fn allocate(&mut self, size: usize) -> Result<BlockID, MemoryError> {
        if !self.capability.can_allocate {
            return Err(MemoryError::PermissionDenied);
        }

        // Try to find a free block
        if let Some(id) = self.find_free_block(size) {
            for block_option in &mut self.blocks {
                if let Some(ref mut block) = *block_option {
                    if block.id() == id {
                        block.set_state(BlockState::Allocated);
                        self.stats.free_blocks -= 1;
                        self.stats.allocated_blocks += 1;
                        self.stats.used_memory += size;
                        return Ok(id);
                    }
                }
            }
        }

        // Create new block
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let address = self.heap_start + (self.stats.used_memory % self.heap_size);
        let block = SimpleMemoryBlock::new(id, size, address, BlockCapability::full());
        block.set_state_atomic(BlockState::Allocated);
        
        self.blocks.push(Some(Box::new(block)));
        self.stats.total_blocks += 1;
        self.stats.allocated_blocks += 1;
        self.stats.total_memory += size;
        self.stats.used_memory += size;
        
        Ok(id)
    }

    fn free(&mut self, id: BlockID) -> Result<(), MemoryError> {
        if !self.capability.can_free {
            return Err(MemoryError::PermissionDenied);
        }

        for block_option in &mut self.blocks {
            if let Some(ref mut block) = *block_option {
                if block.id() == id {
                    if block.state() != BlockState::Allocated {
                        return Err(MemoryError::InvalidBlock);
                    }

                    let size = block.size();
                    block.set_state(BlockState::Free);
                    self.stats.allocated_blocks -= 1;
                    self.stats.free_blocks += 1;
                    self.stats.used_memory -= size;
                    return Ok(());
                }
            }
        }
        Err(MemoryError::InvalidBlock)
    }

    fn get_block(&self, id: BlockID) -> Option<&dyn MemoryBlock> {
        for block_option in &self.blocks {
            if let Some(ref block) = *block_option {
                if block.id() == id {
                    return Some(block.as_ref());
                }
            }
        }
        None
    }

    fn list_blocks(&self, state: BlockState) -> Vec<BlockID> {
        let mut ids = Vec::new();

        for block_option in &self.blocks {
            if let Some(ref block) = *block_option {
                if block.state() == state {
                    ids.push(block.id());
                }
            }
        }

        ids
    }

    fn stats(&self) -> MemoryStats {
        self.stats
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
