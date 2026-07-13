// sigma_pkg.rs — SigmaPKG: Sovereign Package Manager (core resolver)
// Language: Rust (#![no_std], no external crates)
// OOP: PackageSource trait, LocalRegistry/NetworkRegistry (impls), Resolver (composition)
// Specification: docs/design/sigmapkg.md
#![no_std]
#![allow(dead_code)]

// ═══════════════════════════════════════════════════════════════
//  § 1. Version: SemVer (Major.Minor.Patch) — first-principles
// ═══════════════════════════════════════════════════════════════

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }

    /// Compare versions. Returns Ordering::Less / Equal / Greater.
    pub fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering::*;
        if self.major != other.major { return if self.major < other.major { Less } else { Greater }; }
        if self.minor != other.minor { return if self.minor < other.minor { Less } else { Greater }; }
        if self.patch != other.patch { return if self.patch < other.patch { Less } else { Greater }; }
        Equal
    }

    pub fn compatible_with(&self, required: &Self) -> bool {
        // SemVer: major must match, minor/patch of self >= required
        self.major == required.major &&
        (self.minor > required.minor ||
         (self.minor == required.minor && self.patch >= required.patch))
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 2. Package metadata
// ═══════════════════════════════════════════════════════════════

const PKG_NAME_MAX: usize = 64;
const MAX_DEPS:     usize = 16;

#[derive(Clone)]
pub struct PackageId {
    pub name:     [u8; PKG_NAME_MAX],
    pub name_len: usize,
    pub version:  Version,
}

impl PackageId {
    pub fn new(name: &[u8], version: Version) -> Self {
        let n = if name.len() > PKG_NAME_MAX { PKG_NAME_MAX } else { name.len() };
        let mut buf = [0u8; PKG_NAME_MAX];
        let mut i = 0;
        while i < n { buf[i] = name[i]; i += 1; }
        Self { name: buf, name_len: n, version }
    }

    pub fn name_eq(&self, other: &Self) -> bool {
        if self.name_len != other.name_len { return false; }
        let mut i = 0;
        while i < self.name_len {
            if self.name[i] != other.name[i] { return false; }
            i += 1;
        }
        true
    }
}

pub struct PackageManifest {
    pub id:          PackageId,
    pub deps:        [Option<PackageId>; MAX_DEPS],
    pub dep_count:   usize,
    pub checksum:    [u8; 32],  // SHA-256 of archive
    pub installed:   bool,
}

impl PackageManifest {
    pub fn new(id: PackageId) -> Self {
        const NONE_PKG: Option<PackageId> = None;
        Self {
            id,
            deps:      [NONE_PKG; MAX_DEPS],
            dep_count: 0,
            checksum:  [0u8; 32],
            installed: false,
        }
    }

    pub fn add_dep(&mut self, dep: PackageId) -> bool {
        if self.dep_count >= MAX_DEPS { return false; }
        self.deps[self.dep_count] = Some(dep);
        self.dep_count += 1;
        true
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 3. PackageSource trait (abstract — OOP polymorphism)
// ═══════════════════════════════════════════════════════════════

#[derive(Copy, Clone, PartialEq)]
pub enum PkgError {
    NotFound,
    VersionMismatch,
    Corrupted,
    NoSpace,
    DependencyLoop,
}

pub trait PackageSource {
    fn name(&self) -> &[u8];
    fn find(&self, pkg_name: &[u8], min_version: &Version) -> Option<PackageManifest>;
    fn install(&mut self, manifest: &mut PackageManifest) -> Result<(), PkgError>;
}

// ═══════════════════════════════════════════════════════════════
//  § 4. LocalRegistry — in-memory installed package registry
// ═══════════════════════════════════════════════════════════════

const MAX_LOCAL_PKGS: usize = 64;

pub struct LocalRegistry {
    pkgs: [Option<PackageManifest>; MAX_LOCAL_PKGS],
    count: usize,
}

impl LocalRegistry {
    pub const fn new() -> Self {
        const NONE_M: Option<PackageManifest> = None;
        Self { pkgs: [NONE_M; MAX_LOCAL_PKGS], count: 0 }
    }

    pub fn register(&mut self, m: PackageManifest) -> bool {
        if self.count >= MAX_LOCAL_PKGS { return false; }
        self.pkgs[self.count] = Some(m);
        self.count += 1;
        true
    }

    pub fn is_installed(&self, name: &[u8]) -> bool {
        let mut i = 0;
        while i < self.count {
            if let Some(ref m) = self.pkgs[i] {
                if m.id.name_len == name.len() {
                    let mut j = 0;
                    let mut ok = true;
                    while j < m.id.name_len {
                        if m.id.name[j] != name[j] { ok = false; break; }
                        j += 1;
                    }
                    if ok { return true; }
                }
            }
            i += 1;
        }
        false
    }
}

impl PackageSource for LocalRegistry {
    fn name(&self) -> &[u8] { b"local" }

    fn find(&self, pkg_name: &[u8], min_version: &Version) -> Option<PackageManifest> {
        let mut i = 0;
        while i < self.count {
            if let Some(ref m) = self.pkgs[i] {
                let pid = &m.id;
                if pid.name_len == pkg_name.len() {
                    let mut j = 0;
                    let mut ok = true;
                    while j < pid.name_len {
                        if pid.name[j] != pkg_name[j] { ok = false; break; }
                        j += 1;
                    }
                    if ok && pid.version.compatible_with(min_version) {
                        return Some(PackageManifest::new(pid.clone()));
                    }
                }
            }
            i += 1;
        }
        None
    }

    fn install(&mut self, manifest: &mut PackageManifest) -> Result<(), PkgError> {
        manifest.installed = true;
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 5. DependencyResolver (composition of sources)
//        Uses topological sort (Kahn's algorithm — first-principles)
// ═══════════════════════════════════════════════════════════════

const RESOLVER_CAP: usize = 128;

pub struct ResolvedPlan {
    pub order: [Option<PackageId>; RESOLVER_CAP],
    pub len:   usize,
}

impl ResolvedPlan {
    const fn new() -> Self {
        const NONE_ID: Option<PackageId> = None;
        Self { order: [NONE_ID; RESOLVER_CAP], len: 0 }
    }

    fn push(&mut self, id: PackageId) -> bool {
        if self.len >= RESOLVER_CAP { return false; }
        self.order[self.len] = Some(id);
        self.len += 1;
        true
    }

    pub fn contains(&self, name: &[u8]) -> bool {
        let mut i = 0;
        while i < self.len {
            if let Some(ref id) = self.order[i] {
                if id.name_len == name.len() {
                    let mut j = 0;
                    let mut ok = true;
                    while j < id.name_len {
                        if id.name[j] != name[j] { ok = false; break; }
                        j += 1;
                    }
                    if ok { return true; }
                }
            }
            i += 1;
        }
        false
    }
}

pub struct Resolver<'a> {
    source: &'a dyn PackageSource,
}

impl<'a> Resolver<'a> {
    pub fn new(source: &'a dyn PackageSource) -> Self { Self { source } }

    /// Resolve a package and all its transitive dependencies (DFS).
    pub fn resolve(&self, name: &[u8], version: &Version, plan: &mut ResolvedPlan) -> Result<(), PkgError> {
        if plan.contains(name) { return Ok(()); }  // Already resolved
        let manifest = self.source.find(name, version).ok_or(PkgError::NotFound)?;
        // Recurse into deps first (DFS topological order)
        let mut i = 0;
        while i < manifest.dep_count {
            if let Some(ref dep) = manifest.deps[i] {
                self.resolve(&dep.name[..dep.name_len], &dep.version, plan)?;
            }
            i += 1;
        }
        plan.push(manifest.id.clone());
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 6. Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pkg(name: &[u8], ver: Version) -> PackageManifest {
        PackageManifest::new(PackageId::new(name, ver))
    }

    #[test]
    fn test_version_semver_compat() {
        let v100 = Version::new(1, 0, 0);
        let v120 = Version::new(1, 2, 0);
        let v200 = Version::new(2, 0, 0);
        assert!(v120.compatible_with(&v100));
        assert!(!v100.compatible_with(&v120));
        assert!(!v200.compatible_with(&v100)); // Major mismatch
    }

    #[test]
    fn test_local_registry_install_and_find() {
        let mut reg = LocalRegistry::new();
        let m = make_pkg(b"sigma-core", Version::new(1, 0, 0));
        assert!(reg.register(m));
        assert!(reg.is_installed(b"sigma-core"));
        let found = reg.find(b"sigma-core", &Version::new(1, 0, 0));
        assert!(found.is_some());
    }

    #[test]
    fn test_resolver_simple() {
        let mut reg = LocalRegistry::new();
        reg.register(make_pkg(b"libsigma", Version::new(1, 0, 0)));
        let mut libA = make_pkg(b"sigma-audio", Version::new(1, 0, 0));
        libA.add_dep(PackageId::new(b"libsigma", Version::new(1, 0, 0)));
        reg.register(libA);
        let resolver = Resolver::new(&reg);
        let mut plan = ResolvedPlan::new();
        let result = resolver.resolve(b"sigma-audio", &Version::new(1, 0, 0), &mut plan);
        assert!(result.is_ok());
        assert!(plan.contains(b"libsigma"));
        assert!(plan.contains(b"sigma-audio"));
        // libsigma should come before sigma-audio
        let mut lib_idx = 0;
        let mut audio_idx = 0;
        let mut i = 0;
        while i < plan.len {
            if let Some(ref id) = plan.order[i] {
                if &id.name[..id.name_len] == b"libsigma" { lib_idx = i; }
                if &id.name[..id.name_len] == b"sigma-audio" { audio_idx = i; }
            }
            i += 1;
        }
        assert!(lib_idx < audio_idx);
    }
}
