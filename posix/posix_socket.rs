// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// posix/posix_socket.rs — POSIX Socket API
//
// Implements POSIX-like socket API: socket, bind, connect, listen, accept, send, recv, shutdown
// Internally maps to SigmaOS's networking stack.
// Ensures compatibility with common tools (curl, ssh, etc.).
//
// Language: Rust (no_std for kernel compatibility)

#![no_std]

use super::posix_base::{
    PosixSocket, AddressFamily, SocketType, set_errno_and_return, clear_errno,
    EAFNOSUPPORT, EPROTONOSUPPORT, EOPNOTSUPP, EADDRINUSE, EADDRNOTAVAIL,
    ENOTSOCK, ECONNREFUSED, EINPROGRESS, EISCONN, ENOTCONN, EBADF, EINVAL,
    AF_INET, AF_INET6, AF_UNIX, SOCK_STREAM, SOCK_DGRAM, IPPROTO_TCP, IPPROTO_UDP,
    SOL_SOCKET, SO_REUSEADDR, SO_KEEPALIVE, SO_ERROR,
};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type Isize = isize;
type Usize = usize;

// ─── Socket Address Structures ─────────────────────────

#[repr(C)]
pub struct SockAddr {
    pub sa_family: U16,
    pub sa_data: [U8; 14],
}

impl SockAddr {
    pub const fn new() -> Self {
        SockAddr {
            sa_family: 0,
            sa_data: [0; 14],
        }
    }
}

#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: U16,
    pub sin_port: U16,
    pub sin_addr: U32,
    pub sin_zero: [U8; 8],
}

impl SockAddrIn {
    pub const fn new() -> Self {
        SockAddrIn {
            sin_family: 0,
            sin_port: 0,
            sin_addr: 0,
            sin_zero: [0; 8],
        }
    }
}

#[repr(C)]
pub struct SockAddrIn6 {
    pub sin6_family: U16,
    pub sin6_port: U16,
    pub sin6_flowinfo: U32,
    pub sin6_addr: [U8; 16],
    pub sin6_scope_id: U32,
}

impl SockAddrIn6 {
    pub const fn new() -> Self {
        SockAddrIn6 {
            sin6_family: 0,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: [0; 16],
            sin6_scope_id: 0,
        }
    }
}

#[repr(C)]
pub struct SockAddrUn {
    pub sun_family: U16,
    pub sun_path: [U8; 108],
}

impl SockAddrUn {
    pub const fn new() -> Self {
        SockAddrUn {
            sun_family: 0,
            sun_path: [0; 108],
        }
    }
}

// ─── Socket Options ─────────────────────────────────────

pub const SHUT_RD: I32 = 0;
pub const SHUT_WR: I32 = 1;
pub const SHUT_RDWR: I32 = 2;

// ─── Socket Table ───────────────────────────────────────

pub const MAX_SOCKETS: Usize = 1024;

pub struct SocketTable {
    pub sockets: [PosixSocket; MAX_SOCKETS],
    pub next_fd: I32,
}

impl SocketTable {
    pub const fn new() -> Self {
        SocketTable {
            sockets: [PosixSocket::new(); MAX_SOCKETS],
            next_fd: 3, // Start after stdin, stdout, stderr
        }
    }

    pub fn allocate_fd(&mut self) -> I32 {
        if self.next_fd >= MAX_SOCKETS as I32 {
            unsafe { set_errno_and_return(super::posix_base::EMFILE) };
            return -1;
        }

        let fd = self.next_fd;
        self.next_fd += 1;
        fd
    }

    pub fn free_fd(&mut self, fd: I32) {
        if fd >= 3 && (fd as Usize) < MAX_SOCKETS {
            self.sockets[fd as Usize].fd = -1;
            self.sockets[fd as Usize].is_bound = false;
            self.sockets[fd as Usize].is_connected = false;
            self.sockets[fd as Usize].is_listening = false;
        }
    }

    pub fn get_socket(&mut self, fd: I32) -> Option<&mut PosixSocket> {
        if fd >= 3 && (fd as Usize) < MAX_SOCKETS && self.sockets[fd as Usize].fd >= 0 {
            Some(&mut self.sockets[fd as Usize])
        } else {
            None
        }
    }
}

// ─── Global Socket Table ─────────────────────────────

static mut SOCKET_TABLE: SocketTable = SocketTable::new();

// ─── SigmaOS Socket Operations (stubs) ─────────────────

// These would call into SigmaOS's networking stack
// For now, we provide stub implementations

unsafe fn sigma_socket_create(domain: AddressFamily, type_: SocketType, protocol: I32) -> Result<U64, I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(1) // Return a handle
}

unsafe fn sigma_socket_close(handle: U64) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

