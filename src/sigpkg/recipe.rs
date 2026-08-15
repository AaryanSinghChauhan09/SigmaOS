// SigmaOS Package Recipes (Gentoo Ebuild, Arch PKGBUILD, and FreeBSD Ports parity)
// Build recipes for package compilation, USE flags, conditional dependencies, checksum arrays, and licenses.

#![no_std]

#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
use alloc::vec::Vec;

#[cfg(test)]
mod test_types {
    use alloc::string::String;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    impl core::fmt::Display for Version {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
    #[derive(Debug, Clone)]
    pub struct Dependency {
        pub name: String,
        pub version_constraint: VersionConstraint,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum VersionConstraint {
        Any,
    }
}

#[cfg(test)]
use test_types::{Version, Dependency, VersionConstraint};

#[cfg(not(test))]
use crate::sigpkg::{Dependency, Version};

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// Build system type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSystem {
    Cargo,
    Make,
    CMake,
    Autotools,
    Meson,
    Ninja,
}

/// Package recipe with Gentoo, Arch, and Ports capabilities
#[derive(Debug, Clone)]
pub struct PackageRecipe {
    pub name: String,
    pub version: Version,
    pub description: String,
    pub build_system: BuildSystem,
    pub dependencies: Vec<Dependency>,
    pub source_url: String,
    pub hash: String,
    pub build_commands: Vec<String>,
    pub install_commands: Vec<String>,
    pub environment: HashMap<String, String>,

    // Gentoo-style USE flags and conditional dependencies
    pub use_flags: HashMap<String, bool>,
    pub conditional_dependencies: Vec<(String, Dependency)>, // use_flag -> conditional dependency

    // Arch Linux-style checksum arrays & supported architectures
    pub sha256sums: Vec<String>,
    pub supported_archs: Vec<String>,

