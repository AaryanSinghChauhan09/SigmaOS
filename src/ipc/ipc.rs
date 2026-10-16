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
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use core::ptr::NonNull;
use std::vec::Vec;

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
    EndpointClosed = 6,
    NotFound = 7,
    AlreadyExists = 8,
    Timeout = 9,
    InternalError = 10,
    InvalidCapability = 11,
}

/// IPC channel type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPCType {
    Pipe = 0,
    MessageQueue = 1,
    SharedMemory = 2,
    Socket = 3,
    Signal = 4,
}

/// IPC endpoint information
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPCInfo {
    pub endpoint_id: usize,
    pub channel_type: IPCType,
    pub owner_pid: usize,
    pub connected: bool,
    pub bytes_transferred: usize,
}

/// Capability required for IPC operation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPCCapability {
    pub cap_id: usize,
    pub read_allowed: bool,
    pub write_allowed: bool,
    pub execute_allowed: bool,
    pub grant_allowed: bool,
}

impl IPCCapability {
    pub const READ: usize = 1 << 0;
    pub const WRITE: usize = 1 << 1;
    pub const EXECUTE: usize = 1 << 2;
    pub const GRANT: usize = 1 << 3;

    pub fn new(cap_id: usize, mask: usize) -> Self {
        Self {
            cap_id,
            read_allowed: (mask & Self::READ) != 0,
            write_allowed: (mask & Self::WRITE) != 0,
            execute_allowed: (mask & Self::EXECUTE) != 0,
            grant_allowed: (mask & Self::GRANT) != 0,
        }
    }

    pub fn check_read(&self) -> Result<(), IPCError> {
        if self.read_allowed {
            Ok(())
        } else {
            Err(IPCError::PermissionDenied)
        }
    }

    pub fn check_write(&self) -> Result<(), IPCError> {
        if self.write_allowed {
            Ok(())
        } else {
            Err(IPCError::PermissionDenied)
        }
    }
}

// =========================================================================
// Canonical Core IPC Handlers (Phase 2 Specifications)
// =========================================================================

/// IPC Permissions for access control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcPermissions {
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub mode: u16,
    pub capabilities: u64,
}

impl IpcPermissions {
    pub fn new(owner_uid: u32, owner_gid: u32, mode: u16) -> Self {
        Self {
            owner_uid,
            owner_gid,
            mode,
            capabilities: 0,
        }
    }

    pub fn check_access(&self, uid: u32, gid: u32, requested_mode: u16) -> bool {
        let perm = if uid == self.owner_uid {
            (self.mode >> 6) & 0x7
        } else if gid == self.owner_gid {
            (self.mode >> 3) & 0x7
        } else {
            self.mode & 0x7
        };
        (perm & requested_mode) == requested_mode
    }
}

/// Statistics tracking for IPC channels
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IpcStatistics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub dropped_messages: u64,
}

/// Bounded ring buffer for IPC data transmission
#[derive(Debug, Clone)]
pub struct MessageBuffer {
    buffer: Vec<u8>,
    capacity: usize,
    read_pos: usize,
    write_pos: usize,
    len: usize,
}

impl MessageBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            capacity,
            read_pos: 0,
            write_pos: 0,
            len: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let available = self.capacity - self.len;
        let to_write = data.len().min(available);
        for i in 0..to_write {
            self.buffer[self.write_pos] = data[i];
            self.write_pos = (self.write_pos + 1) % self.capacity;
        }
        self.len += to_write;
        to_write
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let to_read = buf.len().min(self.len);
        for i in 0..to_read {
            buf[i] = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.capacity;
        }
        self.len -= to_read;
        to_read
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Canonical Wait Queue for thread block/wakeup synchronization
#[derive(Debug, Default, Clone)]
pub struct WaitQueue {
    waiters: Vec<usize>, // List of blocked thread PIDs
}

impl WaitQueue {
    pub fn new() -> Self {
        Self {
            waiters: Vec::new(),
        }
    }

    pub fn add_waiter(&mut self, thread_pid: usize) {
        if !self.waiters.contains(&thread_pid) {
            self.waiters.push(thread_pid);
        }
    }

    pub fn remove_waiter(&mut self, thread_pid: usize) {
        self.waiters.retain(|&pid| pid != thread_pid);
    }

    pub fn wake_one(&mut self) -> Option<usize> {
        if self.waiters.is_empty() {
            None
        } else {
            Some(self.waiters.remove(0))
        }
    }

