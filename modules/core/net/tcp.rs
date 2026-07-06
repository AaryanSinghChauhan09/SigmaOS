/// SigmaOS — modules/core/net/tcp.rs
/// TCP state machine and segment parsing.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaUsize = usize;
type SigmaI32   = i32;

// ─── TCP State Machine ────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TcpState {
    Closed      = 0,
    Listen      = 1,
    SynSent     = 2,
    SynReceived = 3,
    Established = 4,
    FinWait1    = 5,
    FinWait2    = 6,
    CloseWait   = 7,
    Closing     = 8,
    LastAck     = 9,
    TimeWait    = 10,
}

// ─── Headers & Flags ──────────────────────────────────────────────────────────

pub const TCP_FIN: SigmaU8 = 0x01;
pub const TCP_SYN: SigmaU8 = 0x02;
pub const TCP_RST: SigmaU8 = 0x04;
pub const TCP_PSH: SigmaU8 = 0x08;
pub const TCP_ACK: SigmaU8 = 0x10;
pub const TCP_URG: SigmaU8 = 0x20;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TcpHeader {
    pub src_port: SigmaU16,
    pub dst_port: SigmaU16,
    pub seq_num:  SigmaU32,
    pub ack_num:  SigmaU32,
    pub offset_res_flags: SigmaU16, // 4-bit data offset, 3-bit reserved, 9-bit flags
    pub window:   SigmaU16,
    pub checksum: SigmaU16,
    pub urg_ptr:  SigmaU16,
}

// ─── Connection Control Block (TCB) ───────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TcpConnection {
    pub state:        TcpState,
    pub local_ip:     SigmaU32,
    pub remote_ip:    SigmaU32,
    pub local_port:   SigmaU16,
    pub remote_port:  SigmaU16,
    
    // Send Sequence Variables
    pub snd_una:      SigmaU32, // send unacknowledged
    pub snd_nxt:      SigmaU32, // send next
    pub snd_wnd:      SigmaU16, // send window
    
    // Receive Sequence Variables
    pub rcv_nxt:      SigmaU32, // receive next
    pub rcv_wnd:      SigmaU16, // receive window
    
    pub active:       bool,
}

impl TcpConnection {
    pub const fn empty() -> Self {
        TcpConnection {
            state:       TcpState::Closed,
            local_ip:    0,
            remote_ip:   0,
            local_port:  0,
            remote_port: 0,
            snd_una:     0,
            snd_nxt:     0,
            snd_wnd:     0,
            rcv_nxt:     0,
            rcv_wnd:     0,
            active:      false,
        }
    }
}

const MAX_CONNECTIONS: usize = 128;
static mut TCP_SOCKETS: [TcpConnection; MAX_CONNECTIONS] = [TcpConnection::empty(); MAX_CONNECTIONS];

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn tcp_init() -> SigmaI32 {
    for conn in TCP_SOCKETS.iter_mut() {
        conn.active = false;
        conn.state  = TcpState::Closed;
    }
    0
}

/// Allocate a new TCP connection slot.
#[no_mangle]
pub unsafe extern "C" fn tcp_alloc() -> SigmaI32 {
    for (i, conn) in TCP_SOCKETS.iter_mut().enumerate() {
        if !conn.active {
            conn.active = true;
            conn.state  = TcpState::Closed;
            return i as SigmaI32;
        }
    }
    -1 // ENOMEM
}

/// Input processor for arriving TCP segments.
#[no_mangle]
pub unsafe extern "C" fn tcp_input(
    src_ip: SigmaU32,
    dst_ip: SigmaU32,
    segment: *const u8,
    len: SigmaUsize,
) -> SigmaI32 {
    if segment.is_null() || len < core::mem::size_of::<TcpHeader>() { return -1; }
    
    // In production, read fields from segment bytes safely.
    // For now, this validates the structure exists.
    
    0
}

/// Helper to transition TCP states based on RFC 793
#[no_mangle]
pub unsafe extern "C" fn tcp_set_state(conn_idx: SigmaI32, new_state: SigmaU8) {
    if conn_idx < 0 || conn_idx as usize >= MAX_CONNECTIONS { return; }
    let conn = &mut TCP_SOCKETS[conn_idx as usize];
    
    // Safely cast u8 to enum (assuming values 0..=10)
    if new_state <= 10 {
        conn.state = core::mem::transmute(new_state);
    }
}
