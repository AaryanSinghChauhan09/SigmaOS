// SigmaOS AUR (Arch User Repository) Implementation
// Implements AUR-like package system for SigmaOS
// Inspired by Arch Linux's AUR for community-driven packages

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// AUR package metadata
#[derive(Debug, Clone)]
pub struct AURPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub dependencies: Vec<String>,
    pub makedepends: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
    pub pkgbuild: String,
    pub keywords: Vec<String>,
    pub license: Vec<String>,
    pub maintainers: Vec<String>,
    pub votes: u32,
    pub popularity: f32,
}

/// PKGBUILD structure
#[derive(Debug, Clone, Default)]
pub struct PKGBUILD {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub pkgdesc: String,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
    pub prepare: Option<String>,
    pub build: Option<String>,
    pub check: Option<String>,
    pub package: Option<String>,
}

/// Build error types
#[derive(Debug, Clone)]
pub enum BuildError {
    PackageNotFound,
    DependencyResolutionFailed,
    DownloadFailed,
    ChecksumVerificationFailed,
    BuildFailed,
    PackagingFailed,
}

/// AUR database and build system
pub struct SigmaAUR {
    pub package_db: BTreeMap<String, AURPackage>,
    pub build_scripts: BTreeMap<String, PKGBUILD>,
    pub installed_packages: BTreeMap<String, String>,
}

impl SigmaAUR {
    pub fn new() -> Self {
        Self {
            package_db: BTreeMap::new(),
            build_scripts: BTreeMap::new(),
            installed_packages: BTreeMap::new(),
        }
    }

    /// Add package to database
    pub fn add_package(&mut self, package: AURPackage) {
        self.package_db.insert(package.name.clone(), package);
    }

    /// Search for packages
    pub fn search(&self, query: &str) -> Vec<&AURPackage> {
        let query_lower = query.to_lowercase();
        self.package_db.values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower) ||
                pkg.description.to_lowercase().contains(&query_lower) ||
                pkg.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get package info
    pub fn get_package(&self, name: &str) -> Option<&AURPackage> {
        self.package_db.get(name)
    }

    /// Resolve dependencies
    pub fn resolve_dependencies(&self, pkg: &AURPackage) -> Result<Vec<String>, BuildError> {
        let mut resolved = Vec::new();
        let mut to_resolve = pkg.dependencies.clone();

        while let Some(dep) = to_resolve.pop() {
            if !resolved.contains(&dep) {
                // Check if dependency is in AUR
                if self.package_db.contains_key(&dep) {
                    let dep_pkg = self.package_db.get(&dep).unwrap();
                    // Add sub-dependencies
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

    /// Install package
    pub fn install_package(&mut self, pkg_name: &str) -> Result<(), BuildError> {
        let pkg = self.package_db.get(pkg_name)
            .ok_or(BuildError::PackageNotFound)?;

        let pkg_name_str = pkg.name.clone();
        let pkg_ver_str = pkg.version.clone();

        // Resolve dependencies
        let dependencies = self.resolve_dependencies(pkg)?;

        // Install dependencies first
        for dep in &dependencies {
            if !self.installed_packages.contains_key(dep) {
                self.install_package(dep)?;
            }
        }

        // Simulate build process
        println!("Building package: {}", pkg_name_str);
        println!("Version: {}", pkg_ver_str);
        println!("Dependencies: {:?}", dependencies);

        // Mark as installed
        self.installed_packages.insert(pkg_name.to_string(), pkg_ver_str);

        Ok(())
    }

    /// Update package
    pub fn update_package(&mut self, pkg_name: &str) -> Result<(), BuildError> {
        let pkg = self.package_db.get(pkg_name)
            .ok_or(BuildError::PackageNotFound)?;

        let current_version = self.installed_packages.get(pkg_name);

        if current_version.is_none() {
            return Err(BuildError::PackageNotFound);
        }

        if current_version.unwrap() != &pkg.version {
            println!("Updating {} from {} to {}", pkg_name, current_version.unwrap(), pkg.version);
            self.install_package(pkg_name)?;
        }

        Ok(())
    }

    /// Get popular packages
    pub fn get_popular_packages(&self, limit: usize) -> Vec<&AURPackage> {
        let mut packages: Vec<&AURPackage> = self.package_db.values().collect();
        packages.sort_by(|a, b| b.popularity.partial_cmp(&a.popularity).unwrap());
        packages.truncate(limit);
        packages
    }

    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<(&String, &String)> {
        self.installed_packages.iter().collect()
    }
}

impl Default for SigmaAUR {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aur_search() {
        let mut aur = SigmaAUR::new();

        let pkg = AURPackage {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            description: "An example package".to_string(),
            url: "https://example.com".to_string(),
            dependencies: vec![],
            makedepends: vec![],
            source: vec![],
            sha256sums: vec![],
            pkgbuild: String::new(),
            keywords: vec!["example".to_string(), "test".to_string()],
            license: vec!["MIT".to_string()],
            maintainers: vec!["user".to_string()],
            votes: 100,
            popularity: 5.0,
        };

        aur.add_package(pkg);
        let results = aur.search("example");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_aur_install() {
        let mut aur = SigmaAUR::new();

        let pkg = AURPackage {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            description: "An example package".to_string(),
            url: "https://example.com".to_string(),
            dependencies: vec!["dep1".to_string()],
            makedepends: vec![],
            source: vec![],
            sha256sums: vec![],
            pkgbuild: String::new(),
            keywords: vec![],
            license: vec![],
            maintainers: vec![],
            votes: 0,
            popularity: 0.0,
        };

        let dep = AURPackage {
            name: "dep1".to_string(),
            version: "1.0.0".to_string(),
            description: "A dependency".to_string(),
            url: "https://example.com".to_string(),
            dependencies: vec![],
            makedepends: vec![],
            source: vec![],
            sha256sums: vec![],
            pkgbuild: String::new(),
            keywords: vec![],
            license: vec![],
            maintainers: vec![],
            votes: 0,
            popularity: 0.0,
        };

        aur.add_package(pkg);
        aur.add_package(dep);

        let result = aur.install_package("example-pkg");
        assert!(result.is_ok());
        assert!(aur.installed_packages.contains_key("example-pkg"));
    }
}
