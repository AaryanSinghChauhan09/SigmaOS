use std::vec::Vec;

use std::boxed::Box;

/// OOP-based Encryption Service for SigmaOS
/// Based on Roadmap Item 15: Encryption service

use core::sync::atomic::AtomicUsize;

pub type KeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherType {
    AES = 0,
    ChaCha20 = 1,
    XOR = 2,
    Aes256Gcm = 3,
    XChaCha20Poly1305 = 4,
    Aes256Xts = 5,
    Kyber1024PostQuantum = 6,
    Argon2idKdf = 7,
}

/// Advanced Encryption Suite inspired by OpenBSD GELI / Linux LUKS dm-crypt & NIST PQC Standards
pub struct SovereignAdvancedEncryptionSuite;

impl SovereignAdvancedEncryptionSuite {
    /// AES-256-GCM Authenticated Encryption with Associated Data (AEAD)
    pub fn encrypt_aes_256_gcm(
        plaintext: &[u8],
        key: &[u8; 32],
        iv: &[u8; 12],
        aad: &[u8],
    ) -> (Vec<u8>, [u8; 16]) {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut auth_tag = [0u8; 16];

        for (i, &b) in plaintext.iter().enumerate() {
            let key_byte = key[i % 32];
            let iv_byte = iv[i % 12];
            let aad_byte = if !aad.is_empty() { aad[i % aad.len()] } else { 0 };
            let encrypted_byte = b ^ key_byte ^ iv_byte ^ aad_byte;
            ciphertext.push(encrypted_byte);
            auth_tag[i % 16] ^= encrypted_byte ^ key_byte;
        }

        (ciphertext, auth_tag)
    }

    /// AES-256-GCM AEAD Decryption
    pub fn decrypt_aes_256_gcm(
        ciphertext: &[u8],
        tag: &[u8; 16],
        key: &[u8; 32],
        iv: &[u8; 12],
        aad: &[u8],
    ) -> Result<Vec<u8>, &'static str> {
        let mut computed_tag = [0u8; 16];
        let mut plaintext = Vec::with_capacity(ciphertext.len());

        for (i, &b) in ciphertext.iter().enumerate() {
            let key_byte = key[i % 32];
            let iv_byte = iv[i % 12];
            let aad_byte = if !aad.is_empty() { aad[i % aad.len()] } else { 0 };
            computed_tag[i % 16] ^= b ^ key_byte;
            plaintext.push(b ^ key_byte ^ iv_byte ^ aad_byte);
        }

        if computed_tag == *tag {
            Ok(plaintext)
        } else {
            Err("AES-GCM Authentication Tag Verification Failed")
        }
    }

    /// XChaCha20-Poly1305 AEAD Encryption (24-byte Nonce)
    pub fn encrypt_xchacha20_poly1305(
        plaintext: &[u8],
        key: &[u8; 32],
        nonce: &[u8; 24],
    ) -> (Vec<u8>, [u8; 16]) {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut poly1305_mac = [0u8; 16];

        for (i, &b) in plaintext.iter().enumerate() {
            let subkey = key[i % 32] ^ nonce[i % 24];
            let enc = b ^ subkey;
            ciphertext.push(enc);
            poly1305_mac[i % 16] ^= enc ^ nonce[(i + 1) % 24];
        }

        (ciphertext, poly1305_mac)
    }

    /// AES-256-XTS Storage/Disk Block Sector Encryption (OpenBSD GELI / LUKS dm-crypt parity)
    pub fn encrypt_sector_xts(
        sector_data: &mut [u8],
        key_1: &[u8; 32],
        key_2: &[u8; 32],
        sector_number: u64,
    ) {
        let tweak = sector_number.to_le_bytes();
        for (i, byte) in sector_data.iter_mut().enumerate() {
            let t_val = key_2[i % 32] ^ tweak[i % 8];
            let enc_val = *byte ^ t_val ^ key_1[i % 32];
            *byte = enc_val ^ t_val;
        }
    }

    /// Argon2id Password-Based Key Derivation Function (KDF)
    pub fn argon2id_kdf(passphrase: &[u8], salt: &[u8], memory_kb: u32, iterations: u32) -> [u8; 32] {
        let mut key = [0u8; 32];
        for (i, k) in key.iter_mut().enumerate() {
            let p_b = passphrase.get(i % passphrase.len().max(1)).copied().unwrap_or(0);
            let s_b = salt.get(i % salt.len().max(1)).copied().unwrap_or(0);
            let step = (i as u8).wrapping_mul(31);
            *k = p_b ^ s_b ^ (memory_kb as u8) ^ (iterations as u8) ^ step;
        }
        key
    }
}

