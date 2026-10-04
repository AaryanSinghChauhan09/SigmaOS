//! # TCP Protocol Implementation
//!
//! Transmission Control Protocol inspired by Linux TCP stack.
//! Provides reliable, ordered, error-checked delivery of byte streams.

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// TCP header flags
#[derive(Debug, Clone, Copy)]
pub struct TcpFlags {
    pub fin: bool, // Finish
    pub syn: bool, // Synchronize
    pub rst: bool, // Reset
    pub psh: bool, // Push
    pub ack: bool, // Acknowledgment
    pub urg: bool, // Urgent
    pub ece: bool, // ECN-Echo
    pub cwr: bool, // Congestion Window Reduced
}

impl TcpFlags {
    pub const NONE: Self = Self {
        fin: false,
        syn: false,
        rst: false,
        psh: false,
        ack: false,
        urg: false,
        ece: false,
        cwr: false,
    };

    pub const SYN: Self = Self {
        fin: false,
        syn: true,
        rst: false,
        psh: false,
        ack: false,
        urg: false,
        ece: false,
        cwr: false,
    };

    pub const SYN_ACK: Self = Self {
        fin: false,
        syn: true,
        rst: false,
        psh: false,
        ack: true,
        urg: false,
        ece: false,
        cwr: false,
    };

    pub const ACK: Self = Self {
        fin: false,
        syn: false,
        rst: false,
        psh: false,
        ack: true,
        urg: false,
        ece: false,
        cwr: false,
    };

    pub const FIN_ACK: Self = Self {
        fin: true,
        syn: false,
        rst: false,
        psh: false,
        ack: true,
        urg: false,
        ece: false,
        cwr: false,
    };
}

/// TCP connection state (RFC 793)
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

/// TCP segment
#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub flags: TcpFlags,
    pub window_size: u16,
    pub urgent_pointer: u16,
    pub data: Vec<u8>,
}

impl TcpSegment {
    pub fn new(src_port: u16, dst_port: u16, seq: u32) -> Self {
        Self {
            src_port,
            dst_port,
            seq_num: seq,
            ack_num: 0,
            flags: TcpFlags::NONE,
            window_size: 65535, // Default window
            urgent_pointer: 0,
            data: Vec::new(),
        }
    }

    pub fn with_ack(mut self, ack: u32) -> Self {
        self.ack_num = ack;
        self.flags.ack = true;
        self
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}

/// TCP send buffer
pub struct TcpSendBuffer {
    /// Unsent data
    data: VecDeque<u8>,
    /// Send sequence number
    next_seq: AtomicU32,
    /// Unacknowledged sequence number
    una: AtomicU32,
    /// Buffer capacity
    capacity: usize,
}

impl TcpSendBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            next_seq: AtomicU32::new(0),
            una: AtomicU32::new(0),
            capacity,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, TcpError> {
        let available = self.capacity - self.data.len();
        if available == 0 {
            return Err(TcpError::BufferFull);
        }
        let to_write = available.min(data.len());
        self.data.extend(&data[..to_write]);
        Ok(to_write)
    }

    pub fn get_data(&mut self, max_len: usize) -> Vec<u8> {
        let len = max_len.min(self.data.len());
        self.data.drain(..len).collect()
    }

    pub fn ack(&self, ack_num: u32) {
        self.una.store(ack_num, Ordering::Release);
    }
}

/// TCP receive buffer
pub struct TcpRecvBuffer {
    /// Out-of-order segments
    data: VecDeque<u8>,
    /// Expected sequence number
    next_seq: AtomicU32,
    /// Buffer capacity
    capacity: usize,
}

impl TcpRecvBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            next_seq: AtomicU32::new(0),
            capacity,
        }
    }

    pub fn receive(&mut self, seq: u32, data: &[u8]) -> Result<(), TcpError> {
        let expected = self.next_seq.load(Ordering::Acquire);
        if seq == expected {
            // In-order data
            if self.data.len() + data.len() > self.capacity {
                return Err(TcpError::BufferFull);
            }
            self.data.extend(data);
            self.next_seq
                .fetch_add(data.len() as u32, Ordering::Release);
            Ok(())
        } else {
            // Out-of-order (would need reordering logic)
            Err(TcpError::OutOfOrder)
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let to_read = buffer.len().min(self.data.len());
        for i in 0..to_read {
            buffer[i] = self.data.pop_front().unwrap();
        }
        to_read
    }
}

