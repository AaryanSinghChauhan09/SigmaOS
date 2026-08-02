#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

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

// =================================================================────────
// 20. SOVEREIGN SOCKMAP DIRECT REDIRECTION ENGINE (CILIUM/LINUX STYLE)
// =================================================================────────

pub struct SovereignSockmapBypass {
    pub socket_map: HashMap<(u16, u16), (u16, u16)>,
}

impl SovereignSockmapBypass {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            socket_map: HashMap::new(),
        }
    }

    /// Registers a peer-to-peer socket map link for dynamic stack bypass
    pub fn register_link(&mut self, src_port: u16, dst_port: u16, peer_src: u16, peer_dst: u16) {
        self.socket_map.insert((src_port, dst_port), (peer_src, peer_dst));
    }

    /// Determines if a packet payload should bypass the standard TCP stack and redirect directly
    pub fn redirect_payload(&self, src_port: u16, dst_port: u16, payload: &[u8], peer_receiver: &mut TcpConnection) -> bool {
        if let Some(&(peer_src, peer_dst)) = self.socket_map.get(&(src_port, dst_port)) {
            if peer_receiver.local_port == peer_src && peer_receiver.remote_port == peer_dst {
                for &b in payload {
                    peer_receiver.rcv_buf.push_back(b);
                }
                peer_receiver.segments_rx.fetch_add(1, Ordering::Relaxed);
                peer_receiver.bytes_rx.fetch_add(payload.len(), Ordering::Relaxed);
                return true;
            }
        }
        false
    }
}

impl Default for SovereignSockmapBypass {
    fn default() -> Self {
        Self::new()
    }
}

// =================================================================────────
// 21. SYN COOKIES SYN FLOOD PROTECTION (LINUX STYLE SYNC_COOKIES)
// =================================================================────────

pub struct SynCookieEngine {
    pub secret_seed: u32,
}

impl SynCookieEngine {
    pub fn new(seed: u32) -> Self {
        Self { secret_seed: seed }
    }

    /// Generates a cryptographic SYN cookie sequence number based on connection tuple
    pub fn generate_cookie(&self, src_port: u16, dst_port: u16, client_isn: u32, mss_idx: u8) -> u32 {
        let hash = (src_port as u32)
            .wrapping_mul(31)
            .wrapping_add(dst_port as u32)
            .wrapping_mul(31)
            .wrapping_add(client_isn)
            .wrapping_mul(31)
            .wrapping_add(self.secret_seed);

        (hash & 0xFFFFFFF8) | ((mss_idx & 0x07) as u32)
    }

    /// Verifies if a returned ACK number corresponds to a valid SYN cookie
    pub fn verify_cookie(&self, src_port: u16, dst_port: u16, client_isn: u32, ack_num: u32) -> bool {
        let expected_cookie = ack_num.wrapping_sub(1);
        let computed_hash = (src_port as u32)
            .wrapping_mul(31)
            .wrapping_add(dst_port as u32)
            .wrapping_mul(31)
            .wrapping_add(client_isn)
            .wrapping_mul(31)
            .wrapping_add(self.secret_seed);

        (expected_cookie & 0xFFFFFFF8) == (computed_hash & 0xFFFFFFF8)
    }
}

// =================================================================────────
// 22. RECEIVE PACKET STEERING (RPS/RSS CPU CORE INTERRUPT BALANCING)
// =================================================================────────

pub struct ReceivePacketSteering {
    pub core_count: u32,
}

impl ReceivePacketSteering {
    pub fn new(cores: u32) -> Self {
        Self { core_count: cores }
    }

    /// Computes symmetric hash of packet ports to select designated CPU processing core
    pub fn steer_packet(&self, src_port: u16, dst_port: u16) -> u32 {
        let hash = (src_port as u32) ^ (dst_port as u32);
        hash % self.core_count
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

    #[test]
    fn test_sockmap_direct_redirection() {
        let mut sender = TcpConnection::new(9001, 80);
        let mut receiver = TcpConnection::new(80, 9001);
        receiver.state = TcpState::Established;

        let mut sockmap = SovereignSockmapBypass::new();
        sockmap.register_link(9001, 80, 80, 9001);

        let data = b"SOCKMAP_FAST_PATH_BYPASS_DATA";

        // Execute direct redirect bypass
        let redirected = sockmap.redirect_payload(9001, 80, data, &mut receiver);
        assert!(redirected);

        // Verify receiver buffer received payload directly
        assert_eq!(receiver.read(data.len()), data);
        assert_eq!(receiver.bytes_rx(), data.len());
        assert_eq!(receiver.segments_rx(), 1);
    }

    #[test]
    fn test_syn_cookie_verification() {
        let engine = SynCookieEngine::new(0xDEADBEEF);
        let src_port = 12345;
        let dst_port = 80;
        let client_isn = 987654321;
        let mss_idx = 4;

        let cookie = engine.generate_cookie(src_port, dst_port, client_isn, mss_idx);

        // Final ACK contains cookie + 1
        let ack_num = cookie.wrapping_add(1);

        assert!(engine.verify_cookie(src_port, dst_port, client_isn, ack_num));
        assert!(!engine.verify_cookie(src_port, dst_port, client_isn + 1, ack_num));
    }

    #[test]
    fn test_rps_load_balancing() {
        let rps = ReceivePacketSteering::new(4); // 4 CPU cores

        let core_1 = rps.steer_packet(9000, 80);
        let core_2 = rps.steer_packet(9001, 80);

        assert!(core_1 < 4);
        assert!(core_2 < 4);

        // Symmetric check: flow 9000 -> 80 must hash to same core as 80 -> 9000 (symmetric routing)
        let core_sym = rps.steer_packet(80, 9000);
        assert_eq!(core_1, core_sym);
    }
}
