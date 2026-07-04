// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// net/sigma_tcp.rs — TCP Protocol Implementation
//
// Implements TCP state machine (RFC 793), congestion control,
// and connection management for SigmaOS network stack.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ─── TCP Header ───────────────────────────────────────────────────────────────

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data_off_flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
}

// TCP Flags
pub const TCP_FIN: u16 = 0x0001;
pub const TCP_SYN: u16 = 0x0002;
pub const TCP_RST: u16 = 0x0004;
pub const TCP_PSH: u16 = 0x0008;
pub const TCP_ACK: u16 = 0x0010;
pub const TCP_URG: u16 = 0x0020;
pub const TCP_ECE: u16 = 0x0040;
pub const TCP_CWR: u16 = 0x0080;

// ─── TCP States ───────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

// ─── TCP Connection ───────────────────────────────────────────────────────────

const TCP_RX_BUF_SIZE: usize = 65536;
const TCP_TX_BUF_SIZE: usize = 65536;

pub struct TcpConnection {
    pub state: TcpState,
    pub local_port: u16,
    pub remote_port: u16,
    pub local_ip: [u8; 4],
    pub remote_ip: [u8; 4],
    
    // Sequence numbers
    pub snd_nxt: u32,
    pub snd_una: u32,
    pub rcv_nxt: u32,
    pub rcv_wnd: u32,
    
    // Buffers
    rx_buf: [u8; TCP_RX_BUF_SIZE],
    rx_head: usize,
    rx_tail: usize,
    tx_buf: [u8; TCP_TX_BUF_SIZE],
    tx_head: usize,
    tx_tail: usize,
    
    // Congestion control
    pub cwnd: u32,
    pub ssthresh: u32,
    pub rtt_est: u32,
    pub rtt_var: u32,
}

impl TcpConnection {
    pub const fn new() -> Self {
        TcpConnection {
            state: TcpState::Closed,
            local_port: 0,
            remote_port: 0,
            local_ip: [0; 4],
            remote_ip: [0; 4],
            snd_nxt: 0,
            snd_una: 0,
            rcv_nxt: 0,
            rcv_wnd: 65535,
            rx_buf: [0; TCP_RX_BUF_SIZE],
            rx_head: 0,
            rx_tail: 0,
            tx_buf: [0; TCP_TX_BUF_SIZE],
            tx_head: 0,
            tx_tail: 0,
            cwnd: 1460, // Initial congestion window (1 MSS)
            ssthresh: 65535,
            rtt_est: 0,
            rtt_var: 0,
        }
    }
    
    pub fn rx_push(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let next = (self.rx_tail + 1) % TCP_RX_BUF_SIZE;
            if next == self.rx_head { break; }
            self.rx_buf[self.rx_tail] = b;
            self.rx_tail = next;
            written += 1;
        }
        written
    }
    
    pub fn rx_pop(&mut self, buf: &mut [u8]) -> usize {
        let mut read = 0;
        while read < buf.len() && self.rx_head != self.rx_tail {
            buf[read] = self.rx_buf[self.rx_head];
            self.rx_head = (self.rx_head + 1) % TCP_RX_BUF_SIZE;
            read += 1;
        }
        read
    }
    
    pub fn tx_push(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let next = (self.tx_tail + 1) % TCP_TX_BUF_SIZE;
            if next == self.tx_head { break; }
            self.tx_buf[self.tx_tail] = b;
            self.tx_tail = next;
            written += 1;
        }
        written
    }
    
    pub fn tx_available(&self) -> usize {
        if self.tx_tail >= self.tx_head {
            TCP_TX_BUF_SIZE - (self.tx_tail - self.tx_head) - 1
        } else {
            self.tx_head - self.tx_tail - 1
        }
    }
}

// ─── TCP Connection Table ─────────────────────────────────────────────────────

const MAX_TCP_CONNS: usize = 128;

static mut TCP_CONNS: [TcpConnection; MAX_TCP_CONNS] = [const { TcpConnection::new() }; MAX_TCP_CONNS];
static mut TCP_CONN_COUNT: usize = 0;

pub fn tcp_alloc_conn() -> Option<usize> {
    unsafe {
        for i in 0..MAX_TCP_CONNS {
            if TCP_CONNS[i].state == TcpState::Closed {
                TCP_CONNS[i] = TcpConnection::new();
                TCP_CONN_COUNT += 1;
                return Some(i);
            }
        }
        None
    }
}

pub fn tcp_free_conn(idx: usize) {
    unsafe {
        if idx < MAX_TCP_CONNS {
            TCP_CONNS[idx] = TcpConnection::new();
            TCP_CONN_COUNT = TCP_CONN_COUNT.saturating_sub(1);
        }
    }
}

pub fn tcp_get_conn(idx: usize) -> Option<&'static mut TcpConnection> {
    unsafe {
        if idx < MAX_TCP_CONNS && TCP_CONNS[idx].state != TcpState::Closed {
            Some(&mut TCP_CONNS[idx])
        } else {
            None
        }
    }
}

// ─── TCP Segment Processing ────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub enum SegmentAction {
    Accept,
    Ignore,
    Reset,
    SendSynAck,
    SendAck,
    SendFin,
}

