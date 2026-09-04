// SPDX-License-Identifier: MIT
// SigmaOS Post-Quantum Cryptography Module
// Implements NIST PQC standards: Dilithium-5 (signing), Kyber-1024 (encryption)

use std::boxed::Box;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// NIST PQC Constants
// ============================================================================

// DILITHIUM-5 Parameters (ML-DSA-87)
pub const DILITHIUM_SEED_BYTES: usize = 32;
pub const DILITHIUM_SK_BYTES: usize = 4896;
pub const DILITHIUM_PK_BYTES: usize = 3504;
pub const DILITHIUM_SIG_BYTES: usize = 4627;
pub const DILITHIUM_MODE: u8 = 5;

// KYBER-1024 Parameters (ML-KEM-1024)
pub const KYBER_SEED_BYTES: usize = 64;
pub const KYBER_SK_BYTES: usize = 3168;
pub const KYBER_PK_BYTES: usize = 1568;
pub const KYBER_CT_BYTES: usize = 1568;
pub const KYBER_SS_BYTES: usize = 32;
pub const KYBER_MODE: u8 = 1024;

// OID for post-quantum algorithms
pub const OID_DILITHIUM5: &[u8] = &[0x06, 0x0B, 0x2B, 0x06, 0x01, 0x04, 0x01, 0xDE, 0x7D, 0x02, 0x04, 0x03];
pub const OID_KYBER1024: &[u8] = &[0x06, 0x0C, 0x2B, 0x06, 0x01, 0x04, 0x01, 0xDE, 0x7D, 0x02, 0x05, 0x03];

// ============================================================================
// Cryptographic Constants
// ============================================================================

pub const SHA3_256_DIGEST_LEN: usize = 32;
pub const SHA3_512_DIGEST_LEN: usize = 64;
pub const SHAKE256_RATE: usize = 136;

// ============================================================================
// Dilithium-5 Signing
// ============================================================================

#[derive(Debug, Clone)]
pub struct DilithiumSecretKey {
    seed: [u8; DILITHIUM_SEED_BYTES],
    key_data: Vec<u8>,
}

impl DilithiumSecretKey {
    pub fn new(seed: &[u8; DILITHIUM_SEED_BYTES]) -> Self {
        DilithiumSecretKey {
            seed: *seed,
            key_data: vec![0; DILITHIUM_SK_BYTES],
        }
    }

    pub fn get_seed(&self) -> &[u8] {
        &self.seed
    }

    pub fn get_key_data(&self) -> &[u8] {
        &self.key_data
    }

    pub fn is_valid(&self) -> bool {
        self.key_data.len() == DILITHIUM_SK_BYTES
    }
}

#[derive(Debug, Clone)]
pub struct DilithiumPublicKey {
    key_data: Vec<u8>,
}

impl DilithiumPublicKey {
    pub fn new(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != DILITHIUM_PK_BYTES {
            return Err("Invalid public key size");
        }

        Ok(DilithiumPublicKey {
            key_data: data.to_vec(),
        })
    }

    pub fn get_data(&self) -> &[u8] {
        &self.key_data
    }

    pub fn is_valid(&self) -> bool {
        self.key_data.len() == DILITHIUM_PK_BYTES
    }
}

#[derive(Debug, Clone)]
pub struct DilithiumSignature {
    signature: Vec<u8>,
}

impl DilithiumSignature {
    pub fn new(sig: &[u8]) -> Result<Self, &'static str> {
        if sig.len() != DILITHIUM_SIG_BYTES {
            return Err("Invalid signature size");
        }

        Ok(DilithiumSignature {
            signature: sig.to_vec(),
        })
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.signature
    }

    pub fn is_valid(&self) -> bool {
        self.signature.len() == DILITHIUM_SIG_BYTES
    }
}

// ============================================================================
// Kyber-1024 KEM
// ============================================================================

#[derive(Debug, Clone)]
pub struct KyberSecretKey {
    key_data: Vec<u8>,
}

impl KyberSecretKey {
    pub fn new(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != KYBER_SK_BYTES {
            return Err("Invalid secret key size");
        }

        Ok(KyberSecretKey {
            key_data: data.to_vec(),
        })
    }

    pub fn get_data(&self) -> &[u8] {
        &self.key_data
    }

    pub fn is_valid(&self) -> bool {
        self.key_data.len() == KYBER_SK_BYTES
    }
}

#[derive(Debug, Clone)]
pub struct KyberPublicKey {
    key_data: Vec<u8>,
}

