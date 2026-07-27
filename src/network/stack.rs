#![no_std]
#![no_main]

/// OOP-based Network Stack for SigmaOS
/// Implements networking using OOP principles with traits and structs
/// No dependency on external network frameworks
/// Based on Roadmap Item 6: Network stack

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Socket ID
pub type SocketID = usize;

/// Socket type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SocketType {
    TCP = 0,
    UDP = 1,
    Raw = 2,
}

/// Socket state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SocketState {
    Closed = 0,
    Listening = 1,
    Connecting = 2,
    Connected = 3,
    Closing = 4,
}

/// Socket trait (OOP interface)
pub trait Socket {
    /// Get socket ID
    fn id(&self) -> SocketID;
    /// Get socket type
    fn socket_type(&self) -> SocketType;
    /// Bind socket
    fn bind(&mut self, port: u16) -> Result<(), NetworkError>;
    /// Listen on socket
    fn listen(&mut self) -> Result<(), NetworkError>;
    /// Connect socket
    fn connect(&mut self, addr: [u8; 4], port: u16) -> Result<(), NetworkError>;
    /// Send data
    fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError>;
    /// Receive data
    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
    /// Close socket
    fn close(&mut self) -> Result<(), NetworkError>;
    /// Get socket state
    fn state(&self) -> SocketState;
    /// Get socket info
    fn info(&self) -> SocketInfo;
}

/// Network error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NetworkError {
    Success = 0,
    InvalidPort = 1,
    ConnectionFailed = 2,
    SendFailed = 3,
    ReceiveFailed = 4,
    PermissionDenied = 5,
}

/// Socket info
#[repr(C)]
pub struct SocketInfo {
    pub id: SocketID,
    pub socket_type: SocketType,
    pub state: SocketState,
    pub local_port: u16,
    pub remote_addr: [u8; 4],
    pub remote_port: u16,
    pub capability: SocketCapability,
}

impl SocketInfo {
    pub fn new(id: SocketID, socket_type: SocketType) -> Self {
        SocketInfo {
            id,
            socket_type,
            state: SocketState::Closed,
            local_port: 0,
            remote_addr: [0; 4],
            remote_port: 0,
            capability: SocketCapability::new(),
        }
    }
}

/// Socket capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SocketCapability {
    pub can_send: bool,
    pub can_receive: bool,
}

impl SocketCapability {
    pub fn new() -> Self {
        SocketCapability {
            can_send: false,
            can_receive: false,
        }
    }

    pub fn full() -> Self {
        SocketCapability {
            can_send: true,
            can_receive: true,
        }
    }
}

/// Simple socket (OOP: Concrete socket class)
#[repr(C)]
pub struct SimpleSocket {
    pub id: SocketID,
    pub socket_type: SocketType,
    pub state: AtomicUsize, // SocketState as usize
    pub local_port: u16,
    pub remote_addr: [u8; 4],
    pub remote_port: u16,
    pub capability: SocketCapability,
    pub send_buffer: [u8; 4096],
    pub send_buffer_size: AtomicUsize,
}

impl SimpleSocket {
    pub fn new(id: SocketID, socket_type: SocketType, capability: SocketCapability) -> Self {
        SimpleSocket {
            id,
            socket_type,
            state: AtomicUsize::new(SocketState::Closed as usize),
            local_port: 0,
            remote_addr: [0; 4],
            remote_port: 0,
            capability,
            send_buffer: [0; 4096],
            send_buffer_size: AtomicUsize::new(0),
        }
    }

    pub fn get_state(&self) -> SocketState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: SocketState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Socket for SimpleSocket {
    fn id(&self) -> SocketID {
        self.id
    }

    fn socket_type(&self) -> SocketType {
        self.socket_type
    }

    fn bind(&mut self, port: u16) -> Result<(), NetworkError> {
        if port == 0 {
            return Err(NetworkError::InvalidPort);
        }

        self.local_port = port;
        Ok(())
    }

    fn listen(&mut self) -> Result<(), NetworkError> {
        self.set_state(SocketState::Listening);
        Ok(())
    }

    fn connect(&mut self, addr: [u8; 4], port: u16) -> Result<(), NetworkError> {
        self.set_state(SocketState::Connecting);
        self.remote_addr = addr;
        self.remote_port = port;
        self.set_state(SocketState::Connected);
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        if !self.capability.can_send {
            return Err(NetworkError::PermissionDenied);
        }

        let bytes_to_send = data.len().min(4096);

        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.send_buffer.as_mut_ptr(), bytes_to_send);
        }

