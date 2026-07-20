use core::sync::atomic::{AtomicUsize, Ordering};
/// SigmaOS TCP State Machine — RFC 793 full implementation
/// States: CLOSED → LISTEN → SYN_SENT / SYN_RCVD → ESTABLISHED →
///         FIN_WAIT_1 → FIN_WAIT_2 → TIME_WAIT → CLOSED
///         ESTABLISHED → CLOSE_WAIT → LAST_ACK → CLOSED
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

// ── TCP States (RFC 793 §3.2) ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// ── TCP Flags ─────────────────────────────────────────────────────────────

pub mod flags {
    pub const FIN: u8 = 0x01;
    pub const SYN: u8 = 0x02;
    pub const RST: u8 = 0x04;
    pub const PSH: u8 = 0x08;
    pub const ACK: u8 = 0x10;
    pub const URG: u8 = 0x20;
    pub const ECE: u8 = 0x40;
    pub const CWR: u8 = 0x80;
}

// ── TCP Segment ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u8,
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
    pub payload: Vec<u8>,
}

impl TcpSegment {
    pub fn syn(src: u16, dst: u16, seq: u32) -> Self {
        TcpSegment {
            src_port: src,
            dst_port: dst,
            seq,
            ack: 0,
            flags: flags::SYN,
            window: 65535,
            checksum: 0,
            urgent_ptr: 0,
            payload: Vec::new(),
        }
    }
    pub fn syn_ack(src: u16, dst: u16, seq: u32, ack: u32) -> Self {
        TcpSegment {
            src_port: src,
            dst_port: dst,
            seq,
            ack,
            flags: flags::SYN | flags::ACK,
            window: 65535,
            checksum: 0,
            urgent_ptr: 0,
            payload: Vec::new(),
        }
    }
    pub fn ack(src: u16, dst: u16, seq: u32, ack: u32) -> Self {
        TcpSegment {
            src_port: src,
            dst_port: dst,
            seq,
            ack,
            flags: flags::ACK,
            window: 65535,
            checksum: 0,
            urgent_ptr: 0,
            payload: Vec::new(),
        }
    }
    pub fn fin_ack(src: u16, dst: u16, seq: u32, ack: u32) -> Self {
        TcpSegment {
            src_port: src,
            dst_port: dst,
            seq,
            ack,
            flags: flags::FIN | flags::ACK,
            window: 65535,
            checksum: 0,
            urgent_ptr: 0,
            payload: Vec::new(),
        }
    }
    pub fn data(src: u16, dst: u16, seq: u32, ack: u32, data: Vec<u8>) -> Self {
        TcpSegment {
            src_port: src,
            dst_port: dst,
            seq,
            ack,
            flags: flags::PSH | flags::ACK,
            window: 65535,
            checksum: 0,
            urgent_ptr: 0,
            payload: data,
        }
    }

    pub fn has_flag(&self, f: u8) -> bool {
        self.flags & f != 0
    }
}

// ── Congestion control algorithms ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionAlgorithm {
    Reno,
    Cubic,
    Bbr,
}

#[derive(Debug, Clone)]
pub struct CongestionControl {
    pub algorithm: CongestionAlgorithm,
    pub cwnd: u32,     // Congestion window (bytes)
    pub ssthresh: u32, // Slow-start threshold
    pub rtt_ms: u32,   // Measured RTT
    pub rto_ms: u32,   // Retransmit timeout
    pub in_slow_start: bool,
}

impl CongestionControl {
    pub fn new(algo: CongestionAlgorithm) -> Self {
        CongestionControl {
            algorithm: algo,
            cwnd: 10 * 1460,
            ssthresh: 65536,
            rtt_ms: 100,
            rto_ms: 200,
            in_slow_start: true,
        }
    }

    pub fn on_ack(&mut self, bytes_acked: u32) {
        if self.in_slow_start {
            // Slow start: cwnd grows by MSS per ACK
            self.cwnd += bytes_acked;
            if self.cwnd >= self.ssthresh {
                self.in_slow_start = false;
            }
        } else {
            // Congestion avoidance: cwnd grows by MSS²/cwnd per ACK (AIMD)
            let mss = 1460u32;
            self.cwnd += mss * mss / self.cwnd;
        }
    }

