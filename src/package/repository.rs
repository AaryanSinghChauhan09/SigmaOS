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

/// Repository errors
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RepoError {
    NotFound(String),
    InitError(String),
    ReadError(String),
    WriteError(String),
    UpdateError(String),
    PinError(String),
    MirrorError(String),
    TransactionError(String),
}

/// Package pinning priority rules inspired by APT (apt_preferences) and DNF priority plugins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PinPriority(pub i32);

impl PinPriority {
    pub const NEVER: PinPriority = PinPriority(-1);
    pub const AUTOMATIC: PinPriority = PinPriority(100);
    pub const DEFAULT: PinPriority = PinPriority(500);
    pub const PREFERRED: PinPriority = PinPriority(990);
    pub const FORCE: PinPriority = PinPriority(1001);
}

/// Dynamic Package Pinning Engine for package origin/version control.
#[derive(Debug, Clone)]
pub struct PackagePinRule {
    pub package_pattern: String,
    pub repo_origin: String,
    pub version_pattern: String,
    pub priority: PinPriority,
}

#[derive(Debug, Clone, Default)]
pub struct PackagePinEngine {
    pub rules: Vec<PackagePinRule>,
}

impl PackagePinEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: PackagePinRule) {
        self.rules.push(rule);
    }

    /// Determine calculated priority for a package from a specific repository
    pub fn evaluate_priority(&self, package_name: &str, repo_name: &str, version: &str) -> PinPriority {
        let mut highest_priority = PinPriority::DEFAULT;

        for rule in &self.rules {
            let pkg_match = rule.package_pattern == "*" || rule.package_pattern == package_name;
            let repo_match = rule.repo_origin == "*" || rule.repo_origin == repo_name;
            let ver_match = rule.version_pattern == "*" || rule.version_pattern == version;

            if pkg_match && repo_match && ver_match {
                if rule.priority > highest_priority || rule.priority == PinPriority::NEVER {
                    highest_priority = rule.priority;
                }
            }
        }

        highest_priority
    }
}

/// Mirror candidate with latency and reliability rating (Inspired by Arch Reflector & DNF fastestmirror)
#[derive(Debug, Clone)]
pub struct MirrorCandidate {
    pub url: String,
    pub latency_ms: u32,
    pub failure_count: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct MirrorSyncEngine {
    pub mirrors: Vec<MirrorCandidate>,
}

impl MirrorSyncEngine {
    pub fn new() -> Self {
        Self { mirrors: Vec::new() }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u32) {
        self.mirrors.push(MirrorCandidate {
            url: String::from(url),
            latency_ms,
            failure_count: 0,
            enabled: true,
        });
    }

    pub fn rank_mirrors(&mut self) {
        self.mirrors.sort_by(|a, b| {
            let score_a = (a.latency_ms as u64) + (a.failure_count as u64 * 500);
            let score_b = (b.latency_ms as u64) + (b.failure_count as u64 * 500);
            score_a.cmp(&score_b)
        });
    }

    pub fn mark_failure(&mut self, url: &str) {
        if let Some(mirror) = self.mirrors.iter_mut().find(|m| m.url == url) {
            mirror.failure_count += 1;
            if mirror.failure_count >= 3 {
                mirror.enabled = false;
            }
        }
        self.rank_mirrors();
    }

    pub fn get_best_mirror(&self) -> Result<String, RepoError> {
        self.mirrors
            .iter()
            .find(|m| m.enabled)
            .map(|m| m.url.clone())
            .ok_or_else(|| RepoError::MirrorError(String::from("No healthy mirror available")))
    }
}

/// Transactional Package History & Rollback (Inspired by DNF history & FreeBSD pkg rollback)
#[derive(Debug, Clone)]
pub enum TransactionAction {
    Install { package: String, version: String },
    Remove { package: String, version: String },
    Upgrade { package: String, old_version: String, new_version: String },
}

#[derive(Debug, Clone)]
pub struct PackageTransaction {
    pub id: u64,
    pub timestamp: String,
    pub actions: Vec<TransactionAction>,
    pub status_completed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PackageTransactionJournal {
    pub history: Vec<PackageTransaction>,
    pub current_id: u64,
}

impl PackageTransactionJournal {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_id: 1,
        }
    }

