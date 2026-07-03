// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_socket_syscalls.rs — Socket syscall implementations
// Implements: socket, bind, connect, listen, accept, sendto, recvfrom,
//             setsockopt, getsockopt, shutdown, getpeername, getsockname
//
// Bridges syscall_dispatch.rs → net/sigma_net.rs socket layer.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

pub const ENOTSOCK: i64 = -88;
pub const ECONNREFUSED: i64 = -111;
pub const ETIMEDOUT:    i64 = -110;
pub const ENETUNREACH:  i64 = -101;
pub const EADDRINUSE:   i64 = -98;
pub const ENOTCONN:     i64 = -107;
pub const EBADF:        i64 = -9;
pub const EFAULT:       i64 = -14;
pub const EINVAL:       i64 = -22;
pub const EAGAIN:       i64 = -11;

// ── AF / SOCK constants ────────────────────────────────────────────────────
pub const AF_UNIX:  u32 = 1;
pub const AF_INET:  u32 = 2;
pub const AF_INET6: u32 = 10;

pub const SOCK_STREAM:   u32 = 1;
pub const SOCK_DGRAM:    u32 = 2;
pub const SOCK_RAW:      u32 = 3;
pub const SOCK_NONBLOCK: u32 = 0x800;
pub const SOCK_CLOEXEC:  u32 = 0x80000;

pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 17;

// ── sockaddr_in layout ─────────────────────────────────────────────────────
#[repr(C)]
struct SockaddrIn {
    sin_family: u16,
    sin_port:   u16,  // big-endian
    sin_addr:   u32,  // big-endian IPv4
    sin_zero:   [u8; 8],
}

// ── FD table integration ───────────────────────────────────────────────────
// Each socket gets a file-descriptor slot with FdKind::Socket.
// fd_table_alloc_socket / fd_table_close_socket defined as C externs
// (implemented in sigma_syscalls_io.rs).

extern "C" {
    fn sigma_sock_create(proto: u32) -> i32;
    fn sigma_sock_bind(sockfd: i32, ip: u32, port: u16) -> i32;
    fn sigma_sock_connect(sockfd: i32, dst_ip: u32, dst_port: u16) -> i32;
    fn sigma_sock_listen(sockfd: i32, backlog: i32) -> i32;
    fn sigma_sock_send(sockfd: i32, buf: *const u8, len: usize) -> i64;
    fn sigma_sock_recv(sockfd: i32, buf: *mut u8,  len: usize) -> i64;
    fn sigma_sock_close(sockfd: i32) -> i32;
    fn sigma_fd_alloc_socket(sock_id: i32) -> i32;
}

// ── sys_socket ────────────────────────────────────────────────────────────
pub unsafe fn sys_socket(domain: u64, socktype: u64, protocol: u64) -> i64 {
    let dom = domain as u32;
    if dom != AF_INET && dom != AF_UNIX { return EINVAL; }

    let proto = match (socktype as u32) & 0xF {
        SOCK_STREAM => IPPROTO_TCP,
        SOCK_DGRAM  => IPPROTO_UDP,
        _           => return EINVAL,
    };

    let sock_id = sigma_sock_create(proto);
    if sock_id < 0 { return -24; } // EMFILE

    let fd = sigma_fd_alloc_socket(sock_id);
    if fd < 0 { return -24; }
    fd as i64
}

// ── sys_bind ──────────────────────────────────────────────────────────────
pub unsafe fn sys_bind(sockfd: u64, addr_ptr: u64, addrlen: u64) -> i64 {
    if addr_ptr == 0 { return EFAULT; }
    let addr = &*(addr_ptr as *const SockaddrIn);
    if u16::from_be(addr.sin_family) as u32 != AF_INET { return EINVAL; }
    let ip   = u32::from_be(addr.sin_addr);
    let port = u16::from_be(addr.sin_port);
    let ret  = sigma_sock_bind(sockfd as i32, ip, port);
    if ret < 0 { EADDRINUSE } else { 0 }
}

// ── sys_connect ───────────────────────────────────────────────────────────
pub unsafe fn sys_connect(sockfd: u64, addr_ptr: u64, _addrlen: u64) -> i64 {
    if addr_ptr == 0 { return EFAULT; }
    let addr = &*(addr_ptr as *const SockaddrIn);
    let ip   = u32::from_be(addr.sin_addr);
    let port = u16::from_be(addr.sin_port);
    let ret  = sigma_sock_connect(sockfd as i32, ip, port);
    if ret < 0 { ECONNREFUSED } else { 0 }
}

// ── sys_listen ────────────────────────────────────────────────────────────
pub unsafe fn sys_listen(sockfd: u64, backlog: u64) -> i64 {
    let ret = sigma_sock_listen(sockfd as i32, backlog as i32);
    if ret < 0 { ENOTSOCK } else { 0 }
}

