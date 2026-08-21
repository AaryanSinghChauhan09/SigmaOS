// SPDX-License-Identifier: Apache-2.0
// SigmaOS Debian APT & DPKG Package Management Compatibility Utility (sigma_apt_compat)
// Clean-room representation of Debian's core package utility suite (apt, apt-get, apt-cache, dpkg)

use std::collections::HashMap;

/// Debian-style package priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebianPackagePriority {
    Optional = 0,
    Standard = 1,
    Important = 2,
    Required = 3,
    Essential = 4, // Cannot be removed without breaking the system
}

#[derive(Debug, Clone)]
pub struct DebianPackageRecord {
    pub name: String,
    pub version: String,
    pub priority: DebianPackagePriority,
    pub dependencies: Vec<String>,
    pub description: String,
}

pub struct DebianAptSystem {
    pub repository_index: HashMap<String, DebianPackageRecord>,
    pub installed_packages: HashMap<String, DebianPackageRecord>,
    pub sources_synced: bool,
}

impl DebianAptSystem {
    pub fn new() -> Self {
        let mut sys = Self {
            repository_index: HashMap::new(),
            installed_packages: HashMap::new(),
            sources_synced: false,
        };
        sys.load_default_debian_repo();
        sys
    }

    /// Preloads standard Debian equivalent core packages
    fn load_default_debian_repo(&mut self) {
        self.repository_index.insert(
            "libc6".to_string(),
            DebianPackageRecord {
                name: "libc6".to_string(),
                version: "2.36-9".to_string(),
                priority: DebianPackagePriority::Essential,
                dependencies: Vec::new(),
                description: "GNU C Library: Shared libraries".to_string(),
            },
        );

        self.repository_index.insert(
            "bash".to_string(),
            DebianPackageRecord {
                name: "bash".to_string(),
                version: "5.2.15-2".to_string(),
                priority: DebianPackagePriority::Essential,
                dependencies: vec!["libc6".to_string()],
                description: "GNU Bourne Again SHell".to_string(),
            },
        );

        self.repository_index.insert(
            "curl".to_string(),
            DebianPackageRecord {
                name: "curl".to_string(),
                version: "7.88.1-10".to_string(),
                priority: DebianPackagePriority::Standard,
                dependencies: vec!["libc6".to_string(), "libssl3".to_string()],
                description: "command line tool for transferring data with URLs".to_string(),
            },
        );

        self.repository_index.insert(
            "libssl3".to_string(),
            DebianPackageRecord {
                name: "libssl3".to_string(),
                version: "3.0.8-1".to_string(),
                priority: DebianPackagePriority::Required,
                dependencies: vec!["libc6".to_string()],
                description: "Secure Sockets Layer toolkit - shared libraries".to_string(),
            },
        );
    }

