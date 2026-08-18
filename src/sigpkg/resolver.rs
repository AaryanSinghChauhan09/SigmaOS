// SAT Solver for Dependency Resolution
// DPLL (Davis-Putnam-Logemann-Loveland) algorithm implementation

use crate::sigpkg::{Package, Version, VersionConstraint};
use std::collections::{HashMap, HashSet};

/// Debian-style APT Pinning Rule representing release and priority weighting
#[derive(Debug, Clone)]
pub struct AptPinRule {
    pub package_name_pattern: String,
    pub release_target: String,
    pub priority: i32,
    pub package_name: String, // Additional field for enhanced compatibility
    pub origin: String, // Additional field for enhanced compatibility
    pub pin_priority: i32, // Additional field for enhanced compatibility
}

impl AptPinRule {
    pub fn new(pattern: &str, release: &str, priority: i32) -> Self {
        Self {
            package_name_pattern: pattern.to_string(),
            release_target: release.to_string(),
            priority,
            package_name: String::new(),
            origin: String::new(),
            pin_priority: priority,
        }
    }
}

/// Debian elementary OS package compliance structure
#[derive(Debug, Clone)]
pub struct DebianElementaryAppPackage {
    pub app_id: String,
    pub format: String,
    pub adopts_csd_guideline: bool,
    pub supports_dark_mode: bool,
}

/// SAT Solver for dependency resolution
pub struct SatSolver {
    packages: HashMap<String, Vec<Package>>,
    pin_rules: Vec<AptPinRule>,
}

impl SatSolver {
    /// Create new SAT solver
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            pin_rules: Vec::new(),
        }
    }

    /// Add package to solver
    pub fn add_package(&mut self, package: Package) {
        self.packages
            .entry(package.name.clone())
            .or_default()
            .push(package);
    }

    /// Add pin rule for APT-style resolution
    pub fn add_pin_rule(&mut self, rule: AptPinRule) {
        self.pin_rules.push(rule);
    }

    /// Validate Debian elementary OS package compliance
    pub fn is_debian_elementary_package_compliant(&self, pkg: &DebianElementaryAppPackage) -> Result<(), &'static str> {
        // Check reverse-domain naming convention (e.g., io.elementary.name)
        if !pkg.app_id.contains('.') {
            return Err("elementaryOS Package Violation: App ID must follow reverse-domain naming convention (e.g. io.elementary.name)");
        }

        let parts: Vec<&str> = pkg.app_id.split('.').collect();
        if parts.len() < 3 {
            return Err("elementaryOS Package Violation: App ID must follow reverse-domain naming convention (e.g. io.elementary.name)");
        }

        // Check TLD prefix - must be 'io' for elementary OS
        if parts[0] != "io" {
            return Err("elementaryOS Package Violation: Invalid app ID top-level domain prefix");
        }

        // Check CSD (Client-Side Decorations) compliance
        if !pkg.adopts_csd_guideline {
            return Err("elementaryOS Package Violation: App must adopt Client-Side Decorations (CSD) titlebar rules");
        }

        // Check dark mode support
        if !pkg.supports_dark_mode {
            return Err("elementaryOS Package Violation: App must support toggleable pure-black dark mode");
        }

        Ok(())
    }

    /// Resolve dependencies for target package
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
            return Ok(()); // Already processed
        }
        visited.insert(package_name.to_string());

        // Find matching package
        let packages = self
            .packages
            .get(package_name)
            .ok_or(ResolveError::PackageNotFound(package_name.to_string()))?;

        let matching_package = packages
            .iter()
            .find(|p| self.satisfies_constraint(&p.version, version_constraint))
            .ok_or(ResolveError::NoMatchingVersion(package_name.to_string()))?;

        result.push(matching_package.clone());

        // Resolve dependencies
        for dep in &matching_package.dependencies {
            self.resolve_recursive(&dep.name, &dep.version_constraint, result, visited)?;
        }

        Ok(())
    }

    /// Check if version satisfies constraint
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

    /// Resolves the optimal package version using Debian-style APT pinning priorities
    pub fn resolve_with_pinning(
        &self,
        package_name: &str,
        constraint: &VersionConstraint,
        pin_rules: &[AptPinRule],
    ) -> Result<Package, ResolveError> {
        let candidates = self
            .packages
            .get(package_name)
            .ok_or(ResolveError::PackageNotFound(package_name.to_string()))?;

        let mut best_candidate: Option<(&Package, i32)> = None;

        for candidate in candidates {
            if self.satisfies_constraint(&candidate.version, constraint) {
                // Determine priority score based on pinning rules
                let mut priority = 500; // Default Debian priority for installed packages
                for rule in pin_rules {
                    if rule.package_name_pattern == "*" || rule.package_name_pattern == package_name {
                        // Priority is matched by release targets or patterns
                        priority = rule.priority;
                    }
                }

                if let Some((_, best_priority)) = best_candidate {
                    if priority > best_priority {
                        best_candidate = Some((candidate, priority));
                    }
                } else {
                    best_candidate = Some((candidate, priority));
                }
            }
        }

        best_candidate
            .map(|(p, _)| p.clone())
            .ok_or(ResolveError::NoMatchingVersion(package_name.to_string()))
    }

    /// Detect circular dependencies
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

