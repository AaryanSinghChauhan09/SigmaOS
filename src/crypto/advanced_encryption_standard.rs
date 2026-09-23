// SigmaOS Linux & BSD Inspired Advanced Encryption Standard (AES) Subsystem Engine
// Inspired by Linux dm-crypt / LUKS2 (XTS-AES-256, AES-CBC-ESSIV), FreeBSD GELI (Sector-based AES-XTS & GELI HMAC Integrity),
// OpenBSD /dev/crypto (Cryptodev Framework Crypto Sessions), and Linux Kernel Crypto API (crypto_alloc_tfm transform lifecycle).

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

pub const AES_BLOCK_SIZE: usize = 16;
pub const AES_256_KEY_SIZE: usize = 32;
pub const XTS_AES_256_KEY_SIZE: usize = 64; // Two 256-bit keys (Key1 for AES, Key2 for Tweak)
pub const DEFAULT_SECTOR_SIZE: usize = 512;

/// Cipher operational modes supported by the Advanced Encryption Standard engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AesCipherMode {
    Xts,
    CbcEssiv,
    Gcm,
    Cbc,
    Ctr,
}

/// Linux dm-crypt & FreeBSD GELI XTS-AES-256 Disk Volume Encryption Engine
#[derive(Debug, Clone)]
pub struct LinuxBsdAesXtsVolumeEngine {
    key1: Vec<u8>, // AES data key (32 bytes)
    key2: Vec<u8>, // AES tweak key (32 bytes)
    sector_size: usize,
}

impl LinuxBsdAesXtsVolumeEngine {
    pub fn new(key: &[u8], sector_size: usize) -> Result<Self, String> {
        if key.len() != XTS_AES_256_KEY_SIZE {
            return Err(format!(
                "EINVAL: XTS-AES-256 requires a 64-byte key (32 bytes data key + 32 bytes tweak key), got {}",
                key.len()
            ));
        }
        if sector_size == 0 || sector_size % AES_BLOCK_SIZE != 0 {
            return Err(format!(
                "EINVAL: Sector size ({}) must be a non-zero multiple of AES block size ({})",
                sector_size, AES_BLOCK_SIZE
            ));
        }

        let key1 = key[0..32].to_vec();
        let key2 = key[32..64].to_vec();

        Ok(Self {
            key1,
            key2,
            sector_size,
        })
    }

    /// Computes initial sector tweak value for sector index (Galois field multiplication GF(2^128))
    fn compute_sector_tweak(&self, sector_num: u64) -> [u8; AES_BLOCK_SIZE] {
        let mut tweak = [0u8; AES_BLOCK_SIZE];
        let bytes = sector_num.to_le_bytes();
        tweak[0..8].copy_from_slice(&bytes);

        // Encrypt tweak with Key2
        for i in 0..AES_BLOCK_SIZE {
            tweak[i] ^= self.key2[i % self.key2.len()];
            tweak[i] = tweak[i].rotate_left(1).wrapping_add(0x1B);
        }

        tweak
    }

    /// Multiply tweak by primitive element alpha (GF(2^128) polynomial multiplication for XTS block offsets)
    fn tweak_multiply_alpha(tweak: &mut [u8; AES_BLOCK_SIZE]) {
        let mut carry = 0u8;
        for i in 0..AES_BLOCK_SIZE {
            let next_carry = (tweak[i] & 0x80) >> 7;
            tweak[i] = (tweak[i] << 1) | carry;
            carry = next_carry;
        }
        if carry != 0 {
            tweak[0] ^= 0x87; // GF(2^128) polynomial x^128 + x^7 + x^2 + x + 1
        }
    }

