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
        for i in 0..self.mirrors.len {
            for j in (i + 1)..self.mirrors.len {
                if self.mirrors[j].latency_ms < self.mirrors[i].latency_ms {
                    let temp = self.mirrors[i].clone();
                    self.mirrors[i] = self.mirrors[j].clone();
                    self.mirrors[j] = temp;
                }
            }
        }
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
        for i in 0..self.repositories.len {
            let pkg = &self.repositories[i];
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
                free(self.data as *mut u8, self.capacity * mem::size_of::<T>());
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
                free(self.data as *mut u8, self.capacity * mem::size_of::<T>());
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    if size == 0 { return core::ptr::null_mut(); }
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8, size: usize) {
    if !ptr.is_null() && size > 0 {
        use std::alloc::{dealloc, Layout};
        let layout = Layout::from_size_align(size, 8).unwrap();
        dealloc(ptr, layout);
    }
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8, size: usize);
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
    fn test_pacman_mirrorlist_sorting() {
        let mut mirrorlist = PacmanMirrorlist::new();
        mirrorlist.add_mirror(PacmanMirror::new(b"https://slow.archlinux.org/repo", 250));
        mirrorlist.add_mirror(PacmanMirror::new(b"https://fast.archlinux.org/repo", 20));
        mirrorlist.add_mirror(PacmanMirror::new(b"https://medium.archlinux.org/repo", 80));

        mirrorlist.sort_by_latency();
        assert_eq!(mirrorlist.mirrors[0].latency_ms, 20);
        assert_eq!(mirrorlist.mirrors[1].latency_ms, 80);
        assert_eq!(mirrorlist.mirrors[2].latency_ms, 250);
    }

    #[test]
    fn test_pacman_parallel_download_config() {
        let mut pacman = PacmanManager::new();
        assert_eq!(pacman.parallel_downloads, 5);
        pacman.set_parallel_downloads(10);
        assert_eq!(pacman.parallel_downloads, 10);
    }

    #[test]
    fn test_abs_tree_fallback() {
        let mut abs = AbsTreeEngine::new();
        let mock_sha = [0u8; 32];
        let recipe = PkgBuildScript::new(b"neofetch", b"7.1.0", 1, b"https://arch.org", &mock_sha);
        abs.register_recipe(recipe);

        let found = abs.find_recipe_by_name(b"neofetch");
        assert!(found.is_some());
        assert_eq!(found.unwrap().pkgrel, 1);

        let missing = abs.find_recipe_by_name(b"nonexistent");
        assert!(missing.is_none());
    }
}
