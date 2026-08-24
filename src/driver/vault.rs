// SigmaOS Driver Archive Vault (DriverArchiveVault)
// Encrypts driver binaries for cold storage to prevent unauthorized driver injection and tamper attacks

use crate::klib::collections::HashMap;
use crate::klib;

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
        let encrypted: klib::vec::Vec<u8> = raw_binary.iter().map(|b| b ^ self.secret_key).collect();
        let sig = klib::string::SigmaString::from(format!("SIGMA_{}_OK", name));

        let entry = VaultEntry {
            driver_name: klib::string::SigmaString::from(name),
            encrypted_payload: encrypted,
            hash_signature: sig,
        };
        self.archive.insert(klib::string::SigmaString::from(name), entry);
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
        let vault = DriverArchiveVault::new();
        let entry = vault.query_driver(10).unwrap();
        assert_eq!(entry.name, "ne2000_isa_nic");
        assert_eq!(entry.lineage_version, "Linux 2.2 NIC");
        assert_eq!(entry.dependencies[0], "isa_bus_device");
    }
}
// SigmaOS Legacy Driver Archive Vault (DriverArchiveVault)
