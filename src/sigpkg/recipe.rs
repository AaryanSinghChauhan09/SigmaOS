use alloc::vec;
use alloc::format;
extern crate alloc;
// SigmaOS Package Recipes
// Build recipes for package compilation and installation
// Improved with Gentoo Portage-style USE flags and dynamic stage compilation profiles.

use crate::sigpkg::{Dependency, Version, VersionConstraint};
use crate::klib::collections::HashMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::default::Default;
use core::option::Option::{self, Some, None};
use core::result::Result::{self, Ok, Err};

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

/// Gentoo-inspired compilation optimization profiles (stages)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageProfile {
    Stage1Minimal,   // Basic fallback bootstrap flags (-O1, -mno-sse)
    Stage2Bootstrap, // Balanced standard optimization (-O2)
    Stage3Optimized, // Maximum architecture-targeted performance (-O3 -march=native -flto)
}

/// Gentoo-style Portage USE flags representing conditional package compilation features
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseFlag {
    Ssl,
    Threads,
    X11,
    Gpu,
    Sound,
}

/// Package recipe
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

    // Gentoo-inspired features
    pub active_use_flags: Vec<UseFlag>,
    pub compilation_profile: StageProfile,
    pub conditional_dependencies: Vec<(UseFlag, Dependency)>, // Dependency unlocked ONLY if USE flag is active

    // Arch Linux compatibility fields
    pub arch: String,
    pub pkgrel: String,
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
            active_use_flags: Vec::new(),
            compilation_profile: StageProfile::Stage2Bootstrap,
            conditional_dependencies: Vec::new(),
            arch: "x86_64".to_string(),
            pkgrel: "1".to_string(),
        }
    }

    pub fn with_arch(mut self, arch: String) -> Self {
        self.arch = arch;
        self
    }

    pub fn with_pkgrel(mut self, pkgrel: String) -> Self {
        self.pkgrel = pkgrel;
        self
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

    pub fn with_prepare_command(mut self, command: String) -> Self {
        self.build_commands.push(command);
        self
    }

    pub fn with_install_command(mut self, command: String) -> Self {
        self.install_commands.push(command);
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    // Builder helpers for USE flags and compilation profiles
    pub fn with_use_flag(mut self, flag: UseFlag) -> Self {
        self.active_use_flags.push(flag);
        self
    }

    pub fn with_compilation_profile(mut self, profile: StageProfile) -> Self {
        self.compilation_profile = profile;
        self
    }

    pub fn with_conditional_dependency(mut self, flag: UseFlag, dependency: Dependency) -> Self {
        self.conditional_dependencies.push((flag, dependency));
        self
    }

    pub fn is_use_active(&self, flag: UseFlag) -> bool {
        self.active_use_flags.contains(&flag)
    }

    /// Evaluates and returns all active dependencies including conditional USE-flag targets
    pub fn get_active_dependencies(&self) -> Vec<Dependency> {
        let mut deps = self.dependencies.clone();
        for (flag, dep) in self.conditional_dependencies.iter() {
            if self.is_use_active(*flag) {
                deps.push(dep.clone());
            }
        }
        deps
    }

    /// Returns CFLAGS/CXXFLAGS compilation flags matching the active Stage Profile
    pub fn get_stage_optimization_flags(&self) -> &'static str {
        match self.compilation_profile {
            StageProfile::Stage1Minimal => "-O1 -mno-sse2",
            StageProfile::Stage2Bootstrap => "-O2 -pipe",
            StageProfile::Stage3Optimized => "-O3 -march=native -flto=fat -funroll-loops",
        }
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

/// Kernel header sysroot manager for cross-distro package compilation compatibility
#[derive(Debug, Clone)]
pub struct KernelHeaderSysroot {
    pub include_path: String,
    pub active_headers: Vec<String>,
}

impl KernelHeaderSysroot {
    pub fn default_sysroot() -> Self {
        Self {
            include_path: "include/".to_string(),
            active_headers: vec![
                "sigma_kernel.h".to_string(),
                "sigma_abi.h".to_string(),
                "sigma_kmod.h".to_string(),
                "sigma_vfs.h".to_string(),
                "sigma_net.h".to_string(),
                "sigma_sched.h".to_string(),
            ],
        }
    }

    pub fn validate_sysroot_headers(&self) -> bool {
        self.active_headers.iter().all(|hdr| {
            let _path = format!("{}{}", self.include_path, hdr);
            false
        })
    }

    pub fn get_cflags(&self) -> String {
        format!("-I{} -D__SIGMAOS_KERNEL__", self.include_path)
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
    fn test_portage_style_use_flags() {
        let mut recipe = PackageRecipe::new("libcurl".to_string(), Version::new(8, 2, 1))
            .with_source("https://example.com/curl".to_string(), "99aa88".to_string())
            .with_build_command("make".to_string());

        // Setup conditional openssl dependency if "Ssl" USE flag is toggled active
        let ssl_dependency = Dependency {
            name: "openssl".to_string(),
            version_constraint: VersionConstraint::GreaterOrEqual(Version::new(3, 0, 0)),
        };

        recipe = recipe.with_conditional_dependency(UseFlag::Ssl, ssl_dependency);

        // 1. By default, "Ssl" is inactive, so no conditional dependency is fetched
        assert!(!recipe.is_use_active(UseFlag::Ssl));
        assert_eq!(recipe.get_active_dependencies().len(), 0);

        // 2. Toggle "Ssl" active
        recipe = recipe.with_use_flag(UseFlag::Ssl);
        assert!(recipe.is_use_active(UseFlag::Ssl));

        let active_deps = recipe.get_active_dependencies();
        assert_eq!(active_deps.len(), 1);
        assert_eq!(active_deps[0].name, "openssl");
    }

    #[test]
    fn test_portage_stage_optimization_flags() {
        let mut recipe = PackageRecipe::new("kernel".to_string(), Version::new(6, 1, 0));
        assert_eq!(recipe.get_stage_optimization_flags(), "-O2 -pipe"); // default Stage2

        recipe = recipe.with_compilation_profile(StageProfile::Stage3Optimized);
        assert_eq!(
            recipe.get_stage_optimization_flags(),
            "-O3 -march=native -flto=fat -funroll-loops"
        );
    }

    #[test]
    fn test_kernel_header_sysroot_package_compat() {
        let sysroot = KernelHeaderSysroot::default_sysroot();
        assert!(sysroot.validate_sysroot_headers());
        assert_eq!(sysroot.get_cflags(), "-Iinclude/ -D__SIGMAOS_KERNEL__");
    }
}
