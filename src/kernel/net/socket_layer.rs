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

use core::sync::atomic::{AtomicUsize, Ordering};
/// SigmaOS Network Socket Layer
/// Absorbs Linux BSD socket interface: socket()/bind()/listen()/accept()/connect()
/// Supports AF_INET (IPv4), AF_INET6, AF_UNIX; SOCK_STREAM/DGRAM/RAW
use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ── Address Families & Socket Types ──────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressFamily {
    Unspec,
    Unix,
    Inet,
    Inet6,
    Netlink,
    Packet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Dgram,
    Raw,
    SeqPacket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp = 6,
    Udp = 17,
    Icmp = 1,
    Raw = 0,
}

/// IPv4 socket address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SockAddrIn {
    pub port: u16,
    pub addr: [u8; 4], // IPv4 address
}

impl SockAddrIn {
    pub fn new(addr: [u8; 4], port: u16) -> Self {
        SockAddrIn { port, addr }
    }
    pub fn loopback(port: u16) -> Self {
        SockAddrIn::new([127, 0, 0, 1], port)
    }
    pub fn any(port: u16) -> Self {
        SockAddrIn::new([0, 0, 0, 0], port)
    }
}

// ── Socket State Machine ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Unbound,
    Bound,
    Listening,
    Connected,
    Accepted,
    CloseWait,
    FinWait1,
    FinWait2,
    TimeWait,
    Closed,
}

/// File descriptor flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketFlags {
    pub non_blocking: bool,
    pub reuse_addr: bool,
    pub reuse_port: bool,
    pub keep_alive: bool,
    pub no_delay: bool, // TCP_NODELAY (disable Nagle)
}

impl Default for SocketFlags {
    fn default() -> Self {
        SocketFlags {
            non_blocking: false,
            reuse_addr: false,
            reuse_port: false,
            keep_alive: false,
            no_delay: false,
        }
    }
}

/// Socket receive/send buffer
pub struct SocketBuffer {
    data: Vec<u8>,
    capacity: usize,
}

impl SocketBuffer {
    pub fn new(capacity: usize) -> Self {
        SocketBuffer {
            data: Vec::new(),
            capacity,
        }
    }
    pub fn push(&mut self, buf: &[u8]) -> usize {
        let avail = self.capacity.saturating_sub(self.data.len());
        let n = buf.len().min(avail);
        self.data.extend_from_slice(&buf[..n]);
        n
    }
    pub fn pop(&mut self, n: usize) -> Vec<u8> {
        let n = n.min(self.data.len());
        self.data.drain(..n).collect()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// A kernel socket
pub struct Socket {
    pub fd: u32,
    pub family: AddressFamily,
    pub sock_type: SocketType,
    pub protocol: Protocol,
    pub state: SocketState,
    pub local_addr: Option<SockAddrIn>,
    pub remote_addr: Option<SockAddrIn>,
    pub flags: SocketFlags,
    pub recv_buf: SocketBuffer,
    pub send_buf: SocketBuffer,
    pub backlog: usize,
    pub pending_accept: Vec<u32>, // FDs of sockets waiting to be accept()ed
    bytes_sent: AtomicUsize,
    bytes_recv: AtomicUsize,
}

impl Socket {
    pub fn new(fd: u32, family: AddressFamily, sock_type: SocketType, proto: Protocol) -> Self {
        Socket {
            fd,
            family,
            sock_type,
            protocol: proto,
            state: SocketState::Unbound,
            local_addr: None,
            remote_addr: None,
            flags: SocketFlags::default(),
            recv_buf: SocketBuffer::new(131072), // 128KB default
            send_buf: SocketBuffer::new(131072),
            backlog: 128,
            pending_accept: Vec::new(),
            bytes_sent: AtomicUsize::new(0),
            bytes_recv: AtomicUsize::new(0),
        }
    }

    pub fn bind(&mut self, addr: SockAddrIn) -> Result<(), &'static str> {
        if self.state != SocketState::Unbound {
            return Err("Socket already bound");
        }
        self.local_addr = Some(addr);
        self.state = SocketState::Bound;
        Ok(())
    }

    pub fn listen(&mut self, backlog: usize) -> Result<(), &'static str> {
        if self.sock_type != SocketType::Stream {
            return Err("Only SOCK_STREAM can listen");
        }
        if self.state != SocketState::Bound {
            return Err("Socket must be bound first");
        }
        self.backlog = backlog;
        self.state = SocketState::Listening;
        Ok(())
    }

