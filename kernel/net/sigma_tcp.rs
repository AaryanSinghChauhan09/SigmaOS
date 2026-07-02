// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_tcp.rs — TCP State Machine (cleanroom, no_std)
// Language: Rust #![no_std] — no libc, no alloc, no third-party crates
// Pattern: OOP via TcpSocket struct implementing Socket trait

#![no_std]

// ── TCP State Machine ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TcpState {
    Closed, Listen, SynSent, SynReceived,
    Established, FinWait1, FinWait2,
    CloseWait, Closing, LastAck, TimeWait,
}

// ── TCP Flags ─────────────────────────────────────────────────────────────────

pub struct TcpFlags(pub u8);
impl TcpFlags {
    pub const FIN: u8 = 1 << 0;
    pub const SYN: u8 = 1 << 1;
    pub const RST: u8 = 1 << 2;
    pub const PSH: u8 = 1 << 3;
    pub const ACK: u8 = 1 << 4;
    pub const URG: u8 = 1 << 5;
}

// ── TCP Header ────────────────────────────────────────────────────────────────

pub struct TcpHeader<'a> { raw: &'a [u8] }

impl<'a> TcpHeader<'a> {
    pub fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < 20 { None } else { Some(Self { raw }) }
    }
    pub fn src_port(&self) -> u16 { u16::from_be_bytes([self.raw[0], self.raw[1]]) }
    pub fn dst_port(&self) -> u16 { u16::from_be_bytes([self.raw[2], self.raw[3]]) }
    pub fn seq(&self)       -> u32 { u32::from_be_bytes(self.raw[4..8].try_into().unwrap()) }
    pub fn ack(&self)       -> u32 { u32::from_be_bytes(self.raw[8..12].try_into().unwrap()) }
    pub fn data_off(&self)  -> usize { ((self.raw[12] >> 4) * 4) as usize }
    pub fn flags(&self)     -> u8  { self.raw[13] }
    pub fn window(&self)    -> u16 { u16::from_be_bytes([self.raw[14], self.raw[15]]) }
    pub fn payload(&self)   -> &[u8] { &self.raw[self.data_off()..] }
    pub fn has_syn(&self)   -> bool { self.flags() & TcpFlags::SYN != 0 }
    pub fn has_ack(&self)   -> bool { self.flags() & TcpFlags::ACK != 0 }
    pub fn has_fin(&self)   -> bool { self.flags() & TcpFlags::FIN != 0 }
    pub fn has_rst(&self)   -> bool { self.flags() & TcpFlags::RST != 0 }
}

// ── TCP Socket ────────────────────────────────────────────────────────────────

const RX_BUF: usize = 65536;
const TX_BUF: usize = 65536;

pub struct TcpSocket {
    pub state:    TcpState,
    pub local_port:  u16,
    pub remote_port: u16,
    pub local_ip:    [u8; 4],
    pub remote_ip:   [u8; 4],
    // Sequence numbers
    pub snd_nxt: u32, // next byte to send
    pub snd_una: u32, // oldest unacked byte
    pub rcv_nxt: u32, // next expected receive
    pub rcv_wnd: u16, // receive window
    // Buffers
    rx_buf:  [u8; RX_BUF],
    rx_head: usize,
    rx_tail: usize,
    tx_buf:  [u8; TX_BUF],
    tx_head: usize,
    tx_tail: usize,
}

impl TcpSocket {
    pub const fn new() -> Self {
        Self {
            state: TcpState::Closed,
            local_port: 0, remote_port: 0,
            local_ip: [0;4], remote_ip: [0;4],
            snd_nxt: 0, snd_una: 0, rcv_nxt: 0, rcv_wnd: RX_BUF as u16,
            rx_buf: [0u8; RX_BUF], rx_head: 0, rx_tail: 0,
            tx_buf: [0u8; TX_BUF], tx_head: 0, tx_tail: 0,
        }
    }

    /// Process an incoming TCP segment and update state machine
    pub fn process_segment(&mut self, hdr: &TcpHeader<'_>) -> TcpAction {
        match self.state {
            TcpState::Listen => {
                if hdr.has_syn() && !hdr.has_ack() {
                    self.rcv_nxt = hdr.seq().wrapping_add(1);
                    self.remote_port = hdr.src_port();
                    self.state = TcpState::SynReceived;
                    return TcpAction::SendSynAck;
                }
                TcpAction::Drop
            }
            TcpState::SynReceived => {
                if hdr.has_ack() && !hdr.has_syn() {
                    if hdr.ack() == self.snd_nxt {
                        self.snd_una = hdr.ack();
                        self.state = TcpState::Established;
                        return TcpAction::Established;
                    }
                }
                TcpAction::Drop
            }
            TcpState::SynSent => {
                if hdr.has_syn() && hdr.has_ack() {
                    self.rcv_nxt = hdr.seq().wrapping_add(1);
                    self.snd_una = hdr.ack();
                    self.state = TcpState::Established;
                    return TcpAction::SendAck;
                }
                TcpAction::Drop
            }
            TcpState::Established => {
                if hdr.has_rst() { self.state = TcpState::Closed; return TcpAction::Reset; }
                if hdr.has_fin() {
                    self.rcv_nxt = hdr.seq().wrapping_add(1);
                    self.state = TcpState::CloseWait;
                    return TcpAction::SendAck;
                }
                // Data segment
                let payload = hdr.payload();
                if !payload.is_empty() {
                    let written = self.rx_write(payload);
                    self.rcv_nxt = self.rcv_nxt.wrapping_add(written as u32);
                    return TcpAction::SendAck;
                }
                TcpAction::None
            }
            TcpState::FinWait1 => {
                if hdr.has_ack() { self.state = TcpState::FinWait2; }
                TcpAction::None
            }
            TcpState::FinWait2 => {
                if hdr.has_fin() {
                    self.state = TcpState::TimeWait;
                    return TcpAction::SendAck;
                }
                TcpAction::None
            }
            TcpState::CloseWait => TcpAction::None,
            _ => TcpAction::Drop,
        }
    }

    pub fn send(&mut self, data: &[u8]) -> usize { self.tx_write(data) }

    pub fn recv(&mut self, buf: &mut [u8]) -> usize {
        let n = buf.len().min(self.rx_available());
        for i in 0..n {
            buf[i] = self.rx_buf[self.rx_head];
            self.rx_head = (self.rx_head + 1) % RX_BUF;
        }
        n
    }

    pub fn close(&mut self) {
        if self.state == TcpState::Established {
            self.state = TcpState::FinWait1;
        }
    }

    pub fn is_connected(&self) -> bool { self.state == TcpState::Established }
    pub fn rx_available(&self) -> usize {
        (self.rx_tail + RX_BUF - self.rx_head) % RX_BUF
    }

    fn rx_write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let next = (self.rx_tail + 1) % RX_BUF;
            if next == self.rx_head { break; }
            self.rx_buf[self.rx_tail] = b;
            self.rx_tail = next;
            written += 1;
        }
        written
    }

    fn tx_write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            let next = (self.tx_tail + 1) % TX_BUF;
            if next == self.tx_head { break; }
            self.tx_buf[self.tx_tail] = b;
            self.tx_tail = next;
            written += 1;
        }
        written
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpAction { None, Drop, SendSynAck, SendAck, Established, Reset }
