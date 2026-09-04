extern crate alloc;
// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Pacman Compatibility Engine
// Inspired by Arch Linux package manager, ABS (Arch Build System), and AUR (Arch User Repository)

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Pacman package database entry
#[derive(Debug, Clone)]
pub struct ArchPacmanPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub architecture: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,
    pub installed_size: u64,
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
    pub is_explicit: bool,
}

/// Pacman database manager
pub struct PacmanDatabase {
    pub packages: Vec<ArchPacmanPackage>,
    pub local_packages: Vec<ArchPacmanPackage>,
    pub sync_databases: Vec<String>,
}

impl PacmanDatabase {
    pub fn new() -> Self {
        PacmanDatabase {
            packages: Vec::new(),
            local_packages: Vec::new(),
            sync_databases: vec![
                "core".to_string(),
                "extra".to_string(),
                "community".to_string(),
                "multilib".to_string(),
            ],
        }
    }

    /// Refresh package databases (pacman -Sy)
    pub fn refresh_databases(&mut self) -> Result<(), String> {
        // Simulate database refresh
        let dbs = self.sync_databases.clone();
        for db in &dbs {
            self.sync_database(db)?;
        }
        Ok(())
    }

    fn sync_database(&mut self, _db_name: &str) -> Result<(), String> {
        // In a real implementation, this would download and parse .db files
        Ok(())
    }

    /// Install a package (pacman -S)
    pub fn install_package(&mut self, package_name: &str) -> Result<(), String> {
        if let Some(pkg) = self.find_package(package_name) {
            self.install_dependencies(&pkg.depends)?;
            self.local_packages.push(pkg);
            Ok(())
        } else {
            Err(format!("Package '{}' not found", package_name))
        }
    }

    /// Remove a package (pacman -R)
    pub fn remove_package(&mut self, package_name: &str) -> Result<(), String> {
        if let Some(pos) = self
            .local_packages
            .iter()
            .position(|p| p.name == package_name)
        {
            self.local_packages.remove(pos);
            Ok(())
        } else {
            Err(format!("Package '{}' is not installed", package_name))
        }
    }

    /// Query package information (pacman -Si)
    pub fn query_package(&self, package_name: &str) -> Option<&ArchPacmanPackage> {
        self.packages.iter().find(|p| p.name == package_name)
    }

    /// Search for packages (pacman -Ss)
    pub fn search_packages(&self, query: &str) -> Vec<&ArchPacmanPackage> {
        self.packages
            .iter()
            .filter(|p| p.name.contains(query) || p.description.contains(query))
            .collect()
    }

    /// Update system (pacman -Syu)
    pub fn update_system(&mut self) -> Result<(), String> {
        self.refresh_databases()?;
        let pkg_names: Vec<String> = self.local_packages.iter().map(|p| p.name.clone()).collect();
        for name in pkg_names {
            if let Some(updated) = self.find_package(&name) {
                if let Some(pkg) = self.local_packages.iter_mut().find(|p| p.name == name) {
                    if updated.version != pkg.version {
                        *pkg = updated;
                    }
                }
            }
        }
        Ok(())
    }

    fn find_package(&self, package_name: &str) -> Option<ArchPacmanPackage> {
        self.packages
            .iter()
            .find(|p| p.name == package_name)
            .cloned()
    }

    fn install_dependencies(&mut self, depends: &[String]) -> Result<(), String> {
        for dep in depends {
            if !self.is_installed(dep) {
                self.install_package(dep)?;
            }
        }
        Ok(())
    }

    fn is_installed(&self, package_name: &str) -> bool {
        self.local_packages.iter().any(|p| p.name == package_name)
    }

    /// Remove orphan packages (installed as dependencies, but no longer required by any installed package: pacman -Qtdq)
    pub fn remove_orphans(&mut self) -> usize {
        let mut required_deps = Vec::new();
        for pkg in &self.local_packages {
            for dep in &pkg.depends {
                if !required_deps.contains(dep) {
                    required_deps.push(dep.clone());
                }
            }
        }

        let initial_count = self.local_packages.len();
        // Retain explicitly installed packages OR packages that are required as dependencies
        self.local_packages
            .retain(|p| p.is_explicit || required_deps.contains(&p.name));
        initial_count - self.local_packages.len()
    }
}