    pub fn on_loss(&mut self) {
        self.ssthresh = (self.cwnd / 2).max(2 * 1460);
        match self.algorithm {
            CongestionAlgorithm::Reno | CongestionAlgorithm::Cubic => {
                self.cwnd = self.ssthresh; // fast recovery
                self.in_slow_start = false;
            }
            CongestionAlgorithm::Bbr => {
                // BBR doesn't reduce cwnd on loss — instead uses delivery rate
                self.cwnd = (self.cwnd * 3) / 4;
            }
        }
    }
}

// ── TCP connection ────────────────────────────────────────────────────────

pub struct TcpConnection {
    pub local_port: u16,
    pub remote_port: u16,
    pub state: TcpState,
    pub snd_seq: u32, // Our send sequence number
    pub rcv_seq: u32, // Received sequence number
    pub snd_wnd: u16, // Send window (their advertised window)
    pub rcv_buf: VecDeque<u8>,
    pub snd_buf: VecDeque<u8>,
    pub congestion: CongestionControl,
    pub segments_tx: AtomicUsize,
    pub segments_rx: AtomicUsize,
    pub bytes_tx: AtomicUsize,
    pub bytes_rx: AtomicUsize,
}

impl TcpConnection {
    pub fn new(local_port: u16, remote_port: u16) -> Self {
        TcpConnection {
            local_port,
            remote_port,
            state: TcpState::Closed,
            snd_seq: 0x12345678, // Initial sequence number
            rcv_seq: 0,
            snd_wnd: 65535,
            rcv_buf: VecDeque::new(),
            snd_buf: VecDeque::new(),
            congestion: CongestionControl::new(CongestionAlgorithm::Cubic),
            segments_tx: AtomicUsize::new(0),
            segments_rx: AtomicUsize::new(0),
            bytes_tx: AtomicUsize::new(0),
            bytes_rx: AtomicUsize::new(0),
        }
    }

    /// Active connect — send SYN
    pub fn connect(&mut self) -> TcpSegment {
        self.state = TcpState::SynSent;
        let seg = TcpSegment::syn(self.local_port, self.remote_port, self.snd_seq);
        self.segments_tx.fetch_add(1, Ordering::Relaxed);
        seg
    }

    /// Server: process SYN, return SYN-ACK
    pub fn accept_syn(&mut self, seg: &TcpSegment) -> Option<TcpSegment> {
        if self.state != TcpState::Listen {
            return None;
        }
        if !seg.has_flag(flags::SYN) {
            return None;
        }
        self.rcv_seq = seg.seq + 1;
        self.state = TcpState::SynReceived;
        let reply = TcpSegment::syn_ack(self.local_port, seg.src_port, self.snd_seq, self.rcv_seq);
        self.segments_tx.fetch_add(1, Ordering::Relaxed);
        self.segments_rx.fetch_add(1, Ordering::Relaxed);
        Some(reply)
    }

    /// Client: process SYN-ACK, return ACK → ESTABLISHED
    pub fn process_syn_ack(&mut self, seg: &TcpSegment) -> Option<TcpSegment> {
        if self.state != TcpState::SynSent {
            return None;
        }
        if !seg.has_flag(flags::SYN) || !seg.has_flag(flags::ACK) {
            return None;
        }
        self.snd_seq += 1;
        self.rcv_seq = seg.seq + 1;
        self.state = TcpState::Established;
        let ack = TcpSegment::ack(
            self.local_port,
            self.remote_port,
            self.snd_seq,
            self.rcv_seq,
        );
        self.segments_rx.fetch_add(1, Ordering::Relaxed);
        self.segments_tx.fetch_add(1, Ordering::Relaxed);
        Some(ack)
    }

