// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaRepo - Package Repository Management
//!
//! SigmaRepo provides package repository management for SigmaOS, inspired by
//! Linux package managers like apt, pacman, and dnf.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Repository priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RepoPriority {
    /// Low priority
    Low = 0,
    /// Normal priority
    Normal = 50,
    /// High priority
    High = 100,
}

impl fmt::Display for RepoPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoPriority::Low => write!(f, "low"),
            RepoPriority::Normal => write!(f, "normal"),
            RepoPriority::High => write!(f, "high"),
        }
    }
}

/// Repository type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoType {
    /// Official repository
    Official,
    /// Community repository
    Community,
    /// Third-party repository
    ThirdParty,
    /// Local repository
    Local,
}

impl fmt::Display for RepoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoType::Official => write!(f, "official"),
            RepoType::Community => write!(f, "community"),
            RepoType::ThirdParty => write!(f, "third-party"),
            RepoType::Local => write!(f, "local"),
        }
    }
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct PackageDependency {
    /// Package name
    pub name: String,
    /// Version constraint (e.g., ">=1.0.0", "<2.0.0")
    pub version_constraint: Option<String>,
    /// Whether this is an optional dependency
    pub optional: bool,
}

impl PackageDependency {
    /// Create a new package dependency
    pub fn new(name: String) -> Self {
        Self {
            name,
            version_constraint: None,
            optional: false,
        }
    }

    /// Set version constraint
    pub fn with_version(mut self, constraint: String) -> Self {
        self.version_constraint = Some(constraint);
        self
    }

    /// Set as optional
    pub fn with_optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: String,
    /// Package architecture
    pub architecture: String,
    /// Package license
    pub license: String,
    /// Maintainer
    pub maintainer: String,
    /// Homepage URL
    pub homepage: Option<String>,
    /// Dependencies
    pub dependencies: Vec<PackageDependency>,
    /// Provides (virtual packages)
    pub provides: Vec<String>,
    /// Conflicts with
    pub conflicts: Vec<String>,
    /// Replaces
    pub replaces: Vec<String>,
    /// Download size (bytes)
    pub download_size: u64,
    /// Installed size (bytes)
    pub installed_size: u64,
    /// Checksum (SHA256)
    pub checksum: String,
    /// Build timestamp
    pub build_timestamp: u64,
}

impl PackageMetadata {
    /// Create new package metadata
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            architecture: "x86_64".to_string(),
            license: "MIT".to_string(),
            maintainer: String::new(),
            homepage: None,
            dependencies: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            download_size: 0,
            installed_size: 0,
            checksum: String::new(),
            build_timestamp: 0,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Set architecture
    pub fn with_architecture(mut self, arch: String) -> Self {
        self.architecture = arch;
        self
    }

    /// Set license
    pub fn with_license(mut self, license: String) -> Self {
        self.license = license;
        self
    }

    /// Set maintainer
    pub fn with_maintainer(mut self, maintainer: String) -> Self {
        self.maintainer = maintainer;
        self
    }

    /// Set homepage
    pub fn with_homepage(mut self, homepage: String) -> Self {
        self.homepage = Some(homepage);
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep: PackageDependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Set download size
    pub fn with_download_size(mut self, size: u64) -> Self {
        self.download_size = size;
        self
    }

    /// Set installed size
    pub fn with_installed_size(mut self, size: u64) -> Self {
        self.installed_size = size;
        self
    }

    /// Set checksum
    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = checksum;
        self
    }

    /// Set build timestamp
    pub fn with_build_timestamp(mut self, timestamp: u64) -> Self {
        self.build_timestamp = timestamp;
        self
    }
}

/// Repository
#[derive(Debug, Clone)]
pub struct Repository {
    /// Repository name
    pub name: String,
    /// Repository URL
    pub url: String,
    /// Repository type
    pub repo_type: RepoType,
    /// Repository priority
    pub priority: RepoPriority,
    /// Enabled flag
    pub enabled: bool,
    /// Last update timestamp
    pub last_update: Option<u64>,
    /// Package index (name -> metadata)
    pub packages: BTreeMap<String, PackageMetadata>,
}

impl Repository {
    /// Create a new repository
    pub fn new(name: String, url: String, repo_type: RepoType) -> Self {
        Self {
            name,
            url,
            repo_type,
            priority: RepoPriority::Normal,
            enabled: true,
            last_update: None,
            packages: BTreeMap::new(),
        }
    }

