//! SigmaCrypto - Encryption Management
//!
//! Inspired by LUKS, this module provides comprehensive encryption management
//! including encryption key management, encrypted volume creation, and
//! encryption state tracking.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Encryption type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionType {
    AES256,
    AES512,
    Twofish,
    Serpent,
    Unknown,
}

/// Encryption state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionState {
    Locked,
    Unlocked,
    EncryptionInProgress,
    DecryptionInProgress,
    Failed,
    Unknown,
}

/// Encrypted volume data structure
#[derive(Debug, Clone)]
pub struct EncryptedVolume {
    pub volume_id: String,
    pub volume_name: String,
    pub device_path: String,
    pub encryption_type: EncryptionType,
    pub state: EncryptionState,
    pub key_id: String,
    pub size: u64,
}

impl EncryptedVolume {
    /// Create a new encrypted volume
    pub fn new(
        volume_id: String,
        volume_name: String,
        device_path: String,
        encryption_type: EncryptionType,
        key_id: String,
        size: u64,
    ) -> Self {
        Self {
            volume_id,
            volume_name,
            device_path,
            encryption_type,
            state: EncryptionState::Locked,
            key_id,
            size,
        }
    }

    /// Set encryption state
    pub fn set_state(&mut self, state: EncryptionState) {
        self.state = state;
    }

    /// Check if volume is unlocked
    pub fn is_unlocked(&self) -> bool {
        self.state == EncryptionState::Unlocked
    }

    /// Check if volume is locked
    pub fn is_locked(&self) -> bool {
        self.state == EncryptionState::Locked
    }
}

/// Encryption key data structure
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    pub key_id: String,
    pub key_name: String,
    pub key_type: EncryptionType,
    pub key_size: u32,
    pub is_active: bool,
}

impl EncryptionKey {
    /// Create a new encryption key
    pub fn new(key_id: String, key_name: String, key_type: EncryptionType, key_size: u32) -> Self {
        Self {
            key_id,
            key_name,
            key_type,
            key_size,
            is_active: true,
        }
    }

    /// Set key active state
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Check if key is active
    pub fn is_key_active(&self) -> bool {
        self.is_active
    }
}

/// Encryption manager
pub struct SigmaCrypto {
    volumes: BTreeMap<String, EncryptedVolume>,
    keys: BTreeMap<String, EncryptionKey>,
    running: AtomicBool,
}

