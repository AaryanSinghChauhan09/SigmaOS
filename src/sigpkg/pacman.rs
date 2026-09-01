extern crate alloc;

#[cfg(test)]
extern crate std;

use core::mem;
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

/// Arch Linux-inspired rolling release Pacman package manager database
pub struct PacmanManager {
    pub installed_packages: Vec<Option<PkgBuildScript>>,
    pub checkpoints: Vec<PacmanTransactionCheckpoint>,
}

impl PacmanManager {
    pub fn new() -> Self {
        PacmanManager {
            installed_packages: Vec::new(),
            checkpoints: Vec::new(),
        }
    }

    /// Creates an atomic checkpoint before running rolling upgrades (defeats Arch update breakage)
    pub fn create_checkpoint(&mut self) -> usize {
        let id = self.checkpoints.len + 1;
        let active_cnt = self.installed_packages.len;
        self.checkpoints.push(PacmanTransactionCheckpoint {
            checkpoint_id: id,
            active_packages_count: active_cnt,
        });
        id
    }

    /// Instant sub-millisecond transactional rollback to a specified checkpoint ID
    pub fn rollback_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), PacmanError> {
        let mut target_count = None;
        for i in 0..self.checkpoints.len {
            if self.checkpoints[i].checkpoint_id == checkpoint_id {
                target_count = Some(self.checkpoints[i].active_packages_count);
                break;
            }
        }

        if let Some(cnt) = target_count {
            while self.installed_packages.len > cnt {
                self.installed_packages.len -= 1;
            }
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
        for i in 0..self.installed_packages.len {
            if let Some(ref mut pkg) = self.installed_packages[i] {
                // Increment version suffix to represent rolling release upgrade
                pkg.pkgrel += 1;
                upgraded += 1;
            }
        }
        upgraded
    }
}

// =========================================================================
// PACMAN-CONTRIB SUITE (Arch pacman-contrib parity)
// Absorbs paccache, checkupdates, rankmirrors, and pactree into SigmaOS.
// =========================================================================

#[derive(Debug, Clone)]
pub struct CachedPackageFile {
    pub package_name: [u8; 32],
    pub version: [u8; 16],
    pub release: u32,
    pub size_bytes: u64,
    pub is_installed: bool,
}

impl CachedPackageFile {
    pub fn new(name: &[u8], ver: &[u8], rel: u32, size: u64, installed: bool) -> Self {
        let mut name_arr = [0u8; 32];
        let mut ver_arr = [0u8; 16];

        let name_len = name.len().min(31);
        let ver_len = ver.len().min(15);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_arr.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(ver.as_ptr(), ver_arr.as_mut_ptr(), ver_len);
        }

        Self {
            package_name: name_arr,
            version: ver_arr,
            release: rel,
            size_bytes: size,
            is_installed: installed,
        }
    }
}

/// `paccache` parity: Package cache cleaning engine for retaining N recent versions
pub struct PaccacheEngine {
    pub cached_files: Vec<CachedPackageFile>,
}

impl PaccacheEngine {
    pub fn new() -> Self {
        Self {
            cached_files: Vec::new(),
        }
    }

    pub fn add_cache_entry(&mut self, file: CachedPackageFile) {
        self.cached_files.push(file);
    }

    /// Clean cache (`paccache -r -k <keep_count>`): removes candidates exceeding keep_count
    pub fn clean_cache(&mut self, keep_count: usize, uninstalled_only: bool) -> usize {
        let mut removed_count = 0;
        let mut i = 0;
        while i < self.cached_files.len {
            let file = &self.cached_files[i];
            let should_remove = if uninstalled_only {
                !file.is_installed
            } else {
                // Remove if redundant old versions exist beyond keep_count
                let mut match_count = 0;
                for j in 0..self.cached_files.len {
                    if self.cached_files[j].package_name == file.package_name {
                        match_count += 1;
                    }
                }
                match_count > keep_count
            };

            if should_remove {
                // Remove file from cache
                for j in i..(self.cached_files.len - 1) {
                    let next_file = self.cached_files[j + 1].clone();
                    self.cached_files[j] = next_file;
                }
                self.cached_files.len -= 1;
                removed_count += 1;
            } else {
                i += 1;
            }
        }
        removed_count
    }
}

/// `checkupdates` parity: Non-blocking safe sync database check without DB lock conflicts
pub struct CheckupdatesEngine {
    pub available_upgrades: Vec<PkgBuildScript>,
}

impl CheckupdatesEngine {
    pub fn new() -> Self {
        Self {
            available_upgrades: Vec::new(),
        }
    }

    pub fn add_pending_upgrade(&mut self, pkg: PkgBuildScript) {
        self.available_upgrades.push(pkg);
    }

    pub fn query_pending_count(&self) -> usize {
        self.available_upgrades.len
    }
}

/// Mirror entry with benchmark latency (in milliseconds)
#[derive(Debug, Clone, Copy)]
pub struct PacmanMirrorEntry {
    pub url: [u8; 64],
    pub latency_ms: u32,
    pub is_reachable: bool,
}

impl PacmanMirrorEntry {
    pub fn new(url: &[u8], latency_ms: u32, reachable: bool) -> Self {
        let mut url_arr = [0u8; 64];
        let url_len = url.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(url.as_ptr(), url_arr.as_mut_ptr(), url_len);
        }
        Self {
            url: url_arr,
            latency_ms,
            is_reachable: reachable,
        }
    }
}

/// `rankmirrors` parity: Benchmarks mirror list and sorts by response latency
pub struct RankmirrorsEngine {
    pub mirrors: Vec<PacmanMirrorEntry>,
}

