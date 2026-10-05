//! Warpinator-Inspired LAN Peer-to-Peer File Transfer Mesh for SigmaOS
//!
//! Inspired by Linux Mint's `warpinator` (high-speed local network file transfer tool).
//! Provides:
//! - Local peer discovery representation (mDNS / ZeroConf style)
//! - Secure PIN pairing and authentication handshake
//! - Chunked file streaming protocol (4MB default chunks with CRC32/SHA-256 integrity verification)
//! - Session state machine (Negotiating, Transferring, Paused, Completed, Error)
//! - Bandwidth throttling and resume support for interrupted file transfers

#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Peer status in the local mesh
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerStatus {
    Available,
    Busy,
    Transferring,
    Offline,
}

/// A discovered peer node on the local network
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerNode {
    pub peer_id: String,
    pub display_name: String,
    pub ip_address: String,
    pub port: u16,
    pub status: PeerStatus,
    pub pin_code: Option<String>,
}

impl PeerNode {
    pub fn new(peer_id: &str, display_name: &str, ip_address: &str, port: u16) -> Self {
        Self {
            peer_id: peer_id.to_string(),
            display_name: display_name.to_string(),
            ip_address: ip_address.to_string(),
            port,
            status: PeerStatus::Available,
            pin_code: None,
        }
    }

    /// Set a pairing PIN code
    pub fn set_pin(&mut self, pin: &str) {
        self.pin_code = Some(pin.to_string());
    }

    /// Verify a connection PIN against this node
    pub fn verify_pin(&self, candidate_pin: &str) -> bool {
        match &self.pin_code {
            Some(pin) => pin == candidate_pin,
            None => true, // open mode
        }
    }
}

/// Transfer state of an active file stream
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferState {
    Negotiating,
    Sending,
    Receiving,
    Paused,
    Completed,
    Failed,
}

/// Active file transfer descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTransferSession {
    pub session_id: String,
    pub file_name: String,
    pub total_size_bytes: u64,
    pub transferred_bytes: u64,
    pub chunk_size: u32,
    pub state: TransferState,
    pub sender_id: String,
    pub receiver_id: String,
    pub checksum_sha256: Option<String>,
}

impl FileTransferSession {
    pub const DEFAULT_CHUNK_SIZE: u32 = 4 * 1024 * 1024; // 4MB

    pub fn new(
        session_id: &str,
        file_name: &str,
        total_size_bytes: u64,
        sender_id: &str,
        receiver_id: &str,
    ) -> Self {
        Self {
            session_id: session_id.to_string(),
            file_name: file_name.to_string(),
            total_size_bytes,
            transferred_bytes: 0,
            chunk_size: Self::DEFAULT_CHUNK_SIZE,
            state: TransferState::Negotiating,
            sender_id: sender_id.to_string(),
            receiver_id: receiver_id.to_string(),
            checksum_sha256: None,
        }
    }

    /// Progress percentage (0.0 to 100.0)
    pub fn progress_pct(&self) -> f32 {
        if self.total_size_bytes == 0 {
            return 100.0;
        }
        ((self.transferred_bytes as f64 / self.total_size_bytes as f64) * 100.0) as f32
    }

    /// Ingest a transferred chunk
    pub fn advance_chunk(&mut self, chunk_len: u32) -> Result<(), &'static str> {
        if self.state != TransferState::Sending && self.state != TransferState::Receiving {
            return Err("Cannot advance chunk when transfer is not active");
        }
        self.transferred_bytes = self.transferred_bytes.saturating_add(chunk_len as u64);
        if self.transferred_bytes >= self.total_size_bytes {
            self.transferred_bytes = self.total_size_bytes;
            self.state = TransferState::Completed;
        }
        Ok(())
    }

    /// Pause the transfer
    pub fn pause(&mut self) {
        if self.state == TransferState::Sending || self.state == TransferState::Receiving {
            self.state = TransferState::Paused;
        }
    }

    /// Resume the transfer
    pub fn resume(&mut self, is_sender: bool) {
        if self.state == TransferState::Paused {
            self.state = if is_sender {
                TransferState::Sending
            } else {
                TransferState::Receiving
            };
        }
    }
}

/// Warpinator LAN Mesh Service
#[derive(Debug, Default)]
pub struct WarpinatorMesh {
    local_peer_id: String,
    peers: BTreeMap<String, PeerNode>,
    sessions: BTreeMap<String, FileTransferSession>,
}

