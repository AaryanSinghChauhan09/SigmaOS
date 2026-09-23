use std::string::String;
use std::vec;
use std::vec::Vec;
// OpenStack Murano Inspired Application Catalogue for SigmaOS


/// Application Topology / Environment Requirements
#[derive(Debug, Clone)]
pub struct AppTopology {
    pub required_ram_mb: usize,
    pub required_cpu_cores: usize,
    pub required_storage_gb: usize,
    pub network_ports: Vec<u16>,
}

/// Murano Application Package
#[derive(Debug, Clone)]
pub struct ApplicationPackage {
    pub app_id: usize,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub author: String,
    pub topology: AppTopology,
    pub is_installed: bool,
}

/// Murano-style Application Catalogue Manager
pub struct MuranoApplicationCatalogueManager {
    pub catalogue: Vec<ApplicationPackage>,
    pub next_app_id: usize,
}

impl MuranoApplicationCatalogueManager {
    pub fn new() -> Self {
        let mut manager = MuranoApplicationCatalogueManager {
            catalogue: Vec::new(),
            next_app_id: 1,
        };

        // Seed default system application catalog packages
        manager.register_package(
            "SigmaOS Zenith Desktop",
            "2.4.0",
            "High-performance compositor desktop user environment",
            "Desktop",
            "SigmaOS Core Team",
            AppTopology {
                required_ram_mb: 512,
                required_cpu_cores: 1,
                required_storage_gb: 2,
                network_ports: vec![80, 443],
            },
        );

        manager.register_package(
            "PostgreSQL Database Server",
            "16.1",
            "Enterprise relational database engine",
            "Database",
            "PostgreSQL Global Development Group",
            AppTopology {
                required_ram_mb: 1024,
                required_cpu_cores: 2,
                required_storage_gb: 10,
                network_ports: vec![5432],
            },
        );

        manager
    }

    pub fn register_package(
        &mut self,
        name: &str,
        version: &str,
        description: &str,
        category: &str,
        author: &str,
        topology: AppTopology,
    ) -> usize {
        let id = self.next_app_id;
        self.next_app_id += 1;
        let pkg = ApplicationPackage {
            app_id: id,
            name: String::from(name),
            version: String::from(version),
            description: String::from(description),
            category: String::from(category),
            author: String::from(author),
            topology,
            is_installed: false,
        };
        self.catalogue.push(pkg);
        id
    }

    pub fn search_by_category(&self, category: &str) -> Vec<&ApplicationPackage> {
        self.catalogue
            .iter()
            .filter(|p| p.category.eq_ignore_ascii_case(category))
            .collect()
    }

    pub fn install_package(&mut self, app_id: usize) -> Result<(), &'static str> {
        if let Some(pkg) = self.catalogue.iter_mut().find(|p| p.app_id == app_id) {
            pkg.is_installed = true;
            Ok(())
        } else {
            Err("Application package not found in catalogue")
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_murano_catalogue() {
        let mut manager = MuranoApplicationCatalogueManager::new();
        let db_pkgs = manager.search_by_category("Database");
        assert_eq!(db_pkgs.len(), 1);
        let id = db_pkgs[0].app_id;

        manager.install_package(id).unwrap();
        let pkg = manager.catalogue.iter().find(|p| p.app_id == id).unwrap();
        assert!(pkg.is_installed);
    }
}