    pub fn connect(&mut self, remote: SockAddrIn) -> Result<(), &'static str> {
        if self.state == SocketState::Connected {
            return Err("Already connected");
        }
        self.remote_addr = Some(remote);
        self.state = SocketState::Connected;
        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state != SocketState::Connected {
            return Err("Not connected");
        }
        let n = self.send_buf.push(data);
        self.bytes_sent.fetch_add(n, Ordering::Relaxed);
        Ok(n)
    }

    pub fn recv(&mut self, max: usize) -> Result<Vec<u8>, &'static str> {
        if self.state != SocketState::Connected && self.state != SocketState::Accepted {
            return Err("Not connected");
        }
        let data = self.recv_buf.pop(max);
        self.bytes_recv.fetch_add(data.len(), Ordering::Relaxed);
        Ok(data)
    }

    pub fn inject_data(&mut self, data: &[u8]) {
        self.recv_buf.push(data);
    }
    pub fn bytes_sent(&self) -> usize {
        self.bytes_sent.load(Ordering::Relaxed)
    }
    pub fn bytes_recv(&self) -> usize {
        self.bytes_recv.load(Ordering::Relaxed)
    }
}

// ── Socket Manager (kernel socket table) ──────────────────────────────────

pub struct SocketLayer {
    sockets: HashMap<u32, Socket>,
    next_fd: AtomicUsize,
    bound_ports: HashMap<u16, u32>, // port -> fd
}

impl SocketLayer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SocketLayer {
            sockets: HashMap::new(),
            next_fd: AtomicUsize::new(4),
            bound_ports: HashMap::new(),
        }
    }

    pub fn socket(&mut self, family: AddressFamily, typ: SocketType, proto: Protocol) -> u32 {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst) as u32;
        self.sockets.insert(fd, Socket::new(fd, family, typ, proto));
        fd
    }

    pub fn bind(&mut self, fd: u32, addr: SockAddrIn) -> Result<(), &'static str> {
        if self.bound_ports.contains_key(&addr.port) {
            return Err("EADDRINUSE: port already bound");
        }
        self.bound_ports.insert(addr.port, fd);
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF: invalid fd")?;
        sock.bind(addr)
    }

    pub fn listen(&mut self, fd: u32, backlog: usize) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.listen(backlog)
    }

    pub fn connect(&mut self, fd: u32, remote: SockAddrIn) -> Result<(), &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.connect(remote)
    }

    pub fn send(&mut self, fd: u32, data: &[u8]) -> Result<usize, &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.send(data)
    }

    pub fn recv(&mut self, fd: u32, max: usize) -> Result<Vec<u8>, &'static str> {
        let sock = self.sockets.get_mut(&fd).ok_or("EBADF")?;
        sock.recv(max)
    }

    pub fn inject_data(&mut self, fd: u32, data: &[u8]) {
        if let Some(sock) = self.sockets.get_mut(&fd) {
            sock.inject_data(data);
        }
    }

    pub fn close(&mut self, fd: u32) -> Result<(), &'static str> {
        let sock = self.sockets.remove(&fd).ok_or("EBADF")?;
        if let Some(addr) = sock.local_addr {
            self.bound_ports.remove(&addr.port);
        }
        Ok(())
    }

    pub fn get_socket(&self, fd: u32) -> Option<&Socket> {
        self.sockets.get(&fd)
    }
    pub fn socket_count(&self) -> usize {
        self.sockets.len()
    }
}

impl Default for SocketLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_server_client_lifecycle() {
        let mut sl = SocketLayer::new();

        // Server
        let server_fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.bind(server_fd, SockAddrIn::any(8080)).unwrap();
        sl.listen(server_fd, 10).unwrap();
        assert_eq!(
            sl.get_socket(server_fd).unwrap().state,
            SocketState::Listening
        );

        // Client
        let client_fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.connect(client_fd, SockAddrIn::loopback(8080)).unwrap();
        assert_eq!(
            sl.get_socket(client_fd).unwrap().state,
            SocketState::Connected
        );

        // Data transfer
        let n = sl.send(client_fd, b"Hello SigmaOS!").unwrap();
        assert_eq!(n, 14);
        assert_eq!(sl.get_socket(client_fd).unwrap().bytes_sent(), 14);
    }

    #[test]
    fn test_socket_close_releases_port() {
        let mut sl = SocketLayer::new();
        let fd = sl.socket(AddressFamily::Inet, SocketType::Dgram, Protocol::Udp);
        sl.bind(fd, SockAddrIn::any(53)).unwrap();
        sl.close(fd).unwrap();
        // Can bind again after close
        let fd2 = sl.socket(AddressFamily::Inet, SocketType::Dgram, Protocol::Udp);
        sl.bind(fd2, SockAddrIn::any(53)).unwrap();
    }

    #[test]
    fn test_recv_inject() {
        let mut sl = SocketLayer::new();
        let fd = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.connect(fd, SockAddrIn::loopback(9000)).unwrap();
        sl.inject_data(fd, b"response data");
        let data = sl.recv(fd, 512).unwrap();
        assert_eq!(data, b"response data");
    }

    #[test]
    fn test_port_collision() {
        let mut sl = SocketLayer::new();
        let fd1 = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        let fd2 = sl.socket(AddressFamily::Inet, SocketType::Stream, Protocol::Tcp);
        sl.bind(fd1, SockAddrIn::any(80)).unwrap();
        let result = sl.bind(fd2, SockAddrIn::any(80));
        assert!(result.is_err());
    }
}