    pub fn wake_all(&mut self) -> Vec<usize> {
        let waiters = self.waiters.clone();
        self.waiters.clear();
        waiters
    }

    pub fn waiter_count(&self) -> usize {
        self.waiters.len()
    }
}

/// Canonical IpcChannel representing an active IPC communication channel
#[derive(Debug)]
pub struct IpcChannel {
    pub channel_id: usize,
    pub channel_type: IPCType,
    pub buffer: MessageBuffer,
    pub permissions: IpcPermissions,
    pub stats: IpcStatistics,
    pub read_wait_queue: WaitQueue,
    pub write_wait_queue: WaitQueue,
    pub closed: bool,
}

impl IpcChannel {
    pub fn new(
        channel_id: usize,
        channel_type: IPCType,
        capacity: usize,
        permissions: IpcPermissions,
    ) -> Self {
        Self {
            channel_id,
            channel_type,
            buffer: MessageBuffer::new(capacity),
            permissions,
            stats: IpcStatistics::default(),
            read_wait_queue: WaitQueue::new(),
            write_wait_queue: WaitQueue::new(),
            closed: false,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        let written = self.buffer.write(data);
        self.stats.bytes_written += written as u64;
        self.stats.messages_sent += 1;
        if written > 0 {
            self.read_wait_queue.wake_all();
        }
        Ok(written)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, IPCError> {
        if self.buffer.is_empty() {
            if self.closed {
                return Ok(0); // EOF
            }
            return Err(IPCError::BufferEmpty);
        }
        let read_bytes = self.buffer.read(buf);
        self.stats.bytes_read += read_bytes as u64;
        self.stats.messages_received += 1;
        if read_bytes > 0 {
            self.write_wait_queue.wake_all();
        }
        Ok(read_bytes)
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.read_wait_queue.wake_all();
        self.write_wait_queue.wake_all();
    }
}

/// Endpoint handle connected to an IPC channel
#[derive(Debug, Clone)]
pub struct IpcEndpoint {
    pub endpoint_id: usize,
    pub channel_id: usize,
    pub owner_pid: usize,
    pub read_mode: bool,
    pub write_mode: bool,
    pub closed: bool,
}

impl IpcEndpoint {
    pub fn new(
        endpoint_id: usize,
        channel_id: usize,
        owner_pid: usize,
        read_mode: bool,
        write_mode: bool,
    ) -> Self {
        Self {
            endpoint_id,
            channel_id,
            owner_pid,
            read_mode,
            write_mode,
            closed: false,
        }
    }

    pub fn close(&mut self) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.closed = true;
        Ok(())
    }
}

/// Shared memory region handle
#[derive(Debug)]
pub struct SharedMemoryRegion {
    pub region_id: usize,
    pub size: usize,
    pub permissions: IpcPermissions,
    pub ref_count: usize,
    pub backing_data: Vec<u8>,
}

impl SharedMemoryRegion {
    pub fn new(region_id: usize, size: usize, permissions: IpcPermissions) -> Self {
        Self {
            region_id,
            size,
            permissions,
            ref_count: 1,
            backing_data: vec![0u8; size],
        }
    }

    pub fn attach(&mut self) {
        self.ref_count += 1;
    }

    pub fn detach(&mut self) -> usize {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count
    }
}

/// File descriptor wrapper for IPC primitives
#[derive(Debug, Clone)]
pub struct FileDescriptorHandle {
    pub fd: i32,
    pub channel_id: usize,
    pub is_read_end: bool,
    pub is_closed: bool,
}

impl FileDescriptorHandle {
    pub fn new(fd: i32, channel_id: usize, is_read_end: bool) -> Self {
        Self {
            fd,
            channel_id,
            is_read_end,
            is_closed: false,
        }
    }

