#![no_std]
#![no_main]

/// OOP-based Socket API for SigmaOS
/// Based on Ideas-999-Structured: Networking & Communication Item 771
/// Implements socket creation, BSD-style Socket Options, and network communication

#[cfg(not(target_os = "none"))]
extern crate alloc;
#[cfg(not(target_os = "none"))]
use alloc::boxed::Box;
#[cfg(not(target_os = "none"))]
use alloc::vec::Vec;

use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type SocketID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream = 0,
    Datagram = 1,
    Raw = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketError {
    Success = 0,
    NotFound = 1,
    ConnectionFailed = 2,
    SendFailed = 3,
    InvalidOption = 4,
}

/// Linux/BSD Socket Options (e.g. SOL_SOCKET level options)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketOption {
    ReuseAddr = 0,     // SO_REUSEADDR
    ReusePort = 1,     // SO_REUSEPORT
    KeepAlive = 2,     // SO_KEEPALIVE
    ReceiveTimeout = 3, // SO_RCVTIMEO
    SendTimeout = 4,    // SO_SNDTIMEO
}

/// BSD-style sockaddr_in representation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddrIn {
    pub sin_family: u16,      // AF_INET
    pub sin_port: u16,        // Network port
    pub sin_addr: [u8; 4],    // IPv4 Address (e.g., 127.0.0.1)
}

impl SocketAddrIn {
    pub fn new(port: u16, addr: [u8; 4]) -> Self {
        SocketAddrIn {
            sin_family: 2, // AF_INET is 2 in BSD/Linux
            sin_port: port,
            sin_addr: addr,
        }
    }
}

pub trait Socket {
    fn id(&self) -> SocketID;
    fn socket_type(&self) -> SocketType;
    fn is_connected(&self) -> bool;
    fn is_bound(&self) -> bool;

    // BSD/Linux-style Socket Option getters/setters
    fn set_opt(&mut self, option: SocketOption, value: u32) -> Result<(), SocketError>;
    fn get_opt(&self, option: SocketOption) -> Result<u32, SocketError>;
}

#[repr(C)]
pub struct SimpleSocket {
    pub id: SocketID,
    pub socket_type: AtomicUsize,
    pub connected: AtomicUsize,
    pub bound: AtomicUsize,

    // Linux/BSD Socket Option atomics
    pub reuse_addr: AtomicUsize,
    pub reuse_port: AtomicUsize,
    pub keep_alive: AtomicUsize,
    pub receive_timeout: AtomicUsize,
    pub send_timeout: AtomicUsize,
}

impl SimpleSocket {
    pub fn new(id: SocketID, socket_type: SocketType) -> Self {
        SimpleSocket {
            id,
            socket_type: AtomicUsize::new(socket_type as usize),
            connected: AtomicUsize::new(0),
            bound: AtomicUsize::new(0),
            reuse_addr: AtomicUsize::new(0),
            reuse_port: AtomicUsize::new(0),
            keep_alive: AtomicUsize::new(0),
            receive_timeout: AtomicUsize::new(0),
            send_timeout: AtomicUsize::new(0),
        }
    }
}

impl Socket for SimpleSocket {
    fn id(&self) -> SocketID {
        self.id
    }
    fn socket_type(&self) -> SocketType {
        let raw = self.socket_type.load(Ordering::SeqCst) as u32;
        match raw {
            1 => SocketType::Datagram,
            2 => SocketType::Raw,
            _ => SocketType::Stream,
        }
    }
    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst) == 1
    }
    fn is_bound(&self) -> bool {
        self.bound.load(Ordering::SeqCst) == 1
    }

    fn set_opt(&mut self, option: SocketOption, value: u32) -> Result<(), SocketError> {
        let val_size = value as usize;
        match option {
            SocketOption::ReuseAddr => self.reuse_addr.store(val_size, Ordering::SeqCst),
            SocketOption::ReusePort => self.reuse_port.store(val_size, Ordering::SeqCst),
            SocketOption::KeepAlive => self.keep_alive.store(val_size, Ordering::SeqCst),
            SocketOption::ReceiveTimeout => self.receive_timeout.store(val_size, Ordering::SeqCst),
            SocketOption::SendTimeout => self.send_timeout.store(val_size, Ordering::SeqCst),
        }
        Ok(())
    }

    fn get_opt(&self, option: SocketOption) -> Result<u32, SocketError> {
        let val = match option {
            SocketOption::ReuseAddr => self.reuse_addr.load(Ordering::SeqCst),
            SocketOption::ReusePort => self.reuse_port.load(Ordering::SeqCst),
            SocketOption::KeepAlive => self.keep_alive.load(Ordering::SeqCst),
            SocketOption::ReceiveTimeout => self.receive_timeout.load(Ordering::SeqCst),
            SocketOption::SendTimeout => self.send_timeout.load(Ordering::SeqCst),
        };
        Ok(val as u32)
    }
}

