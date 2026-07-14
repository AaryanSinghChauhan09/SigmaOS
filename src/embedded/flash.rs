#![no_std]
#![no_main]

/// OOP-based Flash Memory for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1046
/// Implements flash memory management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BlockID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FlashState { Erased = 0, Written = 1, Locked = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FlashError { Success = 0, NotFound = 1, WriteFailed = 2 }

pub trait FlashBlock {
    fn id(&self) -> BlockID;
    fn address(&self) -> u32;
    fn size(&self) -> u32;
    fn state(&self) -> FlashState;
}

#[repr(C)]
pub struct SimpleFlashBlock {
    pub id: BlockID,
    pub address: AtomicUsize,
    pub size: AtomicUsize,
    pub state: AtomicUsize,
}

impl SimpleFlashBlock {
    pub fn new(id: BlockID, address: u32, size: u32) -> Self {
        SimpleFlashBlock {
            id,
            address: AtomicUsize::new(address as usize),
            size: AtomicUsize::new(size as usize),
            state: AtomicUsize::new(FlashState::Erased as usize),
        }
    }
}

impl FlashBlock for SimpleFlashBlock {
    fn id(&self) -> BlockID { self.id }
    fn address(&self) -> u32 { self.address.load(Ordering::SeqCst) as u32 }
    fn size(&self) -> u32 { self.size.load(Ordering::SeqCst) as u32 }
    fn state(&self) -> FlashState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait FlashManager {
    fn add_block(&mut self, address: u32, size: u32) -> Result<BlockID, FlashError>;
    fn erase(&mut self, id: BlockID) -> Result<(), FlashError>;
    fn write(&mut self, id: BlockID, offset: u32, data: &[u8]) -> Result<(), FlashError>;
    fn read(&self, id: BlockID, offset: u32, buffer: &mut [u8]) -> Result<(), FlashError>;
}

#[repr(C)]
pub struct SimpleFlashManager {
    pub blocks: Vec<Option<Box<dyn FlashBlock>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFlashManager {
    pub fn new() -> Self {
        SimpleFlashManager {
            blocks: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FlashManager for SimpleFlashManager {
    fn add_block(&mut self, address: u32, size: u32) -> Result<BlockID, FlashError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let block = SimpleFlashBlock::new(id, address, size);
        self.blocks.push(Some(Box::new(block)));
        Ok(id)
    }
    
    fn erase(&mut self, id: BlockID) -> Result<(), FlashError> {
        for block_option in &mut self.blocks {
            if let Some(ref mut block) = *block_option {
                if block.id() == id {
                    block.state.store(FlashState::Erased as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(FlashError::NotFound)
    }
    
    fn write(&mut self, id: BlockID, _offset: u32, _data: &[u8]) -> Result<(), FlashError> {
        for block_option in &mut self.blocks {
            if let Some(ref mut block) = *block_option {
                if block.id() == id {
                    block.state.store(FlashState::Written as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(FlashError::NotFound)
    }
    
    fn read(&self, id: BlockID, _offset: u32, buffer: &mut [u8]) -> Result<(), FlashError> {
        if self.get_block(id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0xFF;
            }
            Ok(())
        } else {
            Err(FlashError::NotFound)
        }
    }
    
    fn get_block(&self, id: BlockID) -> Option<&dyn FlashBlock> {
        for block_option in &self.blocks {
            if let Some(ref block) = *block_option {
                if block.id() == id { return Some(block.as_ref()); }
            }
        }
        None
    }
}

pub trait WearLeveling {
    def get_erase_count(&self, block_id: BlockID) -> u32;
    def select_least_worn(&self) -> Option<BlockID>;
}

#[repr(C)]
pub struct SimpleWearLeveling {
    pub erase_counts: Vec<(BlockID, AtomicUsize)>,
}

impl SimpleWearLeveling {
    pub fn new() -> Self {
        SimpleWearLeveling {
            erase_counts: Vec::new(),
        }
    }
}

impl WearLeveling for SimpleWearLeveling {
    fn get_erase_count(&self, block_id: BlockID) -> u32 {
        for &(id, ref count) in &self.erase_counts {
            if id == block_id {
                return count.load(Ordering::SeqCst) as u32;
            }
        }
        0
    }
    
    fn select_least_worn(&self) -> Option<BlockID> {
        if self.erase_counts.is_empty() {
            None
        } else {
            let mut min_id = self.erase_counts[0].0;
            let mut min_count = self.erase_counts[0].1.load(Ordering::SeqCst);
            for &(id, ref count) in &self.erase_counts {
                let current = count.load(Ordering::SeqCst);
                if current < min_count {
                    min_count = current;
                    min_id = id;
                }
            }
            Some(min_id)
        }
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
    fn clear(&mut self) {
        self.len = 0;
    }
    fn is_empty(&self) -> bool { self.len == 0 }
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