    pub fn close(&mut self) -> Result<(), IPCError> {
        if self.is_closed {
            return Err(IPCError::EndpointClosed);
        }
        self.is_closed = true;
        Ok(())
    }
}

/// Concrete Pipe implementation
pub struct Pipe {
    info: IPCInfo,
    capability: IPCCapability,
    buffer: Vec<u8>,
    capacity: usize,
    head: usize,
    tail: usize,
    len: usize,
    closed: bool,
}

impl Pipe {
    pub fn new(endpoint_id: usize, owner_pid: usize, capacity: usize) -> Self {
        Self {
            info: IPCInfo {
                endpoint_id,
                channel_type: IPCType::Pipe,
                owner_pid,
                connected: true,
                bytes_transferred: 0,
            },
            capability: IPCCapability::new(endpoint_id, IPCCapability::READ | IPCCapability::WRITE),
            buffer: vec![0u8; capacity],
            capacity,
            head: 0,
            tail: 0,
            len: 0,
            closed: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn available_read(&self) -> usize {
        self.len
    }

    pub fn available_write(&self) -> usize {
        self.capacity - self.len
    }
}

impl IPCEndpoint for Pipe {
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.capability.check_write()?;

        if message.len() > self.available_write() {
            return Err(IPCError::BufferFull);
        }

        for &byte in message {
            self.buffer[self.tail] = byte;
            self.tail = (self.tail + 1) % self.capacity;
        }

        self.len += message.len();
        self.info.bytes_transferred += message.len();
        Ok(())
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        if self.closed && self.is_empty() {
            return Err(IPCError::EndpointClosed);
        }
        self.capability.check_read()?;

        if self.is_empty() {
            return Err(IPCError::BufferEmpty);
        }

        let to_read = core::cmp::min(buffer.len(), self.len);
        for i in 0..to_read {
            buffer[i] = self.buffer[self.head];
            self.head = (self.head + 1) % self.capacity;
        }

        self.len -= to_read;
        Ok(to_read)
    }

    fn info(&self) -> IPCInfo {
        self.info
    }

    fn close(&mut self) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.closed = true;
        self.info.connected = false;
        Ok(())
    }
}

/// Message Queue implementation
pub struct MessageQueue {
    info: IPCInfo,
    capability: IPCCapability,
    messages: Vec<Message>,
    max_messages: usize,
    closed: bool,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub sender_pid: usize,
    pub priority: u32,
    pub payload: Vec<u8>,
}

impl MessageQueue {
    pub fn new(endpoint_id: usize, owner_pid: usize, max_messages: usize) -> Self {
        Self {
            info: IPCInfo {
                endpoint_id,
                channel_type: IPCType::MessageQueue,
                owner_pid,
                connected: true,
                bytes_transferred: 0,
            },
            capability: IPCCapability::new(endpoint_id, IPCCapability::READ | IPCCapability::WRITE),
            messages: Vec::new(),
            max_messages,
            closed: false,
        }
    }

    pub fn send_with_priority(&mut self, message: &[u8], priority: u32) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.capability.check_write()?;

        if self.messages.len() >= self.max_messages {
            return Err(IPCError::BufferFull);
        }

        let msg = Message {
            sender_pid: self.info.owner_pid,
            priority,
            payload: message.to_vec(),
        };

        // Insert sorted by priority (higher priority first)
        let pos = self
            .messages
            .iter()
            .position(|m| m.priority < priority)
            .unwrap_or(self.messages.len());

        self.messages.insert(pos, msg);
        self.info.bytes_transferred += message.len();
        Ok(())
    }
}

impl IPCEndpoint for MessageQueue {
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        self.send_with_priority(message, 0)
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        if self.closed && self.messages.is_empty() {
            return Err(IPCError::EndpointClosed);
        }
        self.capability.check_read()?;

        if self.messages.is_empty() {
            return Err(IPCError::BufferEmpty);
        }

        let msg = self.messages.remove(0);
        if buffer.len() < msg.payload.len() {
            return Err(IPCError::InvalidSize);
        }

        buffer[..msg.payload.len()].copy_from_slice(&msg.payload);
        Ok(msg.payload.len())
    }

    fn info(&self) -> IPCInfo {
        self.info
    }

    fn close(&mut self) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.closed = true;
        self.info.connected = false;
        Ok(())
    }
}

/// Shared Memory implementation
pub struct SharedMemory {
    info: IPCInfo,
    capability: IPCCapability,
    size: usize,
    data: NonNull<u8>,
    attached_pids: Vec<usize>,
    closed: bool,
}

impl SharedMemory {
    pub fn new(endpoint_id: usize, owner_pid: usize, size: usize) -> Result<Self, IPCError> {
        if size == 0 {
            return Err(IPCError::InvalidSize);
        }

        let layout =
            std::alloc::Layout::from_size_align(size, 8).map_err(|_| IPCError::InternalError)?;

        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        let non_null = NonNull::new(ptr).ok_or(IPCError::InternalError)?;

        let mut attached = Vec::new();
        attached.push(owner_pid);

        Ok(Self {
            info: IPCInfo {
                endpoint_id,
                channel_type: IPCType::SharedMemory,
                owner_pid,
                connected: true,
                bytes_transferred: 0,
            },
            capability: IPCCapability::new(
                endpoint_id,
                IPCCapability::READ | IPCCapability::WRITE | IPCCapability::GRANT,
            ),
            size,
            data: non_null,
            attached_pids: attached,
            closed: false,
        })
    }

