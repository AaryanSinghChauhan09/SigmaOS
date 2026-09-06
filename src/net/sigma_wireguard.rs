//! SigmaOS WireGuard — Sovereign VPN Kernel Module
//!
//! Implements the WireGuard VPN protocol in pure Rust with no external deps.
//! Inspired by the original WireGuard (Jason A. Donenfeld) and
//! the Linux kernel wireguard module (drivers/net/wireguard/).
//!
//! # Protocol Summary
//! - Uses Curve25519 (ECDH), ChaCha20-Poly1305 (AEAD), BLAKE2s (hash/MAC)
//! - Initiator/responder handshake with 1.5 RTT
//! - Perfect forward secrecy via ephemeral keys
//! - Noise_IKpsk2 handshake framework
//! - UDP transport on configurable port (default 51820)

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ============================================================
// Key Types
// ============================================================

/// A 32-byte Curve25519 public key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublicKey(pub [u8; 32]);

/// A 32-byte Curve25519 private key.
#[derive(Debug, Clone, Copy)]
pub struct PrivateKey(pub [u8; 32]);

/// A 32-byte pre-shared key (optional, for post-quantum hardening).
#[derive(Debug, Clone, Copy)]
pub struct PresharedKey(pub [u8; 32]);

impl PublicKey {
    pub fn zero() -> Self { Self([0u8; 32]) }
    pub fn as_bytes(&self) -> &[u8; 32] { &self.0 }
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| alloc::format!("{:02x}", b)).collect()
    }
}

/// Derive public key from private key (stub — real impl uses Curve25519).
pub fn derive_public_key(private: &PrivateKey) -> PublicKey {
    // In production: Curve25519 scalar multiplication with base point
    let mut pk = [0u8; 32];
    for (i, b) in private.0.iter().enumerate() {
        pk[i] = b.wrapping_mul(0x41).wrapping_add(0x57); // deterministic stub
    }
    pk[0] &= 0xF8; pk[31] = (pk[31] & 0x7F) | 0x40; // Curve25519 clamping
    PublicKey(pk)
}

// ============================================================
// Allowed IP Range
// ============================================================

/// An allowed IP range for a peer (CIDR notation).
#[derive(Debug, Clone)]
pub struct AllowedIp {
    pub addr: [u8; 16], // IPv6-mapped IPv4 or full IPv6
    pub prefix_len: u8,
    pub is_ipv6: bool,
}

impl AllowedIp {
    pub fn ipv4(a: u8, b: u8, c: u8, d: u8, prefix: u8) -> Self {
        let mut addr = [0u8; 16];
        // IPv4-mapped IPv6 prefix: ::ffff:x.x.x.x
        addr[10] = 0xFF; addr[11] = 0xFF;
        addr[12] = a; addr[13] = b; addr[14] = c; addr[15] = d;
        Self { addr, prefix_len: prefix + 96, is_ipv6: false }
    }

    pub fn ipv6(addr: [u8; 16], prefix: u8) -> Self {
        Self { addr, prefix_len: prefix, is_ipv6: true }
    }

    pub fn any_ipv4() -> Self { Self::ipv4(0, 0, 0, 0, 0) }

    pub fn matches_ipv4(&self, ip: [u8; 4]) -> bool {
        if self.is_ipv6 { return false; }
        let prefix = self.prefix_len.saturating_sub(96);
        if prefix == 0 { return true; }
        let mask_bits = prefix as u32;
        let self_ip = u32::from_be_bytes([self.addr[12], self.addr[13], self.addr[14], self.addr[15]]);
        let target = u32::from_be_bytes(ip);
        let mask = if mask_bits >= 32 { u32::MAX } else { !((1u32 << (32 - mask_bits)) - 1) };
        (self_ip & mask) == (target & mask)
    }
}

// ============================================================
// Handshake State
// ============================================================

/// WireGuard handshake state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    /// No handshake in progress
    Idle,
    /// Initiator sent handshake initiation, waiting for response
    InitSent,
    /// Responder received initiation, sent response
    RespSent,
    /// Handshake complete, session keys established
    Established,
    /// Handshake failed/expired
    Failed,
}

/// A WireGuard handshake session (simplified Noise_IKpsk2).
#[derive(Debug, Clone)]
pub struct WgHandshake {
    pub state: HandshakeState,
    /// Local ephemeral private key for this session
    pub ephemeral_private: PrivateKey,
    /// Remote ephemeral public key (received)
    pub remote_ephemeral: Option<PublicKey>,
    /// Session index (for identifying handshake packets)
    pub sender_index: u32,
    pub receiver_index: u32,
    /// Timestamp of handshake start (nanoseconds)
    pub created_at_ns: u64,
}

