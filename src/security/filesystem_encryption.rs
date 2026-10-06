// Filesystem Encryption Manager for SigmaOS
// Filesystem encryption per Wiki 07-Security.md
// Provides fscrypt and LUKS-style filesystem encryption

use std::string::{String, ToString};
use std::vec::Vec;

/// Encryption type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionType {
    Fscrypt, // Directory-level encryption
    Luks,    // Block device encryption
}

impl EncryptionType {
    pub fn as_str(&self) -> &str {
        match self {
            EncryptionType::Fscrypt => "fscrypt",
            EncryptionType::Luks => "luks",
        }
    }
}

/// Encryption algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    Aes256Xts,
    Aes256Gcm,
    Chacha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            EncryptionAlgorithm::Aes256Xts => "aes-256-xts",
            EncryptionAlgorithm::Aes256Gcm => "aes-256-gcm",
            EncryptionAlgorithm::Chacha20Poly1305 => "chacha20-poly1305",
        }
    }
}

/// Encryption status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionStatus {
    Unencrypted,
    Encrypted,
    Locked,
    Unlocked,
}

impl EncryptionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            EncryptionStatus::Unencrypted => "unencrypted",
            EncryptionStatus::Encrypted => "encrypted",
            EncryptionStatus::Locked => "locked",
            EncryptionStatus::Unlocked => "unlocked",
        }
    }
}

/// Fscrypt directory encryption
#[derive(Debug, Clone)]
pub struct FscryptDirectory {
    pub path: String,
    pub status: EncryptionStatus,
    pub algorithm: EncryptionAlgorithm,
    pub key_descriptor: Option<String>,
}

impl FscryptDirectory {
    pub fn new(path: String) -> Self {
        FscryptDirectory {
            path,
            status: EncryptionStatus::Unencrypted,
            algorithm: EncryptionAlgorithm::Aes256Xts,
            key_descriptor: None,
        }
    }

    pub fn set_status(&mut self, status: EncryptionStatus) {
        self.status = status;
    }

    pub fn set_algorithm(&mut self, algorithm: EncryptionAlgorithm) {
        self.algorithm = algorithm;
    }

    pub fn set_key_descriptor(&mut self, key_descriptor: String) {
        self.key_descriptor = Some(key_descriptor);
    }
}

/// LUKS block device encryption
#[derive(Debug, Clone)]
pub struct LuksDevice {
    pub device_path: String,
    pub mapper_name: String,
    pub status: EncryptionStatus,
    pub algorithm: EncryptionAlgorithm,
    pub key_slot: Option<u32>,
}

impl LuksDevice {
    pub fn new(device_path: String, mapper_name: String) -> Self {
        LuksDevice {
            device_path,
            mapper_name,
            status: EncryptionStatus::Unencrypted,
            algorithm: EncryptionAlgorithm::Aes256Xts,
            key_slot: None,
        }
    }

    pub fn set_status(&mut self, status: EncryptionStatus) {
        self.status = status;
    }

    pub fn set_algorithm(&mut self, algorithm: EncryptionAlgorithm) {
        self.algorithm = algorithm;
    }

    pub fn set_key_slot(&mut self, key_slot: u32) {
        self.key_slot = Some(key_slot);
    }

    pub fn get_mapper_path(&self) -> String {
        format!("/dev/mapper/{}", self.mapper_name)
    }
}

/// Filesystem encryption manager
#[derive(Debug, Clone)]
pub struct FilesystemEncryptionManager {
    pub fscrypt_directories: Vec<FscryptDirectory>,
    pub luks_devices: Vec<LuksDevice>,
}

impl Default for FilesystemEncryptionManager {
    fn default() -> Self {
        FilesystemEncryptionManager {
            fscrypt_directories: Vec::new(),
            luks_devices: Vec::new(),
        }
    }
}

