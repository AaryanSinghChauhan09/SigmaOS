//! Integration tests: dependency graph resolution — cycle detection and version conflicts.
//!
//! Test strategy follows Arrange-Act-Assert and FIRST principles
//! (Fast, Isolated, Repeatable, Self-validating, Timely).

use sigmaos_packages::{
    Dependency, DependencyGraph, Package, PackageResolver, ResolveError, Version,
    VersionConstraint,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pkg(name: &str, major: u32, minor: u32, patch: u32) -> Package {
    Package::new(name, Version::new(major, minor, patch)).with_checksum("deadbeef")
}

fn pkg_with_dep(name: &str, version: Version, dep: Dependency) -> Package {
    Package::new(name, version).with_dep(dep).with_checksum("deadbeef")
}

// ---------------------------------------------------------------------------
// Happy-path: linear and diamond dependency graphs
// ---------------------------------------------------------------------------

#[test]
fn resolves_package_with_no_deps() {
    // Arrange
    let mut g = DependencyGraph::new();
    g.register(pkg("libssl", 3, 0, 0));

    // Act
    let r = PackageResolver::new(&g);
    let order = r.resolve("libssl", &VersionConstraint::Any).unwrap();

    // Assert
    assert_eq!(order.len(), 1);
    assert_eq!(order[0].name, "libssl");
}

#[test]
fn resolves_transitive_deps_in_install_order() {
    // Arrange: curl → libssl → libc
    let mut g = DependencyGraph::new();
    g.register(pkg("libc", 2, 35, 0));
    g.register(
        pkg_with_dep(
            "libssl",
            Version::new(3, 0, 0),
            Dependency::at_least("libc", Version::new(2, 0, 0)),
        ),
    );
    g.register(
        pkg_with_dep(
            "curl",
            Version::new(8, 0, 0),
            Dependency::at_least("libssl", Version::new(3, 0, 0)),
        ),
    );

    // Act
    let r = PackageResolver::new(&g);
    let order = r.resolve("curl", &VersionConstraint::Any).unwrap();

    // Assert: dependencies appear before dependents
    let names: Vec<&str> = order.iter().map(|p| p.name.as_str()).collect();
    let libc_pos = names.iter().position(|&n| n == "libc").unwrap();
    let ssl_pos = names.iter().position(|&n| n == "libssl").unwrap();
    let curl_pos = names.iter().position(|&n| n == "curl").unwrap();
    assert!(libc_pos < ssl_pos, "libc must come before libssl");
    assert!(ssl_pos < curl_pos, "libssl must come before curl");
}

#[test]
fn diamond_dependency_resolved_without_duplication() {
    // Arrange: both B and C depend on D; A depends on both B and C.
    let mut g = DependencyGraph::new();
    g.register(pkg("D", 1, 0, 0));
    g.register(pkg_with_dep("B", Version::new(1, 0, 0), Dependency::any("D")));
    g.register(pkg_with_dep("C", Version::new(1, 0, 0), Dependency::any("D")));
    let a = Package::new("A", Version::new(1, 0, 0))
        .with_dep(Dependency::any("B"))
        .with_dep(Dependency::any("C"))
        .with_checksum("deadbeef");
    g.register(a);

    // Act
    let r = PackageResolver::new(&g);
    let order = r.resolve("A", &VersionConstraint::Any).unwrap();

    // Assert: D appears exactly once
    let d_count = order.iter().filter(|p| p.name == "D").count();
    assert_eq!(d_count, 1, "diamond dependency D must appear exactly once");
    assert_eq!(order.len(), 4);
}

// ---------------------------------------------------------------------------
// Cycle detection (fail-fast principle)
// ---------------------------------------------------------------------------

#[test]
fn direct_self_cycle_returns_cycle_error() {
    // Arrange: X depends on itself
    let mut g = DependencyGraph::new();
    g.register(
        Package::new("X", Version::new(1, 0, 0))
            .with_dep(Dependency::any("X"))
            .with_checksum("cc"),
    );

    // Act
    let r = PackageResolver::new(&g);
    let err = r.resolve("X", &VersionConstraint::Any).unwrap_err();

    // Assert
    assert!(
        matches!(err, ResolveError::Cycle(_)),
        "expected Cycle, got {err:?}"
    );
}

#[test]
fn two_node_cycle_detected() {
    // Arrange: A ↔ B
    let mut g = DependencyGraph::new();
    g.register(
        Package::new("A", Version::new(1, 0, 0))
            .with_dep(Dependency::any("B"))
            .with_checksum("aa"),
    );
    g.register(
        Package::new("B", Version::new(1, 0, 0))
            .with_dep(Dependency::any("A"))
            .with_checksum("bb"),
    );

    // Act
    let r = PackageResolver::new(&g);
    let err = r.resolve("A", &VersionConstraint::Any).unwrap_err();

    // Assert
    assert!(matches!(err, ResolveError::Cycle(_)));
}

#[test]
fn three_node_cycle_detected() {
    // Arrange: A → B → C → A
    let mut g = DependencyGraph::new();
    g.register(
        Package::new("A", Version::new(1, 0, 0))
            .with_dep(Dependency::any("B"))
            .with_checksum("aa"),
    );
    g.register(
        Package::new("B", Version::new(1, 0, 0))
            .with_dep(Dependency::any("C"))
            .with_checksum("bb"),
    );
    g.register(
        Package::new("C", Version::new(1, 0, 0))
            .with_dep(Dependency::any("A"))
            .with_checksum("cc"),
    );

    // Act
    let r = PackageResolver::new(&g);
    let err = r.resolve("A", &VersionConstraint::Any).unwrap_err();

    // Assert
    assert!(matches!(err, ResolveError::Cycle(_)));
}

// ---------------------------------------------------------------------------
// Version conflict detection
// ---------------------------------------------------------------------------

#[test]
fn version_conflict_when_constraint_not_satisfiable() {
    // Arrange: only v1.0.0 of "lib" exists; require v2.0.0
    let mut g = DependencyGraph::new();
    g.register(pkg("lib", 1, 0, 0));

    // Act
    let r = PackageResolver::new(&g);
    let err = r
        .resolve("lib", &VersionConstraint::AtLeast(Version::new(2, 0, 0)))
        .unwrap_err();

    // Assert
    assert!(
        matches!(err, ResolveError::VersionConflict { .. }),
        "expected VersionConflict, got {err:?}"
    );
}

#[test]
fn latest_version_chosen_among_multiple() {
    // Arrange: v1.0 and v2.0 both registered
    let mut g = DependencyGraph::new();
    g.register(pkg("lib", 1, 0, 0));
    g.register(pkg("lib", 2, 0, 0));

    // Act
    let r = PackageResolver::new(&g);
    let order = r.resolve("lib", &VersionConstraint::Any).unwrap();

    // Assert: highest version selected
    assert_eq!(order[0].version, Version::new(2, 0, 0));
}

#[test]
fn package_not_found_returns_not_found_error() {
    // Arrange: empty graph
    let g = DependencyGraph::new();

    // Act
    let r = PackageResolver::new(&g);
    let err = r.resolve("nonexistent", &VersionConstraint::Any).unwrap_err();

    // Assert
    assert!(matches!(err, ResolveError::NotFound(_)));
}