unsafe fn sigma_socket_bind(handle: U64, addr: *const SockAddr, addrlen: Usize) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

unsafe fn sigma_socket_connect(handle: U64, addr: *const SockAddr, addrlen: Usize) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

unsafe fn sigma_socket_listen(handle: U64, backlog: I32) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

unsafe fn sigma_socket_accept(handle: U64, addr: *mut SockAddr, addrlen: *mut Usize) -> Result<U64, I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(2) // Return a new handle
}

unsafe fn sigma_socket_send(handle: U64, buffer: *const U8, length: Usize, flags: I32) -> Result<Isize, I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(length as Isize)
}

unsafe fn sigma_socket_recv(handle: U64, buffer: *mut U8, length: Usize, flags: I32) -> Result<Isize, I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(0)
}

unsafe fn sigma_socket_sendto(handle: U64, buffer: *const U8, length: Usize, flags: I32, 
                             addr: *const SockAddr, addrlen: Usize) -> Result<Isize, I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(length as Isize)
}

unsafe fn sigma_socket_recvfrom(handle: U64, buffer: *mut U8, length: Usize, flags: I32,
                               addr: *mut SockAddr, addrlen: *mut Usize) -> Result<Isize, I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(0)
}

unsafe fn sigma_socket_shutdown(handle: U64, how: I32) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

unsafe fn sigma_socket_getsockopt(handle: U64, level: I32, optname: I32, 
                                  optval: *mut U8, optlen: *mut Usize) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

unsafe fn sigma_socket_setsockopt(handle: U64, level: I32, optname: I32, 
                                  optval: *const U8, optlen: Usize) -> Result<(), I32> {
    // Stub: In real implementation, this would call SigmaOS networking stack
    Ok(())
}

// ─── POSIX socket() ───────────────────────────────────

/// Create a socket
#[no_mangle]
pub unsafe extern "C" fn posix_socket(domain: I32, type_: I32, protocol: I32) -> I32 {
    clear_errno();

    // Validate domain
    let addr_family = AddressFamily::from(domain);
    if addr_family == AddressFamily::Unspecified && domain != AF_UNSPEC {
        return set_errno_and_return(EAFNOSUPPORT);
    }

    // Validate type
    let sock_type = SocketType::from(type_);

    // Validate protocol
    if protocol != 0 && protocol != IPPROTO_TCP && protocol != IPPROTO_UDP {
        return set_errno_and_return(EPROTONOSUPPORT);
    }

    // Call SigmaOS socket create
    match sigma_socket_create(addr_family, sock_type, protocol) {
        Ok(handle) => {
            // Allocate file descriptor
            let socket_table = &mut SOCKET_TABLE;
            let fd = socket_table.allocate_fd();
            
            if fd < 0 {
                sigma_socket_close(handle).ok();
                return fd;
            }

            // Set up socket
            if let Some(sock) = socket_table.get_socket(fd) {
                sock.fd = fd;
                sock.sigma_socket.handle = handle;
                sock.domain = addr_family;
                sock.type_ = sock_type;
                sock.protocol = protocol;
                sock.is_bound = false;
                sock.is_connected = false;
                sock.is_listening = false;
            }

            fd
        }
        Err(e) => set_errno_and_return(e),
    }
}

// ─── POSIX bind() ─────────────────────────────────────

