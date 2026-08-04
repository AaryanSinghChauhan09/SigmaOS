// Universal Package Adapter System
// OOPS-based design to support all Linux distro package formats in SigmaOS

use crate::sigpkg::{Dependency, Package, ParseError, Version, VersionConstraint};
use std::collections::HashMap;

/// Abstract trait for package format adapters (OOPS principle)
pub trait PackageFormatAdapter: Send + Sync {
    /// Get the format name (e.g., "deb", "rpm", "pacman")
    fn format_name(&self) -> &str;

    /// Parse package metadata from distro-specific format
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError>;

    /// Convert SigmaOS package to distro-specific format
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError>;

    /// Validate package integrity
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError>;

    /// Extract dependencies from package
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError>;

    /// User-defined hook for custom package processing
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError>;
}

/// Error types for package adapters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    InvalidFormat,
    ParseError(String),
    SerializationError(String),
    ValidationError(String),
    UnsupportedFeature(String),
    HookError(String),
}

// ----------------------------------------------------
// Type alias for User-Defined hooks to ensure OOP consistency
// ----------------------------------------------------
pub type UserHook = Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>;

// ----------------------------------------------------
// Concrete Implementations of Distro Adapters
// ----------------------------------------------------

/// Debian/Ubuntu .deb package adapter
pub struct DebAdapter {
    user_hooks: Vec<UserHook>,
}

impl DebAdapter {
    pub fn new() -> Self {
        Self {
            user_hooks: Vec::new(),
        }
    }

    /// Add user-defined processing hook
    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static,
    {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for DebAdapter {
    fn format_name(&self) -> &str {
        "deb"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("Package: ") {
                name = line[9..].to_string();
            } else if line.starts_with("Version: ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("Description: ") {
                description = line[13..].to_string();
            } else if line.starts_with("Depends: ") {
                let deps_str = &line[9..];
                for dep in deps_str.split(',') {
                    let dep_name = dep.trim().split_whitespace().next().unwrap_or("");
                    if !dep_name.is_empty() {
                        dependencies.push(Dependency {
                            name: dep_name.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        Ok(Package::new(
            name,
            version,
            description,
            dependencies,
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("Package: {}\n", package.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("Description: {}\n", package.description));

        if !package.dependencies.is_empty() {
            output.push_str("Depends: ");
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&dep_names.join(", "));
            output.push('\n');
        }

        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;

        Ok(content.contains("Package:") || content.contains("Version:"))
    }

    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

impl Default for DebAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Fedora/RHEL .rpm package adapter
pub struct RpmAdapter {
    user_hooks: Vec<UserHook>,
}

impl RpmAdapter {
    pub fn new() -> Self {
        Self {
            user_hooks: Vec::new(),
        }
    }

    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static,
    {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for RpmAdapter {
    fn format_name(&self) -> &str {
        "rpm"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.contains("Name") && line.contains(':') {
                name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Version") && line.contains(':') {
                version_str = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Summary") && line.contains(':') {
                description = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("Requires") && line.contains(':') {
                let deps_str = line.split(':').nth(1).unwrap_or("");
                for dep in deps_str.split_whitespace() {
                    if !dep.is_empty() {
                        dependencies.push(Dependency {
                            name: dep.to_string(),
                            version_constraint: VersionConstraint::Any,
                        });
                    }
                }
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        Ok(Package::new(
            name,
            version,
            description,
            dependencies,
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("Name: {}\n", package.name));
        output.push_str(&format!(
            "Version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("Summary: {}\n", package.description));

        if !package.dependencies.is_empty() {
            output.push_str("Requires: ");
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&dep_names.join(" "));
            output.push('\n');
        }

        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;

        Ok(content.contains("Name") || content.contains("Version"))