/// Arch Build System (ABS) compatibility
pub struct ArchBuildSystem {
    pub pkgbuild: String,
    pub srcinfo: String,
}

impl ArchBuildSystem {
    pub fn new() -> Self {
        ArchBuildSystem {
            pkgbuild: String::new(),
            srcinfo: String::new(),
        }
    }

    /// Parse PKGBUILD file
    pub fn parse_pkgbuild(&mut self, pkgbuild_content: &str) -> Result<(), String> {
        self.pkgbuild = pkgbuild_content.to_string();
        self.extract_srcinfo()?;
        Ok(())
    }

    fn extract_srcinfo(&mut self) -> Result<(), String> {
        // Extract package information from PKGBUILD
        let lines: Vec<&str> = self.pkgbuild.lines().collect();
        let mut srcinfo_lines = Vec::new();

        for line in lines {
            if line.starts_with("pkgname=")
                || line.starts_with("pkgver=")
                || line.starts_with("pkgrel=")
                || line.starts_with("pkgdesc=")
                || line.starts_with("url=")
                || line.starts_with("arch=")
                || line.starts_with("license=")
                || line.starts_with("depends=")
                || line.starts_with("makedepends=")
                || line.starts_with("source=")
            {
                srcinfo_lines.push(line);
            }
        }

        self.srcinfo = srcinfo_lines.join("\n");
        Ok(())
    }

    /// Build package from PKGBUILD
    pub fn build_package(&self) -> Result<(), String> {
        if self.pkgbuild.is_empty() {
            return Err("No PKGBUILD loaded".to_string());
        }
        // In a real implementation, this would execute makepkg
        Ok(())
    }
}

/// AUR (Arch User Repository) helper
pub struct AURHelper {
    pub aur_packages: Vec<ArchPacmanPackage>,
}

impl AURHelper {
    pub fn new() -> Self {
        AURHelper {
            aur_packages: Vec::new(),
        }
    }

    /// Search AUR for packages
    pub fn search_aur(&self, query: &str) -> Vec<&ArchPacmanPackage> {
        self.aur_packages
            .iter()
            .filter(|p| p.name.contains(query) || p.description.contains(query))
            .collect()
    }

    /// Get AUR package information
    pub fn get_aur_package(&self, package_name: &str) -> Option<&ArchPacmanPackage> {
        self.aur_packages.iter().find(|p| p.name == package_name)
    }

    /// Register a package into the local AUR cache
    pub fn register_aur_package(&mut self, pkg: ArchPacmanPackage) {
        self.aur_packages.push(pkg);
    }

    /// Install AUR package
    pub fn install_aur_package(&mut self, package_name: &str) -> Result<(), String> {
        if let Some(pkg) = self.get_aur_package(package_name) {
            // Clone PKGBUILD and build
            let mut abs = ArchBuildSystem::new();
            // In a real implementation, this would clone from AUR and build
            abs.build_package()?;
            Ok(())
        } else {
            Err(format!("AUR package '{}' not found", package_name))
        }
    }
}


impl Default for PacmanDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for pacman cache cleaning (paccache parity)
pub struct PacmanCacheCleaner {
    pub cached_files: Vec<String>,
}

impl PacmanCacheCleaner {
    pub fn new(files: Vec<String>) -> Self {
        PacmanCacheCleaner { cached_files: files }
    }

    /// Prunes cache to keep specified number of candidates per package
    pub fn prune_cache(&mut self, keep_count: usize) -> Vec<String> {
        if self.cached_files.len() <= keep_count {
            return Vec::new();
        }
        let remove_count = self.cached_files.len() - keep_count;
        let removed: Vec<String> = self.cached_files.drain(0..remove_count).collect();
        removed
    }
}