impl WgHandshake {
    pub fn new_initiator(private: PrivateKey, sender_index: u32, now_ns: u64) -> Self {
        Self {
            state: HandshakeState::Idle,
            ephemeral_private: private,
            remote_ephemeral: None,
            sender_index,
            receiver_index: 0,
            created_at_ns: now_ns,
        }
    }

    pub fn is_expired(&self, now_ns: u64) -> bool {
        // Handshake expires after 5 seconds
        now_ns.saturating_sub(self.created_at_ns) > 5_000_000_000
    }
}

// ============================================================
// Session Keys
// ============================================================

/// Symmetric session keys derived from handshake.
#[derive(Debug, Clone)]
pub struct WgSessionKeys {
    /// Key for sending (initiator→responder)
    pub sending_key: [u8; 32],
    /// Key for receiving (responder→initiator)
    pub receiving_key: [u8; 32],
    /// Send counter (nonce for ChaCha20-Poly1305)
    pub sending_counter: u64,
    /// Bitmap of received counters (replay protection)
    pub receiving_bitmap: u64,
    /// Last counter value seen
    pub receiving_counter: u64,
    /// Session creation time
    pub created_at_ns: u64,
    /// Whether this is an initiator or responder session
    pub is_initiator: bool,
}

impl WgSessionKeys {
    pub fn new(sending: [u8; 32], receiving: [u8; 32], is_init: bool, now_ns: u64) -> Self {
        Self {
            sending_key: sending, receiving_key: receiving,
            sending_counter: 0, receiving_bitmap: 0,
            receiving_counter: 0, created_at_ns: now_ns,
            is_initiator: is_init,
        }
    }

    /// Check if session has expired (180 seconds).
    pub fn is_expired(&self, now_ns: u64) -> bool {
        now_ns.saturating_sub(self.created_at_ns) > 180_000_000_000
    }

    /// Get next send nonce and increment counter.
    pub fn next_send_nonce(&mut self) -> u64 {
        let nonce = self.sending_counter;
        self.sending_counter += 1;
        nonce
    }

    /// Validate and record a received counter (anti-replay).
    pub fn check_replay(&mut self, counter: u64) -> bool {
        if counter + 64 <= self.receiving_counter { return false; } // Too old
        if counter > self.receiving_counter {
            // Advance window
            let shift = counter - self.receiving_counter;
            if shift >= 64 { self.receiving_bitmap = 0; }
            else { self.receiving_bitmap <<= shift; }
            self.receiving_counter = counter;
        }
        let bit = 1u64 << (self.receiving_counter - counter);
        if self.receiving_bitmap & bit != 0 { return false; } // Replay
        self.receiving_bitmap |= bit;
        true
    }
}

// ============================================================
// WireGuard Peer
// ============================================================

/// A WireGuard peer configuration and state.
#[derive(Debug, Clone)]
pub struct WgPeer {
    /// Peer's static public key
    pub public_key: PublicKey,
    /// Optional pre-shared key (PSK)
    pub preshared_key: Option<PresharedKey>,
    /// Peer's UDP endpoint
    pub endpoint: Option<WgEndpoint>,
    /// Allowed IP ranges for this peer
    pub allowed_ips: Vec<AllowedIp>,
    /// Keepalive interval in seconds (0 = disabled)
    pub persistent_keepalive: u16,
    /// Current handshake state
    pub handshake: Option<WgHandshake>,
    /// Active session keys
    pub session: Option<WgSessionKeys>,
    /// Last handshake time
    pub last_handshake_ns: u64,
    /// Bytes sent to this peer
    pub tx_bytes: u64,
    /// Bytes received from this peer
    pub rx_bytes: u64,
}

/// A UDP endpoint (IP:port).
#[derive(Debug, Clone)]
pub struct WgEndpoint {
    pub ip: [u8; 4],
    pub port: u16,
}

impl WgEndpoint {
    pub fn new(a: u8, b: u8, c: u8, d: u8, port: u16) -> Self {
        Self { ip: [a, b, c, d], port }
    }

    pub fn to_string(&self) -> String {
        alloc::format!("{}.{}.{}.{}:{}", self.ip[0], self.ip[1], self.ip[2], self.ip[3], self.port)
    }
}

impl WgPeer {
    pub fn new(public_key: PublicKey) -> Self {
        Self {
            public_key, preshared_key: None, endpoint: None,
            allowed_ips: Vec::new(), persistent_keepalive: 0,
            handshake: None, session: None, last_handshake_ns: 0,
            tx_bytes: 0, rx_bytes: 0,
        }
    }

