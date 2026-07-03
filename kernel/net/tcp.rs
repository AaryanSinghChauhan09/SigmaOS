// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/tcp.rs — TCP/IP state machine (RFC 793)
//
// Implements the full TCP connection state machine:
//   CLOSED → LISTEN → SYN_RCVD → ESTABLISHED → FIN_WAIT_1/2 → CLOSED
//   CLOSED → SYN_SENT → ESTABLISHED → CLOSE_WAIT → LAST_ACK → CLOSED
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ── TCP header (20 bytes minimum, no options) ─────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TcpHdr {
    pub src_port:  u16,
    pub dst_port:  u16,
    pub seq:       u32,
    pub ack:       u32,
    pub data_off:  u8,   // header length in 32-bit words, upper 4 bits
    pub flags:     u8,
    pub window:    u16,
    pub checksum:  u16,
    pub urgent:    u16,
}

pub const TCP_FLAG_FIN: u8 = 1 << 0;
pub const TCP_FLAG_SYN: u8 = 1 << 1;
pub const TCP_FLAG_RST: u8 = 1 << 2;
pub const TCP_FLAG_PSH: u8 = 1 << 3;
pub const TCP_FLAG_ACK: u8 = 1 << 4;
pub const TCP_FLAG_URG: u8 = 1 << 5;

// ── TCP connection states (RFC 793 §3.2) ─────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TcpState {
    Closed      = 0,
    Listen      = 1,
    SynSent     = 2,
    SynRcvd     = 3,
    Established = 4,
    FinWait1    = 5,
    FinWait2    = 6,
    CloseWait   = 7,
    Closing     = 8,
    LastAck     = 9,
    TimeWait    = 10,
}

// ── TCP socket ────────────────────────────────────────────────────────────
const TCP_RX_BUF: usize = 65536;
const TCP_TX_BUF: usize = 65536;
const MAX_TCP_SOCKETS: usize = 256;
const MSS: u32 = 1460; // Maximum Segment Size

static ISN_COUNTER: AtomicU32 = AtomicU32::new(0x12345678);
fn generate_isn() -> u32 { ISN_COUNTER.fetch_add(1, Ordering::Relaxed) }

pub struct TcpSocket {
    pub state:       TcpState,
    pub local_ip:    u32,
    pub local_port:  u16,
    pub remote_ip:   u32,
    pub remote_port: u16,
    // Send sequence variables (RFC 793 §3.3)
    pub snd_una:     u32,  // oldest unacknowledged seq
    pub snd_nxt:     u32,  // next seq to send
    pub snd_wnd:     u32,  // send window size
    pub snd_wl1:     u32,  // seq number for last window update
    pub snd_wl2:     u32,  // ack number for last window update
    pub iss:         u32,  // initial send sequence number
    // Receive sequence variables
    pub rcv_nxt:     u32,  // next expected seq
    pub rcv_wnd:     u32,  // receive window
    pub irs:         u32,  // initial receive sequence number
    // Retransmission
    pub rtt_ms:      u32,  // estimated RTT in ms
    pub rto_ms:      u32,  // retransmission timeout in ms
    pub retransmit_count: u8,
    // Buffers
    pub rx_buf:      [u8; TCP_RX_BUF],
    pub rx_head:     usize,
    pub rx_tail:     usize,
    pub tx_buf:      [u8; TCP_TX_BUF],
    pub tx_head:     usize,
    pub tx_tail:     usize,
    pub active:      bool,
}

impl TcpSocket {
    pub const fn new() -> Self {
        Self {
            state: TcpState::Closed,
            local_ip: 0, local_port: 0,
            remote_ip: 0, remote_port: 0,
            snd_una: 0, snd_nxt: 0, snd_wnd: 65535,
            snd_wl1: 0, snd_wl2: 0, iss: 0,
            rcv_nxt: 0, rcv_wnd: 65535, irs: 0,
            rtt_ms: 200, rto_ms: 1000,
            retransmit_count: 0,
            rx_buf: [0u8; TCP_RX_BUF],
            rx_head: 0, rx_tail: 0,
            tx_buf: [0u8; TCP_TX_BUF],
            tx_head: 0, tx_tail: 0,
            active: false,
        }
    }

    fn rx_push(&mut self, data: &[u8]) {
        for &b in data {
            let next = (self.rx_tail + 1) % TCP_RX_BUF;
            if next != self.rx_head {
                self.rx_buf[self.rx_tail] = b;
                self.rx_tail = next;
            }
        }
    }

    fn rx_pop(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() && self.rx_head != self.rx_tail {
            buf[n] = self.rx_buf[self.rx_head];
            self.rx_head = (self.rx_head + 1) % TCP_RX_BUF;
            n += 1;
        }
        n
    }

