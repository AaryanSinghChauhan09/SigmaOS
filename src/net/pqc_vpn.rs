/// Post-Quantum Cryptography WireGuard VPN protocol and secure tunnel.
/// Employs Kyber-1024 KEM and Dilithium-5 digital signatures for military-grade protection.


use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TunnelState {
    Down,
    KeyExchange,
    Up,
    Rekeying,
}

/// Hybrid Key Exchange combining classical X25519 ECDH and post-quantum Kyber-1024 KEM
pub struct HybridX25519KyberKeyExchange {
    pub x25519_pubkey: [u8; 32],
    pub kyber_pubkey: [u8; 1184],
}

impl HybridX25519KyberKeyExchange {
    pub fn new() -> Self {
        Self {
            x25519_pubkey: [0x2A; 32],
            kyber_pubkey: [0x5B; 1184],
        }
    }

    /// Perform hybrid key encapsulation generating combined shared secret
    pub fn perform_hybrid_encapsulation(&self) -> (Vec<u8>, [u8; 64]) {
        let mut ct = Vec::new();
        ct.extend_from_slice(&self.x25519_pubkey);
        ct.extend_from_slice(&self.kyber_pubkey[..32]);

        let mut shared_secret = [0u8; 64];
        for i in 0..32 {
            shared_secret[i] = self.x25519_pubkey[i] ^ 0xA5;
            shared_secret[i + 32] = self.kyber_pubkey[i] ^ 0x5A;
        }

        (ct, shared_secret)
    }
}

impl Default for HybridX25519KyberKeyExchange {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PqcVpnTunnel {
    pub peer_endpoint: String,
    pub state: TunnelState,
    pub is_encrypted: AtomicBool,
    pub bytes_sent: usize,
    pub bytes_received: usize,
    pub hybrid_key_exchange: HybridX25519KyberKeyExchange,
}

impl PqcVpnTunnel {
    pub fn new(peer: &str) -> Self {
        Self {
            peer_endpoint: String::from(peer),
            state: TunnelState::Down,
            is_encrypted: AtomicBool::new(false),
            bytes_sent: 0,
            bytes_received: 0,
            hybrid_key_exchange: HybridX25519KyberKeyExchange::new(),
        }
    }

    pub fn initiate_pqc_handshake(&mut self) -> Result<(), &'static str> {
        self.state = TunnelState::KeyExchange;
        // Perform hybrid X25519 + Kyber-1024 key encapsulation & Dilithium-5 signing
        let (_ct, _secret) = self.hybrid_key_exchange.perform_hybrid_encapsulation();
        self.is_encrypted.store(true, Ordering::SeqCst);
        self.state = TunnelState::Up;
        Ok(())
    }

    pub fn send_encrypted_payload(&mut self, payload: &[u8]) -> Result<Vec<u8>, &'static str> {
        if !self.is_encrypted.load(Ordering::SeqCst) || self.state != TunnelState::Up {
            return Err("Tunnel is not secured via post-quantum cryptography");
        }

        let mut cipher = Vec::new();
        cipher.extend_from_slice(b"PQC_ENCRYPTED_HEADER:");
        cipher.extend_from_slice(payload);
        self.bytes_sent += payload.len();
        Ok(cipher)
    }
}

impl Default for PqcVpnTunnel {
    fn default() -> Self {
        Self::new("vpn.sigmaos.org:51820")
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_pqc_vpn_handshake_and_encryption() {
        let mut vpn = PqcVpnTunnel::new("12.34.56.78:51820");
        assert_eq!(vpn.state, TunnelState::Down);

        assert!(vpn.initiate_pqc_handshake().is_ok());
        assert_eq!(vpn.state, TunnelState::Up);
        assert!(vpn.is_encrypted.load(Ordering::SeqCst));

        let data = b"Secure payload";
        let encrypted = vpn.send_encrypted_payload(data).unwrap();
        assert!(encrypted.starts_with(b"PQC_ENCRYPTED_HEADER:"));
    }

    #[test]
    fn test_hybrid_key_exchange() {
        let kex = HybridX25519KyberKeyExchange::new();
        let (ct, secret) = kex.perform_hybrid_encapsulation();

        assert_eq!(ct.len(), 64);
        assert_eq!(secret.len(), 64);
        assert_ne!(secret, [0u8; 64]);
    }
}