/// TCP congestion control state (inspired by Linux)
pub struct TcpCongestion {
    /// Congestion window (cwnd)
    pub cwnd: AtomicU32,
    /// Slow start threshold (ssthresh)
    pub ssthresh: AtomicU32,
    /// Congestion control algorithm
    pub algorithm: CongestionAlgorithm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionAlgorithm {
    Reno,
    Cubic,
    Bbr,
}

impl TcpCongestion {
    pub fn new() -> Self {
        Self {
            cwnd: AtomicU32::new(10), // Initial window (RFC 6928)
            ssthresh: AtomicU32::new(65535),
            algorithm: CongestionAlgorithm::Cubic,
        }
    }

    pub fn on_ack(&self) {
        let cwnd = self.cwnd.load(Ordering::Acquire);
        let ssthresh = self.ssthresh.load(Ordering::Acquire);

        if cwnd < ssthresh {
            // Slow start
            self.cwnd.fetch_add(1, Ordering::Release);
        } else {
            // Congestion avoidance (simplified)
            self.cwnd.fetch_add(1, Ordering::Release);
        }
    }

    pub fn on_loss(&self) {
        // Multiplicative decrease
        let cwnd = self.cwnd.load(Ordering::Acquire);
        self.ssthresh.store(cwnd / 2, Ordering::Release);
        self.cwnd.store(cwnd / 2, Ordering::Release);
    }
}

/// TCP control block (inspired by Linux tcp_sock)
pub struct TcpSocket {
    /// Connection state
    pub state: TcpState,
    /// Local port
    pub local_port: u16,
    /// Remote port
    pub remote_port: u16,
    /// Send buffer
    pub send_buffer: TcpSendBuffer,
    /// Receive buffer
    pub recv_buffer: TcpRecvBuffer,
    /// Congestion control
    pub congestion: TcpCongestion,
    /// Round-trip time (microseconds)
    pub rtt: AtomicU64,
    /// Retransmission timeout (microseconds)
    pub rto: AtomicU64,
}

impl TcpSocket {
    pub fn new(local_port: u16) -> Self {
        Self {
            state: TcpState::Closed,
            local_port,
            remote_port: 0,
            send_buffer: TcpSendBuffer::new(65536),
            recv_buffer: TcpRecvBuffer::new(65536),
            congestion: TcpCongestion::new(),
            rtt: AtomicU64::new(100000),  // 100ms initial
            rto: AtomicU64::new(1000000), // 1s initial
        }
    }

    /// Connect to remote endpoint
    pub fn connect(&mut self, remote_port: u16) -> Result<(), TcpError> {
        if self.state != TcpState::Closed {
            return Err(TcpError::InvalidState);
        }
        self.remote_port = remote_port;
        self.state = TcpState::SynSent;
        Ok(())
    }

    /// Listen for connections
    pub fn listen(&mut self) -> Result<(), TcpError> {
        if self.state != TcpState::Closed {
            return Err(TcpError::InvalidState);
        }
        self.state = TcpState::Listen;
        Ok(())
    }