    /// Set priority
    pub fn with_priority(mut self, priority: RepoPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add package to repository
    pub fn add_package(&mut self, metadata: PackageMetadata) {
        self.packages.insert(metadata.name.clone(), metadata);
    }

    /// Remove package from repository
    pub fn remove_package(&mut self, name: &str) {
        self.packages.remove(name);
    }

    /// Get package metadata
    pub fn get_package(&self, name: &str) -> Option<&PackageMetadata> {
        self.packages.get(name)
    }

    /// Search packages by name (partial match)
    pub fn search_packages(&self, query: &str) -> Vec<&PackageMetadata> {
        self.packages
            .values()
            .filter(|pkg| pkg.name.contains(query))
            .collect()
    }

    /// Get all package names
    pub fn package_names(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

    /// Get package count
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    /// Update last update timestamp
    pub fn update_timestamp(&mut self, timestamp: u64) {
        self.last_update = Some(timestamp);
    }

    /// Check if repository needs update
    pub fn needs_update(&self, current_time: u64, interval_secs: u64) -> bool {
        match self.last_update {
            Some(last) => current_time - last > interval_secs,
            None => true,
        }
    }
}

/// Repository configuration
#[derive(Debug, Clone)]
pub struct RepoConfig {
    /// Update interval in seconds
    pub update_interval: u64,
    /// Maximum repositories
    pub max_repositories: usize,
    /// Cache directory
    pub cache_dir: String,
    /// Enable automatic updates
    pub auto_update: bool,
}

impl RepoConfig {
    /// Create new repository configuration
    pub fn new() -> Self {
        Self {
            update_interval: 86400, // 24 hours
            max_repositories: 100,
            cache_dir: "/var/cache/sigmarepo".to_string(),
            auto_update: false,
        }
    }

    /// Set update interval
    pub fn with_update_interval(mut self, interval: u64) -> Self {
        self.update_interval = interval;
        self
    }

    /// Set max repositories
    pub fn with_max_repositories(mut self, max: usize) -> Self {
        self.max_repositories = max;
        self
    }

    /// Set cache directory
    pub fn with_cache_dir(mut self, dir: String) -> Self {
        self.cache_dir = dir;
        self
    }

    /// Set auto update
    pub fn with_auto_update(mut self, auto: bool) -> Self {
        self.auto_update = auto;
        self
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaRepo - Package Repository Manager
pub struct SigmaRepo {
    /// Repository configuration
    config: RepoConfig,
    /// Repositories (name -> repository)
    repositories: BTreeMap<String, Repository>,
    /// Running state
    running: bool,
}

impl SigmaRepo {
    /// Create a new SigmaRepo instance
    pub fn new(config: RepoConfig) -> Self {
        Self {
            config,
            repositories: BTreeMap::new(),
            running: false,
        }
    }

    /// Create SigmaRepo with default configuration
    pub fn with_default_config() -> Self {
        Self::new(RepoConfig::default())
    }

    /// Start the repository manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the repository manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a repository
    pub fn add_repository(&mut self, repo: Repository) -> Result<(), String> {
        if self.repositories.len() >= self.config.max_repositories {
            return Err("Maximum repository limit reached".to_string());
        }
        self.repositories.insert(repo.name.clone(), repo);
        Ok(())
    }

    /// Remove a repository
    pub fn remove_repository(&mut self, name: &str) -> Result<(), String> {
        if self.repositories.remove(name).is_none() {
            return Err(format!("Repository '{}' not found", name));
        }
        Ok(())
    }

    /// Get a repository
    pub fn get_repository(&self, name: &str) -> Option<&Repository> {
        self.repositories.get(name)
    }

    /// Get a repository (mutable)
    pub fn get_repository_mut(&mut self, name: &str) -> Option<&mut Repository> {
        self.repositories.get_mut(name)
    }

    /// List all repositories
    pub fn list_repositories(&self) -> Vec<&Repository> {
        self.repositories.values().collect()
    }

    /// List enabled repositories
    pub fn list_enabled_repositories(&self) -> Vec<&Repository> {
        self.repositories
            .values()
            .filter(|repo| repo.enabled)
            .collect()
    }

    /// Enable a repository
    pub fn enable_repository(&mut self, name: &str) -> Result<(), String> {
        match self.repositories.get_mut(name) {
            Some(repo) => {
                repo.enabled = true;
                Ok(())
            }
            None => Err(format!("Repository '{}' not found", name)),
        }
    }

    /// Disable a repository
    pub fn disable_repository(&mut self, name: &str) -> Result<(), String> {
        match self.repositories.get_mut(name) {
            Some(repo) => {
                repo.enabled = false;
                Ok(())
            }
            None => Err(format!("Repository '{}' not found", name)),
        }
    }

    /// Search for a package across all repositories
    pub fn search_package(&self, name: &str) -> Vec<(&Repository, &PackageMetadata)> {
        let mut results = Vec::new();
        for repo in self.list_enabled_repositories() {
            if let Some(pkg) = repo.get_package(name) {
                results.push((repo, pkg));
            }
        }
        results
    }

    /// Search for packages by query across all repositories
    pub fn search_packages(&self, query: &str) -> Vec<(&Repository, &PackageMetadata)> {
        let mut results = Vec::new();
        for repo in self.list_enabled_repositories() {
            for pkg in repo.search_packages(query) {
                results.push((repo, pkg));
            }
        }
        results
    }

    /// Get package metadata by priority
    pub fn get_package_by_priority(&self, name: &str) -> Option<(&Repository, &PackageMetadata)> {
        let mut results: Vec<_> = self.search_package(name);
        // Sort by priority (highest first)
        results.sort_by(|a, b| b.0.priority.cmp(&a.0.priority));
        results.into_iter().next()
    }

    /// Update repository metadata
    pub fn update_repository(&mut self, name: &str, timestamp: u64) -> Result<(), String> {
        match self.repositories.get_mut(name) {
            Some(repo) => {
                // In a real implementation, this would fetch metadata from the repository URL
                repo.update_timestamp(timestamp);
                Ok(())
            }
            None => Err(format!("Repository '{}' not found", name)),
        }
    }

    /// Update all enabled repositories
    pub fn update_all(&mut self, timestamp: u64) -> Vec<String> {
        let mut errors = Vec::new();
        let enabled_names: Vec<String> = self
            .list_enabled_repositories()
            .into_iter()
            .map(|repo| repo.name.clone())
            .collect();
        for name in enabled_names {
            if let Err(e) = self.update_repository(&name, timestamp) {
                errors.push(e);
            }
        }
        errors
    }

    /// Get configuration
    pub fn config(&self) -> &RepoConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: RepoConfig) {
        self.config = config;
    }

    /// Get repository count
    pub fn repository_count(&self) -> usize {
        self.repositories.len()
    }

    /// Get total package count across all repositories
    pub fn total_package_count(&self) -> usize {
        self.repositories
            .values()
            .map(|repo| repo.package_count())
            .sum()
    }
}

impl Default for SigmaRepo {
    fn default() -> Self {
        Self::with_default_config()
    }
}

/// Global SigmaRepo instance (placeholder for actual global state)
static mut SIGMAREPO_INSTANCE: Option<SigmaRepo> = None;

/// Initialize the global SigmaRepo instance
pub fn init_sigmarepo(config: RepoConfig) {
    unsafe {
        SIGMAREPO_INSTANCE = Some(SigmaRepo::new(config));
    }
}

/// Initialize SigmaRepo with default configuration
pub fn init_sigmarepo_default() {
    init_sigmarepo(RepoConfig::default());
}

/// Get the global SigmaRepo instance
pub fn get_sigmarepo() -> Option<&'static mut SigmaRepo> {
    unsafe { SIGMAREPO_INSTANCE.as_mut() }
}

/// Convenience function to add a repository
pub fn add_repository(name: String, url: String, repo_type: RepoType) -> Result<(), String> {
    if let Some(repo) = get_sigmarepo() {
        let repository = Repository::new(name, url, repo_type);
        repo.add_repository(repository)
    } else {
        Err("SigmaRepo not initialized".to_string())
    }
}

/// Convenience function to search for a package
pub fn search_package(name: &str) -> Option<(&'static Repository, &'static PackageMetadata)> {
    if let Some(repo) = get_sigmarepo() {
        repo.get_package_by_priority(name)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_priority() {
        assert!(RepoPriority::High > RepoPriority::Normal);
        assert!(RepoPriority::Normal > RepoPriority::Low);
    }

    #[test]
    fn test_repo_type_display() {
        assert_eq!(RepoType::Official.to_string(), "official");
        assert_eq!(RepoType::Community.to_string(), "community");
    }

    #[test]
    fn test_package_dependency() {
        let dep = PackageDependency::new("test-package".to_string())
            .with_version(">=1.0.0".to_string())
            .with_optional(true);

        assert_eq!(dep.name, "test-package");
        assert_eq!(dep.version_constraint, Some(">=1.0.0".to_string()));
        assert!(dep.optional);
    }

    #[test]
    fn test_package_metadata() {
        let metadata = PackageMetadata::new("test-pkg".to_string(), "1.0.0".to_string())
            .with_description("Test package".to_string())
            .with_architecture("x86_64".to_string())
            .with_license("MIT".to_string())
            .with_maintainer("test@example.com".to_string())
            .with_download_size(1024)
            .with_installed_size(2048);

        assert_eq!(metadata.name, "test-pkg");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.description, "Test package");
        assert_eq!(metadata.download_size, 1024);
        assert_eq!(metadata.installed_size, 2048);
    }

    #[test]
    fn test_repository() {
        let mut repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        )
        .with_priority(RepoPriority::High)
        .with_enabled(true);

        let metadata = PackageMetadata::new("test-pkg".to_string(), "1.0.0".to_string());
        repo.add_package(metadata);

        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.priority, RepoPriority::High);
        assert!(repo.enabled);
        assert_eq!(repo.package_count(), 1);
        assert!(repo.get_package("test-pkg").is_some());
    }

