// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_tcp_state.rs — TCP State Machine (RFC 793)
// Implements full TCP state machine with connection management
//
// States: CLOSED, LISTEN, SYN_SENT, SYN_RCVD, ESTABLISHED, FIN_WAIT_1, FIN_WAIT_2,
//         CLOSING, TIME_WAIT, CLOSE_WAIT, LAST_ACK
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── TCP States (RFC 793 Section 3.2) ─────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TcpState {
    Closed = 0,
    Listen = 1,
    SynSent = 2,
    SynRcvd = 3,
    Established = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    Closing = 7,
    TimeWait = 8,
    CloseWait = 9,
    LastAck = 10,
}

impl TcpState {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(TcpState::Closed),
            1 => Some(TcpState::Listen),
            2 => Some(TcpState::SynSent),
            3 => Some(TcpState::SynRcvd),
            4 => Some(TcpState::Established),
            5 => Some(TcpState::FinWait1),
            6 => Some(TcpState::FinWait2),
            7 => Some(TcpState::Closing),
            8 => Some(TcpState::TimeWait),
            9 => Some(TcpState::CloseWait),
            10 => Some(TcpState::LastAck),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TcpState::Closed => "CLOSED",
            TcpState::Listen => "LISTEN",
            TcpState::SynSent => "SYN_SENT",
            TcpState::SynRcvd => "SYN_RCVD",
            TcpState::Established => "ESTABLISHED",
            TcpState::FinWait1 => "FIN_WAIT_1",
            TcpState::FinWait2 => "FIN_WAIT_2",
            TcpState::Closing => "CLOSING",
            TcpState::TimeWait => "TIME_WAIT",
            TcpState::CloseWait => "CLOSE_WAIT",
            TcpState::LastAck => "LAST_ACK",
        }
    }
}

// ── TCP Flags (RFC 793 Section 3.1) ───────────────────────────────────────

pub const TCP_FIN: u8 = 0x01;
pub const TCP_SYN: u8 = 0x02;
pub const TCP_RST: u8 = 0x04;
pub const TCP_PSH: u8 = 0x08;
pub const TCP_ACK: u8 = 0x10;
pub const TCP_URG: u8 = 0x20;
pub const TCP_ECE: u8 = 0x40;
pub const TCP_CWR: u8 = 0x80;

// ── TCP Control Block ─────────────────────────────────────────────────────

#[repr(C)]
pub struct TcpControlBlock {
    pub state: TcpState,
    pub local_ip: u32,
    pub local_port: u16,
    pub remote_ip: u32,
    pub remote_port: u16,
    pub snd_una: u32,  // Send unacknowledged
    pub snd_nxt: u32,  // Send next
    pub snd_wnd: u16,  // Send window
    pub rcv_nxt: u32,  // Receive next
    pub rcv_wnd: u16,  // Receive window
    pub iss: u32,      // Initial send sequence
    pub irs: u32,      // Initial receive sequence
}

impl TcpControlBlock {
    pub const fn new() -> Self {
        Self {
            state: TcpState::Closed,
            local_ip: 0,
            local_port: 0,
            remote_ip: 0,
            remote_port: 0,
            snd_una: 0,
            snd_nxt: 0,
            snd_wnd: 0,
            rcv_nxt: 0,
            rcv_wnd: 0,
            iss: 0,
            irs: 0,
        }
    }

    /// Initialize for passive open (LISTEN state)
    pub fn init_passive(&mut self, local_ip: u32, local_port: u16) {
        self.state = TcpState::Listen;
        self.local_ip = local_ip;
        self.local_port = local_port;
        self.iss = 0; // Will be set on SYN
    }

    /// Initialize for active open (SYN_SENT state)
    pub fn init_active(&mut self, local_ip: u32, local_port: u16, remote_ip: u32, remote_port: u16, iss: u32) {
        self.state = TcpState::SynSent;
        self.local_ip = local_ip;
        self.local_port = local_port;
        self.remote_ip = remote_ip;
        self.remote_port = remote_port;
        self.iss = iss;
        self.snd_una = iss;
        self.snd_nxt = iss + 1;
    }

    /// Process incoming SYN (passive open)
    pub fn process_syn(&mut self, seq: u32, wnd: u16) -> bool {
        if self.state != TcpState::Listen {
            return false;
        }
        self.state = TcpState::SynRcvd;
        self.irs = seq;
        self.rcv_nxt = seq + 1;
        self.rcv_wnd = wnd;
        true
    }

    /// Process SYN-ACK (active open)
    pub fn process_syn_ack(&mut self, seq: u32, ack: u32, wnd: u16) -> bool {
        if self.state != TcpState::SynSent {
            return false;
        }
        if ack != self.snd_nxt {
            return false;
        }
        self.state = TcpState::Established;
        self.irs = seq;
        self.rcv_nxt = seq + 1;
        self.rcv_wnd = wnd;
        self.snd_una = ack;
        true
    }

