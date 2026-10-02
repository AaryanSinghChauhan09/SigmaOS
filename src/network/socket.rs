//! # Socket Layer
//!
//! BSD socket API implementation with Linux socket options.
//! Supports TCP, UDP, Unix domain sockets, and raw sockets.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// Socket domain (address family)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SocketDomain {
    /// IPv4 Internet protocols (AF_INET)
    Inet = 2,
    /// IPv6 Internet protocols (AF_INET6)
    Inet6 = 10,
    /// Unix domain sockets (AF_UNIX)
    Unix = 1,
    /// Netlink sockets (AF_NETLINK)
    Netlink = 16,
    /// Packet sockets (AF_PACKET)
    Packet = 17,
}

/// Socket type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SocketType {
    /// Stream socket (TCP, Unix stream)
    Stream = 1,
    /// Datagram socket (UDP, Unix datagram)
    Dgram = 2,
    /// Raw socket (IP raw, Ethernet raw)
    Raw = 3,
    /// Reliable datagram (SCTP)
    Rdm = 4,
    /// Sequenced packet socket
    Seqpacket = 5,
}

/// Socket protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SocketProtocol {
    /// Default protocol for socket type
    Default = 0,
    /// Transmission Control Protocol
    Tcp = 6,
    /// User Datagram Protocol
    Udp = 17,
    /// Internet Control Message Protocol
    Icmp = 1,
    /// ICMPv6
    Icmpv6 = 58,
    /// Raw IP protocol
    Raw = 255,
}

/// Socket state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SocketState {
    /// Socket created but not bound
    Unbound = 0,
    /// Socket bound to address
    Bound = 1,
    /// Socket listening for connections
    Listening = 2,
    /// Socket connected
    Connected = 3,
    /// Socket disconnected
    Disconnected = 4,
    /// Socket closed
    Closed = 5,
}

/// Socket options (inspired by Linux/BSD setsockopt)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketOption {
    /// SO_REUSEADDR - Allow reuse of local addresses
    ReuseAddr,
    /// SO_REUSEPORT - Allow multiple sockets to bind same port
    ReusePort,
    /// SO_KEEPALIVE - Keep TCP connections alive
    KeepAlive,
    /// SO_BROADCAST - Allow broadcast messages
    Broadcast,
    /// SO_SNDBUF - Send buffer size
    SendBufferSize,
    /// SO_RCVBUF - Receive buffer size
    RecvBufferSize,
    /// SO_LINGER - Linger on close if data present
    Linger,
    /// SO_RCVTIMEO - Receive timeout
    RecvTimeout,
    /// SO_SNDTIMEO - Send timeout
    SendTimeout,
    /// TCP_NODELAY - Disable Nagle's algorithm
    TcpNoDelay,
    /// TCP_CORK - Cork TCP packets (Linux)
    TcpCork,
    /// TCP_QUICKACK - Quick ACK mode
    TcpQuickAck,
    /// IP_TTL - IP time-to-live
    IpTtl,
    /// IP_TOS - Type of service
    IpTos,
    /// IPV6_V6ONLY - Only accept IPv6 connections
    Ipv6Only,
}

/// Socket error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketError {
    /// Address already in use
    AddrInUse,
    /// Cannot assign requested address
    AddrNotAvail,
    /// Connection refused
    ConnectionRefused,
    /// Connection reset by peer
    ConnectionReset,
    /// Connection aborted
    ConnectionAborted,
    /// Connection timed out
    TimedOut,
    /// Network unreachable
    NetworkUnreachable,
    /// Host unreachable
    HostUnreachable,
    /// Socket not connected
    NotConnected,
    /// Socket already connected
    AlreadyConnected,
    /// Operation would block
    WouldBlock,
    /// Message too long
    MessageSize,
    /// Invalid argument
    Invalid,
    /// Socket shutdown
    Shutdown,
    /// Permission denied
    PermissionDenied,
}

