// SigmaOS Alpine Linux APK Implementation
// Implements Alpine Linux's APK package manager
// Inspired by Alpine's lightweight, security-focused package management

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// APK package
#[derive(Debug, Clone)]
pub struct ApkPackage {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub url: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub provides: Vec<String>,
    pub install_if: Vec<String>,
    pub size: u64,
}

/// APK repository
#[derive(Debug, Clone)]
pub struct ApkRepository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
}

/// APK world (system packages)
#[derive(Debug, Clone)]
pub struct ApkWorld {
    pub packages: Vec<String>,
    pub virtual_packages: BTreeMap<String, Vec<String>>,
}

/// APK package manager
pub struct ApkPackageManager {
    pub installed: BTreeMap<String, ApkPackage>,
    pub available: BTreeMap<String, ApkPackage>,
    pub repositories: Vec<ApkRepository>,
    pub world: ApkWorld,
}

impl ApkPackageManager {
    pub fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
            available: BTreeMap::new(),
            repositories: Vec::new(),
            world: ApkWorld {
                packages: Vec::new(),
                virtual_packages: BTreeMap::new(),
            },
        }
    }

    /// Add repository
    pub fn add_repository(&mut self, repo: ApkRepository) {
        self.repositories.push(repo);
    }

    /// Add package to available
    pub fn add_available(&mut self, package: ApkPackage) {
        self.available.insert(package.name.clone(), package);
    }

    /// Install package
    pub fn install(&mut self, package_name: &str) -> Result<(), String> {
        let package = self.available.get(package_name)
            .ok_or_else(|| format!("Package {} not found", package_name))?.clone();

        // Resolve dependencies
        let dependencies = self.resolve_dependencies(&package)?;

        // Install dependencies first
        for dep in &dependencies {
            if !self.installed.contains_key(dep) {
                self.install(dep)?;
            }
        }

        // Install package
        println!("Installing {} ({})", package_name, package.version);
        self.installed.insert(package_name.to_string(), package.clone());

        // Add to world
        if !self.world.packages.contains(&package_name.to_string()) {
            self.world.packages.push(package_name.to_string());
        }

        Ok(())
    }

    /// Remove package
    pub fn remove(&mut self, package_name: &str) -> Result<(), String> {
        if !self.installed.contains_key(package_name) {
            return Err(format!("Package {} not installed", package_name));
        }

        // Check for reverse dependencies
        let reverse_deps = self.find_reverse_dependencies(package_name);
        if !reverse_deps.is_empty() {
            return Err(format!("Package {} is required by: {:?}", package_name, reverse_deps));
        }

        println!("Removing {}", package_name);
        self.installed.remove(package_name);
        self.world.packages.retain(|p| p != package_name);

        Ok(())
    }

    /// Update package
    pub fn update(&mut self, package_name: &str) -> Result<(), String> {
        if let Some(current) = self.installed.get(package_name) {
            if let Some(available) = self.available.get(package_name) {
                if available.version != current.version {
                    println!("Updating {} from {} to {}", package_name, current.version, available.version);
                    self.remove(package_name)?;
                    self.install(package_name)?;
                }
            }
        }
        Ok(())
    }

    /// Update all packages
    pub fn upgrade(&mut self) -> Result<(), String> {
        let mut upgraded = 0;

        for (name, _) in self.installed.clone() {
            if let Some(available) = self.available.get(&name) {
                if let Some(current) = self.installed.get(&name) {
                    if available.version != current.version {
                        self.update(&name)?;
                        upgraded += 1;
                    }
                }
            }
        }

        println!("Upgraded {} packages", upgraded);
        Ok(())
    }

    /// Add to world
    pub fn add_to_world(&mut self, package_name: String) {
        if !self.world.packages.contains(&package_name) {
            self.world.packages.push(package_name);
        }
    }

    /// Add virtual package
    pub fn add_virtual(&mut self, virtual_name: String, providers: Vec<String>) {
        self.world.virtual_packages.insert(virtual_name, providers);
    }

    /// Resolve dependencies
    fn resolve_dependencies(&self, package: &ApkPackage) -> Result<Vec<String>, String> {
        let mut resolved = Vec::new();
        let mut to_resolve = package.dependencies.clone();

        while let Some(dep) = to_resolve.pop() {
            if !resolved.contains(&dep) {
                if self.available.contains_key(&dep) {
                    let dep_pkg = self.available.get(&dep).unwrap();
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
    fn find_reverse_dependencies(&self, package_name: &str) -> Vec<String> {
        self.installed.values()
            .filter(|pkg| pkg.dependencies.contains(&package_name.to_string()))
            .map(|pkg| pkg.name.clone())
            .collect()
    }

    /// Search packages
    pub fn search(&self, query: &str) -> Vec<&ApkPackage> {
        let query_lower = query.to_lowercase();
        self.available.values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower) ||
                pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get world packages
    pub fn get_world(&self) -> &Vec<String> {
        &self.world.packages
    }
}

impl Default for ApkPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apk_install() {
        let mut apk = ApkPackageManager::new();
        
        let package = ApkPackage {
            name: "musl".to_string(),
            version: "1.2.3".to_string(),
            architecture: "x86_64".to_string(),
            description: "musl libc".to_string(),
            url: "https://alpine.org".to_string(),
            license: "MIT".to_string(),
            dependencies: vec![],
            provides: vec![],
            install_if: vec![],
            size: 1024 * 1024,
        };
        
        apk.add_available(package);
        let result = apk.install("musl");
        assert!(result.is_ok());
        assert!(apk.installed.contains_key("musl"));
    }

    #[test]
    fn test_apk_search() {
        let mut apk = ApkPackageManager::new();
        
        let package = ApkPackage {
            name: "alpine-baselayout".to_string(),
            version: "3.0.0".to_string(),
            architecture: "x86_64".to_string(),
            description: "Alpine base layout".to_string(),
            url: "https://alpine.org".to_string(),
            license: "GPL".to_string(),
            dependencies: vec![],
            provides: vec![],
            install_if: vec![],
            size: 1024 * 1024,
        };
        
        apk.add_available(package);
        let results = apk.search("alpine");
        assert_eq!(results.len(), 1);
    }
}
