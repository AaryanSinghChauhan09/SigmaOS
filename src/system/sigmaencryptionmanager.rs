#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Encryption manager for encryption and decryption operations
/// Inspired by Linux cryptographic tools (LUKS, cryptsetup, gpg)
#[derive(Debug, Clone)]
pub struct EncryptionManager {
    volumes: Vec<EncryptedVolume>,
    keys: Vec<EncryptionKey>,
}

/// An encrypted volume
#[derive(Debug, Clone)]
pub struct EncryptedVolume {
    id: String,
    name: String,
    device_path: String,
    cipher: CipherAlgorithm,
    state: VolumeState,
    key_id: Option<String>,
}

/// Encryption key
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    id: String,
    name: String,
    key_type: KeyType,
    algorithm: CipherAlgorithm,
    key_size_bits: u32,
}

/// Cipher algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherAlgorithm {
    Aes256,
    Aes128,
    ChaCha20,
    Twofish,
    Serpent,
}

/// Key type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    Passphrase,
    KeyFile,
    Token,
}

/// Volume state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeState {
    Locked,
    Unlocked,
    Active,
    Inactive,
}

impl EncryptionManager {
    pub fn new() -> Self {
        Self {
            volumes: Vec::new(),
            keys: Vec::new(),
        }
    }

    /// Create an encryption key
    pub fn create_key(&mut self, id: String, name: String, key_type: KeyType, algorithm: CipherAlgorithm, key_size_bits: u32) -> Result<String, String> {
        if self.keys.iter().any(|k| k.id == id) {
            return Err(format!("Key with id {} already exists", id));
        }
        
        let key = EncryptionKey {
            id: id.clone(),
            name,
            key_type,
            algorithm,
            key_size_bits,
        };
        self.keys.push(key);
        Ok(id)
    }

    /// Remove an encryption key
    pub fn remove_key(&mut self, id: &str) -> Result<(), String> {
        let pos = self.keys.iter().position(|k| k.id == id)
            .ok_or_else(|| format!("Key with id {} not found", id))?;
        self.keys.remove(pos);
        Ok(())
    }

    /// Create an encrypted volume
    pub fn create_volume(&mut self, id: String, name: String, device_path: String, cipher: CipherAlgorithm, key_id: Option<String>) -> Result<String, String> {
        if self.volumes.iter().any(|v| v.id == id) {
            return Err(format!("Volume with id {} already exists", id));
        }
        
        let volume = EncryptedVolume {
            id: id.clone(),
            name,
            device_path,
            cipher,
            state: VolumeState::Locked,
            key_id,
        };
        self.volumes.push(volume);
        Ok(id)
    }

    /// Unlock a volume
    pub fn unlock_volume(&mut self, id: &str, key_id: &str) -> Result<(), String> {
        let volume = self.volumes.iter_mut().find(|v| v.id == id)
            .ok_or_else(|| format!("Volume with id {} not found", id))?;
        volume.state = VolumeState::Unlocked;
        volume.key_id = Some(key_id.to_string());
        Ok(())
    }

    /// Lock a volume
    pub fn lock_volume(&mut self, id: &str) -> Result<(), String> {
        let volume = self.volumes.iter_mut().find(|v| v.id == id)
            .ok_or_else(|| format!("Volume with id {} not found", id))?;
        volume.state = VolumeState::Locked;
        volume.key_id = None;
        Ok(())
    }

    /// Activate a volume
    pub fn activate_volume(&mut self, id: &str) -> Result<(), String> {
        let volume = self.volumes.iter_mut().find(|v| v.id == id)
            .ok_or_else(|| format!("Volume with id {} not found", id))?;
        volume.state = VolumeState::Active;
        Ok(())
    }

    /// Deactivate a volume
    pub fn deactivate_volume(&mut self, id: &str) -> Result<(), String> {
        let volume = self.volumes.iter_mut().find(|v| v.id == id)
            .ok_or_else(|| format!("Volume with id {} not found", id))?;
        volume.state = VolumeState::Inactive;
        Ok(())
    }

    /// Remove a volume
    pub fn remove_volume(&mut self, id: &str) -> Result<(), String> {
        let pos = self.volumes.iter().position(|v| v.id == id)
            .ok_or_else(|| format!("Volume with id {} not found", id))?;
        self.volumes.remove(pos);
        Ok(())
    }

    /// Get all volumes
    pub fn get_volumes(&self) -> &[EncryptedVolume] {
        &self.volumes
    }

    /// Get unlocked volumes
    pub fn get_unlocked_volumes(&self) -> Vec<&EncryptedVolume> {
        self.volumes.iter().filter(|v| v.state == VolumeState::Unlocked || v.state == VolumeState::Active).collect()
    }

    /// Get volume by ID
    pub fn get_volume(&self, id: &str) -> Option<&EncryptedVolume> {
        self.volumes.iter().find(|v| v.id == id)
    }

    /// Get all keys
    pub fn get_keys(&self) -> &[EncryptionKey] {
        &self.keys
    }

    /// Get key by ID
    pub fn get_key(&self, id: &str) -> Option<&EncryptionKey> {
        self.keys.iter().find(|k| k.id == id)
    }
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_key() {
        let mut manager = EncryptionManager::new();
        let result = manager.create_key("key1".to_string(), "Main Key".to_string(), KeyType::Passphrase, CipherAlgorithm::Aes256, 256);
        assert!(result.is_ok());
        assert_eq!(manager.get_keys().len(), 1);
    }

    #[test]
    fn test_duplicate_key() {
        let mut manager = EncryptionManager::new();
        manager.create_key("key1".to_string(), "Main Key".to_string(), KeyType::Passphrase, CipherAlgorithm::Aes256, 256).unwrap();
        let result = manager.create_key("key1".to_string(), "Other Key".to_string(), KeyType::KeyFile, CipherAlgorithm::ChaCha20, 256);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_volume() {
        let mut manager = EncryptionManager::new();
        let result = manager.create_volume("vol1".to_string(), "Main Volume".to_string(), "/dev/sda1".to_string(), CipherAlgorithm::Aes256, None);
        assert!(result.is_ok());
        assert_eq!(manager.get_volumes().len(), 1);
    }

    #[test]
    fn test_unlock_lock_volume() {
        let mut manager = EncryptionManager::new();
        manager.create_key("key1".to_string(), "Main Key".to_string(), KeyType::Passphrase, CipherAlgorithm::Aes256, 256).unwrap();
        manager.create_volume("vol1".to_string(), "Main Volume".to_string(), "/dev/sda1".to_string(), CipherAlgorithm::Aes256, None).unwrap();
        
        manager.unlock_volume("vol1", "key1").unwrap();
        assert_eq!(manager.get_volume("vol1").unwrap().state, VolumeState::Unlocked);
        
        manager.lock_volume("vol1").unwrap();
        assert_eq!(manager.get_volume("vol1").unwrap().state, VolumeState::Locked);
    }

    #[test]
    fn test_activate_deactivate_volume() {
        let mut manager = EncryptionManager::new();
        manager.create_volume("vol1".to_string(), "Main Volume".to_string(), "/dev/sda1".to_string(), CipherAlgorithm::Aes256, None).unwrap();
        
        manager.activate_volume("vol1").unwrap();
        assert_eq!(manager.get_volume("vol1").unwrap().state, VolumeState::Active);
        
        manager.deactivate_volume("vol1").unwrap();
        assert_eq!(manager.get_volume("vol1").unwrap().state, VolumeState::Inactive);
    }
}