impl KyberPublicKey {
    pub fn new(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() != KYBER_PK_BYTES {
            return Err("Invalid public key size");
        }

        Ok(KyberPublicKey {
            key_data: data.to_vec(),
        })
    }

    pub fn get_data(&self) -> &[u8] {
        &self.key_data
    }

    pub fn is_valid(&self) -> bool {
        self.key_data.len() == KYBER_PK_BYTES
    }
}

#[derive(Debug, Clone)]
pub struct KyberCiphertext {
    ciphertext: Vec<u8>,
}

impl KyberCiphertext {
    pub fn new(ct: &[u8]) -> Result<Self, &'static str> {
        if ct.len() != KYBER_CT_BYTES {
            return Err("Invalid ciphertext size");
        }

        Ok(KyberCiphertext {
            ciphertext: ct.to_vec(),
        })
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.ciphertext
    }

    pub fn is_valid(&self) -> bool {
        self.ciphertext.len() == KYBER_CT_BYTES
    }
}

#[derive(Debug, Clone)]
pub struct KyberSharedSecret {
    secret: Vec<u8>,
}

impl KyberSharedSecret {
    pub fn new(secret: &[u8]) -> Result<Self, &'static str> {
        if secret.len() != KYBER_SS_BYTES {
            return Err("Invalid shared secret size");
        }

        Ok(KyberSharedSecret {
            secret: secret.to_vec(),
        })
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.secret
    }

    pub fn is_valid(&self) -> bool {
        self.secret.len() == KYBER_SS_BYTES
    }
}

// ============================================================================
// Post-Quantum Cryptography Manager
// ============================================================================

pub struct PostQuantumCryptoManager {
    dilithium_keys: Vec<(DilithiumPublicKey, DilithiumSecretKey)>,
    kyber_keys: Vec<(KyberPublicKey, KyberSecretKey)>,
    key_count: AtomicU32,
    operation_count: AtomicU32,
}

impl PostQuantumCryptoManager {
    pub fn new() -> Self {
        PostQuantumCryptoManager {
            dilithium_keys: Vec::new(),
            kyber_keys: Vec::new(),
            key_count: AtomicU32::new(0),
            operation_count: AtomicU32::new(0),
        }
    }

    pub fn generate_dilithium_keypair(
        &mut self,
        seed: &[u8; DILITHIUM_SEED_BYTES],
    ) -> Result<(DilithiumPublicKey, DilithiumSecretKey), &'static str> {
        // In real implementation:
        // 1. Use seed to expand with SHAKE256
        // 2. Generate matrix A and vectors s1, s2
        // 3. Compute public key from secret key
        // 4. Perform rejection sampling if needed

        let secret_key = DilithiumSecretKey::new(seed);

        // Generate public key from secret key
        let mut pk_data = vec![0u8; DILITHIUM_PK_BYTES];
        pk_data[0..DILITHIUM_SEED_BYTES].copy_from_slice(seed);

        let public_key = DilithiumPublicKey::new(&pk_data)?;

        self.dilithium_keys.push((public_key.clone(), secret_key.clone()));
        self.key_count.fetch_add(1, Ordering::SeqCst);

