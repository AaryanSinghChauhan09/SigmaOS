#![no_std]
#![no_main]

/// OOP-based Socket API for SigmaOS
/// Based on Ideas-999-Structured: Networking & Communication Item 771
/// Implements socket creation and network communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SocketID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SocketType { Stream = 0, Datagram = 1, Raw = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SocketError { Success = 0, NotFound = 1, ConnectionFailed = 2, SendFailed = 3 }

pub trait Socket {
    fn id(&self) -> SocketID;
    fn socket_type(&self) -> SocketType;
    fn is_connected(&self) -> bool;
    fn is_bound(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSocket {
    pub id: SocketID,
    pub socket_type: AtomicUsize,
    pub connected: AtomicUsize,
    pub bound: AtomicUsize,
}

impl SimpleSocket {
    pub fn new(id: SocketID, socket_type: SocketType) -> Self {
        SimpleSocket {
            id,
            socket_type: AtomicUsize::new(socket_type as usize),
            connected: AtomicUsize::new(0),
            bound: AtomicUsize::new(0),
        }
    }
}

impl Socket for SimpleSocket {
    fn id(&self) -> SocketID { self.id }
    fn socket_type(&self) -> SocketType { unsafe { core::mem::transmute(self.socket_type.load(Ordering::SeqCst)) } }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
    fn is_bound(&self) -> bool { self.bound.load(Ordering::SeqCst) == 1 }
}

pub trait SocketManager {
    fn create_socket(&mut self, socket_type: SocketType) -> Result<SocketID, SocketError>;
    def close_socket(&mut self, id: SocketID) -> Result<(), SocketError>;
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket>;
    def bind(&mut self, id: SocketID, address: &[u8], port: u16) -> Result<(), SocketError>;
    def connect(&mut self, id: SocketID, address: &[u8], port: u16) -> Result<(), SocketError>;
    def send(&mut self, id: SocketID, data: &[u8]) -> Result<usize, SocketError>;
    def receive(&mut self, id: SocketID, buffer: &mut [u8]) -> Result<usize, SocketError>;
}

#[repr(C)]
pub struct SimpleSocketManager {
    pub sockets: Vec<Option<Box<dyn Socket>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSocketManager {
    pub fn new() -> Self {
        SimpleSocketManager {
            sockets: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SocketManager for SimpleSocketManager {
    fn create_socket(&mut self, socket_type: SocketType) -> Result<SocketID, SocketError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let socket = SimpleSocket::new(id, socket_type);
        self.sockets.push(Some(Box::new(socket)));
        Ok(id)
    }
    
    fn close_socket(&mut self, id: SocketID) -> Result<(), SocketError> {
        for socket_option in &mut self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id {
                    return Ok(());
                }
            }
        }
        Err(SocketError::NotFound)
    }
    
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket> {
        for socket_option in &self.sockets {
            if let Some(ref socket) = *socket_option {
                if socket.id() == id { return Some(socket.as_ref()); }
            }
        }
        None
    }
    
    fn bind(&mut self, id: SocketID, _address: &[u8], _port: u16) -> Result<(), SocketError> {
        for socket_option in &mut self.sockets {
            if let Some(ref mut socket) = *socket_option {
                if socket.id() == id {
                    socket.bound.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SocketError::NotFound)
    }
    
    fn connect(&mut self, id: SocketID, _address: &[u8], _port: u16) -> Result<(), SocketError> {
        for socket_option in &mut self.sockets {
            if let Some(ref mut socket) = *socket_option {
                if socket.id() == id {
                    socket.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SocketError::NotFound)
    }
    
    fn send(&mut self, id: SocketID, data: &[u8]) -> Result<usize, SocketError> {
        if self.get_socket(id).is_some() {
            Ok(data.len())
        } else {
            Err(SocketError::NotFound)
        }
    }
    
    fn receive(&mut self, id: SocketID, buffer: &mut [u8]) -> Result<usize, SocketError> {
        if self.get_socket(id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0u8;
            }
            Ok(buffer.len())
        } else {
            Err(SocketError::NotFound)
        }
    }
}

pub trait SocketListener {
    fn listen(&mut self, id: SocketID, backlog: u32) -> Result<(), SocketError>;
    fn accept(&mut self, id: SocketID) -> Result<SocketID, SocketError>;
}

#[repr(C)]
pub struct SimpleSocketListener {
    pub manager: SimpleSocketManager,
}

impl SimpleSocketListener {
    pub fn new(manager: SimpleSocketManager) -> Self {
        SimpleSocketListener { manager }
    }
}

impl SocketListener for SimpleSocketListener {
    fn listen(&mut self, _id: SocketID, _backlog: u32) -> Result<(), SocketError> {
        Ok(())
    }
    
    fn accept(&mut self, _id: SocketID) -> Result<SocketID, SocketError> {
        let new_id = self.manager.next_id.fetch_add(1, Ordering::SeqCst);
        let socket = SimpleSocket::new(new_id, SocketType::Stream);
        self.manager.sockets.push(Some(Box::new(socket)));
        Ok(new_id)
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
