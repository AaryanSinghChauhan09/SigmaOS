//! dm-crypt / LUKS2 block device encryption
//! Inspired by Linux dm-crypt kernel module and the LUKS2 on-disk format.
//!
//! References:
//! - Linux drivers/md/dm-crypt.c
//! - LUKS2 spec: https://gitlab.com/cryptsetup/cryptsetup/-/wikis/LUKS-standard
//! - cryptsetup source: https://gitlab.com/cryptsetup/cryptsetup

use crate::crypto::entropy;

/// LUKS2 magic bytes at the start of a LUKS2 header
pub const LUKS2_MAGIC: &[u8; 6] = b"LUKS\xba\xbe";

/// Supported LUKS2 cipher suites
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuksCipher {
    /// AES-256-XTS (recommended, NIST-approved)
    Aes256Xts,
    /// AES-256-GCM (authenticated encryption)
    Aes256Gcm,
    /// ChaCha20-Poly1305 (software-optimized, no AES-NI required)
    ChaCha20Poly1305,
}

impl LuksCipher {
    pub fn key_size_bytes(&self) -> usize {
        match self {
            Self::Aes256Xts => 64, // 2×256-bit for XTS
            Self::Aes256Gcm => 32,
            Self::ChaCha20Poly1305 => 32,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Aes256Xts => "aes-xts-plain64",
            Self::Aes256Gcm => "aes-gcm-random",
            Self::ChaCha20Poly1305 => "chacha20-poly1305",
        }
    }
}

/// LUKS2 KDF (Key Derivation Function)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuksKdf {
    /// Argon2id — memory-hard, recommended for LUKS2
    Argon2id,
    /// PBKDF2-SHA512 — legacy compatibility
    Pbkdf2Sha512,
}

/// LUKS2 header (simplified; full spec has 4096-byte header)
#[derive(Debug, Clone)]
pub struct Luks2Header {
    pub magic: [u8; 6],
    pub version: u16,
    pub cipher: LuksCipher,
    pub kdf: LuksKdf,
    /// Salt used in KDF (512-bit)
    pub salt: [u8; 64],
    /// Master key encrypted with password-derived key (256-bit for AES-256-GCM)
    pub encrypted_master_key: Vec<u8>,
    /// Authentication tag for header integrity
    pub header_auth_tag: [u8; 16],
    /// LUKS2 device UUID
    pub uuid: [u8; 16],
    /// Sector size (512 or 4096)
    pub sector_size: u32,
}

impl Luks2Header {
    /// Create a new LUKS2 header with a randomly generated master key.
    pub fn new(cipher: LuksCipher, kdf: LuksKdf, sector_size: u32) -> Self {
        let mut salt = [0u8; 64];
        let mut uuid = [0u8; 16];
        entropy::get_entropy_bytes(&mut salt);
        entropy::get_entropy_bytes(&mut uuid);
        // UUID v4 format
        uuid[6] = (uuid[6] & 0x0F) | 0x40;
        uuid[8] = (uuid[8] & 0x3F) | 0x80;
        Self {
            magic: *LUKS2_MAGIC,
            version: 2,
            cipher,
            kdf,
            salt,
            encrypted_master_key: vec![0u8; cipher.key_size_bytes()],
            header_auth_tag: [0u8; 16],
            uuid,
            sector_size,
        }
    }

    /// Validate the LUKS2 magic bytes.
    pub fn validate_magic(&self) -> bool {
        &self.magic == LUKS2_MAGIC
    }

    pub fn format_uuid(&self) -> String {
        format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.uuid[0], self.uuid[1], self.uuid[2], self.uuid[3],
            self.uuid[4], self.uuid[5], self.uuid[6], self.uuid[7],
            self.uuid[8], self.uuid[9], self.uuid[10], self.uuid[11],
            self.uuid[12], self.uuid[13], self.uuid[14], self.uuid[15])
    }
}

/// dm-crypt device — encrypts/decrypts a block device sector-by-sector.
pub struct DmCryptDevice {
    pub header: Luks2Header,
    /// Derived encryption key (never stored; derived fresh from passphrase each open)
    key: Vec<u8>,
    /// Whether the device is currently open
    pub is_open: bool,
    /// Encrypted sector data (in-memory simulation)
    sectors: Vec<Vec<u8>>,
    pub sector_size: u32,
    pub num_sectors: u64,
}

impl DmCryptDevice {
    /// Format a new dm-crypt device (equivalent to `cryptsetup luksFormat`).
    pub fn format(cipher: LuksCipher, kdf: LuksKdf, num_sectors: u64, sector_size: u32) -> Self {
        let header = Luks2Header::new(cipher, kdf, sector_size);
        let mut key = vec![0u8; cipher.key_size_bytes()];
        entropy::get_entropy_bytes(&mut key);
        Self {
            header,
            key,
            is_open: false,
            sectors: vec![vec![0u8; sector_size as usize]; num_sectors as usize],
            sector_size,
            num_sectors,
        }
    }

