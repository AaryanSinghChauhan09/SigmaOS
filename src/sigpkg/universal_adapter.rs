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
||||||| 2139cb2f8
        
        Ok(content.contains("Name") || content.contains("Version"))
        
        Ok(content.contains("Name:") || content.contains("Summary:"))
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

impl Default for RpmAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Arch Linux pacman package adapter
pub struct PacmanAdapter {
    user_hooks: Vec<UserHook>,
}

impl PacmanAdapter {
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

impl PackageFormatAdapter for PacmanAdapter {
    fn format_name(&self) -> &str {
        "pacman"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("pkgname = ") {
                name = line[10..].to_string();
            } else if line.starts_with("pkgver = ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("pkgdesc = ") {
                description = line[10..].to_string();
            } else if line.starts_with("depend = ") {
                let dep_name = line[9..].to_string();
                dependencies.push(Dependency {
                    name: dep_name,
                    version_constraint: VersionConstraint::Any,
                });
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
        output.push_str(&format!("pkgname = {}\n", package.name));
        output.push_str(&format!(
            "pkgver = {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("pkgdesc = {}\n", package.description));

        for dep in &package.dependencies {
            output.push_str(&format!("depend = {}\n", dep.name));
        }

        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;

        Ok(content.contains("pkgname") || content.contains("pkgver"))
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

impl Default for PacmanAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// NixOS Nix Package Expression Adapter
pub struct NixAdapter {
    user_hooks: Vec<UserHook>,
}

impl NixAdapter {
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

impl PackageFormatAdapter for NixAdapter {
    fn format_name(&self) -> &str {
        "nix"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let line_trimmed = line.trim();
            if line_trimmed.starts_with("pname = \"") {
                if let Some(end) = line_trimmed[9..].find('"') {
                    name = line_trimmed[9..9 + end].to_string();
                }
            } else if line_trimmed.starts_with("version = \"") {
                if let Some(end) = line_trimmed[11..].find('"') {
                    version_str = line_trimmed[11..11 + end].to_string();
                }
            } else if line_trimmed.starts_with("meta.description = \"") {
                if let Some(end) = line_trimmed[20..].find('"') {
                    description = line_trimmed[20..20 + end].to_string();
                }
            } else if line_trimmed.starts_with("inputs = [") {
                let inputs_str = &line_trimmed[10..];
                if let Some(end) = inputs_str.find(']') {
                    for dep in inputs_str[..end].split_whitespace() {
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
        output.push_str("{\n");
        output.push_str(&format!("  pname = \"{}\";\n", package.name));
        output.push_str(&format!(
            "  version = \"{}.{}.{}\";\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!(
            "  meta.description = \"{}\";\n",
            package.description
        ));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("  inputs = [ {} ];\n", dep_names.join(" ")));
        }
        output.push_str("}\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pname =")
            || content.contains("meta.description =")
            || content.contains("/nix/store"))
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

/// Gentoo Portage Ebuild Adapter
pub struct EbuildAdapter {
    user_hooks: Vec<UserHook>,
}

impl EbuildAdapter {
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

impl PackageFormatAdapter for EbuildAdapter {
    fn format_name(&self) -> &str {
        "ebuild"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let line_trimmed = line.trim();
            if line_trimmed.starts_with("PN=\"") {
                if let Some(end) = line_trimmed[4..].find('"') {
                    name = line_trimmed[4..4 + end].to_string();
                }
            } else if line_trimmed.starts_with("PV=\"") {
                if let Some(end) = line_trimmed[4..].find('"') {
                    version_str = line_trimmed[4..4 + end].to_string();
                }
            } else if line_trimmed.starts_with("DESCRIPTION=\"") {
                if let Some(end) = line_trimmed[13..].find('"') {
                    description = line_trimmed[13..13 + end].to_string();
                }
            } else if line_trimmed.starts_with("RDEPEND=\"") {
                if let Some(end) = line_trimmed[9..].find('"') {
                    for dep in line_trimmed[9..9 + end].split_whitespace() {
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
        output.push_str(&format!("PN=\"{}\"\n", package.name));
        output.push_str(&format!(
            "PV=\"{}.{}.{}\"\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("DESCRIPTION=\"{}\"\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("RDEPEND=\"{}\"\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("PN=")
            || content.contains("PV=")
            || content.contains("ebuild")
            || content.contains("EAPI="))
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

/// Alpine Linux APK Package Adapter
pub struct ApkAdapter {
    user_hooks: Vec<UserHook>,
}

impl ApkAdapter {
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

impl PackageFormatAdapter for ApkAdapter {
    fn format_name(&self) -> &str {
        "apk"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("P:") {
                name = line[2..].to_string();
            } else if line.starts_with("V:") {
                version_str = line[2..].to_string();
            } else if line.starts_with("D:") {
                let dep_name = line[2..].to_string();
                if !dep_name.is_empty() {
                    dependencies.push(Dependency {
                        name: dep_name,
                        version_constraint: VersionConstraint::Any,
                    });
                }
            } else if line.starts_with("T:") {
                description = line[2..].to_string();
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
        output.push_str(&format!("P:{}\n", package.name));
        output.push_str(&format!(
            "V:{}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("T:{}\n", package.description));
        for dep in &package.dependencies {
            output.push_str(&format!("D:{}\n", dep.name));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("P:") && content.contains("V:"))
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

/// Slackware TXZ Package Adapter
pub struct TxzAdapter {
    user_hooks: Vec<UserHook>,
}

impl TxzAdapter {
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

impl PackageFormatAdapter for TxzAdapter {
    fn format_name(&self) -> &str {
        "pkgtool"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();

        for line in content.lines() {
            if line.starts_with("PACKAGE NAME: ") {
                name = line[14..].to_string();
            } else if line.starts_with("PACKAGE VERSION: ") {
                version_str = line[17..].to_string();
            } else if line.starts_with("slack-desc: ") {
                description = line[12..].to_string();
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        Ok(Package::new(
            name,
            version,
            description,
            Vec::new(),
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("PACKAGE NAME: {}\n", package.name));
        output.push_str(&format!(
            "PACKAGE VERSION: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("slack-desc: {}\n", package.description));
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("PACKAGE NAME:") || content.contains("slack-desc:"))
    }

    fn extract_dependencies(&self, _data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        Ok(Vec::new()) // Slackware traditionally lacks dependency information
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

/// Void Linux XBPS Package Adapter
pub struct XbpsAdapter {
    user_hooks: Vec<UserHook>,
}

impl XbpsAdapter {
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

impl PackageFormatAdapter for XbpsAdapter {
    fn format_name(&self) -> &str {
        "xbps"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("pkgname: ") {
                name = line[9..].to_string();
            } else if line.starts_with("version: ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("short_desc: ") {
                description = line[12..].to_string();
            } else if line.starts_with("run_depends: ") {
                let deps_str = &line[13..];
                for dep in deps_str.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
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
        output.push_str(&format!("pkgname: {}\n", package.name));
        output.push_str(&format!(
            "version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("short_desc: {}\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("run_depends: {}\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pkgname:") || content.contains("run_depends:"))
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

/// CachyOS (x86-64 microarchitecture optimized) Package Adapter
pub struct CachyosAdapter {
    user_hooks: Vec<UserHook>,
}

impl CachyosAdapter {
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

impl PackageFormatAdapter for CachyosAdapter {
    fn format_name(&self) -> &str {
        "cachyos"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("pkgname = ") {
                name = line[10..].to_string();
            } else if line.starts_with("pkgver = ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("pkgdesc = ") {
                description = line[10..].to_string();
            } else if line.starts_with("depend = ") {
                let dep_name = line[9..].to_string();
                dependencies.push(Dependency {
                    name: dep_name,
                    version_constraint: VersionConstraint::Any,
                });
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
        output.push_str(&format!("pkgname = {}\n", package.name));
        output.push_str(&format!(
            "pkgver = {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("pkgdesc = {}\n", package.description));
        output.push_str("arch = x86_64-v3\n"); // CachyOS optimization profile
        for dep in &package.dependencies {
            output.push_str(&format!("depend = {}\n", dep.name));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pkgname") && content.contains("x86_64-v"))
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

/// Ubuntu Snap Package Adapter
pub struct SnapAdapter {
    user_hooks: Vec<UserHook>,
}

impl SnapAdapter {
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

impl PackageFormatAdapter for SnapAdapter {
    fn format_name(&self) -> &str {
        "snap"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();

        for line in content.lines() {
            if line.starts_with("name: ") {
                name = line[6..].to_string();
            } else if line.starts_with("version: ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("summary: ") {
                description = line[9..].to_string();
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        Ok(Package::new(
            name,
            version,
            description,
            Vec::new(),
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("name: {}\n", package.name));
        output.push_str(&format!(
            "version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("summary: {}\n", package.description));
        output.push_str("confinement: strict\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("name:")
            && (content.contains("confinement:") || content.contains("grade:")))
    }

    fn extract_dependencies(&self, _data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        Ok(Vec::new()) // Snaps encapsulate their dependencies
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

/// Flatpak Package Adapter
pub struct FlatpakAdapter {
    user_hooks: Vec<UserHook>,
}

impl FlatpakAdapter {
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

impl PackageFormatAdapter for FlatpakAdapter {
    fn format_name(&self) -> &str {
        "flatpak"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();

        for line in content.lines() {
            if line.starts_with("name=") {
                name = line[5..].to_string();
            } else if line.starts_with("version=") {
                version_str = line[8..].to_string();
            } else if line.starts_with("runtime=") {
                description = format!("Flatpak sandbox. Runtime: {}", &line[8..]);
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        Ok(Package::new(
            name,
            version,
            description,
            Vec::new(),
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str("[Application]\n");
        output.push_str(&format!("name={}\n", package.name));
        output.push_str(&format!(
            "version={}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str("runtime=org.freedesktop.Platform/x86_64/23.08\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("[Application]")
            || content.contains("[Extension]")
            || content.contains("runtime="))
    }

    fn extract_dependencies(&self, _data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        Ok(Vec::new()) // Flatpaks run in isolated runtimes
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

/// Intel Clear Linux swupd Package Adapter
pub struct SwupdAdapter {
    user_hooks: Vec<UserHook>,
}

impl SwupdAdapter {
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

impl PackageFormatAdapter for SwupdAdapter {
    fn format_name(&self) -> &str {
        "swupd"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("BUNDLE_NAME: ") {
                name = line[13..].to_string();
            } else if line.starts_with("BUNDLE_VERSION: ") {
                version_str = line[16..].to_string();
            } else if line.starts_with("CONTENTS: ") {
                description = format!("Clear Linux bundle content: {}", &line[10..]);
                for dep in line[10..].split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
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
        output.push_str(&format!("BUNDLE_NAME: {}\n", package.name));
        output.push_str(&format!(
            "BUNDLE_VERSION: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("CONTENTS: {}\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("BUNDLE_NAME:") || content.contains("swupd"))
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

/// Solus eopkg Package Adapter
pub struct EopkgAdapter {
    user_hooks: Vec<UserHook>,
}

impl EopkgAdapter {
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

impl PackageFormatAdapter for EopkgAdapter {
    fn format_name(&self) -> &str {
        "eopkg"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();

        if let Some(start) = content.find("<Name>") {
            if let Some(end) = content[start..].find("</Name>") {
                name = content[start + 6..start + end].to_string();
            }
        }
        if let Some(start) = content.find("<Version>") {
            if let Some(end) = content[start..].find("</Version>") {
                version_str = content[start + 9..start + end].to_string();
            }
        }
        if let Some(start) = content.find("<Description>") {
            if let Some(end) = content[start..].find("</Description>") {
                description = content[start + 13..start + end].to_string();
            }
        }

        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));

        Ok(Package::new(
            name,
            version,
            description,
            Vec::new(),
            String::new(),
        ))
    }

    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str("<Package>\n");
        output.push_str(&format!("  <Name>{}</Name>\n", package.name));
        output.push_str(&format!(
            "  <Version>{}.{}.{}</Version>\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!(
            "  <Description>{}</Description>\n",
            package.description
        ));
        output.push_str("</Package>\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("<Package>")
            || content.contains("<eopkg>")
            || content.contains("eopkg"))
    }

    fn extract_dependencies(&self, _data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        Ok(Vec::new())
    }

    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks {
            hook(package)?;
        }
        Ok(())
    }
}

/// GNU Guix Package Adapter
pub struct GuixAdapter {
    user_hooks: Vec<UserHook>,
}

impl GuixAdapter {
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

impl PackageFormatAdapter for GuixAdapter {
    fn format_name(&self) -> &str {
        "guix"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let line_trimmed = line.trim();
            if line_trimmed.starts_with("(name \"") {
                if let Some(end) = line_trimmed[7..].find('"') {
                    name = line_trimmed[7..7 + end].to_string();
                }
            } else if line_trimmed.starts_with("(version \"") {
                if let Some(end) = line_trimmed[10..].find('"') {
                    version_str = line_trimmed[10..10 + end].to_string();
                }
            } else if line_trimmed.starts_with("(synopsis \"") {
                if let Some(end) = line_trimmed[11..].find('"') {
                    description = line_trimmed[11..11 + end].to_string();
                }
            } else if line_trimmed.starts_with("(inputs (list ") {
                let inputs_str = &line_trimmed[14..];
                if let Some(end) = inputs_str.find(')') {
                    for dep in inputs_str[..end].split_whitespace() {
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
        output.push_str("(package\n");
        output.push_str(&format!("  (name \"{}\")\n", package.name));
        output.push_str(&format!(
            "  (version \"{}.{}.{}\")\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("  (synopsis \"{}\")\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("  (inputs (list {}))\n", dep_names.join(" ")));
        }
        output.push_str(")\n");
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("(package")
            || content.contains("(define-public")
            || content.contains("(synopsis"))
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

/// openSUSE Zypper Package Adapter
pub struct ZypperAdapter {
    user_hooks: Vec<UserHook>,
}

impl ZypperAdapter {
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

impl PackageFormatAdapter for ZypperAdapter {
    fn format_name(&self) -> &str {
        "zypper"
    }

    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("zypper: ") {
                name = line[8..].to_string();
            } else if line.starts_with("version: ") {
                version_str = line[9..].to_string();
            } else if line.starts_with("summary: ") {
                description = line[9..].to_string();
            } else if line.starts_with("requires: ") {
                let deps_str = &line[10..];
                for dep in deps_str.split_whitespace() {
                    dependencies.push(Dependency {
                        name: dep.to_string(),
                        version_constraint: VersionConstraint::Any,
                    });
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
        output.push_str(&format!("zypper: {}\n", package.name));
        output.push_str(&format!(
            "version: {}.{}.{}\n",
            package.version.major, package.version.minor, package.version.patch
        ));
        output.push_str(&format!("summary: {}\n", package.description));
        if !package.dependencies.is_empty() {
            let dep_names: Vec<&str> = package
                .dependencies
                .iter()
                .map(|d| d.name.as_str())
                .collect();
            output.push_str(&format!("requires: {}\n", dep_names.join(" ")));
        }
        Ok(output.into_bytes())
    }

    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("zypper:") || content.contains("requires:"))
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

// ----------------------------------------------------
// Universal Package Manager (OOPS Facade Pattern)
// ----------------------------------------------------
||||||| 984d1301f
/// Universal Package Manager (OOPS Facade Pattern)
/// Alpine Linux APK package adapter
pub struct ApkAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl ApkAdapter {
    pub fn new() -> Self {
        Self { user_hooks: Vec::new() }
    }
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for ApkAdapter {
    fn format_name(&self) -> &str { "apk" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("P:") {
                name = line[2..].to_string();
            } else if line.starts_with("V:") {
                version_str = line[2..].to_string();
            } else if line.starts_with("T:") {
                description = line[2..].to_string();
            } else if line.starts_with("D:") {
                let deps_str = line[2..].to_string();
                for dep in deps_str.split_whitespace() {
                    dependencies.push(Dependency { name: dep.to_string(), version_constraint: VersionConstraint::Any });
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("P:{}\n", package.name));
        output.push_str(&format!("V:{}.{}.{}\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("T:{}\n", package.description));
        if !package.dependencies.is_empty() {
            output.push_str("D:");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push('\n');
        }
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("P:") || content.contains("V:"))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for ApkAdapter {
    fn default() -> Self { Self::new() }
}

/// NixOS derivation package adapter
pub struct NixAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl NixAdapter {
    pub fn new() -> Self {
        Self { user_hooks: Vec::new() }
    }
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for NixAdapter {
    fn format_name(&self) -> &str { "nix" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.contains("pname =") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 { name = parts[1].to_string(); }
            } else if line.contains("version =") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 { version_str = parts[1].to_string(); }
            } else if line.contains("description =") {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 { description = parts[1].to_string(); }
            } else if line.contains("buildInputs =") {
                // extract dependencies simply
                let parts: Vec<&str> = line.split('[').nth(1).unwrap_or("").split(']').next().unwrap_or("").split_whitespace().collect();
                for dep in parts {
                    if !dep.is_empty() {
                        dependencies.push(Dependency { name: dep.to_string(), version_constraint: VersionConstraint::Any });
                    }
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str("{\n");
        output.push_str(&format!("  pname = \"{}\";\n", package.name));
        output.push_str(&format!("  version = \"{}.{}.{}\";\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("  meta.description = \"{}\";\n", package.description));
        if !package.dependencies.is_empty() {
            output.push_str("  buildInputs = [ ");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push_str(" ];\n");
        }
        output.push_str("}\n");
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("pname =") || content.contains("buildInputs"))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for NixAdapter {
    fn default() -> Self { Self::new() }
}

/// Gentoo Portage Ebuild package adapter
pub struct EbuildAdapter {
    user_hooks: Vec<Box<dyn Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync>>,
}

impl EbuildAdapter {
    pub fn new() -> Self {
        Self { user_hooks: Vec::new() }
    }
    pub fn add_hook<F>(&mut self, hook: F) where F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static {
        self.user_hooks.push(Box::new(hook));
    }
}

impl PackageFormatAdapter for EbuildAdapter {
    fn format_name(&self) -> &str { "ebuild" }
    fn parse_package(&self, data: &[u8]) -> Result<Package, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|e| AdapterError::ParseError(e.to_string()))?;
        let mut name = String::new();
        let mut version_str = String::new();
        let mut description = String::new();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            if line.starts_with("PN=") {
                name = line[3..].replace('"', "").replace('\'', "");
            } else if line.starts_with("PV=") {
                version_str = line[3..].replace('"', "").replace('\'', "");
            } else if line.starts_with("DESCRIPTION=") {
                description = line[12..].replace('"', "").replace('\'', "");
            } else if line.starts_with("DEPEND=") {
                let deps_str = line[7..].replace('"', "").replace('\'', "").replace('(', "").replace(')', "");
                for dep in deps_str.split_whitespace() {
                    if !dep.is_empty() {
                        dependencies.push(Dependency { name: dep.to_string(), version_constraint: VersionConstraint::Any });
                    }
                }
            }
        }
        let version = Version::parse(&version_str).unwrap_or_else(|_| Version::new(0, 0, 0));
        Ok(Package::new(name, version, description, dependencies, String::new()))
    }
    fn serialize_package(&self, package: &Package) -> Result<Vec<u8>, AdapterError> {
        let mut output = String::new();
        output.push_str(&format!("PN=\"{}\"\n", package.name));
        output.push_str(&format!("PV=\"{}.{}.{}\"\n", package.version.major, package.version.minor, package.version.patch));
        output.push_str(&format!("DESCRIPTION=\"{}\"\n", package.description));
        if !package.dependencies.is_empty() {
            output.push_str("DEPEND=\"");
            let dep_names: Vec<&str> = package.dependencies.iter().map(|d| d.name.as_str()).collect();
            output.push_str(&dep_names.join(" "));
            output.push_str("\"\n");
        }
        Ok(output.into_bytes())
    }
    fn validate(&self, data: &[u8]) -> Result<bool, AdapterError> {
        let content = String::from_utf8(data.to_vec()).map_err(|_| AdapterError::ValidationError("Invalid UTF-8".to_string()))?;
        Ok(content.contains("PN=") || content.contains("PV="))
    }
    fn extract_dependencies(&self, data: &[u8]) -> Result<Vec<Dependency>, AdapterError> {
        let package = self.parse_package(data)?;
        Ok(package.dependencies)
    }
    fn process_hook(&self, package: &mut Package) -> Result<(), AdapterError> {
        for hook in &self.user_hooks { hook(package)?; }
        Ok(())
    }
}

impl Default for EbuildAdapter {
    fn default() -> Self { Self::new() }
}

/// Universal Package Manager (OOPS Facade Pattern)
pub struct UniversalPackageManager {
    adapters: HashMap<String, Box<dyn PackageFormatAdapter>>,
    default_adapter: Option<String>,
    pub global_hooks: Vec<UserHook>,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            adapters: HashMap::new(),
            default_adapter: None,
            global_hooks: Vec::new(),
        };

        // Register built-in adapters for all major Linux distributions
        manager.register_adapter(Box::new(DebAdapter::new()));
        manager.register_adapter(Box::new(RpmAdapter::new()));
        manager.register_adapter(Box::new(PacmanAdapter::new()));
        manager.register_adapter(Box::new(NixAdapter::new()));
        manager.register_adapter(Box::new(EbuildAdapter::new()));
        manager.register_adapter(Box::new(ApkAdapter::new()));
        manager.register_adapter(Box::new(TxzAdapter::new()));
        manager.register_adapter(Box::new(XbpsAdapter::new()));
        manager.register_adapter(Box::new(CachyosAdapter::new()));
        manager.register_adapter(Box::new(SnapAdapter::new()));
        manager.register_adapter(Box::new(FlatpakAdapter::new()));
        manager.register_adapter(Box::new(SwupdAdapter::new()));
        manager.register_adapter(Box::new(EopkgAdapter::new()));
        manager.register_adapter(Box::new(GuixAdapter::new()));
        manager.register_adapter(Box::new(ZypperAdapter::new()));

||||||| 984d1301f
        
        manager.register_adapter(Box::new(ApkAdapter::new()));
        manager.register_adapter(Box::new(NixAdapter::new()));
        manager.register_adapter(Box::new(EbuildAdapter::new()));
        
        manager
    }

    /// Register a custom package format adapter
    pub fn register_adapter(&mut self, adapter: Box<dyn PackageFormatAdapter>) {
        let format_name = adapter.format_name().to_string();
        self.adapters.insert(format_name.clone(), adapter);

        // Set as default if no default exists
        if self.default_adapter.is_none() {
            self.default_adapter = Some(format_name);
        }
    }

    /// Set default adapter for unknown formats
    pub fn set_default_adapter(&mut self, format_name: &str) {
        if self.adapters.contains_key(format_name) {
            self.default_adapter = Some(format_name.to_string());
        }
    }

    /// Add manager-level global user-defined verification hook
    pub fn add_global_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Package) -> Result<(), AdapterError> + Send + Sync + 'static,
    {
        self.global_hooks.push(Box::new(hook));
    }

    /// Auto-detect package format and parse, running both adapter and manager UDF hooks
    pub fn auto_parse(&self, data: &[u8]) -> Result<Package, AdapterError> {
        for (_format_name, adapter) in &self.adapters {
            if adapter.validate(data).unwrap_or(false) {
                let mut package = adapter.parse_package(data)?;
                adapter.process_hook(&mut package)?;
                // Run manager-level global UDF hooks
                for global_hook in &self.global_hooks {
                    global_hook(&mut package)?;
                }
                return Ok(package);
            }
        }

        // Try default adapter as fallback
        if let Some(default_name) = &self.default_adapter {
            if let Some(adapter) = self.adapters.get(default_name) {
                let mut package = adapter.parse_package(data)?;
                adapter.process_hook(&mut package)?;
                // Run manager-level global UDF hooks
                for global_hook in &self.global_hooks {
                    global_hook(&mut package)?;
                }
                return Ok(package);
            }
        }

        Err(AdapterError::InvalidFormat)
    }

    /// Parse package with specific format
    pub fn parse_with_format(
        &self,
        format_name: &str,
        data: &[u8],
    ) -> Result<Package, AdapterError> {
        let adapter = self
            .adapters
            .get(format_name)
            .ok_or_else(|| AdapterError::UnsupportedFeature(format_name.to_string()))?;

        let mut package = adapter.parse_package(data)?;
        adapter.process_hook(&mut package)?;
        // Run manager-level global UDF hooks
        for global_hook in &self.global_hooks {
            global_hook(&mut package)?;
        }
        Ok(package)
    }

    /// Convert package between formats
    pub fn convert_format(
        &self,
        package: &Package,
        target_format: &str,
    ) -> Result<Vec<u8>, AdapterError> {
        let adapter = self
            .adapters
            .get(target_format)
            .ok_or_else(|| AdapterError::UnsupportedFeature(target_format.to_string()))?;

        adapter.serialize_package(package)
    }

    /// Get list of supported formats
    pub fn supported_formats(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deb_adapter_parsing() {
        let adapter = DebAdapter::new();
        let deb_data = b"Package: test-package
Version: 1.0.0
Description: A test package
Depends: libc, libssl";

        let package = adapter.parse_package(deb_data).unwrap();
        assert_eq!(package.name, "test-package");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_rpm_adapter_parsing() {
        let adapter = RpmAdapter::new();
        let rpm_data = b"Name: test-rpm
Version: 2.0.0
Summary: An RPM test package
Requires: glibc openssl";

        let package = adapter.parse_package(rpm_data).unwrap();
        assert_eq!(package.name, "test-rpm");
        assert_eq!(package.version.major, 2);
    }

    #[test]
    fn test_pacman_adapter_parsing() {
        let adapter = PacmanAdapter::new();
        let pacman_data = b"pkgname = test-pacman
pkgver = 3.0.0
pkgdesc = A Pacman test package
depend = glibc
depend = openssl";

        let package = adapter.parse_package(pacman_data).unwrap();
        assert_eq!(package.name, "test-pacman");
        assert_eq!(package.version.major, 3);
    }

||||||| 984d1301f
    

    #[test]
    fn test_apk_adapter_parsing() {
        let adapter = ApkAdapter::new();
        let apk_data = b"P:test-apk\nV:4.2.0\nT:Alpine test\nD:musl openssl";
        let pkg = adapter.parse_package(apk_data).unwrap();
        assert_eq!(pkg.name, "test-apk");
        assert_eq!(pkg.version.major, 4);
        assert_eq!(pkg.dependencies.len(), 2);
    }

    #[test]
    fn test_nix_adapter_parsing() {
        let adapter = NixAdapter::new();
        let nix_data = b"pname = \"test-nix\";\nversion = \"5.1.0\";\ndescription = \"Nix test\";\nbuildInputs = [ glibc ];";
        let pkg = adapter.parse_package(nix_data).unwrap();
        assert_eq!(pkg.name, "test-nix");
        assert_eq!(pkg.version.major, 5);
        assert_eq!(pkg.dependencies.len(), 1);
    }

    #[test]
    fn test_ebuild_adapter_parsing() {
        let adapter = EbuildAdapter::new();
        let ebuild_data = b"PN=\"test-ebuild\"\nPV=\"6.2.3\"\nDESCRIPTION=\"Gentoo test\"\nDEPEND=\"gcc clang\"";
        let pkg = adapter.parse_package(ebuild_data).unwrap();
        assert_eq!(pkg.name, "test-ebuild");
        assert_eq!(pkg.version.major, 6);
        assert_eq!(pkg.dependencies.len(), 2);
    }
    
    #[test]
    fn test_nix_adapter_parsing_and_serialization() {
        let adapter = NixAdapter::new();
        let nix_data = b"  pname = \"hello\";\n  version = \"2.12.1\";\n  meta.description = \"Gnu hello world\";\n  inputs = [ glibc coreutils ];";

        let package = adapter.parse_package(nix_data).unwrap();
        assert_eq!(package.name, "hello");
        assert_eq!(package.version.major, 2);
        assert_eq!(package.version.minor, 12);
        assert_eq!(package.dependencies.len(), 2);

        let serialized = adapter.serialize_package(&package).unwrap();
        let serialized_str = String::from_utf8(serialized).unwrap();
        assert!(serialized_str.contains("pname = \"hello\";"));
    }

    #[test]
    fn test_ebuild_adapter_parsing_and_serialization() {
        let adapter = EbuildAdapter::new();
        let ebuild_data = b"PN=\"sys-apps/util-linux\"\nPV=\"2.39.2\"\nDESCRIPTION=\"Essential utilities for Linux\"\nRDEPEND=\"sys-libs/ncurses sys-libs/pam\"";

        let package = adapter.parse_package(ebuild_data).unwrap();
        assert_eq!(package.name, "sys-apps/util-linux");
        assert_eq!(package.version.major, 2);
        assert_eq!(package.dependencies.len(), 2);

        let serialized = adapter.serialize_package(&package).unwrap();
        let serialized_str = String::from_utf8(serialized).unwrap();
        assert!(serialized_str.contains("PN=\"sys-apps/util-linux\""));
    }

    #[test]
    fn test_apk_adapter_parsing() {
        let adapter = ApkAdapter::new();
        let apk_data = b"P:musl\nV:1.2.4\nT:standard musl libc\nD:so:libc.so.6";

        let package = adapter.parse_package(apk_data).unwrap();
        assert_eq!(package.name, "musl");
        assert_eq!(package.version.major, 1);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_xbps_adapter_parsing_and_serialization() {
        let adapter = XbpsAdapter::new();
        let xbps_data = b"pkgname: neovim\nversion: 0.9.1\nshort_desc: Vim-fork focused on extensibility\nrun_depends: libuv msgpack";

        let package = adapter.parse_package(xbps_data).unwrap();
        assert_eq!(package.name, "neovim");
        assert_eq!(package.version.major, 0);
        assert_eq!(package.dependencies.len(), 2);
    }

    #[test]
    fn test_cachyos_adapter_microarch_validation() {
        let adapter = CachyosAdapter::new();
        let data = b"pkgname = cachyos-kernel\npkgver = 6.8.1\npkgdesc = Optimized kernel\narch = x86_64-v3";
        assert!(adapter.validate(data).unwrap());
    }

    #[test]
    fn test_snap_adapter_parsing() {
        let adapter = SnapAdapter::new();
        let data = b"name: core22\nversion: 2023.05.31\nsummary: Canonical runtime base";
        let package = adapter.parse_package(data).unwrap();
        assert_eq!(package.name, "core22");
    }

    #[test]
    fn test_flatpak_adapter_parsing() {
        let adapter = FlatpakAdapter::new();
        let data =
            b"[Application]\nname=org.gimp.GIMP\nversion=2.10.36\nruntime=org.gnome.Platform";
        let package = adapter.parse_package(data).unwrap();
        assert_eq!(package.name, "org.gimp.GIMP");
    }

    #[test]
    fn test_swupd_adapter_parsing() {
        let adapter = SwupdAdapter::new();
        let data = b"BUNDLE_NAME: sysadmin-basic\nBUNDLE_VERSION: 41220\nCONTENTS: bash curl sed";
        let package = adapter.parse_package(data).unwrap();
        assert_eq!(package.name, "sysadmin-basic");
        assert_eq!(package.dependencies.len(), 3);
    }

    #[test]
    fn test_eopkg_adapter_parsing_and_serialization() {
        let adapter = EopkgAdapter::new();
        let data = b"<Package>\n  <Name>firefox</Name>\n  <Version>120.0.0</Version>\n  <Description>Mozilla Firefox</Description>\n</Package>";
        let package = adapter.parse_package(data).unwrap();
        assert_eq!(package.name, "firefox");

        let serialized = adapter.serialize_package(&package).unwrap();
        let serialized_str = String::from_utf8(serialized).unwrap();
        assert!(serialized_str.contains("<Name>firefox</Name>"));
    }

    #[test]
    fn test_guix_adapter_parsing_and_serialization() {
        let adapter = GuixAdapter::new();
        let data = b"(package\n  (name \"readline\")\n  (version \"8.2.0\")\n  (synopsis \"GNU readline library\")\n  (inputs (list ncurses))\n)";
        let package = adapter.parse_package(data).unwrap();
        assert_eq!(package.name, "readline");
        assert_eq!(package.dependencies.len(), 1);

        let serialized = adapter.serialize_package(&package).unwrap();
        let serialized_str = String::from_utf8(serialized).unwrap();
        assert!(serialized_str.contains("(name \"readline\")"));
    }

    #[test]
    fn test_zypper_adapter_parsing_and_serialization() {
        let adapter = ZypperAdapter::new();
        let data = b"zypper: patterns-openSUSE-base\nversion: 15.5.0\nsummary: openSUSE base system\nrequires: patterns-openSUSE-minimal_base";
        let package = adapter.parse_package(data).unwrap();
        assert_eq!(package.name, "patterns-openSUSE-base");
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_universal_manager_auto_parse_multiple_formats() {
        let manager = UniversalPackageManager::new();

        // 1. Test Nix format
        let nix_data = b"  pname = \"nix-tool\";\n  version = \"1.0.0\";\n  meta.description = \"Nix tool description\";";
        let nix_pkg = manager.auto_parse(nix_data).unwrap();
        assert_eq!(nix_pkg.name, "nix-tool");

        // 2. Test Ebuild format
        let ebuild_data = b"PN=\"gentoo-tool\"\nPV=\"2.0.0\"\nDESCRIPTION=\"Gentoo tool ebuild\"";
        let ebuild_pkg = manager.auto_parse(ebuild_data).unwrap();
        assert_eq!(ebuild_pkg.name, "gentoo-tool");

        // 3. Test APK format
        let apk_data = b"P:apk-tool\nV:3.0.0\nT:Alpine tool description";
        let apk_pkg = manager.auto_parse(apk_data).unwrap();
        assert_eq!(apk_pkg.name, "apk-tool");
    }

    #[test]
    fn test_global_user_defined_hook() {
        let mut manager = UniversalPackageManager::new();

        // Register global verification hook that enforces open-source licensing constraints
        manager.add_global_hook(|package: &mut Package| -> Result<(), AdapterError> {
            // Simulated validation of open source licensing compliance
            if package.name.contains("proprietary") {
                return Err(AdapterError::HookError(
                    "Non-free proprietary license detected!".to_string(),
                ));
            }
            package.description = format!("{} (Verified Open Source)", package.description);
            Ok(())
        });

        let data = b"Package: open-curl\nVersion: 7.85.0\nDescription: Command line tool";
        let parsed = manager.auto_parse(data).unwrap();
        assert_eq!(parsed.name, "open-curl");
        assert!(parsed.description.contains("(Verified Open Source)"));

        let proprietary_data =
            b"Package: proprietary-driver\nVersion: 525.60.11\nDescription: Closed source driver";
        let parse_result = manager.auto_parse(proprietary_data);
        assert!(parse_result.is_err());
    }

    #[test]
    fn test_format_conversion_cross_platform() {
        let manager = UniversalPackageManager::new();
        let package = Package::new(
            "universal-tool".to_string(),
            Version::new(1, 2, 3),
            "Unified utility".to_string(),
            vec![Dependency {
                name: "libc".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            String::new(),
        );

        // Convert to Nix
        let nix_data = manager.convert_format(&package, "nix").unwrap();
        let nix_str = String::from_utf8(nix_data).unwrap();
        assert!(nix_str.contains("pname = \"universal-tool\";"));

        // Convert to Guix
        let guix_data = manager.convert_format(&package, "guix").unwrap();
        let guix_str = String::from_utf8(guix_data).unwrap();
        assert!(guix_str.contains("(name \"universal-tool\")"));
    }
}