    pub fn add_allowed_ip(&mut self, ip: AllowedIp) { self.allowed_ips.push(ip); }

    pub fn set_endpoint(&mut self, ep: WgEndpoint) { self.endpoint = Some(ep); }

    pub fn has_valid_session(&self, now_ns: u64) -> bool {
        self.session.as_ref().map(|s| !s.is_expired(now_ns)).unwrap_or(false)
    }

    pub fn route_matches(&self, dst_ip: [u8; 4]) -> bool {
        self.allowed_ips.iter().any(|r| r.matches_ipv4(dst_ip))
    }
}

// ============================================================
// WireGuard Device
// ============================================================

/// A WireGuard network device (wg0, wg1, ...).
pub struct WgDevice {
    /// Device name
    pub name: String,
    /// Static private key
    private_key: PrivateKey,
    /// Derived public key
    pub public_key: PublicKey,
    /// Listen port
    pub listen_port: u16,
    /// fwmark for routing
    pub fwmark: u32,
    /// Peers indexed by public key
    peers: BTreeMap<PublicKey, WgPeer>,
    /// Routing table: allowed IP prefix → peer public key (simplified)
    route_table: Vec<(AllowedIp, PublicKey)>,
    /// Interface IP address
    pub interface_ip: Option<[u8; 4]>,
    /// MTU
    pub mtu: u16,
    /// Packet stats
    pub rx_packets: u64,
    pub tx_packets: u64,
}

impl WgDevice {
    /// Create a new WireGuard device.
    pub fn new(name: &str, private_key: PrivateKey, listen_port: u16) -> Self {
        let public_key = derive_public_key(&private_key);
        Self {
            name: name.into(), private_key, public_key, listen_port, fwmark: 0,
            peers: BTreeMap::new(), route_table: Vec::new(),
            interface_ip: None, mtu: 1420, rx_packets: 0, tx_packets: 0,
        }
    }

    /// Add a peer to this device.
    pub fn add_peer(&mut self, peer: WgPeer) {
        for allowed_ip in &peer.allowed_ips {
            self.route_table.push((allowed_ip.clone(), peer.public_key));
        }
        self.peers.insert(peer.public_key, peer);
    }

    /// Remove a peer.
    pub fn remove_peer(&mut self, key: &PublicKey) -> Option<WgPeer> {
        let peer = self.peers.remove(key);
        self.route_table.retain(|(_, pk)| pk != key);
        peer
    }

    /// Find peer responsible for routing to a destination IP.
    pub fn peer_for_dst(&self, dst: [u8; 4]) -> Option<&WgPeer> {
        for (allowed_ip, pk) in &self.route_table {
            if allowed_ip.matches_ipv4(dst) {
                return self.peers.get(pk);
            }
        }
        None
    }

    /// Simulate encapsulating a plaintext packet for a peer.
    ///
    /// In production: ChaCha20-Poly1305 encrypt + UDP wrap.
    pub fn encapsulate(&mut self, dst_ip: [u8; 4], payload: &[u8]) -> Result<Vec<u8>, &'static str> {
        let peer = self.peers.values_mut()
            .find(|p| p.route_matches(dst_ip))
            .ok_or("no peer for destination")?;

        if peer.session.is_none() {
            return Err("no active session — handshake needed");
        }

        let session = peer.session.as_mut().unwrap();
        let nonce = session.next_send_nonce();

        // Simulated WireGuard data packet format:
        // [type=4u32][receiver_index u32][counter u64][encrypted_payload]
        let mut packet = Vec::new();
        packet.extend_from_slice(&4u32.to_le_bytes()); // type 4 = data
        packet.extend_from_slice(&session.receiving_counter.to_le_bytes()[..4]); // receiver_index stub
        packet.extend_from_slice(&nonce.to_le_bytes());
        // XOR-encrypt stub (real: ChaCha20-Poly1305)
        for b in payload {
            packet.push(b ^ session.sending_key[nonce as usize % 32]);
        }
        // 16-byte poly1305 MAC stub
        packet.extend_from_slice(&[0u8; 16]);

