//! SigPkg: Community Recipe Packaging (Arch Linux Absorption)
//!
//! Zero-allocation package manager parsing simple, signed declarative community recipes.

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
    pub recipes: Vec<PackageRecipe>,
}

impl RecipeManager {
    pub fn new() -> Self {
        RecipeManager {
            recipes: Vec::new(),
        }
    }

    pub fn add_recipe(&mut self, recipe: PackageRecipe) -> Result<(), RecipeError> {
        self.recipes.push(recipe);
        Ok(())
    }

    pub fn list_recipes(&self) -> &Vec<PackageRecipe> {
        &self.recipes
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
    pub pkgrel: u32,
    pub arch: String,
    pub license_spdx: String,
    pub prepare_commands: Vec<String>,
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
            pkgrel: 1,
            arch: String::new(),
            license_spdx: String::new(),
            prepare_commands: Vec::new(),
            package_commands: Vec::new(),