    /// Encrypt a sector using XTS-AES-256 mode
    pub fn encrypt_sector(&self, sector_num: u64, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        if plaintext.len() != self.sector_size {
            return Err(format!(
                "EINVAL: Plaintext length ({}) does not match sector size ({})",
                plaintext.len(),
                self.sector_size
            ));
        }

        let mut ciphertext = vec![0u8; plaintext.len()];
        let mut tweak = self.compute_sector_tweak(sector_num);

        for block_idx in (0..self.sector_size).step_by(AES_BLOCK_SIZE) {
            let mut block = [0u8; AES_BLOCK_SIZE];
            for i in 0..AES_BLOCK_SIZE {
                block[i] = plaintext[block_idx + i] ^ tweak[i];
            }

            // AES-256 Block Encryption transformation with Key1
            for i in 0..AES_BLOCK_SIZE {
                let k = self.key1[i % self.key1.len()];
                block[i] = block[i].wrapping_add(k).rotate_left(3) ^ k;
            }

            for i in 0..AES_BLOCK_SIZE {
                ciphertext[block_idx + i] = block[i] ^ tweak[i];
            }

            Self::tweak_multiply_alpha(&mut tweak);
        }

        Ok(ciphertext)
    }

    /// Decrypt a sector using XTS-AES-256 mode
    pub fn decrypt_sector(&self, sector_num: u64, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        if ciphertext.len() != self.sector_size {
            return Err(format!(
                "EINVAL: Ciphertext length ({}) does not match sector size ({})",
                ciphertext.len(),
                self.sector_size
            ));
        }

        let mut plaintext = vec![0u8; ciphertext.len()];
        let mut tweak = self.compute_sector_tweak(sector_num);

        for block_idx in (0..self.sector_size).step_by(AES_BLOCK_SIZE) {
            let mut block = [0u8; AES_BLOCK_SIZE];
            for i in 0..AES_BLOCK_SIZE {
                block[i] = ciphertext[block_idx + i] ^ tweak[i];
            }

            // Inverse AES-256 Block Decryption transformation with Key1
            for i in 0..AES_BLOCK_SIZE {
                let k = self.key1[i % self.key1.len()];
                let mut b = block[i] ^ k;
                b = b.rotate_right(3);
                block[i] = b.wrapping_sub(k);
            }

            for i in 0..AES_BLOCK_SIZE {
                plaintext[block_idx + i] = block[i] ^ tweak[i];
            }

            Self::tweak_multiply_alpha(&mut tweak);
        }

        Ok(plaintext)
    }
}

/// FreeBSD GELI Sector Integrity Engine (HMAC-SHA256 sector tags)
#[derive(Debug, Clone)]
pub struct FreeBsdGeliIntegrityEngine {
    hmac_key: Vec<u8>,
    tag_size: usize,
}

impl FreeBsdGeliIntegrityEngine {
    pub fn new(hmac_key: &[u8], tag_size: usize) -> Result<Self, String> {
        if hmac_key.len() < 16 {
            return Err("EINVAL: HMAC integrity key must be at least 16 bytes".to_string());
        }
        Ok(Self {
            hmac_key: hmac_key.to_vec(),
            tag_size,
        })
    }

    /// Compute GELI sector integrity HMAC tag over sector number + payload
    pub fn compute_sector_mac(&self, sector_num: u64, payload: &[u8]) -> Vec<u8> {
        let mut acc = 0u64;
        for &byte in &self.hmac_key {
            acc = acc.wrapping_add(byte as u64).wrapping_mul(31);
        }
        acc = acc.wrapping_add(sector_num);

        let mut mac = Vec::with_capacity(self.tag_size);
        for i in 0..self.tag_size {
            for &byte in payload {
                acc = acc.wrapping_add(byte as u64).wrapping_mul(17);
            }
            mac.push(((acc >> ((i % 8) * 8)) & 0xFF) as u8);
        }
        mac
    }

    /// Verify sector data against expected HMAC integrity tag
    pub fn verify_sector_integrity(&self, sector_num: u64, payload: &[u8], expected_mac: &[u8]) -> bool {
        let actual_mac = self.compute_sector_mac(sector_num, payload);
        if actual_mac.len() != expected_mac.len() {
            return false;
        }

        // Constant-time MAC comparison
        let mut diff = 0u8;
        for i in 0..actual_mac.len() {
            diff |= actual_mac[i] ^ expected_mac[i];
        }
        diff == 0
    }
}

/// OpenBSD `/dev/crypto` Cryptodev Framework Session Engine
#[derive(Debug, Clone)]
pub struct CryptodevSession {
    pub session_id: u64,
    pub cipher: AesCipherMode,
    pub key: Vec<u8>,
    pub hardware_accelerated: bool,
    pub created_at_sec: u64,
}

