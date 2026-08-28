// SigmaOS Package Repository Management
// Linux & BSD distro-inspired package repository handling
// Manages package repositories, sources, APT/DNF-style pinning, signature verification,
// rankmirrors/reflector mirror failover, drpm delta indexes, and Nix/Guix content-addressed stores.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// Repository release channels inspired by Debian / Fedora / Arch Linux
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RepositoryChannel {
    Stable,
    Security,
    Updates,
    Testing,
    Unstable,
    Experimental,
}

impl RepositoryChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RepositoryChannel::Stable => "stable",
            RepositoryChannel::Security => "security",
            RepositoryChannel::Updates => "updates",
            RepositoryChannel::Testing => "testing",
            RepositoryChannel::Unstable => "unstable",
            RepositoryChannel::Experimental => "experimental",
        }
    }
}

/// APT / DNF style priority pinning rule
#[derive(Debug, Clone)]
pub struct PinRule {
    pub package_pattern: String,
    pub channel_filter: Option<RepositoryChannel>,
    pub version_prefix: Option<String>,
    pub priority: u32,
}

impl PinRule {
    pub fn matches(&self, package_name: &str, channel: RepositoryChannel, version: &str) -> bool {
        let name_match = self.package_pattern == "*" || package_name.contains(&self.package_pattern);
        let channel_match = self.channel_filter.map_or(true, |c| c == channel);
        let version_match = self.version_prefix.as_ref().map_or(true, |v| version.starts_with(v));
        name_match && channel_match && version_match
    }
}

/// Cryptographic Signature Verifier for Repository Index Manifests (Ed25519 & Dilithium5)
#[derive(Debug, Clone)]
pub struct RepositorySignatureVerifier {
    pub ed25519_public_key: [u8; 32],
    pub dilithium5_public_key: [u8; 64], // Simulated post-quantum public key
}

impl RepositorySignatureVerifier {
    pub fn new(ed25519_key: [u8; 32], dilithium5_key: [u8; 64]) -> Self {
        Self {
            ed25519_public_key: ed25519_key,
            dilithium5_public_key: dilithium5_key,
        }
    }

    /// Verifies index signature against index checksum and public keys
    pub fn verify_index_signature(&self, index_sha256: &[u8; 32], signature: &[u8]) -> bool {
        if signature.len() < 32 {
            return false;
        }
        // Simulated signature check: signature must incorporate index_sha256 XOR ed25519_public_key[0..16]
        let mut expected = [0u8; 16];
        for i in 0..16 {
            expected[i] = index_sha256[i] ^ self.ed25519_public_key[i];
        }
        signature[..16] == expected
    }
}

/// Arch Linux rankmirrors / Reflector inspired mirror health tracker and failover engine
#[derive(Debug, Clone)]
pub struct MirrorHealthTracker {
    pub mirror_url: String,
    pub region: String,
    pub latency_ms: u32,
    pub failure_count: u32,
    pub is_active: bool,
}

impl MirrorHealthTracker {
    pub fn new(url: &str, region: &str, latency_ms: u32) -> Self {
        Self {
            mirror_url: url.to_string(),
            region: region.to_string(),
            latency_ms,
            failure_count: 0,
            is_active: true,
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        if self.failure_count >= 3 {
            self.is_active = false;
        }
    }

    pub fn record_success(&mut self, latency_ms: u32) {
        self.latency_ms = latency_ms;
        self.failure_count = 0;
        self.is_active = true;
    }

    pub fn score(&self) -> u32 {
        if !self.is_active {
            return u32::MAX;
        }
        self.latency_ms + (self.failure_count * 1000)
    }
}

/// Fedora drpm / Arch delta-inspired differential package update descriptor
#[derive(Debug, Clone)]
pub struct DeltaPackageDescriptor {
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub delta_url: String,
    pub delta_size_bytes: u64,
    pub full_size_bytes: u64,
    pub delta_sha256: [u8; 32],
}

/// Delta repository index
#[derive(Debug, Clone)]
pub struct DeltaRepositoryIndex {
    pub deltas: BTreeMap<String, Vec<DeltaPackageDescriptor>>, // package_name -> deltas
}

impl DeltaRepositoryIndex {
    pub fn new() -> Self {
        Self {
            deltas: BTreeMap::new(),
        }
    }

    pub fn register_delta(&mut self, delta: DeltaPackageDescriptor) {
        self.deltas
            .entry(delta.package_name.clone())
            .or_insert_with(Vec::new)
            .push(delta);
    }

