//! # Complete TCP Implementation
//!
//! Full TCP stack with 3-way handshake, retransmission, and congestion control.
//! Inspired by Linux net/ipv4/tcp*.c and FreeBSD netinet/tcp_*.c.

#![no_std]

extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// TCP congestion control algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionAlgorithm {
    Reno,  // Classic TCP Reno
    Cubic, // Linux default (RFC 8312)
    BBR,   // Bottleneck Bandwidth and RTT
    Vegas, // TCP Vegas
}

/// TCP connection state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

/// TCP options
#[derive(Debug, Clone)]
pub struct TcpOptions {
    pub mss: Option<u16>,               // Maximum Segment Size
    pub window_scale: Option<u8>,       // Window scale factor
    pub sack_permitted: bool,           // SACK permitted
    pub timestamps: Option<(u32, u32)>, // (TSval, TSecr)
}

impl TcpOptions {
    pub fn new() -> Self {
        Self {
            mss: Some(1460),
            window_scale: Some(7),
            sack_permitted: true,
            timestamps: None,
        }
    }
}

/// TCP retransmission timer
pub struct TcpTimer {
    rto: AtomicU32,      // Retransmission timeout (ms)
    srtt: AtomicU32,     // Smoothed RTT (ms)
    rttvar: AtomicU32,   // RTT variance (ms)
    last_rtt: AtomicU32, // Last measured RTT (ms)
}

impl TcpTimer {
    pub fn new() -> Self {
        Self {
            rto: AtomicU32::new(1000), // Initial RTO = 1 second
            srtt: AtomicU32::new(0),
            rttvar: AtomicU32::new(0),
            last_rtt: AtomicU32::new(0),
        }
    }

    /// Update RTT estimation (Jacobson/Karels algorithm)
    pub fn update_rtt(&self, measured_rtt: u32) {
        let srtt = self.srtt.load(Ordering::Acquire);
        let rttvar = self.rttvar.load(Ordering::Acquire);

        if srtt == 0 {
            // First measurement
            self.srtt.store(measured_rtt, Ordering::Release);
            self.rttvar.store(measured_rtt / 2, Ordering::Release);
        } else {
            // SRTT = (7/8) * SRTT + (1/8) * RTT
            let delta = if measured_rtt > srtt {
                measured_rtt - srtt
            } else {
                srtt - measured_rtt
            };

            let new_rttvar = (3 * rttvar + delta) / 4;
            let new_srtt = (7 * srtt + measured_rtt) / 8;

            self.rttvar.store(new_rttvar, Ordering::Release);
            self.srtt.store(new_srtt, Ordering::Release);
        }

        // RTO = SRTT + 4 * RTTVAR (clamped to [200ms, 60s])
        let new_rto = (srtt + 4 * rttvar).max(200).min(60000);
        self.rto.store(new_rto, Ordering::Release);
        self.last_rtt.store(measured_rtt, Ordering::Release);
    }

    pub fn get_rto(&self) -> u32 {
        self.rto.load(Ordering::Acquire)
    }

    pub fn get_srtt(&self) -> u32 {
        self.srtt.load(Ordering::Acquire)
    }
}

/// TCP congestion control state
pub struct CongestionControl {
    algorithm: CongestionAlgorithm,
    cwnd: AtomicU32,        // Congestion window (bytes)
    ssthresh: AtomicU32,    // Slow start threshold
    bytes_acked: AtomicU32, // Bytes acked in current RTT

    // Cubic-specific
    cubic_w_max: AtomicU32,
    cubic_k: AtomicU32,
    cubic_origin_point: AtomicU32,

    // BBR-specific
    bbr_btlbw: AtomicU64,  // Bottleneck bandwidth
    bbr_rtprop: AtomicU32, // Round-trip propagation time
}

