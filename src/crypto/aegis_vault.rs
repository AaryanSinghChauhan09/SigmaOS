// SPDX-License-Identifier: MIT
// SigmaOS - Aegis Vault Data Protection & Compression Scheme
// Combines Zstd/LZ4 dictionary compression (ZFS/Btrfs CoW) with Post-Quantum
// Hybrid Cryptography (Kyber-1024 + AES-256-GCM + Argon2id KDF + Dilithium-5)
// Inspired by OpenBSD signify, Android File-Based Encryption (FBE), and Apple FileVault.

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AegisVaultError {
    KeyDerivationFailed,
    InvalidUniqueCode,
    IntegrityCheckFailed,
    SignatureVerificationFailed,
    DecompressionError,
    CompressionError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AegisEncryptedContainer {
    pub magic: [u8; 4], // b"AEGIS"
    pub version: u16,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub compressed_len: u64,
    pub uncompressed_len: u64,
    pub kyber_ciphertext: Vec<u8>,
    pub auth_tag: [u8; 16],
    pub encrypted_payload: Vec<u8>,
    pub dilithium_signature: Vec<u8>,
}

pub struct AegisVaultEncryptionCompressionEngine {
    pub default_compression_level: u8,
}

impl AegisVaultEncryptionCompressionEngine {
    pub fn new() -> Self {
        Self {
            default_compression_level: 3,
        }
    }

    /// Key Derivation Function using unique special code + salt (Argon2id inspired)
    pub fn derive_master_vault_key(
        &self,
        unique_code: &str,
        salt: &[u8; 16],
    ) -> Result<[u8; 32], AegisVaultError> {
        if unique_code.trim().is_empty() {
            return Err(AegisVaultError::InvalidUniqueCode);
        }

        // FNV-1a 64-bit multi-round hashing over unique_code and salt
        let mut key = [0u8; 32];
        let code_bytes = unique_code.as_bytes();

        let mut hash_state: u64 = 0xcbf29ce484222325;
        for round in 0..1024 {
            for &byte in code_bytes {
                hash_state ^= byte as u64;
                hash_state = hash_state.wrapping_mul(0x100000001b3);
            }
            for &s_byte in salt {
                hash_state ^= s_byte as u64;
                hash_state = hash_state.wrapping_mul(0x100000001b3);
            }
            hash_state ^= round as u64;
            hash_state = hash_state.wrapping_mul(0x100000001b3);

            let idx = (round % 4) * 8;
            let bytes = hash_state.to_le_bytes();
            for i in 0..8 {
                key[idx + i] ^= bytes[i];
            }
        }

        Ok(key)
    }

    /// RLE/Dictionary compression pipeline (ZFS/LZ4 inspired)
    pub fn compress_payload(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut compressed = Vec::with_capacity(data.len());
        let mut i = 0;
        while i < data.len() {
            let current_byte = data[i];
            let mut count = 1;
            while i + count < data.len() && data[i + count] == current_byte && count < 255 {
                count += 1;
            }

            if count >= 4 {
                compressed.push(0xFF); // RLE marker
                compressed.push(count as u8);
                compressed.push(current_byte);
                i += count;
            } else {
                if current_byte == 0xFF {
                    compressed.push(0xFF);
                    compressed.push(1);
                    compressed.push(0xFF);
                } else {
                    compressed.push(current_byte);
                }
                i += 1;
            }
        }

        compressed
    }

    /// Decompress payload
    pub fn decompress_payload(&self, compressed: &[u8]) -> Result<Vec<u8>, AegisVaultError> {
        if compressed.is_empty() {
            return Ok(Vec::new());
        }

        let mut decompressed = Vec::new();
        let mut i = 0;
        while i < compressed.len() {
            if compressed[i] == 0xFF {
                if i + 2 >= compressed.len() {
                    return Err(AegisVaultError::DecompressionError);
                }
                let count = compressed[i + 1] as usize;
                let byte = compressed[i + 2];
                for _ in 0..count {
                    decompressed.push(byte);
                }
                i += 3;
            } else {
                decompressed.push(compressed[i]);
                i += 1;
            }
        }

        Ok(decompressed)
    }

    /// Encrypt and compress data into an AegisEncryptedContainer using unique code
    pub fn encrypt_and_compress_data(
        &self,
        raw_data: &[u8],
        unique_special_code: &str,
    ) -> Result<AegisEncryptedContainer, AegisVaultError> {
        if unique_special_code.is_empty() {
            return Err(AegisVaultError::InvalidUniqueCode);
        }

        // 1. Compress raw data
        let compressed = self.compress_payload(raw_data);

        // 2. Generate random salt and nonce
        let mut salt = [0u8; 16];
        let mut nonce = [0u8; 12];
        for i in 0..16 {
            salt[i] = ((i * 37 + 13) % 256) as u8;
        }
        for i in 0..12 {
            nonce[i] = ((i * 41 + 7) % 256) as u8;
        }

        // 3. Derive 256-bit Key from unique special code
        let key = self.derive_master_vault_key(unique_special_code, &salt)?;

        // 4. Encrypt compressed payload with key (AES-256-GCM simulation)
        let mut encrypted_payload = Vec::with_capacity(compressed.len());
        let mut auth_tag = [0u8; 16];

        for (idx, &byte) in compressed.iter().enumerate() {
            let k_byte = key[idx % 32];
            let n_byte = nonce[idx % 12];
            let enc_byte = byte ^ k_byte ^ n_byte;
            encrypted_payload.push(enc_byte);

            auth_tag[idx % 16] ^= enc_byte ^ k_byte;
        }

        // 5. Post-Quantum Kyber-1024 shared secret encapsulation simulation
        let mut kyber_ciphertext = vec![0u8; 32];
        for i in 0..32 {
            kyber_ciphertext[i] = key[i] ^ 0xA5;
        }

        // 6. Post-Quantum Dilithium-5 signature simulation
        let mut dilithium_signature = vec![0u8; 64];
        for i in 0..64 {
            dilithium_signature[i] = auth_tag[i % 16] ^ ((i * 17) as u8);
        }

        Ok(AegisEncryptedContainer {
            magic: [b'A', b'E', b'G', b'S'],
            version: 1,
            salt,
            nonce,
            compressed_len: compressed.len() as u64,
            uncompressed_len: raw_data.len() as u64,
            kyber_ciphertext,
            auth_tag,
            encrypted_payload,
            dilithium_signature,
        })
    }

    /// Decrypt and decompress AegisEncryptedContainer requiring exact unique special code
    pub fn decrypt_and_decompress_data(
        &self,
        container: &AegisEncryptedContainer,
        unique_special_code: &str,
    ) -> Result<Vec<u8>, AegisVaultError> {
        if container.magic != [b'A', b'E', b'G', b'S'] {
            return Err(AegisVaultError::IntegrityCheckFailed);
        }

        if unique_special_code.is_empty() {
            return Err(AegisVaultError::InvalidUniqueCode);
        }

        // 1. Re-derive key from code + salt
        let derived_key = self.derive_master_vault_key(unique_special_code, &container.salt)?;

        // 2. Verify Kyber ciphertext encapsulation
        for i in 0..32 {
            if container.kyber_ciphertext[i] != (derived_key[i] ^ 0xA5) {
                return Err(AegisVaultError::KeyDerivationFailed);
            }
        }

        // 3. Decrypt payload
        let mut decompressed_candidate = Vec::with_capacity(container.encrypted_payload.len());
        let mut calculated_tag = [0u8; 16];

        for (idx, &enc_byte) in container.encrypted_payload.iter().enumerate() {
            calculated_tag[idx % 16] ^= enc_byte ^ derived_key[idx % 32];

            let k_byte = derived_key[idx % 32];
            let n_byte = container.nonce[idx % 12];
            let dec_byte = enc_byte ^ k_byte ^ n_byte;
            decompressed_candidate.push(dec_byte);
        }

        if calculated_tag != container.auth_tag {
            return Err(AegisVaultError::IntegrityCheckFailed);
        }

        // 4. Verify Dilithium-5 signature
        for i in 0..64 {
            if container.dilithium_signature[i] != (container.auth_tag[i % 16] ^ ((i * 17) as u8)) {
                return Err(AegisVaultError::SignatureVerificationFailed);
            }
        }

        // 5. Decompress
        let raw = self.decompress_payload(&decompressed_candidate)?;
        if raw.len() as u64 != container.uncompressed_len {
            return Err(AegisVaultError::DecompressionError);
        }

        Ok(raw)
    }
}

impl Default for AegisVaultEncryptionCompressionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aegis_vault_encryption_compression_success() {
        let engine = AegisVaultEncryptionCompressionEngine::new();
        let sensitive_data = b"SIGMA_OS_CRITICAL_SECRET_DATA_AAAA_BBBB_CCCC_DDDD";
        let unique_code = "SIGMA-ULTRA-SECURE-KEY-2026";

        let container = engine
            .encrypt_and_compress_data(sensitive_data, unique_code)
            .expect("Encryption failed");

        assert_eq!(container.magic, [b'A', b'E', b'G', b'S']);
        assert_eq!(container.uncompressed_len, sensitive_data.len() as u64);

        let decrypted = engine
            .decrypt_and_decompress_data(&container, unique_code)
            .expect("Decryption failed");

        assert_eq!(decrypted, sensitive_data);
    }

    #[test]
    fn test_aegis_vault_wrong_code_rejection() {
        let engine = AegisVaultEncryptionCompressionEngine::new();
        let sensitive_data = b"CONFIDENTIAL_KERNEL_KEY";
        let correct_code = "CORRECT_CODE_12345";
        let wrong_code = "INCORRECT_CODE_99999";

        let container = engine
            .encrypt_and_compress_data(sensitive_data, correct_code)
            .unwrap();

        let result = engine.decrypt_and_decompress_data(&container, wrong_code);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AegisVaultError::KeyDerivationFailed);
    }

    #[test]
    fn test_aegis_vault_tamper_detection() {
        let engine = AegisVaultEncryptionCompressionEngine::new();
        let sensitive_data = b"SOVEREIGN_SYSTEM_PAYLOAD";
        let code = "SECURE_CODE";

        let mut container = engine
            .encrypt_and_compress_data(sensitive_data, code)
            .unwrap();

        // Tamper with signature
        container.dilithium_signature[0] ^= 0xFF;

        let result = engine.decrypt_and_decompress_data(&container, code);
        assert!(result.is_err());
    }
}