pub fn tcp_process_segment(
    conn: &mut TcpConnection,
    hdr: &TcpHeader,
    payload: &[u8],
) -> SegmentAction {
    let flags = u16::from_be(hdr.data_off_flags) & 0x0FFF;
    let is_syn = (flags & TCP_SYN) != 0;
    let is_ack = (flags & TCP_ACK) != 0;
    let is_fin = (flags & TCP_FIN) != 0;
    let is_rst = (flags & TCP_RST) != 0;
    
    match conn.state {
        TcpState::Closed => {
            // Send RST for any segment to closed connection
            SegmentAction::Reset
        }
        
        TcpState::Listen => {
            if is_syn && !is_ack {
                // New connection: SYN received
                conn.state = TcpState::SynReceived;
                conn.rcv_nxt = u32::from_be(hdr.seq_num) + 1;
                conn.snd_nxt = 0xDEADBEEF; // Initial ISN
                conn.snd_una = conn.snd_nxt;
                SegmentAction::SendSynAck
            } else {
                SegmentAction::Ignore
            }
        }
        
        TcpState::SynSent => {
            if is_syn && is_ack {
                // SYN-ACK received, connection established
                conn.state = TcpState::Established;
                conn.rcv_nxt = u32::from_be(hdr.seq_num) + 1;
                conn.snd_nxt = u32::from_be(hdr.ack_num);
                conn.snd_una = conn.snd_nxt;
                SegmentAction::SendAck
            } else if is_syn {
                // Simultaneous open
                conn.state = TcpState::SynReceived;
                conn.rcv_nxt = u32::from_be(hdr.seq_num) + 1;
                SegmentAction::SendSynAck
            } else {
                SegmentAction::Ignore
            }
        }
        
        TcpState::SynReceived => {
            if is_ack && u32::from_be(hdr.ack_num) == conn.snd_nxt + 1 {
                // ACK of our SYN-ACK, connection established
                conn.state = TcpState::Established;
                conn.snd_una = u32::from_be(hdr.ack_num);
                SegmentAction::Accept
            } else {
                SegmentAction::Ignore
            }
        }
        
        TcpState::Established => {
            if is_rst {
                conn.state = TcpState::Closed;
                return SegmentAction::Reset;
            }
            
            // Update receive window
            conn.rcv_wnd = u16::from_be(hdr.window) as u32;
            
            // Process ACK
            if is_ack {
                let ack = u32::from_be(hdr.ack_num);
                if ack > conn.snd_una {
                    conn.snd_una = ack;
                    // Update congestion control on ACK
                    tcp_update_congestion(conn);
                }
            }
            
            // Process data
            let seq = u32::from_be(hdr.seq_num);
            if seq == conn.rcv_nxt {
                if !payload.is_empty() {
                    conn.rx_push(payload);
                    conn.rcv_nxt += payload.len() as u32;
                    if is_ack {
                        SegmentAction::Accept
                    } else {
                        SegmentAction::SendAck
                    }
                } else if is_fin {
                    conn.state = TcpState::CloseWait;
                    conn.rcv_nxt += 1;
                    SegmentAction::SendAck
                } else {
                    SegmentAction::Accept
                }
            } else {
                // Out-of-order segment
                SegmentAction::SendAck
            }
        }
        
        TcpState::FinWait1 | TcpState::FinWait2 | TcpState::CloseWait 
            | TcpState::Closing | TcpState::LastAck | TcpState::TimeWait => {
            // Connection teardown states
            if is_fin {
                conn.rcv_nxt += 1;
                match conn.state {
                    TcpState::FinWait1 => {
                        if is_ack {
                            conn.state = TcpState::TimeWait;
                        } else {
                            conn.state = TcpState::Closing;
                        }
                    }
                    TcpState::FinWait2 => {
                        conn.state = TcpState::TimeWait;
                    }
                    TcpState::CloseWait => {
                        // Application will close
                    }
                    _ => {}
                }
                SegmentAction::SendAck
            } else if is_ack {
                match conn.state {
                    TcpState::FinWait1 => {
                        conn.state = TcpState::FinWait2;
                        SegmentAction::Accept
                    }
                    TcpState::Closing => {
                        conn.state = TcpState::TimeWait;
                        SegmentAction::Accept
                    }
                    TcpState::LastAck => {
                        conn.state = TcpState::Closed;
                        SegmentAction::Accept
                    }
                    _ => SegmentAction::Accept,
                }
            } else {
                SegmentAction::Accept
            }
        }
    }
}

// ─── Congestion Control ───────────────────────────────────────────────────────

fn tcp_update_congestion(conn: &mut TcpConnection) {
    // Simplified Reno-style congestion control
    if conn.cwnd < conn.ssthresh {
        // Slow start: double cwnd on each ACK
        conn.cwnd = (conn.cwnd * 2).min(65535);
    } else {
        // Congestion avoidance: additive increase
        conn.cwnd = (conn.cwnd + 1460).min(65535);
    }
}

pub fn tcp_on_timeout(conn: &mut TcpConnection) {
    // Timeout: reduce congestion window
    conn.ssthresh = conn.cwnd / 2;
    conn.cwnd = 1460; // Reset to 1 MSS
}

// ─── C-ABI Exports ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_init() {
    TCP_CONN_COUNT = 0;
    for conn in TCP_CONNS.iter_mut() {
        *conn = TcpConnection::new();
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_conn_alloc() -> i32 {
    match tcp_alloc_conn() {
        Some(idx) => idx as i32,
        None => -1, // EMFILE
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_conn_free(idx: i32) {
    if idx >= 0 {
        tcp_free_conn(idx as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tcp_conn_state(idx: i32) -> i32 {
    if idx < 0 { return -1; }
    match tcp_get_conn(idx as usize) {
        Some(conn) => match conn.state {
            TcpState::Closed => 0,
            TcpState::Listen => 1,
            TcpState::SynSent => 2,
            TcpState::SynReceived => 3,
            TcpState::Established => 4,
            TcpState::FinWait1 => 5,
            TcpState::FinWait2 => 6,
            TcpState::CloseWait => 7,
            TcpState::Closing => 8,
            TcpState::LastAck => 9,
            TcpState::TimeWait => 10,
        },
        None => -1,
    }
}
