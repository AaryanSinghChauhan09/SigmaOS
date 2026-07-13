#![no_std]
#![no_main]

/// OOP-based IPC System for SigmaOS
/// Implements inter-process communication using OOP principles with traits and structs
/// No dependency on external IPC frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// IPC endpoint trait (OOP interface)
pub trait IPCEndpoint {
    /// Send message
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError>;
    /// Receive message
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError>;
    /// Get endpoint info
    fn info(&self) -> IPCInfo;
    /// Close endpoint
    fn close(&mut self) -> Result<(), IPCError>;
}

/// IPC error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IPCError {
    Success = 0,
    NotConnected = 1,
    BufferFull = 2,
    BufferEmpty = 3,
    InvalidSize = 4,
    PermissionDenied = 5,
    Timeout = 6,
}

/// IPC type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IPCType {
    Pipe = 0,
    MessageQueue = 1,
    SharedMemory = 2,
    Socket = 3,
    Signal = 4,
}

/// IPC info
#[repr(C)]
pub struct IPCInfo {
    pub ipc_type: IPCType,
    pub capacity: usize,
    pub buffer_size: usize,
    pub connected: bool,
}

impl IPCInfo {
    pub fn new(ipc_type: IPCType) -> Self {
        IPCInfo {
            ipc_type,
            capacity: 0,
            buffer_size: 0,
            connected: false,
        }
    }
}

/// IPC capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IPCCapability {
    pub can_send: bool,
    pub can_receive: bool,
    pub can_create: bool,
    pub can_destroy: bool,
}

impl IPCCapability {
    pub fn new() -> Self {
        IPCCapability {
            can_send: false,
            can_receive: false,
            can_create: false,
            can_destroy: false,
        }
    }

    pub fn full() -> Self {
        IPCCapability {
            can_send: true,
            can_receive: true,
            can_create: true,
            can_destroy: true,
        }
    }
}

/// Pipe (OOP: Concrete IPC implementation)
#[repr(C)]
pub struct Pipe {
    pub id: usize,
    pub read_end: AtomicUsize,
    pub write_end: AtomicUsize,
    pub buffer: Option<NonNull<u8>>,
    pub buffer_size: usize,
    pub read_pos: AtomicUsize,
    pub write_pos: AtomicUsize,
    pub capability: IPCCapability,
    pub closed: AtomicBool,
}

impl Pipe {
    pub fn new(id: usize, buffer_size: usize, capability: IPCCapability) -> Self {
        let buffer = unsafe {
            let ptr = alloc(buffer_size);
            if ptr.is_null() {
                None
            } else {
                Some(NonNull::new_unchecked(ptr))
            }
        };

        Pipe {
            id,
            read_end: AtomicUsize::new(0),
            write_end: AtomicUsize::new(0),
            buffer,
            buffer_size,
            read_pos: AtomicUsize::new(0),
            write_pos: AtomicUsize::new(0),
            capability,
            closed: AtomicBool::new(false),
        }
    }
}

impl IPCEndpoint for Pipe {
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        if !self.capability.can_send {
            return Err(IPCError::PermissionDenied);
        }

        if self.closed.load(Ordering::SeqCst) {
            return Err(IPCError::NotConnected);
        }

        let buffer = match self.buffer {
            Some(ptr) => ptr.as_ptr(),
            None => return Err(IPCError::NotConnected),
        };

        unsafe {
            let available_space = self.buffer_size - (self.write_pos.load(Ordering::SeqCst) - self.read_pos.load(Ordering::SeqCst));
            if message.len() > available_space {
                return Err(IPCError::BufferFull);
            }

            for (i, &byte) in message.iter().enumerate() {
                let write_pos = self.write_pos.load(Ordering::SeqCst) % self.buffer_size;
                *(buffer.add(write_pos)) = byte;
                self.write_pos.fetch_add(1, Ordering::SeqCst);
            }
        }

        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        if !self.capability.can_receive {
            return Err(IPCError::PermissionDenied);
        }

        if self.closed.load(Ordering::SeqCst) {
            return Err(IPCError::NotConnected);
        }

        let src_buffer = match self.buffer {
            Some(ptr) => ptr.as_ptr(),
            None => return Err(IPCError::NotConnected),
        };

        unsafe {
            let available_data = self.write_pos.load(Ordering::SeqCst) - self.read_pos.load(Ordering::SeqCst);
            let read_count = buffer.len().min(available_data);

            for i in 0..read_count {
                let read_pos = self.read_pos.load(Ordering::SeqCst) % self.buffer_size;
                buffer[i] = *(src_buffer.add(read_pos));
                self.read_pos.fetch_add(1, Ordering::SeqCst);
            }

            Ok(read_count)
        }
    }

    fn info(&self) -> IPCInfo {
        let mut info = IPCInfo::new(IPCType::Pipe);
        info.capacity = self.buffer_size;
        info.buffer_size = self.buffer_size;
        info.connected = !self.closed.load(Ordering::SeqCst);
        info
    }

    fn close(&mut self) -> Result<(), IPCError> {
        self.closed.store(true, Ordering::SeqCst);
        Ok(())
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        unsafe {
            if let Some(buffer) = self.buffer {
                free(buffer.as_ptr());
            }
        }
    }
}

