#![no_std]
#![no_main]

/// OOP-based DMA for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1146
/// Implements DMA transfers

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ChannelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DMADirection { MemoryToMemory = 0, MemoryToPeripheral = 1, PeripheralToMemory = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DMAError { Success = 0, NotFound = 1, TransferFailed = 2 }

pub trait DMAChannel {
    fn id(&self) -> ChannelID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDMAChannel {
    pub id: ChannelID,
    pub busy: AtomicUsize,
}

impl SimpleDMAChannel {
    pub fn new(id: ChannelID) -> Self {
        SimpleDMAChannel {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl DMAChannel for SimpleDMAChannel {
    fn id(&self) -> ChannelID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait DMAController {
    fn configure(&mut self, channel_id: ChannelID, direction: DMADirection) -> Result<(), DMAError>;
    fn start_transfer(&mut self, channel_id: ChannelID, src: u32, dst: u32, size: u32) -> Result<(), DMAError>;
    fn is_complete(&self, channel_id: ChannelID) -> bool;
}

#[repr(C)]
pub struct SimpleDMAController {
    pub channels: Vec<Option<Box<dyn DMAChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDMAController {
    pub fn new() -> Self {
        SimpleDMAController {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DMAController for SimpleDMAController {
    fn configure(&mut self, channel_id: ChannelID, _direction: DMADirection) -> Result<(), DMAError> {
        let channel = SimpleDMAChannel::new(channel_id);
        self.channels.push(Some(Box::new(channel)));
        Ok(())
    }
    
    fn start_transfer(&mut self, channel_id: ChannelID, _src: u32, _dst: u32, _size: u32) -> Result<(), DMAError> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == channel_id {
                    channel.busy.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DMAError::NotFound)
    }
    
    fn is_complete(&self, channel_id: ChannelID) -> bool {
        for channel_option in &self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == channel_id {
                    return !channel.is_busy();
                }
            }
        }
        true
    }
}

pub trait CircularBuffer {
    fn write(&mut self, data: &[u8]) -> usize;
    fn read(&mut self, buffer: &mut [u8]) -> usize;
    fn available(&self) -> usize;
}

#[repr(C)]
pub struct SimpleCircularBuffer {
    pub data: [u8; 256],
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
}

impl SimpleCircularBuffer {
    pub fn new() -> Self {
        SimpleCircularBuffer {
            data: [0u8; 256],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }
}

impl CircularBuffer for SimpleCircularBuffer {
    fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            let next_head = (head + 1) % 256;
            if next_head != tail {
                self.data[head] = byte;
                self.head.store(next_head, Ordering::SeqCst);
                written += 1;
            } else {
                break;
            }
        }
        written
    }
    
    fn read(&mut self, buffer: &mut [u8]) -> usize {
        let mut read = 0;
        for byte in buffer.iter_mut() {
            let head = self.head.load(Ordering::SeqCst);
            let tail = self.tail.load(Ordering::SeqCst);
            if head != tail {
                *byte = self.data[tail];
                let next_tail = (tail + 1) % 256;
                self.tail.store(next_tail, Ordering::SeqCst);
                read += 1;
            } else {
                break;
            }
        }
        read
    }
    
    fn available(&self) -> usize {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        if head >= tail {
            head - tail
        } else {
            256 - tail + head
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
