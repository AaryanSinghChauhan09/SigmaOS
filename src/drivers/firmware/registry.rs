// src/drivers/firmware/registry.rs
// Declarative firmware blob registry for Linux/BSD drivers in SigmaOS

use std::vec::Vec;
use std::string::String;
use std::format;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareBlob {
    pub name: String,           // e.g. "iwlwifi-ax210.ucode"
    pub hash_sha256: String,    // Integrity check
    pub size_bytes: usize,
    pub license: String,        // "Proprietary" or "GPL-2.0"
    pub url_source: String,     // github.com/torvalds/linux-firmware
}

#[derive(Debug, Clone)]
pub struct FirmwareRegistry {
    pub blobs: Vec<FirmwareBlob>,
}

impl FirmwareRegistry {
    pub fn new() -> Self {
        Self {
            blobs: Vec::new(),
        }
    }

    /// Declare firmware at compile-time / initial registry setup
    pub fn declare_blob(name: &str, sha256: &str, license: &str) -> Self {
        let mut registry = Self::new();
        registry.add_blob(FirmwareBlob {
            name: name.to_string(),
            hash_sha256: sha256.to_string(),
            size_bytes: 0,
            license: license.to_string(),
            url_source: format!("https://github.com/torvalds/linux-firmware/raw/main/{}", name),
        });
        registry
    }

    pub fn add_blob(&mut self, blob: FirmwareBlob) {
        if !self.blobs.iter().any(|b| b.name == blob.name) {
            self.blobs.push(blob);
        }
    }

    pub fn get_blob(&self, name: &str) -> Option<&FirmwareBlob> {
        self.blobs.iter().find(|b| b.name == name)
    }

    pub fn verify_sha256(&self, name: &str, calculated_sha256: &str) -> bool {
        if let Some(blob) = self.get_blob(name) {
            blob.hash_sha256.eq_ignore_ascii_case(calculated_sha256)
        } else {
            false
        }
    }

    pub fn list_blobs(&self) -> &[FirmwareBlob] {
        &self.blobs
    }
}

impl Default for FirmwareRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declare_firmware_blob() {
        let registry = FirmwareRegistry::declare_blob("iwlwifi-ax210.ucode", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", "Proprietary");
        assert_eq!(registry.blobs.len(), 1);
        let blob = registry.get_blob("iwlwifi-ax210.ucode").unwrap();
        assert_eq!(blob.license, "Proprietary");
        assert!(blob.url_source.contains("iwlwifi-ax210.ucode"));
    }

    #[test]
    fn test_firmware_registry_verification() {
        let mut registry = FirmwareRegistry::new();
        registry.add_blob(FirmwareBlob {
            name: "amdgpu/picasso_asd.bin".to_string(),
            hash_sha256: "11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff".to_string(),
            size_bytes: 184320,
            license: "GPL-2.0".to_string(),
            url_source: "https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git".to_string(),
        });

        assert!(registry.verify_sha256("amdgpu/picasso_asd.bin", "11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff"));
        assert!(!registry.verify_sha256("amdgpu/picasso_asd.bin", "wronghash"));
        assert!(!registry.verify_sha256("nonexistent.bin", "11223344556677889900aabbccddeeff11223344556677889900aabbccddeeff"));
    }
}