    #[test]
    fn test_repository_search() {
        let mut repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        let metadata1 = PackageMetadata::new("test-pkg".to_string(), "1.0.0".to_string());
        let metadata2 = PackageMetadata::new("example-pkg".to_string(), "1.0.0".to_string());
        repo.add_package(metadata1);
        repo.add_package(metadata2);

        let results = repo.search_packages("test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test-pkg");
    }

    #[test]
    fn test_repository_needs_update() {
        let mut repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        assert!(repo.needs_update(1000, 3600));

        repo.update_timestamp(1000);
        assert!(!repo.needs_update(1500, 3600));
        assert!(repo.needs_update(5000, 3600));
    }

    #[test]
    fn test_sigmarepo() {
        let mut repo_mgr = SigmaRepo::with_default_config();
        assert!(!repo_mgr.is_running());
        repo_mgr.start();
        assert!(repo_mgr.is_running());
        repo_mgr.stop();
        assert!(!repo_mgr.is_running());
    }

    #[test]
    fn test_sigmarepo_add_repository() {
        let mut repo_mgr = SigmaRepo::with_default_config();
        repo_mgr.start();

        let repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        assert!(repo_mgr.add_repository(repo).is_ok());
        assert_eq!(repo_mgr.repository_count(), 1);
        assert!(repo_mgr.get_repository("test-repo").is_some());
    }