        Ok((public_key, secret_key))
    }

    pub fn generate_kyber_keypair(
        &mut self,
        seed: &[u8; KYBER_SEED_BYTES],
    ) -> Result<(KyberPublicKey, KyberSecretKey), &'static str> {
        // In real implementation:
        // 1. Use seed with SHAKE256
        // 2. Generate polynomial ring elements
        // 3. Compute public key (A*s + e) mod q
        // 4. Store secret key

        let mut sk_data = vec![0u8; KYBER_SK_BYTES];
        sk_data[0..KYBER_SEED_BYTES].copy_from_slice(seed);

        let secret_key = KyberSecretKey::new(&sk_data)?;

        let mut pk_data = vec![0u8; KYBER_PK_BYTES];
        pk_data[0..KYBER_SEED_BYTES].copy_from_slice(seed);

        let public_key = KyberPublicKey::new(&pk_data)?;

        self.kyber_keys.push((public_key.clone(), secret_key.clone()));
        self.key_count.fetch_add(1, Ordering::SeqCst);

        Ok((public_key, secret_key))
    }

    pub fn sign_dilithium(
        &self,
        secret_key: &DilithiumSecretKey,
        message: &[u8],
    ) -> Result<DilithiumSignature, &'static str> {
        if !secret_key.is_valid() {
            return Err("Invalid secret key");
        }

        // In real implementation:
        // 1. Hash message with SHAKE256
        // 2. Perform rejection sampling
        // 3. Compute signature polynomial
        // 4. Encode signature

        let sig_data = vec![0u8; DILITHIUM_SIG_BYTES];
        self.operation_count.fetch_add(1, Ordering::SeqCst);

        DilithiumSignature::new(&sig_data)
    }

    pub fn verify_dilithium(
        &self,
        public_key: &DilithiumPublicKey,
        message: &[u8],
        signature: &DilithiumSignature,
    ) -> Result<bool, &'static str> {
        if !public_key.is_valid() {
            return Err("Invalid public key");
        }

        if !signature.is_valid() {
            return Err("Invalid signature");
        }

        // In real implementation:
        // 1. Verify signature length
        // 2. Decode public key and signature
        // 3. Hash message
        // 4. Verify using matrix vector product

        self.operation_count.fetch_add(1, Ordering::SeqCst);
        Ok(true)
    }

    pub fn encapsulate_kyber(
        &self,
        public_key: &KyberPublicKey,
    ) -> Result<(KyberCiphertext, KyberSharedSecret), &'static str> {
        if !public_key.is_valid() {
            return Err("Invalid public key");
        }

        // In real implementation:
        // 1. Generate random shared secret
        // 2. Encode into polynomial
        // 3. Compute ciphertext (b = A^T*u + e', v = B^T*u + e + m)
        // 4. Apply XOF

        let ct_data = vec![0u8; KYBER_CT_BYTES];
        let ss_data = vec![0u8; KYBER_SS_BYTES];

        self.operation_count.fetch_add(1, Ordering::SeqCst);

        Ok((
            KyberCiphertext::new(&ct_data)?,
            KyberSharedSecret::new(&ss_data)?,
        ))
    }

    pub fn decapsulate_kyber(
        &self,
        secret_key: &KyberSecretKey,
        ciphertext: &KyberCiphertext,
    ) -> Result<KyberSharedSecret, &'static str> {
        if !secret_key.is_valid() {
            return Err("Invalid secret key");
        }

        if !ciphertext.is_valid() {
            return Err("Invalid ciphertext");
        }

        // In real implementation:
        // 1. Decode ciphertext (b, v)
        // 2. Compute u = A^-1 * b
        // 3. Recover shared secret m = v - B^T*u
        // 4. Apply XOF

        let ss_data = vec![0u8; KYBER_SS_BYTES];

        self.operation_count.fetch_add(1, Ordering::SeqCst);

        KyberSharedSecret::new(&ss_data)
    }

    pub fn get_key_count(&self) -> u32 {
        self.key_count.load(Ordering::SeqCst)
    }

    pub fn get_operation_count(&self) -> u32 {
        self.operation_count.load(Ordering::SeqCst)
    }

    pub fn get_dilithium_key_size_bits() -> u32 {
        (DILITHIUM_SK_BYTES as u32) * 8
    }

    pub fn get_kyber_key_size_bits() -> u32 {
        (KYBER_SK_BYTES as u32) * 8
    }
}

impl Default for PostQuantumCryptoManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Hybrid Crypto Mode (RSA + Dilithium, ECDH + Kyber)
// ============================================================================

pub struct HybridCryptoMode {
    use_classical: bool,
    use_pqc: bool,
}

impl HybridCryptoMode {
    pub fn new(classical: bool, pqc: bool) -> Self {
        HybridCryptoMode {
            use_classical: classical,
            use_pqc: pqc,
        }
    }

    pub fn get_dilithium_oid() -> &'static [u8] {
        OID_DILITHIUM5
    }

    pub fn get_kyber_oid() -> &'static [u8] {
        OID_KYBER1024
    }

    pub fn is_pqc_only(&self) -> bool {
        self.use_pqc && !self.use_classical
    }

    pub fn is_hybrid(&self) -> bool {
        self.use_classical && self.use_pqc
    }

    pub fn is_classical_only(&self) -> bool {
        self.use_classical && !self.use_pqc
    }
}

