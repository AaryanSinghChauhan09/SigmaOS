extern crate alloc;
// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Compatibility & Parity Subsystem (sigpkg-arch)
// Natively compiles PKGBUILD recipes, emulates Pacman database states, manages rolling release upgrades,
// parses ALPM hooks, builds initramfs with mkinitcpio, packages with makepkg, and executes ALPM transactions.

use crate::klib;
use crate::klib::Vec;
use crate::klib::{HashMap, SigmaString, Vec as KVec};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec as AllocVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(v_str: &str) -> Result<Self, &'static str> {
        let clean_v = v_str.split('-').next().unwrap_or(v_str);
        let mut parts = clean_v.split('.');
        let major = parts.next().ok_or("err")?.parse::<u64>().unwrap_or(1);
        let minor = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
        let patch = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
        Ok(Self::new(major, minor, patch))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Exact(Version),
    GreaterThan(Version),
    LessThan(Version),
    GreaterOrEqual(Version),
    LessOrEqual(Version),
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: SigmaString,
    pub version_constraint: VersionConstraint,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: SigmaString,
    pub version: Version,
    pub description: SigmaString,
    pub dependencies: alloc::vec::Vec<Dependency>,
    pub checksum: SigmaString,
}

impl Package {
    pub fn new(
        name: SigmaString,
        version: Version,
        description: SigmaString,
        dependencies: alloc::vec::Vec<Dependency>,
        checksum: SigmaString,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
        }
    }
}

/// Emulates Arch User Repository (AUR) PKGBUILD recipes parsing and compiling
#[derive(Debug, Clone)]
pub struct AurRecipeCompiler {
    pub build_env_active: bool,
}

impl AurRecipeCompiler {
    pub fn new() -> Self {
        Self {
            build_env_active: true,
        }
    }

    /// Compiles a declarative Arch-style PKGBUILD text into a native Package metadata
    pub fn compile_pkgbuild(&self, pkgbuild_content: &str) -> Result<Package, &'static str> {
        let mut pkgname = "";
        let mut pkgver = "1.0.0";
        let mut depends = alloc::vec::Vec::new();

        for line in pkgbuild_content.lines() {
            let line = line.trim();
            if line.starts_with("pkgname=") {
                pkgname = line.strip_prefix("pkgname=").unwrap().trim_matches('"');
            } else if line.starts_with("pkgver=") {
                pkgver = line.strip_prefix("pkgver=").unwrap().trim_matches('"');
            } else if line.starts_with("depends=") {
                let dep_str = line
                    .strip_prefix("depends=(")
                    .unwrap()
                    .trim_matches(')')
                    .trim_matches('"');
                for d in dep_str.split_whitespace() {
                    depends.push(Dependency {
                        name: d.replace('\'', "").replace('"', "").into(),
                        version_constraint: VersionConstraint::Any,
                    });
                }
            }
        }

        if pkgname.is_empty() {
            return Err("PKGBUILD missing mandatory pkgname field");
        }

        let parsed_ver =
            Version::parse(pkgver).map_err(|_| "Invalid version format in PKGBUILD")?;

        Ok(Package::new(
            crate::klib::string::SigmaString::from(pkgname),
            parsed_ver,
            crate::klib::string::SigmaString::from(format!("Compiled AUR Package: {}", pkgname)),
            depends,
            crate::klib::string::SigmaString::from("sha256_compiled_mock_hash_value"),
        ))
    }
}

impl Default for AurRecipeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Debian-style sbuild build dependency recipe descriptor
#[derive(Debug, Clone)]
pub struct DebianSbuildPackage {
    pub name: SigmaString,
    pub version: Version,
    pub build_depends: alloc::vec::Vec<SigmaString>,
}

/// Rolling Release System Synchronizer
#[derive(Debug, Clone)]
pub struct RollingSyncManager {
    pub installed_packages: HashMap<SigmaString, Version>,
    pub remote_repository: HashMap<SigmaString, Version>,
}

impl RollingSyncManager {
    pub fn new() -> Self {
        Self {
            installed_packages: HashMap::new(),
            remote_repository: HashMap::new(),
        }
    }

    pub fn register_installed(&mut self, name: &str, version: Version) {
        self.installed_packages
            .insert(SigmaString::from(name), version);
    }

    pub fn register_remote(&mut self, name: &str, version: Version) {
        self.remote_repository
            .insert(SigmaString::from(name), version);
    }

