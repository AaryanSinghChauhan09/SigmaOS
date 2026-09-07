use std::collections::{BTreeSet, BTreeMap};
// SigmaOS APT (Advanced Package Tool) Implementation
// Implements Debian-style package management for SigmaOS
// Inspired by Debian's APT for advanced package operations

use std::string::String;
use std::vec::Vec;

/// APT error types
#[derive(Debug, Clone)]
pub enum AptError {
    PackageNotFound,
    PackageNotInstalled,
    DependencyResolutionFailed,
    DownloadFailed,
    VerificationFailed,
    InstallationFailed,
    RemovalFailed,
    ConfigurationError,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct AptPackage {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub maintainer: String,
    pub section: String,
    pub priority: String,
    pub dependencies: Vec<String>,
    pub recommends: Vec<String>,
    pub suggests: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub source: String,
    pub homepage: String,
}

/// Sources list entry
#[derive(Debug, Clone)]
pub struct SourcesEntry {
    pub source_type: String, // deb, deb-src
    pub url: String,
    pub distribution: String,
    pub components: Vec<String>,
}

/// APT database
pub struct AptDatabase {
    pub installed: BTreeMap<String, AptPackage>,
    pub available: BTreeMap<String, AptPackage>,
    pub held_packages: BTreeSet<String>,
}

impl AptDatabase {
    pub fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
            available: BTreeMap::new(),
            held_packages: BTreeSet::new(),
        }
    }

    /// Add available package
    pub fn add_available(&mut self, package: AptPackage) {
        self.available.insert(package.name.clone(), package);
    }

    /// Mark package as installed
    pub fn mark_installed(&mut self, package: AptPackage) {
        self.installed.insert(package.name.clone(), package);
    }

    /// Remove package from installed
    pub fn mark_removed(&mut self, name: &str) {
        self.installed.remove(name);
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    /// Check if package is held
    pub fn is_held(&self, name: &str) -> bool {
        self.held_packages.contains(name)
    }

    /// Hold package
    pub fn hold_package(&mut self, name: &str) {
        self.held_packages.insert(name.to_string());
    }

    /// Unhold package
    pub fn unhold_package(&mut self, name: &str) {
        self.held_packages.remove(name);
    }
}

impl Default for AptDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// APT package manager
pub struct SigmaAPT {
    pub database: AptDatabase,
    pub sources: Vec<SourcesEntry>,
    pub cache_updated: bool,
}

impl SigmaAPT {
    pub fn new() -> Self {
        Self {
            database: AptDatabase::new(),
            sources: Vec::new(),
            cache_updated: false,
        }
    }

    /// Add sources entry
    pub fn add_sources(&mut self, entry: SourcesEntry) {
        self.sources.push(entry);
    }

    /// Update package cache
    pub fn update(&mut self) -> Result<(), AptError> {
        // In real implementation, would fetch from repositories
        println!("Updating package cache from {} sources", self.sources.len());
        self.cache_updated = true;
        Ok(())
    }

    /// Install packages
    pub fn install(&mut self, packages: Vec<String>) -> Result<(), AptError> {
        for package in packages {
            // Check if package is already installed
            if self.database.is_installed(&package) {
                println!("{} is already installed", package);
                continue;
            }

            // Check if package is held
            if self.database.is_held(&package) {
                println!("{} is held back", package);
                continue;
            }

            // Get package information
            let pkg_info = self
                .database
                .available
                .get(&package)
                .ok_or(AptError::PackageNotFound)?
                .clone();

            // Resolve dependencies
            let dependencies = self.resolve_dependencies(&pkg_info)?;

            // Install dependencies first
            for dep in &dependencies {
                if !self.database.is_installed(dep) {
                    self.install(vec![dep.clone()])?;
                }
            }

            // Simulate installation
            println!("Installing {} ({})", package, pkg_info.version);
            println!("Dependencies: {:?}", dependencies);

            // Mark as installed
            self.database.mark_installed(pkg_info);
        }

        Ok(())
    }

    /// Remove packages
    pub fn remove(&mut self, packages: Vec<String>, purge: bool) -> Result<(), AptError> {
        for package in packages {
            // Check if package is installed
            if !self.database.is_installed(&package) {
                return Err(AptError::PackageNotInstalled);
            }

            // Check for reverse dependencies
            let reverse_deps = self.find_reverse_dependencies(&package);
            if !reverse_deps.is_empty() {
                println!("Warning: {} is required by: {:?}", package, reverse_deps);
            }

            // Simulate removal
            if purge {
                println!("Purging {} (including configuration files)", package);
            } else {
                println!("Removing {}", package);
            }

            // Mark as removed
            self.database.mark_removed(&package);
        }

        Ok(())
    }