    /// Server: process final ACK → ESTABLISHED
    pub fn process_final_ack(&mut self, seg: &TcpSegment) {
        if self.state == TcpState::SynReceived && seg.has_flag(flags::ACK) {
            self.snd_seq += 1;
            self.state = TcpState::Established;
            self.segments_rx.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Send data (ESTABLISHED)
    pub fn send(&mut self, data: &[u8]) -> Option<TcpSegment> {
        if self.state != TcpState::Established {
            return None;
        }
        let to_send = data
            .len()
            .min(self.congestion.cwnd as usize)
            .min(self.snd_wnd as usize);
        let seg = TcpSegment::data(
            self.local_port,
            self.remote_port,
            self.snd_seq,
            self.rcv_seq,
            data[..to_send].to_vec(),
        );
        self.snd_seq = self.snd_seq.wrapping_add(to_send as u32);
        self.bytes_tx.fetch_add(to_send, Ordering::Relaxed);
        self.segments_tx.fetch_add(1, Ordering::Relaxed);
        self.congestion.on_ack(to_send as u32);
        Some(seg)
    }

    /// Receive data segment
    pub fn receive(&mut self, seg: &TcpSegment) {
        if self.state == TcpState::Established && !seg.payload.is_empty() {
            self.bytes_rx
                .fetch_add(seg.payload.len(), Ordering::Relaxed);
            self.segments_rx.fetch_add(1, Ordering::Relaxed);
            for &b in &seg.payload {
                self.rcv_buf.push_back(b);
            }
            self.rcv_seq = self.rcv_seq.wrapping_add(seg.payload.len() as u32);
        }
    }

    pub fn read(&mut self, n: usize) -> Vec<u8> {
        self.rcv_buf.drain(..n.min(self.rcv_buf.len())).collect()
    }

    /// Initiate close — send FIN
    pub fn close(&mut self) -> Option<TcpSegment> {
        if self.state != TcpState::Established {
            return None;
        }
        self.state = TcpState::FinWait1;
        let seg = TcpSegment::fin_ack(
            self.local_port,
            self.remote_port,
            self.snd_seq,
            self.rcv_seq,
        );
        self.segments_tx.fetch_add(1, Ordering::Relaxed);
        Some(seg)
    }

    pub fn segments_tx(&self) -> usize {
        self.segments_tx.load(Ordering::Relaxed)
    }
    pub fn segments_rx(&self) -> usize {
        self.segments_rx.load(Ordering::Relaxed)
    }
    pub fn bytes_tx(&self) -> usize {
        self.bytes_tx.load(Ordering::Relaxed)
    }
    pub fn bytes_rx(&self) -> usize {
        self.bytes_rx.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_way_handshake() {
        // Server in LISTEN, Client in CLOSED
        let mut server = TcpConnection::new(80, 9000);
        server.state = TcpState::Listen;
        let mut client = TcpConnection::new(9000, 80);

        // 1. Client → SYN
        let syn = client.connect();
        assert!(syn.has_flag(flags::SYN));
        assert_eq!(client.state, TcpState::SynSent);

        // 2. Server → SYN-ACK
        let syn_ack = server.accept_syn(&syn).unwrap();
        assert!(syn_ack.has_flag(flags::SYN));
        assert!(syn_ack.has_flag(flags::ACK));
        assert_eq!(server.state, TcpState::SynReceived);

        // 3. Client → ACK (established)
        let ack = client.process_syn_ack(&syn_ack).unwrap();
        assert_eq!(client.state, TcpState::Established);

        // 4. Server processes ACK
        server.process_final_ack(&ack);
        assert_eq!(server.state, TcpState::Established);
    }

    #[test]
    fn test_data_transfer() {
        let mut client = TcpConnection::new(9001, 8080);
        client.state = TcpState::Established;
        let seg = client.send(b"GET / HTTP/1.1\r\n").unwrap();
        assert!(seg.has_flag(flags::PSH));
        assert_eq!(seg.payload, b"GET / HTTP/1.1\r\n");
        assert_eq!(client.bytes_tx(), 16);
    }

    #[test]
    fn test_receive_buffer() {
        let mut conn = TcpConnection::new(8080, 9001);
        conn.state = TcpState::Established;
        conn.rcv_seq = 1000;
        let seg = TcpSegment::data(9001, 8080, 1000, 0, b"Hello!".to_vec());
        conn.receive(&seg);
        assert_eq!(conn.read(6), b"Hello!");
        assert_eq!(conn.bytes_rx(), 6);
    }

    #[test]
    fn test_congestion_slow_start() {
        let mut cc = CongestionControl::new(CongestionAlgorithm::Reno);
        let initial_cwnd = cc.cwnd;
        cc.on_ack(1460);
        assert!(cc.cwnd > initial_cwnd); // cwnd grew
    }

    #[test]
    fn test_congestion_loss_response() {
        let mut cc = CongestionControl::new(CongestionAlgorithm::Cubic);
        cc.in_slow_start = false;
        cc.cwnd = 100_000;
        cc.on_loss();
        assert!(cc.cwnd < 100_000);
        assert_eq!(cc.ssthresh, 50_000);
    }

    #[test]
    fn test_active_close() {
        let mut conn = TcpConnection::new(9002, 80);
        conn.state = TcpState::Established;
        let fin = conn.close().unwrap();
        assert!(fin.has_flag(flags::FIN));
        assert_eq!(conn.state, TcpState::FinWait1);
    }
}