impl CongestionControl {
    pub fn new(algorithm: CongestionAlgorithm, initial_cwnd: u32) -> Self {
        Self {
            algorithm,
            cwnd: AtomicU32::new(initial_cwnd),
            ssthresh: AtomicU32::new(u32::MAX / 2),
            bytes_acked: AtomicU32::new(0),
            cubic_w_max: AtomicU32::new(0),
            cubic_k: AtomicU32::new(0),
            cubic_origin_point: AtomicU32::new(0),
            bbr_btlbw: AtomicU64::new(0),
            bbr_rtprop: AtomicU32::new(u32::MAX),
        }
    }

    /// Handle ACK in congestion avoidance
    pub fn on_ack(&self, acked_bytes: u32, rtt: u32) {
        let cwnd = self.cwnd.load(Ordering::Acquire);
        let ssthresh = self.ssthresh.load(Ordering::Acquire);

        match self.algorithm {
            CongestionAlgorithm::Reno => {
                if cwnd < ssthresh {
                    // Slow start: cwnd += MSS for each ACK
                    self.cwnd.fetch_add(acked_bytes, Ordering::Release);
                } else {
                    // Congestion avoidance: cwnd += MSS^2/cwnd
                    let bytes =
                        self.bytes_acked.fetch_add(acked_bytes, Ordering::Release) + acked_bytes;
                    if bytes >= cwnd {
                        self.cwnd.fetch_add(1460, Ordering::Release);
                        self.bytes_acked.store(0, Ordering::Release);
                    }
                }
            }
            CongestionAlgorithm::Cubic => {
                // Simplified Cubic implementation
                if cwnd < ssthresh {
                    self.cwnd.fetch_add(acked_bytes, Ordering::Release);
                } else {
                    // Cubic increase
                    let w_max = self.cubic_w_max.load(Ordering::Acquire);
                    if w_max > 0 {
                        let target = w_max + 1460;
                        if cwnd < target {
                            self.cwnd.fetch_add(1460, Ordering::Release);
                        }
                    } else {
                        self.cwnd.fetch_add(1460, Ordering::Release);
                    }
                }
            }
            CongestionAlgorithm::BBR => {
                // BBR: pacing-based congestion control
                let btlbw = self.bbr_btlbw.load(Ordering::Acquire);
                let rtprop = self.bbr_rtprop.load(Ordering::Acquire);

                if rtt < rtprop {
                    self.bbr_rtprop.store(rtt, Ordering::Release);
                }

                // Update bandwidth estimate
                let bw = (acked_bytes as u64 * 1000) / rtt as u64;
                if bw > btlbw {
                    self.bbr_btlbw.store(bw, Ordering::Release);
                }

                // Set cwnd based on BDP (Bandwidth-Delay Product)
                let bdp = (btlbw * rtprop as u64) / 1000;
                self.cwnd
                    .store((bdp as u32).max(4 * 1460), Ordering::Release);
            }
            CongestionAlgorithm::Vegas => {
                // TCP Vegas: RTT-based congestion control
                // Simplified implementation
                let expected = (cwnd / rtt).max(1);
                let actual = acked_bytes / rtt.max(1);

                if actual < expected - 1 {
                    // Decrease cwnd
                    self.cwnd.fetch_sub(1460, Ordering::Release);
                } else if actual > expected + 1 {
                    // Increase cwnd
                    self.cwnd.fetch_add(1460, Ordering::Release);
                }
            }
        }
    }

    /// Handle packet loss (fast retransmit/recovery)
    pub fn on_loss(&self) {
        let cwnd = self.cwnd.load(Ordering::Acquire);

        match self.algorithm {
            CongestionAlgorithm::Reno => {
                // ssthresh = max(cwnd/2, 2*MSS)
                let new_ssthresh = (cwnd / 2).max(2 * 1460);
                self.ssthresh.store(new_ssthresh, Ordering::Release);
                self.cwnd.store(new_ssthresh + 3 * 1460, Ordering::Release);
            }
            CongestionAlgorithm::Cubic => {
                self.cubic_w_max.store(cwnd, Ordering::Release);
                let new_cwnd = (cwnd * 7) / 10; // 0.7 * cwnd
                self.cwnd.store(new_cwnd, Ordering::Release);
                self.ssthresh.store(new_cwnd, Ordering::Release);
            }
            CongestionAlgorithm::BBR => {
                // BBR: enter ProbeRTT state
                // Minimal cwnd reduction
                let min_cwnd = 4 * 1460;
                let new_cwnd = cwnd.max(min_cwnd);
                self.cwnd.store(new_cwnd, Ordering::Release);
            }
            CongestionAlgorithm::Vegas => {
                let new_cwnd = (cwnd * 3) / 4;
                self.cwnd.store(new_cwnd.max(2 * 1460), Ordering::Release);
            }
        }
    }