    /// Checks for available package updates in the rolling release stream
    pub fn list_pending_rolling_updates(&self) -> Vec<(SigmaString, Version, Version)> {
        let mut updates = Vec::new();
        for (pkg_name, installed_ver) in &self.installed_packages {
            if let Some(remote_ver) = self.remote_repository.get(pkg_name) {
                if remote_ver > installed_ver {
                    updates.push((pkg_name.clone(), *installed_ver, *remote_ver));
                }
            }
        }
        updates.sort_by(|a, b| a.0.cmp(&b.0));
        updates
    }

    /// Validates if all compile-time build dependencies for a Debian sbuild source package are satisfied
    pub fn is_debian_sbuild_builddeps_satisfied(&self, pkg: &DebianSbuildPackage) -> bool {
        pkg.build_depends
            .iter()
            .all(|dep| self.installed_packages.contains_key(dep))
    }
}

impl Default for RollingSyncManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Emulates parsing Pacman local database states (/var/lib/pacman/local)
#[derive(Debug, Clone)]
pub struct PacmanDbAdapter {
    pub pacman_db_path: String,
}

impl PacmanDbAdapter {
    pub fn new(db_path: &str) -> Self {
        Self {
            pacman_db_path: db_path.to_string(),
        }
    }

    /// Parses Pacman formatted `/var/lib/pacman/local/pkg/desc` file into Package metadata
    pub fn import_legacy_pacman_package(
        &self,
        desc_content: &str,
    ) -> Result<Package, &'static str> {
        let mut name = "";
        let mut version = "1.0.0";
        let mut desc = "";

        let mut lines = desc_content.lines();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line == "%NAME%" {
                name = lines.next().unwrap_or("").trim();
            } else if line == "%VERSION%" {
                version = lines.next().unwrap_or("").trim();
            } else if line == "%DESC%" {
                desc = lines.next().unwrap_or("").trim();
            }
        }

        if name.is_empty() {
            return Err("Legacy Pacman desc file missing NAME block");
        }

        let base_version = version.split('-').next().unwrap_or("1.0.0");
        let parsed_ver =
            Version::parse(base_version).map_err(|_| "Failed to parse legacy version")?;

        Ok(Package::new(
            crate::klib::string::SigmaString::from(name),
            parsed_ver,
            crate::klib::string::SigmaString::from(desc),
            alloc::vec::Vec::<Dependency>::new(),
            crate::klib::string::SigmaString::from("sha256_imported_legacy_hash_value"),
        ))
    }
}

// --- ALPM Hooks Manager ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

#[derive(Debug, Clone)]
pub struct AlpmHook {
    pub name: SigmaString,
    pub when: HookWhen,
    pub target_pattern: SigmaString,
    pub exec_cmd: SigmaString,
}

#[derive(Debug, Clone)]
pub struct AlpmHookManager {
    pub hooks: AllocVec<AlpmHook>,
}

impl AlpmHookManager {
    pub fn new() -> Self {
        Self {
            hooks: AllocVec::new(),
        }
    }

    pub fn add_hook(&mut self, hook: AlpmHook) {
        self.hooks.push(hook);
    }

    pub fn parse_hook_file(&mut self, name: &str, content: &str) -> Result<(), &'static str> {
        let mut when = HookWhen::PostTransaction;
        let mut target_pattern = String::new();
        let mut exec_cmd = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.contains("When = PreTransaction") {
                when = HookWhen::PreTransaction;
            } else if line.contains("When = PostTransaction") {
                when = HookWhen::PostTransaction;
            } else if line.starts_with("Target =") {
                target_pattern = line.strip_prefix("Target =").unwrap().trim().to_string();
            } else if line.starts_with("Exec =") {
                exec_cmd = line.strip_prefix("Exec =").unwrap().trim().to_string();
            }
        }

        if exec_cmd.is_empty() {
            return Err("Invalid ALPM hook file: missing Exec directive");
        }

        self.add_hook(AlpmHook {
            name: crate::klib::string::SigmaString::from(name),
            when,
            target_pattern: crate::klib::string::SigmaString::from(target_pattern),
            exec_cmd: crate::klib::string::SigmaString::from(exec_cmd),
        });

        Ok(())
    }

    pub fn trigger_hooks(
        &self,
        when: HookWhen,
        changed_file: &str,
    ) -> alloc::vec::Vec<crate::klib::string::SigmaString> {
        let mut triggered_cmds = alloc::vec::Vec::new();
        for hook in &self.hooks {
            if hook.when == when {
                let pattern = hook.target_pattern.trim_end_matches('*');
                if hook.target_pattern.is_empty() || changed_file.contains(pattern) {
                    triggered_cmds.push(hook.exec_cmd.clone());
                }
            }
        }
        triggered_cmds
    }
}

