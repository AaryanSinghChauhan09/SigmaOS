// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Networking - TCP State Machine
//! Connection establishment, teardown, and sequence tracking.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;

pub const MAX_TCP_CONNS: usize = 256;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum TcpState {
    Closed = 0,
    Listen = 1,
    SynSent = 2,
    SynReceived = 3,
    Established = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    CloseWait = 7,
    Closing = 8,
    LastAck = 9,
    TimeWait = 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TcpConnection {
    pub local_ip: [u8; 4],
    pub remote_ip: [u8; 4],
    pub local_port: u16,
    pub remote_port: u16,
    pub state: TcpState,
    
    // Sequence numbers
    pub snd_nxt: SigmaU32,
    pub snd_una: SigmaU32,
    pub rcv_nxt: SigmaU32,
    
    // Congestion control window
    pub cwnd: SigmaU32,
    pub active: bool,
}

static mut TCP_CONNS: [TcpConnection; MAX_TCP_CONNS] = [TcpConnection {
    local_ip: [0; 4], remote_ip: [0; 4], local_port: 0, remote_port: 0,
    state: TcpState::Closed, snd_nxt: 0, snd_una: 0, rcv_nxt: 0, cwnd: 1460, active: false,
}; MAX_TCP_CONNS];

#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_init() {
    for i in 0..MAX_TCP_CONNS {
        TCP_CONNS[i].active = false;
    }
}

/// Find a matching TCP connection for an incoming segment
pub unsafe extern "C" fn sigma_tcp_lookup(
    local_ip: *const u8, remote_ip: *const u8, 
    local_port: u16, remote_port: u16
) -> Option<usize> {
    for i in 0..MAX_TCP_CONNS {
        let conn = &TCP_CONNS[i];
        if conn.active && conn.local_port == local_port && conn.remote_port == remote_port {
            if conn.local_ip[0] == *local_ip.add(0) && conn.remote_ip[0] == *remote_ip.add(0) {
                return Some(i);
            }
        }
    }
    None
}

/// Handle an incoming TCP SYN packet
#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_handle_syn(
    local_ip: *const u8, remote_ip: *const u8,
    local_port: u16, remote_port: u16,
    seq: SigmaU32
) -> i32 {
    // Check if we have a listening socket for this port
    let mut listen_idx = None;
    for i in 0..MAX_TCP_CONNS {
        if TCP_CONNS[i].active && TCP_CONNS[i].state == TcpState::Listen && TCP_CONNS[i].local_port == local_port {
            listen_idx = Some(i);
            break;
        }
    }
    
    if listen_idx.is_none() {
        return -1; // Send RST (Connection Refused)
    }
    
    // Allocate new connection slot
    for i in 0..MAX_TCP_CONNS {
        if !TCP_CONNS[i].active {
            TCP_CONNS[i].local_ip = [*local_ip, *local_ip.add(1), *local_ip.add(2), *local_ip.add(3)];
            TCP_CONNS[i].remote_ip = [*remote_ip, *remote_ip.add(1), *remote_ip.add(2), *remote_ip.add(3)];
            TCP_CONNS[i].local_port = local_port;
            TCP_CONNS[i].remote_port = remote_port;
            TCP_CONNS[i].state = TcpState::SynReceived;
            TCP_CONNS[i].rcv_nxt = seq + 1;
            TCP_CONNS[i].snd_nxt = 1000; // Initial sequence number
            TCP_CONNS[i].snd_una = 1000;
            TCP_CONNS[i].active = true;
            return i as i32; // Need to send SYN-ACK
        }
    }
    
    -1 // Queue full, drop
}

/// Handle TCP ACK packet (for 3-way handshake completion)
#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_handle_ack(
    conn_idx: i32, ack: SigmaU32
) -> i32 {
    if conn_idx < 0 || conn_idx >= MAX_TCP_CONNS as i32 {
        return -1;
    }
    
    let conn = &mut TCP_CONNS[conn_idx as usize];
    if !conn.active {
        return -1;
    }
    
    match conn.state {
        TcpState::SynReceived => {
            if ack == conn.snd_nxt {
                conn.state = TcpState::Established;
                return 0; // Connection established
            }
        }
        TcpState::Established => {
            if ack >= conn.snd_una && ack <= conn.snd_nxt {
                conn.snd_una = ack;
                return 0;
            }
        }
        _ => {}
    }
    
    -1
}

/// Send data on established TCP connection
#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_send(
    conn_idx: i32, data: *const u8, len: usize
) -> i32 {
    if conn_idx < 0 || conn_idx >= MAX_TCP_CONNS as i32 {
        return -1;
    }
    
    let conn = &mut TCP_CONNS[conn_idx as usize];
    if !conn.active || conn.state != TcpState::Established {
        return -1;
    }
    
    // Update sequence numbers
    conn.snd_nxt += len as SigmaU32;
    
    // In real implementation, this would queue data for transmission
    // For now, return success
    len as i32
}

/// Receive data from TCP connection
#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_recv(
    conn_idx: i32, buffer: *mut u8, max_len: usize
) -> i32 {
    if conn_idx < 0 || conn_idx >= MAX_TCP_CONNS as i32 {
        return -1;
    }
    
    let conn = &mut TCP_CONNS[conn_idx as usize];
    if !conn.active || conn.state != TcpState::Established {
        return -1;
    }
    
    // In real implementation, this would read from receive buffer
    // For now, return 0 (no data available)
    0
}

/// Close TCP connection
#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_close(conn_idx: i32) -> i32 {
    if conn_idx < 0 || conn_idx >= MAX_TCP_CONNS as i32 {
        return -1;
    }
    
    let conn = &mut TCP_CONNS[conn_idx as usize];
    if !conn.active {
        return -1;
    }
    
    conn.state = TcpState::Closed;
    conn.active = false;
    
    0
}