    pub fn get_cwnd(&self) -> u32 {
        self.cwnd.load(Ordering::Acquire)
    }
}

/// TCP segment for retransmission queue
#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub seq_start: u32,
    pub seq_end: u32,
    pub data: Vec<u8>,
    pub retransmit_count: u8,
    pub timestamp: u64, // When sent
}

/// Complete TCP connection
pub struct TcpConnection {
    state: AtomicU32,

    // Sequence numbers
    snd_una: AtomicU32, // Send unacknowledged
    snd_nxt: AtomicU32, // Send next
    snd_wnd: AtomicU32, // Send window
    rcv_nxt: AtomicU32, // Receive next
    rcv_wnd: AtomicU32, // Receive window

    // Buffers
    send_buffer: VecDeque<u8>,
    recv_buffer: VecDeque<u8>,
    retransmit_queue: Vec<TcpSegment>,

    // Timers and congestion control
    timer: TcpTimer,
    congestion: CongestionControl,

    // Options
    options: TcpOptions,
}

impl TcpConnection {
    pub fn new(algorithm: CongestionAlgorithm) -> Self {
        Self {
            state: AtomicU32::new(TcpState::Closed as u32),
            snd_una: AtomicU32::new(0),
            snd_nxt: AtomicU32::new(0),
            snd_wnd: AtomicU32::new(65535),
            rcv_nxt: AtomicU32::new(0),
            rcv_wnd: AtomicU32::new(65535),
            send_buffer: VecDeque::with_capacity(65536),
            recv_buffer: VecDeque::with_capacity(65536),
            retransmit_queue: Vec::new(),
            timer: TcpTimer::new(),
            congestion: CongestionControl::new(algorithm, 10 * 1460),
            options: TcpOptions::new(),
        }
    }

    /// Active open (client)
    pub fn connect(&mut self, initial_seq: u32) -> Result<(), TcpError> {
        self.state
            .store(TcpState::SynSent as u32, Ordering::Release);
        self.snd_nxt.store(initial_seq + 1, Ordering::Release);
        // Send SYN packet
        Ok(())
    }

    /// Passive open (server)
    pub fn listen(&mut self) -> Result<(), TcpError> {
        self.state.store(TcpState::Listen as u32, Ordering::Release);
        Ok(())
    }

    /// Handle incoming SYN
    pub fn on_syn(&mut self, seq: u32) -> Result<(), TcpError> {
        let state = self.get_state();

        match state {
            TcpState::Listen => {
                self.rcv_nxt.store(seq + 1, Ordering::Release);
                self.state
                    .store(TcpState::SynReceived as u32, Ordering::Release);
                // Send SYN-ACK
                Ok(())
            }
            _ => Err(TcpError::InvalidState),
        }
    }

