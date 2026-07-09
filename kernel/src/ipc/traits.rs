// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// OOP-based IPC traits for SigmaOS
// Zero-allocation, performance-optimized inter-process communication

/// Core IPC trait for all communication mechanisms
pub trait IpcEndpoint {
    /// Initialize the IPC endpoint
    fn init(&mut self) -> Result<(), IpcError>;
    
    /// Get endpoint name
    fn name(&self) -> &str;
    
    /// Get endpoint type
    fn endpoint_type(&self) -> IpcType;
    
    /// Check if endpoint is ready
    fn is_ready(&self) -> bool;
    
    /// Close the endpoint
    fn close(&mut self) -> Result<(), IpcError>;
}

/// Message-based IPC trait
pub trait MessageBased: IpcEndpoint {
    /// Send message
    fn send(&mut self, message: &[u8]) -> Result<usize, IpcError>;
    
    /// Receive message
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IpcError>;
    
    /// Send message with timeout
    fn send_timeout(&mut self, message: &[u8], timeout_ms: u64) -> Result<usize, IpcError>;
    
    /// Receive message with timeout
    fn receive_timeout(&mut self, buffer: &mut [u8], timeout_ms: u64) -> Result<usize, IpcError>;
    
    /// Get maximum message size
    fn max_message_size(&self) -> usize;
}

/// Shared memory IPC trait
pub trait SharedMemory: IpcEndpoint {
    /// Create shared memory region
    fn create(&mut self, size: usize) -> Result<(), IpcError>;
    
    /// Attach to existing shared memory region
    fn attach(&mut self, id: u64) -> Result<(), IpcError>;
    
    /// Detach from shared memory region
    fn detach(&mut self) -> Result<(), IpcError>;
    
    /// Get pointer to shared memory
    fn data(&self) -> *mut u8;
    
    /// Get shared memory size
    fn size(&self) -> usize;
    
    /// Get shared memory ID
    fn id(&self) -> u64;
}

/// Semaphore IPC trait
pub trait Semaphore: IpcEndpoint {
    /// Wait on semaphore (decrement)
    fn wait(&mut self) -> Result<(), IpcError>;
    
    /// Signal semaphore (increment)
    fn signal(&mut self) -> Result<(), IpcError>;
    
    /// Try to wait without blocking
    fn try_wait(&mut self) -> Result<bool, IpcError>;
    
    /// Get current semaphore value
    fn value(&self) -> i32;
    
    /// Set semaphore value
    fn set_value(&mut self, value: i32) -> Result<(), IpcError>;
}

/// Mutex IPC trait
pub trait Mutex: IpcEndpoint {
    /// Lock mutex
    fn lock(&mut self) -> Result<(), IpcError>;
    
    /// Try to lock without blocking
    fn try_lock(&mut self) -> Result<bool, IpcError>;
    
    /// Unlock mutex
    fn unlock(&mut self) -> Result<(), IpcError>;
    
    /// Check if mutex is locked
    fn is_locked(&self) -> bool;
}

/// Pipe IPC trait
pub trait Pipe: IpcEndpoint {
    /// Write to pipe
    fn write(&mut self, data: &[u8]) -> Result<usize, IpcError>;
    
    /// Read from pipe
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, IpcError>;
    
    /// Get pipe size
    fn pipe_size(&self) -> usize;
    
    /// Set pipe size
    fn set_pipe_size(&mut self, size: usize) -> Result<(), IpcError>;
    
    /// Check if pipe is readable
    fn can_read(&self) -> bool;
    
    /// Check if pipe is writable
    fn can_write(&self) -> bool;
}

/// Socket IPC trait
pub trait Socket: IpcEndpoint {
    /// Connect to remote endpoint
    fn connect(&mut self, address: SocketAddress) -> Result<(), IpcError>;
    
    /// Bind to local address
    fn bind(&mut self, address: SocketAddress) -> Result<(), IpcError>;
    
    /// Listen for connections
    fn listen(&mut self, backlog: u32) -> Result<(), IpcError>;
    
    /// Accept incoming connection
    fn accept(&mut self) -> Result<Box<dyn Socket>, IpcError>;
    