impl FilesystemEncryptionManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Encrypt directory with fscrypt
    pub fn encrypt_directory(
        &mut self,
        path: String,
        algorithm: EncryptionAlgorithm,
    ) -> Result<(), String> {
        if self.fscrypt_directories.iter().any(|d| d.path == path) {
            return Err(format!("Directory {} already managed", path));
        }

        let mut directory = FscryptDirectory::new(path);
        directory.set_algorithm(algorithm);
        directory.set_status(EncryptionStatus::Encrypted);
        self.fscrypt_directories.push(directory);
        Ok(())
    }

    /// Lock fscrypt directory
    pub fn lock_directory(&mut self, path: &str) -> Result<(), String> {
        let directory = self
            .fscrypt_directories
            .iter_mut()
            .find(|d| d.path == path)
            .ok_or_else(|| format!("Directory {} not found", path))?;

        if directory.status != EncryptionStatus::Unlocked {
            return Err(format!("Directory {} is not unlocked", path));
        }

        directory.set_status(EncryptionStatus::Locked);
        Ok(())
    }

    /// Unlock fscrypt directory
    pub fn unlock_directory(&mut self, path: &str, key_descriptor: String) -> Result<(), String> {
        let directory = self
            .fscrypt_directories
            .iter_mut()
            .find(|d| d.path == path)
            .ok_or_else(|| format!("Directory {} not found", path))?;

        if directory.status != EncryptionStatus::Encrypted
            && directory.status != EncryptionStatus::Locked
        {
            return Err(format!("Directory {} is not encrypted or locked", path));
        }

        directory.set_key_descriptor(key_descriptor);
        directory.set_status(EncryptionStatus::Unlocked);
        Ok(())
    }

    /// Format LUKS device
    pub fn format_luks_device(
        &mut self,
        device_path: String,
        mapper_name: String,
        algorithm: EncryptionAlgorithm,
    ) -> Result<(), String> {
        if self.luks_devices.iter().any(|d| d.device_path == device_path) {
            return Err(format!("Device {} already managed", device_path));
        }

        let mut device = LuksDevice::new(device_path, mapper_name);
        device.set_algorithm(algorithm);
        device.set_status(EncryptionStatus::Encrypted);
        self.luks_devices.push(device);
        Ok(())
    }

    /// Open LUKS device
    pub fn open_luks_device(&mut self, device_path: &str, key_slot: u32) -> Result<(), String> {
        let device = self
            .luks_devices
            .iter_mut()
            .find(|d| d.device_path == device_path)
            .ok_or_else(|| format!("Device {} not found", device_path))?;

        if device.status != EncryptionStatus::Encrypted
            && device.status != EncryptionStatus::Locked
        {
            return Err(format!("Device {} is not encrypted or locked", device_path));
        }

        device.set_key_slot(key_slot);
        device.set_status(EncryptionStatus::Unlocked);
        Ok(())
    }

    /// Close LUKS device
    pub fn close_luks_device(&mut self, device_path: &str) -> Result<(), String> {
        let device = self
            .luks_devices
            .iter_mut()
            .find(|d| d.device_path == device_path)
            .ok_or_else(|| format!("Device {} not found", device_path))?;

        if device.status != EncryptionStatus::Unlocked {
            return Err(format!("Device {} is not unlocked", device_path));
        }

        device.set_status(EncryptionStatus::Locked);
        Ok(())
    }

    /// Get fscrypt directory
    pub fn get_directory(&self, path: &str) -> Option<&FscryptDirectory> {
        self.fscrypt_directories.iter().find(|d| d.path == path)
    }

    /// Get LUKS device
    pub fn get_luks_device(&self, device_path: &str) -> Option<&LuksDevice> {
        self.luks_devices.iter().find(|d| d.device_path == device_path)
    }

    /// List fscrypt directories
    pub fn list_directories(&self) -> Vec<String> {
        self.fscrypt_directories
            .iter()
            .map(|d| format!("{} ({}, {})", d.path, d.status.as_str(), d.algorithm.as_str()))
            .collect()
    }

    /// List LUKS devices
    pub fn list_luks_devices(&self) -> Vec<String> {
        self.luks_devices
            .iter()
            .map(|d| {
                format!(
                    "{} -> {} ({}, {})",
                    d.device_path,
                    d.mapper_name,
                    d.status.as_str(),
                    d.algorithm.as_str()
                )
            })
            .collect()
    }

    /// Get encryption statistics
    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Filesystem Encryption Statistics:\n");

        let encrypted_dirs = self
            .fscrypt_directories
            .iter()
            .filter(|d| {
                d.status == EncryptionStatus::Encrypted || d.status == EncryptionStatus::Locked
            })
            .count();

        let unlocked_dirs = self
            .fscrypt_directories
            .iter()
            .filter(|d| d.status == EncryptionStatus::Unlocked)
            .count();

        let encrypted_devices = self
            .luks_devices
            .iter()
            .filter(|d| {
                d.status == EncryptionStatus::Encrypted || d.status == EncryptionStatus::Locked
            })
            .count();

        let unlocked_devices = self
            .luks_devices
            .iter()
            .filter(|d| d.status == EncryptionStatus::Unlocked)
            .count();

        stats.push_str(&format!(
            "Fscrypt directories: {} total, {} encrypted, {} unlocked\n",
            self.fscrypt_directories.len(),
            encrypted_dirs,
            unlocked_dirs
        ));
        stats.push_str(&format!(
            "LUKS devices: {} total, {} encrypted, {} unlocked\n",
            self.luks_devices.len(),
            encrypted_devices,
            unlocked_devices
        ));

        stats
    }

    /// Check if path is encrypted
    pub fn is_directory_encrypted(&self, path: &str) -> bool {
        self.fscrypt_directories.iter().any(|d| {
            d.path == path
                && (d.status == EncryptionStatus::Encrypted
                    || d.status == EncryptionStatus::Locked
                    || d.status == EncryptionStatus::Unlocked)
        })
    }

    /// Check if device is encrypted
    pub fn is_device_encrypted(&self, device_path: &str) -> bool {
        self.luks_devices.iter().any(|d| {
            d.device_path == device_path
                && (d.status == EncryptionStatus::Encrypted
                    || d.status == EncryptionStatus::Locked
                    || d.status == EncryptionStatus::Unlocked)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_type_as_str() {
        assert_eq!(EncryptionType::Fscrypt.as_str(), "fscrypt");
        assert_eq!(EncryptionType::Luks.as_str(), "luks");
    }

    #[test]
    fn test_encryption_algorithm_as_str() {
        assert_eq!(EncryptionAlgorithm::Aes256Xts.as_str(), "aes-256-xts");
        assert_eq!(
            EncryptionAlgorithm::Chacha20Poly1305.as_str(),
            "chacha20-poly1305"
        );
    }

    #[test]
    fn test_encryption_status_as_str() {
        assert_eq!(EncryptionStatus::Encrypted.as_str(), "encrypted");
        assert_eq!(EncryptionStatus::Locked.as_str(), "locked");
    }

    #[test]
    fn test_fscrypt_directory_creation() {
        let directory = FscryptDirectory::new(String::from("/home/user/sensitive"));
        assert_eq!(directory.path, "/home/user/sensitive");
        assert_eq!(directory.status, EncryptionStatus::Unencrypted);
    }

    #[test]
    fn test_fscrypt_directory_set_status() {
        let mut directory = FscryptDirectory::new(String::from("/home/user/sensitive"));
        directory.set_status(EncryptionStatus::Encrypted);
        assert_eq!(directory.status, EncryptionStatus::Encrypted);
    }

    #[test]
    fn test_luks_device_creation() {
        let device = LuksDevice::new(String::from("/dev/sda1"), String::from("cryptdata"));
        assert_eq!(device.device_path, "/dev/sda1");
        assert_eq!(device.mapper_name, "cryptdata");
    }

    #[test]
    fn test_luks_device_get_mapper_path() {
        let device = LuksDevice::new(String::from("/dev/sda1"), String::from("cryptdata"));
        assert_eq!(device.get_mapper_path(), "/dev/mapper/cryptdata");
    }

    #[test]
    fn test_filesystem_encryption_manager_creation() {
        let manager = FilesystemEncryptionManager::new();
        assert_eq!(manager.fscrypt_directories.len(), 0);
        assert_eq!(manager.luks_devices.len(), 0);
    }

    #[test]
    fn test_filesystem_encryption_manager_encrypt_directory() {
        let mut manager = FilesystemEncryptionManager::new();
        assert!(manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts
            )
            .is_ok());
        assert_eq!(manager.fscrypt_directories.len(), 1);
    }

    #[test]
    fn test_filesystem_encryption_manager_encrypt_duplicate_directory() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        assert!(manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts
            )
            .is_err());
    }

    #[test]
    fn test_filesystem_encryption_manager_lock_directory() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        assert!(manager.lock_directory("/home/user/sensitive").is_err());
    }

    #[test]
    fn test_filesystem_encryption_manager_unlock_directory() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        assert!(manager
            .unlock_directory("/home/user/sensitive", String::from("key1"))
            .is_ok());
    }

    #[test]
    fn test_filesystem_encryption_manager_format_luks_device() {
        let mut manager = FilesystemEncryptionManager::new();
        assert!(manager
            .format_luks_device(
                String::from("/dev/sda1"),
                String::from("cryptdata"),
                EncryptionAlgorithm::Aes256Xts
            )
            .is_ok());
        assert_eq!(manager.luks_devices.len(), 1);
    }

    #[test]
    fn test_filesystem_encryption_manager_open_luks_device() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .format_luks_device(
                String::from("/dev/sda1"),
                String::from("cryptdata"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        assert!(manager.open_luks_device("/dev/sda1", 0).is_ok());
    }

    #[test]
    fn test_filesystem_encryption_manager_close_luks_device() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .format_luks_device(
                String::from("/dev/sda1"),
                String::from("cryptdata"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        manager.open_luks_device("/dev/sda1", 0).unwrap();
        assert!(manager.close_luks_device("/dev/sda1").is_ok());
    }

    #[test]
    fn test_filesystem_encryption_manager_list_directories() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        let dirs = manager.list_directories();
        assert_eq!(dirs.len(), 1);
    }

    #[test]
    fn test_filesystem_encryption_manager_list_luks_devices() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .format_luks_device(
                String::from("/dev/sda1"),
                String::from("cryptdata"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        let devices = manager.list_luks_devices();
        assert_eq!(devices.len(), 1);
    }

    #[test]
    fn test_filesystem_encryption_manager_get_statistics() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        let stats = manager.get_statistics();
        assert!(stats.contains("Fscrypt directories: 1"));
    }

    #[test]
    fn test_filesystem_encryption_manager_is_directory_encrypted() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .encrypt_directory(
                String::from("/home/user/sensitive"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        assert!(manager.is_directory_encrypted("/home/user/sensitive"));
    }

    #[test]
    fn test_filesystem_encryption_manager_is_device_encrypted() {
        let mut manager = FilesystemEncryptionManager::new();
        manager
            .format_luks_device(
                String::from("/dev/sda1"),
                String::from("cryptdata"),
                EncryptionAlgorithm::Aes256Xts,
            )
            .unwrap();
        assert!(manager.is_device_encrypted("/dev/sda1"));
    }
}
