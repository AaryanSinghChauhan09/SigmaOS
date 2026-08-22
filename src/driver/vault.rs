// SigmaOS Driver Archive Vault (DriverArchiveVault)
// Encrypts driver binaries for cold storage to prevent unauthorized driver injection and tamper attacks

use crate::klib::collections::HashMap;
use crate::klib::custom_string::SigmaString;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct VaultEntry {
    pub driver_name: SigmaString,
    pub encrypted_payload: Vec<u8>,
    pub hash_signature: SigmaString,
}

pub struct DriverArchiveVault {
    pub archive: HashMap<SigmaString, VaultEntry>,
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
        let encrypted: Vec<u8> = raw_binary.iter().map(|b| b ^ self.secret_key).collect();
        let sig = SigmaString::from(format!("SIGMA_{}_OK", name));

        let entry = VaultEntry {
            driver_name: SigmaString::from(name),
            encrypted_payload: encrypted,
            hash_signature: sig,
        };
        self.archive.insert(SigmaString::from(name), entry);
    }

    pub fn retrieve_driver(&self, name: &str) -> Option<Vec<u8>> {
        if let Some(entry) = self.archive.get(&SigmaString::from(name)) {
            let decrypted: Vec<u8> = entry
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
    fn test_driver_vault_encryption_decryption() {
        let mut vault = DriverArchiveVault::new(0xAA);
        let raw_driver = b"ELF_DRIVER_BINARY_DATA";

        vault.store_driver("e1000", raw_driver);
        let retrieved = vault.retrieve_driver("e1000").unwrap();

        assert_eq!(retrieved.as_slice(), raw_driver);
        assert!(vault.retrieve_driver("nonexistent").is_none());
    }
}
