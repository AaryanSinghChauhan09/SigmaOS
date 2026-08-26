// SPDX-License-Identifier: MIT
// SigmaOS Debian/Ubuntu APT Compatibility Engine
// Implements APT package management, DEB package parsing, and dpkg compatibility

extern crate alloc;
use crate::klib::collections::HashMap;
use alloc::string::String;
use alloc::vec::Vec;

/// DEB package metadata structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebPackage {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Vec<String>,
    pub pre_depends: Vec<String>,
    pub recommends: Vec<String>,
    pub suggests: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub installed_size: u64,
    pub section: String,
    pub priority: String,
}

/// APT repository configuration
#[derive(Debug, Clone)]
pub struct AptRepository {
    pub name: String,
    pub url: String,
    pub distribution: String,
    pub components: Vec<String>,
    pub enabled: bool,
}

/// APT package source
#[derive(Debug, Clone)]
pub struct AptSource {
    pub binary_url: String,
    pub source_url: String,
    pub suite: String,
    pub component: String,
}

/// DEB control file parser
pub struct DebControlParser {
    current_field: String,
    current_value: String,
    in_continuation: bool,
}

impl DebControlParser {
    pub fn new() -> Self {
        Self {
            current_field: String::new(),
            current_value: String::new(),
            in_continuation: false,
        }
    }

    /// Parse DEB control file content
    pub fn parse_control(&mut self, control_content: &str) -> Result<DebPackage, String> {
        let mut package = DebPackage {
            package: String::new(),
            version: String::new(),
            architecture: String::new(),
            maintainer: String::new(),
            description: String::new(),
            depends: Vec::new(),
            pre_depends: Vec::new(),
            recommends: Vec::new(),
            suggests: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            installed_size: 0,
            section: String::new(),
            priority: String::new(),
        };

        for line in control_content.lines() {
            if line.starts_with(' ') || line.starts_with('\t') {
                // Continuation line
                if self.in_continuation {
                    self.current_value.push_str(line.trim());
                }
            } else if line.contains(':') {
                // New field
                if !self.current_field.is_empty() {
                    self.process_field(&mut package);
                }

                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    self.current_field = parts[0].trim().to_lowercase();
                    self.current_value = parts[1].trim().to_string();
                    self.in_continuation = true;
                }
            } else if line.is_empty() {
                // Empty line - process accumulated field
                if !self.current_field.is_empty() {
                    self.process_field(&mut package);
                    self.current_field.clear();
                    self.current_value.clear();
                    self.in_continuation = false;
                }
            }
        }

        // Process last field
        if !self.current_field.is_empty() {
            self.process_field(&mut package);
        }

        Ok(package)
    }

    fn process_field(&self, package: &mut DebPackage) {
        match self.current_field.as_str() {
            "package" => package.package = self.current_value.clone(),
            "version" => package.version = self.current_value.clone(),
            "architecture" => package.architecture = self.current_value.clone(),
            "maintainer" => package.maintainer = self.current_value.clone(),
            "description" => package.description = self.current_value.clone(),
            "depends" => {
                package.depends = self.parse_dependencies(&self.current_value);
            }
            "pre-depends" => {
                package.pre_depends = self.parse_dependencies(&self.current_value);
            }
            "recommends" => {
                package.recommends = self.parse_dependencies(&self.current_value);
            }
            "suggests" => {
                package.suggests = self.parse_dependencies(&self.current_value);
            }
            "conflicts" => {
                package.conflicts = self.parse_dependencies(&self.current_value);
            }
            "provides" => {
                package.provides = self.parse_dependencies(&self.current_value);
            }
            "installed-size" => {
                package.installed_size = self.current_value.parse::<u64>().unwrap_or(0);
            }
            "section" => package.section = self.current_value.clone(),
            "priority" => package.priority = self.current_value.clone(),
            _ => {} // Ignore unknown fields
        }
    }

    fn parse_dependencies(&self, dep_string: &str) -> Vec<String> {
        dep_string
            .split(',')
            .map(|s| s.trim().split_whitespace().next().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

/// APT repository manager
pub struct AptRepositoryManager {
    repositories: Vec<AptRepository>,
    sources: Vec<AptSource>,
}

impl AptRepositoryManager {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
            sources: Vec::new(),
        }
    }

    /// Add an APT repository
    pub fn add_repository(&mut self, repo: AptRepository) {
        self.repositories.push(repo);
    }

    /// Parse sources.list format
    pub fn parse_sources_list(&mut self, sources_content: &str) -> Result<(), String> {
        for line in sources_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "deb" {
                let source = AptSource {
                    binary_url: parts[1].to_string(),
                    source_url: parts[1].to_string(),
                    suite: parts[2].to_string(),
                    component: parts[3].to_string(),
                };
                self.sources.push(source);
            }
        }
        Ok(())
    }

    /// Get package sources for a given package
    pub fn get_package_sources(&self, package_name: &str) -> Vec<AptSource> {
        self.sources
            .iter()
            .filter(|source| {
                // In production, this would check if the package is available in this source
                true
            })
            .cloned()
            .collect()
    }
}