    /// Simulates 'apt-get update' or 'apt update'
    pub fn apt_update(&mut self) -> Result<usize, &'static str> {
        println!("[apt] Get:1 http://deb.debian.org/debian bookworm InRelease");
        println!("[apt] Get:2 http://deb.debian.org/debian bookworm-updates InRelease");
        println!("[apt] Reading package lists... Done.");
        self.sources_synced = true;
        Ok(self.repository_index.len())
    }

    /// Simulates 'apt-cache search'
    pub fn apt_cache_search(&self, query: &str) -> Vec<DebianPackageRecord> {
        let mut results = Vec::new();
        for pkg in self.repository_index.values() {
            if pkg.name.contains(query) || pkg.description.contains(query) {
                results.push(pkg.clone());
            }
        }
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }

    /// Simulates 'apt-get install <pkg>' or 'apt install <pkg>'
    pub fn apt_install(&mut self, package_name: &str) -> Result<Vec<String>, &'static str> {
        if !self.sources_synced {
            return Err("APT Error: Package lists must be updated first (run apt update).");
        }

        if !self.repository_index.contains_key(package_name) {
            return Err("APT Error: Package not found in repository index.");
        }

        let mut installation_order = Vec::new();
        let mut visited = HashMap::new();

        self.resolve_dependencies(package_name, &mut installation_order, &mut visited)?;

        for pkg_name in &installation_order {
            if let Some(record) = self.repository_index.get(pkg_name) {
                self.installed_packages.insert(pkg_name.clone(), record.clone());
                println!("[apt] Selecting previously unselected package {}.", pkg_name);
                println!("[apt] Preparing to unpack {}_amd64.deb ...", pkg_name);
                println!("[apt] Setting up {} ({})...", pkg_name, record.version);
            }
        }

        Ok(installation_order)
    }

    /// Helper topological resolver
    fn resolve_dependencies(
        &self,
        name: &str,
        order: &mut Vec<String>,
        visited: &mut HashMap<String, bool>,
    ) -> Result<(), &'static str> {
        if let Some(&in_progress) = visited.get(name) {
            if in_progress {
                return Err("APT Error: Circular dependency detected.");
            }
            return Ok(());
        }

        if self.installed_packages.contains_key(name) {
            return Ok(());
        }

        visited.insert(name.to_string(), true);

        if let Some(record) = self.repository_index.get(name) {
            for dep in &record.dependencies {
                self.resolve_dependencies(dep, order, visited)?;
            }
        }

        visited.insert(name.to_string(), false);
        if !order.contains(&name.to_string()) {
            order.push(name.to_string());
        }

        Ok(())
    }

    /// Simulates 'apt-get remove <pkg>' or 'apt remove <pkg>'
    /// Enforces Debian's Essential package protection guidelines
    pub fn apt_remove(&mut self, package_name: &str) -> Result<(), &'static str> {
        if !self.installed_packages.contains_key(package_name) {
            return Err("APT Error: Package is not installed.");
        }

        if let Some(record) = self.installed_packages.get(package_name) {
            if record.priority == DebianPackagePriority::Essential {
                return Err("APT Error: Protected Essential system package cannot be removed. Removal aborted to prevent system bricking.");
            }
        }

        self.installed_packages.remove(package_name);
        println!("[apt] Removing {} ...", package_name);
        Ok(())
    }

    /// Simulates 'dpkg -i <deb_file>' legacy installation
    pub fn dpkg_install_file(&mut self, package_record: DebianPackageRecord) -> Result<(), &'static str> {
        println!("[dpkg] Unpacking legacy package {} ({}) ...", package_record.name, package_record.version);
        self.installed_packages.insert(package_record.name.clone(), package_record);
        Ok(())
    }
}

impl Default for DebianAptSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("Σ SIGMAOS DEBIAN APT PARITY SUITE");
    let mut apt = DebianAptSystem::new();
    apt.apt_update().unwrap();
    apt.apt_install("curl").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_update_and_search() {
        let mut apt = DebianAptSystem::new();
        assert!(!apt.sources_synced);

        let count = apt.apt_update().unwrap();
        assert_eq!(count, 4);
        assert!(apt.sources_synced);

        let results = apt.apt_cache_search("libraries");
        assert_eq!(results.len(), 2); // libc6, libssl3
        assert_eq!(results[0].name, "libc6");
    }

    #[test]
    fn test_apt_topological_install() {
        let mut apt = DebianAptSystem::new();
        // Fails if sources are not synced yet
        assert!(apt.apt_install("curl").is_err());

        apt.apt_update().unwrap();
        let install_order = apt.apt_install("curl").unwrap();

        // Dependencies: curl -> [libc6, libssl3] (libssl3 -> libc6)
        assert_eq!(install_order, vec!["libc6".to_string(), "libssl3".to_string(), "curl".to_string()]);
        assert!(apt.installed_packages.contains_key("curl"));
        assert!(apt.installed_packages.contains_key("libssl3"));
        assert!(apt.installed_packages.contains_key("libc6"));
    }

    #[test]
    fn test_apt_remove_essential_protection() {
        let mut apt = DebianAptSystem::new();
        apt.apt_update().unwrap();

        // Install essential and standard packages
        apt.apt_install("bash").unwrap();
        apt.apt_install("curl").unwrap();

        // Standard package can be removed
        assert!(apt.apt_remove("curl").is_ok());
        assert!(!apt.installed_packages.contains_key("curl"));

        // Essential package CANNOT be removed!
        let remove_res = apt.apt_remove("bash");
        assert!(remove_res.is_err());
        assert_eq!(remove_res.unwrap_err(), "APT Error: Protected Essential system package cannot be removed. Removal aborted to prevent system bricking.");
        assert!(apt.installed_packages.contains_key("bash")); // Remains installed
    }

    #[test]
    fn test_dpkg_install() {
        let mut apt = DebianAptSystem::new();
        let mock_deb = DebianPackageRecord {
            name: "nano".to_string(),
            version: "7.2-1".to_string(),
            priority: DebianPackagePriority::Optional,
            dependencies: Vec::new(),
            description: "small, friendly text editor".to_string(),
        };

        assert!(apt.dpkg_install_file(mock_deb).is_ok());
        assert!(apt.installed_packages.contains_key("nano"));
        assert_eq!(apt.installed_packages.get("nano").unwrap().version, "7.2-1");
    }
}