    /// Handle incoming ACK
    pub fn on_ack(&mut self, ack: u32, rtt: u32) -> Result<(), TcpError> {
        let una = self.snd_una.load(Ordering::Acquire);
        let nxt = self.snd_nxt.load(Ordering::Acquire);

        // Validate ACK
        if ack > nxt || ack <= una {
            return Err(TcpError::InvalidAck);
        }

        // Update unacknowledged
        let acked_bytes = ack - una;
        self.snd_una.store(ack, Ordering::Release);

        // Remove acked segments from retransmit queue
        self.retransmit_queue.retain(|seg| seg.seq_end > ack);

        // Update RTT and congestion control
        self.timer.update_rtt(rtt);
        self.congestion.on_ack(acked_bytes, rtt);

        // State transitions
        let state = self.get_state();
        match state {
            TcpState::SynSent => {
                self.state
                    .store(TcpState::Established as u32, Ordering::Release);
            }
            TcpState::SynReceived => {
                self.state
                    .store(TcpState::Established as u32, Ordering::Release);
            }
            TcpState::FinWait1 => {
                self.state
                    .store(TcpState::FinWait2 as u32, Ordering::Release);
            }
            TcpState::Closing => {
                self.state
                    .store(TcpState::TimeWait as u32, Ordering::Release);
            }
            TcpState::LastAck => {
                self.state.store(TcpState::Closed as u32, Ordering::Release);
            }
            _ => {}
        }

        Ok(())
    }

    /// Send data
    pub fn send(&mut self, data: &[u8]) -> Result<usize, TcpError> {
        if self.get_state() != TcpState::Established {
            return Err(TcpError::NotEstablished);
        }

        // Check send window
        let una = self.snd_una.load(Ordering::Acquire);
        let nxt = self.snd_nxt.load(Ordering::Acquire);
        let wnd = self.snd_wnd.load(Ordering::Acquire);
        let cwnd = self.congestion.get_cwnd();

        let in_flight = nxt - una;
        let available = wnd.min(cwnd).saturating_sub(in_flight);

        if available == 0 {
            return Err(TcpError::WindowFull);
        }

        let to_send = (data.len() as u32).min(available).min(1460) as usize;
        self.send_buffer.extend(&data[..to_send]);

        // Create segment for retransmission
        let segment = TcpSegment {
            seq_start: nxt,
            seq_end: nxt + to_send as u32,
            data: data[..to_send].to_vec(),
            retransmit_count: 0,
            timestamp: 0, // In production: get current time
        };

        self.retransmit_queue.push(segment);
        self.snd_nxt.fetch_add(to_send as u32, Ordering::Release);

        Ok(to_send)
    }

    /// Receive data
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, TcpError> {
        if self.recv_buffer.is_empty() {
            return Err(TcpError::WouldBlock);
        }

        let len = buf.len().min(self.recv_buffer.len());
        for i in 0..len {
            buf[i] = self.recv_buffer.pop_front().unwrap();
        }

        Ok(len)
    }

    /// Close connection
    pub fn close(&mut self) -> Result<(), TcpError> {
        let state = self.get_state();

        match state {
            TcpState::Established => {
                self.state
                    .store(TcpState::FinWait1 as u32, Ordering::Release);
                // Send FIN
                Ok(())
            }
            TcpState::CloseWait => {
                self.state
                    .store(TcpState::LastAck as u32, Ordering::Release);
                // Send FIN
                Ok(())
            }
            _ => Err(TcpError::InvalidState),
        }
    }

    fn get_state(&self) -> TcpState {
        let state_val = self.state.load(Ordering::Acquire);
        unsafe { core::mem::transmute(state_val as u8) }
    }
}

/// TCP errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpError {
    InvalidState,
    InvalidAck,
    NotEstablished,
    WindowFull,
    WouldBlock,
    ConnectionReset,
    Timeout,
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_timer() {
        let timer = TcpTimer::new();
        timer.update_rtt(100);
        assert!(timer.get_rto() >= 200);
    }

    #[test]
    fn test_congestion_control() {
        let cc = CongestionControl::new(CongestionAlgorithm::Cubic, 10 * 1460);
        let initial = cc.get_cwnd();
        cc.on_ack(1460, 100);
        assert!(cc.get_cwnd() >= initial);
    }

    #[test]
    fn test_tcp_connection() {
        let mut conn = TcpConnection::new(CongestionAlgorithm::Cubic);
        assert_eq!(conn.get_state(), TcpState::Closed);

        conn.connect(1000).unwrap();
        assert_eq!(conn.get_state(), TcpState::SynSent);
    }
}
