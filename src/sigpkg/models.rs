//! Package model following SigmaOS principles
//!
//! Implements package metadata and dependency following
//! the sigpkg design with principles: CODE_REUSABILITY,
//! OPEN_CLOSED, SINGLE_RESPONSIBILITY, and VERSION_CONTROL_PRACTICES.

use std::collections::HashMap;

/// Package name and version
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageName {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMeta {
    /// Package name and version
    pub name: PackageName,
    /// Package description
    pub description: String,
    /// Package version
    pub version: String,
    /// Package maintainer
    pub maintainer: String,
    /// License type
    pub license: String,
    /// Repository URL
    pub repository: String,
    /// Dependency dependencies
    pub dependencies: Vec<PackageName>,
    /// Build dependencies
    pub build_dependencies: Vec<PackageName>,
    /// Checksums
    pub checksums: HashMap<String, String>,
    /// Dependencies
    pub dependencies_list: Vec<String>,
}

/// Package repository
#[derive(Debug, Clone)]
pub struct PackageRepository {
    /// Package database
    packages: HashMap<PackageName, PackageMeta>,
    /// Remote repository URL
    remote_url: String,
    /// Local path
    local_path: String,
}

impl PackageRepository {
    /// Create new package repository
    pub fn new(remote_url: String, local_path: String) -> Self {
        PackageRepository {
            packages: HashMap::new(),
            remote_url,
            local_path,
        }
    }
    
    /// Add package to repository
    pub fn add_package(&mut self, meta: PackageMeta) {
        let name = meta.name.clone();
        self.packages.insert(name, meta);
    }
    
    /// Get package by name
    pub fn get_package(&self, name: &PackageName) -> Option<&PackageMeta> {
        self.packages.get(name)
    }
    
    /// List all packages
    pub fn list_packages(&self) -> Vec<&PackageMeta> {
        self.packages.values().collect()
    }
    
    /// Remove package from repository
    pub fn remove_package(&mut self, name: &PackageName) {
        self.packages.remove(name);
    }
    
    /// Check if package exists
    pub fn exists(&self, name: &PackageName) -> bool {
        self.packages.contains_key(name)
    }
    
    /// Update package
    pub fn update_package(&mut self, meta: PackageMeta) {
        let name = meta.name.clone();
        self.packages.insert(name, meta);
    }
}

impl PackageRepository {
    /// Parse package name from string
    pub fn parse_package_name(name_str: &str) -> Result<PackageName, String> {
        let parts: Vec<&str> = name_str.split('-').take_while(|s| s != &"0" && s != &"1").collect();
        if parts.is_empty() {
            return Err("Invalid package name format".to_string());
        }
        
        let name = parts.join("-");
        // Version would be after the last dash with number
        let version_parts: Vec<&str> = name_str.split('-').rev().take(2).rev().collect();
        let version = if version_parts.len() >= 2 {
            version_parts[1].to_string()
        } else {
            "0.1.0".to_string()
        };
        
        let name_name = if version_parts.len() >= 1 {
            version_parts[0].to_string()
        } else {
            name_str.to_string()
        };
        
        Ok(PackageName {
            name: name_name,
            version,
        })
    }
    
    /// Resolve dependencies
    pub fn resolve_dependencies(&self, name: &PackageName) -> Result<Vec<PackageName>, String> {
        if let Some(package) = self.get_package(name) {
            Ok(package.dependencies.clone())
        } else {
            Err(format!("Package not found: {}", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_package_name_display() {
        let name = PackageName {
            name: "sigmaos".to_string(),
            version: "0.1.0".to_string(),
        };
        assert_eq!(format!("{}", name), "sigmaos-0.1.0");
    }
    
    #[test]
    fn test_package_repository() {
        let mut repo = PackageRepository::new("remote".to_string(), "local".to_string());
        
        let meta = PackageMeta {
            name: PackageName {
                name: "sigmaos".to_string(),
                version: "0.1.0".to_string(),
            },
            description: "SigmaOS operating system".to_string(),
            version: "0.1.0".to_string(),
            maintainer: "SigmaOS Team".to_string(),
            license: "MIT".to_string(),
            repository: "remote".to_string(),
            dependencies: vec![],
            build_dependencies: vec![],
            checksums: HashMap::new(),
            dependencies_list: vec![],
        };
        
        repo.add_package(meta);
        assert!(repo.exists(&PackageName {
            name: "sigmaos".to_string(),
            version: "0.1.0".to_string(),
        }));
        assert_eq!(repo.list_packages().len(), 1);
    }
    
    #[test]
    fn test_package_name_parsing() {
        let name = PackageName::parse_package_name("sigmaos-0.1.0");
        assert!(name.is_ok());
        let name = name.unwrap();
        assert_eq!(name.name, "sigmaos");
        assert_eq!(name.version, "0.1.0");
    }
}