    pub fn find_delta(
        &self,
        package_name: &str,
        old_version: &str,
        new_version: &str,
    ) -> Option<&DeltaPackageDescriptor> {
        self.deltas.get(package_name)?.iter().find(|d| {
            d.old_version == old_version && d.new_version == new_version
        })
    }
}

impl Default for DeltaRepositoryIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Nix / Guix content-addressed package store index for zero-duplication package retrieval
#[derive(Debug, Clone)]
pub struct ContentAddressedRepoIndex {
    pub hash_to_package: BTreeMap<String, String>, // sha256_store_hash -> package_spec
}

impl ContentAddressedRepoIndex {
    pub fn new() -> Self {
        Self {
            hash_to_package: BTreeMap::new(),
        }
    }

    pub fn register_content_address(&mut self, store_hash: &str, package_spec: &str) {
        self.hash_to_package
            .insert(store_hash.to_string(), package_spec.to_string());
    }

    pub fn lookup_hash(&self, store_hash: &str) -> Option<&String> {
        self.hash_to_package.get(store_hash)
    }
}

impl Default for ContentAddressedRepoIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Package repository information
#[derive(Debug, Clone)]
pub struct PackageRepository {
    pub name: String,
    pub url: String,
    pub priority: u32,
    pub enabled: bool,
    pub channel: RepositoryChannel,
    pub distribution: String,
    pub components: Vec<String>,
    pub metadata: RepositoryMetadata,
    pub mirrors: Vec<MirrorHealthTracker>,
    pub verifier: Option<RepositorySignatureVerifier>,
}

/// Repository metadata
#[derive(Debug, Clone)]
pub struct RepositoryMetadata {
    pub last_update: String,
    pub package_count: usize,
    pub size_bytes: u64,
    pub checksum: String,
}

/// Package repository manager with APT/DNF pinning, Rankmirrors, Drpm Deltas, and Content-Addressing
pub struct RepositoryManager {
    pub repositories: BTreeMap<String, PackageRepository>,
    pub pin_rules: Vec<PinRule>,
    pub delta_index: DeltaRepositoryIndex,
    pub content_index: ContentAddressedRepoIndex,
    pub config_dir: String,
    pub cache_dir: String,
}

impl RepositoryManager {
    pub fn new(config_dir: &str, cache_dir: &str) -> Self {
        Self {
            repositories: BTreeMap::new(),
            pin_rules: Vec::new(),
            delta_index: DeltaRepositoryIndex::new(),
            content_index: ContentAddressedRepoIndex::new(),
            config_dir: String::from(config_dir),
            cache_dir: String::from(cache_dir),
        }
    }

    /// Initialize repository manager
    pub fn initialize(&self) -> Result<(), RepoError> {
        Ok(())
    }

    /// Add a pin rule
    pub fn add_pin_rule(&mut self, rule: PinRule) {
        self.pin_rules.push(rule);
    }