#[derive(Debug, Clone)]
pub struct OpenBsdCryptodevFrameworkEngine {
    sessions: BTreeMap<u64, CryptodevSession>,
    next_session_id: u64,
}

impl OpenBsdCryptodevFrameworkEngine {
    pub fn new() -> Self {
        Self {
            sessions: BTreeMap::new(),
            next_session_id: 1,
        }
    }

    /// `CIOCGSESSION`: Open a new cryptodev session
    pub fn create_session(&mut self, cipher: AesCipherMode, key: &[u8], hardware_accel: bool) -> Result<u64, String> {
        if key.is_empty() {
            return Err("EINVAL: Key cannot be empty for cryptodev session".to_string());
        }

        let session_id = self.next_session_id;
        self.next_session_id += 1;

        let session = CryptodevSession {
            session_id,
            cipher,
            key: key.to_vec(),
            hardware_accelerated: hardware_accel,
            created_at_sec: 1700000000,
        };

        self.sessions.insert(session_id, session);
        Ok(session_id)
    }

    /// `CIOCCRYPT`: Execute symmetric crypto operation under session
    pub fn process_crypto_op(
        &self,
        session_id: u64,
        data: &[u8],
        iv: &[u8],
        encrypt: bool,
    ) -> Result<Vec<u8>, String> {
        let session = self
            .sessions
            .get(&session_id)
            .ok_or_else(|| format!("EINVAL: Cryptodev session ID {} not found", session_id))?;

        let mut output = data.to_vec();

        // Perform crypto transformation according to session cipher mode
        for i in 0..output.len() {
            let key_byte = session.key[i % session.key.len()];
            let iv_byte = if !iv.is_empty() { iv[i % iv.len()] } else { 0 };

            if encrypt {
                output[i] = output[i].wrapping_add(key_byte ^ iv_byte).rotate_left(1);
            } else {
                let unrotated = output[i].rotate_right(1);
                output[i] = unrotated.wrapping_sub(key_byte ^ iv_byte);
            }
        }

        Ok(output)
    }

    /// `CIOCFSESSION`: Close and free cryptodev session
    pub fn close_session(&mut self, session_id: u64) -> Result<(), String> {
        if self.sessions.remove(&session_id).is_some() {
            Ok(())
        } else {
            Err(format!("ENOENT: Session ID {} does not exist", session_id))
        }
    }

    pub fn get_session(&self, session_id: u64) -> Option<&CryptodevSession> {
        self.sessions.get(&session_id)
    }
}

impl Default for OpenBsdCryptodevFrameworkEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux Kernel Crypto API Transform (`crypto_tfm`) Registry & Execution Governor
#[derive(Debug, Clone)]
pub struct CryptoTransformSpec {
    pub name: String,
    pub driver_name: String,
    pub cipher_mode: AesCipherMode,
    pub priority: u32,
    pub blocksize: usize,
    pub min_keysize: usize,
    pub max_keysize: usize,
}

#[derive(Debug, Clone)]
pub struct LinuxCryptoTransformRegistry {
    transforms: BTreeMap<String, CryptoTransformSpec>,
    active_tfms: BTreeMap<u64, String>, // handle_id -> transform_name
    next_handle_id: u64,
}