/// Socket flags for send/recv operations
#[derive(Debug, Clone, Copy)]
pub struct SocketFlags {
    /// MSG_PEEK - Peek at incoming data
    pub peek: bool,
    /// MSG_DONTWAIT - Non-blocking operation
    pub dontwait: bool,
    /// MSG_WAITALL - Wait for full request or error
    pub waitall: bool,
    /// MSG_OOB - Out-of-band data
    pub oob: bool,
    /// MSG_TRUNC - Return real packet length
    pub trunc: bool,
    /// MSG_CTRUNC - Control data truncated
    pub ctrunc: bool,
    /// MSG_ERRQUEUE - Receive error from error queue
    pub errqueue: bool,
}

impl SocketFlags {
    pub const NONE: Self = Self {
        peek: false,
        dontwait: false,
        waitall: false,
        oob: false,
        trunc: false,
        ctrunc: false,
        errqueue: false,
    };
}

/// Socket address (generic)
#[derive(Debug, Clone)]
pub enum SockAddr {
    /// IPv4 socket address
    Inet(SocketAddr),
    /// IPv6 socket address
    Inet6(SocketAddr),
    /// Unix domain socket path
    Unix(Vec<u8>),
    /// Netlink socket address
    Netlink { pid: u32, groups: u32 },
}

/// Socket buffer
pub struct SocketBuffer {
    data: Vec<u8>,
    capacity: usize,
}

impl SocketBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, SocketError> {
        let available = self.capacity - self.data.len();
        if available == 0 {
            return Err(SocketError::WouldBlock);
        }
        let to_write = available.min(data.len());
        self.data.extend_from_slice(&data[..to_write]);
        Ok(to_write)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SocketError> {
        let to_read = buffer.len().min(self.data.len());
        if to_read == 0 {
            return Err(SocketError::WouldBlock);
        }
        buffer[..to_read].copy_from_slice(&self.data[..to_read]);
        self.data.drain(..to_read);
        Ok(to_read)
    }

    pub fn peek(&self, buffer: &mut [u8]) -> Result<usize, SocketError> {
        let to_read = buffer.len().min(self.data.len());
        if to_read == 0 {
            return Err(SocketError::WouldBlock);
        }
        buffer[..to_read].copy_from_slice(&self.data[..to_read]);
        Ok(to_read)
    }

    pub fn available(&self) -> usize {
        self.data.len()
    }

    pub fn space(&self) -> usize {
        self.capacity - self.data.len()
    }
}

/// Socket control block (inspired by Linux/BSD socket structure)
pub struct Socket {
    /// Socket domain
    pub domain: SocketDomain,
    /// Socket type
    pub socket_type: SocketType,
    /// Socket protocol
    pub protocol: SocketProtocol,
    /// Current state
    pub state: SocketState,
    /// Local address
    pub local_addr: Option<SockAddr>,
    /// Remote address
    pub remote_addr: Option<SockAddr>,
    /// Send buffer
    pub send_buffer: SocketBuffer,
    /// Receive buffer
    pub recv_buffer: SocketBuffer,
    /// Socket options
    pub reuse_addr: bool,
    pub reuse_port: bool,
    pub keepalive: bool,
    pub broadcast: bool,
    pub tcp_nodelay: bool,
    /// Linger time (seconds)
    pub linger: Option<u32>,
    /// Backlog for listen queue
    pub backlog: usize,
}

impl Socket {
    pub fn new(domain: SocketDomain, socket_type: SocketType, protocol: SocketProtocol) -> Self {
        Self {
            domain,
            socket_type,
            protocol,
            state: SocketState::Unbound,
            local_addr: None,
            remote_addr: None,
            send_buffer: SocketBuffer::new(65536), // 64KB default
            recv_buffer: SocketBuffer::new(65536),
            reuse_addr: false,
            reuse_port: false,
            keepalive: false,
            broadcast: false,
            tcp_nodelay: false,
            linger: None,
            backlog: 128, // Linux SOMAXCONN default
        }
    }