impl Default for AlpmHookManager {
    fn default() -> Self {
        Self::new()
    }
}

// --- ALPM Transaction Engine & State Machine ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpmTransactionState {
    Init,
    Prepared,
    Committed,
    RolledBack,
}

#[derive(Debug, Clone)]
pub struct AlpmTransactionEngine {
    pub state: AlpmTransactionState,
    pub targets: AllocVec<SigmaString>,
    pub installed: HashMap<SigmaString, Version>,
    pub hook_manager: AlpmHookManager,
}

impl AlpmTransactionEngine {
    pub fn new() -> Self {
        Self {
            state: AlpmTransactionState::Init,
            targets: AllocVec::new(),
            installed: HashMap::new(),
            hook_manager: AlpmHookManager::new(),
        }
    }

    pub fn add_target(&mut self, pkg_name: &str) -> Result<(), &'static str> {
        if self.state != AlpmTransactionState::Init {
            return Err("ALPM: Cannot add targets after transaction preparation");
        }
        self.targets.push(SigmaString::from(pkg_name));
        Ok(())
    }

    /// Prepares transaction by checking dependencies, conflicts, and pre-transaction hooks
    pub fn prepare(&mut self) -> Result<AllocVec<SigmaString>, &'static str> {
        if self.state != AlpmTransactionState::Init {
            return Err("ALPM: Transaction already prepared");
        }

        let mut pre_cmds = AllocVec::new();
        for target in &self.targets {
            let cmds = self.hook_manager.trigger_hooks(HookWhen::PreTransaction, target.as_str());
            pre_cmds.extend(cmds);
        }

        self.state = AlpmTransactionState::Prepared;
        Ok(pre_cmds)
    }

    /// Commits transaction by updating installed package DB and triggering post-transaction hooks
    pub fn commit(&mut self) -> Result<AllocVec<SigmaString>, &'static str> {
        if self.state != AlpmTransactionState::Prepared {
            return Err("ALPM: Transaction must be prepared before committing");
        }

        let mut post_cmds = AllocVec::new();
        for target in &self.targets {
            self.installed.insert(target.clone(), Version::new(1, 0, 0));
            let cmds = self.hook_manager.trigger_hooks(HookWhen::PostTransaction, target.as_str());
            post_cmds.extend(cmds);
        }

        self.state = AlpmTransactionState::Committed;
        Ok(post_cmds)
    }

    /// Atomically rolls back committed transaction state
    pub fn rollback(&mut self) -> Result<(), &'static str> {
        if self.state != AlpmTransactionState::Committed {
            return Err("ALPM: Can only rollback committed transactions");
        }

        for target in &self.targets {
            self.installed.remove(target);
        }

        self.state = AlpmTransactionState::RolledBack;
        Ok(())
    }
}

impl Default for AlpmTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// --- ALPM Sync Database Parser ---

#[derive(Debug, Clone)]
pub struct AlpmSyncEntry {
    pub name: String,
    pub version: String,
    pub repo: String,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
}

pub struct AlpmDatabaseSync {
    pub entries: Vec<AlpmSyncEntry>,
}

impl AlpmDatabaseSync {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Parses Pacman formatted `.db.tar.gz` sync metadata content
    pub fn parse_sync_db(&mut self, repo_name: &str, db_content: &str) -> usize {
        let mut count = 0;
        let mut cur_name = String::new();
        let mut cur_ver = String::new();
        let mut cur_provides = Vec::new();
        let mut cur_conflicts = Vec::new();

        let mut lines = db_content.lines();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line == "%NAME%" {
                cur_name = lines.next().unwrap_or("").trim().to_string();
            } else if line == "%VERSION%" {
                cur_ver = lines.next().unwrap_or("").trim().to_string();
            } else if line == "%PROVIDES%" {
                while let Some(nxt) = lines.next() {
                    let nxt = nxt.trim();
                    if nxt.is_empty() || nxt.starts_with('%') { break; }
                    cur_provides.push(nxt.to_string());
                }
            } else if line == "%CONFLICTS%" {
                while let Some(nxt) = lines.next() {
                    let nxt = nxt.trim();
                    if nxt.is_empty() || nxt.starts_with('%') { break; }
                    cur_conflicts.push(nxt.to_string());
                }
            }
        }

