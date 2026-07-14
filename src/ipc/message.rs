#![no_std]
#![no_main]

/// OOP-based IPC Message System for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 131
/// Implements message passing and shared memory IPC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ChannelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IPCError { Success = 0, ChannelFull = 1, ChannelEmpty = 2, InvalidChannel = 3 }

pub trait MessageChannel {
    fn id(&self) -> ChannelID;
    fn capacity(&self) -> usize;
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError>;
    fn receive(&mut self) -> Result<Vec<u8>, IPCError>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMessageChannel {
    pub id: ChannelID,
    pub capacity: AtomicUsize,
    pub messages: Vec<[u8; 256]>,
}

impl SimpleMessageChannel {
    pub fn new(id: ChannelID, capacity: usize) -> Self {
        SimpleMessageChannel {
            id,
            capacity: AtomicUsize::new(capacity),
            messages: Vec::new(),
        }
    }
}

impl MessageChannel for SimpleMessageChannel {
    fn id(&self) -> ChannelID { self.id }
    fn capacity(&self) -> usize { self.capacity.load(Ordering::SeqCst) }
    
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        if self.messages.len() >= self.capacity() {
            return Err(IPCError::ChannelFull);
        }
        
        let mut msg_array = [0u8; 256];
        let msg_len = message.len().min(255);
        for i in 0..msg_len {
            msg_array[i] = message[i];
        }
        
        self.messages.push(msg_array);
        Ok(())
    }
    
    fn receive(&mut self) -> Result<Vec<u8>, IPCError> {
        if self.messages.is_empty() {
            return Err(IPCError::ChannelEmpty);
        }
        
        let msg_array = self.messages.remove(0);
        let len = msg_array.iter().position(|&b| b == 0).unwrap_or(256);
        let mut result = Vec::new();
        for i in 0..len {
            result.push(msg_array[i]);
        }
        Ok(result)
    }
    
    fn is_empty(&self) -> bool { self.messages.is_empty() }
    
    fn is_full(&self) -> bool { self.messages.len() >= self.capacity() }
}

pub trait IPCManager {
    fn create_channel(&mut self, capacity: usize) -> Result<ChannelID, IPCError>;
    fn destroy_channel(&mut self, id: ChannelID) -> Result<(), IPCError>;
    fn get_channel(&mut self, id: ChannelID) -> Option<&mut dyn MessageChannel>;
}

#[repr(C)]
pub struct SimpleIPCManager {
    pub channels: Vec<Option<Box<dyn MessageChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIPCManager {
    pub fn new() -> Self {
        SimpleIPCManager {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IPCManager for SimpleIPCManager {
    fn create_channel(&mut self, capacity: usize) -> Result<ChannelID, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let channel = SimpleMessageChannel::new(id, capacity);
        self.channels.push(Some(Box::new(channel)));
        Ok(id)
    }
    
    fn destroy_channel(&mut self, id: ChannelID) -> Result<(), IPCError> {
        for channel_option in &mut self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == id {
                    return Ok(());
                }
            }
        }
        Err(IPCError::InvalidChannel)
    }
    
    fn get_channel(&mut self, id: ChannelID) -> Option<&mut dyn MessageChannel> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == id { return Some(channel.as_mut()); }
            }
        }
        None
    }
}

pub trait SharedMemory {
    fn allocate(&mut self, size: usize) -> Result<usize, IPCError>;
    fn deallocate(&mut self, id: usize) -> Result<(), IPCError>;
    fn write(&mut self, id: usize, offset: usize, data: &[u8]) -> Result<(), IPCError>;
    fn read(&self, id: usize, offset: usize, buffer: &mut [u8]) -> Result<(), IPCError>;
}

#[repr(C)]
pub struct SimpleSharedMemory {
    pub regions: Vec<(usize, Vec<u8>)>,
    pub next_id: AtomicUsize,
}

impl SimpleSharedMemory {
    pub fn new() -> Self {
        SimpleSharedMemory {
            regions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SharedMemory for SimpleSharedMemory {
    fn allocate(&mut self, size: usize) -> Result<usize, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(0u8);
        }
        self.regions.push((id, data));
        Ok(id)
    }
    
    fn deallocate(&mut self, id: usize) -> Result<(), IPCError> {
        for i in 0..self.regions.len() {
            if self.regions[i].0 == id {
                self.regions.remove(i);
                return Ok(());
            }
        }
        Err(IPCError::InvalidChannel)
    }
    
    fn write(&mut self, id: usize, offset: usize, data: &[u8]) -> Result<(), IPCError> {
        for region in &mut self.regions {
            if region.0 == id {
                let region_data = &mut region.1;
                let end = (offset + data.len()).min(region_data.len());
                for i in 0..data.len() {
                    if offset + i < end {
                        region_data[offset + i] = data[i];
                    }
                }
                return Ok(());
            }
        }
        Err(IPCError::InvalidChannel)
    }
    
    fn read(&self, id: usize, offset: usize, buffer: &mut [u8]) -> Result<(), IPCError> {
        for region in &self.regions {
            if region.0 == id {
                let region_data = &region.1;
                let end = (offset + buffer.len()).min(region_data.len());
                for i in 0..buffer.len() {
                    if offset + i < end {
                        buffer[i] = region_data[offset + i];
                    }
                }
                return Ok(());
            }
        }
        Err(IPCError::InvalidChannel)
    }
}

pub trait Semaphore {
    fn acquire(&mut self) -> Result<(), IPCError>;
    fn release(&mut self) -> Result<(), IPCError>;
    fn count(&self) -> usize;
}

#[repr(C)]
pub struct SimpleSemaphore {
    pub count: AtomicUsize,
    pub max_count: AtomicUsize,
}

impl SimpleSemaphore {
    pub fn new(initial_count: usize, max_count: usize) -> Self {
        SimpleSemaphore {
            count: AtomicUsize::new(initial_count),
            max_count: AtomicUsize::new(max_count),
        }
    }
}

impl Semaphore for SimpleSemaphore {
    fn acquire(&mut self) -> Result<(), IPCError> {
        let current = self.count.load(Ordering::SeqCst);
        if current > 0 {
            self.count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(IPCError::ChannelEmpty)
        }
    }
    
    fn release(&mut self) -> Result<(), IPCError> {
        let max = self.max_count.load(Ordering::SeqCst);
        let current = self.count.load(Ordering::SeqCst);
        if current < max {
            self.count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(IPCError::ChannelFull)
        }
    }
    
    fn count(&self) -> usize { self.count.load(Ordering::SeqCst) }
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
