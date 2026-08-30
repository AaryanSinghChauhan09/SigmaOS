extern crate alloc;
/// Repository Management System (Debian APT + Arch Pacman Inspiration)
/// Manages package repositories, mirrors, and metadata
use crate::klib::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use core::default::Default;
use core::option::Option::{self, Some, None};
use core::result::Result::{self, Ok, Err};
use crate::sigpkg::{Package, Version, VersionConstraint};

/// Repository configuration (Debian sources.list inspiration)
#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub components: Vec<String>,
    pub architectures: Vec<String>,
    pub trusted: bool,
    pub priority: u32,
}

impl Repository {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            components: Vec::new(),
            architectures: Vec::new(),
            trusted: false,
            priority: 100,
        }
    }

    pub fn add_component(&mut self, component: &str) {
        self.components.push(component.to_string());
    }

    pub fn add_architecture(&mut self, arch: &str) {
        self.architectures.push(arch.to_string());
    }
}

/// Repository manager (APT sources.list.d inspiration)
pub struct RepositoryManager {
    repositories: Vec<Repository>,
    mirrors: BTreeMap<String, Vec<String>>,
    current_mirror: BTreeMap<String, String>,
}

impl RepositoryManager {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
            mirrors: BTreeMap::new(),
            current_mirror: BTreeMap::new(),
        }
    }

    /// Add a repository (Arch pacman.conf inspiration)
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Add mirror for a repository (Arch mirrorlist inspiration)
    pub fn add_mirror(&mut self, repo_name: &str, mirror_url: &str) {
        if let Some(mirrors) = self.mirrors.get_mut(repo_name) {
            mirrors.push(mirror_url.to_string());
        } else {
            let mut mirrors = Vec::new();
            mirrors.push(mirror_url.to_string());
            self.mirrors.insert(repo_name.to_string(), mirrors);
        }
    }

    /// Select best mirror (Arch rankmirrors inspiration)
    pub fn select_best_mirror(&mut self, repo_name: &str) -> Result<String, String> {
        if let Some(mirrors) = self.mirrors.get(repo_name) {
            // Simple selection - in production would test latency
            if let Some(first) = mirrors.first() {
                self.current_mirror
                    .insert(repo_name.to_string(), first.clone());
                return Ok(first.clone());
            }
        }
        Err(format!(
            "No mirrors available for repository: {}",
            repo_name
        ))
    }

    /// Get repository URL with mirror substitution
    pub fn get_repository_url(&self, repo_name: &str) -> Result<String, String> {
        if let Some(mirror) = self.current_mirror.get(repo_name) {
            if let Some(repo) = self.repositories.iter().find(|r| r.name == repo_name) {
                return Ok(format!("{}/{}", mirror, repo.name));
            }
        }
        Err(format!("Repository not found: {}", repo_name))
    }

    /// Update repository metadata (APT update inspiration)
    pub fn update_metadata(&mut self) -> Result<(), String> {
        for repo in &self.repositories {
            // In production, would fetch and parse Packages/Sources files
            println!("Updating metadata for repository: {}", repo.name);
        }
        Ok(())
    }

    /// List all repositories
    pub fn list_repositories(&self) -> Vec<&Repository> {
        self.repositories.iter().collect()
    }
}

impl Default for RepositoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new("core", "https://repo.sigmaos.org");
        assert_eq!(repo.name, "core");
        assert_eq!(repo.url, "https://repo.sigmaos.org");
    }

    #[test]
    fn test_repository_manager() {
        let mut manager = RepositoryManager::new();
        let repo = Repository::new("core", "https://repo.sigmaos.org");
        manager.add_repository(repo);

        assert_eq!(manager.list_repositories().len(), 1);
    }

    #[test]
    fn test_mirror_selection() {
        let mut manager = RepositoryManager::new();
        manager.add_mirror("core", "https://mirror1.sigmaos.org");
        manager.add_mirror("core", "https://mirror2.sigmaos.org");

        let result = manager.select_best_mirror("core");
        assert!(result.is_ok());
    }
}
