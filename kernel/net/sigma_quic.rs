// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/net/sigma_quic.rs — QUIC transport with Zero-Trust Identities (RFC 9000 skeleton, no_std)
// Language: Rust #![no_std]

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub const QUIC_VERSION: u32 = 0x0000_0001;
pub const MAX_STREAMS:  usize = 64;
pub const MAX_PKT:      usize = 1350; // max QUIC payload (PMTU - overhead)

// Zero-Trust Identity for QUIC connection verification
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ZeroTrustIdentity {
    pub identity_hash: [u8; 32],
    pub public_key: [u8; 32],
    pub verified: AtomicBool,
}

impl ZeroTrustIdentity {
    pub const fn new() -> Self {
        Self {
            identity_hash: [0u8; 32],
            public_key: [0u8; 32],
            verified: AtomicBool::new(false),
        }
    }
    
    pub fn is_verified(&self) -> bool {
        self.verified.load(Ordering::Acquire)
    }
    
    pub fn verify(&self) {
        self.verified.store(true, Ordering::Release);
    }
}

// ── Packet Types ──────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PacketType { Initial=0x00, ZeroRTT=0x01, Handshake=0x02, Retry=0x03, Short=0x40 }

// ── Frame Types ───────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Padding=0x00, Ping=0x01, Ack=0x02, ResetStream=0x04,
    StopSending=0x05, Crypto=0x06, NewToken=0x07,
    Stream=0x08, MaxData=0x10, MaxStreamData=0x11,
    MaxStreams=0x12, Blocked=0x14, StreamBlocked=0x15,
    NewConnectionId=0x18, RetireConnectionId=0x19,
    PathChallenge=0x1A, PathResponse=0x1B, ConnectionClose=0x1C,
    HandshakeDone=0x1E,
}

// ── Connection ID ──────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ConnectionId { pub data: [u8;20], pub len: u8 }

impl ConnectionId {
    pub fn new(data: &[u8]) -> Self {
        let mut id = Self::default();
        id.len = data.len().min(20) as u8;
        id.data[..id.len as usize].copy_from_slice(&data[..id.len as usize]);
        id
    }
    pub fn as_bytes(&self) -> &[u8] { &self.data[..self.len as usize] }
}

// ── Stream ─────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StreamState { Open, HalfClosedLocal, HalfClosedRemote, Closed }

#[derive(Clone, Copy)]
pub struct Stream {
    pub id:        u64,
    pub state:     StreamState,
    pub send_off:  AtomicU64,
    pub recv_off:  AtomicU64,
    pub max_data:  AtomicU64,
}

impl Stream {
    pub fn new(id: u64, max_data: u64) -> Self {
        Self { id, state: StreamState::Open, 
               send_off: AtomicU64::new(0), 
               recv_off: AtomicU64::new(0), 
               max_data: AtomicU64::new(max_data) }
    }
    pub fn is_open(&self) -> bool { self.state == StreamState::Open }
}

// ── QUIC Connection ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConnState { Initial, Handshake, Connected, Closing, Closed }

pub struct QuicConn {
    pub state:      ConnState,
    pub local_cid:  ConnectionId,
    pub remote_cid: ConnectionId,
    pub version:    u32,
    // Packet number spaces
    pub pkt_num_init: AtomicU64,
    pub pkt_num_hs:   AtomicU64,
    pub pkt_num_app:  AtomicU64,
    // Flow control
    pub max_data_local:  AtomicU64,
    pub max_data_remote: AtomicU64,
    pub data_sent:       AtomicU64,
    pub data_recv:       AtomicU64,
    // Streams
    streams:     [Option<Stream>; MAX_STREAMS],
    n_streams:   usize,
    next_stream: u64,
    // Crypto buffers (TLS handshake data)
    crypto_send: [u8; 4096],
    crypto_recv: [u8; 4096],
    crypto_send_len: usize,
    crypto_recv_len: usize,
    // Zero-trust identity
    pub peer_identity: ZeroTrustIdentity,
}

impl QuicConn {
    pub fn new_client(local_cid: &[u8]) -> Self {
        Self {
            state: ConnState::Initial,
            local_cid: ConnectionId::new(local_cid),
            remote_cid: ConnectionId::default(),
            version: QUIC_VERSION,
            pkt_num_init: AtomicU64::new(0), 
            pkt_num_hs: AtomicU64::new(0), 
            pkt_num_app: AtomicU64::new(0),
            max_data_local: AtomicU64::new(1 << 20), 
            max_data_remote: AtomicU64::new(1 << 20),
            data_sent: AtomicU64::new(0), 
            data_recv: AtomicU64::new(0),
            streams: [const { None }; MAX_STREAMS], n_streams: 0, next_stream: 0,
            crypto_send: [0u8;4096], crypto_recv: [0u8;4096],
            crypto_send_len: 0, crypto_recv_len: 0,
            peer_identity: ZeroTrustIdentity::new(),
        }
    }

    pub fn open_stream(&mut self) -> Option<u64> {
        if self.n_streams >= MAX_STREAMS { return None; }
        let id = self.next_stream * 4; // client-initiated bidi
        self.next_stream += 1;
        for slot in &mut self.streams {
            if slot.is_none() {
                *slot = Some(Stream::new(id, self.max_data_remote.load(Ordering::Acquire)));
                self.n_streams += 1;
                return Some(id);
            }
        }
        None
    }

    pub fn close_stream(&mut self, id: u64) {
        for slot in &mut self.streams {
            if let Some(ref mut s) = slot {
                if s.id == id { s.state = StreamState::Closed; break; }
            }
        }
    }

    pub fn stream(&self, id: u64) -> Option<&Stream> {
        self.streams.iter().flatten().find(|s| s.id == id)
    }

    /// Build an Initial packet header into `buf`. Returns header length.
    pub fn build_initial_header(&mut self, buf: &mut [u8; MAX_PKT]) -> usize {
        // Long header: 1 byte flags + 4 byte version + DCIL + DCID + SCIL + SCID + token + len + pkt_num
        let mut off = 0;
        // Flags: Long(1) | Fixed(1) | Initial(00) | Reserved(00) | PKT_NUM_LEN(11=4bytes)
        buf[off] = 0xC3; off += 1;
        buf[off..off+4].copy_from_slice(&self.version.to_be_bytes()); off += 4;
        // DCID
        buf[off] = self.remote_cid.len; off += 1;
        if self.remote_cid.len > 0 {
            let l = self.remote_cid.len as usize;
            buf[off..off+l].copy_from_slice(self.remote_cid.as_bytes()); off += l;
        }
        // SCID
        buf[off] = self.local_cid.len; off += 1;
        let l = self.local_cid.len as usize;
        buf[off..off+l].copy_from_slice(self.local_cid.as_bytes()); off += l;
        // Token length = 0
        buf[off] = 0; off += 1;
        // Packet number (4 bytes)
        let pn = self.pkt_num_init.fetch_add(1, Ordering::Relaxed);
        buf[off..off+4].copy_from_slice(&(pn as u32).to_be_bytes()); off += 4;
        off
    }

    pub fn is_connected(&self) -> bool { self.state == ConnState::Connected }
    
    /// Verify peer identity (zero-trust)
    pub fn verify_peer_identity(&self) -> bool {
        self.peer_identity.is_verified()
    }
    
    /// Set peer identity for verification
    pub fn set_peer_identity(&mut self, identity_hash: &[u8; 32]) {
        self.peer_identity.identity_hash.copy_from_slice(identity_hash);
    }
}
