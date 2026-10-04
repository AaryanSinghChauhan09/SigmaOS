//! # Unix Domain Sockets
//!
//! AF_UNIX socket implementation providing inter-process communication
//! through filesystem paths. Inspired by Linux unix_sock.c and FreeBSD uipc_usrreq.c.

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Unix socket type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UnixSocketType {
    Stream = 1,    // SOCK_STREAM - connection-oriented byte streams
    Datagram = 2,  // SOCK_DGRAM - connectionless datagrams
    SeqPacket = 5, // SOCK_SEQPACKET - connection-oriented packets
}

/// Unix socket state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UnixSocketState {
    Unbound = 0,
    Bound = 1,
    Listening = 2,
    Connecting = 3,
    Connected = 4,
    Disconnecting = 5,
    Disconnected = 6,
}

/// Unix socket address
#[derive(Debug, Clone)]
pub enum UnixSocketAddr {
    Unnamed,           // Anonymous socket
    Path(String),      // Filesystem path
    Abstract(Vec<u8>), // Abstract namespace (Linux)
}

pub type UnixSocketAddress = UnixSocketAddr;

#[derive(Debug, Default)]
pub struct UnixSocketManager;

/// Message with ancillary data
#[derive(Debug, Clone)]
pub struct UnixMessage {
    pub data: Vec<u8>,
    pub control_data: Vec<u8>, // Ancillary data (SCM_RIGHTS, SCM_CREDENTIALS)
    pub flags: u32,
}

/// Unix domain socket control block
pub struct UnixSocket {
    socket_type: UnixSocketType,
    state: AtomicU32,
    addr: Option<UnixSocketAddr>,

    // Connection state
    peer: Option<Arc<UnixSocket>>,
    backlog: VecDeque<Arc<UnixSocket>>,
    max_backlog: usize,

    // Data buffers
    send_buffer: VecDeque<UnixMessage>,
    recv_buffer: VecDeque<UnixMessage>,
    buffer_capacity: usize,

    // Flags
    non_blocking: AtomicBool,
    pass_cred: AtomicBool,

    // Credentials
    uid: u32,
    gid: u32,
    pid: u32,
}

impl UnixSocket {
    /// Create new Unix socket
    pub fn new(socket_type: UnixSocketType) -> Self {
        Self {
            socket_type,
            state: AtomicU32::new(UnixSocketState::Unbound as u32),
            addr: None,
            peer: None,
            backlog: VecDeque::new(),
            max_backlog: 128,
            send_buffer: VecDeque::with_capacity(64),
            recv_buffer: VecDeque::with_capacity(64),
            buffer_capacity: 212992, // 208 KB default (from Linux)
            non_blocking: AtomicBool::new(false),
            pass_cred: AtomicBool::new(false),
            uid: 0,
            gid: 0,
            pid: 0,
        }
    }

    /// Bind socket to address
    pub fn bind(&mut self, addr: UnixSocketAddr) -> Result<(), UnixSocketError> {
        let state = self.state.load(Ordering::Acquire);
        if state != UnixSocketState::Unbound as u32 {
            return Err(UnixSocketError::AlreadyBound);
        }

        // Register address in namespace
        match &addr {
            UnixSocketAddr::Path(path) => {
                // In production: check filesystem, create socket inode
                if path.is_empty() || path.len() > 108 {
                    return Err(UnixSocketError::InvalidAddress);
                }
            }
            UnixSocketAddr::Abstract(name) => {
                if name.len() > 107 {
                    return Err(UnixSocketError::InvalidAddress);
                }
            }
            UnixSocketAddr::Unnamed => {
                return Err(UnixSocketError::InvalidAddress);
            }
        }

        self.addr = Some(addr);
        self.state
            .store(UnixSocketState::Bound as u32, Ordering::Release);
        Ok(())
    }

    /// Listen for connections (SOCK_STREAM only)
    pub fn listen(&mut self, backlog: usize) -> Result<(), UnixSocketError> {
        if self.socket_type != UnixSocketType::Stream {
            return Err(UnixSocketError::NotSupported);
        }

        let state = self.state.load(Ordering::Acquire);
        if state != UnixSocketState::Bound as u32 {
            return Err(UnixSocketError::NotBound);
        }

        self.max_backlog = backlog.min(4096); // Clamp to reasonable limit
        self.state
            .store(UnixSocketState::Listening as u32, Ordering::Release);
        Ok(())
    }

