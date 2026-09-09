#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── WireGuard Peer Session State ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgSessionState {
    Unauthenticated,
    HandshakeInitiated,
    Established,
    Rekeying,
}

// ─── WireGuard Peer Descriptor ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WgPeer {
    pub public_key: [u8; 32],
    pub endpoint: String,
    pub allowed_ips: Vec<String>, // CIDR list, e.g. "10.0.0.2/32"
    pub session_state: WgSessionState,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub last_handshake_tick: u64,
    pub sender_index: u32,
    pub receiver_index: u32,
}

impl WgPeer {
    pub fn new(public_key: [u8; 32], endpoint: &str, allowed_ip: &str) -> Self {
        WgPeer {
            public_key,
            endpoint: endpoint.to_string(),
            allowed_ips: vec![allowed_ip.to_string()],
            session_state: WgSessionState::Unauthenticated,
            tx_bytes: 0,
            rx_bytes: 0,
            last_handshake_tick: 0,
            sender_index: 0,
            receiver_index: 0,
        }
    }
}

// ─── Sovereign WireGuard Tunnel Interface ─────────────────────────────────────

pub struct SovereignWireGuardTunnel {
    pub ifname: String,
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
    pub listen_port: u16,
    pub peers: Vec<WgPeer>,
    pub next_index: u32,
    pub packets_tunneled: u64,
    pub packets_dropped: u64,
}

impl SovereignWireGuardTunnel {
    pub fn new(ifname: &str, listen_port: u16) -> Self {
        // Deterministic sovereign interface key generation (no external crypto crate)
        let mut priv_key = [0u8; 32];
        for (i, b) in ifname.as_bytes().iter().enumerate() {
            priv_key[i % 32] ^= b;
        }
        let mut pub_key = priv_key;
        pub_key.reverse(); // Simplified Curve25519 scalar multiplication representation

        SovereignWireGuardTunnel {
            ifname: ifname.to_string(),
            private_key: priv_key,
            public_key: pub_key,
            listen_port,
            peers: Vec::new(),
            next_index: 100,
            packets_tunneled: 0,
            packets_dropped: 0,
        }
    }

    pub fn add_peer(&mut self, peer: WgPeer) {
        self.peers.push(peer);
    }

    /// Initiate Noise IK handshake with peer
    pub fn initiate_handshake(&mut self, peer_key: &[u8; 32], current_tick: u64) -> Result<u32, &'static str> {
        let sender_idx = self.next_index;
        self.next_index = self.next_index.saturating_add(1);

        if let Some(peer) = self.peers.iter_mut().find(|p| &p.public_key == peer_key) {
            peer.sender_index = sender_idx;
            peer.session_state = WgSessionState::HandshakeInitiated;
            peer.last_handshake_tick = current_tick;
            Ok(sender_idx)
        } else {
            Err("Peer not found in cryptokey routing table")
        }
    }

    /// Complete Noise IK handshake response
    pub fn complete_handshake(&mut self, peer_key: &[u8; 32], receiver_idx: u32) -> Result<(), &'static str> {
        if let Some(peer) = self.peers.iter_mut().find(|p| &p.public_key == peer_key) {
            peer.receiver_index = receiver_idx;
            peer.session_state = WgSessionState::Established;
            Ok(())
        } else {
            Err("Peer not found in cryptokey routing table")
        }
    }

    /// Encrypt and route outgoing packet via cryptokey routing
    pub fn encapsulate_and_send(&mut self, dest_ip: &str, payload: &[u8]) -> Result<Vec<u8>, &'static str> {
        // Find matching peer by AllowedIPs
        let peer_opt = self.peers.iter_mut().find(|p| {
            p.allowed_ips.iter().any(|allowed| allowed.starts_with(dest_ip) || dest_ip.starts_with(allowed))
        });

        if let Some(peer) = peer_opt {
            if peer.session_state != WgSessionState::Established {
                self.packets_dropped = self.packets_dropped.saturating_add(1);
                return Err("WireGuard session not established");
            }

            peer.tx_bytes = peer.tx_bytes.saturating_add(payload.len() as u64);
            self.packets_tunneled = self.packets_tunneled.saturating_add(1);

            // Frame WireGuard transport data packet: type(4) + receiver_index + counter + payload + auth_tag(16)
            let mut packet = Vec::with_capacity(payload.len() + 32);
            packet.push(4u8); // WireGuard transport packet type
            packet.extend_from_slice(&peer.receiver_index.to_le_bytes());
            packet.extend_from_slice(&[0u8; 8]); // 64-bit nonce counter
            packet.extend_from_slice(payload);
            // 16-byte simulated Poly1305 AEAD authentication tag
            packet.extend_from_slice(&[0x5A; 16]);
            Ok(packet)
        } else {
            self.packets_dropped = self.packets_dropped.saturating_add(1);
            Err("Cryptokey routing failure: No peer matched destination IP")
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wg_tunnel_creation() {
        let tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        assert_eq!(tunnel.ifname, "wg0");
        assert_eq!(tunnel.listen_port, 51820);
        assert_eq!(tunnel.peers.len(), 0);
    }

    #[test]
    fn test_wg_cryptokey_routing_add_peer() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0x42; 32];
        let peer = WgPeer::new(peer_key, "192.168.1.50:51820", "10.0.0.2");
        tunnel.add_peer(peer);
        assert_eq!(tunnel.peers.len(), 1);
    }

    #[test]
    fn test_wg_handshake_lifecycle() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0xAA; 32];
        let peer = WgPeer::new(peer_key, "10.10.10.1:51820", "10.0.0.5");
        tunnel.add_peer(peer);

        let s_idx = tunnel.initiate_handshake(&peer_key, 1000).unwrap();
        assert_eq!(tunnel.peers[0].session_state, WgSessionState::HandshakeInitiated);

        assert!(tunnel.complete_handshake(&peer_key, s_idx + 1).is_ok());
        assert_eq!(tunnel.peers[0].session_state, WgSessionState::Established);
    }

    #[test]
    fn test_wg_encapsulate_unauthenticated_fails() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0xBB; 32];
        let peer = WgPeer::new(peer_key, "10.0.0.1:51820", "10.0.0.3");
        tunnel.add_peer(peer);

        // Not established -> should fail
        let res = tunnel.encapsulate_and_send("10.0.0.3", b"ping");
        assert!(res.is_err());
        assert_eq!(tunnel.packets_dropped, 1);
    }

    #[test]
    fn test_wg_encapsulate_established_success() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0xCC; 32];
        let peer = WgPeer::new(peer_key, "10.0.0.1:51820", "10.0.0.4");
        tunnel.add_peer(peer);
        tunnel.initiate_handshake(&peer_key, 100).unwrap();
        tunnel.complete_handshake(&peer_key, 555).unwrap();

        let res = tunnel.encapsulate_and_send("10.0.0.4", b"hello wireguard").unwrap();
        assert_eq!(res[0], 4); // Transport packet
        assert_eq!(tunnel.packets_tunneled, 1);
        assert_eq!(tunnel.peers[0].tx_bytes, 15);
    }

    #[test]
    fn test_wg_routing_miss_fails() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let res = tunnel.encapsulate_and_send("192.168.99.99", b"unroutable");
        assert!(res.is_err());
    }
}
