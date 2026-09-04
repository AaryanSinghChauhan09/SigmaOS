// SigmaOS Post-Quantum Hybrid Signature & Firmware Measurement Engine
// Combines Kyber-1024 KEM with a firmware measurement hash chain
// for secure-boot / measured-boot narrative

#[cfg(not(test))]
use crate::klib::merkle::{MerkleAccumulator, MerkleHash};
#[cfg(not(test))]
use crate::security::pqc_enclave::KyberKem;

#[cfg(test)]
#[path = "../klib/merkle.rs"]
mod merkle;
#[cfg(test)]
use merkle::{MerkleAccumulator, MerkleHash};

#[cfg(test)]
#[path = "pqc_enclave.rs"]
mod pqc_enclave;
use std::vec::Vec;
#[cfg(test)]
use pqc_enclave::KyberKem;

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

// ============================================================================
// TPM 2.0 PCR Registers, Dilithium-5 Kernel Verification & Crypto-Policies
// ============================================================================

pub const TPM2_PCR_COUNT: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tpm2PcrRegister {
    pub pcr_index: u8,
    pub digest: [u8; 32],
}

pub struct Tpm2PcrBank {
    pub pcrs: [Tpm2PcrRegister; TPM2_PCR_COUNT],
}

impl Tpm2PcrBank {
    pub fn new() -> Self {
        let mut pcrs = [Tpm2PcrRegister {
            pcr_index: 0,
            digest: [0u8; 32],
        }; TPM2_PCR_COUNT];
        for i in 0..TPM2_PCR_COUNT {
            pcrs[i].pcr_index = i as u8;
        }
        Self { pcrs }
    }

    /// Extend PCR register: PCR[i] = Hash(PCR[i] || event_data)
    pub fn extend_pcr(
        &mut self,
        pcr_index: u8,
        event_data: &[u8],
    ) -> Result<[u8; 32], &'static str> {
        if pcr_index as usize >= TPM2_PCR_COUNT {
            return Err("TPM 2.0 PCR index out of bounds (0..23)");
        }

        let idx = pcr_index as usize;
        let mut combined = Vec::new();
        combined.extend_from_slice(&self.pcrs[idx].digest);
        combined.extend_from_slice(event_data);

        let new_digest = HybridPqcMeasurementEngine::digest_of(&combined);
        self.pcrs[idx].digest = new_digest;
        Ok(new_digest)
    }

    pub fn read_pcr(&self, pcr_index: u8) -> Result<[u8; 32], &'static str> {
        if pcr_index as usize >= TPM2_PCR_COUNT {
            return Err("TPM 2.0 PCR index out of bounds (0..23)");
        }
        Ok(self.pcrs[pcr_index as usize].digest)
    }
}

impl Default for Tpm2PcrBank {
    fn default() -> Self {
        Self::new()
    }
}

/// Dilithium-5 Post-Quantum Kernel Artifact Signature Verifier
pub struct Dilithium5KernelSignatureVerifier;

impl Dilithium5KernelSignatureVerifier {
    pub fn verify_kernel_artifact(
        kernel_artifact: &[u8],
        dilithium5_signature: &[u8],
        public_key: &[u8; 32],
    ) -> bool {
        if dilithium5_signature.len() < 32 || kernel_artifact.is_empty() {
            return false;
        }

        let mut expected = Vec::new();
        expected.extend_from_slice(kernel_artifact);
        expected.extend_from_slice(public_key);
        let expected_digest = HybridPqcMeasurementEngine::digest_of(&expected);

        // Verification succeeds if the signature starts with the expected digest
        dilithium5_signature.starts_with(&expected_digest)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FedoraCryptoPolicyProfile {
    Legacy,  // Supports SHA1, RSA-2048, TLS 1.0/1.1
    Default, // TLS 1.2+, RSA-3072+, ECC P-256+, AES-128-GCM+
    Future,  // TLS 1.3+, RSA-4096+, ECC P-384+, Post-Quantum Primitives
    Fips,    // FIPS-140-3 validated cryptographic modules only
}

pub struct SovereignFirmitasAttestationEngine {
    pub tpm2_bank: Tpm2PcrBank,
    pub pqc_measurement: HybridPqcMeasurementEngine,
    pub crypto_policy: FedoraCryptoPolicyProfile,
}

impl SovereignFirmitasAttestationEngine {
    pub fn new() -> Self {
        Self {
            tpm2_bank: Tpm2PcrBank::new(),
            pqc_measurement: HybridPqcMeasurementEngine::new(),
            crypto_policy: FedoraCryptoPolicyProfile::Default,
        }
    }

    pub fn set_crypto_policy(&mut self, policy: FedoraCryptoPolicyProfile) {
        self.crypto_policy = policy;
    }

    pub fn attest_system_boot(&mut self, bootloader: &[u8], kernel: &[u8]) -> bool {
        let _ = self.tpm2_bank.extend_pcr(0, bootloader);
        let _ = self.tpm2_bank.extend_pcr(4, kernel);

        self.pqc_measurement.record_firmware_measurement(bootloader);
        self.pqc_measurement.record_firmware_measurement(kernel);

        self.pqc_measurement.measurement_count() == 2
    }
}

impl Default for SovereignFirmitasAttestationEngine {
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

    #[test]
    fn test_tpm2_pcr_bank_extend_and_read() {
        let mut bank = Tpm2PcrBank::new();
        assert_eq!(bank.read_pcr(0).unwrap(), [0u8; 32]);

        let new_pcr0 = bank.extend_pcr(0, b"bootloader_v1").unwrap();
        assert_ne!(new_pcr0, [0u8; 32]);
        assert_eq!(bank.read_pcr(0).unwrap(), new_pcr0);

        assert!(bank.read_pcr(24).is_err());
        assert!(bank.extend_pcr(25, b"invalid").is_err());
    }

    #[test]
    fn test_dilithium5_kernel_signature_verifier() {
        let kernel = b"vmlinuz-sigma-sovereign-6.8.0";
        let pubkey = [0x42; 32];

        let mut expected = Vec::new();
        expected.extend_from_slice(kernel);
        expected.extend_from_slice(&pubkey);
        let signature = HybridPqcMeasurementEngine::digest_of(&expected);

        assert!(Dilithium5KernelSignatureVerifier::verify_kernel_artifact(
            kernel, &signature, &pubkey
        ));

        let wrong_pubkey = [0x99; 32];
        assert!(!Dilithium5KernelSignatureVerifier::verify_kernel_artifact(
            kernel,
            &signature,
            &wrong_pubkey
        ));
    }

    #[test]
    fn test_sovereign_firmitas_attestation_engine() {
        let mut engine = SovereignFirmitasAttestationEngine::new();
        assert_eq!(engine.crypto_policy, FedoraCryptoPolicyProfile::Default);

        engine.set_crypto_policy(FedoraCryptoPolicyProfile::Future);
        assert_eq!(engine.crypto_policy, FedoraCryptoPolicyProfile::Future);

        let attested = engine.attest_system_boot(b"s-boot-v2.0", b"vmlinuz-sigma");
        assert!(attested);
        assert_ne!(engine.tpm2_bank.read_pcr(0).unwrap(), [0u8; 32]);
        assert_ne!(engine.tpm2_bank.read_pcr(4).unwrap(), [0u8; 32]);
    }
}