    /// Accept incoming connection
    pub fn accept(&mut self) -> Result<Arc<UnixSocket>, UnixSocketError> {
        let state = self.state.load(Ordering::Acquire);
        if state != UnixSocketState::Listening as u32 {
            return Err(UnixSocketError::NotListening);
        }

        // Pop connection from backlog
        match self.backlog.pop_front() {
            Some(peer) => {
                peer.state
                    .store(UnixSocketState::Connected as u32, Ordering::Release);
                Ok(peer)
            }
            None => {
                if self.non_blocking.load(Ordering::Acquire) {
                    Err(UnixSocketError::WouldBlock)
                } else {
                    // In production: block on wait queue
                    Err(UnixSocketError::WouldBlock)
                }
            }
        }
    }

    /// Connect to listening socket
    pub fn connect(&mut self, addr: &UnixSocketAddr) -> Result<(), UnixSocketError> {
        if self.socket_type != UnixSocketType::Stream {
            return Err(UnixSocketError::NotSupported);
        }

        let state = self.state.load(Ordering::Acquire);
        if state == UnixSocketState::Connected as u32 {
            return Err(UnixSocketError::AlreadyConnected);
        }

        // In production: lookup addr in socket registry, add to backlog
        self.state
            .store(UnixSocketState::Connecting as u32, Ordering::Release);

        // Simplified: immediately transition to connected
        // Real implementation waits for accept()
        self.state
            .store(UnixSocketState::Connected as u32, Ordering::Release);
        Ok(())
    }

    /// Send data
    pub fn send(&mut self, data: &[u8], flags: u32) -> Result<usize, UnixSocketError> {
        let state = self.state.load(Ordering::Acquire);

        match self.socket_type {
            UnixSocketType::Stream | UnixSocketType::SeqPacket => {
                if state != UnixSocketState::Connected as u32 {
                    return Err(UnixSocketError::NotConnected);
                }
            }
            UnixSocketType::Datagram => {
                if state == UnixSocketState::Unbound as u32 {
                    return Err(UnixSocketError::NotBound);
                }
            }
        }

        // Check buffer capacity
        let current_size: usize = self.send_buffer.iter().map(|msg| msg.data.len()).sum();

        if current_size + data.len() > self.buffer_capacity {
            if self.non_blocking.load(Ordering::Acquire) {
                return Err(UnixSocketError::WouldBlock);
            } else {
                // In production: block on wait queue
                return Err(UnixSocketError::WouldBlock);
            }
        }

        // Enqueue message
        let msg = UnixMessage {
            data: data.to_vec(),
            control_data: Vec::new(),
            flags,
        };

        let len = msg.data.len();
        self.send_buffer.push_back(msg);

        // In production: wake peer's receive wait queue
        Ok(len)
    }

    /// Send message with ancillary data (file descriptors, credentials)
    pub fn sendmsg(&mut self, msg: UnixMessage) -> Result<usize, UnixSocketError> {
        let state = self.state.load(Ordering::Acquire);
        if state != UnixSocketState::Connected as u32
            && self.socket_type != UnixSocketType::Datagram
        {
            return Err(UnixSocketError::NotConnected);
        }

        let len = msg.data.len();
        self.send_buffer.push_back(msg);
        Ok(len)
    }

    /// Receive data
    pub fn recv(&mut self, buf: &mut [u8], flags: u32) -> Result<usize, UnixSocketError> {
        let state = self.state.load(Ordering::Acquire);

        if self.socket_type == UnixSocketType::Stream
            || self.socket_type == UnixSocketType::SeqPacket
        {
            if state != UnixSocketState::Connected as u32 {
                return Err(UnixSocketError::NotConnected);
            }
        }

        // Dequeue message
        match self.recv_buffer.pop_front() {
            Some(msg) => {
                let copy_len = msg.data.len().min(buf.len());
                buf[..copy_len].copy_from_slice(&msg.data[..copy_len]);

                // MSG_TRUNC: return actual message size even if truncated
                if flags & 0x20 != 0 {
                    Ok(msg.data.len())
                } else {
                    Ok(copy_len)
                }
            }
            None => {
                if self.non_blocking.load(Ordering::Acquire) {
                    Err(UnixSocketError::WouldBlock)
                } else {
                    // In production: block on wait queue
                    Err(UnixSocketError::WouldBlock)
                }
            }
        }
    }

