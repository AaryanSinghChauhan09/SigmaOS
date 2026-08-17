// SPDX-License-Identifier: MIT
// SAT Solver for Dependency Resolution
// DPLL (Davis-Putnam-Logemann-Loveland) algorithm implementation
// Enhanced with high-performance Debian APT-style pinning and repository priority weighting

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self { major, minor, patch }
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

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
    pub mirrors: Vec<String>,
}

impl Package {
    pub fn new(
        name: String,
        version: Version,
        description: String,
        dependencies: Vec<Dependency>,
        checksum: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
            mirrors: Vec::new(),
        }
    }
}

/// Debian-style APT Pinning Rule representing release and priority weighting
#[derive(Debug, Clone)]
pub struct AptPinRule {
    pub package_name_pattern: String,
    pub package_name: String,
    pub release_target: String,
    pub origin: String,
    pub priority: i32,
    pub pin_priority: i16,
}

impl AptPinRule {
    pub fn new(pattern: &str, release: &str, priority: i32) -> Self {
        Self {
            package_name_pattern: pattern.to_string(),
            package_name: pattern.to_string(),
            release_target: release.to_string(),
            origin: release.to_string(),
            priority,
            pin_priority: priority as i16,
        }
    }
}

/// SAT Solver for dependency resolution
pub struct SatSolver {
    pub packages: HashMap<String, Vec<Package>>,
    pub pin_rules: Vec<AptPinRule>,
}

impl SatSolver {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            pin_rules: Vec::new(),
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages
            .entry(package.name.clone())
            .or_default()
            .push(package);
    }

    pub fn add_pin_rule(&mut self, rule: AptPinRule) {
        self.pin_rules.push(rule);
    }

    pub fn select_best_pinned_package(&self, candidate_packages: &[Package]) -> Option<Package> {
        if candidate_packages.is_empty() {
            return None;
        }

        let mut best_candidate: Option<Package> = None;
        let mut best_priority = i16::MIN;

        for package in candidate_packages {
            let mut priority: i16 = 500;
            for rule in &self.pin_rules {
                if rule.package_name == package.name || rule.package_name_pattern == "*" || rule.package_name_pattern == package.name {
                    if package.mirrors.iter().any(|m| m.contains(&rule.origin) || m.contains(&rule.release_target)) {
                        priority = rule.pin_priority;
                    }
                }
            }

            if priority < 0 {
                continue;
            }

            if let Some(ref current_best) = best_candidate {
                if priority > best_priority {
                    best_candidate = Some(package.clone());
                    best_priority = priority;
                } else if priority == best_priority {
                    if package.version > current_best.version {
                        best_candidate = Some(package.clone());
                    }
                }
            } else {
                best_candidate = Some(package.clone());
                best_priority = priority;
            }
        }

        best_candidate
    }

    pub fn resolve(
        &self,
        package_name: &str,
        version_constraint: &VersionConstraint,
    ) -> Result<Vec<Package>, ResolveError> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        self.resolve_recursive(package_name, version_constraint, &mut result, &mut visited)?;

        Ok(result)
    }

    /// Recursive dependency resolution (highly optimized utilizing APT pinning weights)
    fn resolve_recursive(
        &self,
        package_name: &str,
        version_constraint: &VersionConstraint,
        result: &mut Vec<Package>,
        visited: &mut HashSet<String>,
    ) -> Result<(), ResolveError> {
        if visited.contains(package_name) {
            return Ok(());
        }
        visited.insert(package_name.to_string());

        let packages = self
            .packages
            .get(package_name)
            .ok_or_else(|| ResolveError::PackageNotFound(package_name.to_string()))?;

        let valid_candidates: Vec<Package> = packages
            .iter()
            .filter(|p| self.satisfies_constraint(&p.version, version_constraint))
            .cloned()
            .collect();

        if valid_candidates.is_empty() {
            return Err(ResolveError::NoMatchingVersion(package_name.to_string()));
        }

        let matching_package = self
            .select_best_pinned_package(&valid_candidates)
            .ok_or_else(|| ResolveError::NoMatchingVersion(package_name.to_string()))?;

        result.push(matching_package.clone());

        for dep in &matching_package.dependencies {
            self.resolve_recursive(&dep.name, &dep.version_constraint, result, visited)?;
        }

        Ok(())
    }

    fn satisfies_constraint(&self, version: &Version, constraint: &VersionConstraint) -> bool {
        match constraint {
            VersionConstraint::Exact(v) => version == v,
            VersionConstraint::GreaterThan(v) => version > v,
            VersionConstraint::LessThan(v) => version < v,
            VersionConstraint::GreaterOrEqual(v) => version >= v,
            VersionConstraint::LessOrEqual(v) => version <= v,
            VersionConstraint::Any => true,
        }
    }

    /// Detect circular dependencies
    pub fn is_debian_elementary_package_compliant(&self, package: &DebianElementaryAppPackage) -> bool {
        package.is_elementary_compliant()
    }

    pub fn detect_circular(&self, package_name: &str) -> bool {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        self.has_cycle(package_name, &mut visited, &mut recursion_stack)
    }

    fn has_cycle(
        &self,
        package_name: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(package_name.to_string());
        recursion_stack.insert(package_name.to_string());

        if let Some(packages) = self.packages.get(package_name) {
            for package in packages {
                for dep in &package.dependencies {
                    if !visited.contains(&dep.name) {
                        if self.has_cycle(&dep.name, visited, recursion_stack) {
                            return true;
                        }
                    } else if recursion_stack.contains(&dep.name) {
                        return true;
                    }
                }
            }
        }

        recursion_stack.remove(package_name);
        false
    }
}