    fn rx_available(&self) -> usize {
        (self.rx_tail + TCP_RX_BUF - self.rx_head) % TCP_RX_BUF
    }

    fn tx_push(&mut self, data: &[u8]) -> usize {
        let mut n = 0;
        for &b in data {
            let next = (self.tx_tail + 1) % TCP_TX_BUF;
            if next == self.tx_head { break; }
            self.tx_buf[self.tx_tail] = b;
            self.tx_tail = next;
            n += 1;
        }
        n
    }
}

// ── TCP state machine ─────────────────────────────────────────────────────
pub struct TcpStack {
    sockets: [TcpSocket; MAX_TCP_SOCKETS],
}

impl TcpStack {
    pub const fn new() -> Self {
        Self { sockets: [const { TcpSocket::new() }; MAX_TCP_SOCKETS] }
    }

    fn alloc_socket(&mut self) -> Option<usize> {
        for i in 0..MAX_TCP_SOCKETS {
            if !self.sockets[i].active { return Some(i); }
        }
        None
    }

    pub fn socket_create(&mut self) -> i32 {
        let idx = self.alloc_socket()?;
        self.sockets[idx].active = true;
        self.sockets[idx].state  = TcpState::Closed;
        idx as i32
    }

    /// connect(): initiate 3-way handshake (active open)
    pub fn connect(&mut self, fd: usize, remote_ip: u32, remote_port: u16) -> i32 {
        if fd >= MAX_TCP_SOCKETS || !self.sockets[fd].active { return -9; }
        let s = &mut self.sockets[fd];
        if s.state != TcpState::Closed { return -106; } // EISCONN

        s.remote_ip   = remote_ip;
        s.remote_port = remote_port;
        s.local_port  = 49152 + (fd as u16 % 16383); // ephemeral port
        s.iss         = generate_isn();
        s.snd_nxt     = s.iss + 1;
        s.snd_una     = s.iss;
        s.state       = TcpState::SynSent;

        // Send SYN packet
        self.send_segment(fd, TCP_FLAG_SYN, s.iss, 0, &[]);
        0
    }

    /// listen(): passive open
    pub fn listen(&mut self, fd: usize, local_port: u16) -> i32 {
        if fd >= MAX_TCP_SOCKETS || !self.sockets[fd].active { return -9; }
        let s = &mut self.sockets[fd];
        if s.state != TcpState::Closed { return -22; }
        s.local_port = local_port;
        s.state = TcpState::Listen;
        0
    }

