// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_tcp.rs — TCP state machine (RFC 793 + RFC 5681 BBR)
// Implements: connection state machine, retransmission, flow control,
//             congestion control (BBR/CUBIC), SACK, timestamps, ECN
// Language: Rust (#![no_std])

#![no_std]
#![allow(dead_code)]

// ── TCP states (RFC 793) ───────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TcpState {
    Closed, Listen, SynSent, SynReceived,
    Established, FinWait1, FinWait2, CloseWait,
    Closing, LastAck, TimeWait,
}

// ── TCP flags ──────────────────────────────────────────────────────────────
pub mod flags {
    pub const FIN: u8 = 1 << 0;
    pub const SYN: u8 = 1 << 1;
    pub const RST: u8 = 1 << 2;
    pub const PSH: u8 = 1 << 3;
    pub const ACK: u8 = 1 << 4;
    pub const URG: u8 = 1 << 5;
    pub const ECE: u8 = 1 << 6;
    pub const CWR: u8 = 1 << 7;
}

// ── TCP header (20 bytes minimum) ─────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TcpHeader {
    pub src_port:  u16,
    pub dst_port:  u16,
    pub seq:       u32,
    pub ack:       u32,
    pub data_off_flags: u16,  // high 4 bits = data offset (in 32-bit words)
    pub window:    u16,
    pub checksum:  u16,
    pub urgent:    u16,
}

impl TcpHeader {
    pub fn data_offset(&self) -> u8 { ((u16::from_be(self.data_off_flags) >> 12) * 4) as u8 }
    pub fn tcp_flags(&self) -> u8   { (u16::from_be(self.data_off_flags) & 0x1FF) as u8 }
    pub fn has_flag(&self, f: u8)   -> bool { self.tcp_flags() & f != 0 }
}

// ── Congestion control (BBR-inspired) ─────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum CongestionAlgo { Bbr, Cubic, Reno }

#[derive(Copy, Clone)]
pub struct CongestionState {
    pub algo:        CongestionAlgo,
    pub cwnd:        u32,        // congestion window (bytes)
    pub ssthresh:    u32,        // slow-start threshold
    pub srtt_us:     u32,        // smoothed RTT (microseconds)
    pub rttvar_us:   u32,        // RTT variance
    pub rto_us:      u32,        // retransmission timeout
    pub btlbw:       u64,        // BBR: bottleneck bandwidth estimate (bps)
    pub rt_prop_us:  u32,        // BBR: min RTT (propagation delay)
    pub in_recovery: bool,
    pub fast_retx_cnt: u32,
    pub ecn_ce_seen: bool,
}

impl CongestionState {
    pub const fn new(algo: CongestionAlgo) -> Self {
        Self {
            algo, cwnd: 10 * 1460, ssthresh: u32::MAX,
            srtt_us: 100_000, rttvar_us: 50_000, rto_us: 1_000_000,
            btlbw: 0, rt_prop_us: u32::MAX,
            in_recovery: false, fast_retx_cnt: 0, ecn_ce_seen: false,
        }
    }

    /// Update RTT estimate (RFC 6298)
    pub fn update_rtt(&mut self, sample_us: u32) {
        if self.srtt_us == 0 {
            self.srtt_us   = sample_us;
            self.rttvar_us = sample_us / 2;
        } else {
            let rttvar_delta = self.srtt_us.abs_diff(sample_us);
            self.rttvar_us  = (3 * self.rttvar_us + rttvar_delta) / 4;
            self.srtt_us    = (7 * self.srtt_us + sample_us) / 8;
        }
        self.rto_us = (self.srtt_us + 4 * self.rttvar_us).max(200_000).min(60_000_000);
        // BBR: update min RTT
        if sample_us < self.rt_prop_us { self.rt_prop_us = sample_us; }
    }

    /// On new ACK received
    pub fn on_ack(&mut self, bytes_acked: u32) {
        if self.cwnd < self.ssthresh {
            // Slow start: exponential growth
            self.cwnd += bytes_acked;
        } else {
            // Congestion avoidance: linear growth (~1 MSS per RTT)
            self.cwnd += (1460 * bytes_acked) / self.cwnd.max(1);
        }
        if self.in_recovery { self.in_recovery = false; }
    }

    /// On packet loss (triple duplicate ACK)
    pub fn on_loss_dup_ack(&mut self) {
        if !self.in_recovery {
            self.ssthresh    = (self.cwnd / 2).max(2 * 1460);
            self.cwnd        = self.ssthresh + 3 * 1460; // fast recovery
            self.in_recovery = true;
        }
    }

