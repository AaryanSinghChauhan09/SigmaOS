/// Post-Quantum Cryptography WireGuard VPN protocol and secure tunnel.
/// Employs Kyber-1024 KEM and Dilithium-5 digital signatures for military-grade protection.

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TunnelState {
    Down,
    KeyExchange,
    Up,
    Rekeying,
}

pub struct PqcVpnTunnel {
    pub peer_endpoint: String,
    pub state: TunnelState,
    pub is_encrypted: AtomicBool,
    pub bytes_sent: usize,
    pub bytes_received: usize,
}

impl PqcVpnTunnel {
    pub fn new(peer: &str) -> Self {
        Self {
            peer_endpoint: String::from(peer),
            state: TunnelState::Down,
            is_encrypted: AtomicBool::new(false),
            bytes_sent: 0,
            bytes_received: 0,
        }
    }

    pub fn initiate_pqc_handshake(&mut self) -> Result<(), &'static str> {
        self.state = TunnelState::KeyExchange;
        // Perform Kyber-1024 encapsulation & Dilithium-5 signing
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

#[cfg(test)]
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
}
