// Package Repository Configuration Manager for SigmaOS
// Implements repository configuration per Wiki 09-Packaging.md
// Supports multiple package repositories with priority and trust settings

use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Repository configuration entry
#[derive(Debug, Clone)]
pub struct RepoConfig {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
    pub gpg_verify: bool,
    pub gpg_key: Option<String>,
}

impl RepoConfig {
    pub fn new(name: String, url: String) -> Self {
        RepoConfig {
            name,
            url,
            enabled: true,
            priority: 100,
            gpg_verify: true,
            gpg_key: None,
        }
    }

    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_gpg_verify(&mut self, verify: bool) {
        self.gpg_verify = verify;
    }

    pub fn set_gpg_key(&mut self, key: String) {
        self.gpg_key = Some(key);
    }
}

/// Repository configuration manager
pub struct RepositoryConfigManager {
    pub repositories: HashMap<String, RepoConfig>,
    pub default_priority: u32,
}

impl RepositoryConfigManager {
    pub fn new() -> Self {
        let mut manager = RepositoryConfigManager {
            repositories: HashMap::new(),
            default_priority: 100,
        };
        manager.initialize_default_repositories();
        manager
    }

    fn initialize_default_repositories(&mut self) {
        // Official repository
        let mut official = RepoConfig::new(
            String::from("official"),
            String::from("https://packages.sigmaos.org"),
        );
        official.set_priority(1000);
        self.repositories.insert(String::from("official"), official);

        // Community repository
        let mut community = RepoConfig::new(
            String::from("community"),
            String::from("https://community.sigmaos.org"),
        );
        community.set_priority(500);
        self.repositories
            .insert(String::from("community"), community);

        // Testing repository (disabled by default)
        let mut testing = RepoConfig::new(
            String::from("testing"),
            String::from("https://testing.sigmaos.org"),
        );
        testing.set_enabled(false);
        testing.set_priority(100);
        self.repositories.insert(String::from("testing"), testing);
    }

    pub fn add_repository(&mut self, repo: RepoConfig) {
        self.repositories.insert(repo.name.clone(), repo);
    }

    pub fn remove_repository(&mut self, name: &str) -> bool {
        self.repositories.remove(name).is_some()
    }

    pub fn get_repository(&self, name: &str) -> Option<&RepoConfig> {
        self.repositories.get(name)
    }

    pub fn get_repository_mut(&mut self, name: &str) -> Option<&mut RepoConfig> {
        self.repositories.get_mut(name)
    }

    pub fn enable_repository(&mut self, name: &str) -> bool {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.set_enabled(true);
            true
        } else {
            false
        }
    }

    pub fn disable_repository(&mut self, name: &str) -> bool {
        if let Some(repo) = self.repositories.get_mut(name) {
            repo.set_enabled(false);
            true
        } else {
            false
        }
    }

    pub fn list_repositories(&self) -> Vec<String> {
        self.repositories.keys().cloned().collect()
    }

    pub fn list_enabled_repositories(&self) -> Vec<String> {
        self.repositories
            .iter()
            .filter(|(_, repo)| repo.enabled)
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn get_repositories_by_priority(&self) -> Vec<RepoConfig> {
        let mut repos: Vec<RepoConfig> = self
            .repositories
            .values()
            .filter(|repo| repo.enabled)
            .cloned()
            .collect();
        repos.sort_by(|a, b| b.priority.cmp(&a.priority));
        repos
    }
}

impl Default for RepositoryConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_config_manager_creation() {
        let manager = RepositoryConfigManager::new();
        assert!(manager.get_repository("official").is_some());
        assert!(manager.get_repository("community").is_some());
        assert!(manager.get_repository("testing").is_some());
    }

    #[test]
    fn test_add_repository() {
        let mut manager = RepositoryConfigManager::new();
        let custom = RepoConfig::new(
            String::from("custom"),
            String::from("https://custom.repo.org"),
        );
        manager.add_repository(custom);
        assert!(manager.get_repository("custom").is_some());
    }

    #[test]
    fn test_remove_repository() {
        let mut manager = RepositoryConfigManager::new();
        let success = manager.remove_repository("community");
        assert!(success);
        assert!(manager.get_repository("community").is_none());
    }

    #[test]
    fn test_enable_disable_repository() {
        let mut manager = RepositoryConfigManager::new();
        assert!(!manager.get_repository("testing").unwrap().enabled);

        manager.enable_repository("testing");
        assert!(manager.get_repository("testing").unwrap().enabled);

        manager.disable_repository("testing");
        assert!(!manager.get_repository("testing").unwrap().enabled);
    }

    #[test]
    fn test_list_repositories() {
        let manager = RepositoryConfigManager::new();
        let repos = manager.list_repositories();
        assert!(repos.contains(&String::from("official")));
        assert!(repos.contains(&String::from("community")));
        assert!(repos.contains(&String::from("testing")));
    }

    #[test]
    fn test_list_enabled_repositories() {
        let manager = RepositoryConfigManager::new();
        let enabled = manager.list_enabled_repositories();
        assert!(enabled.contains(&String::from("official")));
        assert!(enabled.contains(&String::from("community")));
        assert!(!enabled.contains(&String::from("testing")));
    }

    #[test]
    fn test_repositories_by_priority() {
        let manager = RepositoryConfigManager::new();
        let repos = manager.get_repositories_by_priority();
        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0].name, "official");
        assert_eq!(repos[1].name, "community");
    }

    #[test]
    fn test_repository_priority() {
        let mut manager = RepositoryConfigManager::new();
        let mut custom = RepoConfig::new(
            String::from("custom"),
            String::from("https://custom.repo.org"),
        );
        custom.set_priority(2000);
        manager.add_repository(custom);

        let repos = manager.get_repositories_by_priority();
        assert_eq!(repos[0].name, "custom");
        assert_eq!(repos[1].name, "official");
    }

    #[test]
    fn test_repository_gpg_settings() {
        let mut repo = RepoConfig::new(String::from("test"), String::from("https://test.repo.org"));
        assert!(repo.gpg_verify);

        repo.set_gpg_verify(false);
        assert!(!repo.gpg_verify);

        repo.set_gpg_key(String::from("test-key"));
        assert!(repo.gpg_key.is_some());
    }
}