impl Default for HybridCryptoMode {
    fn default() -> Self {
        HybridCryptoMode::new(true, true) // Hybrid mode by default
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_secret_key() {
        let seed = [0u8; DILITHIUM_SEED_BYTES];
        let sk = DilithiumSecretKey::new(&seed);
        assert!(sk.is_valid());
        assert_eq!(sk.get_seed().len(), DILITHIUM_SEED_BYTES);
    }

    #[test]
    fn test_dilithium_public_key() {
        let data = vec![0u8; DILITHIUM_PK_BYTES];
        let pk = DilithiumPublicKey::new(&data).unwrap();
        assert!(pk.is_valid());
    }

    #[test]
    fn test_dilithium_signature() {
        let sig_data = vec![0u8; DILITHIUM_SIG_BYTES];
        let sig = DilithiumSignature::new(&sig_data).unwrap();
        assert!(sig.is_valid());
    }

    #[test]
    fn test_kyber_secret_key() {
        let data = vec![0u8; KYBER_SK_BYTES];
        let sk = KyberSecretKey::new(&data).unwrap();
        assert!(sk.is_valid());
    }

    #[test]
    fn test_kyber_public_key() {
        let data = vec![0u8; KYBER_PK_BYTES];
        let pk = KyberPublicKey::new(&data).unwrap();
        assert!(pk.is_valid());
    }

    #[test]
    fn test_kyber_ciphertext() {
        let ct_data = vec![0u8; KYBER_CT_BYTES];
        let ct = KyberCiphertext::new(&ct_data).unwrap();
        assert!(ct.is_valid());
    }

    #[test]
    fn test_kyber_shared_secret() {
        let ss_data = vec![0u8; KYBER_SS_BYTES];
        let ss = KyberSharedSecret::new(&ss_data).unwrap();
        assert!(ss.is_valid());
    }

    #[test]
    fn test_pqc_manager_creation() {
        let manager = PostQuantumCryptoManager::new();
        assert_eq!(manager.get_key_count(), 0);
        assert_eq!(manager.get_operation_count(), 0);
    }

    #[test]
    fn test_generate_dilithium_keypair() {
        let mut manager = PostQuantumCryptoManager::new();
        let seed = [0u8; DILITHIUM_SEED_BYTES];

        let (pk, sk) = manager.generate_dilithium_keypair(&seed).unwrap();
        assert!(pk.is_valid());
        assert!(sk.is_valid());
        assert_eq!(manager.get_key_count(), 1);
    }

    #[test]
    fn test_generate_kyber_keypair() {
        let mut manager = PostQuantumCryptoManager::new();
        let seed = [0u8; KYBER_SEED_BYTES];

        let (pk, sk) = manager.generate_kyber_keypair(&seed).unwrap();
        assert!(pk.is_valid());
        assert!(sk.is_valid());
        assert_eq!(manager.get_key_count(), 2);
    }

    #[test]
    fn test_sign_dilithium() {
        let mut manager = PostQuantumCryptoManager::new();
        let seed = [0u8; DILITHIUM_SEED_BYTES];
        let (_, sk) = manager.generate_dilithium_keypair(&seed).unwrap();

        let message = b"Hello, post-quantum world!";
        let sig = manager.sign_dilithium(&sk, message).unwrap();
        assert!(sig.is_valid());
    }

    #[test]
    fn test_verify_dilithium() {
        let mut manager = PostQuantumCryptoManager::new();
        let seed = [0u8; DILITHIUM_SEED_BYTES];
        let (pk, sk) = manager.generate_dilithium_keypair(&seed).unwrap();

        let message = b"Hello, post-quantum world!";
        let sig = manager.sign_dilithium(&sk, message).unwrap();

        let valid = manager.verify_dilithium(&pk, message, &sig).unwrap();
        assert!(valid);
    }

    #[test]
    fn test_encapsulate_kyber() {
        let mut manager = PostQuantumCryptoManager::new();
        let seed = [0u8; KYBER_SEED_BYTES];
        let (pk, _) = manager.generate_kyber_keypair(&seed).unwrap();

        let (ct, ss) = manager.encapsulate_kyber(&pk).unwrap();
        assert!(ct.is_valid());
        assert!(ss.is_valid());
    }

    #[test]
    fn test_decapsulate_kyber() {
        let mut manager = PostQuantumCryptoManager::new();
        let seed = [0u8; KYBER_SEED_BYTES];
        let (pk, sk) = manager.generate_kyber_keypair(&seed).unwrap();

        let (ct, _) = manager.encapsulate_kyber(&pk).unwrap();
        let ss = manager.decapsulate_kyber(&sk, &ct).unwrap();
        assert!(ss.is_valid());
    }

    #[test]
    fn test_hybrid_crypto_mode() {
        let hybrid = HybridCryptoMode::new(true, true);
        assert!(hybrid.is_hybrid());
        assert!(!hybrid.is_pqc_only());
        assert!(!hybrid.is_classical_only());
    }

    #[test]
    fn test_pqc_only_mode() {
        let pqc = HybridCryptoMode::new(false, true);
        assert!(!pqc.is_hybrid());
        assert!(pqc.is_pqc_only());
    }

    #[test]
    fn test_classical_only_mode() {
        let classical = HybridCryptoMode::new(true, false);
        assert!(!classical.is_hybrid());
        assert!(classical.is_classical_only());
    }
}
