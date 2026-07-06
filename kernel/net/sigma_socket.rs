// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Networking - Socket IPC Layer
//! POSIX-like sockets built on static arrays.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const MAX_SOCKETS: usize = 1024;
pub const SOCK_STREAM: u8 = 1;
pub const SOCK_DGRAM: u8 = 2;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum SocketState {
    Closed = 0,
    Listen = 1,
    Established = 2,
    FinWait = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaSocket {
    pub fd: SigmaI32,
    pub protocol: u8, // SOCK_STREAM or SOCK_DGRAM
    pub state: SocketState,
    pub local_port: u16,
    pub remote_port: u16,
    pub remote_ip: [u8; 4], // IPv4
    pub active: bool,
}

static mut SOCKETS: [SigmaSocket; MAX_SOCKETS] = [SigmaSocket {
    fd: -1, protocol: 0, state: SocketState::Closed,
    local_port: 0, remote_port: 0, remote_ip: [0; 4], active: false,
}; MAX_SOCKETS];

#[no_mangle]
pub unsafe extern "C" fn sigma_socket_create(protocol: u8) -> SigmaI32 {
    for i in 0..MAX_SOCKETS {
        if !SOCKETS[i].active {
            SOCKETS[i].fd = i as SigmaI32;
            SOCKETS[i].protocol = protocol;
            SOCKETS[i].state = SocketState::Closed;
            SOCKETS[i].active = true;
            return i as SigmaI32;
        }
    }
    -1 // EMFD
}

#[no_mangle]
pub unsafe extern "C" fn sigma_socket_bind(fd: SigmaI32, port: u16) -> SigmaI32 {
    if fd < 0 || fd as usize >= MAX_SOCKETS { return -1; }
    let sock = &mut SOCKETS[fd as usize];
    if !sock.active { return -1; }
    
    // Check if port in use
    for i in 0..MAX_SOCKETS {
        if SOCKETS[i].active && SOCKETS[i].local_port == port {
            return -1; // EADDRINUSE
        }
    }
    
    sock.local_port = port;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_socket_listen(fd: SigmaI32) -> SigmaI32 {
    if fd < 0 || fd as usize >= MAX_SOCKETS { return -1; }
    let sock = &mut SOCKETS[fd as usize];
    if !sock.active || sock.protocol != SOCK_STREAM { return -1; }
    
    sock.state = SocketState::Listen;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_socket_close(fd: SigmaI32) -> SigmaI32 {
    if fd < 0 || fd as usize >= MAX_SOCKETS { return -1; }
    let sock = &mut SOCKETS[fd as usize];
    if !sock.active { return -1; }
    
    sock.state = SocketState::Closed;
    sock.active = false;
    0
}