// ── sys_accept ────────────────────────────────────────────────────────────
pub unsafe fn sys_accept(sockfd: u64, addr_ptr: u64, _addrlen: u64) -> i64 {
    // Accept a new connection — returns a new FD
    // In production: dequeue from accept backlog, create new socket
    // For now: return EAGAIN (non-blocking) if no connection pending
    let _ = (sockfd, addr_ptr);
    EAGAIN
}

// ── sys_sendto ────────────────────────────────────────────────────────────
pub unsafe fn sys_sendto(
    sockfd: u64, buf: u64, len: u64,
    _flags: u64, dest_addr: u64, _addrlen: u64,
) -> i64 {
    if buf == 0 { return EFAULT; }
    if dest_addr != 0 {
        // UDP sendto: set remote addr first
        let addr = &*(dest_addr as *const SockaddrIn);
        let ip   = u32::from_be(addr.sin_addr);
        let port = u16::from_be(addr.sin_port);
        sigma_sock_connect(sockfd as i32, ip, port);
    }
    sigma_sock_send(sockfd as i32, buf as *const u8, len as usize)
}

// ── sys_recvfrom ──────────────────────────────────────────────────────────
pub unsafe fn sys_recvfrom(
    sockfd: u64, buf: u64, len: u64,
    _flags: u64, src_addr: u64, addrlen: u64,
) -> i64 {
    if buf == 0 { return EFAULT; }
    let n = sigma_sock_recv(sockfd as i32, buf as *mut u8, len as usize);
    // Fill in src_addr if provided
    if src_addr != 0 && addrlen != 0 {
        // For now: zero-fill (real impl reads from socket state)
        core::ptr::write_bytes(src_addr as *mut u8, 0, 16);
    }
    n
}

// ── sys_shutdown ──────────────────────────────────────────────────────────
pub unsafe fn sys_shutdown(sockfd: u64, _how: u64) -> i64 {
    let ret = sigma_sock_close(sockfd as i32);
    if ret < 0 { EBADF } else { 0 }
}

// ── sys_setsockopt ────────────────────────────────────────────────────────
pub unsafe fn sys_setsockopt(
    _sockfd: u64, _level: u64, _optname: u64,
    _optval: u64, _optlen: u64,
) -> i64 {
    0 // Accept all options silently (no-op)
}

// ── sys_getsockopt ────────────────────────────────────────────────────────
pub unsafe fn sys_getsockopt(
    _sockfd: u64, _level: u64, optname: u64,
    optval: u64, optlen: u64,
) -> i64 {
    const SO_ERROR: u64 = 4;
    if optval != 0 && optname == SO_ERROR {
        (optval as *mut i32).write(0); // no error
    }
    0
}

// ── sys_getsockname / getpeername ─────────────────────────────────────────
pub unsafe fn sys_getsockname(sockfd: u64, addr: u64, _len: u64) -> i64 {
    if addr != 0 {
        let sa = addr as *mut SockaddrIn;
        (*sa).sin_family = (AF_INET as u16).to_be();
        (*sa).sin_port   = 0;
        (*sa).sin_addr   = 0;
    }
    0
}

pub unsafe fn sys_getpeername(sockfd: u64, addr: u64, len: u64) -> i64 {
    sys_getsockname(sockfd, addr, len)
}

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle] pub unsafe extern "C" fn sigma_sys_socket(d:u64,t:u64,p:u64)->i64      { sys_socket(d,t,p) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_bind(s:u64,a:u64,l:u64)->i64        { sys_bind(s,a,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_connect(s:u64,a:u64,l:u64)->i64     { sys_connect(s,a,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_listen(s:u64,b:u64)->i64            { sys_listen(s,b) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_accept(s:u64,a:u64,l:u64)->i64      { sys_accept(s,a,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_sendto(s:u64,b:u64,n:u64,f:u64,d:u64,l:u64)->i64 { sys_sendto(s,b,n,f,d,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_recvfrom(s:u64,b:u64,n:u64,f:u64,a:u64,l:u64)->i64 { sys_recvfrom(s,b,n,f,a,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_setsockopt(s:u64,l:u64,o:u64,v:u64,n:u64)->i64 { sys_setsockopt(s,l,o,v,n) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_getsockopt(s:u64,l:u64,o:u64,v:u64,n:u64)->i64 { sys_getsockopt(s,l,o,v,n) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_getsockname(s:u64,a:u64,l:u64)->i64 { sys_getsockname(s,a,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_getpeername(s:u64,a:u64,l:u64)->i64 { sys_getpeername(s,a,l) }
#[no_mangle] pub unsafe extern "C" fn sigma_sys_shutdown(s:u64,h:u64)->i64          { sys_shutdown(s,h) }
