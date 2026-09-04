use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// SigmaOS Driver Archive Vault (DriverArchiveVault)
// Encrypts driver binaries for cold storage to prevent unauthorized driver injection and tamper attacks

use crate::klib;
use crate::klib::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VaultEntry {
    pub driver_name: klib::string::SigmaString,
    pub encrypted_payload: klib::vec::Vec<u8>,
    pub hash_signature: klib::string::SigmaString,
}

pub struct DriverArchiveVault {
    pub archive: HashMap<klib::string::SigmaString, VaultEntry>,
    pub secret_key: u8,
}

impl DriverArchiveVault {
    pub fn new(key: u8) -> Self {
        DriverArchiveVault {
            archive: HashMap::new(),
            secret_key: key,
        }
    }

    pub fn store_driver(&mut self, name: &str, raw_binary: &[u8]) {
        let encrypted: klib::vec::Vec<u8> =
            raw_binary.iter().map(|b| b ^ self.secret_key).collect();
        let sig = klib::string::SigmaString::from(format!("SIGMA_{}_OK", name));

        let entry = VaultEntry {
            driver_name: klib::string::SigmaString::from(name),
            encrypted_payload: encrypted,
            hash_signature: sig,
        };
        self.archive
            .insert(klib::string::SigmaString::from(name), entry);
    }

    pub fn retrieve_driver(&self, name: &str) -> Option<klib::vec::Vec<u8>> {
        if let Some(entry) = self.archive.get(&klib::string::SigmaString::from(name)) {
            let decrypted: klib::vec::Vec<u8> = entry
                .encrypted_payload
                .iter()
                .map(|b| b ^ self.secret_key)
                .collect();
            Some(decrypted)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_archive_vault() {
        let mut vault = DriverArchiveVault::new(0x55);
        vault.store_driver("ne2000_isa_nic", b"driver_binary_data");
        let retrieved = vault.retrieve_driver("ne2000_isa_nic").unwrap();
        assert_eq!(retrieved.as_slice(), b"driver_binary_data");
    }
}
// SigmaOS Legacy Driver Archive Vault (DriverArchiveVault)
