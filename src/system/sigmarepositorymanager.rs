// SPDX-License-Identifier: MIT
// SigmaOS - Repository Manager
// Repository management inspired by Arch (pacman repositories), Debian (apt sources), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Repository type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryType {
    Official,
    Community,
    ThirdParty,
    Local,
}

/// Repository
#[derive(Debug, Clone)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub url: String,
    pub repository_type: RepositoryType,
    pub enabled: bool,
    pub priority: u32,
}

/// Repository manager
pub struct SigmaRepositoryManager {
    repositories: Vec<Repository>,
}

impl SigmaRepositoryManager {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    pub fn add_repository(&mut self, repository: Repository) {
        self.repositories.push(repository);
    }

    pub fn get_repositories(&self) -> &[Repository] {
        &self.repositories
    }

    pub fn get_repository(&self, id: &str) -> Option<&Repository> {
        self.repositories.iter().find(|r| r.id == id)
    }

    pub fn get_repository_mut(&mut self, id: &str) -> Option<&mut Repository> {
        self.repositories.iter_mut().find(|r| r.id == id)
    }

    pub fn enable_repository(&mut self, id: String) -> Result<(), String> {
        if let Some(repo) = self.get_repository_mut(&id) {
            repo.enabled = true;
            Ok(())
        } else {
            Err(format!("Repository {} not found", id))
        }
    }

    pub fn disable_repository(&mut self, id: String) -> Result<(), String> {
        if let Some(repo) = self.get_repository_mut(&id) {
            repo.enabled = false;
            Ok(())
        } else {
            Err(format!("Repository {} not found", id))
        }
    }

    pub fn remove_repository(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.repositories.iter().position(|r| r.id == id) {
            self.repositories.remove(idx);
            Ok(())
        } else {
            Err(format!("Repository {} not found", id))
        }
    }

    pub fn get_enabled_repositories(&self) -> Vec<&Repository> {
        self.repositories.iter().filter(|r| r.enabled).collect()
    }

    pub fn get_repositories_by_type(&self, repository_type: RepositoryType) -> Vec<&Repository> {
        self.repositories.iter().filter(|r| r.repository_type == repository_type).collect()
    }
}

impl Default for SigmaRepositoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_manager_creation() {
        let manager = SigmaRepositoryManager::new();
        assert_eq!(manager.get_repositories().len(), 0);
    }

    #[test]
    fn test_add_repository() {
        let mut manager = SigmaRepositoryManager::new();
        let repo = Repository {
            id: "core".to_string(),
            name: "Core".to_string(),
            url: "https://repo.sigmaos.org/core".to_string(),
            repository_type: RepositoryType::Official,
            enabled: true,
            priority: 10,
        };
        manager.add_repository(repo);
        assert_eq!(manager.get_repositories().len(), 1);
    }

    #[test]
    fn test_enable_repository() {
        let mut manager = SigmaRepositoryManager::new();
        let repo = Repository {
            id: "core".to_string(),
            name: "Core".to_string(),
            url: "https://repo.sigmaos.org/core".to_string(),
            repository_type: RepositoryType::Official,
            enabled: false,
            priority: 10,
        };
        manager.add_repository(repo);
        manager.enable_repository("core".to_string()).unwrap();
        assert!(manager.get_repositories()[0].enabled);
    }

    #[test]
    fn test_disable_repository() {
        let mut manager = SigmaRepositoryManager::new();
        let repo = Repository {
            id: "core".to_string(),
            name: "Core".to_string(),
            url: "https://repo.sigmaos.org/core".to_string(),
            repository_type: RepositoryType::Official,
            enabled: true,
            priority: 10,
        };
        manager.add_repository(repo);
        manager.disable_repository("core".to_string()).unwrap();
        assert!(!manager.get_repositories()[0].enabled);
    }

    #[test]
    fn test_remove_repository() {
        let mut manager = SigmaRepositoryManager::new();
        let repo = Repository {
            id: "core".to_string(),
            name: "Core".to_string(),
            url: "https://repo.sigmaos.org/core".to_string(),
            repository_type: RepositoryType::Official,
            enabled: true,
            priority: 10,
        };
        manager.add_repository(repo);
        manager.remove_repository("core".to_string()).unwrap();
        assert_eq!(manager.get_repositories().len(), 0);
    }

    #[test]
    fn test_enabled_repositories() {
        let mut manager = SigmaRepositoryManager::new();
        let repo1 = Repository {
            id: "core".to_string(),
            name: "Core".to_string(),
            url: "https://repo.sigmaos.org/core".to_string(),
            repository_type: RepositoryType::Official,
            enabled: true,
            priority: 10,
        };
        let repo2 = Repository {
            id: "extra".to_string(),
            name: "Extra".to_string(),
            url: "https://repo.sigmaos.org/extra".to_string(),
            repository_type: RepositoryType::Official,
            enabled: false,
            priority: 10,
        };
        manager.add_repository(repo1);
        manager.add_repository(repo2);
        assert_eq!(manager.get_enabled_repositories().len(), 1);
    }

    #[test]
    fn test_repositories_by_type() {
        let mut manager = SigmaRepositoryManager::new();
        let repo1 = Repository {
            id: "core".to_string(),
            name: "Core".to_string(),
            url: "https://repo.sigmaos.org/core".to_string(),
            repository_type: RepositoryType::Official,
            enabled: true,
            priority: 10,
        };
        let repo2 = Repository {
            id: "community".to_string(),
            name: "Community".to_string(),
            url: "https://repo.sigmaos.org/community".to_string(),
            repository_type: RepositoryType::Community,
            enabled: true,
            priority: 10,
        };
        manager.add_repository(repo1);
        manager.add_repository(repo2);
        assert_eq!(manager.get_repositories_by_type(RepositoryType::Official).len(), 1);
    }
}
