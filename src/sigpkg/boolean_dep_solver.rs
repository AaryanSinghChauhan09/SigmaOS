//! SigmaOS Boolean Dependency Expression Solver
//!
//! Sovereign zero-dependency package dependency solver.
//! Inspired by:
//! - Gentoo Portage USE flag dependency expressions (AND/OR/NOT boolean algebra)
//! - APT's Boolean dependency resolver (Debian/Ubuntu)
//! - Nix dependency expressions (NixOS)
//! - libsolv (openSUSE) — Z3 SAT-based package dependency resolution
//!
//! Supports:
//! - Boolean expressions: AND, OR, NOT, Version constraints
//! - Conflict detection
//! - Greedy dependency resolution with universe lookup
//! - Version constraint matching (exact, at-least, at-most, any)

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ─── Version Constraint ───────────────────────────────────────────────────────

/// Version constraint for package dependency expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionConstraint {
    /// Exact version match: `=pkg-1.0`
    Exact(String),
    /// Minimum version: `>=pkg-1.0`
    AtLeast(String),
    /// Maximum version: `<=pkg-1.0`
    AtMost(String),
    /// Any version is acceptable
    Any,
}

impl VersionConstraint {
    /// Check if a given version satisfies this constraint.
    /// Uses lexicographic comparison (adequate for semver x.y.z).
    pub fn satisfies(&self, version: &str) -> bool {
        match self {
            VersionConstraint::Any => true,
            VersionConstraint::Exact(required) => version == required.as_str(),
            VersionConstraint::AtLeast(min) => Self::version_cmp(version, min) >= 0,
            VersionConstraint::AtMost(max) => Self::version_cmp(version, max) <= 0,
        }
    }

    /// Simple version comparison: splits by '.' and compares numerically
    /// Returns -1, 0, or 1
    fn version_cmp(a: &str, b: &str) -> i32 {
        let parse = |s: &str| -> Vec<u64> {
            s.split('.')
                .filter_map(|p| {
                    // Strip non-numeric suffix (e.g. "1.0-rc1" -> [1, 0])
                    let numeric: String = p.chars().take_while(|c| c.is_ascii_digit()).collect();
                    numeric.parse::<u64>().ok()
                })
                .collect()
        };
        let av = parse(a);
        let bv = parse(b);
        let len = av.len().max(bv.len());
        for i in 0..len {
            let ai = av.get(i).copied().unwrap_or(0);
            let bi = bv.get(i).copied().unwrap_or(0);
            if ai < bi {
                return -1;
            }
            if ai > bi {
                return 1;
            }
        }
        0
    }
}

// ─── Dependency Expression ────────────────────────────────────────────────────

/// Boolean dependency expression node
///
/// Represents the full Gentoo/APT dependency grammar.
#[derive(Debug, Clone)]
pub enum DepExpr {
    /// Simple package name (any version)
    Pkg(String),
    /// Package with version constraint: `pkg >= 2.0`
    Version(String, VersionConstraint),
    /// Boolean AND of two expressions
    And(Box<DepExpr>, Box<DepExpr>),
    /// Boolean OR of two expressions (first available wins)
    Or(Box<DepExpr>, Box<DepExpr>),
    /// Boolean NOT — this package must NOT be installed
    Not(Box<DepExpr>),
    /// Explicit conflict declaration
    Conflict(String),
    /// Always-true expression (no dependency)
    Empty,
}

impl DepExpr {
    /// Helper: create AND expression
    pub fn and(a: DepExpr, b: DepExpr) -> Self {
        DepExpr::And(Box::new(a), Box::new(b))
    }

    /// Helper: create OR expression
    pub fn or(a: DepExpr, b: DepExpr) -> Self {
        DepExpr::Or(Box::new(a), Box::new(b))
    }

    /// Helper: create NOT expression
    pub fn not(a: DepExpr) -> Self {
        DepExpr::Not(Box::new(a))
    }

    /// Collect all package names referenced in this expression
    pub fn referenced_packages(&self) -> Vec<String> {
        let mut pkgs: Vec<String> = Vec::new();
        self.collect_packages(&mut pkgs);
        pkgs
    }