        self.send_buffer_size.store(bytes_to_send, Ordering::SeqCst);
        Ok(bytes_to_send)
    }

    fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        if !self.capability.can_receive {
            return Err(NetworkError::PermissionDenied);
        }

        let buffer_size = buffer.len();
        let bytes_to_receive = buffer_size.min(1024);

        // Simulate receiving data
        for i in 0..bytes_to_receive {
            buffer[i] = i as u8;
        }

        Ok(bytes_to_receive)
    }

    fn close(&mut self) -> Result<(), NetworkError> {
        self.set_state(SocketState::Closing);
        self.set_state(SocketState::Closed);
        Ok(())
    }

    fn state(&self) -> SocketState {
        self.get_state()
    }

    fn info(&self) -> SocketInfo {
        SocketInfo {
            id: self.id,
            socket_type: self.socket_type,
            state: self.get_state(),
            local_port: self.local_port,
            remote_addr: self.remote_addr,
            remote_port: self.remote_port,
            capability: self.capability,
        }
    }
}

/// Network stack trait (OOP interface)
pub trait NetworkStack {
    /// Create socket
    fn create_socket(&mut self, socket_type: SocketType) -> Result<SocketID, NetworkError>;
    /// Destroy socket
    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError>;
    /// Get socket
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket>;
    /// List sockets
    fn list_sockets(&self) -> Vec<SocketID>;
    /// Get stack statistics
    fn stats(&self) -> NetworkStats;
}

/// Network statistics
#[repr(C)]
pub struct NetworkStats {
    pub total_sockets: usize,
    pub active_sockets: usize,
    pub by_type: [usize; 3],
}

impl NetworkStats {
    pub fn new() -> Self {
        NetworkStats {
            total_sockets: 0,
            active_sockets: 0,
            by_type: [0; 3],
        }
    }
}

/// Simple network stack (OOP: Concrete stack class)
pub struct SimpleNetworkStack {
    sockets: Vec<Option<Box<dyn Socket>>>,
    next_id: AtomicUsize,
    stats: NetworkStats,
    capability: StackCapability,
}

/// Stack capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StackCapability {
    pub can_create: bool,
    pub can_destroy: bool,
}

impl StackCapability {
    pub fn new() -> Self {
        StackCapability {
            can_create: false,
            can_destroy: false,
        }
    }

    pub fn full() -> Self {
        StackCapability {
            can_create: true,
            can_destroy: true,
        }
    }
}

impl SimpleNetworkStack {
    pub fn new(capability: StackCapability) -> Self {
        SimpleNetworkStack {
            sockets: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: NetworkStats::new(),
            capability,
        }
    }
}

impl NetworkStack for SimpleNetworkStack {
    fn create_socket(&mut self, socket_type: SocketType) -> Result<SocketID, NetworkError> {
        if !self.capability.can_create {
            return Err(NetworkError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let socket = SimpleSocket::new(id, socket_type, SocketCapability::full());
        self.sockets.push(Some(Box::new(socket)));
        self.stats.total_sockets += 1;
        self.stats.active_sockets += 1;
        self.stats.by_type[socket_type as usize] += 1;
        Ok(id)
    }

    fn destroy_socket(&mut self, id: SocketID) -> Result<(), NetworkError> {
        if !self.capability.can_destroy {
            return Err(NetworkError::PermissionDenied);
        }

        let mut index = None;
        let mut socket_type = SocketType::TCP;

        for (i, socket_option) in self.sockets.iter().enumerate() {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
                    index = Some(i);
                    socket_type = socket.socket_type();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.sockets[i] = None;
            self.stats.total_sockets -= 1;
            self.stats.active_sockets -= 1;
            self.stats.by_type[socket_type as usize] -= 1;
            Ok(())
        } else {
            Err(NetworkError::PermissionDenied)
        }
    }

    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket> {
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
                    return Some(socket.as_ref());
                }
            }
        }
        None
    }

    fn list_sockets(&self) -> Vec<SocketID> {
        let mut ids = Vec::new();
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
                ids.push(socket.id());
            }
        }
        ids
    }

    fn stats(&self) -> NetworkStats {
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
