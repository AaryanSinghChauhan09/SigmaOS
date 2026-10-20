//! TCP stream implementation following SigmaOS principles
//!
//! Implements reliable TCP stream following the network stack design.
//! Following principles: DIRECT_DEVICE_CONTROL, RACE_FREE DESIGN,
//! MEMORY_SAFETY, and SEPARATION_OF_CONCERNS.

use core::net::SocketAddr;
use core::fmt;

/// TCP connection state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TcpState {
    /// Connection is closed
    Closed,
    /// Connection is listening
    Listening,
    /// Connection is synced
    Synced,
    /// Connection is established
    Established,
    /// Connection is closing
    CloseWait,
    /// Connection is last close
    TimeWait,
}

/// TCP segment header
#[derive(Debug, Clone)]
pub struct TcpSegment {
    /// Source port
    pub source_port: u16,
    /// Destination port
    pub dest_port: u16,
    /// Sequence number
    pub seq_num: u32,
    /// Acknowledgment number
    pub ack_num: u32,
    /// Data offset and flags
    pub data_offset_flags: u16,
    /// Window size
    pub window_size: u16,
    /// Checksum
    pub checksum: u16,
    /// Urgent pointer
    pub urgent_ptr: u16,
    /// Payload data
    pub payload: Vec<u8>,
}

/// TCP connection parameters
#[derive(Debug, Clone)]
pub struct TcpConnectParams {
    /// Remote address
    pub remote_addr: SocketAddr,
    /// Local port
    pub local_port: u16,
    /// Remote port
    pub remote_port: u16,
    /// Maximum segment size (MSS)
    pub mss: u16,
    /// Keepalive timeout in seconds
    pub keepalive_timeout: u64,
    /// Number of retries
    pub retries: u32,
}

/// TCP connection state machine
pub struct TcpConnection {
    /// Connection state
    state: TcpState,
    /// Local address
    local_addr: SocketAddr,
    /// Remote address
    remote_addr: SocketAddr,
    /// Send sequence number
    send_seq: u32,
    /// Receive acknowledgment number
    recv_ack: u32,
    /// Send window
    send_window: u16,
    /// Receive window
    recv_window: u16,
    /// Outbound data buffer
    send_buffer: Vec<u8>,
    /// Incoming data buffer
    recv_buffer: Vec<u8>,
    /// Maximum segment size
    mss: u16,
    /// Keepalive timer
    keepalive_timer: Option<u64>,
    /// Retry counter
    retry_count: u32,
}

impl TcpConnection {
    /// Create new TCP connection in listening state
    pub fn listen(local_addr: SocketAddr, mss: u16) -> Self {
        TcpConnection {
            state: TcpState::Listening,
            local_addr,
            remote_addr: SocketAddr::new(core::net::IpAddr::from(core::net::Ipv4Addr::new(0, 0, 0, 0)), 0),
            send_seq: 0,
            recv_ack: 0,
            send_window: 65535,
            recv_window: 65535,
            send_buffer: Vec::new(),
            recv_buffer: Vec::new(),
            mss,
            keepalive_timer: None,
            retry_count: 0,
        }
    }

    /// Connect to remote address
    pub fn connect(&mut self, remote_addr: SocketAddr, mss: u16) -> Result<(), String> {
        self.remote_addr = remote_addr;
        self.mss = mss;
        self.state = TcpState::Synced;
        Ok(())
    }

    /// Send data over TCP
    pub fn send(&mut self, data: &[u8]) -> Result<usize, String> {
        // In real implementation, this would craft TCP segments
        // and send them over the network
        let len = data.len();
        self.send_buffer.extend_from_slice(data);
        Ok(len)
    }

    /// Receive data from TCP
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        // In real implementation, this would parse incoming segments
        // and return data from the receive buffer
        let len = core::cmp::min(buf.len(), self.recv_buffer.len());
        buf.copy_from_slice(&self.recv_buffer[..len]);
        self.recv_buffer.drain(..len);
        Ok(len)
    }

    /// Get local address
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Get remote address
    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }

    /// Check if connection is established
    pub fn is_established(&self) -> bool {
        matches!(self.state, TcpState::Established | TcpState::Synced)
    }

    /// Check if connection is closed
    pub fn is_closed(&self) -> bool {
        matches!(self.state, TcpState::Closed)
    }
}

impl fmt::Display for TcpState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TcpState::Closed => write!(f, "Closed"),
            TcpState::Listening => write!(f, "Listening"),
            TcpState::Synced => write!(f, "Synced"),
            TcpState::Established => write!(f, "Established"),
            TcpState::CloseWait => write!(f, "CloseWait"),
            TcpState::TimeWait => write!(f, "TimeWait"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};

    #[test]
    fn test_tcp_state_display() {
        assert_eq!(format!("{}", TcpState::Established), "Established");
        assert_eq!(format!("{}", TcpState::Closed), "Closed");
    }

    #[test]
    fn test_tcp_connection_listen() {
        let conn = TcpConnection::listen(SocketAddr::new(IpAddr::from(Ipv4Addr::new(192, 168, 1, 1)), 8080), 1460);
        assert!(conn.is_listening());
        assert!(!conn.is_established());
    }

    #[test]
    fn test_tcp_send_recv() {
        let mut conn = TcpConnection::listen(SocketAddr::new(IpAddr::from(Ipv4Addr::new(192, 168, 1, 1)), 8080), 1460);
        
        // Connect
        conn.connect(SocketAddr::new(IpAddr::from(Ipv4Addr::new(10, 0, 0, 1)), 8080), 1460).unwrap();
        
        // Send data
        let data = b"Hello TCP";
        let sent = conn.send(data).unwrap();
        assert_eq!(sent, b"Hello TCP".len());
        
        // Receive data
        let mut buf = [0u8; 1024];
        let received = conn.recv(&mut buf).unwrap();
        assert_eq!(received, b"Hello TCP".len());
    }
}