    #[test]
    fn test_sigmarepo_search() {
        let mut repo_mgr = SigmaRepo::with_default_config();
        repo_mgr.start();

        let mut repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        let metadata = PackageMetadata::new("test-pkg".to_string(), "1.0.0".to_string());
        repo.add_package(metadata);

        repo_mgr.add_repository(repo).unwrap();

        let results = repo_mgr.search_package("test-pkg");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.name, "test-pkg");
    }

    #[test]
    fn test_sigmarepo_enable_disable() {
        let mut repo_mgr = SigmaRepo::with_default_config();
        repo_mgr.start();

        let repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        repo_mgr.add_repository(repo).unwrap();
        assert!(repo_mgr.enable_repository("test-repo").is_ok());
        assert!(repo_mgr.disable_repository("test-repo").is_ok());
    }

    #[test]
    fn test_sigmarepo_priority() {
        let mut repo_mgr = SigmaRepo::with_default_config();
        repo_mgr.start();

        let mut repo1 = Repository::new(
            "repo1".to_string(),
            "https://example.com/repo1".to_string(),
            RepoType::Official,
        )
        .with_priority(RepoPriority::Low);

        let mut repo2 = Repository::new(
            "repo2".to_string(),
            "https://example.com/repo2".to_string(),
            RepoType::Official,
        )
        .with_priority(RepoPriority::High);

        let metadata1 = PackageMetadata::new("test-pkg".to_string(), "1.0.0".to_string());
        let metadata2 = PackageMetadata::new("test-pkg".to_string(), "2.0.0".to_string());

        repo1.add_package(metadata1);
        repo2.add_package(metadata2);

        repo_mgr.add_repository(repo1).unwrap();
        repo_mgr.add_repository(repo2).unwrap();

        let result = repo_mgr.get_package_by_priority("test-pkg");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0.name, "repo2"); // High priority
    }

    #[test]
    fn test_add_repository_convenience() {
        init_sigmarepo_default();

        let result = add_repository(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_search_package_convenience() {
        init_sigmarepo_default();

        let mut repo_mgr = get_sigmarepo().unwrap();
        let mut repo = Repository::new(
            "test-repo".to_string(),
            "https://example.com/repo".to_string(),
            RepoType::Official,
        );

        let metadata = PackageMetadata::new("test-pkg".to_string(), "1.0.0".to_string());
        repo.add_package(metadata);

        repo_mgr.add_repository(repo).unwrap();

        let result = search_package("test-pkg");
        assert!(result.is_some());
    }
}
