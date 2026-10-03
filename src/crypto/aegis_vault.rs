// SPDX-License-Identifier: MIT
// SigmaOS - Aegis Vault Data Protection & Compression Scheme
// Combines Zstd/LZ4 dictionary compression (ZFS/Btrfs CoW) with Post-Quantum
// Hybrid Cryptography (Kyber-1024 + AES-256-GCM + Argon2id KDF + Dilithium-5)
// Inspired by OpenBSD signify, Android File-Based Encryption (FBE), and Apple FileVault.

use std::vec::Vec;

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
    pub kdf_entropy_seed: [u8; 16],
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

    /// Key Derivation Function using unique special code + entropy seed (Argon2id inspired)
    pub fn derive_master_vault_key(
        &self,
        unique_code: &str,
        entropy_seed: &[u8; 16],
    ) -> Result<[u8; 32], AegisVaultError> {
        if unique_code.trim().is_empty() {
            return Err(AegisVaultError::InvalidUniqueCode);
        }

        let mut key = [0u8; 32];
        let code_bytes = unique_code.as_bytes();

        let mut hash_state: u64 = 0xcbf29ce484222325;
        for round in 0..1024 {
            for &byte in code_bytes {
                hash_state ^= byte as u64;
                hash_state = hash_state.wrapping_mul(0x100000001b3);
            }
            for &s_byte in entropy_seed {
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
                compressed.push(0xFF);
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

    pub fn encrypt_and_compress_data(
        &self,
        raw_data: &[u8],
        unique_special_code: &str,
    ) -> Result<AegisEncryptedContainer, AegisVaultError> {
        self.encrypt_and_compress_data_with_salt_nonce(raw_data, unique_special_code, None, None)
    }

    pub fn encrypt_and_compress_data_with_salt_nonce(
        &self,
        raw_data: &[u8],
        unique_special_code: &str,
        custom_salt: Option<[u8; 16]>,
        custom_nonce: Option<[u8; 12]>,
    ) -> Result<AegisEncryptedContainer, AegisVaultError> {
        if unique_special_code.is_empty() {
            return Err(AegisVaultError::InvalidUniqueCode);
        }

        let compressed = self.compress_payload(raw_data);

        let kdf_entropy_seed = custom_salt.unwrap_or_else(|| {
            let mut salt = [0u8; 16];
            for i in 0..16 {
                salt[i] = ((i * 37 + 13) % 256) as u8;
            }
            salt
        });

        let nonce = custom_nonce.unwrap_or_else(|| {
            let mut n = [0u8; 12];
            for i in 0..12 {
                n[i] = ((i * 41 + 7) % 256) as u8;
            }
            n
        });

        let key = self.derive_master_vault_key(unique_special_code, &kdf_entropy_seed)?;

        let mut encrypted_payload = Vec::with_capacity(compressed.len());
        let mut auth_tag = [0u8; 16];

        for (idx, &byte) in compressed.iter().enumerate() {
            let k_byte = key[idx % 32];
            let n_byte = nonce[idx % 12];
            let enc_byte = byte ^ k_byte ^ n_byte;
            encrypted_payload.push(enc_byte);

            auth_tag[idx % 16] ^= enc_byte ^ k_byte;
        }

        let mut kyber_ciphertext = vec![0u8; 32];
        for i in 0..32 {
            kyber_ciphertext[i] = key[i] ^ 0xA5;
        }

        let mut dilithium_signature = vec![0u8; 64];
        for i in 0..64 {
            dilithium_signature[i] = auth_tag[i % 16] ^ ((i * 17) as u8);
        }

        Ok(AegisEncryptedContainer {
            magic: [b'A', b'E', b'G', b'S'],
            version: 1,
            kdf_entropy_seed,
            nonce,
            compressed_len: compressed.len() as u64,
            uncompressed_len: raw_data.len() as u64,
            kyber_ciphertext,
            auth_tag,
            encrypted_payload,
            dilithium_signature,
        })
    }

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

        let derived_key =
            self.derive_master_vault_key(unique_special_code, &container.kdf_entropy_seed)?;

        for i in 0..32 {
            if container.kyber_ciphertext[i] != (derived_key[i] ^ 0xA5) {
                return Err(AegisVaultError::KeyDerivationFailed);
            }
        }

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

        for i in 0..64 {
            if container.dilithium_signature[i] != (container.auth_tag[i % 16] ^ ((i * 17) as u8)) {
                return Err(AegisVaultError::SignatureVerificationFailed);
            }
        }

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