    // FreeBSD ports-style licenses and config options
    pub license: String,
}

impl PackageRecipe {
    pub fn new(name: String, version: Version) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            build_system: BuildSystem::Cargo,
            dependencies: Vec::new(),
            source_url: String::new(),
            hash: String::new(),
            build_commands: Vec::new(),
            install_commands: Vec::new(),
            environment: HashMap::new(),
            use_flags: HashMap::new(),
            conditional_dependencies: Vec::new(),
            sha256sums: Vec::new(),
            supported_archs: Vec::new(),
            license: "MIT".to_string(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_build_system(mut self, build_system: BuildSystem) -> Self {
        self.build_system = build_system;
        self
    }

    pub fn with_source(mut self, url: String, hash: String) -> Self {
        self.source_url = url;
        self.sha256sums.push(hash.clone());
        self.hash = hash;
        self
    }

    pub fn with_dependency(mut self, dependency: Dependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    pub fn with_build_command(mut self, command: String) -> Self {
        self.build_commands.push(command);
        self
    }

    pub fn with_install_command(mut self, command: String) -> Self {
        self.install_commands.push(command);
        self
    }

    pub fn with_prepare_command(mut self, command: String) -> Self {
        self.build_commands.insert(0, command);
        self
    }

    pub fn with_pkgrel(self, _pkgrel: u32) -> Self {
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    /// Gentoo ebuild builder helper: registers a USE flag with its default state
    pub fn with_use_flag(mut self, flag_name: &str, enabled: bool) -> Self {
        self.use_flags.insert(flag_name.to_string(), enabled);
        self
    }

    /// Gentoo ebuild builder helper: registers a dependency activated only if a specific USE flag is enabled
    pub fn with_conditional_dependency(mut self, use_flag: &str, dep: Dependency) -> Self {
        self.conditional_dependencies.push((use_flag.to_string(), dep));
        self
    }

    /// Arch-style builder helper: registers multiple supported architectures
    pub fn with_arch(mut self, arch: &str) -> Self {
        self.supported_archs.push(arch.to_string());
        self
    }

    /// Ports-style builder helper: configures package license type
    pub fn with_license(mut self, lic: &str) -> Self {
        self.license = lic.to_string();
        self
    }

    pub fn validate(&self) -> Result<(), RecipeError> {
        if self.name.is_empty() {
            return Err(RecipeError::InvalidName);
        }
        if self.source_url.is_empty() {
            return Err(RecipeError::InvalidSource);
        }
        if self.hash.is_empty() {
            return Err(RecipeError::InvalidHash);
        }
        if self.build_commands.is_empty() {
            return Err(RecipeError::NoBuildCommands);
        }
        Ok(())
    }

    pub fn get_build_script(&self) -> String {
        match self.build_system {
            BuildSystem::Cargo => "cargo build --release\ncargo install --path .".to_string(),
            BuildSystem::Make => "make -j$(nproc)\nmake install".to_string(),
            BuildSystem::CMake => {
                "mkdir -p build\ncd build\ncmake ..\nmake -j$(nproc)\nmake install".to_string()
            }
            BuildSystem::Autotools => "./configure\nmake -j$(nproc)\nmake install".to_string(),
            BuildSystem::Meson => {
                "meson setup build\nmeson compile -C build\nmeson install -C build".to_string()
            }
            BuildSystem::Ninja => "ninja\nninja install".to_string(),
        }
    }

    /// Returns all active dependencies, consolidating static and active conditional USE flag dependencies
    pub fn resolve_active_dependencies(&self) -> Vec<Dependency> {
        let mut active: Vec<Dependency> = self.dependencies.clone();
        for (flag, dep) in &self.conditional_dependencies {
            if let Some(&true) = self.use_flags.get(flag) {
                active.push(dep.clone());
            }
        }
        active
    }
}

/// Recipe errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeError {
    InvalidName,
    InvalidSource,
    InvalidHash,
    NoBuildCommands,
    DependencyConflict,
    BuildFailed,
}

/// Recipe manager
pub struct RecipeManager {
    recipes: HashMap<String, PackageRecipe>,
}

impl RecipeManager {
    pub fn new() -> Self {
        Self {
            recipes: HashMap::new(),
        }
    }

    pub fn add_recipe(&mut self, recipe: PackageRecipe) -> Result<(), RecipeError> {
        recipe.validate()?;
        let key = format!("{}@{}", recipe.name, recipe.version);
        self.recipes.insert(key, recipe);
        Ok(())
    }

    pub fn get_recipe(&self, name: &str, version: &Version) -> Option<&PackageRecipe> {
        let key = format!("{}@{}", name, version);
        self.recipes.get(&key)
    }

    pub fn list_recipes(&self) -> Vec<&PackageRecipe> {
        self.recipes.values().collect()
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&PackageRecipe> {
        self.recipes.values().filter(|r| r.name == name).collect()
    }

    pub fn remove_recipe(&mut self, name: &str, version: &Version) {
        let key = format!("{}@{}", name, version);
        self.recipes.remove(&key);
    }
}

impl Default for RecipeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_creation() {
        let recipe = PackageRecipe::new("test".to_string(), Version::new(1, 0, 0));
        assert_eq!(recipe.name, "test");
    }

    #[test]
    fn test_recipe_builder() {
        let recipe = PackageRecipe::new("test".to_string(), Version::new(1, 0, 0))
            .with_description("Test package".to_string())
            .with_build_system(BuildSystem::Cargo)
            .with_source("https://example.com".to_string(), "abc123".to_string())
            .with_build_command("cargo build".to_string());

        assert_eq!(recipe.description, "Test package");
        assert_eq!(recipe.build_system, BuildSystem::Cargo);
    }

    #[test]
    fn test_recipe_validation() {
        let recipe = PackageRecipe::new("test".to_string(), Version::new(1, 0, 0))
            .with_source("https://example.com".to_string(), "abc123".to_string())
            .with_build_command("cargo build".to_string());

        assert!(recipe.validate().is_ok());
    }

    #[test]
    fn test_invalid_recipe() {
        let recipe = PackageRecipe::new("".to_string(), Version::new(1, 0, 0));
        assert!(recipe.validate().is_err());
    }

    #[test]
    fn test_recipe_manager() {
        let mut manager = RecipeManager::new();
        let recipe = PackageRecipe::new("test".to_string(), Version::new(1, 0, 0))
            .with_source("https://example.com".to_string(), "abc123".to_string())
            .with_build_command("cargo build".to_string());

        assert!(manager.add_recipe(recipe).is_ok());
        assert_eq!(manager.list_recipes().len(), 1);
    }

    #[test]
    fn test_build_script_generation() {
        let recipe = PackageRecipe::new("test".to_string(), Version::new(1, 0, 0))
            .with_build_system(BuildSystem::Cargo);

        let script = recipe.get_build_script();
        assert!(script.contains("cargo build"));
    }

    #[test]
    fn test_distro_inspired_ebuild_ports_features() {
        let ssl_dep = Dependency {
            name: "openssl".to_string(),
            version_constraint: VersionConstraint::Any,
        };
        let mut recipe = PackageRecipe::new("nginx-sovereign".to_string(), Version::new(1, 25, 0))
            .with_source("https://nginx.org/nginx-1.25.0.tar.gz".to_string(), "sha-abc-123".to_string())
            .with_build_command("make".to_string())
            .with_license("BSD-2-Clause")
            .with_arch("x86_64")
            .with_arch("aarch64")
            // 1. Gentoo ebuild USE flags
            .with_use_flag("ssl", true)
            .with_use_flag("http2", false)
            // 2. Conditional ebuild dependencies
            .with_conditional_dependency("ssl", ssl_dep);

        // Under enabled 'ssl' flag, should pull in openssl dependency
        let active_deps = recipe.resolve_active_dependencies();
        assert_eq!(active_deps.len(), 1);
        assert_eq!(active_deps[0].name, "openssl");

        // Toggle 'ssl' flag off -> conditional dependency is no longer active
        recipe.use_flags.insert("ssl".to_string(), false);
        let active_deps_off = recipe.resolve_active_dependencies();
        assert_eq!(active_deps_off.len(), 0);

        // Verify Arch & Ports metadata
        assert_eq!(recipe.license, "BSD-2-Clause");
        assert_eq!(recipe.supported_archs.len(), 2);
        assert_eq!(recipe.supported_archs[1], "aarch64");
        assert_eq!(recipe.sha256sums[0], "sha-abc-123");
    }
}