    /// Bind socket to local address
    pub fn bind(&mut self, addr: SockAddr) -> Result<(), SocketError> {
        if self.state != SocketState::Unbound {
            return Err(SocketError::Invalid);
        }
        self.local_addr = Some(addr);
        self.state = SocketState::Bound;
        Ok(())
    }

    /// Listen for connections
    pub fn listen(&mut self, backlog: usize) -> Result<(), SocketError> {
        if self.state != SocketState::Bound {
            return Err(SocketError::Invalid);
        }
        if self.socket_type != SocketType::Stream {
            return Err(SocketError::Invalid);
        }
        self.backlog = backlog;
        self.state = SocketState::Listening;
        Ok(())
    }

    /// Connect to remote address
    pub fn connect(&mut self, addr: SockAddr) -> Result<(), SocketError> {
        if self.state == SocketState::Connected {
            return Err(SocketError::AlreadyConnected);
        }
        self.remote_addr = Some(addr);
        self.state = SocketState::Connected;
        Ok(())
    }

    /// Send data
    pub fn send(&mut self, data: &[u8], _flags: SocketFlags) -> Result<usize, SocketError> {
        if self.state != SocketState::Connected {
            return Err(SocketError::NotConnected);
        }
        self.send_buffer.write(data)
    }

    /// Receive data
    pub fn recv(&mut self, buffer: &mut [u8], flags: SocketFlags) -> Result<usize, SocketError> {
        if self.state != SocketState::Connected && self.state != SocketState::Bound {
            return Err(SocketError::NotConnected);
        }
        if flags.peek {
            self.recv_buffer.peek(buffer)
        } else {
            self.recv_buffer.read(buffer)
        }
    }

    /// Set socket option
    pub fn setsockopt(&mut self, option: SocketOption, value: bool) -> Result<(), SocketError> {
        match option {
            SocketOption::ReuseAddr => self.reuse_addr = value,
            SocketOption::ReusePort => self.reuse_port = value,
            SocketOption::KeepAlive => self.keepalive = value,
            SocketOption::Broadcast => self.broadcast = value,
            SocketOption::TcpNoDelay => self.tcp_nodelay = value,
            _ => return Err(SocketError::Invalid),
        }
        Ok(())
    }

    /// Close socket
    pub fn close(&mut self) {
        self.state = SocketState::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_creation() {
        let socket = Socket::new(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp);
        assert_eq!(socket.state, SocketState::Unbound);
        assert_eq!(socket.domain, SocketDomain::Inet);
    }

    #[test]
    fn test_socket_bind() {
        let mut socket = Socket::new(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp);
        let addr = SockAddr::Inet("127.0.0.1:8080".parse().unwrap());
        socket.bind(addr).unwrap();
        assert_eq!(socket.state, SocketState::Bound);
    }

    #[test]
    fn test_socket_listen() {
        let mut socket = Socket::new(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp);
        let addr = SockAddr::Inet("127.0.0.1:8080".parse().unwrap());
        socket.bind(addr).unwrap();
        socket.listen(10).unwrap();
        assert_eq!(socket.state, SocketState::Listening);
        assert_eq!(socket.backlog, 10);
    }

    #[test]
    fn test_socket_options() {
        let mut socket = Socket::new(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp);
        socket.setsockopt(SocketOption::ReuseAddr, true).unwrap();
        socket.setsockopt(SocketOption::TcpNoDelay, true).unwrap();
        assert!(socket.reuse_addr);
        assert!(socket.tcp_nodelay);
    }

    #[test]
    fn test_socket_buffer() {
        let mut buffer = SocketBuffer::new(1024);
        let data = b"Hello, socket!";
        let written = buffer.write(data).unwrap();
        assert_eq!(written, data.len());

        let mut read_buf = [0u8; 64];
        let read = buffer.read(&mut read_buf).unwrap();
        assert_eq!(read, data.len());
        assert_eq!(&read_buf[..read], data);
    }
}
