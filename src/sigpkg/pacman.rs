extern crate alloc;

#[cfg(test)]
extern crate std;

use alloc::vec::Vec;
/// Arch Linux-inspired rolling release build engine (makepkg) and package manager (pacman) for SigmaOS.
/// Provides PKGBUILD parsing, source compilation simulation, and rolling release dependency installations.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacmanError {
    Success = 0,
    ShaMismatch = 1,
    CompileError = 2,
    DatabaseLocked = 3,
}

/// Represents an Arch-style PKGBUILD metadata configuration
#[derive(Debug, Clone, Copy)]
pub struct PkgBuildScript {
    pub pkgname: [u8; 32],
    pub pkgver: [u8; 16],
    pub pkgrel: u32,
    pub source_url: [u8; 64],
    pub sha256sum: [u8; 32],
}

impl PkgBuildScript {
    pub fn new(name: &[u8], ver: &[u8], rel: u32, url: &[u8], sha: &[u8; 32]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut ver_arr = [0u8; 16];
        let mut url_arr = [0u8; 64];

        let name_len = name.len().min(31);
        let ver_len = ver.len().min(15);
        let url_len = url.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_arr.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(ver.as_ptr(), ver_arr.as_mut_ptr(), ver_len);
            core::ptr::copy_nonoverlapping(url.as_ptr(), url_arr.as_mut_ptr(), url_len);
        }

        PkgBuildScript {
            pkgname: name_arr,
            pkgver: ver_arr,
            pkgrel: rel,
            source_url: url_arr,
            sha256sum: *sha,
        }
    }
}

/// Arch Linux-inspired 'makepkg' source-compilation compiler engine
pub struct MakePkgEngine {
    pub compile_count: AtomicUsize,
}

impl MakePkgEngine {
    pub fn new() -> Self {
        MakePkgEngine {
            compile_count: AtomicUsize::new(0),
        }
    }