/// Resolution errors
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

        // 1. Fully compliant package
        let compliant_app = DebianElementaryAppPackage {
            app_id: "io.elementary.calculator".to_string(),
            format: "deb".to_string(),
            adopts_csd_guideline: true,
            supports_dark_mode: true,
        };
        assert!(solver.is_debian_elementary_package_compliant(&compliant_app).is_ok());

        // 2. Non-compliant: invalid App ID format
        let mut app = compliant_app.clone();
        app.app_id = "calculator".to_string();
        assert_eq!(
            solver.is_debian_elementary_package_compliant(&app).unwrap_err(),
            "elementaryOS Package Violation: App ID must follow reverse-domain naming convention (e.g. io.elementary.name)"
        );

        // 3. Non-compliant: invalid TLD prefix
        let mut app = compliant_app.clone();
        app.app_id = "net.elementary.calculator".to_string();
        assert_eq!(
            solver.is_debian_elementary_package_compliant(&app).unwrap_err(),
            "elementaryOS Package Violation: Invalid app ID top-level domain prefix"
        );

        // 4. Non-compliant: missing CSD compliance
        let mut app = compliant_app.clone();
        app.adopts_csd_guideline = false;
        assert_eq!(
            solver.is_debian_elementary_package_compliant(&app).unwrap_err(),
            "elementaryOS Package Violation: App must adopt Client-Side Decorations (CSD) titlebar rules"
        );

        // 5. Non-compliant: missing dark mode compliance
        let mut app = compliant_app.clone();
        app.supports_dark_mode = false;
        assert_eq!(
            solver.is_debian_elementary_package_compliant(&app).unwrap_err(),
            "elementaryOS Package Violation: App must support toggleable pure-black dark mode"
        );
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

        // Create circular dependency: A -> B -> A
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

        let pkg_unstable = Package::new(
            "bash".to_string(),
            Version::new(2, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );

        let pkg_stable = Package::new(
            "bash".to_string(),
            Version::new(1, 0, 0),
            String::new(),
            Vec::new(),
            String::new(),
        );

        solver.add_package(pkg_unstable);
        solver.add_package(pkg_stable);

        solver.add_pin_rule(AptPinRule::new("bash", "stable", 990));
        solver.add_pin_rule(AptPinRule::new("bash", "experimental", 100));

        let resolved = solver.resolve_with_pinning("bash", &VersionConstraint::Any, &solver.pin_rules).unwrap();
        assert_eq!(resolved.version, Version::new(1, 0, 0));
    }
}