    /// Process ACK (various states)
    pub fn process_ack(&mut self, ack: u32, wnd: u16) -> bool {
        match self.state {
            TcpState::SynRcvd => {
                if ack == self.snd_nxt {
                    self.state = TcpState::Established;
                    self.snd_una = ack;
                    self.snd_wnd = wnd;
                    true
                } else {
                    false
                }
            }
            TcpState::Established | TcpState::FinWait1 | TcpState::FinWait2 => {
                if ack >= self.snd_una && ack <= self.snd_nxt {
                    self.snd_una = ack;
                    self.snd_wnd = wnd;
                    true
                } else {
                    false
                }
            }
            TcpState::Closing => {
                if ack == self.snd_nxt {
                    self.state = TcpState::TimeWait;
                    true
                } else {
                    false
                }
            }
            TcpState::LastAck => {
                if ack == self.snd_nxt {
                    self.state = TcpState::Closed;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Process FIN
    pub fn process_fin(&mut self, seq: u32) -> bool {
        if seq != self.rcv_nxt {
            return false;
        }
        match self.state {
            TcpState::Established => {
                self.state = TcpState::CloseWait;
                self.rcv_nxt = seq + 1;
                true
            }
            TcpState::FinWait1 => {
                self.state = TcpState::Closing;
                self.rcv_nxt = seq + 1;
                true
            }
            TcpState::FinWait2 => {
                self.state = TcpState::TimeWait;
                self.rcv_nxt = seq + 1;
                true
            }
            _ => false,
        }
    }

    /// Initiate close (active close)
    pub fn close(&mut self) -> bool {
        match self.state {
            TcpState::Established => {
                self.state = TcpState::FinWait1;
                true
            }
            TcpState::CloseWait => {
                self.state = TcpState::LastAck;
                true
            }
            _ => false,
        }
    }

    /// Check if connection is established
    pub fn is_established(&self) -> bool {
        self.state == TcpState::Established
    }

    /// Check if connection is closed
    pub fn is_closed(&self) -> bool {
        self.state == TcpState::Closed
    }
}

// ── Global TCP connection table ───────────────────────────────────────────

const MAX_TCP_CONNECTIONS: usize = 1024;

pub struct TcpConnectionTable {
    connections: [TcpControlBlock; MAX_TCP_CONNECTIONS],
    count: usize,
}

impl TcpConnectionTable {
    pub const fn new() -> Self {
        Self {
            connections: [TcpControlBlock::new(); MAX_TCP_CONNECTIONS],
            count: 0,
        }
    }

    pub fn allocate(&mut self) -> Option<usize> {
        if self.count >= MAX_TCP_CONNECTIONS {
            return None;
        }
        let idx = self.count;
        self.count += 1;
        Some(idx)
    }

    pub fn free(&mut self, idx: usize) {
        if idx < self.count {
            self.connections[idx] = TcpControlBlock::new();
            // Compact table
            for i in idx..self.count - 1 {
                self.connections[i] = self.connections[i + 1];
            }
            self.count -= 1;
        }
    }

    pub fn get(&self, idx: usize) -> Option<&TcpControlBlock> {
        if idx < self.count {
            Some(&self.connections[idx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut TcpControlBlock> {
        if idx < self.count {
            Some(&mut self.connections[idx])
        } else {
            None
        }
    }

    pub fn find_by_local(&self, local_ip: u32, local_port: u16) -> Option<usize> {
        for i in 0..self.count {
            let tcb = &self.connections[i];
            if tcb.local_ip == local_ip && tcb.local_port == local_port {
                return Some(i);
            }
        }
        None
    }

    pub fn find_by_tuple(&self, local_ip: u32, local_port: u16, remote_ip: u32, remote_port: u16) -> Option<usize> {
        for i in 0..self.count {
            let tcb = &self.connections[i];
            if tcb.local_ip == local_ip && tcb.local_port == local_port
                && tcb.remote_ip == remote_ip && tcb.remote_port == remote_port {
                return Some(i);
            }
        }
        None
    }
}

static mut G_TCP_TABLE: TcpConnectionTable = TcpConnectionTable::new();

#[no_mangle]
pub unsafe extern "C" fn tcp_table_alloc() -> i32 {
    match G_TCP_TABLE.allocate() {
        Some(idx) => idx as i32,
        None => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_table_free(idx: i32) {
    if idx >= 0 {
        G_TCP_TABLE.free(idx as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcp_table_get(idx: i32) -> *mut TcpControlBlock {
    if idx >= 0 {
        G_TCP_TABLE.get_mut(idx as usize).map(|p| p as *mut _).unwrap_or(core::ptr::null_mut())
    } else {
        core::ptr::null_mut()
    }
}