        peer.tx_bytes += payload.len() as u64;
        self.tx_packets += 1;
        Ok(packet)
    }

    /// Simulate decapsulating a received WireGuard packet.
    pub fn decapsulate(&mut self, src_ip: [u8; 4], packet: &[u8]) -> Result<Vec<u8>, &'static str> {
        if packet.len() < 32 { return Err("packet too short"); }

        // Find peer by source IP (simplified)
        let peer = self.peers.values_mut()
            .find(|p| p.endpoint.as_ref().map(|e| e.ip == src_ip).unwrap_or(false))
            .ok_or("unknown peer")?;

        let session = peer.session.as_mut().ok_or("no active session")?;

        // Parse counter from bytes 8..16
        let mut counter_bytes = [0u8; 8];
        counter_bytes.copy_from_slice(&packet[8..16]);
        let counter = u64::from_le_bytes(counter_bytes);

        if !session.check_replay(counter) { return Err("replay attack detected"); }

        // Decrypt (stub: XOR)
        let payload = &packet[16..packet.len() - 16]; // strip MAC
        let decrypted: Vec<u8> = payload.iter()
            .map(|b| b ^ session.receiving_key[counter as usize % 32])
            .collect();

        peer.rx_bytes += decrypted.len() as u64;
        self.rx_packets += 1;
        Ok(decrypted)
    }

    /// Initiate handshake with a peer (stub).
    pub fn initiate_handshake(&mut self, peer_key: &PublicKey, now_ns: u64) -> Result<(), &'static str> {
        let peer = self.peers.get_mut(peer_key).ok_or("peer not found")?;
        let eph = PrivateKey([0x42u8; 32]); // In production: random ephemeral key
        let sender_index = (now_ns & 0xFFFF_FFFF) as u32;
        peer.handshake = Some(WgHandshake::new_initiator(eph, sender_index, now_ns));
        peer.handshake.as_mut().unwrap().state = HandshakeState::InitSent;
        Ok(())
    }

    /// Complete handshake and establish session (stub).
    pub fn complete_handshake(&mut self, peer_key: &PublicKey, now_ns: u64) -> Result<(), &'static str> {
        let peer = self.peers.get_mut(peer_key).ok_or("peer not found")?;
        // Stub: derive session keys from hash of public keys + timestamp
        let mut send_key = [0u8; 32];
        let mut recv_key = [0u8; 32];
        for i in 0..32 {
            send_key[i] = self.public_key.0[i] ^ peer_key.0[i];
            recv_key[i] = peer_key.0[i] ^ self.public_key.0[i] ^ 0xAA;
        }
        peer.session = Some(WgSessionKeys::new(send_key, recv_key, true, now_ns));
        peer.last_handshake_ns = now_ns;
        if let Some(hs) = peer.handshake.as_mut() {
            hs.state = HandshakeState::Established;
        }
        Ok(())
    }

    pub fn peer_count(&self) -> usize { self.peers.len() }
    pub fn get_peer(&self, key: &PublicKey) -> Option<&WgPeer> { self.peers.get(key) }
    pub fn get_peer_mut(&mut self, key: &PublicKey) -> Option<&mut WgPeer> { self.peers.get_mut(key) }
}

// ============================================================
// WireGuard Configuration Parser
// ============================================================

/// Parse a wg-quick style configuration.
pub struct WgConfig {
    pub interface_private_key: Option<PrivateKey>,
    pub listen_port: u16,
    pub interface_address: Option<String>,
    pub dns: Vec<String>,
    pub peers: Vec<WgPeerConfig>,
}

pub struct WgPeerConfig {
    pub public_key: PublicKey,
    pub endpoint: Option<WgEndpoint>,
    pub allowed_ips: Vec<AllowedIp>,
    pub persistent_keepalive: u16,
}

impl WgConfig {
    /// Parse a wg-quick configuration string.
    pub fn parse(config: &str) -> Self {
        let mut cfg = WgConfig {
            interface_private_key: None, listen_port: 51820,
            interface_address: None, dns: Vec::new(), peers: Vec::new(),
        };
        let mut current_peer: Option<WgPeerConfig> = None;

        for line in config.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }

            if line == "[Interface]" {
                if let Some(p) = current_peer.take() { cfg.peers.push(p); }
                continue;
            }
            if line == "[Peer]" {
                if let Some(p) = current_peer.take() { cfg.peers.push(p); }
                current_peer = Some(WgPeerConfig {
                    public_key: PublicKey::zero(),
                    endpoint: None, allowed_ips: Vec::new(), persistent_keepalive: 0,
                });
                continue;
            }

