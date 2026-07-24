// SigmaOS Package Recipes
// Build recipes for package compilation and installation

use crate::sigpkg::{Dependency, Version};
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
    pub pkgrel: u32,
    pub arch: String,
    pub license_spdx: String,
    pub prepare_commands: Vec<String>,
    pub package_commands: Vec<String>,
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
            pkgrel: 1,
            arch: "x86_64".to_string(),
            license_spdx: "GPL".to_string(),
            prepare_commands: Vec::new(),
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

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    pub fn with_pkgrel(mut self, pkgrel: u32) -> Self {
        self.pkgrel = pkgrel;
        self
    }

    pub fn with_arch(mut self, arch: String) -> Self {
        self.arch = arch;
        self
    }

    pub fn with_prepare_command(mut self, command: String) -> Self {
        self.prepare_commands.push(command);
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
        let key = format!("{}@{:?}", recipe.name, recipe.version);
        self.recipes.insert(key, recipe);
        Ok(())
    }

    pub fn get_recipe(&self, name: &str, version: &Version) -> Option<&PackageRecipe> {
        let key = format!("{}@{:?}", name, version);
        self.recipes.get(&key)
    }

    pub fn list_recipes(&self) -> Vec<&PackageRecipe> {
        self.recipes.values().collect()
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&PackageRecipe> {
        self.recipes.values().filter(|r| r.name == name).collect()
    }

    pub fn remove_recipe(&mut self, name: &str, version: &Version) {
        let key = format!("{}@{:?}", name, version);
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
    fn test_pkgbuild_and_aur_compilation_fields() {
        let recipe = PackageRecipe::new("neofetch-pqc".to_string(), Version::new(7, 1, 0))
            .with_pkgrel(3)
            .with_arch("aarch64".to_string())
            .with_source(
                "https://github.com/dylanaraps/neofetch".to_string(),
                "hash_neofetch".to_string(),
            )
            .with_prepare_command("patch -p1 < pqc_patch.diff".to_string())
            .with_build_command("make build".to_string())
            .with_package_command("make DESTDIR=\"$pkgdir\" install".to_string());

        assert_eq!(recipe.pkgrel, 3);
        assert_eq!(recipe.arch, "aarch64");
        assert_eq!(recipe.prepare_commands[0], "patch -p1 < pqc_patch.diff");
        assert_eq!(
            recipe.package_commands[0],
            "make DESTDIR=\"$pkgdir\" install"
        );
    }
}