    pub fn attach(&mut self, pid: usize) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        if !self.attached_pids.contains(&pid) {
            self.attached_pids.push(pid);
        }
        Ok(())
    }

    pub fn detach(&mut self, pid: usize) -> Result<(), IPCError> {
        if let Some(pos) = self.attached_pids.iter().position(|&p| p == pid) {
            self.attached_pids.remove(pos);
            Ok(())
        } else {
            Err(IPCError::NotFound)
        }
    }

    pub fn read_at(&self, offset: usize, buffer: &mut [u8]) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.capability.check_read()?;

        if offset + buffer.len() > self.size {
            return Err(IPCError::InvalidSize);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                self.data.as_ptr().add(offset),
                buffer.as_mut_ptr(),
                buffer.len(),
            );
        }
        Ok(())
    }

    pub fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.capability.check_write()?;

        if offset + data.len() > self.size {
            return Err(IPCError::InvalidSize);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.data.as_ptr().add(offset),
                data.len(),
            );
        }
        self.info.bytes_transferred += data.len();
        Ok(())
    }
}

impl IPCEndpoint for SharedMemory {
    fn send(&mut self, message: &[u8]) -> Result<(), IPCError> {
        self.write_at(0, message)
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IPCError> {
        self.read_at(0, buffer)?;
        Ok(buffer.len())
    }

    fn info(&self) -> IPCInfo {
        self.info
    }

    fn close(&mut self) -> Result<(), IPCError> {
        if self.closed {
            return Err(IPCError::EndpointClosed);
        }
        self.closed = true;
        self.info.connected = false;
        Ok(())
    }
}

impl Drop for SharedMemory {
    fn drop(&mut self) {
        if self.size > 0 {
            if let Ok(layout) = std::alloc::Layout::from_size_align(self.size, 8) {
                unsafe {
                    std::alloc::dealloc(self.data.as_ptr(), layout);
                }
            }
        }
    }
}

/// SerenityOS LibIPC & WindowServer backing-store parity protocol message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerenityIpcMessage {
    UpdateBackingStore {
        window_id: u32,
        shm_id: u32,
        width: u32,
        height: u32,
    },
    InputEvent {
        window_id: u32,
        event_type: u16,
        x: i32,
        y: i32,
    },
}

impl SerenityIpcMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::UpdateBackingStore {
                window_id,
                shm_id,
                width,
                height,
            } => {
                buf.push(1); // Type 1
                buf.extend_from_slice(&window_id.to_le_bytes());
                buf.extend_from_slice(&shm_id.to_le_bytes());
                buf.extend_from_slice(&width.to_le_bytes());
                buf.extend_from_slice(&height.to_le_bytes());
            }
            Self::InputEvent {
                window_id,
                event_type,
                x,
                y,
            } => {
                buf.push(2); // Type 2
                buf.extend_from_slice(&window_id.to_le_bytes());
                buf.extend_from_slice(&event_type.to_le_bytes());
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
            }
        }
        buf
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, IPCError> {
        if buf.is_empty() {
            return Err(IPCError::InvalidSize);
        }
        match buf[0] {
            1 => {
                if buf.len() < 17 {
                    return Err(IPCError::InvalidSize);
                }
                let window_id = u32::from_le_bytes(buf[1..5].try_into().unwrap());
                let shm_id = u32::from_le_bytes(buf[5..9].try_into().unwrap());
                let width = u32::from_le_bytes(buf[9..13].try_into().unwrap());
                let height = u32::from_le_bytes(buf[13..17].try_into().unwrap());
                Ok(Self::UpdateBackingStore {
                    window_id,
                    shm_id,
                    width,
                    height,
                })
            }
            2 => {
                if buf.len() < 15 {
                    return Err(IPCError::InvalidSize);
                }
                let window_id = u32::from_le_bytes(buf[1..5].try_into().unwrap());
                let event_type = u16::from_le_bytes(buf[5..7].try_into().unwrap());
                let x = i32::from_le_bytes(buf[7..11].try_into().unwrap());
                let y = i32::from_le_bytes(buf[11..15].try_into().unwrap());
                Ok(Self::InputEvent {
                    window_id,
                    event_type,
                    x,
                    y,
                })
            }
            _ => Err(IPCError::InvalidSize),
        }
    }
}