/// Message queue (OOP: Concrete IPC implementation)
#[repr(C)]
pub struct MessageQueue {
    pub id: usize,
    pub messages: Vec<Message>,
    pub capacity: usize,
    pub capability: IPCCapability,
}

#[repr(C)]
pub struct Message {
    pub data: Vec<u8>,
    pub priority: u8,
}

impl Message {
    pub fn new(data: &[u8], priority: u8) -> Self {
        Message {
            data: data.to_vec(),
            priority,
        }
    }
}

impl MessageQueue {
    pub fn new(id: usize, capacity: usize, capability: IPCCapability) -> Self {
        MessageQueue {
            id,
            messages: Vec::new(),
            capacity,
            capability,
        }
    }
}

impl IPCEndpoint for MessageQueue {
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        if !self.capability.can_send {
            return Err(IPCError::PermissionDenied);
        }

        if self.messages.len() >= self.capacity {
            return Err(IPCError::BufferFull);
        }

        let msg = Message::new(message, 0);
        self.messages.push(msg);
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        if !self.capability.can_receive {
            return Err(IPCError::PermissionDenied);
        }

        if self.messages.is_empty() {
            return Err(IPCError::BufferEmpty);
        }

        let message = self.messages.remove(0);
        let len = buffer.len().min(message.data.len());
        buffer[..len].copy_from_slice(&message.data[..len]);

        Ok(len)
    }

    fn info(&self) -> IPCInfo {
        let mut info = IPCInfo::new(IPCType::MessageQueue);
        info.capacity = self.capacity;
        info.buffer_size = self.capacity * 256; // Assume max message size
        info.connected = true;
        info
    }

    fn close(&mut self) -> Result<(), IPCError> {
        self.messages.clear();
        Ok(())
    }
}

/// Shared memory (OOP: Concrete IPC implementation)
#[repr(C)]
pub struct SharedMemory {
    pub id: usize,
    pub data: Option<NonNull<u8>>,
    pub size: usize,
    pub ref_count: AtomicUsize,
    pub capability: IPCCapability,
}

impl SharedMemory {
    pub fn new(id: usize, size: usize, capability: IPCCapability) -> Self {
        let data = unsafe {
            let ptr = alloc(size);
            if ptr.is_null() {
                None
            } else {
                Some(NonNull::new_unchecked(ptr))
            }
        };

        SharedMemory {
            id,
            data,
            size,
            ref_count: AtomicUsize::new(0),
            capability,
        }
    }

    pub unsafe fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<usize, IPCError> {
        if !self.capability.can_receive {
            return Err(IPCError::PermissionDenied);
        }

        let data = match self.data {
            Some(ptr) => ptr.as_ptr(),
            None => return Err(IPCError::NotConnected),
        };

        let len = buffer.len().min(self.size - offset);
        buffer[..len].copy_from_slice(core::slice::from_raw_parts(data.add(offset), len));

        Ok(len)
    }

    pub unsafe fn write(&self, offset: usize, buffer: &[u8]) -> Result<usize, IPCError> {
        if !self.capability.can_send {
            return Err(IPCError::PermissionDenied);
        }

        let data = match self.data {
            Some(ptr) => ptr.as_ptr(),
            None => return Err(IPCError::NotConnected),
        };

        let len = buffer.len().min(self.size - offset);
        core::slice::from_raw_parts_mut(data.add(offset), len).copy_from_slice(&buffer[..len]);

        Ok(len)
    }