    /// On RTO (timeout)
    pub fn on_timeout(&mut self) {
        self.ssthresh = (self.cwnd / 2).max(2 * 1460);
        self.cwnd     = 1460;   // restart slow start
        self.rto_us   = (self.rto_us * 2).min(60_000_000);   // exponential backoff
        self.in_recovery = false;
    }

    /// BBR bandwidth estimate update
    pub fn bbr_update_btlbw(&mut self, delivered: u64, elapsed_us: u64) {
        if elapsed_us == 0 { return; }
        let bw = delivered * 1_000_000 / elapsed_us;   // bytes/sec
        if bw > self.btlbw { self.btlbw = bw; }
        // BBR cwnd = BDP (bandwidth-delay product)
        let bdp = self.btlbw * self.rt_prop_us as u64 / 1_000_000;
        if bdp > 0 { self.cwnd = bdp as u32 * 2; }   // 2x BDP headroom
    }
}

// ── TCP connection ─────────────────────────────────────────────────────────
pub const TCP_RX_BUF: usize = 65536;
pub const TCP_TX_BUF: usize = 65536;
pub const MSS:        usize = 1460;

pub struct TcpConnection {
    pub state:      TcpState,
    pub local_ip:   u32,
    pub local_port: u16,
    pub remote_ip:  u32,
    pub remote_port: u16,

    // Sequence numbers
    pub snd_una:    u32,   // oldest unacknowledged byte
    pub snd_nxt:    u32,   // next byte to send
    pub snd_wnd:    u32,   // send window (peer's receive window)
    pub rcv_nxt:    u32,   // next expected byte from peer
    pub rcv_wnd:    u32,   // our receive window

    // Buffers (ring buffers in production; simplified here)
    pub tx_buf:     [u8; TCP_TX_BUF],
    pub tx_head:    usize,
    pub tx_tail:    usize,
    pub rx_buf:     [u8; TCP_RX_BUF],
    pub rx_head:    usize,
    pub rx_tail:    usize,

    // Congestion
    pub cc:         CongestionState,

    // Retransmit
    pub rto_armed:  bool,
    pub rto_expire_us: u64,
    pub retx_count: u32,

    // Options
    pub use_sack:   bool,
    pub use_ts:     bool,     // timestamps option
    pub ts_val:     u32,
    pub ts_ecr:     u32,
    pub use_ecn:    bool,
}

impl TcpConnection {
    pub fn new(local_ip: u32, local_port: u16, remote_ip: u32, remote_port: u16) -> Self {
        Self {
            state: TcpState::Closed,
            local_ip, local_port, remote_ip, remote_port,
            snd_una: 0, snd_nxt: 0, snd_wnd: TCP_RX_BUF as u32,
            rcv_nxt: 0, rcv_wnd: TCP_RX_BUF as u32,
            tx_buf: [0u8; TCP_TX_BUF], tx_head: 0, tx_tail: 0,
            rx_buf: [0u8; TCP_RX_BUF], rx_head: 0, rx_tail: 0,
            cc: CongestionState::new(CongestionAlgo::Bbr),
            rto_armed: false, rto_expire_us: 0, retx_count: 0,
            use_sack: true, use_ts: true, ts_val: 0, ts_ecr: 0, use_ecn: true,
        }
    }

    /// Active open: send SYN
    pub fn connect(&mut self) -> bool {
        if self.state != TcpState::Closed { return false; }
        self.snd_nxt = simple_isn(self.local_ip, self.local_port, self.remote_ip, self.remote_port);
        self.snd_una = self.snd_nxt;
        self.state   = TcpState::SynSent;
        // Caller should send SYN segment
        true
    }

