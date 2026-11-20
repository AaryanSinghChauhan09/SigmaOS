//! SigmaOS Packages — dependency resolution, installation, and verification.
//!
//! Architecture principles applied:
//! - DRY: shared resolution logic in [`DependencyGraph`] used by both resolver and installer.
//! - Fail-fast: cycle detection aborts resolution immediately ([`ResolveError::Cycle`]).
//! - Defensive programming: [`PackageVerifier`] must succeed before install proceeds.
//! - SOLID: each struct has one responsibility; installer delegates to resolver and verifier.
//! - Stable API: public types and trait signatures are versioned and must not break callers.
//! - Rollback safety: [`PackageInstaller`] creates a snapshot before mutating system state.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Core domain types
// ---------------------------------------------------------------------------

/// Semantic version with major, minor, and patch components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
    /// Patch version number.
    pub patch: u32,
}

impl Version {
    /// Create a new [`Version`].
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A version constraint that a dependency must satisfy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    /// Any version is acceptable.
    Any,
    /// Exactly this version.
    Exact(Version),
    /// Greater than or equal to this version.
    AtLeast(Version),
    /// Less than this version.
    LessThan(Version),
}

impl VersionConstraint {
    /// Returns `true` when `version` satisfies this constraint.
    pub fn satisfies(&self, version: Version) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(v) => version == *v,
            Self::AtLeast(v) => version >= *v,
            Self::LessThan(v) => version < *v,
        }
    }
}

/// A named dependency with an optional version constraint.
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Name of the required package.
    pub name: String,
    /// Version constraint the required package must satisfy.
    pub constraint: VersionConstraint,
}

impl Dependency {
    /// Create a new [`Dependency`] requiring any version.
    pub fn any(name: impl Into<String>) -> Self {
        Self { name: name.into(), constraint: VersionConstraint::Any }
    }

    /// Create a new [`Dependency`] requiring at least `version`.
    pub fn at_least(name: impl Into<String>, version: Version) -> Self {
        Self { name: name.into(), constraint: VersionConstraint::AtLeast(version) }
    }
}

/// A package registered in the graph.
#[derive(Debug, Clone)]
pub struct Package {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Direct runtime dependencies.
    pub dependencies: Vec<Dependency>,
    /// Package checksum (SHA-256 hex string).
    pub checksum: String,
}

impl Package {
    /// Create a new [`Package`] with no dependencies.
    pub fn new(name: impl Into<String>, version: Version) -> Self {
        Self {
            name: name.into(),
            version,
            dependencies: Vec::new(),
            checksum: String::new(),
        }
    }