    /// Receive message with ancillary data
    pub fn recvmsg(&mut self, buf: &mut [u8]) -> Result<(usize, Vec<u8>), UnixSocketError> {
        match self.recv_buffer.pop_front() {
            Some(msg) => {
                let copy_len = msg.data.len().min(buf.len());
                buf[..copy_len].copy_from_slice(&msg.data[..copy_len]);
                Ok((copy_len, msg.control_data))
            }
            None => {
                if self.non_blocking.load(Ordering::Acquire) {
                    Err(UnixSocketError::WouldBlock)
                } else {
                    Err(UnixSocketError::WouldBlock)
                }
            }
        }
    }

    /// Shutdown socket
    pub fn shutdown(&mut self, how: ShutdownHow) -> Result<(), UnixSocketError> {
        let state = self.state.load(Ordering::Acquire);
        if state != UnixSocketState::Connected as u32 {
            return Err(UnixSocketError::NotConnected);
        }

        match how {
            ShutdownHow::Read => {
                // Clear recv buffer
                self.recv_buffer.clear();
            }
            ShutdownHow::Write => {
                // Flush send buffer, prevent further writes
                // In production: send remaining data and signal EOF
            }
            ShutdownHow::Both => {
                self.recv_buffer.clear();
                self.send_buffer.clear();
            }
        }

        self.state
            .store(UnixSocketState::Disconnecting as u32, Ordering::Release);
        Ok(())
    }

    /// Set socket option
    pub fn setsockopt(
        &mut self,
        level: i32,
        optname: i32,
        optval: &[u8],
    ) -> Result<(), UnixSocketError> {
        match (level, optname) {
            (1, 9) => {
                // SOL_SOCKET, SO_PASSCRED
                if optval.len() >= 4 {
                    let val = u32::from_ne_bytes([optval[0], optval[1], optval[2], optval[3]]);
                    self.pass_cred.store(val != 0, Ordering::Release);
                    Ok(())
                } else {
                    Err(UnixSocketError::InvalidOption)
                }
            }
            _ => Err(UnixSocketError::NotSupported),
        }
    }

    /// Get socket state
    pub fn get_state(&self) -> UnixSocketState {
        let state_val = self.state.load(Ordering::Acquire);
        unsafe { core::mem::transmute(state_val as u8) }
    }

    /// Get socket address
    pub fn get_addr(&self) -> Option<&UnixSocketAddr> {
        self.addr.as_ref()
    }
}

/// Shutdown direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownHow {
    Read = 0,  // SHUT_RD
    Write = 1, // SHUT_WR
    Both = 2,  // SHUT_RDWR
}

/// Unix socket errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixSocketError {
    AlreadyBound,
    AlreadyConnected,
    NotBound,
    NotListening,
    NotConnected,
    NotSupported,
    InvalidAddress,
    InvalidOption,
    WouldBlock,
    BufferFull,
    Disconnected,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_socket_creation() {
        let sock = UnixSocket::new(UnixSocketType::Stream);
        assert_eq!(sock.get_state(), UnixSocketState::Unbound);
    }

    #[test]
    fn test_unix_socket_bind() {
        let mut sock = UnixSocket::new(UnixSocketType::Stream);
        let addr = UnixSocketAddr::Path("/tmp/test.sock".into());
        assert!(sock.bind(addr).is_ok());
        assert_eq!(sock.get_state(), UnixSocketState::Bound);
    }

    #[test]
    fn test_unix_socket_listen() {
        let mut sock = UnixSocket::new(UnixSocketType::Stream);
        let addr = UnixSocketAddr::Path("/tmp/test.sock".into());
        sock.bind(addr).unwrap();
        assert!(sock.listen(128).is_ok());
        assert_eq!(sock.get_state(), UnixSocketState::Listening);
    }

    #[test]
    fn test_datagram_socket() {
        let mut sock = UnixSocket::new(UnixSocketType::Datagram);
        let addr = UnixSocketAddr::Abstract(b"test".to_vec());
        assert!(sock.bind(addr).is_ok());
    }

    #[test]
    fn test_invalid_path_length() {
        let mut sock = UnixSocket::new(UnixSocketType::Stream);
        let long_path = "a".repeat(109);
        let addr = UnixSocketAddr::Path(long_path);
        assert_eq!(sock.bind(addr), Err(UnixSocketError::InvalidAddress));
    }
}
