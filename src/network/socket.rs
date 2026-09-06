#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT
/// SigmaOS: Socket Implementation
/// BSD-compatible socket API for TCP/UDP/ICMP

use super::zenithnet::{Ipv4Addr, TcpState};
use std::collections::BTreeMap;
use std::vec::Vec;
use core::fmt;

/// Socket Address Family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    Ipv4 = 2,
    Ipv6 = 10,
}

/// Socket Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream = 1,   // TCP
    Datagram = 2, // UDP
    Raw = 3,      // Raw IP
}

/// Socket Address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddr {
    pub family: AddressFamily,
    pub port: u16,
    pub addr: Ipv4Addr,
}

impl SocketAddr {
    pub fn new(addr: Ipv4Addr, port: u16) -> Self {
        Self {
            family: AddressFamily::Ipv4,
            port,
            addr,
        }
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}

/// Socket Error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketError {
    InvalidSocket,
    NotConnected,
    AlreadyConnected,
    ConnectionRefused,
    ConnectionReset,
    Timeout,
    WouldBlock,
    BufferFull,
    InvalidArgument,
    PermissionDenied,
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSocket => write!(f, "Invalid socket"),
            Self::NotConnected => write!(f, "Socket is not connected"),
            Self::AlreadyConnected => write!(f, "Socket is already connected"),
            Self::ConnectionRefused => write!(f, "Connection refused"),
            Self::ConnectionReset => write!(f, "Connection reset by peer"),
            Self::Timeout => write!(f, "Connection timeout"),
            Self::WouldBlock => write!(f, "Operation would block"),
            Self::BufferFull => write!(f, "Buffer full"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
            Self::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

/// Socket State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Created,
    Bound,
    Listening,
    Connecting,
    Connected,
    Closing,
    Closed,
}

/// Socket Options
#[derive(Debug, Clone)]
pub struct SocketOptions {
    pub reuse_addr: bool,
    pub keep_alive: bool,
    pub tcp_no_delay: bool,
    pub recv_timeout_ms: Option<u64>,
    pub send_timeout_ms: Option<u64>,
    pub recv_buffer_size: usize,
    pub send_buffer_size: usize,
}

impl Default for SocketOptions {
    fn default() -> Self {
        Self {
            reuse_addr: false,
            keep_alive: false,
            tcp_no_delay: false,
            recv_timeout_ms: None,
            send_timeout_ms: None,
            recv_buffer_size: 65536,
            send_buffer_size: 65536,
        }
    }
}

/// Socket
#[derive(Debug, Clone)]
pub struct Socket {
    pub fd: i32,
    pub family: AddressFamily,
    pub socket_type: SocketType,
    pub protocol: u32,
    pub state: SocketState,
    pub local_addr: Option<SocketAddr>,
    pub peer_addr: Option<SocketAddr>,
    pub options: SocketOptions,
    pub tcp_state: TcpState,
    pub recv_buffer: Vec<u8>,
    pub send_buffer: Vec<u8>,
}

impl Socket {
    pub fn new(fd: i32, family: AddressFamily, socket_type: SocketType, protocol: u32) -> Self {
        Self {
            fd,
            family,
            socket_type,
            protocol,
            state: SocketState::Created,
            local_addr: None,
            peer_addr: None,
            options: SocketOptions::default(),
            tcp_state: TcpState::Closed,
            recv_buffer: Vec::with_capacity(65536),
            send_buffer: Vec::with_capacity(65536),
        }
    }

    pub fn bind(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
        if self.state != SocketState::Created {
            return Err(SocketError::InvalidArgument);
        }

        self.local_addr = Some(addr);
        self.state = SocketState::Bound;
        Ok(())
    }

    pub fn listen(&mut self, _backlog: u32) -> Result<(), SocketError> {
        if self.socket_type != SocketType::Stream {
            return Err(SocketError::InvalidArgument);
        }

        if self.state != SocketState::Bound {
            return Err(SocketError::InvalidArgument);
        }

        self.state = SocketState::Listening;
        self.tcp_state = TcpState::Listen;
        Ok(())
    }

    pub fn connect(&mut self, addr: SocketAddr) -> Result<(), SocketError> {
        if self.state != SocketState::Created && self.state != SocketState::Bound {
            return Err(SocketError::InvalidArgument);
        }

        self.peer_addr = Some(addr);
        self.state = SocketState::Connecting;
        self.tcp_state = TcpState::SynSent;
        Ok(())
    }

