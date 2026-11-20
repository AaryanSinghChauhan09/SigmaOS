//! Integration tests: package install workflow — download, verify, install, rollback.
//!
//! Tests exercise the full lifecycle managed by [`PackageInstaller`]:
//!   1. Dependency resolution
//!   2. Pre-install verification
//!   3. Atomic install with snapshot
//!   4. Rollback on failure

use sigmaos_packages::{
    Dependency, DependencyGraph, InstallError, Package, PackageInstaller, PackageVerifier,
    ResolveError, Version, VersionConstraint, VerifyError,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pkg(name: &str, major: u32, minor: u32, patch: u32, checksum: &str) -> Package {
    Package::new(name, Version::new(major, minor, patch)).with_checksum(checksum)
}

/// Build a small graph:  curl → libssl → (no deps)
fn curl_graph() -> DependencyGraph {
    let mut g = DependencyGraph::new();
    g.register(pkg("libssl", 3, 0, 0, "ssl_cksum"));
    g.register(
        Package::new("curl", Version::new(8, 0, 0))
            .with_dep(Dependency::at_least("libssl", Version::new(3, 0, 0)))
            .with_checksum("curl_cksum"),
    );
    g
}

fn trusted_verifier() -> PackageVerifier {
    let mut v = PackageVerifier::new();
    v.trust("libssl", "ssl_cksum");
    v.trust("curl", "curl_cksum");
    v
}

// ---------------------------------------------------------------------------
// Happy-path install
// ---------------------------------------------------------------------------

#[test]
fn installs_package_and_its_transitive_deps() {
    // Arrange
    let mut installer = PackageInstaller::new(curl_graph(), trusted_verifier());

    // Act
    let installed = installer
        .install("curl", &VersionConstraint::Any)
        .unwrap();

    // Assert: both curl and libssl were installed
    assert!(installed.contains(&"libssl".to_string()));
    assert!(installed.contains(&"curl".to_string()));
    assert_eq!(installer.installed().len(), 2);
}

#[test]
fn already_installed_packages_are_not_duplicated() {
    // Arrange
    let mut installer = PackageInstaller::new(curl_graph(), trusted_verifier());

    // Act: install twice
    installer.install("curl", &VersionConstraint::Any).unwrap();
    let second = installer.install("curl", &VersionConstraint::Any).unwrap();

    // Assert: second install reports no newly-installed packages
    assert!(second.is_empty(), "packages already installed should not be re-installed");
    assert_eq!(installer.installed().len(), 2, "total installed count unchanged");
}

#[test]
fn snapshot_is_created_before_each_install() {
    // Arrange
    let mut installer = PackageInstaller::new(curl_graph(), trusted_verifier());

    // Act
    installer.install("curl", &VersionConstraint::Any).unwrap();

    // Assert: at least one snapshot was captured
    assert!(!installer.snapshots().is_empty());
    assert_eq!(installer.snapshots()[0].id, "snap-0");
}

// ---------------------------------------------------------------------------
// Verification failure triggers rollback
// ---------------------------------------------------------------------------

#[test]
fn install_fails_and_rolls_back_when_checksum_wrong() {
    // Arrange: verifier has the wrong expected checksum for libssl
    let mut g = DependencyGraph::new();
    g.register(pkg("libssl", 3, 0, 0, "actual_cksum"));

    let mut v = PackageVerifier::new();
    v.trust("libssl", "expected_different_cksum"); // mismatch

    let mut installer = PackageInstaller::new(g, v);
    // Pre-install something to confirm rollback restores state
    // (nothing installed initially)

    // Act
    let err = installer
        .install("libssl", &VersionConstraint::Any)
        .unwrap_err();

    // Assert: error is Verification(ChecksumMismatch)
    assert!(
        matches!(err, InstallError::Verification(VerifyError::ChecksumMismatch { .. })),
        "expected ChecksumMismatch, got {err:?}"
    );
    // Rollback: nothing should be installed
    assert!(
        installer.installed().is_empty(),
        "installed list should be empty after rollback"
    );
}

#[test]
fn install_fails_and_rolls_back_when_dep_missing() {
    // Arrange: curl requires libssl but libssl is not in the graph
    let mut g = DependencyGraph::new();
    g.register(
        Package::new("curl", Version::new(8, 0, 0))
            .with_dep(Dependency::at_least("libssl", Version::new(3, 0, 0)))
            .with_checksum("curl_cksum"),
    );

    let mut installer = PackageInstaller::new(g, PackageVerifier::new());

    // Act
    let err = installer
        .install("curl", &VersionConstraint::Any)
        .unwrap_err();

    // Assert: resolution failure, nothing installed
    assert!(
        matches!(err, InstallError::Resolution(ResolveError::NotFound(_))),
        "expected NotFound resolution error, got {err:?}"
    );
    assert!(installer.installed().is_empty());
}

// ---------------------------------------------------------------------------
// Manual rollback
// ---------------------------------------------------------------------------

#[test]
fn manual_rollback_restores_previous_state() {
    // Arrange: install libssl first, then attempt curl (but manually rollback)
    let mut g = DependencyGraph::new();
    g.register(pkg("libssl", 3, 0, 0, "ssl_ck"));
    g.register(pkg("curl", 8, 0, 0, "curl_ck"));

    let mut v = PackageVerifier::new();
    v.trust("libssl", "ssl_ck");
    v.trust("curl", "curl_ck");

    let mut installer = PackageInstaller::new(g, v);
    installer.install("libssl", &VersionConstraint::Any).unwrap();
    let snap_id = installer.snapshots().last().unwrap().id.clone();

    installer.install("curl", &VersionConstraint::Any).unwrap();
    assert!(installer.installed().contains(&"curl".to_string()));

    // Act: roll back to after libssl was installed
    installer.rollback(&snap_id);

    // Assert: only libssl remains
    assert!(installer.installed().contains(&"libssl".to_string()));
    assert!(
        !installer.installed().contains(&"curl".to_string()),
        "curl should have been rolled back"
    );
}