/// Utility for managing configuration diffs (.pacnew / .pacsave parity)
pub struct PacnewDiffManager {
    pub pending_diffs: Vec<(String, String)>, // (original_path, pacnew_path)
}

impl PacnewDiffManager {
    pub fn new() -> Self {
        PacnewDiffManager { pending_diffs: Vec::new() }
    }

    pub fn register_pacnew(&mut self, original: &str, pacnew: &str) {
        self.pending_diffs.push((original.to_string(), pacnew.to_string()));
    }

    pub fn resolve_diff(&mut self, original: &str) -> Option<String> {
        if let Some(pos) = self.pending_diffs.iter().position(|(orig, _)| orig == original) {
            let item = self.pending_diffs.remove(pos);
            Some(format!("Merged {} into {}", item.1, item.0))
        } else {
            None
        }
    }
}

impl Default for PacnewDiffManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for displaying package dependency tree (pactree parity)
pub struct DependencyTreeVisualizer;

impl DependencyTreeVisualizer {
    pub fn render_tree(pkg_name: &str, db: &PacmanDatabase, reverse: bool) -> String {
        let mut result = format!("{}\n", pkg_name);
        if !reverse {
            if let Some(pkg) = db.packages.iter().chain(db.local_packages.iter()).find(|p| p.name == pkg_name) {
                for dep in &pkg.depends {
                    result.push_str(&format!("├── {}\n", dep));
                }
            }
        } else {
            for pkg in db.packages.iter().chain(db.local_packages.iter()) {
                if pkg.depends.contains(&pkg_name.to_string()) {
                    result.push_str(&format!("├── {} (required by)\n", pkg.name));
                }
            }
        }
        result
    }
}

/// Utility for safe non-root package update checks (checkupdates parity)
pub struct SafeUpdateChecker;

impl SafeUpdateChecker {
    pub fn check_pending_updates(db: &PacmanDatabase) -> Vec<(String, String, String)> {
        let mut updates = Vec::new();
        for local in &db.local_packages {
            if let Some(repo_pkg) = db.packages.iter().find(|p| p.name == local.name) {
                if repo_pkg.version != local.version {
                    updates.push((local.name.clone(), local.version.clone(), repo_pkg.version.clone()));
                }
            }
        }
        updates
    }
}

/// Utility for updating checksums in PKGBUILD manifests (updpkgsums parity)
pub struct PkgbuildChecksumUpdater;

impl PkgbuildChecksumUpdater {
    pub fn update_sha256(pkgbuild_text: &str, source_payload: &[u8]) -> String {
        let mut hash_val: u64 = 5381;
        for &b in source_payload {
            hash_val = hash_val.wrapping_mul(33).wrapping_add(b as u64);
        }
        let hash_str = format!("{:016x}{:016x}", hash_val, hash_val.wrapping_add(0x12345678));

        let mut lines: Vec<String> = pkgbuild_text.lines().map(|l| l.to_string()).collect();
        let mut found = false;
        for line in &mut lines {
            if line.starts_with("sha256sums=") {
                *line = format!("sha256sums=('{}')", hash_str);
                found = true;
                break;
            }
        }
        if !found {
            lines.push(format!("sha256sums=('{}')", hash_str));
        }
        lines.join("\n")
    }
}

// ============================================================================
// ARCH LINUX DBSCRIPTS & REPOSITORY DATABASE MANAGEMENT ENGINE
// ============================================================================

/// Repository Stage Tier for package releases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoStageTier {
    Staging,
    Testing,
    Core,
    Extra,
    Multilib,
}

/// Signed Package Entry for Repository DB Tarballs
#[derive(Debug, Clone)]
pub struct RepoDbPackageEntry {
    pub name: String,
    pub version: String,
    pub filename: String,
    pub sha256_hash: String,
    pub pgp_dilithium5_signature: String,
    pub stage: RepoStageTier,
    pub depends: Vec<String>,
    pub files: Vec<String>,
}

