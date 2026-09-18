// Gentoo Portage-Inspired Advanced Dependency Resolution
// Sophisticated dependency solver with USE flags, slot conflicts, and optimization

use std::boxed::Box;
use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Portage-inspired USE flag (compile-time feature selection)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFlag {
    pub name: String,
    pub enabled: bool,
    pub is_global: bool,
    pub description: String,
}

impl UseFlag {
    pub fn new(name: String, enabled: bool) -> Self {
        Self {
            name,
            enabled,
            is_global: false,
            description: String::new(),
        }
    }

    pub fn global(name: String, enabled: bool) -> Self {
        Self {
            name,
            enabled,
            is_global: true,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn negate(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Portage-inspired package slot (allows multiple versions)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot {
    pub name: String,
    pub sub_slot: Option<String>,
}

impl Slot {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sub_slot: None,
        }
    }

    pub fn with_sub_slot(mut self, sub_slot: String) -> Self {
        self.sub_slot = Some(sub_slot);
        self
    }

    pub fn matches(&self, other: &Slot) -> bool {
        if self.name != other.name {
            return false;
        }

        match (&self.sub_slot, &other.sub_slot) {
            (core::option::Option::None, core::option::Option::None) => true,
            (core::option::Option::Some(a), core::option::Option::Some(b)) => a == b,
            _ => false,
        }
    }
}

/// Portage-inspired dependency condition
#[derive(Debug, Clone)]
pub enum DependencyCondition {
    Any,
    All(Vec<DependencyCondition>),
    ExactlyOne(Vec<DependencyCondition>),
    AtLeastOne(Vec<DependencyCondition>),
    UseConditional {
        flag: String,
        enabled: bool,
        condition: Box<DependencyCondition>,
    },
    Package {
        name: String,
        version_constraint: VersionConstraint,
        slot: Option<Slot>,
        use_flags: Vec<UseFlag>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionConstraint {
    Any,
    Greater(Version),
    GreaterEqual(Version),
    Equal(Version),
    LessEqual(Version),
    Less(Version),
    Range(Version, Version),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub revision: u32,
    pub suffix: VersionSuffix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionSuffix {
    None,
    Alpha,
    Beta,
    Rc,
    Pre,
    P, // Patch
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            revision: 0,
            suffix: VersionSuffix::None,
        }
    }

    pub fn with_revision(mut self, revision: u32) -> Self {
        self.revision = revision;
        self
    }

    pub fn with_suffix(mut self, suffix: VersionSuffix) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn satisfies(&self, constraint: &VersionConstraint) -> bool {
        match constraint {
            VersionConstraint::Any => true,
            VersionConstraint::Greater(v) => self > v,
            VersionConstraint::GreaterEqual(v) => self >= v,
            VersionConstraint::Equal(v) => self == v,
            VersionConstraint::LessEqual(v) => self <= v,
            VersionConstraint::Less(v) => self < v,
            VersionConstraint::Range(min, max) => self >= min && self <= max,
        }
    }
}

/// Portage-inspired ebuild-like package specification
#[derive(Debug, Clone)]
pub struct EbuildSpec {
    pub name: String,
    pub version: Version,
    pub slot: Slot,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub use_flags: Vec<UseFlag>,
    pub dependencies: DependencyCondition,
    pub rdependencies: DependencyCondition, // Runtime dependencies
    pub pdependencies: DependencyCondition, // Post-install dependencies
    pub keywords: Vec<String>,              // Architecture keywords
    pub iuse: Vec<String>,                  // Available USE flags
    pub required_use: Option<DependencyCondition>, // Required USE flag combinations
}

impl EbuildSpec {
    pub fn new(name: String, version: Version) -> Self {
        Self {
            name,
            version,
            slot: Slot::new("0".to_string()),
            description: String::new(),
            homepage: String::new(),
            license: String::new(),
            use_flags: Vec::new(),
            dependencies: DependencyCondition::Any,
            rdependencies: DependencyCondition::Any,
            pdependencies: DependencyCondition::Any,
            keywords: Vec::new(),
            iuse: Vec::new(),
            required_use: None,
        }
    }

    pub fn with_slot(mut self, slot: Slot) -> Self {
        self.slot = slot;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_dependencies(mut self, dependencies: DependencyCondition) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn is_stable_for_arch(&self, arch: &str) -> bool {
        self.keywords.contains(&arch.to_string()) || self.keywords.contains(&"*".to_string())
    }
}

/// Portage-inspired dependency resolver
pub struct PortageResolver {
    packages: BTreeMap<String, Vec<EbuildSpec>>, // name -> versions
    installed: BTreeMap<String, (Version, Slot)>, // name -> (version, slot)
    use_flags: BTreeMap<String, bool>,           // Global USE flags
    arch: String,
}

impl PortageResolver {
    pub fn new(arch: String) -> Self {
        Self {
            packages: BTreeMap::new(),
            installed: BTreeMap::new(),
            use_flags: BTreeMap::new(),
            arch,
        }
    }

    pub fn add_package(&mut self, spec: EbuildSpec) {
        self.packages
            .entry(spec.name.clone())
            .or_insert_with(Vec::new)
            .push(spec);
    }

    pub fn set_use_flag(&mut self, flag: String, enabled: bool) {
        self.use_flags.insert(flag, enabled);
    }

    pub fn get_use_flag(&self, flag: &str) -> Option<bool> {
        self.use_flags.get(flag).copied()
    }

    pub fn mark_installed(&mut self, name: String, version: Version, slot: Slot) {
        self.installed.insert(name, (version, slot));
    }

    pub fn mark_uninstalled(&mut self, name: &str) {
        self.installed.remove(name);
    }

    /// Resolve dependencies for a package
    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, DependencyError> {
        let mut resolved = Vec::new();
        let mut visited = BTreeSet::new();
        self.resolve_recursive(package_name, &mut resolved, &mut visited)?;
        Ok(resolved)
    }

    fn resolve_recursive(
        &self,
        package_name: &str,
        resolved: &mut Vec<String>,
        visited: &mut BTreeSet<String>,
    ) -> Result<(), DependencyError> {
        if visited.contains(package_name) {
            return Err(DependencyError::CircularDependency(
                package_name.to_string(),
            ));
        }

        visited.insert(package_name.to_string());

        // Find best matching version
        let versions = self
            .packages
            .get(package_name)
            .ok_or_else(|| DependencyError::PackageNotFound(package_name.to_string()))?;

        let best_version = self.select_best_version(versions)?;

        // Resolve build dependencies
        self.resolve_condition(&best_version.dependencies, resolved, visited)?;

        resolved.push(package_name.to_string());
        visited.remove(package_name);

        Ok(())
    }

    fn resolve_condition(
        &self,
        condition: &DependencyCondition,
        resolved: &mut Vec<String>,
        visited: &mut BTreeSet<String>,
    ) -> Result<(), DependencyError> {
        match condition {
            DependencyCondition::Any => Ok(()),
            DependencyCondition::All(conditions) => {
                for cond in conditions {
                    self.resolve_condition(cond, resolved, visited)?;
                }
                Ok(())
            }
            DependencyCondition::ExactlyOne(conditions) => {
                let mut satisfied = 0;
                for cond in conditions {
                    if self.resolve_condition(cond, resolved, visited).is_ok() {
                        satisfied += 1;
                    }
                }
                if satisfied == 1 {
                    Ok(())
                } else {
                    Err(DependencyError::UnsatisfiedCondition)
                }
            }
            DependencyCondition::AtLeastOne(conditions) => {
                for cond in conditions {
                    if self.resolve_condition(cond, resolved, visited).is_ok() {
                        return Ok(());
                    }
                }
                Err(DependencyError::UnsatisfiedCondition)
            }
            DependencyCondition::UseConditional {
                flag,
                enabled,
                condition,
            } => {
                let flag_enabled = self.get_use_flag(flag).unwrap_or(false);
                if flag_enabled == *enabled {
                    self.resolve_condition(condition, resolved, visited)
                } else {
                    Ok(())
                }
            }
            DependencyCondition::Package {
                name,
                version_constraint,
                slot,
                ..
            } => {
                let versions = self
                    .packages
                    .get(name)
                    .ok_or_else(|| DependencyError::PackageNotFound(name.clone()))?;

                let matching: Vec<_> = versions
                    .iter()
                    .filter(|v| v.version.satisfies(version_constraint))
                    .filter(|v| slot.as_ref().map_or(true, |s| v.slot.matches(s)))
                    .collect();

                if matching.is_empty() {
                    return Err(DependencyError::VersionNotFound(name.clone()));
                }

                let best = self.select_best_version_from_refs(&matching)?;
                self.resolve_recursive(&best.name, resolved, visited)
            }
        }
    }

    fn select_best_version<'a>(
        &self,
        versions: &'a [EbuildSpec],
    ) -> Result<&'a EbuildSpec, DependencyError> {
        let stable: Vec<&'a EbuildSpec> = versions
            .iter()
            .filter(|v| v.is_stable_for_arch(&self.arch))
            .collect();

        if !stable.is_empty() {
            stable
                .into_iter()
                .max_by_key(|v| &v.version)
                .ok_or(DependencyError::NoSuitableVersion)
        } else {
            versions
                .iter()
                .max_by_key(|v| &v.version)
                .ok_or(DependencyError::NoSuitableVersion)
        }
    }

    fn select_best_version_from_refs<'a>(
        &self,
        versions: &[&'a EbuildSpec],
    ) -> Result<&'a EbuildSpec, DependencyError> {
        let stable: Vec<&'a EbuildSpec> = versions
            .iter()
            .copied()
            .filter(|v| v.is_stable_for_arch(&self.arch))
            .collect();

        if !stable.is_empty() {
            stable
                .into_iter()
                .max_by_key(|v| &v.version)
                .ok_or(DependencyError::NoSuitableVersion)
        } else {
            versions
                .iter()
                .copied()
                .max_by_key(|v| &v.version)
                .ok_or(DependencyError::NoSuitableVersion)
        }
    }

    /// Check for slot conflicts
    pub fn check_slot_conflicts(&self, packages: &[String]) -> Vec<SlotConflict> {
        let mut conflicts = Vec::new();
        let mut slot_usage: BTreeMap<String, Vec<(String, Version)>> = BTreeMap::new();

        for package_name in packages {
            if let Some(versions) = self.packages.get(package_name) {
                if let Ok(best) = self.select_best_version(versions) {
                    let slot_key = format!("{}:{}", best.name, best.slot.name);
                    slot_usage
                        .entry(slot_key.clone())
                        .or_insert_with(Vec::new)
                        .push((best.name.clone(), best.version));
                }
            }
        }

        for (slot, packages) in slot_usage {
            if packages.len() > 1 {
                conflicts.push(SlotConflict {
                    slot,
                    packages: packages
                        .iter()
                        .map(|(n, v)| format!("{}-{}", n, v))
                        .collect(),
                });
            }
        }

        conflicts
    }

    /// Optimize USE flags for minimal dependencies
    pub fn optimize_use_flags(&self, package_name: &str) -> Result<Vec<String>, DependencyError> {
        let versions = self
            .packages
            .get(package_name)
            .ok_or_else(|| DependencyError::PackageNotFound(package_name.to_string()))?;

        let best = self.select_best_version(versions)?;
        let mut required_flags = Vec::new();

        if let Some(required_use) = &best.required_use {
            self.extract_required_flags(required_use, &mut required_flags);
        }

        Ok(required_flags)
    }

    fn extract_required_flags(&self, condition: &DependencyCondition, flags: &mut Vec<String>) {
        match condition {
            DependencyCondition::UseConditional {
                flag,
                enabled: true,
                ..
            } => {
                if !flags.contains(flag) {
                    flags.push(flag.clone());
                }
            }
            DependencyCondition::All(conditions) => {
                for cond in conditions {
                    self.extract_required_flags(cond, flags);
                }
            }
            DependencyCondition::AtLeastOne(_conditions) => {
                // For at least one, we can't determine which one to enable
                // In a real implementation, this would be more sophisticated
            }
            _ => {}
        }
    }

    /// Get world file (explicitly installed packages)
    pub fn get_world_set(&self) -> Vec<String> {
        self.installed.keys().cloned().collect()
    }

    /// Calculate installation order (topological sort)
    pub fn calculate_install_order(
        &self,
        packages: &[String],
    ) -> Result<Vec<String>, DependencyError> {
        let mut order = Vec::new();
        let mut visited = BTreeSet::new();
        let mut temp_visited = BTreeSet::new();

        for package in packages {
            self.visit_package(package, &mut order, &mut visited, &mut temp_visited)?;
        }

        Ok(order)
    }

    fn visit_package(
        &self,
        package: &str,
        order: &mut Vec<String>,
        visited: &mut BTreeSet<String>,
        temp_visited: &mut BTreeSet<String>,
    ) -> Result<(), DependencyError> {
        if visited.contains(package) {
            return Ok(());
        }

        if temp_visited.contains(package) {
            return Err(DependencyError::CircularDependency(package.to_string()));
        }

        temp_visited.insert(package.to_string());

        if let Some(versions) = self.packages.get(package) {
            if let Ok(best) = self.select_best_version(versions) {
                self.visit_condition(&best.dependencies, order, visited, temp_visited)?;
            }
        }

        temp_visited.remove(package);
        visited.insert(package.to_string());
        order.push(package.to_string());

        Ok(())
    }

    fn visit_condition(
        &self,
        condition: &DependencyCondition,
        order: &mut Vec<String>,
        visited: &mut BTreeSet<String>,
        temp_visited: &mut BTreeSet<String>,
    ) -> Result<(), DependencyError> {
        match condition {
            DependencyCondition::Package { name, .. } => {
                self.visit_package(name, order, visited, temp_visited)?;
            }
            DependencyCondition::All(conditions) => {
                for cond in conditions {
                    self.visit_condition(cond, order, visited, temp_visited)?;
                }
            }
            DependencyCondition::AtLeastOne(conditions) => {
                // Try to resolve at least one
                for cond in conditions {
                    if self
                        .visit_condition(cond, order, visited, temp_visited)
                        .is_ok()
                    {
                        return Ok(());
                    }
                }
            }
            DependencyCondition::UseConditional {
                condition: cond, ..
            } => {
                self.visit_condition(cond, order, visited, temp_visited)?;
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DependencyError {
    PackageNotFound(String),
    VersionNotFound(String),
    CircularDependency(String),
    UnsatisfiedCondition,
    NoSuitableVersion,
}

#[derive(Debug)]
pub struct SlotConflict {
    pub slot: String,
    pub packages: Vec<String>,
}

/// Portage-inspired emerge operation
pub struct EmergeOperation {
    pub packages: Vec<String>,
    pub world_update: bool,
    pub deep: bool,
    pub clean: bool,
    pub pretend: bool,
    pub verbose: bool,
    pub use_flags: Vec<UseFlag>,
}

impl EmergeOperation {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            world_update: false,
            deep: false,
            clean: false,
            pretend: false,
            verbose: false,
            use_flags: Vec::new(),
        }
    }

    pub fn with_packages(mut self, packages: Vec<String>) -> Self {
        self.packages = packages;
        self
    }

    pub fn world_update(mut self) -> Self {
        self.world_update = true;
        self
    }

    pub fn deep(mut self) -> Self {
        self.deep = true;
        self
    }

    pub fn clean(mut self) -> Self {
        self.clean = true;
        self
    }

    pub fn pretend(mut self) -> Self {
        self.pretend = true;
        self
    }

    pub fn with_use_flag(mut self, flag: UseFlag) -> Self {
        self.use_flags.push(flag);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        assert!(v2 > v1);
    }

    #[test]
    fn test_slot_matching() {
        let slot1 = Slot::new("0".to_string());
        let slot2 = Slot::new("0".to_string());
        assert!(slot1.matches(&slot2));

        let slot3 = Slot::new("0".to_string()).with_sub_slot("1".to_string());
        let slot4 = Slot::new("0".to_string()).with_sub_slot("2".to_string());
        assert!(!slot3.matches(&slot4));
    }

    #[test]
    fn test_dependency_resolution() {
        let mut resolver = PortageResolver::new("amd64".to_string());

        let lib_spec = EbuildSpec::new("libfoo".to_string(), Version::new(1, 0, 0));
        resolver.add_package(lib_spec);

        let app_spec = EbuildSpec::new("myapp".to_string(), Version::new(1, 0, 0))
            .with_dependencies(DependencyCondition::Package {
                name: "libfoo".to_string(),
                version_constraint: VersionConstraint::Any,
                slot: None,
                use_flags: Vec::new(),
            });
        resolver.add_package(app_spec);

        let deps = resolver.resolve_dependencies("myapp").unwrap();
        assert!(deps.contains(&"libfoo".to_string()));
    }

    #[test]
    fn test_use_flag_handling() {
        let mut resolver = PortageResolver::new("amd64".to_string());
        resolver.set_use_flag("gtk".to_string(), true);

        assert_eq!(resolver.get_use_flag("gtk"), Some(true));
    }
}