/// dpkg-like package database
pub struct DpkgDatabase {
    installed_packages: HashMap<String, DebPackage>,
    status_database: HashMap<String, String>,
}

impl DpkgDatabase {
    pub fn new() -> Self {
        Self {
            installed_packages: HashMap::new(),
            status_database: HashMap::new(),
        }
    }

    /// Install a package
    pub fn install_package(&mut self, package: DebPackage) -> Result<(), String> {
        let pkg_name = package.package.clone();
        self.installed_packages
            .insert(pkg_name.clone(), package);
        self.status_database
            .insert(pkg_name, "install ok installed".to_string());
        Ok(())
    }

    /// Remove a package
    pub fn remove_package(&mut self, package_name: &str) -> Result<(), String> {
        self.installed_packages.remove(package_name);
        self.status_database.remove(package_name);
        Ok(())
    }

    /// Get package status
    pub fn get_package_status(&self, package_name: &str) -> Option<&str> {
        self.status_database.get(package_name).map(|s| s.as_str())
    }

    /// Get installed package
    pub fn get_package(&self, package_name: &str) -> Option<&DebPackage> {
        self.installed_packages.get(package_name)
    }

    /// List all installed packages
    pub fn list_installed(&self) -> Vec<&DebPackage> {
        self.installed_packages.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deb_control_parser() {
        let control_content = r#"Package: nginx
Version: 1.18.0-0ubuntu1
Architecture: amd64
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Description: A high performance web server and a reverse proxy server
 Depends: libc6 (>= 2.28), libssl1.1 (>= 1.1.1)
Installed-Size: 1024
Section: web
Priority: optional"#;

        let mut parser = DebControlParser::new();
        let package = parser.parse_control(control_content).unwrap();

        assert_eq!(package.package, "nginx");
        assert_eq!(package.version, "1.18.0-0ubuntu1");
        assert_eq!(package.architecture, "amd64");
        assert_eq!(package.section, "web");
        assert_eq!(package.priority, "optional");
    }

    #[test]
    fn test_dependency_parsing() {
        let mut parser = DebControlParser::new();
        let deps = parser.parse_dependencies("libc6 (>= 2.28), libssl1.1 (>= 1.1.1), zlib1g");

        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0], "libc6");
        assert_eq!(deps[1], "libssl1.1");
        assert_eq!(deps[2], "zlib1g");
    }

    #[test]
    fn test_apt_repository_manager() {
        let mut manager = AptRepositoryManager::new();
        let sources = "deb https://archive.ubuntu.com/ubuntu/ focal main restricted";

        manager.parse_sources_list(sources).unwrap();
        assert_eq!(manager.sources.len(), 1);
        assert_eq!(manager.sources[0].suite, "focal");
        assert_eq!(manager.sources[0].component, "main");
    }

    #[test]
    fn test_dpkg_database() {
        let mut db = DpkgDatabase::new();

        let package = DebPackage {
            package: "test-pkg".to_string(),
            version: "1.0.0".to_string(),
            architecture: "amd64".to_string(),
            maintainer: "Test".to_string(),
            description: "Test package".to_string(),
            depends: Vec::new(),
            pre_depends: Vec::new(),
            recommends: Vec::new(),
            suggests: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            installed_size: 1024,
            section: "test".to_string(),
            priority: "optional".to_string(),
        };

        db.install_package(package).unwrap();
        assert_eq!(
            db.get_package_status("test-pkg"),
            Some("install ok installed")
        );
        assert_eq!(db.list_installed().len(), 1);

        db.remove_package("test-pkg").unwrap();
        assert_eq!(db.get_package_status("test-pkg"), None);
    }
}