/// SerenityOS WindowServer Shared Memory Backing Store.
pub struct SerenitySharedBackingStore {
    pub shm_id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

impl SerenitySharedBackingStore {
    pub fn new(shm_id: u32, width: u32, height: u32) -> Self {
        Self {
            shm_id,
            width,
            height,
            pitch: width * 4, // 32-bit RGBA
        }
    }

    pub fn get_pixel_offset(&self, x: u32, y: u32) -> usize {
        (y * self.pitch + x * 4) as usize
    }
}

/// SerenityOS Pledge & Unveil Sandbox IPC Enforcer.
pub struct SerenityIpcSandboxEnforcer {
    pub allow_send_fd: bool,
    pub allow_unix_sockets: bool,
}

impl SerenityIpcSandboxEnforcer {
    pub fn new(allow_send_fd: bool, allow_unix_sockets: bool) -> Self {
        Self {
            allow_send_fd,
            allow_unix_sockets,
        }
    }

    pub fn validate_ipc_transfer(
        &self,
        is_fd_passing: bool,
        is_unix_socket: bool,
    ) -> Result<(), IPCError> {
        if is_fd_passing && !self.allow_send_fd {
            return Err(IPCError::PermissionDenied);
        }
        if is_unix_socket && !self.allow_unix_sockets {
            return Err(IPCError::PermissionDenied);
        }
        Ok(())
    }
}

/// System-wide IPC Manager
pub struct IPCManager {
    pipes: [Option<NonNull<Pipe>>; 256],
    message_queues: [Option<NonNull<MessageQueue>>; 256],
    shared_memories: [Option<NonNull<SharedMemory>>; 256],
    next_id: usize,
}

impl IPCManager {
    pub fn new() -> Self {
        Self {
            pipes: [None; 256],
            message_queues: [None; 256],
            shared_memories: [None; 256],
            next_id: 1,
        }
    }

    pub fn create_pipe(&mut self, owner_pid: usize, capacity: usize) -> Result<usize, IPCError> {
        let id = self.next_id;
        if id >= 256 {
            return Err(IPCError::BufferFull);
        }

        let pipe = Pipe::new(id, owner_pid, capacity);
        let boxed = Box::new(pipe);
        let ptr = NonNull::new(Box::into_raw(boxed)).ok_or(IPCError::InternalError)?;

        self.pipes[id] = Some(ptr);
        self.next_id += 1;
        Ok(id)
    }

    pub fn create_message_queue(
        &mut self,
        owner_pid: usize,
        max_messages: usize,
    ) -> Result<usize, IPCError> {
        let id = self.next_id;
        if id >= 256 {
            return Err(IPCError::BufferFull);
        }

        let mq = MessageQueue::new(id, owner_pid, max_messages);
        let boxed = Box::new(mq);
        let ptr = NonNull::new(Box::into_raw(boxed)).ok_or(IPCError::InternalError)?;

        self.message_queues[id] = Some(ptr);
        self.next_id += 1;
        Ok(id)
    }