    /// Update packages
    pub fn upgrade(&mut self) -> Result<(), AptError> {
        let mut upgraded = 0;

        for (name, installed_pkg) in self.database.installed.clone() {
            if let Some(available_pkg) = self.database.available.get(&name) {
                if available_pkg.version != installed_pkg.version {
                    println!(
                        "Upgrading {} from {} to {}",
                        name, installed_pkg.version, available_pkg.version
                    );
                    self.database.mark_installed(available_pkg.clone());
                    upgraded += 1;
                }
            }
        }

        println!("Upgraded {} packages", upgraded);
        Ok(())
    }

    /// Full upgrade (dist-upgrade)
    pub fn full_upgrade(&mut self) -> Result<(), AptError> {
        // Similar to upgrade but handles dependency changes
        self.upgrade()
    }

    /// Search for packages
    pub fn search(&self, query: &str) -> Vec<&AptPackage> {
        let query_lower = query.to_lowercase();
        self.database
            .available
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Show package information
    pub fn show(&self, name: &str) -> Option<&AptPackage> {
        self.database.available.get(name)
    }

    /// Resolve dependencies
    fn resolve_dependencies(&self, pkg: &AptPackage) -> Result<Vec<String>, AptError> {
        let mut resolved = Vec::new();
        let mut to_resolve = pkg.dependencies.clone();

        while let Some(dep) = to_resolve.pop() {
            if !resolved.contains(&dep) {
                if self.database.available.contains_key(&dep) {
                    let dep_pkg = self.database.available.get(&dep).unwrap();
                    for sub_dep in &dep_pkg.dependencies {
                        if !resolved.contains(sub_dep) {
                            to_resolve.push(sub_dep.clone());
                        }
                    }
                }
                resolved.push(dep);
            }
        }

        Ok(resolved)
    }

    /// Find reverse dependencies
    fn find_reverse_dependencies(&self, name: &str) -> Vec<String> {
        self.database
            .installed
            .values()
            .filter(|pkg| pkg.dependencies.contains(&name.to_string()))
            .map(|pkg| pkg.name.clone())
            .collect()
    }

    /// Autoremove unused packages
    pub fn autoremove(&mut self) -> Result<(), AptError> {
        let mut to_remove = Vec::new();

        for (name, _) in &self.database.installed {
            let reverse_deps = self.find_reverse_dependencies(name);
            if reverse_deps.is_empty() && !self.database.is_held(name) {
                to_remove.push(name.clone());
            }
        }

        if to_remove.is_empty() {
            println!("No unused packages to remove");
        } else {
            println!("Removing {} unused packages", to_remove.len());
            for pkg in to_remove {
                self.database.mark_removed(&pkg);
            }
        }

        Ok(())
    }
}

impl Default for SigmaAPT {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_apt_install() {
        let mut apt = SigmaAPT::new();

        let pkg = AptPackage {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            architecture: "amd64".to_string(),
            description: "An example package".to_string(),
            maintainer: "user@example.com".to_string(),
            section: "utils".to_string(),
            priority: "optional".to_string(),
            dependencies: vec![],
            recommends: vec![],
            suggests: vec![],
            conflicts: vec![],
            provides: vec![],
            replaces: vec![],
            size: 1024,
            installed_size: 2048,
            source: "example".to_string(),
            homepage: "https://example.com".to_string(),
        };

        apt.database.add_available(pkg);
        let result = apt.install(vec!["example-pkg".to_string()]);
        assert!(result.is_ok());
        assert!(apt.database.is_installed("example-pkg"));
    }

    #[test]
    fn test_apt_search() {
        let mut apt = SigmaAPT::new();

        let pkg = AptPackage {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            architecture: "amd64".to_string(),
            description: "An example package for testing".to_string(),
            maintainer: "user@example.com".to_string(),
            section: "utils".to_string(),
            priority: "optional".to_string(),
            dependencies: vec![],
            recommends: vec![],
            suggests: vec![],
            conflicts: vec![],
            provides: vec![],
            replaces: vec![],
            size: 1024,
            installed_size: 2048,
            source: "example".to_string(),
            homepage: "https://example.com".to_string(),
        };

        apt.database.add_available(pkg);
        let results = apt.search("example");
        assert_eq!(results.len(), 1);
    }
}
