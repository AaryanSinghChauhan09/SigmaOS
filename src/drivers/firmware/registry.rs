// Firmware Blob Registry & Loader System for SigmaOS (`src/drivers/firmware/registry.rs`)

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct FirmwareBlobSpec {
    pub name: String,
    pub target_driver: String,
    pub payload_bytes: Vec<u8>,
    pub version: String,
}

pub struct SovereignFirmwareRegistryEngine {
    pub firmware_blobs: BTreeMap<String, FirmwareBlobSpec>,
}

impl SovereignFirmwareRegistryEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            firmware_blobs: BTreeMap::new(),
        };
        engine.register_default_firmware_blobs();
        engine
    }

    fn register_default_firmware_blobs(&mut self) {
        let default_blobs = vec![
            ("iwlwifi-ax210.ucode", "iwlwifi", "72.a1158133.0", vec![0x11, 0x22, 0x33, 0x44]),
            ("rtl8852ae.bin", "rtw89", "1.0.4", vec![0x55, 0x66, 0x77, 0x88]),
            ("i915-guc.bin", "intel_i915", "70.1.1", vec![0x99, 0xAA, 0xBB, 0xCC]),
            ("amdgpu-navi10.bin", "amdgpu", "2.1.0", vec![0xDD, 0xEE, 0xFF, 0x00]),
        ];

        for (name, driver, ver, payload) in default_blobs {
            self.firmware_blobs.insert(
                name.to_string(),
                FirmwareBlobSpec {
                    name: name.to_string(),
                    target_driver: driver.to_string(),
                    payload_bytes: payload,
                    version: ver.to_string(),
                },
            );
        }
    }

    pub fn load_firmware(&self, blob_name: &str) -> Option<&FirmwareBlobSpec> {
        self.firmware_blobs.get(blob_name)
    }
}

impl Default for SovereignFirmwareRegistryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_registry() {
        let registry = SovereignFirmwareRegistryEngine::new();
        assert_eq!(registry.firmware_blobs.len(), 4);

        let iwl = registry.load_firmware("iwlwifi-ax210.ucode").unwrap();
        assert_eq!(iwl.target_driver, "iwlwifi");
        assert_eq!(iwl.payload_bytes, vec![0x11, 0x22, 0x33, 0x44]);
    }
}
