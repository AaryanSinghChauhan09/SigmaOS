//! Package repository management following SigmaOS principles
//! 
//! Following principles: VERSION_CONTROL_PRACTICES, SINGLE_RESPONSIBILITY,
//! OPEN_CLOSED, and CODE_REUSABILITY.

use crate::sigpkg::models::{PackageMeta, PackageRepository, PackageName};

/// Package repository manager
pub struct PackageManager {
    /// Local repository
    local_repo: PackageRepository,
    /// Remote repository URL
    remote_url: String,
    /// Auto-update flag
    auto_update: bool,
}

impl PackageManager {
    /// Create new package manager
    pub fn new(remote_url: String, local_path: String) -> Self {
        let local_repo = PackageRepository::new(remote_url.clone(), local_path);
        PackageManager {
            local_repo,
            remote_url,
            auto_update: true,
        }
    }
    
    /// Install package
    pub fn install(&mut self, package_name: &str) -> Result<(), String> {
        let name = PackageName::parse_package_name(package_name)
            .map_err(|e| format!("Invalid package name: {}", e))?;
        
        if !self.local_repo.exists(&name) {
            // Try to download from remote
            // In real implementation, this would fetch from remote repository
            return Err(format!("Package not found locally or remotely: {}", package_name));
        }
        
        // Resolve dependencies
        if let Some(package) = self.local_repo.get_package(&name) {
            for dep in &package.dependencies {
                self.install(&dep.to_string())?;
            }
        }
        
        Ok(())
    }
    
    /// Remove package
    pub fn remove(&mut self, package_name: &str) -> Result<(), String> {
        let name = PackageName::parse_package_name(package_name)
            .map_err(|e| format!("Invalid package name: {}", e))?;
        
        if self.local_repo.exists(&name) {
            self.local_repo.remove_package(&name);
            Ok(())
        } else {
            Err(format!("Package not found: {}", package_name))
        }
    }
    
    /// Update package
    pub fn update(&mut self, package_name: &str) -> Result<(), String> {
        let name = PackageName::parse_package_name(package_name)
            .map_err(|e| format!("Invalid package name: {}", e))?;
        
        if self.local_repo.exists(&name) {
            // In real implementation, this would fetch latest version from remote
            Ok(())
        } else {
            Err(format!("Package not found: {}", package_name))
        }
    }
    
    /// List installed packages
    pub fn list_installed(&self) -> Vec<String> {
        self.local_repo
            .list_packages()
            .iter()
            .map(|p| format!("{}-{}", p.name.name, p.name.version))
            .collect()
    }
    
    /// Search packages
    pub fn search(&self, term: &str) -> Vec<PackageMeta> {
        self.local_repo
            .list_packages()
            .iter()
            .filter(|p| p.description.contains(term) || p.name.name.contains(term))
            .cloned()
            .collect()
    }
    
    /// Set auto-update
    pub fn set_auto_update(&mut self, auto_update: bool) {
        self.auto_update = auto_update;
    }
    
    /// Check auto-update status
    pub fn is_auto_update(&self) -> bool {
        self.auto_update
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_package_manager_new() {
        let pm = PackageManager::new("remote".to_string(), "local".to_string());
        assert!(pm.is_auto_update());
    }
    
    #[test]
    fn test_package_manager_list() {
        let mut pm = PackageManager::new("remote".to_string(), "local".to_string());
        
        // Add a test package
        let meta = crate::sigpkg::models::PackageMeta {
            name: crate::sigpkg::models::PackageName {
                name: "testpkg".to_string(),
                version: "1.0.0".to_string(),
            },
            description: "Test package".to_string(),
            version: "1.0.0".to_string(),
            maintainer: "Test".to_string(),
            license: "MIT".to_string(),
            repository: "remote".to_string(),
            dependencies: vec![],
            build_dependencies: vec![],
            checksums: std::collections::HashMap::new(),
            dependencies_list: vec![],
        };
        
        pm.local_repo.add_package(meta);
        
        let installed = pm.list_installed();
        assert!(installed.contains(&"testpkg-1.0.0".to_string()));
    }
    
    #[test]
    fn test_package_manager_search() {
        let mut pm = PackageManager::new("remote".to_string(), "local".to_string());
        
        let meta = crate::sigpkg::models::PackageMeta {
            name: crate::sigpkg::models::PackageName {
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
            checksums: std::collections::HashMap::new(),
            dependencies_list: vec![],
        };
        
        pm.local_repo.add_package(meta);
        
        let results = pm.search("sigma");
        assert!(results.len() >= 1);
        assert!(results.iter().any(|p| p.name.name == "sigmaos"));
    }
}