    /// Evaluates pin priority for a given package, channel, and version
    pub fn evaluate_pin_priority(&self, package_name: &str, channel: RepositoryChannel, version: &str) -> u32 {
        let mut highest_priority = match channel {
            RepositoryChannel::Security => 990,
            RepositoryChannel::Updates => 900,
            RepositoryChannel::Stable => 500,
            RepositoryChannel::Testing => 100,
            RepositoryChannel::Unstable => 50,
            RepositoryChannel::Experimental => 1,
        };

        for rule in &self.pin_rules {
            if rule.matches(package_name, channel, version) {
                if rule.priority > highest_priority {
                    highest_priority = rule.priority;
                }
            }
        }

        highest_priority
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

    /// Get best available mirror URL for a repository with automatic health-based failover
    pub fn select_best_mirror_url(&mut self, repo_name: &str) -> Result<String, RepoError> {
        let repo = self
            .repositories
            .get_mut(repo_name)
            .ok_or_else(|| RepoError::NotFound(String::from(repo_name)))?;

        if repo.mirrors.is_empty() {
            return Ok(repo.url.clone());
        }

        // Rank mirrors by score
        repo.mirrors.sort_by_key(|m| m.score());

        if let Some(best) = repo.mirrors.iter().find(|m| m.is_active) {
            Ok(best.mirror_url.clone())
        } else {
            // All mirrors failed, fallback to main repo URL
            Ok(repo.url.clone())
        }
    }

    /// Get repository
    pub fn get_repository(&self, name: &str) -> Option<&PackageRepository> {
        self.repositories.get(name)
    }

    /// Get enabled repositories
    pub fn get_enabled_repositories(&self) -> Vec<&PackageRepository> {
        self.repositories
            .values()
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
                channel: RepositoryChannel::Stable,
                distribution: String::from("stable"),
                components: vec![String::from("main"), String::from("contrib")],
                metadata: RepositoryMetadata {
                    last_update: String::new(),
                    package_count: 0,
                    size_bytes: 0,
                    checksum: String::new(),
                },
                mirrors: Vec::new(),
                verifier: None,
            },
            PackageRepository {
                name: String::from("updates"),
                url: String::from("https://packages.sigmaos.org/updates"),
                priority: 90,
                enabled: true,
                channel: RepositoryChannel::Updates,
                distribution: String::from("stable"),
                components: vec![String::from("main")],
                metadata: RepositoryMetadata {
                    last_update: String::new(),
                    package_count: 0,
                    size_bytes: 0,
                    checksum: String::new(),
                },
                mirrors: Vec::new(),
                verifier: None,
            },
            PackageRepository {
                name: String::from("security"),
                url: String::from("https://packages.sigmaos.org/security"),
                priority: 95,
                enabled: true,
                channel: RepositoryChannel::Security,
                distribution: String::from("stable"),
                components: vec![String::from("main")],
                metadata: RepositoryMetadata {
                    last_update: String::new(),
                    package_count: 0,
                    size_bytes: 0,
                    checksum: String::new(),
                },
                mirrors: Vec::new(),
                verifier: None,
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
    SignatureError(String),
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
    fn test_pin_rule_evaluation() {
        let mut manager = RepositoryManager::new("/tmp/test_repo_config", "/tmp/test_repo_cache");
        manager.add_pin_rule(PinRule {
            package_pattern: String::from("sigmaos-kernel"),
            channel_filter: Some(RepositoryChannel::Security),
            version_prefix: None,
            priority: 1000,
        });

        let sec_priority = manager.evaluate_pin_priority(
            "sigmaos-kernel",
            RepositoryChannel::Security,
            "6.8.0",
        );
        assert_eq!(sec_priority, 1000);

        let stable_priority = manager.evaluate_pin_priority(
            "sigmaos-kernel",
            RepositoryChannel::Stable,
            "6.8.0",
        );
        assert_eq!(stable_priority, 500);
    }

    #[test]
    fn test_mirror_ranking_and_failover() {
        let mut manager = RepositoryManager::new("/tmp/test_repo_config", "/tmp/test_repo_cache");
        let mut repo = PackageRepository {
            name: String::from("core"),
            url: String::from("https://packages.sigmaos.org/core"),
            priority: 100,
            enabled: true,
            channel: RepositoryChannel::Stable,
            distribution: String::from("stable"),
            components: vec![String::from("main")],
            metadata: RepositoryMetadata {
                last_update: String::new(),
                package_count: 0,
                size_bytes: 0,
                checksum: String::new(),
            },
            mirrors: vec![
                MirrorHealthTracker::new("https://slow.mirror.org/core", "US", 250),
                MirrorHealthTracker::new("https://fast.mirror.org/core", "EU", 20),
            ],
            verifier: None,
        };

        // Record 3 failures on slow mirror to trigger disablement
        repo.mirrors[0].record_failure();
        repo.mirrors[0].record_failure();
        repo.mirrors[0].record_failure();

        manager.add_repository(repo).unwrap();

        let best_url = manager.select_best_mirror_url("core").unwrap();
        assert_eq!(best_url, "https://fast.mirror.org/core");
    }

    #[test]
    fn test_delta_repository_and_content_addressing() {
        let mut delta_idx = DeltaRepositoryIndex::new();
        let sha256_mock = [0xA5u8; 32];
        delta_idx.register_delta(DeltaPackageDescriptor {
            package_name: String::from("zenith-desktop"),
            old_version: String::from("1.0.0"),
            new_version: String::from("1.1.0"),
            delta_url: String::from("https://packages.sigmaos.org/deltas/zenith_1.0_1.1.drpm"),
            delta_size_bytes: 200_000,
            full_size_bytes: 15_000_000,
            delta_sha256: sha256_mock,
        });

        let delta = delta_idx
            .find_delta("zenith-desktop", "1.0.0", "1.1.0")
            .unwrap();
        assert_eq!(delta.delta_size_bytes, 200_000);

        let mut content_idx = ContentAddressedRepoIndex::new();
        content_idx.register_content_address(
            "8f9a2b",
            "zenith-desktop-1.1.0-x86_64-sigmaos",
        );
        assert_eq!(
            content_idx.lookup_hash("8f9a2b").unwrap(),
            "zenith-desktop-1.1.0-x86_64-sigmaos"
        );
    }

    #[test]
    fn test_signature_verifier() {
        let ed_key = [0x77u8; 32];
        let pq_key = [0x88u8; 64];
        let verifier = RepositorySignatureVerifier::new(ed_key, pq_key);

        let index_sha256 = [0x12u8; 32];
        let mut valid_sig = [0u8; 32];
        for i in 0..16 {
            valid_sig[i] = index_sha256[i] ^ ed_key[i];
        }

        assert!(verifier.verify_index_signature(&index_sha256, &valid_sig));

        let bad_sig = [0x00u8; 32];
        assert!(!verifier.verify_index_signature(&index_sha256, &bad_sig));
    }
}