    pub fn create_shared_memory(
        &mut self,
        owner_pid: usize,
        size: usize,
    ) -> Result<usize, IPCError> {
        let id = self.next_id;
        if id >= 256 {
            return Err(IPCError::BufferFull);
        }

        let shm = SharedMemory::new(id, owner_pid, size)?;
        let boxed = Box::new(shm);
        let ptr = NonNull::new(Box::into_raw(boxed)).ok_or(IPCError::InternalError)?;

        self.shared_memories[id] = Some(ptr);
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_pipe(&mut self, id: usize) -> Result<&mut Pipe, IPCError> {
        if id >= 256 {
            return Err(IPCError::NotFound);
        }
        unsafe {
            self.pipes[id]
                .map(|ptr| &mut *ptr.as_ptr())
                .ok_or(IPCError::NotFound)
        }
    }

    pub fn get_message_queue(&mut self, id: usize) -> Result<&mut MessageQueue, IPCError> {
        if id >= 256 {
            return Err(IPCError::NotFound);
        }
        unsafe {
            self.message_queues[id]
                .map(|ptr| &mut *ptr.as_ptr())
                .ok_or(IPCError::NotFound)
        }
    }

    pub fn get_shared_memory(&mut self, id: usize) -> Result<&mut SharedMemory, IPCError> {
        if id >= 256 {
            return Err(IPCError::NotFound);
        }
        unsafe {
            self.shared_memories[id]
                .map(|ptr| &mut *ptr.as_ptr())
                .ok_or(IPCError::NotFound)
        }
    }

    pub fn destroy_endpoint(&mut self, id: usize, channel_type: IPCType) -> Result<(), IPCError> {
        if id >= 256 {
            return Err(IPCError::NotFound);
        }

        match channel_type {
            IPCType::Pipe => {
                if let Some(ptr) = self.pipes[id].take() {
                    unsafe {
                        let mut boxed = Box::from_raw(ptr.as_ptr());
                        boxed.close()?;
                    }
                    Ok(())
                } else {
                    Err(IPCError::NotFound)
                }
            }
            IPCType::MessageQueue => {
                if let Some(ptr) = self.message_queues[id].take() {
                    unsafe {
                        let mut boxed = Box::from_raw(ptr.as_ptr());
                        boxed.close()?;
                    }
                    Ok(())
                } else {
                    Err(IPCError::NotFound)
                }
            }
            IPCType::SharedMemory => {
                if let Some(ptr) = self.shared_memories[id].take() {
                    unsafe {
                        let mut boxed = Box::from_raw(ptr.as_ptr());
                        boxed.close()?;
                    }
                    Ok(())
                } else {
                    Err(IPCError::NotFound)
                }
            }
            _ => Err(IPCError::InvalidSize),
        }
    }
}

impl Drop for IPCManager {
    fn drop(&mut self) {
        for i in 0..256 {
            if let Some(ptr) = self.pipes[i].take() {
                unsafe {
                    let _ = Box::from_raw(ptr.as_ptr());
                }
            }
            if let Some(ptr) = self.message_queues[i].take() {
                unsafe {
                    let _ = Box::from_raw(ptr.as_ptr());
                }
            }
            if let Some(ptr) = self.shared_memories[i].take() {
                unsafe {
                    let _ = Box::from_raw(ptr.as_ptr());
                }
            }
        }
    }
}

/// Custom vector implementation for no_std IPC structures without standard lib
pub struct CustomIpcVec<T> {
    data: *mut T,
    capacity: usize,
    len: usize,
}

impl<T> CustomIpcVec<T> {
    pub const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
            capacity: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            let new_cap = if self.capacity == 0 {
                4
            } else {
                self.capacity * 2
            };
            let new_data = unsafe { alloc(new_cap * core::mem::size_of::<T>()) } as *mut T;
            if !self.data.is_null() {
                unsafe {
                    core::ptr::copy_nonoverlapping(self.data, new_data, self.len);
                    free(self.data as *mut u8);
                }
            }
            self.data = new_data;
            self.capacity = new_cap;
        }

        unsafe {
            core::ptr::write(self.data.add(self.len), item);
        }
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for CustomIpcVec<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            for i in 0..self.len {
                unsafe {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
            unsafe {
                free(self.data as *mut u8);
            }
        }
    }
}

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
    fn test_canonical_ipc_channel_and_objects() {
        let perms = IpcPermissions::new(1000, 1000, 0o660);
        assert!(perms.check_access(1000, 1000, 0o4));
        assert!(!perms.check_access(2000, 2000, 0o4));

        let mut channel = IpcChannel::new(1, IPCType::Pipe, 64, perms);
        let data = b"hello ipc world";
        let written = channel.write(data).unwrap();
        assert_eq!(written, data.len());

        let mut read_buf = [0u8; 64];
        let read_bytes = channel.read(&mut read_buf).unwrap();
        assert_eq!(read_bytes, data.len());
        assert_eq!(&read_buf[..read_bytes], data);

        assert_eq!(channel.stats.bytes_written, data.len() as u64);
        assert_eq!(channel.stats.bytes_read, data.len() as u64);

        let mut endpoint = IpcEndpoint::new(10, 1, 100, true, true);
        assert!(!endpoint.closed);
        assert!(endpoint.close().is_ok());
        assert_eq!(endpoint.close(), Err(IPCError::EndpointClosed));

        let mut shm = SharedMemoryRegion::new(5, 4096, perms);
        shm.attach();
        assert_eq!(shm.ref_count, 2);
        shm.detach();
        assert_eq!(shm.ref_count, 1);

        let mut fd = FileDescriptorHandle::new(3, 1, true);
        assert!(fd.close().is_ok());
        assert_eq!(fd.close(), Err(IPCError::EndpointClosed));
    }

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