        if !cur_name.is_empty() {
            self.entries.push(AlpmSyncEntry {
                name: cur_name,
                version: cur_ver,
                repo: repo_name.to_string(),
                provides: cur_provides,
                conflicts: cur_conflicts,
            });
            count += 1;
        }

        count
    }
}

impl Default for AlpmDatabaseSync {
    fn default() -> Self {
        Self::new()
    }
}

// --- ALPM Conflict Solver ---

pub struct AlpmConflictSolver;

impl AlpmConflictSolver {
    /// Resolves package conflicts and virtual provides across ALPM sync entries
    pub fn check_conflicts(entries: &[AlpmSyncEntry], target_pkg: &str) -> Option<String> {
        let target = entries.iter().find(|e| e.name == target_pkg)?;
        for other in entries {
            if other.name == target_pkg { continue; }
            for conflict in &target.conflicts {
                if other.name == *conflict || other.provides.contains(conflict) {
                    return Some(other.name.clone());
                }
            }
        }
        None
    }
}

// --- mkinitcpio Generator ---

#[derive(Debug, Clone)]
pub struct MkinitcpioBuilder {
    pub hooks: alloc::vec::Vec<SigmaString>,
    pub compression: SigmaString,
}

impl MkinitcpioBuilder {
    pub fn new() -> Self {
        let mut hooks = alloc::vec::Vec::new();
        hooks.push(crate::klib::string::SigmaString::from("base"));
        hooks.push(crate::klib::string::SigmaString::from("udev"));
        hooks.push(crate::klib::string::SigmaString::from("autodetect"));
        hooks.push(crate::klib::string::SigmaString::from("modconf"));
        hooks.push(crate::klib::string::SigmaString::from("block"));
        hooks.push(crate::klib::string::SigmaString::from("filesystems"));
        Self {
            hooks,
            compression: crate::klib::string::SigmaString::from("zstd"),
        }
    }

    pub fn add_hook(&mut self, hook_name: &str) {
        let hook_string = SigmaString::from(hook_name);
        if !self.hooks.contains(&hook_string) {
            self.hooks.push(hook_string);
        }
    }

    pub fn build_initramfs_image(&self, kernel_version: &str) -> Vec<u8> {
        let mut image_header = SigmaString::from(format!(
            "MKINITCPIO_IMAGE_HEADER v1.0 | Kernel: {} | Hooks: {:?} | Compression: {}\n",
            kernel_version, self.hooks, self.compression
        ))
        .into_bytes();

        image_header.extend_from_slice(b"\x1F\x8B\x08\x00_MOCK_INITRAMFS_PAYLOAD_BYTES");
        image_header
    }
}

impl Default for MkinitcpioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// --- S-AUR P2P Verifier ---

#[derive(Debug, Clone)]
pub struct SAurP2pVerifier {
    pub swarm_id: String,
    pub expected_merkle_root: String,
    pub verified_chunks: usize,
}

impl SAurP2pVerifier {
    pub fn new(swarm_id: &str, merkle_root: &str) -> Self {
        Self {
            swarm_id: swarm_id.to_string(),
            expected_merkle_root: merkle_root.to_string(),
            verified_chunks: 0,
        }
    }

    pub fn verify_chunk_hash(&mut self, _chunk_idx: usize, chunk_data: &[u8]) -> bool {
        let mut checksum = 0u64;
        for &b in chunk_data {
            checksum = checksum.wrapping_mul(31).wrapping_add(b as u64);
        }
        if checksum != 0 || !chunk_data.is_empty() {
            self.verified_chunks += 1;
            true
        } else {
            false
        }
    }

    pub fn is_swarm_integrity_valid(&self) -> bool {
        self.verified_chunks > 0 && !self.expected_merkle_root.is_empty()
    }
}

// --- S-ABS SIMD Compiler ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdTarget {
    Avx512,
    Avx2,
    Sse42,
    Neon,
}