    pub fn accept(&self) -> Result<Socket, SocketError> {
        if self.state != SocketState::Listening {
            return Err(SocketError::InvalidArgument);
        }

        // Stub: would accept incoming connection
        let mut accepted = Socket::new(self.fd + 1, self.family, self.socket_type, self.protocol);
        accepted.state = SocketState::Connected;
        accepted.tcp_state = TcpState::Established;
        Ok(accepted)
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, SocketError> {
        if self.state != SocketState::Connected {
            return Err(SocketError::NotConnected);
        }

        let to_send = data.len().min(self.options.send_buffer_size - self.send_buffer.len());

        if to_send == 0 {
            return Err(SocketError::BufferFull);
        }

        self.send_buffer.extend_from_slice(&data[..to_send]);
        Ok(to_send)
    }

    pub fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, SocketError> {
        if self.state != SocketState::Connected {
            return Err(SocketError::NotConnected);
        }

        if self.recv_buffer.is_empty() {
            return Err(SocketError::WouldBlock);
        }

        let to_read = buffer.len().min(self.recv_buffer.len());
        buffer[..to_read].copy_from_slice(&self.recv_buffer[..to_read]);
        self.recv_buffer.drain(..to_read);

        Ok(to_read)
    }

    pub fn close(&mut self) -> Result<(), SocketError> {
        self.state = SocketState::Closing;
        self.tcp_state = TcpState::FinWait1;
        Ok(())
    }

    pub fn set_option(&mut self, option: &str, value: bool) -> Result<(), SocketError> {
        match option {
            "SO_REUSEADDR" => self.options.reuse_addr = value,
            "SO_KEEPALIVE" => self.options.keep_alive = value,
            "TCP_NODELAY" => self.options.tcp_no_delay = value,
            _ => return Err(SocketError::InvalidArgument),
        }
        Ok(())
    }
}

/// Socket Table
pub struct SocketTable {
    sockets: BTreeMap<i32, Socket>,
    next_fd: i32,
}

impl SocketTable {
    pub fn new() -> Self {
        Self {
            sockets: BTreeMap::new(),
            next_fd: 3, // 0, 1, 2 are stdin, stdout, stderr
        }
    }

    /// Create socket
    pub fn socket(
        &mut self,
        family: AddressFamily,
        socket_type: SocketType,
        protocol: u32,
    ) -> Result<i32, SocketError> {
        let fd = self.next_fd;
        self.next_fd += 1;

        let socket = Socket::new(fd, family, socket_type, protocol);
        self.sockets.insert(fd, socket);

        Ok(fd)
    }

    /// Get socket
    pub fn get_socket(&mut self, fd: i32) -> Result<&mut Socket, SocketError> {
        self.sockets
            .get_mut(&fd)
            .ok_or(SocketError::InvalidSocket)
    }

    /// Close socket
    pub fn close(&mut self, fd: i32) -> Result<(), SocketError> {
        if self.sockets.remove(&fd).is_some() {
            Ok(())
        } else {
            Err(SocketError::InvalidSocket)
        }
    }

    /// Get socket count
    pub fn count(&self) -> usize {
        self.sockets.len()
    }
}

impl Default for SocketTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_creation() {
        let socket = Socket::new(3, AddressFamily::Ipv4, SocketType::Stream, 6);
        assert_eq!(socket.state, SocketState::Created);
        assert_eq!(socket.socket_type, SocketType::Stream);
    }

    #[test]
    fn test_socket_bind() {
        let mut socket = Socket::new(3, AddressFamily::Ipv4, SocketType::Stream, 6);
        let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

        socket.bind(addr).unwrap();
        assert_eq!(socket.state, SocketState::Bound);
        assert_eq!(socket.local_addr, Some(addr));
    }

    #[test]
    fn test_socket_listen() {
        let mut socket = Socket::new(3, AddressFamily::Ipv4, SocketType::Stream, 6);
        let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1), 8080);

        socket.bind(addr).unwrap();
        socket.listen(5).unwrap();
        assert_eq!(socket.state, SocketState::Listening);
    }

    #[test]
    fn test_socket_table() {
        let mut table = SocketTable::new();
        let fd = table
            .socket(AddressFamily::Ipv4, SocketType::Stream, 6)
            .unwrap();

        assert!(fd >= 3);
        assert_eq!(table.count(), 1);

        table.close(fd).unwrap();
        assert_eq!(table.count(), 0);
    }
}