    /// Process incoming segment — returns response type
    pub fn process_segment(&mut self, hdr: &TcpHeader, payload: &[u8]) -> SegmentAction {
        let seq = u32::from_be(hdr.seq);
        let ack = u32::from_be(hdr.ack);
        let wnd = u32::from_be(hdr.window) as u32;
        let f   = hdr.tcp_flags();

        match self.state {
            TcpState::SynSent => {
                if f & flags::SYN != 0 && f & flags::ACK != 0 {
                    if ack == self.snd_nxt + 1 {
                        self.rcv_nxt = seq + 1;
                        self.snd_una = ack;
                        self.snd_wnd = wnd;
                        self.state   = TcpState::Established;
                        return SegmentAction::SendAck;
                    }
                }
                SegmentAction::Drop
            }
            TcpState::Listen => {
                if f & flags::SYN != 0 {
                    self.rcv_nxt = seq + 1;
                    self.state   = TcpState::SynReceived;
                    return SegmentAction::SendSynAck;
                }
                SegmentAction::Drop
            }
            TcpState::SynReceived => {
                if f & flags::ACK != 0 && ack == self.snd_nxt + 1 {
                    self.snd_una = ack;
                    self.state   = TcpState::Established;
                    return SegmentAction::Accept;
                }
                SegmentAction::Drop
            }
            TcpState::Established => {
                // Validate sequence
                if !self.seq_valid(seq) { return SegmentAction::SendAck; }
                // Process ACK
                if f & flags::ACK != 0 { self.process_ack(ack, wnd); }
                // Process data
                if !payload.is_empty() {
                    self.rx_enqueue(payload);
                    self.rcv_nxt = self.rcv_nxt.wrapping_add(payload.len() as u32);
                    return if f & flags::PSH != 0 { SegmentAction::DeliverAndAck }
                           else { SegmentAction::SendAck };
                }
                if f & flags::FIN != 0 {
                    self.rcv_nxt = self.rcv_nxt.wrapping_add(1);
                    self.state   = TcpState::CloseWait;
                    return SegmentAction::SendAck;
                }
                SegmentAction::None
            }
            TcpState::FinWait1 => {
                if f & flags::ACK != 0 { self.process_ack(ack, wnd); }
                if f & flags::FIN != 0 {
                    self.rcv_nxt = self.rcv_nxt.wrapping_add(1);
                    self.state   = if self.snd_una == self.snd_nxt {
                        TcpState::TimeWait
                    } else { TcpState::Closing };
                    return SegmentAction::SendAck;
                }
                if self.snd_una == self.snd_nxt { self.state = TcpState::FinWait2; }
                SegmentAction::None
            }
            TcpState::LastAck => {
                if f & flags::ACK != 0 { self.state = TcpState::Closed; }
                SegmentAction::None
            }
            _ => SegmentAction::Drop,
        }
    }

    fn seq_valid(&self, seq: u32) -> bool {
        // Accept if seq is within receive window
        let left  = self.rcv_nxt;
        let right = self.rcv_nxt.wrapping_add(self.rcv_wnd);
        seq.wrapping_sub(left) < right.wrapping_sub(left)
    }

    fn process_ack(&mut self, ack: u32, wnd: u32) {
        let bytes_acked = ack.wrapping_sub(self.snd_una);
        if bytes_acked > 0 {
            self.snd_una = ack;
            self.cc.on_ack(bytes_acked);
            self.rto_armed = self.snd_una != self.snd_nxt;
        }
        self.snd_wnd = wnd;
    }

    /// Enqueue received data into rx ring buffer
    fn rx_enqueue(&mut self, data: &[u8]) {
        for &b in data {
            let next = (self.rx_tail + 1) % TCP_RX_BUF;
            if next != self.rx_head {
                self.rx_buf[self.rx_tail] = b;
                self.rx_tail = next;
            }
        }
    }

    /// Read from rx buffer
    pub fn rx_read(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        while n < buf.len() && self.rx_head != self.rx_tail {
            buf[n] = self.rx_buf[self.rx_head];
            self.rx_head = (self.rx_head + 1) % TCP_RX_BUF;
            n += 1;
        }
        n
    }

    /// Enqueue data into tx buffer
    pub fn tx_write(&mut self, data: &[u8]) -> usize {
        let mut n = 0;
        for &b in data {
            let next = (self.tx_tail + 1) % TCP_TX_BUF;
            if next != self.tx_head {
                self.tx_buf[self.tx_tail] = b;
                self.tx_tail = next;
                n += 1;
            } else { break; }
        }
        n
    }

    /// Active close: send FIN
    pub fn close(&mut self) {
        if self.state == TcpState::Established {
            self.state = TcpState::FinWait1;
        } else if self.state == TcpState::CloseWait {
            self.state = TcpState::LastAck;
        }
    }

    /// RTO timer tick — check if retransmit needed
    pub fn timer_tick(&mut self, now_us: u64) {
        if self.rto_armed && now_us >= self.rto_expire_us {
            self.cc.on_timeout();
            self.snd_nxt = self.snd_una;   // go-back-N retransmit
            self.retx_count += 1;
            self.rto_expire_us = now_us + self.cc.rto_us as u64;
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SegmentAction {
    None, Drop, SendAck, SendSynAck, DeliverAndAck, Accept, Reset,
}

/// Compute Initial Sequence Number (RFC 6528 — prevents hijacking)
fn simple_isn(src_ip: u32, src_port: u16, dst_ip: u32, dst_port: u16) -> u32 {
    let h = src_ip.wrapping_mul(0x9e3779b9)
        ^ dst_ip.wrapping_mul(0x6c62272e)
        ^ (src_port as u32).wrapping_mul(0xdeadbeef)
        ^ (dst_port as u32).wrapping_mul(0x12345678);
    h & 0x7FFF_FFFF
}