    pub fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_ref(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

impl IPCEndpoint for SharedMemory {
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        unsafe {
            let len = self.write(0, message)?;
            if len != message.len() {
                return Err(IPCError::InvalidSize);
            }
        }
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        unsafe {
            self.read(0, buffer)
        }
    }

    fn info(&self) -> IPCInfo {
        let mut info = IPCInfo::new(IPCType::SharedMemory);
        info.capacity = self.size;
        info.buffer_size = self.size;
        info.connected = self.data.is_some();
        info
    }

    fn close(&mut self) -> Result<(), IPCError> {
        unsafe {
            if let Some(data) = self.data {
                free(data.as_ptr());
                self.data = None;
            }
        }
        Ok(())
    }
}

impl Drop for SharedMemory {
    fn drop(&mut self) {
        unsafe {
            if let Some(data) = self.data {
                free(data.as_ptr());
            }
        }
    }
}

/// IPC manager (OOP: Manager class)
pub struct IPCManager {
    pipes: Vec<Option<NonNull<Pipe>>>,
    message_queues: Vec<Option<NonNull<MessageQueue>>>,
    shared_memories: Vec<Option<NonNull<SharedMemory>>>,
    next_id: AtomicUsize,
}

impl IPCManager {
    pub fn new() -> Self {
        IPCManager {
            pipes: Vec::new(),
            message_queues: Vec::new(),
            shared_memories: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn create_pipe(&mut self, buffer_size: usize, capability: IPCCapability) -> Result<usize, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let pipe = Pipe::new(id, buffer_size, capability);

        let pipe_ptr = unsafe {
            let ptr = alloc(mem::size_of::<Pipe>()) as *mut Pipe;
            if ptr.is_null() {
                return Err(IPCError::BufferFull);
            }
            core::ptr::write(ptr, pipe);
            NonNull::new_unchecked(ptr)
        };

        self.pipes.push(Some(pipe_ptr));
        Ok(id)
    }

    pub fn create_message_queue(&mut self, capacity: usize, capability: IPCCapability) -> Result<usize, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mq = MessageQueue::new(id, capacity, capability);

        let mq_ptr = unsafe {
            let ptr = alloc(mem::size_of::<MessageQueue>()) as *mut MessageQueue;
            if ptr.is_null() {
                return Err(IPCError::BufferFull);
            }
            core::ptr::write(ptr, mq);
            NonNull::new_unchecked(ptr)
        };

        self.message_queues.push(Some(mq_ptr));
        Ok(id)
    }

    pub fn create_shared_memory(&mut self, size: usize, capability: IPCCapability) -> Result<usize, IPCError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let shm = SharedMemory::new(id, size, capability);

        let shm_ptr = unsafe {
            let ptr = alloc(mem::size_of::<SharedMemory>()) as *mut SharedMemory;
            if ptr.is_null() {
                return Err(IPCError::BufferFull);
            }
            core::ptr::write(ptr, shm);
            NonNull::new_unchecked(ptr)
        };

        self.shared_memories.push(Some(shm_ptr));
        Ok(id)
    }

    pub unsafe fn get_pipe(&self, id: usize) -> Option<&Pipe> {
        if id < self.pipes.len() {
            self.pipes[id].map(|ptr| &*ptr.as_ptr())
        } else {
            None
        }
    }

    pub unsafe fn get_pipe_mut(&mut self, id: usize) -> Option<&mut Pipe> {
        if id < self.pipes.len() {
            self.pipes[id].map(|mut ptr| &mut *ptr.as_ptr())
        } else {
            None
        }
    }

    pub unsafe fn get_message_queue(&self, id: usize) -> Option<&MessageQueue> {
        if id < self.message_queues.len() {
            self.message_queues[id].map(|ptr| &*ptr.as_ptr())
        } else {
            None
        }
    }

    pub unsafe fn get_message_queue_mut(&mut self, id: usize) -> Option<&mut MessageQueue> {
        if id < self.message_queues.len() {
            self.message_queues[id].map(|mut ptr| &mut *ptr.as_ptr())
        } else {
            None
        }
    }

    pub unsafe fn get_shared_memory(&self, id: usize) -> Option<&SharedMemory> {
        if id < self.shared_memories.len() {
            self.shared_memories[id].map(|ptr| &*ptr.as_ptr())
        } else {
            None
        }
    }

    pub unsafe fn get_shared_memory_mut(&mut self, id: usize) -> Option<&mut SharedMemory> {
        if id < self.shared_memories.len() {
            self.shared_memories[id].map(|mut ptr| &mut *ptr.as_ptr())
        } else {
            None
        }
    }

    pub unsafe fn destroy_pipe(&mut self, id: usize) -> Result<(), IPCError> {
        if id >= self.pipes.len() {
            return Err(IPCError::NotConnected);
        }

        if let Some(pipe_ptr) = self.pipes[id] {
            core::ptr::drop_in_place(pipe_ptr.as_ptr());
            free(pipe_ptr.as_ptr() as *mut u8);
        }

        self.pipes[id] = None;
        Ok(())
    }

    pub unsafe fn destroy_message_queue(&mut self, id: usize) -> Result<(), IPCError> {
        if id >= self.message_queues.len() {
            return Err(IPCError::NotConnected);
        }

        if let Some(mq_ptr) = self.message_queues[id] {
            core::ptr::drop_in_place(mq_ptr.as_ptr());
            free(mq_ptr.as_ptr() as *mut u8);
        }

        self.message_queues[id] = None;
        Ok(())
    }

    pub unsafe fn destroy_shared_memory(&mut self, id: usize) -> Result<(), IPCError> {
        if id >= self.shared_memories.len() {
            return Err(IPCError::NotConnected);
        }

        if let Some(shm_ptr) = self.shared_memories[id] {
            core::ptr::drop_in_place(shm_ptr.as_ptr());
            free(shm_ptr.as_ptr() as *mut u8);
        }

        self.shared_memories[id] = None;
        Ok(())
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

    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            core::ptr::copy(self.data.add(index + 1), self.data.add(index), self.len - index - 1);
            self.len -= 1;
            item
        }
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
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