    pub fn record_transaction(&mut self, timestamp: &str, actions: Vec<TransactionAction>) -> u64 {
        let id = self.current_id;
        self.current_id += 1;
        self.history.push(PackageTransaction {
            id,
            timestamp: String::from(timestamp),
            actions,
            status_completed: true,
        });
        id
    }

    pub fn rollback_transaction(&mut self, transaction_id: u64) -> Result<Vec<TransactionAction>, RepoError> {
        let tx = self
            .history
            .iter()
            .find(|t| t.id == transaction_id)
            .ok_or_else(|| RepoError::TransactionError(format!("Transaction ID {} not found", transaction_id)))?;

        let mut rollback_actions = Vec::new();
        for action in tx.actions.iter().rev() {
            match action {
                TransactionAction::Install { package, version } => {
                    rollback_actions.push(TransactionAction::Remove {
                        package: package.clone(),
                        version: version.clone(),
                    });
                }
                TransactionAction::Remove { package, version } => {
                    rollback_actions.push(TransactionAction::Install {
                        package: package.clone(),
                        version: version.clone(),
                    });
                }
                TransactionAction::Upgrade { package, old_version, new_version } => {
                    rollback_actions.push(TransactionAction::Upgrade {
                        package: package.clone(),
                        old_version: new_version.clone(),
                        new_version: old_version.clone(),
                    });
                }
            }
        }

        Ok(rollback_actions)
    }
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
    fn test_package_pinning_rules() {
        let mut pin_engine = PackagePinEngine::new();
        pin_engine.add_rule(PackagePinRule {
            package_pattern: String::from("sigmaos-kernel"),
            repo_origin: String::from("security"),
            version_pattern: String::from("*"),
            priority: PinPriority::PREFERRED,
        });

        let p1 = pin_engine.evaluate_priority("sigmaos-kernel", "security", "1.0.0");
        let p2 = pin_engine.evaluate_priority("sigmaos-kernel", "main", "1.0.0");
        assert_eq!(p1, PinPriority::PREFERRED);
        assert_eq!(p2, PinPriority::DEFAULT);
    }

    #[test]
    fn test_mirror_sync_ranking_and_failover() {
        let mut sync_engine = MirrorSyncEngine::new();
        sync_engine.add_mirror("https://mirror2.sigmaos.org", 150);
        sync_engine.add_mirror("https://mirror1.sigmaos.org", 20);

        sync_engine.rank_mirrors();
        assert_eq!(sync_engine.get_best_mirror().unwrap(), "https://mirror1.sigmaos.org");

        // Fail mirror 1 thrice to trigger failover
        sync_engine.mark_failure("https://mirror1.sigmaos.org");
        sync_engine.mark_failure("https://mirror1.sigmaos.org");
        sync_engine.mark_failure("https://mirror1.sigmaos.org");

        assert_eq!(sync_engine.get_best_mirror().unwrap(), "https://mirror2.sigmaos.org");
    }

    #[test]
    fn test_package_transaction_rollback() {
        let mut journal = PackageTransactionJournal::new();
        let tx_id = journal.record_transaction("2026-03-30 12:00:00", vec![
            TransactionAction::Install {
                package: String::from("htop"),
                version: String::from("3.2.0"),
            },
            TransactionAction::Upgrade {
                package: String::from("bash"),
                old_version: String::from("5.1"),
                new_version: String::from("5.2"),
            },
        ]);

        let rollback = journal.rollback_transaction(tx_id).unwrap();
        assert_eq!(rollback.len(), 2);
        match &rollback[0] {
            TransactionAction::Upgrade { package, old_version, new_version } => {
                assert_eq!(package, "bash");
                assert_eq!(old_version, "5.2");
                assert_eq!(new_version, "5.1");
            }
            _ => panic!("Expected upgrade rollback"),
        }
        match &rollback[1] {
            TransactionAction::Remove { package, version } => {
                assert_eq!(package, "htop");
                assert_eq!(version, "3.2.0");
            }
            _ => panic!("Expected remove rollback"),
        }
    }
}
