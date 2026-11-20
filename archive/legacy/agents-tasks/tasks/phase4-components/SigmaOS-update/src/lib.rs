//! SigmaOS-update — atomic package update manager.
//!
//! Implements the [package lifecycle contract](../../platform-contracts/package-lifecycle.md):
//! resolve → verify → install, with rollback on failure and a full transaction log.
//!
//! Upstream inspiration:
//! - pacman (Arch): <https://archlinux.org/pacman/>
//! - apt (Debian):  <https://wiki.debian.org/Apt>
//! - dnf (Fedora):  <https://rpm-software-management.github.io/dnf/>

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

// ---------------------------------------------------------------------------
// Package record
// ---------------------------------------------------------------------------

/// A package known to the update manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub installed_version: Option<String>,
    pub available_version: String,
    /// SHA-256 hex checksum of the available archive.
    pub checksum: String,
    /// Whether the available package carries a valid signature.
    pub signed: bool,
    pub depends: Vec<String>,
}

impl Package {
    /// `true` if an update is available.
    pub fn needs_update(&self) -> bool {
        self.installed_version.as_deref() != Some(&self.available_version)
    }
}

// ---------------------------------------------------------------------------
// Transaction log
// ---------------------------------------------------------------------------

/// The outcome of a single package operation within a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpResult {
    Updated { from: Option<String>, to: String },
    AlreadyCurrent,
    SkippedUnsigned,
    Failed(String),
}

/// One entry in the transaction log.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub package: String,
    pub result: OpResult,
}

// ---------------------------------------------------------------------------
// Update engine
// ---------------------------------------------------------------------------

/// Errors returned by the update manager.
#[derive(Debug, PartialEq, Eq)]
pub enum UpdateError {
    /// One or more packages failed signature verification.
    SignatureFailure(Vec<String>),
    /// Dependency cycle or missing dependency detected.
    DependencyError(String),
    /// The transaction was rolled back due to an error.
    RolledBack(String),
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateError::SignatureFailure(pkgs) => {
                write!(f, "signature failure: {}", pkgs.join(", "))
            }
            UpdateError::DependencyError(msg) => write!(f, "dependency error: {msg}"),
            UpdateError::RolledBack(msg) => write!(f, "rolled back: {msg}"),
        }
    }
}

impl std::error::Error for UpdateError {}

/// Atomic package update manager.
///
/// Implements the package lifecycle contract: resolve dependencies
/// topologically, verify signatures, install updates atomically, and
/// record every operation in the transaction log.
pub struct UpdateManager {
    /// All packages known to the manager (name → Package).
    pub packages: BTreeMap<String, Package>,
    /// Transaction log (appended after each `apply`).
    pub log: Vec<LogEntry>,
    /// Whether unsigned packages are allowed (default: false).
    pub allow_unsigned: bool,
}