impl SigmaCrypto {
    /// Create a new encryption manager
    pub fn new() -> Self {
        Self {
            volumes: BTreeMap::new(),
            keys: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the encryption manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the encryption manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add an encryption key
    pub fn add_key(&mut self, key: EncryptionKey) {
        self.keys.insert(key.key_id.clone(), key);
    }

    /// Remove an encryption key
    pub fn remove_key(&mut self, key_id: &str) -> Option<EncryptionKey> {
        self.keys.remove(key_id)
    }

    /// Get a key by ID
    pub fn get_key(&self, key_id: &str) -> Option<&EncryptionKey> {
        self.keys.get(key_id)
    }

    /// List all keys
    pub fn list_keys(&self) -> Vec<&EncryptionKey> {
        self.keys.values().collect()
    }

    /// List active keys
    pub fn list_active_keys(&self) -> Vec<&EncryptionKey> {
        self.keys.values().filter(|k| k.is_key_active()).collect()
    }

    /// Add an encrypted volume
    pub fn add_volume(&mut self, volume: EncryptedVolume) {
        self.volumes.insert(volume.volume_id.clone(), volume);
    }

    /// Remove an encrypted volume
    pub fn remove_volume(&mut self, volume_id: &str) -> Option<EncryptedVolume> {
        self.volumes.remove(volume_id)
    }

    /// Get a volume by ID
    pub fn get_volume(&self, volume_id: &str) -> Option<&EncryptedVolume> {
        self.volumes.get(volume_id)
    }

    /// List all volumes
    pub fn list_volumes(&self) -> Vec<&EncryptedVolume> {
        self.volumes.values().collect()
    }

    /// List volumes by encryption type
    pub fn list_volumes_by_type(&self, encryption_type: EncryptionType) -> Vec<&EncryptedVolume> {
        self.volumes
            .values()
            .filter(|v| v.encryption_type == encryption_type)
            .collect()
    }

    /// List volumes by state
    pub fn list_volumes_by_state(&self, state: EncryptionState) -> Vec<&EncryptedVolume> {
        self.volumes.values().filter(|v| v.state == state).collect()
    }

    /// Lock a volume
    pub fn lock_volume(&mut self, volume_id: &str) -> bool {
        if let Some(volume) = self.volumes.get_mut(volume_id) {
            volume.set_state(EncryptionState::Locked);
            true
        } else {
            false
        }
    }

    /// Unlock a volume
    pub fn unlock_volume(&mut self, volume_id: &str) -> bool {
        if let Some(volume) = self.volumes.get_mut(volume_id) {
            volume.set_state(EncryptionState::Unlocked);
            true
        } else {
            false
        }
    }

    /// Get volume count
    pub fn volume_count(&self) -> usize {
        self.volumes.len()
    }

    /// Get key count
    pub fn key_count(&self) -> usize {
        self.keys.len()
    }
}

impl Default for SigmaCrypto {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaCrypto instance (placeholder for actual global state)
static mut SIGMACRYPTO_INSTANCE: Option<SigmaCrypto> = None;

/// Initialize the global SigmaCrypto instance
pub fn init_sigmacrypto() {
    unsafe {
        SIGMACRYPTO_INSTANCE = Some(SigmaCrypto::new());
    }
}

/// Get the global SigmaCrypto instance
pub fn get_sigmacrypto() -> Option<&'static SigmaCrypto> {
    unsafe { SIGMACRYPTO_INSTANCE.as_ref() }
}

/// Get mutable global SigmaCrypto instance
pub fn get_sigmacrypto_mut() -> Option<&'static mut SigmaCrypto> {
    unsafe { SIGMACRYPTO_INSTANCE.as_mut() }
}

/// Convenience function to add an encryption key
pub fn add_encryption_key(key: EncryptionKey) {
    if let Some(crypto) = get_sigmacrypto_mut() {
        crypto.add_key(key);
    }
}

/// Convenience function to unlock a volume
pub fn unlock_volume(volume_id: &str) -> bool {
    if let Some(crypto) = get_sigmacrypto_mut() {
        crypto.unlock_volume(volume_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_key_creation() {
        let key = EncryptionKey::new(
            "key1".to_string(),
            "main_key".to_string(),
            EncryptionType::AES256,
            256,
        );

        assert_eq!(key.key_id, "key1");
        assert_eq!(key.key_name, "main_key");
        assert_eq!(key.encryption_type, EncryptionType::AES256);
        assert_eq!(key.key_size, 256);
        assert!(key.is_key_active());
    }

    #[test]
    fn test_encryption_key_active() {
        let mut key = EncryptionKey::new(
            "key1".to_string(),
            "main_key".to_string(),
            EncryptionType::AES256,
            256,
        );

        key.set_active(false);
        assert!(!key.is_key_active());

        key.set_active(true);
        assert!(key.is_key_active());
    }

    #[test]
    fn test_encrypted_volume_creation() {
        let volume = EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        );

        assert_eq!(volume.volume_id, "vol1");
        assert_eq!(volume.volume_name, "encrypted_data");
        assert_eq!(volume.encryption_type, EncryptionType::AES256);
        assert!(volume.is_locked());
    }

    #[test]
    fn test_encrypted_volume_state() {
        let mut volume = EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        );

        volume.set_state(EncryptionState::Unlocked);
        assert!(volume.is_unlocked());

        volume.set_state(EncryptionState::Locked);
        assert!(volume.is_locked());
    }

    #[test]
    fn test_sigmacrypto_creation() {
        let crypto = SigmaCrypto::new();
        assert!(!crypto.is_running());
        assert_eq!(crypto.volume_count(), 0);
        assert_eq!(crypto.key_count(), 0);
    }

    #[test]
    fn test_sigmacrypto_start_stop() {
        let mut crypto = SigmaCrypto::new();
        crypto.start();
        assert!(crypto.is_running());
        crypto.stop();
        assert!(!crypto.is_running());
    }

    #[test]
    fn test_sigmacrypto_add_key() {
        let mut crypto = SigmaCrypto::new();
        let key = EncryptionKey::new(
            "key1".to_string(),
            "main_key".to_string(),
            EncryptionType::AES256,
            256,
        );

        crypto.add_key(key);
        assert_eq!(crypto.key_count(), 1);
        assert!(crypto.get_key("key1").is_some());
    }

    #[test]
    fn test_sigmacrypto_remove_key() {
        let mut crypto = SigmaCrypto::new();
        let key = EncryptionKey::new(
            "key1".to_string(),
            "main_key".to_string(),
            EncryptionType::AES256,
            256,
        );

        crypto.add_key(key);
        assert_eq!(crypto.key_count(), 1);

        let removed = crypto.remove_key("key1");
        assert!(removed.is_some());
        assert_eq!(crypto.key_count(), 0);
    }

    #[test]
    fn test_sigmacrypto_add_volume() {
        let mut crypto = SigmaCrypto::new();
        let volume = EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        );

        crypto.add_volume(volume);
        assert_eq!(crypto.volume_count(), 1);
        assert!(crypto.get_volume("vol1").is_some());
    }