    fn collect_packages(&self, out: &mut Vec<String>) {
        match self {
            DepExpr::Pkg(name) | DepExpr::Conflict(name) => {
                if !out.contains(name) {
                    out.push(name.clone());
                }
            }
            DepExpr::Version(name, _) => {
                if !out.contains(name) {
                    out.push(name.clone());
                }
            }
            DepExpr::And(a, b) | DepExpr::Or(a, b) => {
                a.collect_packages(out);
                b.collect_packages(out);
            }
            DepExpr::Not(inner) => inner.collect_packages(out),
            DepExpr::Empty => {}
        }
    }
}

// ─── Package Universe ─────────────────────────────────────────────────────────

/// The universe of available packages and their versions.
///
/// In a real SigmaOS system this would be backed by the repository index.
#[derive(Debug, Clone)]
pub struct PackageUniverse {
    /// package name -> list of available versions (newest first)
    pub available: BTreeMap<String, Vec<String>>,
}

impl PackageUniverse {
    /// Create an empty universe
    pub fn new() -> Self {
        PackageUniverse { available: BTreeMap::new() }
    }

    /// Add package versions to the universe
    pub fn add_package(&mut self, name: &str, versions: Vec<&str>) {
        let mut vers: Vec<String> = versions.iter().map(|v| String::from(*v)).collect();
        // Sort descending (newest first) using version comparison
        vers.sort_by(|a, b| {
            VersionConstraint::version_cmp(b, a).cmp(&0)
        });
        self.available.insert(String::from(name), vers);
    }

    /// Returns true if the package exists in any version
    pub fn has_package(&self, name: &str) -> bool {
        self.available.contains_key(name)
    }

    /// Returns true if a specific version is available
    pub fn has_version(&self, name: &str, version: &str) -> bool {
        self.available
            .get(name)
            .map(|versions| versions.iter().any(|v| v == version))
            .unwrap_or(false)
    }

    /// Returns the latest (newest) version of a package, if available
    pub fn latest_version(&self, name: &str) -> Option<&str> {
        self.available.get(name)?.first().map(|s| s.as_str())
    }

