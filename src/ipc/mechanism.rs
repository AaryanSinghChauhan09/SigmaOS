#![no_std]
#![no_main]

/// OOP-based IPC Mechanism for SigmaOS
/// Based on Roadmap Item 9: IPC mechanism

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ChannelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ChannelType { MessageQueue = 0, SharedMemory = 1, Pipe = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ChannelState { Closed = 0, Open = 1 }

pub trait Channel {
    fn id(&self) -> ChannelID;
    fn channel_type(&self) -> ChannelType;
    fn state(&self) -> ChannelState;
    fn send(&mut self, data: &[u8]) -> Result<(), IPCError>;
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IPCError { Success = 0, SendFailed = 1, ReceiveFailed = 2 }

#[repr(C)]
pub struct SimpleChannel {
    pub id: ChannelID,
    pub channel_type: ChannelType,
    pub state: AtomicUsize,
    pub buffer: [u8; 4096],
    pub buffer_size: AtomicUsize,
}

impl SimpleChannel {
    pub fn new(id: ChannelID, channel_type: ChannelType) -> Self {
        SimpleChannel {
            id,
            channel_type,
            state: AtomicUsize::new(ChannelState::Open as usize),
            buffer: [0; 4096],
            buffer_size: AtomicUsize::new(0),
        }
    }
}

impl Channel for SimpleChannel {
    fn id(&self) -> ChannelID { self.id }
    fn channel_type(&self) -> ChannelType { self.channel_type }
    fn state(&self) -> ChannelState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn send(&mut self, data: &[u8]) -> Result<(), IPCError> {
        let bytes = data.len().min(4096);
        unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), self.buffer.as_mut_ptr(), bytes); }
        self.buffer_size.store(bytes, Ordering::SeqCst);
        Ok(())
    }
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        let size = self.buffer_size.load(Ordering::SeqCst);
        let bytes = buffer.len().min(size);
        unsafe { core::ptr::copy_nonoverlapping(self.buffer.as_ptr(), buffer.as_mut_ptr(), bytes); }
        Ok(bytes)
    }
}

pub trait IPCMechanism {
    fn create_channel(&mut self, channel_type: ChannelType) -> Result<ChannelID, IPCError>;
    fn close_channel(&mut self, id: ChannelID) -> Result<(), IPCError>;
    fn get_channel(&self, id: ChannelID) -> Option<&dyn Channel>;
}

pub struct SimpleIPCMechanism {
    channels: Vec<Option<Box<dyn Channel>>>,
    next_id: AtomicUsize,
}

impl SimpleIPCMechanism {
    pub fn new() -> Self { SimpleIPCMechanism { channels: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl IPCMechanism for SimpleIPCMechanism {
    fn create_channel(&mut self, channel_type: ChannelType) -> Result<ChannelID, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let channel = SimpleChannel::new(id, channel_type);
        self.channels.push(Some(Box::new(channel)));
        Ok(id)
    }
    fn close_channel(&mut self, id: ChannelID) -> Result<(), IPCError> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == id {
                    channel.state.store(ChannelState::Closed as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(IPCError::ReceiveFailed)
    }
    fn get_channel(&self, id: ChannelID) -> Option<&dyn Channel> {
        for channel_option in &self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == id { return Some(channel.as_ref()); }
            }
        }
        None
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