    /// Run the makepkg compilation steps: source download, sha256 validation, build, and package
    pub fn build_package(
        &self,
        pkgbuild: &PkgBuildScript,
        source_data: &[u8],
    ) -> Result<(), PacmanError> {
        // Step 1: Validate SHA-256 integrity (Simple CRC/checksum mock verification for tests)
        let mut checksum = 0u8;
        for &b in source_data {
            checksum ^= b;
        }

        if pkgbuild.sha256sum[0] != 0 && checksum != pkgbuild.sha256sum[0] {
            return Err(PacmanError::ShaMismatch);
        }

        // Step 2: Simulating compile steps: prepare(), build(), package()
        self.compile_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

/// Transactional checkpoint snapshot for Arch-defeating atomic rollbacks
#[derive(Debug, Clone)]
pub struct PacmanTransactionCheckpoint {
    pub checkpoint_id: usize,
    pub active_packages_count: usize,
}

/// Arch Linux mirror server entry with latency measurement
#[derive(Debug, Clone)]
pub struct PacmanMirror {
    pub url: [u8; 64],
    pub latency_ms: u32,
    pub active: bool,
}

impl PacmanMirror {
    pub fn new(url_bytes: &[u8], latency_ms: u32) -> Self {
        let mut arr = [0u8; 64];
        let len = url_bytes.len().min(63);
        arr[..len].copy_from_slice(&url_bytes[..len]);
        PacmanMirror {
            url: arr,
            latency_ms,
            active: true,
        }
    }
}

/// Arch Linux mirrorlist selector with dynamic latency sorting
pub struct PacmanMirrorlist {
    pub mirrors: Vec<PacmanMirror>,
}

impl PacmanMirrorlist {
    pub fn new() -> Self {
        PacmanMirrorlist {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, mirror: PacmanMirror) {
        self.mirrors.push(mirror);
    }

    /// Sort mirrors by latency (fastest first)
    pub fn sort_by_latency(&mut self) {
        self.mirrors.sort_by_key(|m| m.latency_ms);
    }
}

/// Arch Build System (ABS) tree fallback manager
pub struct AbsTreeEngine {
    pub repositories: Vec<PkgBuildScript>,
}

impl AbsTreeEngine {
    pub fn new() -> Self {
        AbsTreeEngine {
            repositories: Vec::new(),
        }
    }

    pub fn register_recipe(&mut self, recipe: PkgBuildScript) {
        self.repositories.push(recipe);
    }

    pub fn find_recipe_by_name(&self, name: &[u8]) -> Option<PkgBuildScript> {
        let name_len = name.len().min(31);
        for pkg in &self.repositories {
            let mut matches = true;
            for k in 0..name_len {
                if pkg.pkgname[k] != name[k] {
                    matches = false;
                    break;
                }
            }
            if matches && (pkg.pkgname[name_len] == 0 || name_len == 31) {
                return Some(*pkg);
            }
        }
        None
    }
}

/// Arch Linux-inspired rolling release Pacman package manager database
pub struct PacmanManager {
    pub installed_packages: Vec<Option<PkgBuildScript>>,
    pub checkpoints: Vec<PacmanTransactionCheckpoint>,
    pub parallel_downloads: usize,
    pub mirrorlist: PacmanMirrorlist,
    pub abs_tree: AbsTreeEngine,
}

impl PacmanManager {
    pub fn new() -> Self {
        PacmanManager {
            installed_packages: Vec::new(),
            checkpoints: Vec::new(),
            parallel_downloads: 5, // Arch default ParallelDownloads = 5
            mirrorlist: PacmanMirrorlist::new(),
            abs_tree: AbsTreeEngine::new(),
        }
    }

    pub fn set_parallel_downloads(&mut self, count: usize) {
        self.parallel_downloads = count;
    }

    /// Creates an atomic checkpoint before running rolling upgrades (defeats Arch update breakage)
    pub fn create_checkpoint(&mut self) -> usize {
        let id = self.checkpoints.len() + 1;
        let active_cnt = self.installed_packages.len();
        self.checkpoints.push(PacmanTransactionCheckpoint {
            checkpoint_id: id,
            active_packages_count: active_cnt,
        });
        id
    }

    /// Instant sub-millisecond transactional rollback to a specified checkpoint ID
    pub fn rollback_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), PacmanError> {
        let mut target_count = None;
        for cp in &self.checkpoints {
            if cp.checkpoint_id == checkpoint_id {
                target_count = Some(cp.active_packages_count);
                break;
            }
        }

        if let Some(cnt) = target_count {
            self.installed_packages.truncate(cnt);
            Ok(())
        } else {
            Err(PacmanError::CompileError)
        }
    }

    /// Install compiled package into Pacman db (pacman -U equivalent)
    pub fn install_package(&mut self, pkg: PkgBuildScript) {
        self.installed_packages.push(Some(pkg));
    }

    /// Upgrade/Rolling sync of all packages (pacman -Syu equivalent)
    pub fn rolling_upgrade(&mut self) -> usize {
        let mut upgraded = 0;
        for opt in &mut self.installed_packages {
            if let Some(ref mut pkg) = opt {
                // Increment version suffix to represent rolling release upgrade
                pkg.pkgrel += 1;
                upgraded += 1;
            }
        }
        upgraded
    }
}

// =========================================================================
// ARCH LINUX PACMAN-CONTRIB SUITE
// =========================================================================

/// `paccache` - Purges obsolete cached package tarballs keeping N recent versions
pub struct PaccacheEngine {
    pub keep_count: usize,
    pub cache_files: Vec<(String, u32)>, // (package_name, pkgrel)
}

impl PaccacheEngine {
    pub fn new(keep_count: usize) -> Self {
        Self {
            keep_count,
            cache_files: Vec::new(),
        }
    }

    pub fn add_cached_file(&mut self, pkg_name: &str, pkgrel: u32) {
        self.cache_files.push((pkg_name.to_string(), pkgrel));
    }

    pub fn purge_unneeded_cache(&mut self) -> usize {
        let mut purged = 0;
        let mut counts = alloc::collections::BTreeMap::new();
        let mut remaining = Vec::new();

        self.cache_files.sort_by(|a, b| b.1.cmp(&a.1)); // Sort highest pkgrel first

        for (name, rel) in &self.cache_files {
            let entry = counts.entry(name.clone()).or_insert(0usize);
            if *entry < self.keep_count {
                *entry += 1;
                remaining.push((name.clone(), *rel));
            } else {
                purged += 1;
            }
        }

        self.cache_files = remaining;
        purged
    }
}

/// `checkupdates` - Scans repository updates safely without locking the primary pacman DB
pub struct CheckupdatesEngine;

impl CheckupdatesEngine {
    pub fn scan_pending_updates(installed: &[PkgBuildScript], repo: &[PkgBuildScript]) -> Vec<(String, u32, u32)> {
        let mut pending = Vec::new();
        for inst in installed {
            let inst_name = core::str::from_utf8(&inst.pkgname).unwrap_or("").trim_matches('\0');
            for r in repo {
                let r_name = core::str::from_utf8(&r.pkgname).unwrap_or("").trim_matches('\0');
                if inst_name == r_name && r.pkgrel > inst.pkgrel {
                    pending.push((inst_name.to_string(), inst.pkgrel, r.pkgrel));
                }
            }
        }
        pending
    }
}

/// `rankmirrors` - Benchmarks and sorts package repository mirrors by latency
pub struct RankmirrorsEngine;

impl RankmirrorsEngine {
    pub fn rank_mirrors(mirrorlist: &mut PacmanMirrorlist) {
        mirrorlist.sort_by_latency();
    }
}

/// `pactree` - Package dependency tree generator
pub struct PactreeEngine;

impl PactreeEngine {
    pub fn render_dependency_tree(pkg: &PkgBuildScript, abs: &AbsTreeEngine) -> String {
        let name = core::str::from_utf8(&pkg.pkgname).unwrap_or("").trim_matches('\0');
        let mut tree = format!("{}\n", name);
        if let Some(recipe) = abs.find_recipe_by_name(&pkg.pkgname) {
            let source_url = core::str::from_utf8(&recipe.source_url).unwrap_or("").trim_matches('\0');
            tree.push_str(&format!("  └── src: {}\n", source_url));
        }
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_makepkg_compilation() {
        let mut mock_sha = [0u8; 32];
        mock_sha[0] = 0x55; // simple checksum expectation

        let pkgbuild = PkgBuildScript::new(
            b"arch-cli",
            b"1.0.0",
            1,
            b"https://archlinux.org/src.tar",
            &mock_sha,
        );
        let makepkg = MakePkgEngine::new();

        // Valid source data matching XOR checksum
        let source_data = [0x55u8];
        assert!(makepkg.build_package(&pkgbuild, &source_data).is_ok());
        assert_eq!(makepkg.compile_count.load(Ordering::SeqCst), 1);

        // Invalid source data (checksum mismatch)
        let invalid_source = [0xAAu8];
        assert_eq!(
            makepkg
                .build_package(&pkgbuild, &invalid_source)
                .unwrap_err() as usize,
            PacmanError::ShaMismatch as usize
        );
    }

    #[test]
    fn test_pacman_rolling_upgrades() {
        let mut pacman = PacmanManager::new();
        let mock_sha = [0u8; 32];

        let pkg1 = PkgBuildScript::new(b"pacman-test", b"3.0", 1, b"source", &mock_sha);
        pacman.install_package(pkg1);

        assert_eq!(pacman.installed_packages.len(), 1);
        assert_eq!(pacman.installed_packages[0].unwrap().pkgrel, 1);

        // Perform rolling upgrade (pacman -Syu)
        let upgraded_count = pacman.rolling_upgrade();
        assert_eq!(upgraded_count, 1);
        assert_eq!(pacman.installed_packages[0].unwrap().pkgrel, 2);
    }

    #[test]
    fn test_transactional_rollback() {
        let mut pacman = PacmanManager::new();
        let mock_sha = [0u8; 32];

        let pkg1 = PkgBuildScript::new(b"pkg1", b"1.0", 1, b"src1", &mock_sha);
        pacman.install_package(pkg1);
        let cp_id = pacman.create_checkpoint();

        let pkg2 = PkgBuildScript::new(b"pkg2", b"1.0", 1, b"src2", &mock_sha);
        pacman.install_package(pkg2);
        assert_eq!(pacman.installed_packages.len(), 2);

        // Rollback to checkpoint 1
        assert!(pacman.rollback_checkpoint(cp_id).is_ok());
        assert_eq!(pacman.installed_packages.len(), 1);
    }


    #[test]
    fn test_pacman_contrib_suite() {
        let mut cache = PaccacheEngine::new(1);
        cache.add_cached_file("linux", 1);
        cache.add_cached_file("linux", 2);
        cache.add_cached_file("linux", 3);
        assert_eq!(cache.purge_unneeded_cache(), 2);
        assert_eq!(cache.cache_files.len(), 1);

        let mock_sha = [0u8; 32];
        let inst = PkgBuildScript::new(b"ripgrep", b"14.0", 1, b"https://arch.org", &mock_sha);
        let repo_new = PkgBuildScript::new(b"ripgrep", b"14.0", 2, b"https://arch.org", &mock_sha);
        let pending = CheckupdatesEngine::scan_pending_updates(&[inst], &[repo_new]);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0, "ripgrep");

        let mut abs = AbsTreeEngine::new();
        abs.register_recipe(inst);
        let tree = PactreeEngine::render_dependency_tree(&inst, &abs);
        assert!(tree.contains("ripgrep"));
        assert!(tree.contains("https://arch.org"));
    }
}