/// Bind socket to address
#[no_mangle]
pub unsafe extern "C" fn posix_bind(fd: I32, addr: *const SockAddr, addrlen: Usize) -> I32 {
    clear_errno();

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        if sock.is_bound {
            return set_errno_and_return(EINVAL);
        }

        // Call SigmaOS socket bind
        match sigma_socket_bind(sock.sigma_socket.handle, addr, addrlen) {
            Ok(()) => {
                sock.is_bound = true;
                0
            }
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── POSIX connect() ───────────────────────────────────

/// Connect socket to address
#[no_mangle]
pub unsafe extern "C" fn posix_connect(fd: I32, addr: *const SockAddr, addrlen: Usize) -> I32 {
    clear_errno();

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        if sock.is_connected {
            return set_errno_and_return(EISCONN);
        }

        // Call SigmaOS socket connect
        match sigma_socket_connect(sock.sigma_socket.handle, addr, addrlen) {
            Ok(()) => {
                sock.is_connected = true;
                0
            }
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── POSIX listen() ───────────────────────────────────

/// Listen for connections
#[no_mangle]
pub unsafe extern "C" fn posix_listen(fd: I32, backlog: I32) -> I32 {
    clear_errno();

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        if !sock.is_bound {
            return set_errno_and_return(EINVAL);
        }

        if sock.type_ != SocketType::Stream {
            return set_errno_and_return(EOPNOTSUPP);
        }

        // Call SigmaOS socket listen
        match sigma_socket_listen(sock.sigma_socket.handle, backlog) {
            Ok(()) => {
                sock.is_listening = true;
                0
            }
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── POSIX accept() ───────────────────────────────────

/// Accept a connection
#[no_mangle]
pub unsafe extern "C" fn posix_accept(fd: I32, addr: *mut SockAddr, addrlen: *mut Usize) -> I32 {
    clear_errno();

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        if !sock.is_listening {
            return set_errno_and_return(EINVAL);
        }

        // Call SigmaOS socket accept
        match sigma_socket_accept(sock.sigma_socket.handle, addr, addrlen) {
            Ok(handle) => {
                // Allocate new file descriptor for accepted connection
                let new_fd = socket_table.allocate_fd();
                
                if new_fd < 0 {
                    sigma_socket_close(handle).ok();
                    return new_fd;
                }

                // Set up new socket
                if let Some(new_sock) = socket_table.get_socket(new_fd) {
                    new_sock.fd = new_fd;
                    new_sock.sigma_socket.handle = handle;
                    new_sock.domain = sock.domain;
                    new_sock.type_ = sock.type_;
                    new_sock.protocol = sock.protocol;
                    new_sock.is_bound = true;
                    new_sock.is_connected = true;
                    new_sock.is_listening = false;
                }

                new_fd
            }
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── POSIX send() ─────────────────────────────────────

/// Send data on socket
#[no_mangle]
pub unsafe extern "C" fn posix_send(fd: I32, buffer: *const U8, length: Usize, flags: I32) -> Isize {
    clear_errno();

    if buffer.is_null() || length == 0 {
        return set_errno_and_return(EFAULT) as Isize;
    }

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        if !sock.is_connected && sock.type_ == SocketType::Stream {
            return set_errno_and_return(ENOTCONN) as Isize;
        }

        // Call SigmaOS socket send
        match sigma_socket_send(sock.sigma_socket.handle, buffer, length, flags) {
            Ok(bytes_sent) => bytes_sent,
            Err(e) => set_errno_and_return(e) as Isize,
        }
    } else {
        set_errno_and_return(ENOTSOCK) as Isize
    }
}

// ─── POSIX recv() ─────────────────────────────────────

/// Receive data from socket
#[no_mangle]
pub unsafe extern "C" fn posix_recv(fd: I32, buffer: *mut U8, length: Usize, flags: I32) -> Isize {
    clear_errno();

    if buffer.is_null() || length == 0 {
        return set_errno_and_return(EFAULT) as Isize;
    }

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        if !sock.is_connected && sock.type_ == SocketType::Stream {
            return set_errno_and_return(ENOTCONN) as Isize;
        }

        // Call SigmaOS socket recv
        match sigma_socket_recv(sock.sigma_socket.handle, buffer, length, flags) {
            Ok(bytes_recv) => bytes_recv,
            Err(e) => set_errno_and_return(e) as Isize,
        }
    } else {
        set_errno_and_return(ENOTSOCK) as Isize
    }
}

// ─── POSIX sendto() ───────────────────────────────────

/// Send data on socket to specific address
#[no_mangle]
pub unsafe extern "C" fn posix_sendto(fd: I32, buffer: *const U8, length: Usize, flags: I32,
                                      addr: *const SockAddr, addrlen: Usize) -> Isize {
    clear_errno();

    if buffer.is_null() || length == 0 {
        return set_errno_and_return(EFAULT) as Isize;
    }

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        // Call SigmaOS socket sendto
        match sigma_socket_sendto(sock.sigma_socket.handle, buffer, length, flags, addr, addrlen) {
            Ok(bytes_sent) => bytes_sent,
            Err(e) => set_errno_and_return(e) as Isize,
        }
    } else {
        set_errno_and_return(ENOTSOCK) as Isize
    }
}

// ─── POSIX recvfrom() ─────────────────────────────────

/// Receive data from socket and get sender address
#[no_mangle]
pub unsafe extern "C" fn posix_recvfrom(fd: I32, buffer: *mut U8, length: Usize, flags: I32,
                                        addr: *mut SockAddr, addrlen: *mut Usize) -> Isize {
    clear_errno();

    if buffer.is_null() || length == 0 {
        return set_errno_and_return(EFAULT) as Isize;
    }

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        // Call SigmaOS socket recvfrom
        match sigma_socket_recvfrom(sock.sigma_socket.handle, buffer, length, flags, addr, addrlen) {
            Ok(bytes_recv) => bytes_recv,
            Err(e) => set_errno_and_return(e) as Isize,
        }
    } else {
        set_errno_and_return(ENOTSOCK) as Isize
    }
}

// ─── POSIX shutdown() ─────────────────────────────────

/// Shutdown socket
#[no_mangle]
pub unsafe extern "C" fn posix_shutdown(fd: I32, how: I32) -> I32 {
    clear_errno();

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        // Validate how parameter
        if how != SHUT_RD && how != SHUT_WR && how != SHUT_RDWR {
            return set_errno_and_return(EINVAL);
        }

        // Call SigmaOS socket shutdown
        match sigma_socket_shutdown(sock.sigma_socket.handle, how) {
            Ok(()) => 0,
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── POSIX getsockopt() ───────────────────────────────

/// Get socket option
#[no_mangle]
pub unsafe extern "C" fn posix_getsockopt(fd: I32, level: I32, optname: I32,
                                         optval: *mut U8, optlen: *mut Usize) -> I32 {
    clear_errno();

    if optval.is_null() || optlen.is_null() {
        return set_errno_and_return(EFAULT);
    }

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        // Call SigmaOS socket getsockopt
        match sigma_socket_getsockopt(sock.sigma_socket.handle, level, optname, optval, optlen) {
            Ok(()) => 0,
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── POSIX setsockopt() ───────────────────────────────

/// Set socket option
#[no_mangle]
pub unsafe extern "C" fn posix_setsockopt(fd: I32, level: I32, optname: I32,
                                         optval: *const U8, optlen: Usize) -> I32 {
    clear_errno();

    if optval.is_null() {
        return set_errno_and_return(EFAULT);
    }

    let socket_table = &mut SOCKET_TABLE;
    
    if let Some(sock) = socket_table.get_socket(fd) {
        // Call SigmaOS socket setsockopt
        match sigma_socket_setsockopt(sock.sigma_socket.handle, level, optname, optval, optlen) {
            Ok(()) => 0,
            Err(e) => set_errno_and_return(e),
        }
    } else {
        set_errno_and_return(ENOTSOCK)
    }
}

// ─── C-ABI Wrappers ───────────────────────────────────

#[no_mangle]
pub extern "C" fn socket(domain: I32, type_: I32, protocol: I32) -> I32 {
    unsafe { posix_socket(domain, type_, protocol) }
}

#[no_mangle]
pub extern "C" fn bind(fd: I32, addr: *const SockAddr, addrlen: Usize) -> I32 {
    unsafe { posix_bind(fd, addr, addrlen) }
}

#[no_mangle]
pub extern "C" fn connect(fd: I32, addr: *const SockAddr, addrlen: Usize) -> I32 {
    unsafe { posix_connect(fd, addr, addrlen) }
}

#[no_mangle]
pub extern "C" fn listen(fd: I32, backlog: I32) -> I32 {
    unsafe { posix_listen(fd, backlog) }
}

#[no_mangle]
pub extern "C" fn accept(fd: I32, addr: *mut SockAddr, addrlen: *mut Usize) -> I32 {
    unsafe { posix_accept(fd, addr, addrlen) }
}

#[no_mangle]
pub extern "C" fn send(fd: I32, buffer: *const U8, length: Usize, flags: I32) -> Isize {
    unsafe { posix_send(fd, buffer, length, flags) }
}

#[no_mangle]
pub extern "C" fn recv(fd: I32, buffer: *mut U8, length: Usize, flags: I32) -> Isize {
    unsafe { posix_recv(fd, buffer, length, flags) }
}

#[no_mangle]
pub extern "C" fn sendto(fd: I32, buffer: *const U8, length: Usize, flags: I32,
                        addr: *const SockAddr, addrlen: Usize) -> Isize {
    unsafe { posix_sendto(fd, buffer, length, flags, addr, addrlen) }
}

#[no_mangle]
pub extern "C" fn recvfrom(fd: I32, buffer: *mut U8, length: Usize, flags: I32,
                          addr: *mut SockAddr, addrlen: *mut Usize) -> Isize {
    unsafe { posix_recvfrom(fd, buffer, length, flags, addr, addrlen) }
}

#[no_mangle]
pub extern "C" fn shutdown(fd: I32, how: I32) -> I32 {
    unsafe { posix_shutdown(fd, how) }
}

#[no_mangle]
pub extern "C" fn getsockopt(fd: I32, level: I32, optname: I32,
                            optval: *mut U8, optlen: *mut Usize) -> I32 {
    unsafe { posix_getsockopt(fd, level, optname, optval, optlen) }
}

#[no_mangle]
pub extern "C" fn setsockopt(fd: I32, level: I32, optname: I32,
                            optval: *const U8, optlen: Usize) -> I32 {
    unsafe { posix_setsockopt(fd, level, optname, optval, optlen) }
}
