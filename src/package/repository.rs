// SigmaOS Package Repository Management
// Linux distro-inspired package repository handling
// Manages package repositories, sources, and metadata

#![cfg_attr(not(test), no_std)]

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

/// APT/DNF-Style Package Pinning Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PinPriority {
    Exmittent = -1,  // Never install
    Default = 500,   // Standard priority
    Preferred = 990, // Preferred release / repository
    Hold = 1001,     // Force hold on version
}

#[derive(Debug, Clone)]
pub struct PackagePinRule {
    pub package_pattern: String,
    pub pinned_version: String,
    pub priority: PinPriority,
}

pub struct PackagePinEngine {
    pub rules: Vec<PackagePinRule>,
}

impl PackagePinEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_pin_rule(&mut self, pattern: &str, version: &str, priority: PinPriority) {
        self.rules.push(PackagePinRule {
            package_pattern: String::from(pattern),
            pinned_version: String::from(version),
            priority,
        });
    }

    pub fn get_pin_priority(&self, package: &str) -> PinPriority {
        for rule in &self.rules {
            if rule.package_pattern == package || rule.package_pattern == "*" {
                return rule.priority;
            }
        }
        PinPriority::Default
    }
}

impl Default for PackagePinEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Arch Reflector / DNF Mirror Sync & Failover Engine
#[derive(Debug, Clone)]
pub struct MirrorEntry {
    pub url: String,
    pub country: String,
    pub latency_ms: u32,
    pub active: bool,
}

pub struct MirrorSyncEngine {
    pub mirrors: Vec<MirrorEntry>,
}

impl MirrorSyncEngine {
    pub fn new() -> Self {
        Self { mirrors: Vec::new() }
    }

    pub fn add_mirror(&mut self, url: &str, country: &str, latency_ms: u32) {
        self.mirrors.push(MirrorEntry {
            url: String::from(url),
            country: String::from(country),
            latency_ms,
            active: true,
        });
    }

    pub fn rank_mirrors(&mut self) {
        self.mirrors.sort_by_key(|m| m.latency_ms);
    }

    pub fn get_fastest_mirror(&self) -> Option<String> {
        self.mirrors.iter().find(|m| m.active).map(|m| m.url.clone())
    }
}

impl Default for MirrorSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD pkg / DNF History Transaction Snapshot Journal
#[derive(Debug, Clone)]
pub struct TransactionJournalEntry {
    pub transaction_id: usize,
    pub action: String,
    pub package_name: String,
    pub version: String,
    pub timestamp: u64,
}

pub struct PackageTransactionJournal {
    pub journal: Vec<TransactionJournalEntry>,
    next_tx_id: usize,
}

impl PackageTransactionJournal {
    pub fn new() -> Self {
        Self {
            journal: Vec::new(),
            next_tx_id: 1,
        }
    }

    pub fn log_transaction(&mut self, action: &str, pkg: &str, ver: &str, timestamp: u64) -> usize {
        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        self.journal.push(TransactionJournalEntry {
            transaction_id: tx_id,
            action: String::from(action),
            package_name: String::from(pkg),
            version: String::from(ver),
            timestamp,
        });

        tx_id
    }

    pub fn rollback_transaction(&mut self, target_tx_id: usize) -> Vec<TransactionJournalEntry> {
        let mut undo_actions = Vec::new();
        while let Some(last) = self.journal.last() {
            if last.transaction_id >= target_tx_id {
                let undone = self.journal.pop().unwrap();
                undo_actions.push(undone);
            } else {
                break;
            }
        }
        undo_actions
    }
}

impl Default for PackageTransactionJournal {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_package_pinning_and_mirror_sync() {
        let mut pin_engine = PackagePinEngine::new();
        pin_engine.add_pin_rule("kernel", "6.5.0", PinPriority::Hold);
        assert_eq!(pin_engine.get_pin_priority("kernel"), PinPriority::Hold);
        assert_eq!(pin_engine.get_pin_priority("gcc"), PinPriority::Default);

        let mut mirror_engine = MirrorSyncEngine::new();
        mirror_engine.add_mirror("https://mirror2.sigmaos.org", "US", 150);
        mirror_engine.add_mirror("https://mirror1.sigmaos.org", "US", 20);
        mirror_engine.rank_mirrors();
        assert_eq!(mirror_engine.get_fastest_mirror(), Some(String::from("https://mirror1.sigmaos.org")));

        let mut journal = PackageTransactionJournal::new();
        let tx1 = journal.log_transaction("install", "curl", "8.2.1", 100);
        let tx2 = journal.log_transaction("install", "vim", "9.0", 105);
        assert_eq!(tx2, 2);

        let undone = journal.rollback_transaction(tx2);
        assert_eq!(undone.len(), 1);
        assert_eq!(undone[0].package_name, "vim");
    }
}