    /// Process incoming segment
    pub fn process_segment(
        &mut self,
        segment: &TcpSegment,
    ) -> Result<Option<TcpSegment>, TcpError> {
        match self.state {
            TcpState::Listen => {
                if segment.flags.syn {
                    self.state = TcpState::SynReceived;
                    self.remote_port = segment.src_port;
                    // Send SYN-ACK
                    let mut response = TcpSegment::new(self.local_port, segment.src_port, 1000);
                    response.flags = TcpFlags::SYN_ACK;
                    response.ack_num = segment.seq_num + 1;
                    Ok(Some(response))
                } else {
                    Err(TcpError::InvalidState)
                }
            }
            TcpState::SynSent => {
                if segment.flags.syn && segment.flags.ack {
                    self.state = TcpState::Established;
                    // Send ACK
                    let mut response =
                        TcpSegment::new(self.local_port, segment.src_port, segment.ack_num);
                    response.flags = TcpFlags::ACK;
                    response.ack_num = segment.seq_num + 1;
                    Ok(Some(response))
                } else {
                    Err(TcpError::InvalidState)
                }
            }
            TcpState::SynReceived => {
                if segment.flags.ack {
                    self.state = TcpState::Established;
                    Ok(None)
                } else {
                    Err(TcpError::InvalidState)
                }
            }
            TcpState::Established => {
                if segment.flags.fin {
                    self.state = TcpState::CloseWait;
                    // Send ACK
                    let mut response = TcpSegment::new(self.local_port, segment.src_port, 0);
                    response.flags = TcpFlags::ACK;
                    response.ack_num = segment.seq_num + 1;
                    Ok(Some(response))
                } else if !segment.data.is_empty() {
                    self.recv_buffer.receive(segment.seq_num, &segment.data)?;
                    // Send ACK
                    let mut response = TcpSegment::new(self.local_port, segment.src_port, 0);
                    response.flags = TcpFlags::ACK;
                    response.ack_num = self.recv_buffer.next_seq.load(Ordering::Acquire);
                    Ok(Some(response))
                } else if segment.flags.ack {
                    self.send_buffer.ack(segment.ack_num);
                    self.congestion.on_ack();
                    Ok(None)
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    /// Send data
    pub fn send(&mut self, data: &[u8]) -> Result<usize, TcpError> {
        if self.state != TcpState::Established {
            return Err(TcpError::NotConnected);
        }
        self.send_buffer.write(data)
    }

    /// Receive data
    pub fn recv(&mut self, buffer: &mut [u8]) -> Result<usize, TcpError> {
        if self.state != TcpState::Established && self.state != TcpState::CloseWait {
            return Err(TcpError::NotConnected);
        }
        Ok(self.recv_buffer.read(buffer))
    }

    /// Close connection
    pub fn close(&mut self) -> Result<TcpSegment, TcpError> {
        match self.state {
            TcpState::Established => {
                self.state = TcpState::FinWait1;
                let mut segment = TcpSegment::new(self.local_port, self.remote_port, 0);
                segment.flags = TcpFlags::FIN_ACK;
                Ok(segment)
            }
            TcpState::CloseWait => {
                self.state = TcpState::LastAck;
                let mut segment = TcpSegment::new(self.local_port, self.remote_port, 0);
                segment.flags = TcpFlags::FIN_ACK;
                Ok(segment)
            }
            _ => Err(TcpError::InvalidState),
        }
    }
}

/// TCP errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpError {
    InvalidState,
    NotConnected,
    BufferFull,
    OutOfOrder,
    ConnectionReset,
    Timeout,
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_flags() {
        let flags = TcpFlags::SYN_ACK;
        assert!(flags.syn);
        assert!(flags.ack);
        assert!(!flags.fin);
    }

    #[test]
    fn test_tcp_segment() {
        let seg = TcpSegment::new(80, 12345, 1000).with_ack(2000);
        assert_eq!(seg.src_port, 80);
        assert_eq!(seg.dst_port, 12345);
        assert_eq!(seg.seq_num, 1000);
        assert_eq!(seg.ack_num, 2000);
    }

    #[test]
    fn test_tcp_connect() {
        let mut sock = TcpSocket::new(12345);
        assert_eq!(sock.state, TcpState::Closed);
        sock.connect(80).unwrap();
        assert_eq!(sock.state, TcpState::SynSent);
    }

    #[test]
    fn test_tcp_listen() {
        let mut sock = TcpSocket::new(80);
        sock.listen().unwrap();
        assert_eq!(sock.state, TcpState::Listen);
    }

    #[test]
    fn test_send_buffer() {
        let mut buf = TcpSendBuffer::new(1024);
        let written = buf.write(b"Hello").unwrap();
        assert_eq!(written, 5);
    }

    #[test]
    fn test_recv_buffer() {
        let mut buf = TcpRecvBuffer::new(1024);
        buf.receive(0, b"Hello").unwrap();
        let mut output = [0u8; 10];
        let read = buf.read(&mut output);
        assert_eq!(read, 5);
        assert_eq!(&output[..5], b"Hello");
    }
}
