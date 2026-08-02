//! OOP-based IPC System for SigmaOS with SerenityOS Parity
//!
//! Implements strongly-typed message routing, shared-memory window backing stores,
//! and capability-gated security gates inspired by SerenityOS LibIPC and WindowServer.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use alloc::string::ToString;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPCType {
    Pipe = 0,
    MessageQueue = 1,
    SharedMemory = 2,
    Socket = 3,
    Signal = 4,
}

/// IPC info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPCCapability {
    pub can_send: bool,
    pub can_receive: bool,
    pub can_create: bool,
    pub can_destroy: bool,
    // Serenity parity sandboxing rules
    pub allow_send_fd: bool,
    pub allow_unix_sockets: bool,
}

impl IPCCapability {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        IPCCapability {
            can_send: false,
            can_receive: false,
            can_create: false,
            can_destroy: false,
            allow_send_fd: false,
            allow_unix_sockets: false,
        }
    }

    pub fn full() -> Self {
        IPCCapability {
            can_send: true,
            can_receive: true,
            can_create: true,
            can_destroy: true,
            allow_send_fd: true,
            allow_unix_sockets: true,
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

            for &byte in message {
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
    pub messages: CustomIpcVec<Message>,
    pub capacity: usize,
    pub capability: IPCCapability,
}

#[repr(C)]
pub struct Message {
    pub data: CustomIpcVec<u8>,
    pub priority: u8,
}

impl Message {
    pub fn new(data: &[u8], priority: u8) -> Self {
        let mut v = CustomIpcVec::new();
        for &byte in data {
            v.push(byte);
        }
        Message {
            data: v,
            priority,
        }
    }
}

impl MessageQueue {
    pub fn new(id: usize, capacity: usize, capability: IPCCapability) -> Self {
        MessageQueue {
            id,
            messages: CustomIpcVec::new(),
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

// =========================================================================
// SerenityOS Parity Extensions: Strongly-Typed IPC, WindowServer Backing, Sandboxing
// =========================================================================

/// Serenity-inspired strongly-typed IPC messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerenityIpcMessage {
    /// WindowServer Backing Store mapping updates: (window_id, shm_id, width, height)
    UpdateBackingStore { window_id: usize, shm_id: usize, width: usize, height: usize },
    /// Standard key/mouse input event message: (window_id, event_type, x, y)
    InputEvent { window_id: usize, event_type: u32, x: i32, y: i32 },
    /// General system call adaptation payload: (syscall_id, payload)
    SyscallShim { syscall_id: usize, payload: Vec<u8> },
}

impl SerenityIpcMessage {
    /// Serialize the message into strongly-typed bytes
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Self::UpdateBackingStore { window_id, shm_id, width, height } => {
                out.push(1); // Msg Type ID
                out.extend_from_slice(&window_id.to_le_bytes());
                out.extend_from_slice(&shm_id.to_le_bytes());
                out.extend_from_slice(&width.to_le_bytes());
                out.extend_from_slice(&height.to_le_bytes());
            }
            Self::InputEvent { window_id, event_type, x, y } => {
                out.push(2); // Msg Type ID
                out.extend_from_slice(&window_id.to_le_bytes());
                out.extend_from_slice(&event_type.to_le_bytes());
                out.extend_from_slice(&x.to_le_bytes());
                out.extend_from_slice(&y.to_le_bytes());
            }
            Self::SyscallShim { syscall_id, payload } => {
                out.push(3); // Msg Type ID
                out.extend_from_slice(&syscall_id.to_le_bytes());
                out.extend_from_slice(&payload);
            }
        }
        out
    }

    /// Deserialize raw bytes back into strongly-typed messages
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }
        let type_id = bytes[0];
        match type_id {
            1 => {
                let sz = mem::size_of::<usize>();
                if bytes.len() < 1 + 4 * sz {
                    return None;
                }

                let read_usize = |offset: usize| {
                    let mut b = [0; mem::size_of::<usize>()];
                    b.copy_from_slice(&bytes[offset..offset + sz]);
                    usize::from_le_bytes(b)
                };

                let window_id = read_usize(1);
                let shm_id = read_usize(1 + sz);
                let width = read_usize(1 + 2 * sz);
                let height = read_usize(1 + 3 * sz);
                Some(Self::UpdateBackingStore { window_id, shm_id, width, height })
            }
            2 => {
                let sz = mem::size_of::<usize>();
                if bytes.len() < 1 + sz + 12 {
                    return None;
                }

                let read_usize = |offset: usize| {
                    let mut b = [0; mem::size_of::<usize>()];
                    b.copy_from_slice(&bytes[offset..offset + sz]);
                    usize::from_le_bytes(b)
                };

                let read_u32 = |offset: usize| {
                    let mut b = [0; 4];
                    b.copy_from_slice(&bytes[offset..offset + 4]);
                    u32::from_le_bytes(b)
                };

                let read_i32 = |offset: usize| {
                    let mut b = [0; 4];
                    b.copy_from_slice(&bytes[offset..offset + 4]);
                    i32::from_le_bytes(b)
                };

                let window_id = read_usize(1);
                let event_type = read_u32(1 + sz);
                let x = read_i32(1 + sz + 4);
                let y = read_i32(1 + sz + 8);
                Some(Self::InputEvent { window_id, event_type, x, y })
            }
            3 => {
                let sz = mem::size_of::<usize>();
                if bytes.len() < 1 + sz {
                    return None;
                }
                let mut b = [0; mem::size_of::<usize>()];
                b.copy_from_slice(&bytes[1..1 + sz]);
                let syscall_id = usize::from_le_bytes(b);
                let payload = bytes[1 + sz..].to_vec();
                Some(Self::SyscallShim { syscall_id, payload })
            }
            _ => None,
        }
    }
}