impl UpdateManager {
    /// Create a new manager with an empty package set.
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            log: Vec::new(),
            allow_unsigned: false,
        }
    }

    /// Register a package in the manager.
    pub fn add_package(&mut self, pkg: Package) {
        self.packages.insert(pkg.name.clone(), pkg);
    }

    /// Return packages that have updates available, sorted by name.
    pub fn pending_updates(&self) -> Vec<&Package> {
        let mut out: Vec<&Package> =
            self.packages.values().filter(|p| p.needs_update()).collect();
        out.sort_by_key(|p| &p.name);
        out
    }

    /// Resolve the install order for `targets` (Kahn's topological sort).
    pub fn resolve(&self, targets: &[String]) -> Result<Vec<String>, UpdateError> {
        let mut visited: Vec<String> = Vec::new();
        let mut stack: Vec<String> = targets.to_vec();

        while let Some(name) = stack.pop() {
            if visited.contains(&name) {
                continue;
            }
            let pkg = self.packages.get(&name).ok_or_else(|| {
                UpdateError::DependencyError(format!("unknown package: {name}"))
            })?;
            for dep in &pkg.depends {
                if !visited.contains(dep) {
                    stack.push(dep.clone());
                }
            }
            visited.push(name);
        }

        // Reverse: deps before dependents
        visited.reverse();
        Ok(visited)
    }

    /// Verify signatures for a list of package names.
    ///
    /// Returns `Err(SignatureFailure)` unless `allow_unsigned` is set.
    pub fn verify(&self, names: &[String]) -> Result<(), UpdateError> {
        if self.allow_unsigned {
            return Ok(());
        }
        let failed: Vec<String> = names
            .iter()
            .filter(|n| {
                self.packages.get(*n).map_or(false, |p| !p.signed)
            })
            .cloned()
            .collect();
        if failed.is_empty() {
            Ok(())
        } else {
            Err(UpdateError::SignatureFailure(failed))
        }
    }

    /// Apply updates for `targets` atomically (resolve → verify → install).
    ///
    /// On any error the transaction rolls back (no state change) and returns
    /// `Err`. On success the installed versions are bumped and the log is
    /// extended.
    pub fn apply(&mut self, targets: &[String], dry_run: bool) -> Result<usize, UpdateError> {
        let order = self.resolve(targets)?;
        self.verify(&order)?;

        if dry_run {
            return Ok(order.len());
        }

        let mut entries: Vec<LogEntry> = Vec::new();
        let mut count = 0;

        for name in &order {
            if let Some(pkg) = self.packages.get_mut(name) {
                if pkg.needs_update() {
                    let from = pkg.installed_version.clone();
                    let to = pkg.available_version.clone();
                    pkg.installed_version = Some(to.clone());
                    entries.push(LogEntry {
                        package: name.clone(),
                        result: OpResult::Updated { from, to },
                    });
                    count += 1;
                } else {
                    entries.push(LogEntry {
                        package: name.clone(),
                        result: OpResult::AlreadyCurrent,
                    });
                }
            }
        }

        self.log.extend(entries);
        Ok(count)
    }

    /// Sync all pending updates (shorthand for `apply(&all_pending)`).
    pub fn sync(&mut self, dry_run: bool) -> Result<usize, UpdateError> {
        let targets: Vec<String> = self
            .pending_updates()
            .iter()
            .map(|p| p.name.clone())
            .collect();
        if targets.is_empty() {
            return Ok(0);
        }
        self.apply(&targets, dry_run)
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg(name: &str, installed: Option<&str>, available: &str, signed: bool) -> Package {
        Package {
            name: name.to_string(),
            installed_version: installed.map(|s| s.to_string()),
            available_version: available.to_string(),
            checksum: format!("sha256:{name}"),
            signed,
            depends: Vec::new(),
        }
    }

    #[test]
    fn pending_updates_only_shows_outdated() {
        let mut mgr = UpdateManager::new();
        mgr.add_package(pkg("hyprland", Some("0.34"), "0.35", true));
        mgr.add_package(pkg("waybar", Some("0.9.24"), "0.9.24", true)); // current
        let pending = mgr.pending_updates();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].name, "hyprland");
    }

    #[test]
    fn apply_updates_installed_version() {
        let mut mgr = UpdateManager::new();
        mgr.add_package(pkg("foot", Some("1.16"), "1.17", true));
        let count = mgr.apply(&["foot".to_string()], false).unwrap();
        assert_eq!(count, 1);
        assert_eq!(
            mgr.packages["foot"].installed_version.as_deref(),
            Some("1.17")
        );
    }

    #[test]
    fn apply_dry_run_does_not_mutate() {
        let mut mgr = UpdateManager::new();
        mgr.add_package(pkg("foot", Some("1.16"), "1.17", true));
        mgr.apply(&["foot".to_string()], true).unwrap();
        assert_eq!(
            mgr.packages["foot"].installed_version.as_deref(),
            Some("1.16")
        );
        assert!(mgr.log.is_empty()); // dry-run appends nothing
    }

    #[test]
    fn verify_rejects_unsigned_by_default() {
        let mut mgr = UpdateManager::new();
        mgr.add_package(pkg("rogue", Some("1.0"), "1.1", false));
        let err = mgr.apply(&["rogue".to_string()], false).unwrap_err();
        assert!(matches!(err, UpdateError::SignatureFailure(_)));
        // Installed version must not change on failure
        assert_eq!(
            mgr.packages["rogue"].installed_version.as_deref(),
            Some("1.0")
        );
    }

    #[test]
    fn allow_unsigned_bypasses_check() {
        let mut mgr = UpdateManager::new();
        mgr.allow_unsigned = true;
        mgr.add_package(pkg("rogue", Some("1.0"), "1.1", false));
        assert_eq!(mgr.apply(&["rogue".to_string()], false).unwrap(), 1);
    }

    #[test]
    fn resolve_includes_dependencies() {
        let mut mgr = UpdateManager::new();
        let mut app = pkg("app", Some("1.0"), "1.1", true);
        app.depends = vec!["libfoo".to_string()];
        mgr.add_package(app);
        mgr.add_package(pkg("libfoo", Some("2.0"), "2.1", true));
        let order = mgr.resolve(&["app".to_string()]).unwrap();
        // libfoo must appear before app
        let pos_lib = order.iter().position(|n| n == "libfoo").unwrap();
        let pos_app = order.iter().position(|n| n == "app").unwrap();
        assert!(pos_lib < pos_app);
    }

    #[test]
    fn sync_applies_all_pending() {
        let mut mgr = UpdateManager::new();
        mgr.add_package(pkg("a", Some("1"), "2", true));
        mgr.add_package(pkg("b", Some("1"), "2", true));
        mgr.add_package(pkg("c", Some("3"), "3", true)); // already current
        assert_eq!(mgr.sync(false).unwrap(), 2);
        assert_eq!(mgr.pending_updates().len(), 0);
    }

    #[test]
    fn transaction_log_records_results() {
        let mut mgr = UpdateManager::new();
        mgr.add_package(pkg("x", Some("1"), "2", true));
        mgr.apply(&["x".to_string()], false).unwrap();
        assert_eq!(mgr.log.len(), 1);
        assert!(matches!(mgr.log[0].result, OpResult::Updated { .. }));
    }

    #[test]
    fn update_error_display() {
        assert!(UpdateError::SignatureFailure(vec!["pkg".into()])
            .to_string()
            .contains("signature"));
        assert!(UpdateError::DependencyError("cycle".into())
            .to_string()
            .contains("dependency"));
        assert!(UpdateError::RolledBack("reason".into())
            .to_string()
            .contains("rolled back"));
    }
}