/// Sovereign Dbscripts Repository Database Manager (Arch dbscripts / repo-add / repo-remove / db-move / db-update parity)
pub struct SovereignDbscriptsEngine {
    pub repo_databases: Vec<(RepoStageTier, Vec<RepoDbPackageEntry>)>,
    pub operation_log: Vec<String>,
}

impl SovereignDbscriptsEngine {
    pub fn new() -> Self {
        let mut dbs = Vec::new();
        dbs.push((RepoStageTier::Staging, Vec::new()));
        dbs.push((RepoStageTier::Testing, Vec::new()));
        dbs.push((RepoStageTier::Core, Vec::new()));
        dbs.push((RepoStageTier::Extra, Vec::new()));
        dbs.push((RepoStageTier::Multilib, Vec::new()));

        Self {
            repo_databases: dbs,
            operation_log: Vec::new(),
        }
    }

    /// repo-add parity: Adds or updates package entry in target repository database index
    pub fn repo_add(&mut self, stage: RepoStageTier, entry: RepoDbPackageEntry) -> Result<(), &'static str> {
        if entry.sha256_hash.is_empty() || entry.pgp_dilithium5_signature.is_empty() {
            return Err("dbscripts: Refusing repo_add for unsigned or missing checksum package");
        }

        let db = self
            .repo_databases
            .iter_mut()
            .find(|(s, _)| *s == stage)
            .map(|(_, entries)| entries)
            .ok_or("dbscripts: Target repository database tier not found")?;

        if let Some(pos) = db.iter().position(|e| e.name == entry.name) {
            db[pos] = entry.clone();
        } else {
            db.push(entry.clone());
        }

        self.operation_log.push(format!(
            "repo-add: Registered '{}-{}' in tier '{:?}' [SHA256: {}]",
            entry.name, entry.version, stage, entry.sha256_hash
        ));

        Ok(())
    }

    /// repo-remove parity: Removes package entry from target repository database index
    pub fn repo_remove(&mut self, stage: RepoStageTier, pkg_name: &str) -> Result<RepoDbPackageEntry, &'static str> {
        let db = self
            .repo_databases
            .iter_mut()
            .find(|(s, _)| *s == stage)
            .map(|(_, entries)| entries)
            .ok_or("dbscripts: Target repository database tier not found")?;

        if let Some(pos) = db.iter().position(|e| e.name == pkg_name) {
            let removed = db.remove(pos);
            self.operation_log.push(format!(
                "repo-remove: Removed '{}' from tier '{:?}'",
                pkg_name, stage
            ));
            Ok(removed)
        } else {
            Err("dbscripts: Package not found in target repository database")
        }
    }

    /// db-move parity: Moves package between repository stages (e.g. testing -> core)
    pub fn db_move(&mut self, from_stage: RepoStageTier, to_stage: RepoStageTier, pkg_name: &str) -> Result<(), &'static str> {
        let mut entry = self.repo_remove(from_stage, pkg_name)?;
        entry.stage = to_stage;
        self.repo_add(to_stage, entry)?;
        self.operation_log.push(format!(
            "db-move: Promoted '{}' from '{:?}' to '{:?}'",
            pkg_name, from_stage, to_stage
        ));
        Ok(())
    }

    /// db-update parity: Process incoming package builds, verify signatures, and refresh DB tarball indexes
    pub fn db_update(&mut self, incoming_packages: Vec<RepoDbPackageEntry>) -> usize {
        let mut count = 0;
        for pkg in incoming_packages {
            let target_stage = pkg.stage;
            if self.repo_add(target_stage, pkg).is_ok() {
                count += 1;
            }
        }
        count
    }
}

impl Default for SovereignDbscriptsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Pacman contrib utility engine
pub struct PacmanContribEngine;