    #[test]
    fn test_sigmacrypto_lock_unlock() {
        let mut crypto = SigmaCrypto::new();
        let volume = EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        );

        crypto.add_volume(volume);
        let unlocked = crypto.unlock_volume("vol1");
        assert!(unlocked);

        let volume = crypto.get_volume("vol1").unwrap();
        assert!(volume.is_unlocked());

        let locked = crypto.lock_volume("vol1");
        assert!(locked);

        let volume = crypto.get_volume("vol1").unwrap();
        assert!(volume.is_locked());
    }

    #[test]
    fn test_sigmacrypto_list_volumes_by_type() {
        let mut crypto = SigmaCrypto::new();

        crypto.add_volume(EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data1".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        ));

        crypto.add_volume(EncryptedVolume::new(
            "vol2".to_string(),
            "encrypted_data2".to_string(),
            "/dev/sda2".to_string(),
            EncryptionType::AES256,
            "key2".to_string(),
            1024 * 1024 * 1024,
        ));

        crypto.add_volume(EncryptedVolume::new(
            "vol3".to_string(),
            "encrypted_data3".to_string(),
            "/dev/sda3".to_string(),
            EncryptionType::AES512,
            "key3".to_string(),
            1024 * 1024 * 1024,
        ));

        let aes256_vols = crypto.list_volumes_by_type(EncryptionType::AES256);
        assert_eq!(aes256_vols.len(), 2);

        let aes512_vols = crypto.list_volumes_by_type(EncryptionType::AES512);
        assert_eq!(aes512_vols.len(), 1);
    }

    #[test]
    fn test_sigmacrypto_list_volumes_by_state() {
        let mut crypto = SigmaCrypto::new();

        let mut vol1 = EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data1".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        );
        vol1.set_state(EncryptionState::Unlocked);
        crypto.add_volume(vol1);

        let vol2 = EncryptedVolume::new(
            "vol2".to_string(),
            "encrypted_data2".to_string(),
            "/dev/sda2".to_string(),
            EncryptionType::AES256,
            "key2".to_string(),
            1024 * 1024 * 1024,
        );
        crypto.add_volume(vol2);

        let unlocked_vols = crypto.list_volumes_by_state(EncryptionState::Unlocked);
        assert_eq!(unlocked_vols.len(), 1);

        let locked_vols = crypto.list_volumes_by_state(EncryptionState::Locked);
        assert_eq!(locked_vols.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmacrypto();

        let crypto = get_sigmacrypto();
        assert!(crypto.is_some());

        let mut crypto_mut = get_sigmacrypto_mut();
        assert!(crypto_mut.is_some());

        if let Some(crypto) = crypto_mut {
            crypto.start();
            assert!(crypto.is_running());
        }
    }

    #[test]
    fn test_convenience_add_key() {
        init_sigmacrypto();

        let key = EncryptionKey::new(
            "key1".to_string(),
            "main_key".to_string(),
            EncryptionType::AES256,
            256,
        );

        add_encryption_key(key);

        let crypto = get_sigmacrypto().unwrap();
        assert_eq!(crypto.key_count(), 1);
    }

    #[test]
    fn test_convenience_unlock_volume() {
        init_sigmacrypto();

        let volume = EncryptedVolume::new(
            "vol1".to_string(),
            "encrypted_data".to_string(),
            "/dev/sda1".to_string(),
            EncryptionType::AES256,
            "key1".to_string(),
            1024 * 1024 * 1024,
        );

        let crypto = get_sigmacrypto_mut().unwrap();
        crypto.add_volume(volume);

        let unlocked = unlock_volume("vol1");
        assert!(unlocked);

        let crypto = get_sigmacrypto().unwrap();
        let volume = crypto.get_volume("vol1").unwrap();
        assert!(volume.is_unlocked());
    }
}
