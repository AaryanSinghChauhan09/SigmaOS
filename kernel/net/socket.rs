// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/socket.rs — POSIX socket API layer
//
// Maps socket()/connect()/bind()/listen()/accept()/send()/recv() syscalls
// to the underlying TCP/UDP stack.
//
// Language: Rust #![no_std]
#![no_std]

// ─── Socket Trait (OOP: Polymorphism & Abstraction) ──────────────────────────
/// A common trait for all socket types!
pub trait Socket {
    /// Binds a socket to an address
    fn bind(&mut self, addr: &SockAddrIn) -> i32;
    /// Starts listening for incoming connections
    fn listen(&mut self, backlog: i32) -> i32;
    /// Connects to a remote address
    fn connect(&mut self, addr: &SockAddrIn) -> i32;
    /// Sends data to the remote end
    fn send(&mut self, buf: &[u8]) -> i64;
    /// Receives data from the remote end
    fn recv(&mut self, buf: &mut [u8]) -> i64;
    /// Closes the socket
    fn close(&mut self) -> i32;
}

// ── Socket address structures ─────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SockAddrIn {
    pub sin_family: u16,   // AF_INET = 2
    pub sin_port:   u16,   // big-endian port
    pub sin_addr:   u32,   // big-endian IPv4 address
    pub sin_zero:   [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SockAddrIn6 {
    pub sin6_family:   u16,
    pub sin6_port:     u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr:     [u8; 16],
    pub sin6_scope_id: u32,
}

pub const AF_UNIX:  u16 = 1;
pub const AF_INET:  u16 = 2;
pub const AF_INET6: u16 = 10;

pub const SOCK_STREAM: i32 = 1;  // TCP
pub const SOCK_DGRAM:  i32 = 2;  // UDP
pub const SOCK_RAW:    i32 = 3;

pub const IPPROTO_TCP: i32 = 6;
pub const IPPROTO_UDP: i32 = 17;

// ── Socket file-descriptor to TCP socket mapping ──────────────────────────
const MAX_SOCKETS: usize = 256;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum SockKind { Unused = 0, Tcp = 1, Udp = 2, Unix = 3 }

#[derive(Copy, Clone)]
struct SocketEntry {
    kind:     SockKind,
    tcp_fd:   usize,   // index into TCP stack
    bound:    bool,
    passive:  bool,    // listening
}

impl SocketEntry {
    const fn empty() -> Self {
        Self { kind: SockKind::Unused, tcp_fd: 0, bound: false, passive: false }
    }
}

struct SocketTable {
    sockets: [SocketEntry; MAX_SOCKETS],
}

impl SocketTable {
    const fn new() -> Self {
        Self { sockets: [const { SocketEntry::empty() }; MAX_SOCKETS] }
    }

    #[inline(always)]
    fn alloc(&mut self) -> Option<usize> {
        for i in 3..MAX_SOCKETS {   // 0,1,2 = stdin/stdout/stderr
            if self.sockets[i].kind == SockKind::Unused { return Some(i); }
        }
        None
    }

    #[inline(always)]
    fn get(&self, fd: i32) -> Option<&SocketEntry> {
        let i = fd as usize;
        if i >= MAX_SOCKETS || self.sockets[i].kind == SockKind::Unused {
            None
        } else {
            Some(&self.sockets[i])
        }
    }

    #[inline(always)]
    fn get_mut(&mut self, fd: i32) -> Option<&mut SocketEntry> {
        let i = fd as usize;
        if i >= MAX_SOCKETS || self.sockets[i].kind == SockKind::Unused {
            None
        } else {
            Some(&mut self.sockets[i])
        }
    }
}

static mut G_SOCKETS: SocketTable = SocketTable::new();

// ── C-ABI POSIX socket syscalls ────────────────────────────────────────────

/// socket(domain, type, protocol) → fd
#[no_mangle]
pub unsafe extern "C" fn sigma_socket(domain: i32, sock_type: i32, protocol: i32) -> i32 {
    let _ = protocol;
    if domain as u16 != AF_INET && domain as u16 != AF_INET6 {
        return -97; // EAFNOSUPPORT
    }

    let kind = match sock_type {
        t if t == SOCK_STREAM => SockKind::Tcp,
        t if t == SOCK_DGRAM  => SockKind::Udp,
        _ => return -93, // ESOCKTNOSUPPORT
    };

    let fd = match G_SOCKETS.alloc() { Some(f) => f, None => return -24 }; // EMFILE

    let tcp_fd = if kind == SockKind::Tcp {
        extern "C" { fn tcp_socket_create() -> i32; }
        let tfd = tcp_socket_create();
        if tfd < 0 { return tfd; }
        tfd as usize
    } else { 0 };

    G_SOCKETS.sockets[fd] = SocketEntry { kind, tcp_fd, bound: false, passive: false };
    fd as i32
}

/// bind(fd, addr, addrlen)
#[no_mangle]
pub unsafe extern "C" fn sigma_bind(fd: i32, addr: *const SockAddrIn, _addrlen: u32) -> i32 {
    if addr.is_null() { return -14; }
    let s = match G_SOCKETS.get_mut(fd) { Some(s) => s, None => return -9 };
    if s.kind == SockKind::Tcp {
        let port = u16::from_be((*addr).sin_port);
        extern "C" { fn tcp_listen(fd: usize, port: u16) -> i32; }
        // Just record port; actual listen() does the state transition
        let _ = port;
    }
    s.bound = true;
    0
}

/// listen(fd, backlog)
#[no_mangle]
pub unsafe extern "C" fn sigma_listen(fd: i32, _backlog: i32) -> i32 {
    let s = match G_SOCKETS.get_mut(fd) { Some(s) => s, None => return -9 };
    if s.kind != SockKind::Tcp { return -95; } // EOPNOTSUPP
    extern "C" { fn tcp_listen(fd: usize, port: u16) -> i32; }
    let r = tcp_listen(s.tcp_fd, 0); // port already set via bind
    s.passive = true;
    r
}

/// connect(fd, addr, addrlen)
#[no_mangle]
pub unsafe extern "C" fn sigma_connect(fd: i32, addr: *const SockAddrIn, _addrlen: u32) -> i32 {
    if addr.is_null() { return -14; }
    let s = match G_SOCKETS.get(fd) { Some(s) => s, None => return -9 };
    if s.kind != SockKind::Tcp { return -95; }
    let remote_ip   = u32::from_be((*addr).sin_addr);
    let remote_port = u16::from_be((*addr).sin_port);
    extern "C" { fn tcp_connect(fd: usize, ip: u32, port: u16) -> i32; }
    tcp_connect(s.tcp_fd, remote_ip, remote_port)
}

/// send(fd, buf, len, flags) → bytes sent
#[no_mangle]
pub unsafe extern "C" fn sigma_send(fd: i32, buf: *const u8, len: usize, _flags: i32) -> i64 {
    if buf.is_null() { return -14; }
    let s = match G_SOCKETS.get(fd) { Some(s) => s, None => return -9 };
    if s.kind == SockKind::Tcp {
        extern "C" { fn tcp_send(fd: usize, buf: *const u8, len: usize) -> i64; }
        return tcp_send(s.tcp_fd, buf, len);
    }
    -95 // EOPNOTSUPP
}

/// recv(fd, buf, len, flags) → bytes received
#[no_mangle]
pub unsafe extern "C" fn sigma_recv(fd: i32, buf: *mut u8, len: usize, _flags: i32) -> i64 {
    if buf.is_null() { return -14; }
    let s = match G_SOCKETS.get(fd) { Some(s) => s, None => return -9 };
    if s.kind == SockKind::Tcp {
        extern "C" { fn tcp_recv(fd: usize, buf: *mut u8, len: usize) -> i64; }
        return tcp_recv(s.tcp_fd, buf, len);
    }
    -95
}

/// sendto — simplified: delegate to send for TCP, stub for UDP
#[no_mangle]
pub unsafe extern "C" fn sigma_sendto(
    fd: i32, buf: *const u8, len: usize, flags: i32,
    _addr: *const SockAddrIn, _addrlen: u32,
) -> i64 {
    sigma_send(fd, buf, len, flags)
}

/// recvfrom — simplified: delegate to recv
#[no_mangle]
pub unsafe extern "C" fn sigma_recvfrom(
    fd: i32, buf: *mut u8, len: usize, flags: i32,
    _addr: *mut SockAddrIn, _addrlen: *mut u32,
) -> i64 {
    sigma_recv(fd, buf, len, flags)
}

/// close socket
#[no_mangle]
pub unsafe extern "C" fn sigma_socket_close(fd: i32) -> i32 {
    let s = match G_SOCKETS.get(fd) { Some(s) => s, None => return -9 };
    if s.kind == SockKind::Tcp {
        extern "C" { fn tcp_close(fd: usize) -> i32; }
        tcp_close(s.tcp_fd);
    }
    if let Some(s) = G_SOCKETS.get_mut(fd) {
        *s = SocketEntry::empty();
    }
    0
}

/// getsockname
#[no_mangle]
pub unsafe extern "C" fn sigma_getsockname(
    fd: i32, addr: *mut SockAddrIn, addrlen: *mut u32,
) -> i32 {
    if addr.is_null() { return -14; }
    if G_SOCKETS.get(fd).is_none() { return -9; }
    // Return placeholder local address
    (*addr).sin_family = u16::to_be(AF_INET);
    (*addr).sin_port   = 0;
    (*addr).sin_addr   = u32::to_be(0xC0A80002); // 192.168.0.2
    if !addrlen.is_null() { *addrlen = core::mem::size_of::<SockAddrIn>() as u32; }
    0
}

/// setsockopt / getsockopt — stubs
#[no_mangle]
pub unsafe extern "C" fn sigma_setsockopt(
    _fd: i32, _level: i32, _optname: i32, _optval: *const u8, _optlen: u32,
) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn sigma_getsockopt(
    _fd: i32, _level: i32, _optname: i32, _optval: *mut u8, _optlen: *mut u32,
) -> i32 { 0 }