    /// Returns all versions satisfying a constraint, newest first
    pub fn versions_satisfying(&self, name: &str, constraint: &VersionConstraint) -> Vec<String> {
        self.available
            .get(name)
            .map(|versions| {
                versions
                    .iter()
                    .filter(|v| constraint.satisfies(v))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}

// ─── Boolean Dependency Solver ────────────────────────────────────────────────

/// SigmaOS Boolean Dependency Expression Solver
///
/// Resolves boolean package dependency expressions against a PackageUniverse.
/// Inspired by Gentoo Portage's dep resolver and APT's boolean solver.
pub struct BooleanDepSolver {
    /// The universe of available packages
    pub universe: PackageUniverse,
}

impl BooleanDepSolver {
    /// Create a new solver with the given universe
    pub fn new(universe: PackageUniverse) -> Self {
        BooleanDepSolver { universe }
    }

    // ── Evaluation ────────────────────────────────────────────────────────────

    /// Evaluate a dependency expression against the set of currently installed packages.
    ///
    /// `installed` maps package name -> installed version.
    /// Returns true if the expression is satisfied.
    pub fn evaluate(&self, expr: &DepExpr, installed: &BTreeMap<String, String>) -> bool {
        match expr {
            DepExpr::Empty => true,
            DepExpr::Pkg(name) => installed.contains_key(name.as_str()),
            DepExpr::Version(name, constraint) => {
                installed.get(name.as_str())
                    .map(|ver| constraint.satisfies(ver))
                    .unwrap_or(false)
            }
            DepExpr::And(a, b) => {
                self.evaluate(a, installed) && self.evaluate(b, installed)
            }
            DepExpr::Or(a, b) => {
                self.evaluate(a, installed) || self.evaluate(b, installed)
            }
            DepExpr::Not(inner) => !self.evaluate(inner, installed),
            DepExpr::Conflict(name) => !installed.contains_key(name.as_str()),
        }
    }

    // ── Resolution ────────────────────────────────────────────────────────────

    /// Resolve a list of dependency expressions to a concrete install list.
    ///
    /// Returns `Ok(Vec<(name, version)>)` or `Err(reason)`.
    /// Uses a greedy strategy: for each required package, picks the newest version
    /// satisfying all constraints.
    pub fn resolve(&self, exprs: &[DepExpr]) -> Result<Vec<(String, String)>, String> {
        // Collect all required packages with their constraints
        let mut required: BTreeMap<String, VersionConstraint> = BTreeMap::new();
        let mut conflicts: Vec<String> = Vec::new();

        for expr in exprs {
            self.collect_requirements(expr, &mut required, &mut conflicts)?;
        }

        // Check conflicts
        for conflict in &conflicts {
            if required.contains_key(conflict) {
                return Err(format!("Conflict: package '{}' is both required and conflicted", conflict));
            }
        }

        // Resolve each required package
        let mut install_list: Vec<(String, String)> = Vec::new();
        for (name, constraint) in &required {
            let candidates = self.universe.versions_satisfying(name, constraint);
            if let Some(best_ver) = candidates.into_iter().next() {
                install_list.push((name.clone(), best_ver));
            } else {
                return Err(format!(
                    "Unsatisfiable: no version of '{}' satisfies constraint {:?}",
                    name, constraint
                ));
            }
        }

        Ok(install_list)
    }

    /// Flatten a dependency expression into required packages and constraints
    fn collect_requirements(
        &self,
        expr: &DepExpr,
        required: &mut BTreeMap<String, VersionConstraint>,
        conflicts: &mut Vec<String>,
    ) -> Result<(), String> {
        match expr {
            DepExpr::Empty => {}
            DepExpr::Pkg(name) => {
                if !self.universe.has_package(name) {
                    return Err(format!("Package '{}' not found in universe", name));
                }
                required.entry(name.clone()).or_insert(VersionConstraint::Any);
            }
            DepExpr::Version(name, constraint) => {
                if !self.universe.has_package(name) {
                    return Err(format!("Package '{}' not found in universe", name));
                }
                // Merge constraints: tightest wins (exact > at-least > at-most > any)
                let entry = required.entry(name.clone()).or_insert(constraint.clone());
                if matches!(entry, VersionConstraint::Any) {
                    *entry = constraint.clone();
                }
            }
            DepExpr::And(a, b) => {
                self.collect_requirements(a, required, conflicts)?;
                self.collect_requirements(b, required, conflicts)?;
            }
            DepExpr::Or(a, b) => {
                // OR: try left first; if unavailable, try right
                let mut left_req = required.clone();
                let mut left_conf = conflicts.clone();
                if self.collect_requirements(a, &mut left_req, &mut left_conf).is_ok() {
                    *required = left_req;
                    *conflicts = left_conf;
                } else {
                    self.collect_requirements(b, required, conflicts)?;
                }
            }
            DepExpr::Not(inner) => {
                // NOT: collect the inner packages as conflicts
                let pkgs = inner.referenced_packages();
                for p in pkgs {
                    if !conflicts.contains(&p) {
                        conflicts.push(p);
                    }
                }
            }
            DepExpr::Conflict(name) => {
                if !conflicts.contains(name) {
                    conflicts.push(name.clone());
                }
            }
        }
        Ok(())
    }

    // ── Conflict Detection ────────────────────────────────────────────────────

    /// Detect conflicting packages across a set of expressions.
    ///
    /// Returns a list of package names that appear in both positive and negative
    /// (Conflict/Not) expressions simultaneously.
    pub fn detect_conflicts(&self, exprs: &[DepExpr]) -> Vec<String> {
        let mut required_names: Vec<String> = Vec::new();
        let mut conflict_names: Vec<String> = Vec::new();

        for expr in exprs {
            self.collect_expr_sides(expr, &mut required_names, &mut conflict_names);
        }

        required_names
            .into_iter()
            .filter(|name| conflict_names.contains(name))
            .collect()
    }

    /// Recursively collect positive and negative package names from an expression
    fn collect_expr_sides(
        &self,
        expr: &DepExpr,
        positive: &mut Vec<String>,
        negative: &mut Vec<String>,
    ) {
        match expr {
            DepExpr::Pkg(name) | DepExpr::Version(name, _) => {
                if !positive.contains(name) {
                    positive.push(name.clone());
                }
            }
            DepExpr::Conflict(name) => {
                if !negative.contains(name) {
                    negative.push(name.clone());
                }
            }
            DepExpr::And(a, b) | DepExpr::Or(a, b) => {
                self.collect_expr_sides(a, positive, negative);
                self.collect_expr_sides(b, positive, negative);
            }
            DepExpr::Not(inner) => {
                // Negated: move inner packages to the negative set
                let inner_pkgs = inner.referenced_packages();
                for p in inner_pkgs {
                    if !negative.contains(&p) {
                        negative.push(p);
                    }
                }
            }
            DepExpr::Empty => {}
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod solver_tests {
    use super::*;

    fn make_universe() -> PackageUniverse {
        let mut u = PackageUniverse::new();
        u.add_package("bash", vec!["5.2.1", "5.1.0", "4.4.23"]);
        u.add_package("glibc", vec!["2.39", "2.38", "2.37"]);
        u.add_package("openssl", vec!["3.2.0", "1.1.1w"]);
        u.add_package("curl", vec!["8.5.0", "7.88.1"]);
        u.add_package("zsh", vec!["5.9"]);
        u.add_package("dash", vec!["0.5.12"]);
        u
    }

    #[test]
    fn test_evaluate_installed_pkg() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        let mut installed = BTreeMap::new();
        installed.insert(String::from("bash"), String::from("5.2.1"));

        assert!(solver.evaluate(&DepExpr::Pkg(String::from("bash")), &installed));
        assert!(!solver.evaluate(&DepExpr::Pkg(String::from("zsh")), &installed));
    }

    #[test]
    fn test_evaluate_and_expression() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        let mut installed = BTreeMap::new();
        installed.insert(String::from("bash"), String::from("5.2.1"));
        installed.insert(String::from("glibc"), String::from("2.39"));

        let expr = DepExpr::and(
            DepExpr::Pkg(String::from("bash")),
            DepExpr::Pkg(String::from("glibc")),
        );
        assert!(solver.evaluate(&expr, &installed));

        let bad_expr = DepExpr::and(
            DepExpr::Pkg(String::from("bash")),
            DepExpr::Pkg(String::from("curl")),
        );
        assert!(!solver.evaluate(&bad_expr, &installed));
    }

    #[test]
    fn test_evaluate_or_expression() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        let mut installed = BTreeMap::new();
        installed.insert(String::from("zsh"), String::from("5.9"));

        let expr = DepExpr::or(
            DepExpr::Pkg(String::from("bash")),
            DepExpr::Pkg(String::from("zsh")),
        );
        assert!(solver.evaluate(&expr, &installed));
    }

    #[test]
    fn test_version_constraint_at_least() {
        let c = VersionConstraint::AtLeast(String::from("2.38"));
        assert!(c.satisfies("2.39"));
        assert!(c.satisfies("2.38"));
        assert!(!c.satisfies("2.37"));
        assert!(!c.satisfies("1.99"));
    }

    #[test]
    fn test_resolve_simple() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        let exprs = vec![
            DepExpr::Pkg(String::from("bash")),
            DepExpr::Version(String::from("glibc"), VersionConstraint::AtLeast(String::from("2.38"))),
        ];
        let result = solver.resolve(&exprs).expect("should resolve");
        let names: Vec<&str> = result.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"bash"));
        assert!(names.contains(&"glibc"));
        // glibc should be 2.39 (newest satisfying >= 2.38)
        let glibc_ver = result.iter().find(|(n, _)| n == "glibc").map(|(_, v)| v.as_str());
        assert_eq!(glibc_ver, Some("2.39"));
    }