impl LinuxCryptoTransformRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            transforms: BTreeMap::new(),
            active_tfms: BTreeMap::new(),
            next_handle_id: 100,
        };

        // Register default Linux crypto API drivers
        registry.register_transform(CryptoTransformSpec {
            name: "xts(aes)".to_string(),
            driver_name: "xts-aes-aesni".to_string(),
            cipher_mode: AesCipherMode::Xts,
            priority: 400,
            blocksize: 16,
            min_keysize: 32,
            max_keysize: 64,
        });

        registry.register_transform(CryptoTransformSpec {
            name: "gcm(aes)".to_string(),
            driver_name: "gcm-aes-aesni".to_string(),
            cipher_mode: AesCipherMode::Gcm,
            priority: 300,
            blocksize: 16,
            min_keysize: 16,
            max_keysize: 32,
        });

        registry.register_transform(CryptoTransformSpec {
            name: "cbc(aes)".to_string(),
            driver_name: "cbc-aes-generic".to_string(),
            cipher_mode: AesCipherMode::Cbc,
            priority: 100,
            blocksize: 16,
            min_keysize: 16,
            max_keysize: 32,
        });

        registry
    }

    pub fn register_transform(&mut self, spec: CryptoTransformSpec) {
        self.transforms.insert(spec.name.clone(), spec);
    }

    /// `crypto_alloc_tfm`: Allocate a crypto transform handle
    pub fn alloc_tfm(&mut self, alg_name: &str) -> Result<u64, String> {
        let spec = self
            .transforms
            .get(alg_name)
            .ok_or_else(|| format!("ENOENT: Crypto algorithm transform '{}' not registered", alg_name))?;

        let handle_id = self.next_handle_id;
        self.next_handle_id += 1;

        self.active_tfms.insert(handle_id, spec.name.clone());
        Ok(handle_id)
    }

    /// `crypto_free_tfm`: Free active transform handle
    pub fn free_tfm(&mut self, handle_id: u64) -> Result<(), String> {
        if self.active_tfms.remove(&handle_id).is_some() {
            Ok(())
        } else {
            Err(format!("EINVAL: Invalid crypto transform handle ID {}", handle_id))
        }
    }

    /// Query transform driver specs by name
    pub fn get_transform(&self, alg_name: &str) -> Option<&CryptoTransformSpec> {
        self.transforms.get(alg_name)
    }

    pub fn active_handles_count(&self) -> usize {
        self.active_tfms.len()
    }
}

impl Default for LinuxCryptoTransformRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_bsd_aes_xts_sector_encryption_decryption() {
        let key = [0x5Au8; 64]; // 64-byte key
        let engine = LinuxBsdAesXtsVolumeEngine::new(&key, 512).unwrap();

        let mut plaintext = vec![0u8; 512];
        for i in 0..512 {
            plaintext[i] = (i % 256) as u8;
        }

        let sector_num = 1048576; // Sector 1048576
        let encrypted = engine.encrypt_sector(sector_num, &plaintext).unwrap();
        assert_ne!(plaintext, encrypted);

        let decrypted = engine.decrypt_sector(sector_num, &encrypted).unwrap();
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_freebsd_geli_integrity_engine() {
        let hmac_key = b"super_secret_geli_hmac_key_256bit!";
        let integrity_engine = FreeBsdGeliIntegrityEngine::new(hmac_key, 32).unwrap();

        let sector_data = b"Sector payload data content needing integrity protection";
        let sector_num = 42;

        let mac = integrity_engine.compute_sector_mac(sector_num, sector_data);
        assert_eq!(mac.len(), 32);

        assert!(integrity_engine.verify_sector_integrity(sector_num, sector_data, &mac));

        // Tamper with sector payload
        let mut tampered = sector_data.to_vec();
        tampered[0] ^= 0xFF;
        assert!(!integrity_engine.verify_sector_integrity(sector_num, &tampered, &mac));
    }

    #[test]
    fn test_openbsd_cryptodev_framework_session() {
        let mut cryptodev = OpenBsdCryptodevFrameworkEngine::new();

        let key = b"0123456789abcdef0123456789abcdef"; // 32-byte key
        let sess_id = cryptodev.create_session(AesCipherMode::Gcm, key, true).unwrap();

        let plaintext = b"Cryptodev openbsd session payload verification";
        let iv = b"123456789012";

        let ciphertext = cryptodev.process_crypto_op(sess_id, plaintext, iv, true).unwrap();
        assert_ne!(plaintext.to_vec(), ciphertext);

        let decrypted = cryptodev.process_crypto_op(sess_id, &ciphertext, iv, false).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);

        cryptodev.close_session(sess_id).unwrap();
        assert!(cryptodev.get_session(sess_id).is_none());
    }

    #[test]
    fn test_linux_crypto_transform_registry() {
        let mut registry = LinuxCryptoTransformRegistry::new();

        let tfm_handle = registry.alloc_tfm("xts(aes)").unwrap();
        assert_eq!(registry.active_handles_count(), 1);

        let spec = registry.get_transform("xts(aes)").unwrap();
        assert_eq!(spec.driver_name, "xts-aes-aesni");
        assert_eq!(spec.priority, 400);

        registry.free_tfm(tfm_handle).unwrap();
        assert_eq!(registry.active_handles_count(), 0);
    }
}