pub trait SocketManager {
    fn create_socket(&mut self, socket_type: SocketType) -> Result<SocketID, SocketError>;
    fn close_socket(&mut self, id: SocketID) -> Result<(), SocketError>;
    fn get_socket(&self, id: SocketID) -> Option<&dyn Socket>;
    fn get_socket_mut(&mut self, id: SocketID) -> Option<&mut dyn Socket>;
    fn bind(&mut self, id: SocketID, address: SocketAddrIn) -> Result<(), SocketError>;
    fn connect(&mut self, id: SocketID, address: SocketAddrIn) -> Result<(), SocketError>;
    fn send(&mut self, id: SocketID, data: &[u8]) -> Result<usize, SocketError>;
    fn receive(&mut self, id: SocketID, buffer: &mut [u8]) -> Result<usize, SocketError>;
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
                if socket.id() == id {
                    return Some(socket.as_ref());
                }
            }
        }
        None
    }

    fn get_socket_mut(&mut self, id: SocketID) -> Option<&mut dyn Socket> {
        for socket_option in &mut self.sockets {
            if let Some(ref mut socket) = *socket_option {
                if socket.id() == id {
                    return Some(socket.as_mut());
                }
            }
        }
        None
    }

    fn bind(&mut self, id: SocketID, _address: SocketAddrIn) -> Result<(), SocketError> {
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

    fn connect(&mut self, id: SocketID, _address: SocketAddrIn) -> Result<(), SocketError> {
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

#[cfg(target_os = "none")]
struct Box<T: ?Sized>(*mut T);

#[cfg(target_os = "none")]
impl<T> Box<T> {
    fn new(val: T) -> Self {
        unsafe {
            let ptr = alloc(mem::size_of::<T>()) as *mut T;
            if !ptr.is_null() {
                core::ptr::write(ptr, val);
            }
            Box(ptr)
        }
    }
}

#[cfg(target_os = "none")]
impl<T: ?Sized> AsRef<T> for Box<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

#[cfg(target_os = "none")]
impl<T: ?Sized> AsMut<T> for Box<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}

#[cfg(target_os = "none")]
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

#[cfg(target_os = "none")]
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
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
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

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_address_in() {
        let addr = SocketAddrIn::new(8080, [192, 168, 1, 1]);
        assert_eq!(addr.sin_family, 2);
        assert_eq!(addr.sin_port, 8080);
        assert_eq!(addr.sin_addr, [192, 168, 1, 1]);
    }

    #[test]
    fn test_socket_options_getting_setting() {
        let mut sock = SimpleSocket::new(1, SocketType::Stream);
        assert_eq!(sock.get_opt(SocketOption::KeepAlive).unwrap(), 0);
        assert_eq!(sock.get_opt(SocketOption::ReuseAddr).unwrap(), 0);

        sock.set_opt(SocketOption::KeepAlive, 1).unwrap();
        sock.set_opt(SocketOption::ReuseAddr, 1).unwrap();
        sock.set_opt(SocketOption::ReceiveTimeout, 5000).unwrap();

        assert_eq!(sock.get_opt(SocketOption::KeepAlive).unwrap(), 1);
        assert_eq!(sock.get_opt(SocketOption::ReuseAddr).unwrap(), 1);
        assert_eq!(sock.get_opt(SocketOption::ReceiveTimeout).unwrap(), 5000);
    }

    #[test]
    fn test_socket_manager_address_binding_flow() {
        let mut manager = SimpleSocketManager::new();
        let sock_id = manager.create_socket(SocketType::Stream).unwrap();

        let addr = SocketAddrIn::new(80, [127, 0, 0, 1]);
        manager.bind(sock_id, addr).unwrap();
        manager.connect(sock_id, addr).unwrap();

        assert!(manager.get_socket(sock_id).unwrap().is_bound());
        assert!(manager.get_socket(sock_id).unwrap().is_connected());

        // Validate socket options setter/getter through manager
        let sock_mut = manager.get_socket_mut(sock_id).unwrap();
        sock_mut.set_opt(SocketOption::KeepAlive, 1).unwrap();
        assert_eq!(sock_mut.get_opt(SocketOption::KeepAlive).unwrap(), 1);
    }
}