    /// Send data
    fn send(&mut self, data: &[u8]) -> Result<usize, IpcError>;
    
    /// Receive data
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, IpcError>;
    
    /// Get socket address
    fn local_address(&self) -> SocketAddress;
    
    /// Get peer address
    fn peer_address(&self) -> Option<SocketAddress>;
}

/// Event IPC trait
pub trait Event: IpcEndpoint {
    /// Signal event
    fn signal(&mut self) -> Result<(), IpcError>;
    
    /// Wait for event
    fn wait(&mut self) -> Result<(), IpcError>;
    
    /// Wait for event with timeout
    fn wait_timeout(&mut self, timeout_ms: u64) -> Result<bool, IpcError>;
    
    /// Reset event
    fn reset(&mut self) -> Result<(), IpcError>;
    
    /// Check if event is signaled
    fn is_signaled(&self) -> bool;
}

/// File descriptor IPC trait
pub trait FileDescriptor: IpcEndpoint {
    /// Read from file descriptor
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, IpcError>;
    
    /// Write to file descriptor
    fn write(&mut self, data: &[u8]) -> Result<usize, IpcError>;
    
    /// Seek to position
    fn seek(&mut self, offset: i64, whence: SeekWhence) -> Result<u64, IpcError>;
    
    /// Get file descriptor flags
    fn flags(&self) -> u32;
    
    /// Set file descriptor flags
    fn set_flags(&mut self, flags: u32) -> Result<(), IpcError>;
    
    /// Get file descriptor number
    fn fd(&self) -> i32;
}

/// IPC registry for managing endpoints
pub trait IpcRegistry {
    /// Register IPC endpoint
    fn register(&mut self, endpoint: Box<dyn IpcEndpoint>) -> Result<u64, IpcError>;
    
    /// Unregister IPC endpoint
    fn unregister(&mut self, id: u64) -> Result<(), IpcError>;
    
    /// Get endpoint by ID
    fn get(&self, id: u64) -> Option<&dyn IpcEndpoint>;
    
    /// Get mutable endpoint by ID
    fn get_mut(&mut self, id: u64) -> Option<&mut dyn IpcEndpoint>;
    
    /// List all endpoints
    fn list(&self) -> Vec<u64>;
    
    /// Find endpoint by name
    fn find_by_name(&self, name: &str) -> Option<u64>;
}

/// Error types for IPC operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    NotFound,
    PermissionDenied,
    InvalidParameter,
    Timeout,
    WouldBlock,
    NotConnected,
    ConnectionRefused,
    ConnectionReset,
    BufferTooSmall,
    NoMemory,
    InvalidState,
    Interrupted,
    Other,
}

/// IPC types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcType {
    MessageQueue,
    SharedMemory,
    Semaphore,
    Mutex,
    Pipe,
    Socket,
    Event,
    FileDescriptor,
    Other,
}

/// Socket address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddress {
    pub family: AddressFamily,
    pub path: [u8; 108], // Unix domain socket path
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    Unix,
    Inet,
    Inet6,
    Other,
}

/// Seek whence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekWhence {
    Set,
    Current,
    End,
}

/// IPC message header for structured communication
#[derive(Debug, Clone, Copy)]
pub struct IpcMessageHeader {
    pub message_id: u64,
    pub sender_id: u64,
    pub receiver_id: u64,
    pub message_type: u32,
    pub flags: u32,
    pub timestamp: u64,
    pub payload_size: usize,
}

impl IpcMessageHeader {
    pub const fn new() -> Self {
        Self {
            message_id: 0,
            sender_id: 0,
            receiver_id: 0,
            message_type: 0,
            flags: 0,
            timestamp: 0,
            payload_size: 0,
        }
    }
}

/// IPC message flags
pub mod flags {
    pub const NONE: u32 = 0;
    pub const URGENT: u32 = 1 << 0;
    pub const BROADCAST: u32 = 1 << 1;
    pub const REPLY_EXPECTED: u32 = 1 << 2;
    pub const NO_REPLY: u32 = 1 << 3;
    pub const PRIORITY_HIGH: u32 = 1 << 4;
    pub const PRIORITY_LOW: u32 = 1 << 5;
}
