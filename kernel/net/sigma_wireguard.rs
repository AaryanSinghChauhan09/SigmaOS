// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/net/sigma_wireguard.rs — WireGuard VPN (no_std, cleanroom)
// Language: Rust #![no_std] — OOP via WgDevice + WgPeer structs

#![no_std]

pub const WG_KEY_LEN:   usize = 32;
pub const WG_HASH_LEN:  usize = 32;
pub const WG_MAC_LEN:   usize = 16;
pub const WG_TAG_LEN:   usize = 16;
pub const WG_NONCE_LEN: usize = 12;
pub const MAX_PEERS:    usize = 32;
pub const MAX_ALLOWED_IPS: usize = 8;

// ── Noise IK Handshake Constants ─────────────────────────────────────────────
const NOISE_CONSTRUCTION: &[u8] = b"Noise_IKpsk2_25519_ChaChaPoly_BLAKE2s";
const WG_IDENTIFIER:      &[u8] = b"WireGuard v1 zx2c4 Jason@zx2c4.com";

// ── Key Types ─────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Default)]
pub struct PrivKey(pub [u8; WG_KEY_LEN]);
#[derive(Clone, Copy, Default)]
pub struct PubKey(pub [u8; WG_KEY_LEN]);
#[derive(Clone, Copy, Default)]
pub struct PresharedKey(pub [u8; WG_KEY_LEN]);
#[derive(Clone, Copy, Default)]
pub struct SessionKey(pub [u8; WG_KEY_LEN]);

// ── Allowed IP entry ──────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct AllowedIp {
    pub ip:     [u8; 4],
    pub prefix: u8,
}

impl AllowedIp {
    pub fn matches(&self, ip: &[u8; 4]) -> bool {
        if self.prefix == 0 { return true; }
        let bits = self.prefix as u32;
        let mask = if bits >= 32 { 0xFFFF_FFFFu32 } else { !((1u32 << (32 - bits)) - 1) };
        let a = u32::from_be_bytes(*ip);
        let b = u32::from_be_bytes(self.ip);
        (a & mask) == (b & mask)
    }
}

// ── Session State ─────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SessionState { None, Initiating, Responding, Established, Rekeying }

#[derive(Clone, Copy, Default)]
pub struct Session {
    pub state:       SessionState,
    pub send_key:    SessionKey,
    pub recv_key:    SessionKey,
    pub send_nonce:  u64,
    pub recv_nonce:  u64,
    pub local_idx:   u32,
    pub remote_idx:  u32,
}

impl Default for SessionState {
    fn default() -> Self { SessionState::None }
}

// ── Peer ──────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct WgPeer {
    pub public_key:   PubKey,
    pub preshared:    PresharedKey,
    pub endpoint_ip:  [u8; 4],
    pub endpoint_port: u16,
    pub allowed_ips:  [Option<AllowedIp>; MAX_ALLOWED_IPS],
    pub n_allowed:    usize,
    pub session:      Session,
    pub last_handshake: u64, // ticks
    pub rx_bytes:     u64,
    pub tx_bytes:     u64,
    pub enabled:      bool,
}

impl WgPeer {
    pub fn new(pubkey: PubKey) -> Self {
        Self {
            public_key: pubkey,
            preshared:  PresharedKey::default(),
            endpoint_ip: [0;4], endpoint_port: 51820,
            allowed_ips: [const { None }; MAX_ALLOWED_IPS],
            n_allowed: 0,
            session: Session::default(),
            last_handshake: 0, rx_bytes: 0, tx_bytes: 0, enabled: true,
        }
    }

    pub fn add_allowed_ip(&mut self, ip: [u8; 4], prefix: u8) -> bool {
        if self.n_allowed >= MAX_ALLOWED_IPS { return false; }
        self.allowed_ips[self.n_allowed] = Some(AllowedIp { ip, prefix });
        self.n_allowed += 1; true
    }

