#![no_std]

/// Network Socket Implementation for SigmaOS
/// Based on 100-Improvement-Ideas.md networking concepts
/// Implements TCP/UDP socket abstraction for network communication

use core::sync::atomic::{AtomicU64, Ordering};

/// Socket ID type
pub type SocketID = u64;

/// Socket types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream = 0,    // TCP
    Datagram = 1,  // UDP
    Raw = 2,       // Raw socket
}

/// Socket states
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Closed = 0,
    Listening = 1,
    Connecting = 2,
    Connected = 3,
    Error = 4,
}

/// Socket error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SocketError {
    Success = 0,
    InvalidSocket = 1,
    ConnectionFailed = 2,
    BindFailed = 3,
    ListenFailed = 4,
    AcceptFailed = 5,
    SendFailed = 6,
    ReceiveFailed = 7,
}

/// Socket address
#[repr(C)]
pub struct SocketAddress {
    pub ip: [u8; 16],
    pub port: u16,
}

impl SocketAddress {
    pub fn new(ip: [u8; 16], port: u16) -> Self {
        SocketAddress { ip, port }
    }
    
    pub fn localhost(port: u16) -> Self {
        let mut ip = [0u8; 16];
        ip[15] = 1; // 127.0.0.1
        SocketAddress { ip, port }
    }
}

/// Socket configuration
#[repr(C)]
pub struct SocketConfig {
    pub socket_type: SocketType,
    pub non_blocking: bool,
    pub reuse_addr: bool,
    pub keep_alive: bool,
    pub send_buffer_size: u32,
    pub recv_buffer_size: u32,
}

impl SocketConfig {
    pub fn new(socket_type: SocketType) -> Self {
        SocketConfig {
            socket_type,
            non_blocking: false,
            reuse_addr: false,
            keep_alive: false,
            send_buffer_size: 8192,
            recv_buffer_size: 8192,
        }
    }
}

/// Network socket
pub struct Socket {
    pub id: SocketID,
    pub config: SocketConfig,
    pub state: SocketState,
    pub local_addr: SocketAddress,
    pub remote_addr: SocketAddress,
    pub bytes_sent: AtomicU64,
    pub bytes_received: AtomicU64,
}

impl Socket {
    pub fn new(id: SocketID, config: SocketConfig) -> Self {
        Socket {
            id,
            config,
            state: SocketState::Closed,
            local_addr: SocketAddress::localhost(0),
            remote_addr: SocketAddress::localhost(0),
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
        }
    }
    
    pub fn bind(&mut self, addr: SocketAddress) -> Result<(), SocketError> {
        if self.state != SocketState::Closed {
            return Err(SocketError::BindFailed);
        }
        
        self.local_addr = addr;
        Ok(())
    }
    
    pub fn listen(&mut self, backlog: u32) -> Result<(), SocketError> {
        if self.config.socket_type != SocketType::Stream {
            return Err(SocketError::ListenFailed);
        }
        
        if self.state != SocketState::Closed {
            return Err(SocketError::ListenFailed);
        }
        
        self.state = SocketState::Listening;
        Ok(())
    }
    
    pub fn connect(&mut self, addr: SocketAddress) -> Result<(), SocketError> {
        if self.config.socket_type != SocketType::Stream {
            return Err(SocketError::ConnectionFailed);
        }
        
        if self.state != SocketState::Closed {
            return Err(SocketError::ConnectionFailed);
        }
        
        self.remote_addr = addr;
        self.state = SocketState::Connected;
        Ok(())
    }
    
    pub fn send(&mut self, data: &[u8]) -> Result<usize, SocketError> {
        if self.state != SocketState::Connected {
            return Err(SocketError::SendFailed);
        }
        
        let sent = data.len();
        self.bytes_sent.fetch_add(sent as u64, Ordering::SeqCst);
        Ok(sent)
    }
    
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, SocketError> {
        if self.state != SocketState::Connected {
            return Err(SocketError::ReceiveFailed);
        }
        
        let received = buffer.len();
        self.bytes_received.fetch_add(received as u64, Ordering::SeqCst);
        Ok(received)
    }
    
    pub fn close(&mut self) {
        self.state = SocketState::Closed;
    }
    
    pub fn stats(&self) -> SocketStats {
        SocketStats {
            bytes_sent: self.bytes_sent.load(Ordering::SeqCst),
            bytes_received: self.bytes_received.load(Ordering::SeqCst),
        }
    }
}

/// Socket statistics
#[repr(C)]
pub struct SocketStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

/// Socket manager
pub struct SocketManager {
    sockets: Vec<Option<Socket>>,
    next_socket_id: AtomicU64,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            sockets: Vec::new(),
            next_socket_id: AtomicU64::new(1),
        }
    }
    
    pub fn create_socket(&mut self, config: SocketConfig) -> SocketID {
        let id = self.next_socket_id.fetch_add(1, Ordering::SeqCst);
        let socket = Socket::new(id, config);
        self.sockets.push(Some(socket));
        id
    }
    
    pub fn get_socket(&mut self, socket_id: SocketID) -> Option<&mut Socket> {
        for socket_option in &mut self.sockets {
            if let Some(ref mut socket) = *socket_option {
                if socket.id == socket_id {
                    return Some(socket);
                }
            }
        }
        None
    }
    
    pub fn close_socket(&mut self, socket_id: SocketID) -> Result<(), SocketError> {
        for socket_option in &mut self.sockets {
            if let Some(ref mut socket) = *socket_option {
                if socket.id == socket_id {
                    socket.close();
                    return Ok(());
                }
            }
        }
        Err(SocketError::InvalidSocket)
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
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

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
