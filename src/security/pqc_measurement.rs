// SigmaOS Post-Quantum Hybrid Signature & Firmware Measurement Engine
// Combines Kyber-1024 KEM with a firmware measurement hash chain
// for secure-boot / measured-boot narrative

extern crate alloc;
use alloc::vec::Vec;
use crate::klib::merkle::{MerkleAccumulator, MerkleHash};
use crate::security::pqc_enclave::KyberKem;

/// Hybrid PQC signature state combining lattice-based KEM with firmware measurements
pub struct HybridPqcMeasurementEngine {
    kem: KyberKem,
    measurements: MerkleAccumulator,
    firmware_chain: Vec<MerkleHash>,
}

impl HybridPqcMeasurementEngine {
    /// Create a new hybrid PQC measurement engine
    pub fn new() -> Self {
        HybridPqcMeasurementEngine {
            kem: KyberKem::new(),
            measurements: MerkleAccumulator::new(),
            firmware_chain: Vec::new(),
        }
    }

    /// Record a firmware measurement and extend the hash chain
    pub fn record_firmware_measurement(&mut self, measurement: &[u8]) -> MerkleHash {
        let hash = self.measurements.append(measurement);
        self.firmware_chain.push(hash);
        hash
    }

    /// Encapsulate a shared secret using Kyber-1024 KEM
    pub fn encapsulate(&self, peer_pubkey: &[u8; 32]) -> (Vec<u8>, [u8; 32]) {
        self.kem.encapsulate(peer_pubkey)
    }

    /// Decapsulate a ciphertext to retrieve the shared secret
    pub fn decapsulate(&self, ciphertext: &[u8]) -> [u8; 32] {
        self.kem.decapsulate(ciphertext)
    }

    /// Produce a hybrid signature digest binding `message` to the KEM-derived
    /// shared secret. The same ciphertext must be supplied at verification time so
    /// the secret can be recovered via decapsulation.
    pub fn sign(&self, message: &[u8], ciphertext: &[u8]) -> MerkleHash {
        let shared_secret = self.kem.decapsulate(ciphertext);
        let mut combined = Vec::new();
        combined.extend_from_slice(message);
        combined.extend_from_slice(&shared_secret);
        Self::digest_of(&combined)
    }

    /// Verify a hybrid signature: re-derive the digest from `message` and the
    /// decapsulated secret and compare it against `expected_root`. Because the
    /// digest folds in the message, a different message yields a different digest
    /// and fails verification.
    pub fn verify_hybrid_signature(
        &self,
        message: &[u8],
        ciphertext: &[u8],
        expected_root: MerkleHash,
    ) -> bool {
        let digest = self.sign(message, ciphertext);
        digest == expected_root
    }

fn digest_of(data: &[u8]) -> [u8; 32] {
    let mut h: u32 = 0x811C_9DC5;
    for &b in data {
        h ^= b as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    let mut out = [0u8; 32];
    for i in 0..32u32 {
        h ^= i.wrapping_mul(0x9E37_79B1);
        h = h.wrapping_mul(0x0100_0193);
        let bytes = h.to_be_bytes();
        out[i as usize] = bytes[0] ^ bytes[3];
    }
    out
}

    /// Get the current firmware measurement root
    pub fn measurement_root(&self) -> Option<MerkleHash> {
        self.measurements.root()
    }

    /// Number of recorded firmware measurements
    pub fn measurement_count(&self) -> usize {
        self.firmware_chain.len()
    }

    /// Verify a single firmware measurement by index
    pub fn verify_measurement(&self, index: usize, data: &[u8]) -> bool {
        self.measurements.verify_leaf(index, data)
    }
}

impl Default for HybridPqcMeasurementEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_pqc_new() {
        let engine = HybridPqcMeasurementEngine::new();
        assert!(engine.measurement_root().is_none());
        assert_eq!(engine.measurement_count(), 0);
    }

    #[test]
    fn test_record_firmware_measurements() {
        let mut engine = HybridPqcMeasurementEngine::new();
        let h1 = engine.record_firmware_measurement(b"bootloader");
        let h2 = engine.record_firmware_measurement(b"kernel");

        assert_eq!(engine.measurement_count(), 2);
        assert_ne!(h1, h2);
        let root = engine.measurement_root().unwrap();
        assert!(engine.verify_measurement(0, b"bootloader"));
        assert!(engine.verify_measurement(1, b"kernel"));
        assert!(!engine.verify_measurement(1, b"bootloader"));
    }

    #[test]
    fn test_encapsulate_decapsulate() {
        let engine = HybridPqcMeasurementEngine::new();
        let peer_pubkey = [0xAB; 32];
        let (ciphertext, secret1) = engine.encapsulate(&peer_pubkey);
        let secret2 = engine.decapsulate(&ciphertext);
        assert_eq!(secret1, secret2);
    }

    #[test]
    fn test_verify_hybrid_signature() {
        let mut engine = HybridPqcMeasurementEngine::new();
        engine.record_firmware_measurement(b"bootloader");
        engine.record_firmware_measurement(b"kernel");

        let peer_pubkey = [0xAB; 32];
        let (ciphertext, _secret) = engine.encapsulate(&peer_pubkey);
        let root = engine.measurement_root().unwrap();
        let sig = engine.sign(b"boot-chain", &ciphertext);

        assert!(engine.verify_hybrid_signature(b"boot-chain", &ciphertext, sig));
        assert!(!engine.verify_hybrid_signature(b"wrong-chain", &ciphertext, sig));
        // An unrelated digest (here the firmware root) must not verify.
        assert!(!engine.verify_hybrid_signature(b"boot-chain", &ciphertext, root));
    }

    #[test]
    fn test_hybrid_signature_wrong_root() {
        let mut engine = HybridPqcMeasurementEngine::new();
        engine.record_firmware_measurement(b"bootloader");
        let peer_pubkey = [0xAB; 32];
        let (ciphertext, _secret) = engine.encapsulate(&peer_pubkey);
        let sig = engine.sign(b"msg", &ciphertext);
        let wrong_root = [0xFF; 32];
        assert!(!engine.verify_hybrid_signature(b"msg", &ciphertext, wrong_root));
    }
}