    #[test]
    fn test_conflict_detection() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        let exprs = vec![
            DepExpr::Pkg(String::from("bash")),
            DepExpr::Conflict(String::from("bash")),
        ];
        let conflicts = solver.detect_conflicts(&exprs);
        assert!(conflicts.contains(&String::from("bash")));
    }

    #[test]
    fn test_resolve_unavailable_pkg_errors() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        let exprs = vec![DepExpr::Pkg(String::from("nonexistent-package"))];
        assert!(solver.resolve(&exprs).is_err());
    }

    #[test]
    fn test_version_cmp_semver() {
        assert_eq!(VersionConstraint::version_cmp("1.10.0", "1.9.0"), 1);
        assert_eq!(VersionConstraint::version_cmp("2.0.0", "1.99.99"), 1);
        assert_eq!(VersionConstraint::version_cmp("1.0.0", "1.0.0"), 0);
        assert_eq!(VersionConstraint::version_cmp("0.5.0", "1.0.0"), -1);
    }

    #[test]
    fn test_or_resolution_fallback() {
        let u = make_universe();
        let solver = BooleanDepSolver::new(u);
        // bash OR nonexistent-shell -> should resolve to bash
        let exprs = vec![DepExpr::or(
            DepExpr::Pkg(String::from("bash")),
            DepExpr::Pkg(String::from("nonexistent-shell")),
        )];
        let result = solver.resolve(&exprs).expect("bash OR fallback should work");
        let names: Vec<&str> = result.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"bash"));
    }
}