impl RankmirrorsEngine {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, mirror: PacmanMirrorEntry) {
        self.mirrors.push(mirror);
    }

    /// Sorts reachable mirrors by lowest latency (`rankmirrors -n <top_n>`)
    pub fn rank_mirrors(&mut self, top_n: usize) -> usize {
        // Selection sort reachable mirrors by latency
        for i in 0..self.mirrors.len {
            for j in (i + 1)..self.mirrors.len {
                if self.mirrors[j].latency_ms < self.mirrors[i].latency_ms {
                    let val_i = self.mirrors[i];
                    let val_j = self.mirrors[j];
                    self.mirrors[i] = val_j;
                    self.mirrors[j] = val_i;
                }
            }
        }

        let mut reachable_cnt = 0;
        for i in 0..self.mirrors.len {
            if self.mirrors[i].is_reachable {
                reachable_cnt += 1;
            }
        }
        reachable_cnt.min(top_n)
    }
}

/// Dependency graph node for `pactree`
#[derive(Debug, Clone, Copy)]
pub struct PactreeNode {
    pub package_name: [u8; 32],
    pub dependency_name: [u8; 32],
}

impl PactreeNode {
    pub fn new(pkg: &[u8], dep: &[u8]) -> Self {
        let mut pkg_arr = [0u8; 32];
        let mut dep_arr = [0u8; 32];
        let pkg_len = pkg.len().min(31);
        let dep_len = dep.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(pkg.as_ptr(), pkg_arr.as_mut_ptr(), pkg_len);
            core::ptr::copy_nonoverlapping(dep.as_ptr(), dep_arr.as_mut_ptr(), dep_len);
        }
        Self {
            package_name: pkg_arr,
            dependency_name: dep_arr,
        }
    }
}

/// `pactree` parity: Package dependency tree resolution engine
pub struct PactreeEngine {
    pub edges: Vec<PactreeNode>,
}

impl PactreeEngine {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn add_dependency_edge(&mut self, pkg: &[u8], dep: &[u8]) {
        self.edges.push(PactreeNode::new(pkg, dep));
    }

    /// Resolves depth-1 direct dependencies for a target package (`pactree <package>`)
    pub fn resolve_direct_dependencies(&self, target_pkg: &[u8]) -> usize {
        let mut target_arr = [0u8; 32];
        let len = target_pkg.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(target_pkg.as_ptr(), target_arr.as_mut_ptr(), len);
        }

        let mut dep_count = 0;
        for i in 0..self.edges.len {
            if self.edges[i].package_name == target_arr {
                dep_count += 1;
            }
        }
        dep_count
    }
}

/// Master pacman-contrib suite combining paccache, checkupdates, rankmirrors, and pactree
pub struct PacmanContribSuite {
    pub paccache: PaccacheEngine,
    pub checkupdates: CheckupdatesEngine,
    pub rankmirrors: RankmirrorsEngine,
    pub pactree: PactreeEngine,
}

impl PacmanContribSuite {
    pub fn new() -> Self {
        Self {
            paccache: PaccacheEngine::new(),
            checkupdates: CheckupdatesEngine::new(),
            rankmirrors: RankmirrorsEngine::new(),
            pactree: PactreeEngine::new(),
        }
    }
}

impl Default for PacmanContribSuite {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use alloc::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
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

        assert_eq!(pacman.installed_packages.len, 1);
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
        assert_eq!(pacman.installed_packages.len, 2);

        // Rollback to checkpoint 1
        assert!(pacman.rollback_checkpoint(cp_id).is_ok());
        assert_eq!(pacman.installed_packages.len, 1);
    }

    #[test]
    fn test_pacman_contrib_suite_paccache_and_checkupdates() {
        let mut contrib = PacmanContribSuite::new();

        // 1. Test paccache
        contrib.paccache.add_cache_entry(CachedPackageFile::new(b"linux", b"6.6.1", 1, 120000000, true));
        contrib.paccache.add_cache_entry(CachedPackageFile::new(b"linux", b"6.6.2", 1, 120000000, true));
        contrib.paccache.add_cache_entry(CachedPackageFile::new(b"linux", b"6.6.3", 1, 120000000, true));

        // Retain keep_count=2 versions, cleans 1 old version
        let removed = contrib.paccache.clean_cache(2, false);
        assert_eq!(removed, 1);
        assert_eq!(contrib.paccache.cached_files.len, 2);

        // 2. Test checkupdates
        let mock_sha = [0u8; 32];
        let pending = PkgBuildScript::new(b"systemd", b"255.2", 1, b"https://arch.org", &mock_sha);
        contrib.checkupdates.add_pending_upgrade(pending);
        assert_eq!(contrib.checkupdates.query_pending_count(), 1);

        // 3. Test rankmirrors
        contrib.rankmirrors.add_mirror(PacmanMirrorEntry::new(b"https://slow.mirror.org", 250, true));
        contrib.rankmirrors.add_mirror(PacmanMirrorEntry::new(b"https://fast.mirror.org", 15, true));
        contrib.rankmirrors.add_mirror(PacmanMirrorEntry::new(b"https://dead.mirror.org", 9999, false));

        let top = contrib.rankmirrors.rank_mirrors(2);
        assert_eq!(top, 2);
        assert_eq!(contrib.rankmirrors.mirrors[0].latency_ms, 15);

        // 4. Test pactree
        contrib.pactree.add_dependency_edge(b"neovim", b"libunwind");
        contrib.pactree.add_dependency_edge(b"neovim", b"libuv");
        contrib.pactree.add_dependency_edge(b"neovim", b"luajit");
        assert_eq!(contrib.pactree.resolve_direct_dependencies(b"neovim"), 3);
        assert_eq!(contrib.pactree.resolve_direct_dependencies(b"tmux"), 0);
    }
}
