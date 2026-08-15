// SAT Solver for Dependency Resolution
// DPLL (Davis-Putnam-Logemann-Loveland) algorithm implementation
// Enhanced with high-performance Debian APT-style pinning and repository priority weighting

#[cfg(not(test))]
use crate::sigpkg::{Package, Version, VersionConstraint};

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[cfg(test)]
impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Exact(Version),
    GreaterThan(Version),
    LessThan(Version),
    GreaterOrEqual(Version),
    LessOrEqual(Version),
    Any,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
}

#[cfg(test)]
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
        }
    }
}

use std::collections::{HashMap, HashSet};

/// elementaryOS-inspired strict reverse-domain and layout validator
#[derive(Debug, Clone)]
pub struct DebianElementaryAppPackage {
    pub app_id: String, // e.g. "io.elementary.calculator"
    pub has_csd_decorations: bool,
    pub prefers_dark_theme: bool,
}

impl DebianElementaryAppPackage {
    pub fn new(app_id: &str, csd: bool, dark: bool) -> Self {
        Self {
            app_id: app_id.to_string(),
            has_csd_decorations: csd,
            prefers_dark_theme: dark,
        }
    }

    /// Validates reverse-domain naming and elementary design standards
    pub fn is_elementary_compliant(&self) -> bool {
        // App ID must be reverse domain starting with "io.elementary." or "org.sigmaos."
        let valid_prefix =
            self.app_id.starts_with("io.elementary.") || self.app_id.starts_with("org.sigmaos.");
        valid_prefix && self.has_csd_decorations
    }
}

/// SAT Solver for dependency resolution
pub struct SatSolver {
    pub packages: HashMap<String, Vec<Package>>,
    pub pin_rules: Vec<AptPinRule>,
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

    /// Add a Debian APT-style preference pinning rule
    pub fn add_pin_rule(&mut self, rule: AptPinRule) {
        self.pin_rules.push(rule);
    }

    /// Selects the best candidate package from a list of versions based on Debian APT pinning rules and version comparison.
    /// Pin priorities below 0 forbid package installations. Default pin priority is 500.
    pub fn select_best_pinned_package(&self, candidate_packages: &[Package]) -> Option<Package> {
        if candidate_packages.is_empty() {
            return None;
        }

        let mut best_candidate: Option<Package> = None;
        let mut best_priority = i16::MIN;

        for package in candidate_packages {
            // Find applicable pin priority rule
            let mut priority = 500; // Default standard Debian pin priority
            for rule in &self.pin_rules {
                if rule.package_name == package.name {
                    // Check if package lists matching origin
                    if package.mirrors.iter().any(|m: &String| m.contains(&rule.origin)) {
                        priority = rule.pin_priority;
                    }
                }
            }

            // Priorities below 0 are ignored/forbid install
            if priority < 0 {
                continue;
            }

            if let Some(ref current_best) = best_candidate {
                if priority > best_priority {
                    best_candidate = Some(package.clone());
                    best_priority = priority;
                } else if priority == best_priority {
                    // Tie-breaker: prefer newer Version (SemVer)
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

    /// Check if an elementaryOS package is fully compliant before resolution
    pub fn is_debian_elementary_package_compliant(&self, app: &DebianElementaryAppPackage) -> bool {
        app.is_elementary_compliant()
    }

    /// Recursive dependency resolution
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

        // Find matching package list
        let packages = self
            .packages
            .get(package_name)
            .ok_or(ResolveError::PackageNotFound(package_name.to_string()))?;

        // Filter versions satisfying constraint
        let valid_candidates: Vec<Package> = packages
            .iter()
            .filter(|p| self.satisfies_constraint(&p.version, version_constraint))
            .cloned()
            .collect();

        if valid_candidates.is_empty() {
            return Err(ResolveError::NoMatchingVersion(package_name.to_string()));
        }

        // Apply high-performance Debian APT pinning logic to select the best weighted version
        let matching_package = self
            .select_best_pinned_package(&valid_candidates)
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

/// Represents a Debian-compatible package targeting elementaryOS Pantheon desktop
#[derive(Debug, Clone)]
pub struct DebianElementaryAppPackage {
    pub app_id: String,             // Must be reverse-domain e.g. "io.elementary.calculator"
    pub format: String,             // Must be "deb" or "flatpak"
    pub adopts_csd_guideline: bool,  // Client-Side Decorations compliance
    pub supports_dark_mode: bool,   // Strict pure-black dark mode compliance
}

impl SatSolver {
    /// Validates if a Debian-style elementary app package is compliant with both Debian package
    /// structure standards and elementaryOS HIG guidelines.
    pub fn is_debian_elementary_package_compliant(&self, package: &DebianElementaryAppPackage) -> Result<bool, &'static str> {
        // 1. Validate reverse-domain app naming (elementaryOS standard)
        let parts: Vec<&str> = package.app_id.split('.').collect();
        if parts.len() < 3 {
            return Err("elementaryOS Package Violation: App ID must follow reverse-domain naming convention (e.g. io.elementary.name)");
        }
        if parts[0] != "io" && parts[0] != "com" && parts[0] != "org" {
            return Err("elementaryOS Package Violation: Invalid app ID top-level domain prefix");
        }

        // 2. Validate Client-Side Decorations (CSD) adoption
        if !package.adopts_csd_guideline {
            return Err("elementaryOS Package Violation: App must adopt Client-Side Decorations (CSD) titlebar rules");
        }

        // 3. Validate toggleable dark mode support
        if !package.supports_dark_mode {
            return Err("elementaryOS Package Violation: App must support toggleable pure-black dark mode");
        }

        Ok(true)
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
    fn test_debian_elementary_compliance() {
        let app1 = DebianElementaryAppPackage::new("io.elementary.calculator", true, true);
        assert!(app1.is_elementary_compliant());

        let app2 = DebianElementaryAppPackage::new("io.elementary.calculator", false, true); // No CSD
        assert!(!app2.is_elementary_compliant());

        let app3 = DebianElementaryAppPackage::new("org.gnome.builder", true, true); // Not elementary or sigmaos
        assert!(!app3.is_elementary_compliant());

        let solver = SatSolver::new();
        assert!(solver.is_debian_elementary_package_compliant(&app1));
        assert!(!solver.is_debian_elementary_package_compliant(&app3));
    }
}