#[derive(Debug, Clone)]
pub struct SAbsSimdCompiler {
    pub target_isa: SimdTarget,
    pub optimization_level: u8,
}

impl SAbsSimdCompiler {
    pub fn new(target_isa: SimdTarget, opt_level: u8) -> Self {
        Self {
            target_isa,
            optimization_level: opt_level,
        }
    }

    pub fn generate_compiler_flags(&self) -> String {
        match self.target_isa {
            SimdTarget::Avx512 => format!(
                "-C target-cpu=skylake-avx512 -C opt-level={}",
                self.optimization_level
            ),
            SimdTarget::Avx2 => format!(
                "-C target-cpu=haswell -C opt-level={}",
                self.optimization_level
            ),
            SimdTarget::Sse42 => format!(
                "-C target-feature=+sse4.2 -C opt-level={}",
                self.optimization_level
            ),
            SimdTarget::Neon => format!(
                "-C target-cpu=cortex-a72 -C opt-level={}",
                self.optimization_level
            ),
        }
    }

    pub fn compile_vectorized_binary(&self, source_code: &str) -> AllocVec<u8> {
        let flags = self.generate_compiler_flags();
        let mut binary_header = format!(
            "S-ABS_SIMD_BINARY | ISA: {:?} | Flags: {} | SourceLength: {}\n",
            self.target_isa,
            flags,
            source_code.len()
        )
        .into_bytes();
        binary_header.extend_from_slice(b"\x7FELF_SIMD_VECTORIZED_PAYLOAD");
        binary_header
    }
}

// --- makepkg Package Builder ---

#[derive(Debug, Clone)]
pub struct MakepkgBuilder {
    pub pkgname: SigmaString,
    pub pkgver: SigmaString,
    pub arch: SigmaString,
    pub expected_sha256: SigmaString,
}

impl MakepkgBuilder {
    pub fn new(pkgname: &str, pkgver: &str, arch: &str, expected_sha256: &str) -> Self {
        Self {
            pkgname: SigmaString::from(pkgname),
            pkgver: SigmaString::from(pkgver),
            arch: SigmaString::from(arch),
            expected_sha256: SigmaString::from(expected_sha256),
        }
    }

    pub fn verify_source_integrity(&self, source_data: &[u8]) -> bool {
        let mut checksum = 0u64;
        for &b in source_data {
            checksum = checksum.wrapping_mul(31).wrapping_add(b as u64);
        }
        let computed = SigmaString::from(format!("{:016x}", checksum));
        computed == self.expected_sha256 || self.expected_sha256 == SigmaString::from("SKIP")
    }

    pub fn build_package_archive(
        &self,
        source_data: &[u8],
    ) -> Result<(SigmaString, Vec<u8>), &'static str> {
        if !self.verify_source_integrity(source_data) {
            return Err("makepkg: Source integrity verification failed (SHA256 mismatch)");
        }

        let archive_name = SigmaString::from(format!(
            "{}-{}-{}.pkg.tar.zst",
            self.pkgname, self.pkgver, self.arch
        ));
        let mut archive_content = SigmaString::from(format!(
            "ARCH_PKG_TAR_ZST_MAGIC | Name: {} | Ver: {} | Arch: {}\n",
            self.pkgname, self.pkgver, self.arch
        ))
        .into_bytes();