    /// Process incoming TCP segment
    pub fn rx_segment(&mut self, fd: usize, hdr: &TcpHdr, payload: &[u8]) -> i32 {
        if fd >= MAX_TCP_SOCKETS { return -9; }
        let flags = hdr.flags;
        let seq   = u32::from_be(hdr.seq);
        let ack   = u32::from_be(hdr.ack);
        let win   = u32::from(u16::from_be(hdr.window));

        let s = &mut self.sockets[fd];

        match s.state {
            TcpState::SynSent => {
                // Expect SYN+ACK
                if flags & (TCP_FLAG_SYN | TCP_FLAG_ACK) == (TCP_FLAG_SYN | TCP_FLAG_ACK) {
                    s.irs     = seq;
                    s.rcv_nxt = seq + 1;
                    s.snd_una = ack;
                    s.snd_wnd = win;
                    s.state   = TcpState::Established;
                    // Send ACK
                    let seq_out = s.snd_nxt;
                    let ack_out = s.rcv_nxt;
                    self.send_segment(fd, TCP_FLAG_ACK, seq_out, ack_out, &[]);
                } else if flags & TCP_FLAG_RST != 0 {
                    self.sockets[fd].state = TcpState::Closed;
                }
            }

            TcpState::Listen => {
                // Expect SYN (accept the connection)
                if flags & TCP_FLAG_SYN != 0 {
                    s.remote_ip   = u32::from_be(hdr.src_port as u32); // simplified
                    s.remote_port = u16::from_be(hdr.src_port);
                    s.irs         = seq;
                    s.rcv_nxt     = seq + 1;
                    s.iss         = generate_isn();
                    s.snd_nxt     = s.iss + 1;
                    s.snd_una     = s.iss;
                    s.state       = TcpState::SynRcvd;
                    // Send SYN+ACK
                    let seq_out = s.iss;
                    let ack_out = s.rcv_nxt;
                    self.send_segment(fd, TCP_FLAG_SYN | TCP_FLAG_ACK, seq_out, ack_out, &[]);
                }
            }

            TcpState::SynRcvd => {
                if flags & TCP_FLAG_ACK != 0 && ack == self.sockets[fd].snd_nxt {
                    self.sockets[fd].state = TcpState::Established;
                }
            }

            TcpState::Established => {
                // Process ACK
                if flags & TCP_FLAG_ACK != 0 {
                    let s = &mut self.sockets[fd];
                    if ack > s.snd_una { s.snd_una = ack; }
                    s.snd_wnd = win;
                }
                // Deliver payload
                if !payload.is_empty() {
                    let s = &mut self.sockets[fd];
                    if seq == s.rcv_nxt {
                        s.rx_push(payload);
                        s.rcv_nxt = s.rcv_nxt.wrapping_add(payload.len() as u32);
                        // Send ACK
                        let seq_out = s.snd_nxt;
                        let ack_out = s.rcv_nxt;
                        self.send_segment(fd, TCP_FLAG_ACK, seq_out, ack_out, &[]);
                    }
                    // Out-of-order: drop (simplified — no reorder buffer)
                }
                // FIN from remote
                let s = &mut self.sockets[fd];
                if flags & TCP_FLAG_FIN != 0 {
                    s.rcv_nxt = s.rcv_nxt.wrapping_add(1);
                    s.state   = TcpState::CloseWait;
                    let seq_out = s.snd_nxt;
                    let ack_out = s.rcv_nxt;
                    self.send_segment(fd, TCP_FLAG_ACK, seq_out, ack_out, &[]);
                }
            }

            TcpState::FinWait1 => {
                if flags & TCP_FLAG_ACK != 0 {
                    self.sockets[fd].state = TcpState::FinWait2;
                }
                if flags & TCP_FLAG_FIN != 0 {
                    let s = &mut self.sockets[fd];
                    s.rcv_nxt = s.rcv_nxt.wrapping_add(1);
                    s.state   = TcpState::TimeWait;
                    let seq_out = s.snd_nxt;
                    let ack_out = s.rcv_nxt;
                    self.send_segment(fd, TCP_FLAG_ACK, seq_out, ack_out, &[]);
                }
            }

            TcpState::FinWait2 => {
                if flags & TCP_FLAG_FIN != 0 {
                    let s = &mut self.sockets[fd];
                    s.rcv_nxt = s.rcv_nxt.wrapping_add(1);
                    s.state   = TcpState::TimeWait;
                    let seq_out = s.snd_nxt;
                    let ack_out = s.rcv_nxt;
                    self.send_segment(fd, TCP_FLAG_ACK, seq_out, ack_out, &[]);
                }
            }

            TcpState::LastAck => {
                if flags & TCP_FLAG_ACK != 0 {
                    self.sockets[fd].state = TcpState::Closed;
                    self.sockets[fd].active = false;
                }
            }

            _ => {}
        }
        0
    }

    /// Initiate active close (send FIN)
    pub fn close(&mut self, fd: usize) -> i32 {
        if fd >= MAX_TCP_SOCKETS || !self.sockets[fd].active { return -9; }
        let s = &mut self.sockets[fd];
        match s.state {
            TcpState::Established => {
                let seq = s.snd_nxt;
                s.snd_nxt = s.snd_nxt.wrapping_add(1);
                s.state = TcpState::FinWait1;
                let ack = s.rcv_nxt;
                self.send_segment(fd, TCP_FLAG_FIN | TCP_FLAG_ACK, seq, ack, &[]);
            }
            TcpState::CloseWait => {
                let seq = self.sockets[fd].snd_nxt;
                self.sockets[fd].snd_nxt = seq.wrapping_add(1);
                self.sockets[fd].state = TcpState::LastAck;
                let ack = self.sockets[fd].rcv_nxt;
                self.send_segment(fd, TCP_FLAG_FIN | TCP_FLAG_ACK, seq, ack, &[]);
            }
            _ => { self.sockets[fd].state = TcpState::Closed; }
        }
        0
    }

    /// send(): queue data into TX buffer
    pub fn send(&mut self, fd: usize, data: &[u8]) -> i64 {
        if fd >= MAX_TCP_SOCKETS || !self.sockets[fd].active { return -9; }
        if self.sockets[fd].state != TcpState::Established { return -107; } // ENOTCONN
        let pushed = self.sockets[fd].tx_push(data) as i64;
        // Flush TX immediately (simplified — no Nagle algorithm)
        self.flush_tx(fd);
        pushed
    }

    /// recv(): read from RX buffer
    pub fn recv(&mut self, fd: usize, buf: &mut [u8]) -> i64 {
        if fd >= MAX_TCP_SOCKETS || !self.sockets[fd].active { return -9; }
        let n = self.sockets[fd].rx_pop(buf);
        if n == 0 && self.sockets[fd].state == TcpState::CloseWait {
            return 0; // EOF
        }
        n as i64
    }