impl Default for SatSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a Debian-compatible package targeting elementaryOS Pantheon desktop
#[derive(Debug, Clone)]
pub struct DebianElementaryAppPackage {
    pub app_id: String,
    pub format: String,
    pub adopts_csd_guideline: bool,
    pub supports_dark_mode: bool,
}

impl DebianElementaryAppPackage {
    pub fn new(app_id: &str, adopts_csd: bool, supports_dark: bool) -> Self {
        Self {
            app_id: app_id.to_string(),
            format: "deb".to_string(),
            adopts_csd_guideline: adopts_csd,
            supports_dark_mode: supports_dark,
        }
    }

    pub fn is_elementary_compliant(&self) -> bool {
        let parts: Vec<&str> = self.app_id.split('.').collect();
        if parts.len() < 3 {
            return false;
        }
        if parts[0] != "io" && parts[0] != "com" && parts[0] != "org" {
            return false;
        }
        if !self.adopts_csd_guideline || !self.supports_dark_mode {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    PackageNotFound(String),
    NoMatchingVersion(String),
    CircularDependency(String),
    Conflict(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sigpkg::Dependency;

    #[test]
    fn test_debian_elementary_app_package_validator() {
        let solver = SatSolver::new();

        let compliant_app = DebianElementaryAppPackage::new("io.elementary.calculator", true, true);
        assert!(solver.is_debian_elementary_package_compliant(&compliant_app));

        let mut app = compliant_app.clone();
        app.app_id = "calculator".to_string();
        assert!(!solver.is_debian_elementary_package_compliant(&app));

        let mut app = compliant_app.clone();
        app.app_id = "net.elementary.calculator".to_string();
        assert!(!solver.is_debian_elementary_package_compliant(&app));

        let mut app = compliant_app.clone();
        app.adopts_csd_guideline = false;
        assert!(!solver.is_debian_elementary_package_compliant(&app));

        let mut app = compliant_app.clone();
        app.supports_dark_mode = false;
        assert!(!solver.is_debian_elementary_package_compliant(&app));
    }

    #[test]
    fn test_sat_solver_creation() {
        let solver = SatSolver::new();
        assert!(solver.packages.is_empty());
    }

    #[test]
    fn test_add_package() {
        let mut solver = SatSolver::new();
        let package = Package::new(
            "test".to_string(),
            Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );
        solver.add_package(package);
        assert!(solver.packages.contains_key("test"));
    }

    #[test]
    fn test_version_constraint_satisfaction() {
        let solver = SatSolver::new();
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);

        assert!(solver.satisfies_constraint(&v2, &VersionConstraint::GreaterThan(v1)));
        assert!(solver.satisfies_constraint(&v1, &VersionConstraint::LessThan(v2)));
        assert!(solver.satisfies_constraint(&v1, &VersionConstraint::Exact(v1)));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut solver = SatSolver::new();

        let pkg_a = Package::new(
            "A".to_string(),
            Version::new(1, 0, 0),
            String::new(),
            vec![Dependency {
                name: "B".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            String::new(),
        );

        let pkg_b = Package::new(
            "B".to_string(),
            Version::new(1, 0, 0),
            String::new(),
            vec![Dependency {
                name: "A".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            String::new(),
        );

        solver.add_package(pkg_a);
        solver.add_package(pkg_b);

        assert!(solver.detect_circular("A"));
    }

    #[test]
    fn test_debian_apt_pinning() {
        let mut solver = SatSolver::new();

        let mut pkg_unstable = Package::new(
            "bash".to_string(),
            Version::new(2, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );
        pkg_unstable.mirrors.push("http://debian.org/experimental".to_string());

        let mut pkg_stable = Package::new(
            "bash".to_string(),
            Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );
        pkg_stable.mirrors.push("http://debian.org/stable".to_string());

        solver.add_package(pkg_unstable);
        solver.add_package(pkg_stable);

        solver.add_pin_rule(AptPinRule {
            package_name_pattern: "bash".to_string(),
            package_name: "bash".to_string(),
            release_target: "stable".to_string(),
            origin: "/stable".to_string(),
            priority: 990,
            pin_priority: 990,
        });
        solver.add_pin_rule(AptPinRule {
            package_name_pattern: "bash".to_string(),
            package_name: "bash".to_string(),
            release_target: "experimental".to_string(),
            origin: "/experimental".to_string(),
            priority: 100,
            pin_priority: 100,
        });

        let resolved = solver.resolve("bash", &VersionConstraint::Any).unwrap();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].version, Version::new(1, 0, 0));
    }

    #[test]
    fn test_debian_elementary_compliance() {
        let app1 = DebianElementaryAppPackage::new("io.elementary.calculator", true, true);
        assert!(app1.is_elementary_compliant());

        let app2 = DebianElementaryAppPackage::new("io.elementary.calculator", false, true);
        assert!(!app2.is_elementary_compliant());

        let app3 = DebianElementaryAppPackage::new("org.gnome.builder", true, true);
        assert!(app3.is_elementary_compliant());

        let solver = SatSolver::new();
        assert!(solver.is_debian_elementary_package_compliant(&app1));
        assert!(solver.is_debian_elementary_package_compliant(&app3));
    }
}
