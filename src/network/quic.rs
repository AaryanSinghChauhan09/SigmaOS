//! QUIC Transport Protocol (RFC 9000 + RFC 9001 TLS 1.3)
//! Inspired by QUIC implementations: quic-go, lsquic, ngtcp2, quinn (Rust).
//! QUIC = reliable, ordered, multiplexed streams over UDP with built-in TLS 1.3.
//!
//! References:
//! - RFC 9000: https://www.rfc-editor.org/rfc/rfc9000
//! - RFC 9001 (QUIC-TLS): https://www.rfc-editor.org/rfc/rfc9001
//! - quinn (Rust QUIC): https://github.com/quinn-rs/quinn

use std::collections::HashMap;
use crate::crypto::entropy;

/// QUIC connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuicConnState {
    Idle,
    Connecting,
    Connected,
    Draining,
    Closed,
}

/// QUIC stream state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuicStreamState {
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed,
}

/// QUIC stream — bidirectional or unidirectional in-order byte stream
#[derive(Debug, Clone)]
pub struct QuicStream {
    pub stream_id: u64,
    pub state: QuicStreamState,
    pub send_offset: u64,
    pub recv_offset: u64,
    pub max_send: u64,
    pub max_recv: u64,
    pub send_buf: Vec<u8>,
    pub recv_buf: Vec<u8>,
    pub is_unidirectional: bool,
}

impl QuicStream {
    pub fn new(stream_id: u64, max_data: u64) -> Self {
        let is_uni = (stream_id & 0x2) != 0; // RFC 9000: bit 1 = unidirectional
        Self {
            stream_id,
            state: QuicStreamState::Open,
            send_offset: 0,
            recv_offset: 0,
            max_send: max_data,
            max_recv: max_data,
            send_buf: Vec::new(),
            recv_buf: Vec::new(),
            is_unidirectional: is_uni,
        }
    }

    /// Write data to the stream's send buffer.
    pub fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state != QuicStreamState::Open {
            return Err("Stream not open for writing");
        }
        if self.send_offset + data.len() as u64 > self.max_send {
            return Err("Stream flow control limit exceeded");
        }
        self.send_buf.extend_from_slice(data);
        self.send_offset += data.len() as u64;
        Ok(data.len())
    }

    /// Deliver received data into the stream's receive buffer.
    pub fn deliver(&mut self, data: &[u8]) -> usize {
        self.recv_buf.extend_from_slice(data);
        self.recv_offset += data.len() as u64;
        data.len()
    }

    /// Read available received data.
    pub fn read(&mut self, max_bytes: usize) -> Vec<u8> {
        let take = max_bytes.min(self.recv_buf.len());
        self.recv_buf.drain(..take).collect()
    }
}

/// QUIC connection ID (8 bytes per RFC 9000 recommendations)
pub type ConnectionId = [u8; 8];

/// QUIC packet types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuicPacketType {
    Initial,
    Retry,
    Handshake,
    ZeroRtt,
    OneRtt,   // Short header
}

/// QUIC connection
pub struct QuicConnection {
    pub local_cid: ConnectionId,
    pub remote_cid: Option<ConnectionId>,
    pub state: QuicConnState,
    pub streams: HashMap<u64, QuicStream>,
    next_stream_id: u64,
    pub max_stream_data: u64,
    /// Connection-level flow control
    pub max_data: u64,
    pub sent_bytes: u64,
    pub received_bytes: u64,
    /// RTT estimate in microseconds
    pub rtt_us: u64,
    /// Packet loss rate (0-100)
    pub packet_loss_pct: u8,
    /// Whether 0-RTT is available
    pub zero_rtt_available: bool,
    /// TLS handshake keys (simulated)
    pub handshake_key: [u8; 32],
    pub is_server: bool,
}

impl QuicConnection {
    /// Create a new client-side QUIC connection.
    pub fn new_client() -> Self {
        let mut cid = [0u8; 8];
        let mut hkey = [0u8; 32];
        entropy::get_entropy_bytes(&mut cid);
        entropy::get_entropy_bytes(&mut hkey);
        Self {
            local_cid: cid,
            remote_cid: None,
            state: QuicConnState::Idle,
            streams: HashMap::new(),
            next_stream_id: 0, // client-initiated bidirectional: 0, 4, 8, ...
            max_stream_data: 1 << 20, // 1MB per stream
            max_data: 1 << 24,       // 16MB connection total
            sent_bytes: 0,
            received_bytes: 0,
            rtt_us: 1000, // 1ms initial estimate
            packet_loss_pct: 0,
            zero_rtt_available: false,
            handshake_key: hkey,
            is_server: false,
        }
    }