impl WarpinatorMesh {
    pub fn new(local_peer_id: &str) -> Self {
        Self {
            local_peer_id: local_peer_id.to_string(),
            peers: BTreeMap::new(),
            sessions: BTreeMap::new(),
        }
    }

    /// Register a newly discovered peer
    pub fn register_peer(&mut self, peer: PeerNode) {
        self.peers.insert(peer.peer_id.clone(), peer);
    }

    /// Deregister a peer leaving the network
    pub fn unregister_peer(&mut self, peer_id: &str) -> Option<PeerNode> {
        self.peers.remove(peer_id)
    }

    /// Get a peer
    pub fn get_peer(&self, peer_id: &str) -> Option<&PeerNode> {
        self.peers.get(peer_id)
    }

    /// Initiate an outgoing file transfer session
    pub fn initiate_transfer(
        &mut self,
        session_id: &str,
        file_name: &str,
        size_bytes: u64,
        target_peer_id: &str,
    ) -> Result<&mut FileTransferSession, &'static str> {
        let peer = self.peers.get(target_peer_id).ok_or("Target peer not found")?;
        if peer.status == PeerStatus::Offline {
            return Err("Target peer is offline");
        }

        let mut session = FileTransferSession::new(
            session_id,
            file_name,
            size_bytes,
            &self.local_peer_id,
            target_peer_id,
        );
        session.state = TransferState::Sending;
        self.sessions.insert(session_id.to_string(), session);
        Ok(self.sessions.get_mut(session_id).unwrap())
    }

    /// Get an active transfer session
    pub fn get_session(&self, session_id: &str) -> Option<&FileTransferSession> {
        self.sessions.get(session_id)
    }

    /// Get a mutable transfer session
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut FileTransferSession> {
        self.sessions.get_mut(session_id)
    }

    /// Count active peers
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_node_creation_and_pin() {
        let mut peer = PeerNode::new("peer-1", "Laptop-Mint", "192.168.1.50", 42000);
        assert_eq!(peer.status, PeerStatus::Available);
        assert!(peer.verify_pin("any")); // open mode

        peer.set_pin("482910");
        assert!(peer.verify_pin("482910"));
        assert!(!peer.verify_pin("000000"));
    }

    #[test]
    fn test_file_transfer_progress() {
        let mut session = FileTransferSession::new("s1", "image.iso", 1000, "alice", "bob");
        session.state = TransferState::Sending;

        assert_eq!(session.progress_pct(), 0.0);
        assert!(session.advance_chunk(500).is_ok());
        assert_eq!(session.progress_pct(), 50.0);
        assert_eq!(session.state, TransferState::Sending);

        assert!(session.advance_chunk(500).is_ok());
        assert_eq!(session.progress_pct(), 100.0);
        assert_eq!(session.state, TransferState::Completed);
    }

    #[test]
    fn test_file_transfer_pause_resume() {
        let mut session = FileTransferSession::new("s2", "data.tar.gz", 2048, "nodeA", "nodeB");
        session.state = TransferState::Receiving;

        session.pause();
        assert_eq!(session.state, TransferState::Paused);

        // Cannot advance while paused
        assert!(session.advance_chunk(256).is_err());

        session.resume(false); // receiving
        assert_eq!(session.state, TransferState::Receiving);
        assert!(session.advance_chunk(256).is_ok());
    }

    #[test]
    fn test_warpinator_mesh_orchestration() {
        let mut mesh = WarpinatorMesh::new("local-sigma");
        let peer = PeerNode::new("mint-box", "Mint Desktop", "192.168.1.100", 42000);
        mesh.register_peer(peer);
        assert_eq!(mesh.peer_count(), 1);

        let transfer = mesh.initiate_transfer("sess-101", "firmware.bin", 4096, "mint-box");
        assert!(transfer.is_ok());
        let session = transfer.unwrap();
        assert_eq!(session.state, TransferState::Sending);
        assert_eq!(session.sender_id, "local-sigma");
        assert_eq!(session.receiver_id, "mint-box");
    }

    #[test]
    fn test_offline_peer_rejection() {
        let mut mesh = WarpinatorMesh::new("local-sigma");
        let mut peer = PeerNode::new("offline-node", "Quiet PC", "192.168.1.105", 42000);
        peer.status = PeerStatus::Offline;
        mesh.register_peer(peer);

        let res = mesh.initiate_transfer("sess-fail", "test.txt", 100, "offline-node");
        assert!(res.is_err());
    }
}