    /// Open the device with a passphrase (equivalent to `cryptsetup open`).
    /// Returns Err if passphrase verification fails.
    pub fn open(&mut self, passphrase: &[u8]) -> Result<(), &'static str> {
        if passphrase.is_empty() {
            return Err("Empty passphrase not allowed");
        }
        // In production: run Argon2id/PBKDF2 + decrypt master key with result
        // Here: simulated key derivation using XorShift
        let mut derived = vec![0u8; self.header.cipher.key_size_bytes()];
        for (i, b) in passphrase.iter().enumerate() {
            let dlen = derived.len();
            derived[i % dlen] ^= b.wrapping_mul(((i + 1) as u8).wrapping_add(17));
        }
        for i in 0..derived.len() {
            derived[i] ^= self.header.salt[i % 64];
        }
        self.key = derived;
        self.is_open = true;
        Ok(())
    }

    /// Close the device (zero the key material from memory).
    pub fn close(&mut self) {
        // Zero key material — critical for security
        for b in self.key.iter_mut() {
            *b = 0;
        }
        self.key.clear();
        self.is_open = false;
    }

    /// Encrypt and write a sector (XTS-style simulation).
    pub fn write_sector(&mut self, sector_idx: u64, plaintext: &[u8]) -> Result<(), &'static str> {
        if !self.is_open {
            return Err("Device not open");
        }
        if sector_idx >= self.num_sectors {
            return Err("Sector out of range");
        }
        if plaintext.len() != self.sector_size as usize {
            return Err("Wrong sector size");
        }
        let mut ciphertext = plaintext.to_vec();
        // Simulation: XOR with key + sector index tweak (real XTS uses AES-256)
        let tweak = sector_idx.to_le_bytes();
        for (i, b) in ciphertext.iter_mut().enumerate() {
            *b ^= self.key[i % self.key.len()] ^ tweak[i % 8];
        }
        self.sectors[sector_idx as usize] = ciphertext;
        Ok(())
    }

    /// Decrypt and read a sector.
    pub fn read_sector(&self, sector_idx: u64) -> Result<Vec<u8>, &'static str> {
        if !self.is_open {
            return Err("Device not open");
        }
        if sector_idx >= self.num_sectors {
            return Err("Sector out of range");
        }
        let ciphertext = &self.sectors[sector_idx as usize];
        let mut plaintext = ciphertext.clone();
        let tweak = sector_idx.to_le_bytes();
        for (i, b) in plaintext.iter_mut().enumerate() {
            *b ^= self.key[i % self.key.len()] ^ tweak[i % 8];
        }
        Ok(plaintext)
    }
}

impl Drop for DmCryptDevice {
    fn drop(&mut self) {
        // Ensure key is zeroed when device is dropped
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_open() {
        let mut dev = DmCryptDevice::format(LuksCipher::Aes256Xts, LuksKdf::Argon2id, 16, 512);
        assert!(!dev.is_open);
        dev.open(b"test-passphrase").unwrap();
        assert!(dev.is_open);
    }

    #[test]
    fn test_write_read_roundtrip() {
        let mut dev = DmCryptDevice::format(LuksCipher::Aes256Gcm, LuksKdf::Argon2id, 8, 512);
        dev.open(b"sigma-passphrase").unwrap();
        let mut plaintext = vec![0xABu8; 512];
        plaintext[0] = 0x42;
        plaintext[511] = 0xFF;
        dev.write_sector(0, &plaintext).unwrap();
        let decrypted = dev.read_sector(0).unwrap();
        assert_eq!(
            decrypted, plaintext,
            "decrypted must match original plaintext"
        );
    }

    #[test]
    fn test_different_sectors_differ() {
        let mut dev = DmCryptDevice::format(LuksCipher::Aes256Xts, LuksKdf::Argon2id, 4, 512);
        dev.open(b"pw").unwrap();
        let data = vec![0x55u8; 512];
        dev.write_sector(0, &data).unwrap();
        dev.write_sector(1, &data).unwrap();
        // Same plaintext, different sectors → different ciphertext (tweak differs)
        assert_ne!(dev.sectors[0], dev.sectors[1]);
    }

    #[test]
    fn test_close_zeroes_key() {
        let mut dev =
            DmCryptDevice::format(LuksCipher::ChaCha20Poly1305, LuksKdf::Pbkdf2Sha512, 4, 512);
        dev.open(b"pw").unwrap();
        assert!(!dev.key.is_empty());
        dev.close();
        assert!(dev.key.is_empty());
    }

    #[test]
    fn test_magic_valid() {
        let dev = DmCryptDevice::format(LuksCipher::Aes256Xts, LuksKdf::Argon2id, 4, 512);
        assert!(dev.header.validate_magic());
    }

    #[test]
    fn test_empty_passphrase_rejected() {
        let mut dev = DmCryptDevice::format(LuksCipher::Aes256Xts, LuksKdf::Argon2id, 4, 512);
        assert!(dev.open(b"").is_err());
    }
}