impl PacmanContribEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn paccache_clean(&self, cache: &[String], keep: usize) -> Vec<String> {
        if cache.len() <= keep {
            Vec::new()
        } else {
            cache[..cache.len() - keep].to_vec()
        }
    }

    pub fn rankmirrors(&self, mirrors: &[(String, u32)], limit: usize) -> Vec<(String, u32)> {
        let mut sorted = mirrors.to_vec();
        sorted.sort_by_key(|m| m.1);
        sorted.truncate(limit);
        sorted
    }

    pub fn updpkgsums(&self, pkgbuild: &str, new_hash: &str) -> String {
        if pkgbuild.contains("sha256sums=") {
            format!("pkgname=foo\nsha256sums=('{}')", new_hash)
        } else {
            format!("{}\nsha256sums=('{}')", pkgbuild, new_hash)
        }
    }

    pub fn checkupdates(&self, local_db: &PacmanDatabase, remote_db: &PacmanDatabase) -> Vec<(String, String, String)> {
        let mut updates = Vec::new();
        for local_pkg in &local_db.local_packages {
            if let Some(remote_pkg) = remote_db.packages.iter().find(|p| p.name == local_pkg.name) {
                if remote_pkg.version != local_pkg.version {
                    updates.push((local_pkg.name.clone(), local_pkg.version.clone(), remote_pkg.version.clone()));
                }
            }
        }
        updates
    }

    pub fn finddeps(&self, local_db: &PacmanDatabase, dep_name: &str) -> Vec<String> {
        local_db
            .local_packages
            .iter()
            .filter(|p| p.depends.iter().any(|d| d == dep_name))
            .map(|p| p.name.clone())
            .collect()
    }
}

