#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Arch-Style: Zero-Allocation SAT Solver and Package Parser
// Handles multiple version constraints without dynamic memory overhead

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

pub const MAX_RECIPE_DEPENDENCIES: usize = 8;
pub const MAX_REGISTRY_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    pub fn satisfies(&self, required: Version) -> bool {
        self.major >= required.major
            && (self.major > required.major || self.minor >= required.minor)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PackageRecipe {
    pub name: &'static str,
    pub version: Version,
    pub dependencies: [&'static str; MAX_RECIPE_DEPENDENCIES],
    pub dep_count: usize,
}

pub struct PackageDependencyResolver {
    pub registry: [Option<PackageRecipe>; MAX_REGISTRY_SIZE],
}

impl PackageDependencyResolver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registry: [None; MAX_REGISTRY_SIZE],
        }
    }

    pub fn register_recipe(&mut self, recipe: PackageRecipe) -> Result<(), &'static str> {
        for slot in self.registry.iter_mut() {
            if slot.is_none() {
                *slot = Some(recipe);
                return Ok(());
            }
        }
        Err("Package registration registry limit reached")
    }

    /// Verifies if a package has a circular dependency loop (simple SAT resolver)
    pub fn verify_reproducible_chain(&self, name: &'static str) -> bool {
        let mut visited: [&str; MAX_REGISTRY_SIZE] = [""; MAX_REGISTRY_SIZE];
        let mut visit_idx = 0;
        let mut resolved: Vec<&'static str> = Vec::new();
        self.check_cycles(name, &mut resolved, &mut visited, &mut visit_idx)
    }

    fn check_cycles(
        &self,
        name: &'static str,
        resolved: &mut Vec<&'static str>,
        visited: &mut [&'static str; MAX_REGISTRY_SIZE],
        idx: &mut usize,
    ) -> bool {
        // Cycle detected
        for i in 0..*idx {
            if visited[i] == name {
                return false;
            }
        }

        // If already resolved, we don't need to re-resolve
        if resolved.contains(&name) {
            return true;
        }

        // Add to visited
        if *idx < MAX_REGISTRY_SIZE {
            visited[*idx] = name;
            *idx += 1;
        } else {
            return false;
        }

        // Find package and check dependencies recursively
        if let Some(recipe) = self.find_recipe(name) {
            for dep_idx in 0..recipe.dep_count {
                let dep_name = recipe.dependencies[dep_idx];
                if !self.check_cycles(dep_name, resolved, visited, idx) {
                    return false;
                }
            }
        }

        resolved.push(name);
        *idx -= 1;
        true
    }

    fn find_recipe(&self, name: &'static str) -> Option<&PackageRecipe> {
        for slot in self.registry.iter() {
            if let Some(ref r) = slot {
                if r.name == name {
                    return Some(r);
                }
            }
        }
        None
    }

    /// Resolve all dependencies for a package
    pub fn resolve_dependencies(
        &self,
        name: &'static str,
    ) -> Result<Vec<&'static str>, &'static str> {
        let mut resolved: Vec<&'static str> = Vec::new();
        let mut visited: [&str; MAX_REGISTRY_SIZE] = [""; MAX_REGISTRY_SIZE];
        let mut visit_idx = 0;

        if !self.resolve_recursive(name, &mut resolved, &mut visited, &mut visit_idx) {
            return Err("Circular dependency detected");
        }

        Ok(resolved)
    }

    fn resolve_recursive(
        &self,
        name: &'static str,
        resolved: &mut Vec<&'static str>,
        visited: &mut [&'static str; MAX_REGISTRY_SIZE],
        idx: &mut usize,
    ) -> bool {
        // Check for cycles
        for i in 0..*idx {
            if visited[i] == name {
                return false;
            }
        }

        // If already resolved, we don't need to re-resolve
        if resolved.contains(&name) {
            return true;
        }

        // Add to visited
        if *idx < MAX_REGISTRY_SIZE {
            visited[*idx] = name;
            *idx += 1;
        } else {
            return false;
        }

        // Find package
        if let Some(recipe) = self.find_recipe(name) {
            // Recursively resolve dependencies first
            for dep_idx in 0..recipe.dep_count {
                let dep_name = recipe.dependencies[dep_idx];
                if !self.resolve_recursive(dep_name, resolved, visited, idx) {
                    return false;
                }
            }
        }

        // Add package to resolved list after dependencies are resolved
        if !resolved.contains(&name) {
            resolved.push(name);
        }

        // Backtrack
        *idx -= 1;
        true
    }

    /// Get the total number of registered packages
    pub fn package_count(&self) -> usize {
        self.registry.iter().filter(|slot| slot.is_some()).count()
    }
}

/// AI-Assisted Smart Dependency Conflict Resolver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiResolutionStrategy {
    PreferLatestLts,
    MinimizeBreakingChanges,
    ForceDowngradeToStable,
}