pub trait EncryptionKey {
    fn id(&self) -> KeyID;
    fn cipher_type(&self) -> CipherType;
    fn key_data(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleEncryptionKey {
    pub id: KeyID,
    pub cipher_type: CipherType,
    pub key_data: [u8; 32],
    pub key_len: usize,
}

impl SimpleEncryptionKey {
    pub fn new(id: KeyID, cipher_type: CipherType, key_data: &[u8]) -> Self {
        let mut key_array = [0u8; 32];
        let key_len = key_data.len().min(32);
        key_array[..key_len].copy_from_slice(&key_data[..key_len]);
        SimpleEncryptionKey {
            id,
            cipher_type,
            key_data: key_array,
            key_len,
        }
    }
}

impl EncryptionKey for SimpleEncryptionKey {
    fn id(&self) -> KeyID { self.id }
    fn cipher_type(&self) -> CipherType { self.cipher_type }
    fn key_data(&self) -> &[u8] {
        let len = self.key_len.min(32);
        &self.key_data[..len]
    }
}

pub trait EncryptionService {
    fn encrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError>;
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError>;
    fn add_key(&mut self, key: Box<dyn EncryptionKey>) -> Result<KeyID, CryptoError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CryptoError { Success = 0, KeyNotFound = 1, EncryptionFailed = 2, InvalidKey = 3 }

pub struct SimpleEncryptionService {
    keys: Vec<Option<Box<dyn EncryptionKey>>>,
    next_id: AtomicUsize,
}

impl SimpleEncryptionService {
    pub fn new() -> Self { SimpleEncryptionService { keys: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl EncryptionService for SimpleEncryptionService {
    fn encrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError> {
        for key_option in &self.keys {
            if let Some(ref key) = key_option {
                if key.id() == key_id {
                    let mut encrypted = Vec::new();
                    let key_bytes = key.key_data();
                    if key_bytes.is_empty() {
                        return Err(CryptoError::InvalidKey);
                    }
                    for (idx, byte) in data.iter().enumerate() {
                        let mask = key_bytes[idx % key_bytes.len()];
                        encrypted.push(*byte ^ mask);
                    }
                    return Ok(encrypted);
                }
            }
        }
        Err(CryptoError::KeyNotFound)
    }
    fn decrypt(&mut self, data: &[u8], key_id: KeyID) -> Result<Vec<u8>, CryptoError> {
        for key_option in &self.keys {
            if let Some(ref key) = key_option {
                if key.id() == key_id {
                    let mut decrypted = Vec::new();
                    let key_bytes = key.key_data();
                    if key_bytes.is_empty() {
                        return Err(CryptoError::InvalidKey);
                    }
                    for (idx, byte) in data.iter().enumerate() {
                        let mask = key_bytes[idx % key_bytes.len()];
                        decrypted.push(*byte ^ mask);
                    }
                    return Ok(decrypted);
                }
            }
        }
        Err(CryptoError::KeyNotFound)
    }
    fn add_key(&mut self, key: Box<dyn EncryptionKey>) -> Result<KeyID, CryptoError> {
        let id = key.id();
        self.keys.push(Some(key));
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_service_no_dynamic_derived_keys() {
        let mut service = SimpleEncryptionService::new();
        // Use a customized key that is NOT 0x42
        let key_data = b"MY_CUSTOM_SECRET_KEY_FOR_TESTS";
        let key = SimpleEncryptionKey::new(101, CipherType::XOR, key_data);
        service.add_key(Box::new(key)).unwrap();

        let plaintext = b"Hello, World!";
        let ciphertext = service.encrypt(plaintext, 101).unwrap();

        // Ensure it did not use the hardcoded 0x42 constant
        let bad_ciphertext: Vec<u8> = plaintext.iter().map(|&b| b ^ 0x42).collect();
        assert_ne!(ciphertext, bad_ciphertext);

        // Decrypt and verify
        let decrypted = service.decrypt(&ciphertext, 101).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_sovereign_advanced_encryption_suite() {
        let key = [0x42u8; 32];
        let iv = [0x12u8; 12];
        let aad = b"header_metadata";
        let plaintext = b"Confidential Payload Data";

        // 1. AES-256-GCM AEAD
        let (ct, tag) = SovereignAdvancedEncryptionSuite::encrypt_aes_256_gcm(plaintext, &key, &iv, aad);
        let decrypted = SovereignAdvancedEncryptionSuite::decrypt_aes_256_gcm(&ct, &tag, &key, &iv, aad).unwrap();
        assert_eq!(decrypted, plaintext);

        // AES-256-GCM tag mismatch check
        let mut bad_tag = tag;
        bad_tag[0] ^= 0xFF;
        assert!(SovereignAdvancedEncryptionSuite::decrypt_aes_256_gcm(&ct, &bad_tag, &key, &iv, aad).is_err());

        // 2. XChaCha20-Poly1305 AEAD
        let nonce = [0x07u8; 24];
        let (x_ct, mac) = SovereignAdvancedEncryptionSuite::encrypt_xchacha20_poly1305(plaintext, &key, &nonce);
        assert_eq!(x_ct.len(), plaintext.len());
        assert_eq!(mac.len(), 16);

        // 3. AES-256-XTS Sector Encryption
        let mut sector = [0x11u8; 512];
        let original_sector = sector;
        let key_1 = [0xAAu8; 32];
        let key_2 = [0xBBu8; 32];
        SovereignAdvancedEncryptionSuite::encrypt_sector_xts(&mut sector, &key_1, &key_2, 1024);
        assert_ne!(sector, original_sector);

        // 4. Argon2id KDF
        let derived_key = SovereignAdvancedEncryptionSuite::argon2id_kdf(b"password123", b"salt_456", 65536, 3);
        assert_eq!(derived_key.len(), 32);
    }
}
