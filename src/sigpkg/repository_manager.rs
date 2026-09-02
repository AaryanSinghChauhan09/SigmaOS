extern crate alloc;
/// Repository Management System (Debian APT + Arch Pacman Inspiration)
/// Manages package repositories, mirrors, and metadata
use crate::klib::BTreeMap;
use crate::sigpkg::{Package, Version, VersionConstraint};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::default::Default;
use core::option::Option::{self, None, Some};
use core::result::Result::{self, Err, Ok};

/// Ubuntu PPA (Personal Package Archive) representation
#[derive(Debug, Clone)]
pub struct PpaRepository {
    pub owner: String,
    pub name: String,
    pub gpg_fingerprint: String,
    pub enabled: bool,
}

impl PpaRepository {
    pub fn new(owner: &str, name: &str, fingerprint: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
            gpg_fingerprint: fingerprint.to_string(),
            enabled: true,
        }
    }

    pub fn to_sources_list_entry(&self) -> String {
        format!(
            "deb https://ppa.launchpadcontent.net/{}/{}/ubuntu main",
            self.owner, self.name
        )
    }
}

/// Linux Mint Sources Mirror Benchmark Engine
#[derive(Debug, Clone)]
pub struct MirrorBenchmark {
    pub url: String,
    pub latency_ms: u32,
    pub download_speed_kbps: u32,
}

pub struct MirrorBenchmarkEngine;

impl MirrorBenchmarkEngine {
    pub fn benchmark_mirrors(mirrors: &[String]) -> Vec<MirrorBenchmark> {
        let mut results = Vec::new();
        for (idx, url) in mirrors.iter().enumerate() {
            // Simulated latency and speed benchmark calculation
            let latency = 20 + ((idx * 15) % 100) as u32;
            let speed = 10000 - (latency * 30);
            results.push(MirrorBenchmark {
                url: url.clone(),
                latency_ms: latency,
                download_speed_kbps: speed,
            });
        }
        results
    }
}

/// GPG Key Verification for Repositories
#[derive(Debug, Clone)]
pub struct RepositoryGpgKey {
    pub key_id: String,
    pub owner_email: String,
    pub is_valid: bool,
}

impl RepositoryGpgKey {
    pub fn new(key_id: &str, owner_email: &str) -> Self {
        Self {
            key_id: key_id.to_string(),
            owner_email: owner_email.to_string(),
            is_valid: true,
        }
    }
}

/// Debian / Ubuntu Official Archives & Foreign Backports
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OfficialArchiveSource {
    Main,
    Universe,
    Restricted,
    Multiverse,
    Backports,
}

/// Repository configuration (Debian sources.list inspiration)
#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub components: Vec<String>,
    pub architectures: Vec<String>,
    pub trusted: bool,
    pub priority: u32,
    pub gpg_key: Option<RepositoryGpgKey>,
    pub archive_source: OfficialArchiveSource,
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
            gpg_key: None,
            archive_source: OfficialArchiveSource::Main,
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

    #[test]
    fn test_ppa_repository() {
        let ppa = PpaRepository::new("graphics-drivers", "ppa", "0x12345678");
        assert_eq!(ppa.owner, "graphics-drivers");
        assert_eq!(
            ppa.to_sources_list_entry(),
            "deb https://ppa.launchpadcontent.net/graphics-drivers/ppa/ubuntu main"
        );
    }

    #[test]
    fn test_mirror_benchmark_engine() {
        let mirrors = vec![
            "https://mirror1.org".to_string(),
            "https://mirror2.org".to_string(),
        ];
        let bench = MirrorBenchmarkEngine::benchmark_mirrors(&mirrors);
        assert_eq!(bench.len(), 2);
        assert!(bench[0].latency_ms < bench[1].latency_ms);
    }

    #[test]
    fn test_repository_gpg_key() {
        let key = RepositoryGpgKey::new("0xABCD", "security@sigmaos.org");
        assert_eq!(key.key_id, "0xABCD");
        assert!(key.is_valid);
    }

    #[test]
    fn test_official_archive_source() {
        let mut repo = Repository::new("universe", "https://archive.ubuntu.com/ubuntu");
        repo.archive_source = OfficialArchiveSource::Universe;
        assert_eq!(repo.archive_source, OfficialArchiveSource::Universe);
    }
}
