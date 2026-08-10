// SigmaOS Legacy Driver Archive Vault (DriverArchiveVault)
// Stores legacy drivers in secure vault entries with lineage metadata and dependency chains

use std::collections::HashMap;

pub struct VaultEntry {
    pub id: usize,
    pub name: String,
    pub lineage_version: String,
    pub dependencies: Vec<String>,
}

pub struct DriverArchiveVault {
    pub vault: HashMap<usize, VaultEntry>,
}

impl DriverArchiveVault {
    pub fn new() -> Self {
        let mut archive = DriverArchiveVault {
            vault: HashMap::new(),
        };
        // Seed default driver vault entries
        archive.register_driver(10, "ne2000_isa_nic".to_string(), "Linux 2.2 NIC".to_string(), vec!["isa_bus_device".to_string()]);
        archive.register_driver(11, "ide_piix4_controller".to_string(), "Linux 2.4 IDE".to_string(), vec!["pci_express_bus".to_string()]);
        archive
    }

    pub fn register_driver(&mut self, id: usize, name: String, lineage: String, deps: Vec<String>) {
        self.vault.insert(id, VaultEntry {
            id,
            name,
            lineage_version: lineage,
            dependencies: deps,
        });
    }

    pub fn query_driver(&self, id: usize) -> Option<&VaultEntry> {
        self.vault.get(&id)
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
