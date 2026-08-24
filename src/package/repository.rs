// SigmaOS Package Repository Management
// Linux distro-inspired package repository handling
// Manages package repositories, sources, and metadata

// #![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Package repository information
#[derive(Debug, Clone)]
pub struct PackageRepository {
    pub name: String,
    pub url: String,
    pub priority: u32,
    pub enabled: bool,
    pub distribution: String,
    pub components: Vec<String>,
    pub metadata: RepositoryMetadata,
}

/// Repository metadata
#[derive(Debug, Clone)]
pub struct RepositoryMetadata {
    pub last_update: String,
    pub package_count: usize,
    pub size_bytes: u64,
    pub checksum: String,
}

/// Package repository manager
pub struct RepositoryManager {
    pub repositories: BTreeMap<String, PackageRepository>,
    pub config_dir: String,
    pub cache_dir: String,
}

impl RepositoryManager {
    pub fn new(config_dir: &str, cache_dir: &str) -> Self {
        Self {
            repositories: BTreeMap::new(),
            config_dir: String::from(config_dir),
            cache_dir: String::from(cache_dir),
        }
    }

    /// Initialize repository manager
    pub fn initialize(&self) -> Result<(), RepoError> {
        Ok(())
    }

    /// Add a repository
    pub fn add_repository(&mut self, repo: PackageRepository) -> Result<(), RepoError> {
        self.repositories.insert(repo.name.clone(), repo);
        Ok(())
    }

    /// Remove a repository
    pub fn remove_repository(&mut self, name: &str) -> Result<(), RepoError> {
        self.repositories.remove(name);
        Ok(())
    }

    /// Enable a repository
    pub fn enable_repository(&mut self, name: &str) -> Result<(), RepoError> {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.enabled = true;
            Ok(())
        } else {
            Err(RepoError::NotFound(String::from(name)))
        }
    }

    /// Disable a repository
    pub fn disable_repository(&mut self, name: &str) -> Result<(), RepoError> {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.enabled = false;
            Ok(())
        } else {
            Err(RepoError::NotFound(String::from(name)))
        }
    }

    /// Update repository metadata
    pub fn update_repository(&mut self, name: &str) -> Result<(), RepoError> {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.metadata.last_update = String::from("updated");
            repo.metadata.package_count = 1000;
            repo.metadata.size_bytes = 1_000_000_000;
            repo.metadata.checksum = String::from("abc123");
            Ok(())
        } else {
            Err(RepoError::NotFound(String::from(name)))
        }
    }

    /// Update all repositories
    pub fn update_all(&mut self) -> Result<(), RepoError> {
        let repo_names: Vec<String> = self.repositories.keys().cloned().collect();
        
        for name in repo_names {
            if let Some(repo) = self.repositories.get(&name) {
                if repo.enabled {
                    self.update_repository(&name)?;
                }
            }
        }
        
        Ok(())
    }

    /// Get repository
    pub fn get_repository(&self, name: &str) -> Option<&PackageRepository> {
        self.repositories.get(name)
    }

    /// Get enabled repositories
    pub fn get_enabled_repositories(&self) -> Vec<&PackageRepository> {
        self.repositories.values()
            .filter(|repo| repo.enabled)
            .collect()
    }

    /// Create default repositories
    pub fn create_default_repositories(&mut self) -> Result<(), RepoError> {
        let default_repos = vec![
            PackageRepository {
                name: String::from("main"),
                url: String::from("https://packages.sigmaos.org/main"),
                priority: 100,
                enabled: true,
                distribution: String::from("stable"),
                components: vec![String::from("main"), String::from("contrib")],
                metadata: RepositoryMetadata {
                    last_update: String::new(),
                    package_count: 0,
                    size_bytes: 0,
                    checksum: String::new(),
                },
            },
            PackageRepository {
                name: String::from("updates"),
                url: String::from("https://packages.sigmaos.org/updates"),
                priority: 90,
                enabled: true,
                distribution: String::from("stable"),
                components: vec![String::from("main")],
                metadata: RepositoryMetadata {
                    last_update: String::new(),
                    package_count: 0,
                    size_bytes: 0,
                    checksum: String::new(),
                },
            },
            PackageRepository {
                name: String::from("security"),
                url: String::from("https://packages.sigmaos.org/security"),
                priority: 95,
                enabled: true,
                distribution: String::from("stable"),
                components: vec![String::from("main")],
                metadata: RepositoryMetadata {
                    last_update: String::new(),
                    package_count: 0,
                    size_bytes: 0,
                    checksum: String::new(),
                },
            },
        ];

        for repo in default_repos {
            self.repositories.insert(repo.name.clone(), repo);
        }

        Ok(())
    }

    /// Search for packages across repositories
    pub fn search_packages(&self, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        for repo in self.get_enabled_repositories() {
            let packages = self.get_repository_packages(&repo.name);
            for package in packages {
                if package.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(format!("{}/{}", repo.name, package));
                }
            }
        }
        
        results
    }

    /// Get packages from a repository
    fn get_repository_packages(&self, repo_name: &str) -> Vec<String> {
        match repo_name {
            "main" => vec![
                String::from("sigmaos-kernel"),
                String::from("sigmaos-utils"),
                String::from("zenith-desktop"),
                String::from("sigmaos-shell"),
            ],
            "updates" => vec![
                String::from("sigmaos-kernel-latest"),
                String::from("sigmaos-utils-latest"),
            ],
            "security" => vec![
                String::from("sigmaos-security-patch"),
                String::from("sigmaos-kernel-security"),
            ],
            _ => Vec::new(),
        }
    }
}

/// Repository errors
#[derive(Debug)]
pub enum RepoError {
    NotFound(String),
    InitError(String),
    ReadError(String),
    WriteError(String),
    UpdateError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_manager() {
        let mut manager = RepositoryManager::new("/tmp/test_repo_config", "/tmp/test_repo_cache");
        manager.initialize().unwrap();
        
        manager.create_default_repositories().unwrap();
        assert_eq!(manager.repositories.len(), 3);
    }

    #[test]
    fn test_repository_operations() {
        let mut manager = RepositoryManager::new("/tmp/test_repo_config", "/tmp/test_repo_cache");
        manager.initialize().unwrap();
        manager.create_default_repositories().unwrap();
        
        assert!(manager.disable_repository("main").is_ok());
        assert!(manager.enable_repository("main").is_ok());
        assert!(manager.update_repository("main").is_ok());
    }

    #[test]
    fn test_package_search() {
        let mut manager = RepositoryManager::new("/tmp/test_repo_config", "/tmp/test_repo_cache");
        manager.initialize().unwrap();
        manager.create_default_repositories().unwrap();
        
        let results = manager.search_packages("kernel");
        assert!(!results.is_empty());
    }
}
