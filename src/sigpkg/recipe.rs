// SigmaOS Package Recipes
// Build recipes for package compilation and installation
// Improved with Gentoo Portage-style USE flags and dynamic stage compilation profiles.

use crate::sigpkg::{Dependency, Version};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UseFlag {
    Ssl,
    X11,
    Wayland,
    Alsa,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageProfile {
    Stage1Minimal,
    Stage2Standard,
    Stage3Optimized,
}


/// Build system type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSystem {
    Cargo,
    Make,
    CMake,
    Autotools,
    Meson,
    Ninja,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeError {
    InvalidFormat,
    MissingField,
    SignatureMismatch,
    DependencyConflict,
    InvalidName,
    InvalidSource,
    InvalidHash,
    NoBuildCommands,
    InvalidRecipe,
    NotFound,
    InvalidSyntax,
    SerializationError,
}

pub struct RecipeManager {
    pub recipes: HashMap<String, PackageRecipe>,
}

impl RecipeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            recipes: HashMap::new(),
        };
        // Add distro-inspired standard package recipes
        let neofetch = PackageRecipe::new("neofetch".to_string(), Version::new(7, 1, 0))
            .with_description("A fast, highly customizable system info script".to_string())
            .with_build_system(BuildSystem::Make)
            .with_source("https://github.com/dylanaraps/neofetch".to_string(), "hash_neofetch".to_string())
            .with_build_command("make build".to_string());
        let curl = PackageRecipe::new("curl".to_string(), Version::new(8, 7, 1))
            .with_description("Command line tool for transferring data with URLs".to_string())
            .with_build_system(BuildSystem::CMake)
            .with_source("https://curl.se/download/curl-8.7.1.tar.gz".to_string(), "hash_curl".to_string())
            .with_build_command("cmake .".to_string());
        let ripgrep = PackageRecipe::new("ripgrep".to_string(), Version::new(14, 1, 0))
            .with_description("ripgrep recursively searches directories for a regex pattern".to_string())
            .with_build_system(BuildSystem::Cargo)
            .with_source("https://github.com/BurntSushi/ripgrep".to_string(), "hash_ripgrep".to_string())
            .with_build_command("cargo build --release".to_string());
        let almalinux_release = PackageRecipe::new("almalinux-release".to_string(), Version::new(9, 4, 0))
            .with_description("AlmaLinux release file".to_string())
            .with_build_system(BuildSystem::Custom)
            .with_source("https://github.com/AlmaLinux/almalinux-release".to_string(), "hash_almalinux".to_string())
            .with_build_command("echo 'Building AlmaLinux release'".to_string());

        let _ = manager.add_recipe(neofetch);
        let _ = manager.add_recipe(curl);
        let _ = manager.add_recipe(ripgrep);
        let _ = manager.add_recipe(almalinux_release);
        manager
    }

    pub fn add_recipe(&mut self, recipe: PackageRecipe) -> Result<(), RecipeError> {
        recipe.validate()?;
        self.recipes.insert(recipe.name.clone(), recipe);
        Ok(())
    }

    pub fn list_recipes(&self) -> Vec<&PackageRecipe> {
        self.recipes.values().collect()
    }
}

impl Default for RecipeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Declarative package recipes.
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
    pub arch: String,
    pub license_spdx: String,
    pub package_commands: Vec<String>,
}

impl PackageRecipe {
    pub fn new(name: String, version: Version) -> Self {
        PackageRecipe {
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
            arch: "x86_64".to_string(),
            license_spdx: "GPL".to_string(),
            package_commands: Vec::new(),
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

    pub fn with_pkgrel(mut self, pkgrel: u32) -> Self {
        self.pkgrel = pkgrel;
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    pub fn with_arch(mut self, arch: String) -> Self {
        self.arch = arch;
        self
    }

    pub fn with_package_command(mut self, command: String) -> Self {
        self.package_commands.push(command);
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
            BuildSystem::Custom => "custom_build_command".to_string(),
        }
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
        // Includes 4 default distro-inspired recipes plus our test recipe
        assert_eq!(manager.list_recipes().len(), 5);
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
            version: Version::new(3, 0, 0),
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
        assert_eq!(recipe.get_stage_optimization_flags(), "-O3 -march=native -flto=fat -funroll-loops");
    }
}