    pub fn allows_ip(&self, ip: &[u8; 4]) -> bool {
        self.allowed_ips[..self.n_allowed].iter().flatten().any(|a| a.matches(ip))
    }

    pub fn needs_rekey(&self, now: u64) -> bool {
        self.session.state != SessionState::Established
            || now.saturating_sub(self.last_handshake) > 180_000 // 3 min in ms
    }
}

// ── WireGuard Device ──────────────────────────────────────────────────────────
pub struct WgDevice {
    pub private_key:  PrivKey,
    pub public_key:   PubKey,
    pub listen_port:  u16,
    pub fwmark:       u32,
    peers:            [Option<WgPeer>; MAX_PEERS],
    n_peers:          usize,
    pub rx_total:     u64,
    pub tx_total:     u64,
}

impl WgDevice {
    pub fn new(privkey: PrivKey, port: u16) -> Self {
        let pubkey = Self::derive_pubkey(&privkey);
        Self {
            private_key: privkey, public_key: pubkey,
            listen_port: port, fwmark: 0,
            peers: [const { None }; MAX_PEERS],
            n_peers: 0, rx_total: 0, tx_total: 0,
        }
    }

    /// Derive public key from private key (X25519 — placeholder; real impl in crypto)
    fn derive_pubkey(privkey: &PrivKey) -> PubKey {
        // TODO: X25519 scalar multiply with base point
        // For now: echo back (correct impl needed for crypto layer)
        PubKey(privkey.0)
    }

    pub fn add_peer(&mut self, peer: WgPeer) -> bool {
        if self.n_peers >= MAX_PEERS { return false; }
        for slot in &mut self.peers {
            if slot.is_none() { *slot = Some(peer); self.n_peers += 1; return true; }
        }
        false
    }

    pub fn remove_peer(&mut self, pubkey: &PubKey) -> bool {
        for slot in &mut self.peers {
            if matches!(slot, Some(p) if p.public_key.0 == pubkey.0) {
                *slot = None; self.n_peers -= 1; return true;
            }
        }
        false
    }

    pub fn peer_for_ip(&mut self, dst_ip: &[u8; 4]) -> Option<&mut WgPeer> {
        self.peers.iter_mut().flatten().find(|p| p.allows_ip(dst_ip))
    }

    pub fn peer_by_idx(&mut self, idx: u32) -> Option<&mut WgPeer> {
        self.peers.iter_mut().flatten()
            .find(|p| p.session.local_idx == idx)
    }

    /// Encrypt outbound packet payload using ChaCha20-Poly1305
    /// (placeholder — real impl calls kernel/crypto/sigma_chacha20.rs)
    pub fn encrypt_packet(&mut self, peer_ip: &[u8; 4], plaintext: &[u8],
                          out: &mut [u8]) -> usize {
        if let Some(peer) = self.peer_for_ip(peer_ip) {
            let key = &peer.session.send_key.0;
            let nonce = peer.session.send_nonce;
            peer.session.send_nonce += 1;
            // TODO: ChaCha20-Poly1305 encrypt
            // For now: XOR with key bytes (placeholder)
            let n = plaintext.len().min(out.len().saturating_sub(WG_TAG_LEN));
            for i in 0..n { out[i] = plaintext[i] ^ key[i % WG_KEY_LEN]; }
            peer.tx_bytes += n as u64;
            self.tx_total += n as u64;
            return n + WG_TAG_LEN;
        }
        0
    }

    /// Decrypt inbound packet
    pub fn decrypt_packet(&mut self, sender_idx: u32, ciphertext: &[u8],
                          out: &mut [u8]) -> usize {
        if let Some(peer) = self.peer_by_idx(sender_idx) {
            let key = peer.session.recv_key.0;
            let n = ciphertext.len().saturating_sub(WG_TAG_LEN).min(out.len());
            for i in 0..n { out[i] = ciphertext[i] ^ key[i % WG_KEY_LEN]; }
            peer.rx_bytes += n as u64;
            self.rx_total += n as u64;
            return n;
        }
        0
    }
}