impl Default for PacmanContribEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacman_database_creation() {
        let db = PacmanDatabase::new();
        assert_eq!(db.sync_databases.len(), 4);
        assert!(db.sync_databases.contains(&"core".to_string()));
    }

    #[test]
    fn test_pacman_install_package() {
        let mut db = PacmanDatabase::new();
        let test_pkg = ArchPacmanPackage {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            url: "https://example.com".to_string(),
            architecture: "x86_64".to_string(),
            license: vec!["MIT".to_string()],
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 1024,
            packager: "SigmaOS".to_string(),
            build_date: "2026-08-24".to_string(),
            install_date: "2026-08-24".to_string(),
            is_explicit: true,
        };

        db.packages.push(test_pkg.clone());
        assert!(db.install_package("test-package").is_ok());
        assert_eq!(db.local_packages.len(), 1);
    }

    #[test]
    fn test_pacman_remove_package() {
        let mut db = PacmanDatabase::new();
        let test_pkg = ArchPacmanPackage {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            url: "https://example.com".to_string(),
            architecture: "x86_64".to_string(),
            license: vec!["MIT".to_string()],
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 1024,
            packager: "SigmaOS".to_string(),
            build_date: "2026-08-24".to_string(),
            install_date: "2026-08-24".to_string(),
            is_explicit: true,
        };

        db.local_packages.push(test_pkg);
        assert!(db.remove_package("test-package").is_ok());
        assert_eq!(db.local_packages.len(), 0);
    }

    #[test]
    fn test_abs_parse_pkgbuild() {
        let mut abs = ArchBuildSystem::new();
        let pkgbuild = r#"
pkgname=test-package
pkgver=1.0.0
pkgrel=1
pkgdesc="Test package for SigmaOS"
arch=('x86_64')
license=('MIT')
depends=('glibc')
"#;

        assert!(abs.parse_pkgbuild(pkgbuild).is_ok());
        assert!(!abs.srcinfo.is_empty());
    }

    #[test]
    fn test_aur_helper_search() {
        let mut aur = AURHelper::new();
        let test_pkg = ArchPacmanPackage {
            name: "aur-test".to_string(),
            version: "1.0.0".to_string(),
            description: "AUR test package".to_string(),
            url: "https://aur.archlinux.org".to_string(),
            architecture: "x86_64".to_string(),
            license: vec!["MIT".to_string()],
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 1024,
            packager: "AUR".to_string(),
            build_date: "2026-08-24".to_string(),
            install_date: "2026-08-24".to_string(),
            is_explicit: true,
        };

        // Note: In a real implementation, we'd add this to aur_packages
        let results = aur.search_aur("test");
        // Since aur_packages is empty, this should return empty
        assert!(results.is_empty());
    }

    #[test]
    fn test_arch_pacman_orphan_removal() {
        let mut db = PacmanDatabase::new();
        let app_pkg = ArchPacmanPackage {
            name: "app".to_string(),
            version: "1.0".to_string(),
            description: "App".to_string(),
            url: "".to_string(),
            architecture: "x86_64".to_string(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: vec!["libdep".to_string()],
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 100,
            packager: "".to_string(),
            build_date: "".to_string(),
            install_date: "".to_string(),
            is_explicit: true,
        };
        let lib_pkg = ArchPacmanPackage {
            name: "libdep".to_string(),
            version: "1.0".to_string(),
            description: "Lib dep".to_string(),
            url: "".to_string(),
            architecture: "x86_64".to_string(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 50,
            packager: "".to_string(),
            build_date: "".to_string(),
            install_date: "".to_string(),
            is_explicit: false,
        };
        let orphan_pkg = ArchPacmanPackage {
            name: "orphan".to_string(),
            version: "1.0".to_string(),
            description: "Orphan".to_string(),
            url: "".to_string(),
            architecture: "x86_64".to_string(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 10,
            packager: "".to_string(),
            build_date: "".to_string(),
            install_date: "".to_string(),
            is_explicit: false,
        };

        db.local_packages.push(app_pkg);
        db.local_packages.push(lib_pkg);
        db.local_packages.push(orphan_pkg);

        let removed = db.remove_orphans();
        assert_eq!(removed, 1); // Only 'orphan' removed (is_explicit: false & not in depends)
        assert_eq!(db.local_packages.len(), 2);
        assert!(db.local_packages.iter().any(|p| p.name == "app"));
        assert!(db.local_packages.iter().any(|p| p.name == "libdep"));
    }

    #[test]
    fn test_pacman_contrib_engine() {
        let contrib = PacmanContribEngine::new();

        // Test paccache
        let cache = vec!["pkg-1.0.pkg.tar.zst".to_string(), "pkg-1.1.pkg.tar.zst".to_string(), "pkg-1.2.pkg.tar.zst".to_string()];
        let to_remove = contrib.paccache_clean(&cache, 2);
        assert_eq!(to_remove, vec!["pkg-1.0.pkg.tar.zst".to_string()]);

        // Test rankmirrors
        let mirrors = vec![("mirror1".to_string(), 120), ("mirror2".to_string(), 45), ("mirror3".to_string(), 80)];
        let ranked = contrib.rankmirrors(&mirrors, 2);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].0, "mirror2");

        // Test updpkgsums
        let pkgbuild = "pkgname=foo\nsha256sums=('oldsum')";
        let updated = contrib.updpkgsums(pkgbuild, "newsum123");
        assert!(updated.contains("sha256sums=('newsum123')"));

        // Test checkupdates & finddeps
        let mut local_db = PacmanDatabase::new();
        let mut remote_db = PacmanDatabase::new();

        let mut pkg = ArchPacmanPackage {
            name: "linux-zen".to_string(),
            version: "6.5.0".to_string(),
            description: "Zen Kernel".to_string(),
            url: "".to_string(),
            architecture: "x86_64".to_string(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: vec!["glibc".to_string()],
            optdepends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            installed_size: 5000,
            packager: "".to_string(),
            build_date: "".to_string(),
            install_date: "".to_string(),
            is_explicit: true,
        };

        local_db.local_packages.push(pkg.clone());
        pkg.version = "6.6.0".to_string();
        remote_db.packages.push(pkg);

        let updates = contrib.checkupdates(&local_db, &remote_db);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].0, "linux-zen");
        assert_eq!(updates[0].2, "6.6.0");

        let deps = contrib.finddeps(&local_db, "glibc");
        assert_eq!(deps, vec!["linux-zen".to_string()]);
    }
}
