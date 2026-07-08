// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/crypto/sigma_luks.rs — LUKS Disk Encryption
//
// Implements LUKS-style disk encryption for SigmaOS.
// Inspired by: LUKS2, dm-crypt, Ubuntu/Fedora full-disk encryption
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum passphrase length.
const MAX_PASSPHRASE_LEN: SigmaUsize = 128;
/// Salt length for key derivation.
const SALT_LEN: SigmaUsize = 32;
/// Key derivation iteration count.
const PBKDF2_ITERATIONS: SigmaU32 = 100000;
/// Master key length (AES-256).
const MASTER_KEY_LEN: SigmaUsize = 32;
/// Sector size for encryption.
const SECTOR_SIZE: SigmaUsize = 512;

// ── Encryption Algorithm ───────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// AES-256-XTS (default).
    Aes256Xts = 0,
    /// AES-256-CBC.
    Aes256Cbc = 1,
    /// ChaCha20-Poly1305.
    ChaCha20Poly1305 = 2,
}

// ── Key Derivation Function ───────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KdfType {
    /// PBKDF2-SHA256 (default).
    Pbkdf2Sha256 = 0,
    /// Argon2id (future).
    Argon2id = 1,
}

// ── LUKS Header ─────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LuksHeader {
    pub magic: [SigmaU8; 6],
    pub version: SigmaU16,
    pub cipher_name: [SigmaU8; 32],
    pub cipher_mode: [SigmaU8; 32],
    pub hash_spec: [SigmaU8; 32],
    pub payload_offset: SigmaU32,
    pub key_bytes: SigmaU32,
    pub mk_digest: [SigmaU8; 64],
    pub mk_digest_salt: [SigmaU8; SALT_LEN],
    pub mk_digest_iterations: SigmaU32,
    pub uuid: [SigmaU8; 40],
}

impl LuksHeader {
    pub const MAGIC: [SigmaU8; 6] = *b"LUKS\xBA\xBE";
    
    pub const fn new() -> Self {
        Self {
            magic: Self::MAGIC,
            version: 2,
            cipher_name: [0u8; 32],
            cipher_mode: [0u8; 32],
            hash_spec: [0u8; 32],
            payload_offset: 4096,
            key_bytes: MASTER_KEY_LEN as SigmaU32,
            mk_digest: [0u8; 64],
            mk_digest_salt: [0u8; SALT_LEN],
            mk_digest_iterations: PBKDF2_ITERATIONS,
            uuid: [0u8; 40],
        }
    }
}

// ── Key Slot ───────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeySlot {
    pub active: SigmaBool,
    pub iterations: SigmaU32,
    pub salt: [SigmaU8; SALT_LEN],
    pub key_offset: SigmaU32,
    pub stripes: SigmaU32,
}

impl KeySlot {
    pub const fn new() -> Self {
        Self {
            active: false,
            iterations: 0,
            salt: [0u8; SALT_LEN],
            key_offset: 0,
            stripes: 4000,
        }
    }
}

// ── Encrypted Volume ─────────────────────────────────────────────────────────
#[repr(C)]
pub struct EncryptedVolume {
    pub device_path: [SigmaU8; 256],
    pub header: LuksHeader,
    pub key_slots: [KeySlot; 8],
    pub algorithm: EncryptionAlgorithm,
    pub kdf_type: KdfType,
    pub unlocked: SigmaBool,
    pub mounted: SigmaBool,
}

impl EncryptedVolume {
    pub const fn new() -> Self {
        Self {
            device_path: [0u8; 256],
            header: LuksHeader::new(),
            key_slots: [KeySlot::new(); 8],
            algorithm: EncryptionAlgorithm::Aes256Xts,
            kdf_type: KdfType::Pbkdf2Sha256,
            unlocked: false,
            mounted: false,
        }
    }
}

// ── Volume Manager ─────────────────────────────────────────────────────────
pub struct VolumeManager {
    volumes: [EncryptedVolume; 16],
    count: SigmaUsize,
    default_encrypt_home: SigmaBool,
}

impl VolumeManager {
    pub const fn new() -> Self {
        Self {
            volumes: [EncryptedVolume::new(); 16],
            count: 0,
            default_encrypt_home: true,
        }
    }