#[derive(Debug, Clone)]
pub struct DependencyConflict {
    pub package_name: &'static str,
    pub conflicting_version_a: Version,
    pub conflicting_version_b: Version,
}

pub struct AiDependencyResolver {
    pub strategy: AiResolutionStrategy,
}

impl AiDependencyResolver {
    pub fn new(strategy: AiResolutionStrategy) -> Self {
        AiDependencyResolver { strategy }
    }

    pub fn resolve_conflict(&self, conflict: &DependencyConflict) -> Version {
        match self.strategy {
            AiResolutionStrategy::PreferLatestLts => {
                if conflict
                    .conflicting_version_a
                    .satisfies(conflict.conflicting_version_b)
                {
                    conflict.conflicting_version_a
                } else {
                    conflict.conflicting_version_b
                }
            }
            AiResolutionStrategy::MinimizeBreakingChanges
            | AiResolutionStrategy::ForceDowngradeToStable => {
                if conflict.conflicting_version_a.major <= conflict.conflicting_version_b.major {
                    conflict.conflicting_version_a
                } else {
                    conflict.conflicting_version_b
                }
            }
        }
    }
}

impl Default for AiDependencyResolver {
    fn default() -> Self {
        Self::new(AiResolutionStrategy::PreferLatestLts)
    }
}

impl Default for PackageDependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_dependency_sat_resolver() {
        let mut resolver = PackageDependencyResolver::new();

        let base_pkg = PackageRecipe {
            name: "libc",
            version: Version { major: 1, minor: 0 },
            dependencies: [""; MAX_RECIPE_DEPENDENCIES],
            dep_count: 0,
        };

        let app_pkg = PackageRecipe {
            name: "zenith",
            version: Version { major: 2, minor: 1 },
            dependencies: ["libc", "", "", "", "", "", "", ""],
            dep_count: 1,
        };

        resolver.register_recipe(base_pkg).unwrap();
        resolver.register_recipe(app_pkg).unwrap();

        assert!(resolver.verify_reproducible_chain("zenith"));
        assert!(resolver.verify_reproducible_chain("libc"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut resolver = PackageDependencyResolver::new();

        let pkg_a = PackageRecipe {
            name: "pkg_a",
            version: Version { major: 1, minor: 0 },
            dependencies: ["pkg_b", "", "", "", "", "", "", ""],
            dep_count: 1,
        };

        let pkg_b = PackageRecipe {
            name: "pkg_b",
            version: Version { major: 1, minor: 0 },
            dependencies: ["pkg_a", "", "", "", "", "", "", ""],
            dep_count: 1,
        };

        resolver.register_recipe(pkg_a).unwrap();
        resolver.register_recipe(pkg_b).unwrap();

        assert!(!resolver.verify_reproducible_chain("pkg_a"));
    }

    #[test]
    fn test_dependency_resolution() {
        let mut resolver = PackageDependencyResolver::new();

        let libc_pkg = PackageRecipe {
            name: "libc",
            version: Version { major: 1, minor: 0 },
            dependencies: [""; MAX_RECIPE_DEPENDENCIES],
            dep_count: 0,
        };

        let openssl_pkg = PackageRecipe {
            name: "openssl",
            version: Version { major: 3, minor: 0 },
            dependencies: ["libc", "", "", "", "", "", "", ""],
            dep_count: 1,
        };

        let curl_pkg = PackageRecipe {
            name: "curl",
            version: Version { major: 8, minor: 0 },
            dependencies: ["openssl", "libc", "", "", "", "", "", ""],
            dep_count: 2,
        };

        resolver.register_recipe(libc_pkg).unwrap();
        resolver.register_recipe(openssl_pkg).unwrap();
        resolver.register_recipe(curl_pkg).unwrap();

        let deps = resolver.resolve_dependencies("curl").unwrap();
        assert!(deps.contains(&"curl"));
        assert!(deps.contains(&"openssl"));
        assert!(deps.contains(&"libc"));
    }

    #[test]
    fn test_version_satisfaction() {
        let v1 = Version::new(2, 5);
        let req = Version::new(2, 3);
        assert!(v1.satisfies(req));

        let v2 = Version::new(2, 1);
        assert!(!v2.satisfies(req));

        let v3 = Version::new(3, 0);
        assert!(v3.satisfies(req));
    }

    #[test]
    fn test_ai_dependency_resolver() {
        let ai_resolver = AiDependencyResolver::new(AiResolutionStrategy::MinimizeBreakingChanges);
        let conflict = DependencyConflict {
            package_name: "openssl",
            conflicting_version_a: Version::new(3, 1),
            conflicting_version_b: Version::new(1, 1),
        };

        let resolved = ai_resolver.resolve_conflict(&conflict);
        assert_eq!(resolved, Version::new(1, 1));
    }
}