    /// Create a server-side QUIC connection.
    pub fn new_server() -> Self {
        let mut conn = Self::new_client();
        conn.next_stream_id = 1; // server-initiated bidirectional: 1, 5, 9, ...
        conn.is_server = true;
        conn
    }

    /// Start connection (send Initial packet).
    pub fn connect(&mut self) -> Result<(), &'static str> {
        if self.state != QuicConnState::Idle { return Err("Already connecting"); }
        self.state = QuicConnState::Connecting;
        // Simulate TLS 1.3 handshake completion
        self.state = QuicConnState::Connected;
        Ok(())
    }

    /// Open a new bidirectional stream.
    pub fn open_stream(&mut self) -> Result<u64, &'static str> {
        if self.state != QuicConnState::Connected { return Err("Not connected"); }
        let id = self.next_stream_id;
        self.next_stream_id += 4; // RFC 9000: increment by 4 for bidi streams
        self.streams.insert(id, QuicStream::new(id, self.max_stream_data));
        Ok(id)
    }

    /// Send data on a stream. Returns bytes buffered.
    pub fn stream_send(&mut self, stream_id: u64, data: &[u8]) -> Result<usize, &'static str> {
        let stream = self.streams.get_mut(&stream_id).ok_or("Stream not found")?;
        let n = stream.write(data)?;
        self.sent_bytes += n as u64;
        Ok(n)
    }

    /// Receive data from a stream.
    pub fn stream_recv(&mut self, stream_id: u64, max_bytes: usize) -> Result<Vec<u8>, &'static str> {
        let stream = self.streams.get_mut(&stream_id).ok_or("Stream not found")?;
        Ok(stream.read(max_bytes))
    }

    /// Simulate delivery of incoming data on a stream (e.g., from network).
    pub fn deliver_stream_data(&mut self, stream_id: u64, data: &[u8]) -> Result<usize, &'static str> {
        let stream = self.streams.get_mut(&stream_id).ok_or("Stream not found")?;
        let n = stream.deliver(data);
        self.received_bytes += n as u64;
        Ok(n)
    }

    /// Close a stream gracefully.
    pub fn close_stream(&mut self, stream_id: u64) -> Result<(), &'static str> {
        let stream = self.streams.get_mut(&stream_id).ok_or("Stream not found")?;
        stream.state = QuicStreamState::HalfClosedLocal;
        Ok(())
    }

    /// Initiate connection close (send CONNECTION_CLOSE frame).
    pub fn close(&mut self, error_code: u64) -> Result<(), &'static str> {
        self.state = QuicConnState::Draining;
        let _ = error_code;
        self.state = QuicConnState::Closed;
        Ok(())
    }

    /// Estimate congestion window size in bytes (CUBIC-inspired).
    pub fn cwnd_bytes(&self) -> u64 {
        // Simplified: start at 10 * MSS (1460 bytes), grow with RTT
        let mss = 1460u64;
        let base_cwnd = 10 * mss;
        if self.rtt_us < 10_000 {
            base_cwnd * 4 // low latency: large window
        } else {
            base_cwnd
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_and_stream() {
        let mut conn = QuicConnection::new_client();
        conn.connect().unwrap();
        assert_eq!(conn.state, QuicConnState::Connected);
        let sid = conn.open_stream().unwrap();
        let sent = conn.stream_send(sid, b"Hello QUIC!").unwrap();
        assert_eq!(sent, 11);
    }

    #[test]
    fn test_stream_bidirectional() {
        let mut conn = QuicConnection::new_client();
        conn.connect().unwrap();
        let sid = conn.open_stream().unwrap();
        conn.deliver_stream_data(sid, b"server response").unwrap();
        let data = conn.stream_recv(sid, 1024).unwrap();
        assert_eq!(data, b"server response");
    }

    #[test]
    fn test_close() {
        let mut conn = QuicConnection::new_client();
        conn.connect().unwrap();
        conn.close(0).unwrap();
        assert_eq!(conn.state, QuicConnState::Closed);
    }

    #[test]
    fn test_stream_ids_client_server() {
        let client = QuicConnection::new_client();
        let server = QuicConnection::new_server();
        // RFC 9000: client bidi starts at 0, server bidi at 1
        assert_eq!(client.next_stream_id, 0);
        assert_eq!(server.next_stream_id, 1);
    }
}