    fn flush_tx(&mut self, fd: usize) {
        let s = &mut self.sockets[fd];
        while s.tx_head != s.tx_tail {
            let avail = if s.tx_tail > s.tx_head {
                s.tx_tail - s.tx_head
            } else {
                TCP_TX_BUF - s.tx_head
            };
            let send_sz = avail.min(MSS as usize);
            let payload = &s.tx_buf[s.tx_head..s.tx_head + send_sz];

            let seq = s.snd_nxt;
            let ack = s.rcv_nxt;
            let flags = TCP_FLAG_PSH | TCP_FLAG_ACK;

            // Copy payload to static buffer to avoid borrow issues
            let mut tmp = [0u8; 1460];
            tmp[..send_sz].copy_from_slice(payload);

            s.snd_nxt = s.snd_nxt.wrapping_add(send_sz as u32);
            s.tx_head = (s.tx_head + send_sz) % TCP_TX_BUF;

            self.send_segment(fd, flags, seq, ack, &tmp[..send_sz]);
        }
    }

    // ── Checksum (RFC 793 §3.1) ──────────────────────────────────────────
    fn checksum(src_ip: u32, dst_ip: u32, data: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        // Pseudo header: src_ip, dst_ip, 0, protocol=6, tcp_length
        sum += src_ip >> 16;
        sum += src_ip & 0xFFFF;
        sum += dst_ip >> 16;
        sum += dst_ip & 0xFFFF;
        sum += 6; // TCP protocol
        sum += data.len() as u32;
        // TCP header + data
        let mut i = 0;
        while i + 1 < data.len() {
            sum += u32::from(data[i]) << 8 | u32::from(data[i+1]);
            i += 2;
        }
        if data.len() % 2 != 0 { sum += u32::from(data[data.len()-1]) << 8; }
        while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
        !(sum as u16)
    }

    fn send_segment(&mut self, fd: usize, flags: u8, seq: u32, ack: u32, payload: &[u8]) {
        let s = &self.sockets[fd];
        // Build TCP header
        let hdr = TcpHdr {
            src_port: u16::to_be(s.local_port),
            dst_port: u16::to_be(s.remote_port),
            seq:      u32::to_be(seq),
            ack:      u32::to_be(ack),
            data_off: 0x50, // 5 × 4 = 20 bytes, no options
            flags,
            window:   u16::to_be(s.rcv_wnd as u16),
            checksum: 0,
            urgent:   0,
        };

        // Serialize header + payload
        let hdr_bytes = unsafe {
            core::slice::from_raw_parts(
                &hdr as *const TcpHdr as *const u8,
                core::mem::size_of::<TcpHdr>(),
            )
        };

        // Compute checksum
        let mut combined = [0u8; 1500];
        let total = hdr_bytes.len() + payload.len();
        combined[..hdr_bytes.len()].copy_from_slice(hdr_bytes);
        combined[hdr_bytes.len()..total].copy_from_slice(payload);
        let csum = Self::checksum(s.local_ip, s.remote_ip, &combined[..total]);

        // Write checksum into combined buffer
        combined[16] = (csum >> 8) as u8;
        combined[17] = (csum & 0xFF) as u8;

        // Hand to IP layer
        unsafe {
            extern "C" { fn sigma_ip_send(dst: u32, proto: u8, data: *const u8, len: usize) -> i32; }
            sigma_ip_send(s.remote_ip, 6, combined.as_ptr(), total);
        }
    }
}

// ── Global TCP stack ──────────────────────────────────────────────────────
static mut G_TCP: TcpStack = TcpStack::new();

#[no_mangle]
pub unsafe extern "C" fn tcp_socket_create() -> i32 { G_TCP.socket_create() }

#[no_mangle]
pub unsafe extern "C" fn tcp_connect(fd: usize, ip: u32, port: u16) -> i32 {
    G_TCP.connect(fd, ip, port)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_listen(fd: usize, port: u16) -> i32 {
    G_TCP.listen(fd, port)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_close(fd: usize) -> i32 { G_TCP.close(fd) }

#[no_mangle]
pub unsafe extern "C" fn tcp_send(fd: usize, buf: *const u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    G_TCP.send(fd, core::slice::from_raw_parts(buf, len))
}

#[no_mangle]
pub unsafe extern "C" fn tcp_recv(fd: usize, buf: *mut u8, len: usize) -> i64 {
    if buf.is_null() { return -14; }
    G_TCP.recv(fd, core::slice::from_raw_parts_mut(buf, len))
}

#[no_mangle]
pub unsafe extern "C" fn tcp_rx_segment(
    fd: usize, hdr: *const TcpHdr, payload: *const u8, payload_len: usize,
) -> i32 {
    if hdr.is_null() { return -14; }
    let h = &*hdr;
    let p = if payload.is_null() || payload_len == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(payload, payload_len)
    };
    G_TCP.rx_segment(fd, h, p)
}