    pub fn init(&mut self) {
        self.default_encrypt_home = true;
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Create encrypted volume on device.
    pub fn create_volume(
        &mut self,
        device_path: &[SigmaU8],
        passphrase: &[SigmaU8],
        algorithm: EncryptionAlgorithm,
    ) -> SigmaI32 {
        if self.count >= 16 {
            return -1;
        }

        let idx = self.count;
        let mut volume = EncryptedVolume::new();
        Self::copy_str(&mut volume.device_path, device_path);
        volume.algorithm = algorithm;

        // Generate random master key
        let mut master_key = [0u8; MASTER_KEY_LEN];
        // In production: use cryptographically secure RNG
        for i in 0..MASTER_KEY_LEN {
            master_key[i] = (i as SigmaU8).wrapping_mul(17).wrapping_add(42);
        }

        // Derive key from passphrase
        let salt = [0u8; SALT_LEN]; // In production: random salt
        let derived_key = self.pbkdf2_sha256(passphrase, &salt, PBKDF2_ITERATIONS);

        // Encrypt master key with derived key
        // In production: use actual encryption
        volume.header.mk_digest = derived_key;
        volume.header.mk_digest_salt = salt;

        // Add key slot
        volume.key_slots[0].active = true;
        volume.key_slots[0].iterations = PBKDF2_ITERATIONS;
        volume.key_slots[0].salt = salt;

        self.volumes[idx] = volume;
        self.count += 1;
        0
    }

    /// Unlock encrypted volume.
    pub fn unlock_volume(
        &mut self,
        device_path: &[SigmaU8],
        passphrase: &[SigmaU8],
    ) -> SigmaI32 {
        for i in 0..self.count {
            if self.device_path_matches(&self.volumes[i], device_path) {
                // Try to unlock with passphrase
                let salt = self.volumes[i].header.mk_digest_salt;
                let derived_key = self.pbkdf2_sha256(passphrase, &salt, PBKDF2_ITERATIONS);

                // In production: verify derived key against stored digest
                if derived_key == self.volumes[i].header.mk_digest {
                    self.volumes[i].unlocked = true;
                    return 0;
                }
            }
        }
        -1
    }

    /// Lock encrypted volume.
    pub fn lock_volume(&mut self, device_path: &[SigmaU8]) -> SigmaI32 {
        for i in 0..self.count {
            if self.device_path_matches(&self.volumes[i], device_path) {
                self.volumes[i].unlocked = false;
                self.volumes[i].mounted = false;
                return 0;
            }
        }
        -1
    }

    /// Mount encrypted volume.
    pub fn mount_volume(
        &mut self,
        device_path: &[SigmaU8],
        mount_point: &[SigmaU8],
    ) -> SigmaI32 {
        for i in 0..self.count {
            if self.device_path_matches(&self.volumes[i], device_path) {
                if !self.volumes[i].unlocked {
                    return -1;
                }
                // In production: mount via dm-crypt
                self.volumes[i].mounted = true;
                return 0;
            }
        }
        -1
    }

    /// Unmount encrypted volume.
    pub fn unmount_volume(&mut self, device_path: &[SigmaU8]) -> SigmaI32 {
        for i in 0..self.count {
            if self.device_path_matches(&self.volumes[i], device_path) {
                self.volumes[i].mounted = false;
                return 0;
            }
        }
        -1
    }

    /// Add passphrase to volume.
    pub fn add_passphrase(
        &mut self,
        device_path: &[SigmaU8],
        old_passphrase: &[SigmaU8],
        new_passphrase: &[SigmaU8],
    ) -> SigmaI32 {
        for i in 0..self.count {
            if self.device_path_matches(&self.volumes[i], device_path) {
                if !self.volumes[i].unlocked {
                    return -1;
                }

                // Find free key slot
                for slot_idx in 0..8 {
                    if !self.volumes[i].key_slots[slot_idx].active {
                        let salt = [0u8; SALT_LEN]; // In production: random salt
                        let derived_key = self.pbkdf2_sha256(new_passphrase, &salt, PBKDF2_ITERATIONS);

                        self.volumes[i].key_slots[slot_idx].active = true;
                        self.volumes[i].key_slots[slot_idx].iterations = PBKDF2_ITERATIONS;
                        self.volumes[i].key_slots[slot_idx].salt = salt;
                        return 0;
                    }
                }
                return -1; // No free slots
            }
        }
        -1
    }

    /// Remove passphrase from volume.
    pub fn remove_passphrase(
        &mut self,
        device_path: &[SigmaU8],
        passphrase: &[SigmaU8],
    ) -> SigmaI32 {
        for i in 0..self.count {
            if self.device_path_matches(&self.volumes[i], device_path) {
                if !self.volumes[i].unlocked {
                    return -1;
                }

                // Find matching key slot
                for slot_idx in 0..8 {
                    if self.volumes[i].key_slots[slot_idx].active {
                        let salt = self.volumes[i].key_slots[slot_idx].salt;
                        let derived_key = self.pbkdf2_sha256(passphrase, &salt, PBKDF2_ITERATIONS);

                        if derived_key == self.volumes[i].header.mk_digest {
                            self.volumes[i].key_slots[slot_idx].active = false;
                            return 0;
                        }
                    }
                }
                return -1; // Passphrase not found
            }
        }
        -1
    }

    /// Set default home encryption policy.
    pub fn set_encrypt_home_default(&mut self, encrypt: SigmaBool) {
        self.default_encrypt_home = encrypt;
    }

    /// Get default home encryption policy.
    pub fn encrypt_home_default(&self) -> SigmaBool {
        self.default_encrypt_home
    }

    // ── Helper Functions ───────────────────────────────────────────────────────

    fn device_path_matches(&self, volume: &EncryptedVolume, path: &[SigmaU8]) -> bool {
        if path.len() > volume.device_path.len() {
            return false;
        }
        let mut i = 0;
        while i < path.len() && volume.device_path[i] != 0 {
            if volume.device_path[i] != path[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    fn pbkdf2_sha256(
        &self,
        passphrase: &[SigmaU8],
        salt: &[SigmaU8],
        iterations: SigmaU32,
    ) -> [SigmaU8; 64] {
        // Simplified PBKDF2-SHA256
        // In production: use actual PBKDF2 implementation
        let mut result = [0u8; 64];
        
        // Simple hash for demonstration
        let mut hash: SigmaU32 = 5381;
        for &byte in passphrase.iter().chain(salt.iter()) {
            hash = hash.wrapping_mul(33).wrapping_add(byte as SigmaU32);
        }
        
        // Apply iterations
        let mut hash = hash;
        for _ in 0..iterations {
            hash = hash.wrapping_mul(31).wrapping_add(17);
        }

        // Convert to bytes
        for i in 0..64 {
            result[i] = ((hash >> (i * 4)) & 0xFF) as SigmaU8;
        }

        result
    }
}

static mut G_VOLUME_MGR: VolumeManager = VolumeManager::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_init() {
    G_VOLUME_MGR.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_create_volume(
    device_path: *const SigmaU8,
    path_len: SigmaUsize,
    passphrase: *const SigmaU8,
    pass_len: SigmaUsize,
    algorithm: SigmaU32,
) -> SigmaI32 {
    let dp = core::slice::from_raw_parts(device_path, path_len.min(256));
    let pp = core::slice::from_raw_parts(passphrase, pass_len.min(MAX_PASSPHRASE_LEN));
    let algo = match algorithm {
        0 => EncryptionAlgorithm::Aes256Xts,
        1 => EncryptionAlgorithm::Aes256Cbc,
        2 => EncryptionAlgorithm::ChaCha20Poly1305,
        _ => EncryptionAlgorithm::Aes256Xts,
    };
    G_VOLUME_MGR.create_volume(dp, pp, algo)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_unlock_volume(
    device_path: *const SigmaU8,
    path_len: SigmaUsize,
    passphrase: *const SigmaU8,
    pass_len: SigmaUsize,
) -> SigmaI32 {
    let dp = core::slice::from_raw_parts(device_path, path_len.min(256));
    let pp = core::slice::from_raw_parts(passphrase, pass_len.min(MAX_PASSPHRASE_LEN));
    G_VOLUME_MGR.unlock_volume(dp, pp)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_lock_volume(
    device_path: *const SigmaU8,
    path_len: SigmaUsize,
) -> SigmaI32 {
    let dp = core::slice::from_raw_parts(device_path, path_len.min(256));
    G_VOLUME_MGR.lock_volume(dp)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_mount_volume(
    device_path: *const SigmaU8,
    path_len: SigmaUsize,
    mount_point: *const SigmaU8,
    mount_len: SigmaUsize,
) -> SigmaI32 {
    let dp = core::slice::from_raw_parts(device_path, path_len.min(256));
    let mp = core::slice::from_raw_parts(mount_point, mount_len.min(256));
    G_VOLUME_MGR.mount_volume(dp, mp)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_unmount_volume(
    device_path: *const SigmaU8,
    path_len: SigmaUsize,
) -> SigmaI32 {
    let dp = core::slice::from_raw_parts(device_path, path_len.min(256));
    G_VOLUME_MGR.unmount_volume(dp)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_set_encrypt_home_default(encrypt: SigmaU32) {
    G_VOLUME_MGR.set_encrypt_home_default(encrypt != 0);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_luks_encrypt_home_default() -> SigmaU32 {
    if G_VOLUME_MGR.encrypt_home_default() { 1 } else { 0 }
}