/// Serenity WindowServer Backing Store interface.
/// Encapsulates direct shared-memory painting backing buffers.
pub struct SerenitySharedBackingStore {
    pub shm_id: usize,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

impl SerenitySharedBackingStore {
    pub fn new(shm_id: usize, width: usize, height: usize) -> Self {
        Self {
            shm_id,
            width,
            height,
            stride: width * 4, // 32-bit RGBA pixel formats
        }
    }

    /// Read or write pixel offsets in the backing store buffer mapped via shared memory.
    pub fn get_pixel_offset(&self, x: usize, y: usize) -> usize {
        y * self.stride + x * 4
    }
}

/// Serenity-style Sandboxed IPC Enforcer.
/// Ensures that processes sandboxed with pledge/unveil restrictions cannot bypass sandbox boundaries
/// by sending or receiving file descriptors or establishing disallowed socket connections over IPC.
pub struct SerenityIpcSandboxEnforcer {
    pub has_send_fd_pledge: bool,
    pub has_unix_sockets_pledge: bool,
}

impl SerenityIpcSandboxEnforcer {
    pub fn new(has_send_fd_pledge: bool, has_unix_sockets_pledge: bool) -> Self {
        Self {
            has_send_fd_pledge,
            has_unix_sockets_pledge,
        }
    }

    /// Validate whether a message or dynamic transfer is allowed under the current sandbox parameters.
    pub fn validate_ipc_transfer(&self, is_sending_fd: bool, is_unix_connect: bool) -> Result<(), IPCError> {
        if is_sending_fd && !self.has_send_fd_pledge {
            return Err(IPCError::PermissionDenied);
        }
        if is_unix_connect && !self.has_unix_sockets_pledge {
            return Err(IPCError::PermissionDenied);
        }
        Ok(())
    }
}

// =========================================================================
// IPC manager (OOP: Manager class)
// =========================================================================

pub struct IPCManager {
    pipes: CustomIpcVec<Option<NonNull<Pipe>>>,
    message_queues: CustomIpcVec<Option<NonNull<MessageQueue>>>,
    shared_memories: CustomIpcVec<Option<NonNull<SharedMemory>>>,
    next_id: AtomicUsize,
    // Serenity Parity Enforcer integration
    pub sandbox_enforcer: SerenityIpcSandboxEnforcer,
}

impl IPCManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        IPCManager {
            pipes: CustomIpcVec::new(),
            message_queues: CustomIpcVec::new(),
            shared_memories: CustomIpcVec::new(),
            next_id: AtomicUsize::new(1),
            sandbox_enforcer: SerenityIpcSandboxEnforcer::new(true, true),
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

/// Simple CustomIpcVec implementation for no_std
pub struct CustomIpcVec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> CustomIpcVec<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        CustomIpcVec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
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

    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            core::ptr::copy(self.data.add(index + 1), self.data.add(index), self.len - index - 1);
            self.len -= 1;
            item
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
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

impl<T> core::ops::Deref for CustomIpcVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for CustomIpcVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a CustomIpcVec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut CustomIpcVec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serenity_ipc_message_serialization() {
        let msg = SerenityIpcMessage::UpdateBackingStore {
            window_id: 12,
            shm_id: 45,
            width: 800,
            height: 600,
        };
        let bytes = msg.serialize();
        assert_eq!(bytes[0], 1); // Msg Type ID 1

        let decoded = SerenityIpcMessage::deserialize(&bytes).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_serenity_input_event_serialization() {
        let msg = SerenityIpcMessage::InputEvent {
            window_id: 3,
            event_type: 100,
            x: -25,
            y: 400,
        };
        let bytes = msg.serialize();
        assert_eq!(bytes[0], 2); // Msg Type ID 2

        let decoded = SerenityIpcMessage::deserialize(&bytes).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_shared_backing_store() {
        let backing = SerenitySharedBackingStore::new(99, 1024, 768);
        assert_eq!(backing.get_pixel_offset(10, 20), 20 * 1024 * 4 + 10 * 4);
    }

    #[test]
    fn test_serenity_ipc_sandboxing() {
        let enforcer = SerenityIpcSandboxEnforcer::new(false, false);
        // Blocking FD sends since send_fd capability is not pledged
        assert!(enforcer.validate_ipc_transfer(true, false).is_err());
        // Blocking unix sockets since unix sockets are not pledged
        assert!(enforcer.validate_ipc_transfer(false, true).is_err());

        let enforcer_full = SerenityIpcSandboxEnforcer::new(true, true);
        assert!(enforcer_full.validate_ipc_transfer(true, true).is_ok());
    }
}
