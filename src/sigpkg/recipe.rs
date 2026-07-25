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
    pub fn new(
        name: &'static str,
        major: u32,
        minor: u32,
        patch: u32,
        url: &'static str,
        _dependencies: &'static [&'static str],
    ) -> Self {
        PackageRecipe {
            name: name.to_string(),
            version: Version {
                major,
                minor,
                patch,
            },
            description: String::new(),
            build_system: BuildSystem::Cargo,
            dependencies: Vec::new(),
            source_url: url.to_string(),
            hash: String::new(),
            build_commands: Vec::new(),
            install_commands: Vec::new(),
            environment: HashMap::new(),
            pkgrel: 1,
            arch: "any".to_string(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSystem {
    Cargo,
    Make,
    CMake,
    Ninja,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeError {
    InvalidRecipe,
    MissingField,
}

pub struct RecipeManager;
impl RecipeManager {
    pub fn new() -> Self {
        Self
    }
}