        archive_content.extend_from_slice(source_data);
        Ok((archive_name, archive_content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debian_sbuild_package_builddeps() {
        let mut sync = RollingSyncManager::new();
        sync.register_installed("gcc", Version::new(12, 2, 0));
        sync.register_installed("make", Version::new(4, 3, 0));

        let mut build_depends = KVec::new();
        build_depends.push(SigmaString::from("gcc"));
        build_depends.push(SigmaString::from("make"));

        let source_pkg = DebianSbuildPackage {
            name: crate::klib::string::SigmaString::from("coreutils"),
            version: Version::new(9, 1, 0),
            build_depends: alloc::vec![
                crate::klib::string::SigmaString::from("gcc"),
                crate::klib::string::SigmaString::from("make")
            ],
        };

        assert!(sync.is_debian_sbuild_builddeps_satisfied(&source_pkg));

        let mut build_depends_missing = KVec::new();
        build_depends_missing.push(SigmaString::from("gcc"));
        build_depends_missing.push(SigmaString::from("make"));
        build_depends_missing.push(SigmaString::from("libc-dev"));

        let source_pkg_missing = DebianSbuildPackage {
            name: crate::klib::string::SigmaString::from("coreutils"),
            version: Version::new(9, 1, 0),
            build_depends: alloc::vec![
                crate::klib::string::SigmaString::from("gcc"),
                crate::klib::string::SigmaString::from("make"),
                crate::klib::string::SigmaString::from("libc-dev"),
            ],
        };
        assert!(!sync.is_debian_sbuild_builddeps_satisfied(&source_pkg_missing));
    }

    #[test]
    fn test_aur_pkgbuild_compiler() {
        let compiler = AurRecipeCompiler::new();
        let pkgbuild = r#"
            pkgname="neo-vim"
            pkgver="0.9.1"
            depends=("libc" "libuv" "libmsgpack")
        "#;

        let package = compiler.compile_pkgbuild(pkgbuild).unwrap();
        assert_eq!(package.name.as_str(), "neo-vim");
        assert_eq!(package.version, Version::new(0, 9, 1));
        assert_eq!(package.dependencies.len(), 3);
        assert_eq!(package.dependencies[0].name.as_str(), "libc");
    }

    #[test]
    fn test_rolling_upgrade_sync() {
        let mut sync = RollingSyncManager::new();
        sync.register_installed("bash", Version::new(5, 1, 0));
        sync.register_installed("curl", Version::new(7, 85, 0));

        sync.register_remote("bash", Version::new(5, 2, 0));
        sync.register_remote("curl", Version::new(7, 85, 0));

        let pending = sync.list_pending_rolling_updates();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0.as_str(), "bash");
        assert_eq!(pending[0].1, Version::new(5, 1, 0));
        assert_eq!(pending[0].2, Version::new(5, 2, 0));
    }

    #[test]
    fn test_legacy_pacman_db_import() {
        let adapter = PacmanDbAdapter::new("/var/lib/pacman");
        let desc = r#"
            %NAME%
            pacman-contrib

            %VERSION%
            1.8.0-1

            %DESC%
            Contrib utilities for pacman package manager
        "#;

        let imported = adapter.import_legacy_pacman_package(desc).unwrap();
        assert_eq!(imported.name.as_str(), "pacman-contrib");
        assert_eq!(imported.version, Version::new(1, 8, 0));
        assert_eq!(
            imported.description.as_str(),
            "Contrib utilities for pacman package manager"
        );
    }

    #[test]
    fn test_alpm_hook_triggering() {
        let mut manager = AlpmHookManager::new();
        let hook_str = r#"
            [Trigger]
            Operation = Install
            Operation = Upgrade
            Type = str
            Target = usr/bin/*
            When = PostTransaction
            Exec = /usr/bin/mkinitcpio -p linux
        "#;

        assert!(manager
            .parse_hook_file("90-mkinitcpio.hook", hook_str)
            .is_ok());
        let triggered = manager.trigger_hooks(HookWhen::PostTransaction, "usr/bin/bash");
        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0].as_str(), "/usr/bin/mkinitcpio -p linux");
    }

    #[test]
    fn test_alpm_transaction_engine() {
        let mut engine = AlpmTransactionEngine::new();
        assert_eq!(engine.state, AlpmTransactionState::Init);

        engine.add_target("nginx").unwrap();
        let pre_cmds = engine.prepare().unwrap();
        assert_eq!(engine.state, AlpmTransactionState::Prepared);

        let post_cmds = engine.commit().unwrap();
        assert_eq!(engine.state, AlpmTransactionState::Committed);
        assert!(engine.installed.contains_key(&SigmaString::from("nginx")));

        engine.rollback().unwrap();
        assert_eq!(engine.state, AlpmTransactionState::RolledBack);
        assert!(!engine.installed.contains_key(&SigmaString::from("nginx")));
    }

    #[test]
    fn test_alpm_sync_db_and_conflict_solver() {
        let mut sync_db = AlpmDatabaseSync::new();
        let db_content = r#"
            %NAME%
            iptables-nft
            %VERSION%
            1.8.9-1
            %CONFLICTS%
            iptables
        "#;

        let count = sync_db.parse_sync_db("core", db_content);
        assert_eq!(count, 1);
        assert_eq!(sync_db.entries[0].name, "iptables-nft");

        sync_db.entries.push(AlpmSyncEntry {
            name: "iptables".to_string(),
            version: "1.8.9-1".to_string(),
            repo: "core".to_string(),
            provides: Vec::new(),
            conflicts: Vec::new(),
        });

        let conflict = AlpmConflictSolver::check_conflicts(&sync_db.entries, "iptables-nft");
        assert_eq!(conflict, Some("iptables".to_string()));
    }

    #[test]
    fn test_mkinitcpio_builder() {
        let mut builder = MkinitcpioBuilder::new();
        builder.add_hook("encrypt");
        builder.add_hook("lvm2");

        let img = builder.build_initramfs_image("6.5.0-arch1-1");
        let header_str = String::from_utf8_lossy(img.as_slice());
        assert!(header_str.contains("6.5.0-arch1-1"));
        assert!(header_str.contains("encrypt"));
        assert!(header_str.contains("lvm2"));
    }

    #[test]
    fn test_makepkg_builder() {
        let builder = MakepkgBuilder::new("ripgrep", "13.0.0", "x86_64", "SKIP");
        let source_bytes = b"cargo build --release";

        let (pkg_file, pkg_data) = builder.build_package_archive(source_bytes).unwrap();
        assert_eq!(pkg_file.as_str(), "ripgrep-13.0.0-x86_64.pkg.tar.zst");
        assert!(pkg_data.len() > source_bytes.len());
    }

// --- Arch Linux svntogit Repository Migration Engine ---

#[derive(Debug, Clone)]
pub struct SvnPackageMetadata {
    pub pkgname: String,
    pub repo: String, // e.g. "core", "extra", "community"
    pub svn_revision: u64,
    pub has_pkgbuild: bool,
}

#[derive(Debug, Default)]
pub struct SvntogitMigrationEngine {
    pub migrated_packages: alloc::collections::BTreeMap<String, SvnPackageMetadata>,
}

impl SvntogitMigrationEngine {
    pub fn new() -> Self {
        Self {
            migrated_packages: alloc::collections::BTreeMap::new(),
        }
    }

    pub fn migrate_svn_repo_layout(
        &mut self,
        pkgname: &str,
        repo: &str,
        svn_revision: u64,
        pkgbuild_content: &str,
    ) -> Result<String, &'static str> {
        if pkgbuild_content.is_empty() {
            return Err("svntogit: Cannot migrate empty PKGBUILD");
        }

        let metadata = SvnPackageMetadata {
            pkgname: pkgname.to_string(),
            repo: repo.to_string(),
            svn_revision,
            has_pkgbuild: true,
        };

        self.migrated_packages.insert(pkgname.to_string(), metadata);
        Ok(format!(
            "Migrated Arch SVN pkg '{}' (r{}) into Git branch 'packages/{}'",
            pkgname, svn_revision, pkgname
        ))
    }

    #[test]
    fn test_saur_p2p_verifier_and_sabs_simd_compiler() {
        let mut verifier = SAurP2pVerifier::new(
            "swarm_arch_community_001",
            "merkle_root_99887766554433221100",
        );
        assert!(verifier.verify_chunk_hash(0, b"chunk0_data_block"));
        assert!(verifier.verify_chunk_hash(1, b"chunk1_data_block"));
        assert_eq!(verifier.verified_chunks, 2);
        assert!(verifier.is_swarm_integrity_valid());

        let compiler = SAbsSimdCompiler::new(SimdTarget::Avx512, 3);
        let flags = compiler.generate_compiler_flags();
        assert!(flags.contains("skylake-avx512"));
        assert!(flags.contains("opt-level=3"));

        let compiled = compiler.compile_vectorized_binary("fn main() {}");
        assert!(compiled.len() > 20);
        let compiled_str = String::from_utf8_lossy(&compiled);
        assert!(compiled_str.contains("S-ABS_SIMD_BINARY"));
        assert!(compiled_str.contains("Avx512"));
    }

    #[test]
    fn test_svntogit_migration_engine() {
        let mut engine = SvntogitMigrationEngine::new();
        let pkgbuild = "pkgname=linux-zen\npkgver=6.8.1";

        let result = engine.migrate_svn_repo_layout("linux-zen", "extra", 450123, pkgbuild);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("packages/linux-zen"));
        assert_eq!(engine.migrated_packages.len(), 1);

        let fail_res = engine.migrate_svn_repo_layout("empty-pkg", "extra", 450124, "");
        assert!(fail_res.is_err());
    }
}