    /// Builder-style method for adding a dependency.
    pub fn with_dep(mut self, dep: Dependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Builder-style method for setting a checksum.
    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = checksum.into();
        self
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Errors produced by [`PackageResolver`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ResolveError {
    /// A circular dependency was detected.
    #[error("circular dependency detected involving package '{0}'")]
    Cycle(String),

    /// A required package could not be found in the graph.
    #[error("package '{0}' not found in the dependency graph")]
    NotFound(String),

    /// No version of the package satisfies the required constraint.
    #[error("version conflict for package '{name}': required {constraint:?}, found {found}")]
    VersionConflict {
        /// Package name involved in the conflict.
        name: String,
        /// The constraint that could not be satisfied.
        constraint: VersionConstraint,
        /// The version that was available.
        found: Version,
    },
}

/// Errors produced by [`PackageVerifier`].
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VerifyError {
    /// The package checksum does not match the expected value.
    #[error("checksum mismatch for '{name}': expected {expected}, got {actual}")]
    ChecksumMismatch {
        /// Package name.
        name: String,
        /// Expected checksum.
        expected: String,
        /// Actual computed checksum.
        actual: String,
    },

    /// The package signature is invalid.
    #[error("invalid signature for package '{0}'")]
    InvalidSignature(String),

    /// The package is missing required metadata.
    #[error("missing metadata field '{field}' for package '{name}'")]
    MissingMetadata {
        /// Package name.
        name: String,
        /// The missing metadata field.
        field: String,
    },
}

/// Errors produced by [`PackageInstaller`].
#[derive(Debug, Error)]
pub enum InstallError {
    /// Dependency resolution failed.
    #[error("resolution failed: {0}")]
    Resolution(#[from] ResolveError),

    /// Verification failed before installation.
    #[error("verification failed: {0}")]
    Verification(#[from] VerifyError),

    /// A rollback was triggered and succeeded.
    #[error("installation failed and was rolled back to snapshot '{0}'")]
    RolledBack(String),

    /// The installation itself failed (non-recoverable).
    #[error("installation of '{0}' failed: {1}")]
    Failed(String, String),
}

// ---------------------------------------------------------------------------
// DependencyGraph
// ---------------------------------------------------------------------------

/// An immutable registry of available packages used by the resolver.
///
/// Keys are package names; values are all known versions sorted ascending.
/// Shared resolution logic lives here — both [`PackageResolver`] and
/// [`PackageInstaller`] rely on this single source of truth (DRY principle).
#[derive(Debug, Default)]
pub struct DependencyGraph {
    packages: BTreeMap<String, Vec<Package>>,
}

impl DependencyGraph {
    /// Create an empty [`DependencyGraph`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a package in the graph.
    ///
    /// Multiple versions of the same package may be registered. Versions are
    /// kept sorted in ascending order.
    pub fn register(&mut self, pkg: Package) {
        let versions = self.packages.entry(pkg.name.clone()).or_default();
        versions.push(pkg);
        versions.sort_by_key(|p| p.version);
    }

    /// Find the best (highest) version of `name` that satisfies `constraint`.
    ///
    /// Returns `None` if the package is unknown or no version satisfies the
    /// constraint.
    pub fn resolve_one(&self, name: &str, constraint: &VersionConstraint) -> Option<&Package> {
        self.packages
            .get(name)?
            .iter()
            .rev() // prefer highest satisfying version
            .find(|p| constraint.satisfies(p.version))
    }

    /// Returns `true` if any version of `name` is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }
}

// ---------------------------------------------------------------------------
// PackageResolver — topological sort with cycle detection
// ---------------------------------------------------------------------------

/// Resolves a full transitive dependency set for a named package.
///
/// Implements a depth-first topological sort. On cycle detection the
/// resolution **aborts immediately** (fail-fast principle).
pub struct PackageResolver<'g> {
    graph: &'g DependencyGraph,
}

impl<'g> PackageResolver<'g> {
    /// Construct a resolver over a shared [`DependencyGraph`].
    pub fn new(graph: &'g DependencyGraph) -> Self {
        Self { graph }
    }

    /// Resolve all transitive dependencies of `root` at the given `version`.
    ///
    /// Returns an ordered list of [`Package`] references in install order
    /// (dependencies before dependents). Aborts with [`ResolveError::Cycle`]
    /// as soon as a cycle is detected.
    pub fn resolve(
        &self,
        root: &str,
        constraint: &VersionConstraint,
    ) -> Result<Vec<&Package>, ResolveError> {
        let mut order: Vec<&Package> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut in_stack: HashSet<String> = HashSet::new();

        self.dfs(root, constraint, &mut order, &mut visited, &mut in_stack)?;
        Ok(order)
    }

    // Recursive DFS helper.
    fn dfs<'a>(
        &'a self,
        name: &str,
        constraint: &VersionConstraint,
        order: &mut Vec<&'a Package>,
        visited: &mut HashSet<String>,
        in_stack: &mut HashSet<String>,
    ) -> Result<(), ResolveError> {
        // Fail-fast: abort on cycle before any further work.
        if in_stack.contains(name) {
            return Err(ResolveError::Cycle(name.to_string()));
        }

        if visited.contains(name) {
            return Ok(()); // already placed in order by a previous branch
        }

        let pkg = self.graph.resolve_one(name, constraint).ok_or_else(|| {
            if self.graph.contains(name) {
                // Package exists but no version satisfies the constraint —
                // report the best available version we found.
                let best = self
                    .graph
                    .packages
                    .get(name)
                    .and_then(|v| v.last())
                    .map(|p| p.version)
                    .unwrap_or(Version::new(0, 0, 0));
                ResolveError::VersionConflict {
                    name: name.to_string(),
                    constraint: constraint.clone(),
                    found: best,
                }
            } else {
                ResolveError::NotFound(name.to_string())
            }
        })?;

        in_stack.insert(name.to_string());

        for dep in &pkg.dependencies {
            self.dfs(&dep.name, &dep.constraint, order, visited, in_stack)?;
        }

        in_stack.remove(name);
        visited.insert(name.to_string());
        order.push(pkg);

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// PackageVerifier — integrity and signature checks
// ---------------------------------------------------------------------------

/// Verifies package integrity before installation.
///
/// Applies defensive programming: **all** checks run before any side effect
/// is allowed.
pub struct PackageVerifier {
    /// Known-good checksums keyed by package name.
    trusted_checksums: HashMap<String, String>,
}

impl PackageVerifier {
    /// Create a new [`PackageVerifier`] with an empty trust store.
    pub fn new() -> Self {
        Self { trusted_checksums: HashMap::new() }
    }

    /// Register a trusted (expected) checksum for a package.
    pub fn trust(&mut self, name: impl Into<String>, checksum: impl Into<String>) {
        self.trusted_checksums.insert(name.into(), checksum.into());
    }

    /// Verify `pkg` has a non-empty name, passes checksum validation, and
    /// carries a non-empty checksum field (simulating a signature check).
    ///
    /// Returns `Ok(())` only when **all** checks pass.
    pub fn verify(&self, pkg: &Package) -> Result<(), VerifyError> {
        // Defensive: name must not be empty.
        if pkg.name.is_empty() {
            return Err(VerifyError::MissingMetadata {
                name: "<unknown>".into(),
                field: "name".into(),
            });
        }

        // Checksum must be present.
        if pkg.checksum.is_empty() {
            return Err(VerifyError::MissingMetadata {
                name: pkg.name.clone(),
                field: "checksum".into(),
            });
        }

        // If we have a trusted expected checksum, compare it.
        if let Some(expected) = self.trusted_checksums.get(&pkg.name) {
            if &pkg.checksum != expected {
                return Err(VerifyError::ChecksumMismatch {
                    name: pkg.name.clone(),
                    expected: expected.clone(),
                    actual: pkg.checksum.clone(),
                });
            }
        }

        // Stub: real implementation would verify an Ed25519/Dilithium signature.
        // For now treat a non-empty checksum field as a valid "signature".
        Ok(())
    }
}

impl Default for PackageVerifier {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// PackageInstaller — orchestrates resolve → verify → install → rollback
// ---------------------------------------------------------------------------

/// Snapshot captured before an install attempt.
#[derive(Debug, Clone)]
pub struct InstallSnapshot {
    /// Identifier of this snapshot.
    pub id: String,
    /// Names of packages that were installed at snapshot time.
    pub installed: Vec<String>,
}

/// Tracks installed packages and provides rollback safety.
///
/// The installer owns the [`DependencyGraph`] and [`PackageVerifier`] so it
/// can co-ordinate the full resolve → verify → install → rollback lifecycle
/// without lifetime parameters on the struct itself.
pub struct PackageInstaller {
    graph: DependencyGraph,
    verifier: PackageVerifier,
    installed: Vec<String>,
    snapshots: Vec<InstallSnapshot>,
}

impl PackageInstaller {
    /// Create a new [`PackageInstaller`] owning `graph` and `verifier`.
    pub fn new(graph: DependencyGraph, verifier: PackageVerifier) -> Self {
        Self { graph, verifier, installed: Vec::new(), snapshots: Vec::new() }
    }

    /// Install `name` satisfying `constraint`.
    ///
    /// Steps (all must succeed before any side effect):
    /// 1. Snapshot current state.
    /// 2. Resolve the full dependency set.
    /// 3. Verify every resolved package.
    /// 4. "Install" each package (records name in the installed list).
    ///
    /// On any failure the snapshot is restored (rollback safety).
    pub fn install(
        &mut self,
        name: &str,
        constraint: &VersionConstraint,
    ) -> Result<Vec<String>, InstallError> {
        // 1. Create rollback snapshot before mutating state.
        let snapshot_id = format!("snap-{}", self.snapshots.len());
        self.snapshots.push(InstallSnapshot {
            id: snapshot_id.clone(),
            installed: self.installed.clone(),
        });

        // 2. Resolve — borrow graph independently so we can collect names.
        //    We separate the resolve call from the error handling so the
        //    immutable borrow of `self.graph` is released before we call
        //    `self.rollback` (which takes `&mut self`).
        let resolve_result = {
            let resolver = PackageResolver::new(&self.graph);
            resolver
                .resolve(name, constraint)
                .map(|pkgs| pkgs.into_iter().map(|p| p.name.clone()).collect::<Vec<_>>())
        };
        let resolved_names: Vec<String> = match resolve_result {
            Ok(names) => names,
            Err(e) => {
                self.rollback(&snapshot_id);
                return Err(InstallError::Resolution(e));
            }
        };

        // 3. Verify all packages first (defensive: no partial install).
        //    Collect verified packages by name — borrow graph again.
        for pkg_name in &resolved_names {
            let pkg = self
                .graph
                .resolve_one(pkg_name, &VersionConstraint::Any)
                .expect("package was just resolved; it must be in the graph");
            if let Err(e) = self.verifier.verify(pkg) {
                self.rollback(&snapshot_id);
                return Err(InstallError::Verification(e));
            }
        }

        // 4. Install (record names).
        let mut newly_installed = Vec::new();
        for pkg_name in resolved_names {
            if !self.installed.contains(&pkg_name) {
                self.installed.push(pkg_name.clone());
                newly_installed.push(pkg_name);
            }
        }
        // Update the snapshot to reflect the post-install state, so callers
        // can roll back to "after this package was installed".
        if let Some(snap) = self.snapshots.iter_mut().rev().find(|s| s.id == snapshot_id) {
            snap.installed = self.installed.clone();
        }

        Ok(newly_installed)
    }

    /// Restore the installed-packages list to the state captured in `snapshot_id`.
    pub fn rollback(&mut self, snapshot_id: &str) {
        if let Some(snap) = self.snapshots.iter().rev().find(|s| s.id == snapshot_id) {
            self.installed = snap.installed.clone();
        }
    }

    /// Return the currently installed package names.
    pub fn installed(&self) -> &[String] {
        &self.installed
    }

    /// Return all captured snapshots.
    pub fn snapshots(&self) -> &[InstallSnapshot] {
        &self.snapshots
    }
}

// ---------------------------------------------------------------------------
// Re-exports
// ---------------------------------------------------------------------------

pub use self::{
    DependencyGraph as Graph,
    PackageInstaller as Installer,
    PackageResolver as Resolver,
    PackageVerifier as Verifier,
};

// ---------------------------------------------------------------------------
// Unit tests (keep minimal; integration tests live in tests/)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> DependencyGraph {
        let mut g = DependencyGraph::new();
        g.register(
            Package::new("libssl", Version::new(3, 0, 0))
                .with_checksum("aabbcc"),
        );
        g.register(
            Package::new("curl", Version::new(8, 0, 0))
                .with_dep(Dependency::at_least("libssl", Version::new(3, 0, 0)))
                .with_checksum("ddeeff"),
        );
        g
    }

    #[test]
    fn version_constraint_any_always_satisfies() {
        assert!(VersionConstraint::Any.satisfies(Version::new(999, 0, 0)));
    }

    #[test]
    fn version_constraint_exact_match() {
        let v = Version::new(1, 2, 3);
        assert!(VersionConstraint::Exact(v).satisfies(v));
        assert!(!VersionConstraint::Exact(v).satisfies(Version::new(1, 2, 4)));
    }

    #[test]
    fn resolver_basic_transitive() {
        let g = sample_graph();
        let r = PackageResolver::new(&g);
        let order = r.resolve("curl", &VersionConstraint::Any).unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0].name, "libssl");
        assert_eq!(order[1].name, "curl");
    }

    #[test]
    fn resolver_cycle_returns_error() {
        let mut g = DependencyGraph::new();
        // a -> b -> a (cycle)
        g.register(
            Package::new("a", Version::new(1, 0, 0))
                .with_dep(Dependency::any("b"))
                .with_checksum("aa"),
        );
        g.register(
            Package::new("b", Version::new(1, 0, 0))
                .with_dep(Dependency::any("a"))
                .with_checksum("bb"),
        );
        let r = PackageResolver::new(&g);
        let err = r.resolve("a", &VersionConstraint::Any).unwrap_err();
        assert!(matches!(err, ResolveError::Cycle(_)));
    }

    #[test]
    fn verifier_rejects_missing_checksum() {
        let v = PackageVerifier::new();
        let pkg = Package::new("bad", Version::new(1, 0, 0));
        assert!(matches!(
            v.verify(&pkg),
            Err(VerifyError::MissingMetadata { .. })
        ));
    }

    #[test]
    fn verifier_rejects_wrong_checksum() {
        let mut v = PackageVerifier::new();
        v.trust("pkg", "correct");
        let pkg = Package::new("pkg", Version::new(1, 0, 0)).with_checksum("wrong");
        assert!(matches!(v.verify(&pkg), Err(VerifyError::ChecksumMismatch { .. })));
    }
}