            if let Some((key, val)) = line.split_once(" = ").or_else(|| line.split_once('=')) {
                let val = val.trim();
                if let Some(ref mut peer) = current_peer {
                    match key.trim() {
                        "PublicKey" => {
                            // In production: decode base64, here use first 32 bytes of UTF-8
                            let bytes: Vec<u8> = val.bytes().take(32).collect();
                            let mut arr = [0u8; 32];
                            arr[..bytes.len()].copy_from_slice(&bytes);
                            peer.public_key = PublicKey(arr);
                        }
                        "AllowedIPs" => {
                            for cidr in val.split(',') {
                                let cidr = cidr.trim();
                                if cidr == "0.0.0.0/0" { peer.allowed_ips.push(AllowedIp::any_ipv4()); }
                            }
                        }
                        "Endpoint" => {
                            if let Some((ip_str, port_str)) = val.rsplit_once(':') {
                                let port: u16 = port_str.parse().unwrap_or(51820);
                                let parts: Vec<u8> = ip_str.split('.').filter_map(|p| p.parse().ok()).collect();
                                if parts.len() == 4 {
                                    peer.endpoint = Some(WgEndpoint::new(parts[0], parts[1], parts[2], parts[3], port));
                                }
                            }
                        }
                        "PersistentKeepalive" => {
                            peer.persistent_keepalive = val.parse().unwrap_or(0);
                        }
                        _ => {}
                    }
                } else {
                    match key.trim() {
                        "ListenPort" => { cfg.listen_port = val.parse().unwrap_or(51820); }
                        "Address" => { cfg.interface_address = Some(val.into()); }
                        "DNS" => { cfg.dns.push(val.into()); }
                        _ => {}
                    }
                }
            }
        }
        if let Some(p) = current_peer { cfg.peers.push(p); }
        cfg
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_device(name: &str, seed: u8) -> WgDevice {
        WgDevice::new(name, PrivateKey([seed; 32]), 51820)
    }

    #[test]
    fn test_key_derivation() {
        let priv_key = PrivateKey([0x42u8; 32]);
        let pub_key = derive_public_key(&priv_key);
        assert_ne!(pub_key.0, [0u8; 32]);
        assert_eq!(pub_key.0[0] & 0x7, 0); // Clamped
    }

    #[test]
    fn test_allowed_ip_match() {
        let range = AllowedIp::ipv4(10, 0, 0, 0, 8); // 10.0.0.0/8
        assert!(range.matches_ipv4([10, 1, 2, 3]));
        assert!(range.matches_ipv4([10, 255, 0, 1]));
        assert!(!range.matches_ipv4([192, 168, 1, 1]));
    }

    #[test]
    fn test_peer_routing() {
        let mut dev = make_device("wg0", 0x11);
        let peer_key = PublicKey([0xAA; 32]);
        let mut peer = WgPeer::new(peer_key);
        peer.add_allowed_ip(AllowedIp::ipv4(10, 0, 0, 0, 8));
        peer.set_endpoint(WgEndpoint::new(1, 2, 3, 4, 51820));
        dev.add_peer(peer);
        assert!(dev.peer_for_dst([10, 5, 5, 5]).is_some());
        assert!(dev.peer_for_dst([192, 168, 1, 1]).is_none());
    }

    #[test]
    fn test_handshake_and_encrypt() {
        let mut dev = make_device("wg0", 0x11);
        let peer_key = PublicKey([0xBB; 32]);
        let mut peer = WgPeer::new(peer_key);
        peer.add_allowed_ip(AllowedIp::ipv4(10, 8, 0, 0, 24));
        peer.set_endpoint(WgEndpoint::new(5, 6, 7, 8, 51820));
        dev.add_peer(peer);
        dev.initiate_handshake(&peer_key, 1_000_000_000).unwrap();
        dev.complete_handshake(&peer_key, 1_000_000_000).unwrap();
        let payload = b"Hello SigmaOS WireGuard";
        let encrypted = dev.encapsulate([10, 8, 0, 1], payload).unwrap();
        assert!(encrypted.len() > payload.len());
        assert_eq!(dev.tx_packets, 1);
    }

    #[test]
    fn test_replay_protection() {
        let mut keys = WgSessionKeys::new([0u8; 32], [0u8; 32], true, 0);
        assert!(keys.check_replay(5));
        assert!(!keys.check_replay(5)); // Replay!
        assert!(keys.check_replay(6));
        assert!(keys.check_replay(4)); // Old but in window
    }

    #[test]
    fn test_config_parse() {
        let config = "\
[Interface]
ListenPort = 51820
Address = 10.0.0.1/24

[Peer]
PublicKey = AAABBBCCC
Endpoint = 1.2.3.4:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
";
        let cfg = WgConfig::parse(config);
        assert_eq!(cfg.listen_port, 51820);
        assert_eq!(cfg.peers.len(), 1);
        assert_eq!(cfg.peers[0].persistent_keepalive, 25);
        assert!(cfg.peers[0].endpoint.is_some());
    }
}
